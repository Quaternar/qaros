/** \file main.cpp
 *  \brief The Vulkan cube: a source app that joins the Hub on this machine as
 *  a local app and renders a cube with Vulkan for every target app.
 */

// The page holds tables and command lines wider than the column limit.
// clang-format off
/** \page qar_c_tutorial_vulkan_rendering Vulkan Rendering
 * \tableofcontents
 *
 * \section vulkan_build What you build
 * The Vulkan cube: a rotating cube with one flat color per face, rendered on
 * the GPU in stereo color and depth for every target app in the session. Each
 * target, a Visualizer or a headset, gets its own sender and its own camera.
 * The cube runs on the Hub machine, joins the Hub as a local app, creates its
 * own Vulkan device on the GPU the session uses, and hands each frame to QAROS
 * through the Vulkan render sender without a copy.
 *
 * Take this path when your renderer already runs on Vulkan. QAROS creates the
 * frame images on your device; you render into them and hand them back.
 *
 * What you will learn:
 * - how to join as a local app, and how to cancel the approval wait on a stop
 *   request
 * - how to create your Vulkan device on the session's GPU with the extensions
 *   QAROS needs
 * - how to find the target apps and serve each on a thread of its own
 * - the frame hand-off: take each image from the state QAROS reports, render,
 *   release it to the external queue family, signal the frame's semaphore,
 *   write the new state back, show
 * - which handle lives how long, and why the device outlives the session
 *
 * The cube is C++20, not C: it uses threads, a mutex and `std::unique_ptr`
 * for the parts that are not QAROS. Every QAROS call is the C API. The program
 * is `qar-streaming-c/examples/vulkan_cube/`: `main.cpp` holds everything
 * QAROS, `renderer.cpp` the Vulkan device and the camera. This page shows
 * their code. The \ref qar_c_tutorial_cpu_rendering "CPU Rendering" tutorial
 * explains peers, app volumes and the frame cycle; this one builds on it.
 *
 * \section vulkan_prereq Before you start
 * - **Windows 10 or 11**, x64. The cube is built for Windows only.
 * - **A GPU** whose Vulkan driver offers `VK_KHR_external_memory_win32` and
 *   `VK_KHR_external_semaphore_win32`. QAROS supports NVIDIA GPUs
 *   (<a href="/docs/developer-guide/onboarding-and-sessions#choosing-the-gpu">Choosing
 *   the GPU</a>).
 * - **QAROS installed** on the Hub machine, where the cube runs. When the Hub
 *   does not run, the join starts it, and the Hub keeps running after the cube
 *   exits.
 * - **Visual Studio 2022 Build Tools** ("Desktop development with C++"),
 *   **CMake 3.29** or newer and **Git**. The Vulkan headers, the loader, glm and
 *   the shader compiler come from vcpkg.
 * - **What you do:** on the first run, click **Allow** in the QAROS approval
 *   prompt, in the Visualizer or in a tray window, within 120 s
 *   (<a href="/docs/developer-guide/onboarding-and-sessions#the-approval-prompt">The
 *   approval prompt</a>). The cube is not code-signed, so QAROS asks again
 *   after every rebuild.
 * - **A target:** open the Visualizer, or start a headset that joined the Hub.
 *
 * \section vulkan_run Run it
 * Build from the root of this repository, in a Developer PowerShell for
 * VS 2022. The first configure downloads vcpkg and builds the dependencies,
 * which takes several minutes:
 * \code{.bash}
 * cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
 * cmake --build --preset x64-windows-release --target qar-vulkan-source
 * .\build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe
 * \endcode
 *
 * The cube takes no arguments. `--help` prints its usage; any other argument
 * prints the usage to stderr and exits with 1. Stop it with Ctrl+C. When its
 * stdin is a pipe, the end of stdin stops it too: that is how QAROS stops an
 * app it started.
 *
 * Expected output, with the Visualizer open, for a run that you stop later
 * (your GPU and peer id differ; with console logging on, the runtime adds log
 * lines of its own):
 * \code{.txt}
 * Joining the QAROS on this machine. On the first run, click Allow in the QAROS approval prompt.
 * Connecting to the local QAROS hub.
 * Certificate signed.
 * Onboarding complete.
 * Rendering on <GPU name> (LUID <adapter LUID>), the session's GPU
 * Connected. Waiting for rendering targets. Ctrl+C to quit.
 * Rendering for target <peer id>
 * \endcode
 *
 * The three lines after `Joining` are the runtime's progress messages. While
 * the approval prompt is open, one more appears:
 * `Waiting for approval in QAROS hub`. In the Visualizer, the cube appears in
 * the "Vulkan Cube" app volume, 1.5 m in front of the room origin.
 *
 * Exit codes:
 * | Code | Meaning |
 * |---|---|
 * | 0 | stopped by Ctrl+C or the end of stdin |
 * | 1 | an argument other than `--help` or `-h`, `qar_library_init` or `qar_runtime_create` failed, or any other failure: a join failure not listed below, the GPU, the app volume, the peer list |
 * | 2 | no runtime loaded |
 * | 3 | the join could neither start nor reach the Hub (`QAR_STATUS_ONBOARDING_HUB_UNREACHABLE`) |
 * | 4 | the approval was denied or revoked (`QAR_STATUS_ONBOARDING_APPROVAL_DENIED`) |
 * | 5 | nobody answered the approval prompt in time (`QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT`) |
 * | 6 | the program answering on this machine is not the installed QAROS (`QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC`) |
 * | 7 | the executable no longer matches what QAROS approved (`QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH`) |
 *
 * \section vulkan_walkthrough Walkthrough
 *
 * \subsection vulkan_main_step 1. Start, load the runtime, map failures to exit codes
 * \snippet main.cpp vulkan_main
 *
 * A signal handler may safely do nothing but write a lock-free atomic, so
 * Ctrl+C only sets `g_stopRequested`. Every QAROS and GPU cleanup stays on
 * the threads that own the objects. When stdin is a pipe, a thread waits for
 * its end and sets the same flag.
 *
 * `qar_library_load(NULL)` runs the default lookup: the runtime beside the
 * executable, the folder `QAROS_RUNTIME_DIR` names, the installed QAROS, then
 * `PATH`. It prints where it looked when it finds nothing. The
 * \ref qar_c_tutorial_dynamic_loading "Dynamic Loading" tutorial explains each
 * step.
 *
 * \subsection vulkan_order_step 2. The order of the whole program
 * \snippet main.cpp vulkan_run_app
 *
 * Join first: the session decides the GPU, and the device is created there.
 * Note the order of the declarations. `gpu` is declared first, so it is
 * destroyed last, after the session.
 * \ref vulkan_ownership "Ownership and teardown" explains why.
 *
 * \subsection vulkan_join_step 3. Join as a local app
 * A source app on the Hub machine joins as a local app: the user approves it
 * once in QAROS, and there is no code to type. Nothing is stored; every start
 * joins again.
 * \snippet main.cpp vulkan_join
 *
 * The call can wait minutes for the approval, so it takes a cancel token. A
 * second thread watches the stop flag and cancels the token when a stop
 * arrives first. A zeroed QarOnboardLocalAppExt::approval_timeout_ms keeps the
 * Hub's default of 120 s. No QarGraphicsDeviceId is chained: QAROS selects the
 * GPU. The progress callback prints the runtime's messages, such as
 * `Waiting for approval in QAROS hub`.
 *
 * When joining fails, the cube tells the user what happened and what to do,
 * by status code, and exits with that refusal's own code, the exit codes of
 * \ref qar_c_tutorial_local_app_onboarding "Local App Onboarding":
 * \snippet main.cpp vulkan_explain
 *
 * These are the codes of
 * <a href="/docs/developer-guide/onboarding-and-sessions#status-codes-and-what-to-tell-the-user">Status
 * codes and what to tell the user</a>. Retry none of them silently. A stop
 * during the wait is not a failure: `RunApp` returns 0 without an
 * explanation.
 *
 * \subsection vulkan_gpu_step 4. Create the device on the session's GPU
 * QAROS creates the frame images on your device, and the receiver reads them
 * on the session's GPU. Render on that GPU, so frames need no copy between
 * GPUs. \ref qar_session_get_graphics_device_id returns the adapter the
 * session uses: the one a chained QarGraphicsDeviceId named, else the one
 * QAROS selected, else the one the library picked.
 * \snippet main.cpp vulkan_adapter
 *
 * `VulkanDevice::Create` in `renderer.cpp` takes the first physical device
 * whose `VkPhysicalDeviceIDProperties` match that LUID or UUID, that has a
 * graphics queue family, and that offers these device extensions:
 * \snippet renderer.cpp vulkan_extensions
 *
 * When the session names an adapter and no physical device on it qualifies,
 * the cube stops with an error rather than render on another GPU. When the
 * session names none, it takes the first supported GPU and says so.
 *
 * \subsection vulkan_targets_step 5. Find the targets
 * The app volume is the cube's own. A sender attaches its target to it, and
 * that is what makes the target ask for the content:
 * \snippet main.cpp vulkan_volume
 *
 * Which targets to serve is the source app's choice. The cube serves every
 * target app that is not gone. The peer callback runs on a library thread, so
 * it only queues ids under a lock:
 * \snippet main.cpp vulkan_targets
 *
 * Subscribe first, then list the peers already there, so a target that joins
 * in between is not missed. The main thread then manages targets only: it
 * starts one for each new target app, restarts one that failed after 3 s, and
 * forgets one whose stream closed for good.
 * \snippet main.cpp vulkan_follow
 *
 * \subsection vulkan_sender_step 6. One sender per target
 * Each `Target` runs on a thread of its own. It creates its sender, then
 * renders until it is stopped or its stream fails. A target that never
 * answers blocks only its own thread.
 * \snippet main.cpp vulkan_sender
 *
 * QarStreamParamsVulkan, chained into the init, hands QAROS your physical
 * device and device. Each eye gets a color and a depth texture of its own,
 * the `QAR_FRAME_LAYOUT_SEPARATED_TEXTURES` layout, four views in all.
 * \ref qar_render_sender_create blocks until the target answers;
 * `RequestStop` cancels it through the target's token.
 *
 * \subsection vulkan_frame_step 7. One frame
 * \snippet main.cpp vulkan_frame
 *
 * \ref qar_render_sender_begin_frame paces the thread: it returns when this
 * target wants its next frame, so each target runs at its own display rate,
 * and a slow or paused one never delays another. The frame path sleeps only
 * after a waiting status, 2 ms before the next begin.
 *
 * The frame info it returns describes the begun frame. Keep it until
 * \ref qar_render_sender_show_frame has shown that frame. The `FrameInfo`
 * type, a `std::unique_ptr` whose deleter calls
 * `qar_render_frame_info_handle_destroy`, destroys it on every path out of
 * `RenderNext`, after the show.
 *
 * A begun frame is always shown, as in the CPU and D3D11 programs: the show
 * closes the frame cycle that begin opened. When rendering fails, the camera
 * first discards the frame by signaling its semaphore, the cube shows it, and
 * the target ends. So a stop request that arrives after a successful begin
 * waits for that one frame, and the thread stops at the next turn of its
 * loop. One case differs, for a Vulkan reason: the show waits for the frame's
 * semaphore, which comes with the frame record. When reading the record
 * fails, nothing can signal it, so the target ends without a show and its
 * sender is destroyed with it.
 *
 * The pose and the field of view of each eye come from the view showing that
 * eye's color. The poses are in the app volume's app content space, the space
 * the cube renders in:
 * \snippet main.cpp vulkan_views
 *
 * A failed begin-frame or show is not always the end of the target:
 * \snippet main.cpp vulkan_waiting
 *
 * Reconnecting and transfer-less streams keep their sender, and so does an app
 * volume that does not resolve yet; begin-frame holds the unresolved volume
 * about 16 ms before it reports it. A paused stream does not show up here:
 * begin-frame waits through the pause. Any other failure ends the
 * target, `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED` included; a closed
 * stream is forgotten until the peer list offers the target again
 * (<a href="/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow">Status
 * codes that are part of normal flow</a>).
 *
 * \subsection vulkan_handoff_step 8. Hand the images back
 * `qar_render_sender_frame_vulkan` gives the sender's frame: its views, its
 * images with their current layout and owning queue family, and the frame's
 * semaphore. The pointer stays valid until the next call to it or the
 * sender's destruction. QAROS owns the images, their memory and the
 * semaphore: never destroy, free or re-bind them.
 *
 * The camera takes each image from exactly the state the frame reports, and
 * leaves it in `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, released to
 * `VK_QUEUE_FAMILY_EXTERNAL`:
 * \snippet renderer.cpp vulkan_barriers
 *
 * Then, in this order: submit the work that signals the frame's semaphore,
 * write the images' new state back into the frame, and only then show:
 * \snippet renderer.cpp vulkan_handoff
 *
 * The camera keeps two frames in flight. It waits on a slot's fence only when
 * it reuses that slot, never for the frame it just submitted: the semaphore
 * hands that work to QAROS, which waits for it on the GPU.
 *
 * A begun frame that could not be rendered still signals its semaphore, so
 * the show that closes its cycle has something to wait for:
 * \snippet renderer.cpp vulkan_discard
 *
 * Image handles can change between frames. Read the frame every frame. The
 * camera caches its image views and framebuffers by image handle, so a new
 * handle gets new ones.
 *
 * \section vulkan_ownership Ownership and teardown
 * | Object | Created by | Lives until |
 * |---|---|---|
 * | `QarRenderFrameInfo*` | \ref qar_render_sender_begin_frame | the frame is shown or discarded; `FrameInfo` destroys it |
 * | `QarVideoFrameVulkan*` | `qar_render_sender_frame_vulkan` | the next call to it, or the sender's destruction; never freed by you |
 * | frame images, memory, semaphore | QAROS | the sender; never destroyed by you |
 * | camera (views, framebuffers, fences) | `CubeCamera::Create` | the `Target`, before its sender |
 * | `QarRenderSender*` | \ref qar_render_sender_create | the `Target`; \ref qar_render_stream_handle_destroy |
 * | target token | \ref qar_cancel_token_create | the `Target`, after its sender |
 * | subscription token | \ref qar_cancel_token_create | the end of `Run`, after every target |
 * | `QarSession*` | \ref qar_runtime_onboard | `RunApp`, before the runtime and the device |
 * | `VulkanDevice` | `VulkanDevice::Create` | `RunApp`, last |
 *
 * - **Stop before join.** `Run` asks every target to stop first, so all their
 *   waits end together, then destroys them. `~Target` stops the thread and
 *   joins it before it touches anything the thread uses.
 * - **Camera before sender.** The camera waits for its own GPU work, then
 *   destroys its views and framebuffers over the sender's images while those
 *   images still exist. Then the sender goes, then its token.
 * - **Subscription before its state.** `Targets` lives in `RunApp`, longer
 *   than the subscription that writes into it. Cancelling the subscription's
 *   token waits for a running callback, so nothing writes into `Targets`
 *   afterwards.
 * - **Session before device.** QAROS keeps using your device until the
 *   session is gone, not only until the senders are. `RunApp` declares the
 *   device first, so C++ destroys it after the session and the runtime.
 * - **Library last.** `main` calls \ref qar_library_destroy, then
 *   \ref qar_library_unload.
 *
 * \section vulkan_troubleshooting Troubleshooting
 * | The cube says | Cause | What to do |
 * |---|---|---|
 * | `Could not load QAROS. Install QAROS on this machine, then start this app again.` (exit 2) | no runtime in the default lookup; the line before says where it looked | install QAROS, or fix `QAROS_RUNTIME_DIR` |
 * | an error logged by `qar_library_init` (exit 1) | the runtime loaded but did not initialize | read the message; check the QAROS installation |
 * | `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.` (exit 3) | `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE`: the join could not start the Hub, or not reach it | start QAROS from the Start menu, then the cube |
 * | `QAROS did not allow this app to join ...` (exit 4) | `QAR_STATUS_ONBOARDING_APPROVAL_DENIED`: Deny, or a revoked approval | start it again and click **Allow** |
 * | `Nobody answered the approval prompt in QAROS in time. ...` (exit 5) | `QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT` | start it again and answer within 120 s |
 * | `The program answering on this machine is not the installed QAROS ...` (exit 6) | `QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC` | reinstall QAROS; do not retry |
 * | `This app no longer matches what QAROS approved ...` (exit 7) | `QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH`: the executable moved, was copied or re-signed | revoke it in the Visualizer, then start it again |
 * | `` The session's GPU (...) names no graphics GPU with external memory/semaphore support `` (exit 1) | no Vulkan device on the session's adapter has a graphics queue and both extensions | update the GPU driver; check that the GPU is supported |
 * | `Connected. Waiting for rendering targets.` and nothing more | no target app runs in the session, or none answered its sender yet | open the Visualizer, or start a headset |
 * | `Expected separated stereo color/depth layout` | the sender reported another layout than the cube asked for | none in your setup; the cube ends that target and tries it again after 3 s |
 * | no more frames after the "Vulkan Cube" volume was removed | begin-frame reports `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED`; a removed volume never resolves, and the cube does not recreate it | start the cube again |
 * | your own app crashes at exit | the Vulkan device was destroyed before the session | destroy the session first, the device last |
 *
 * \section vulkan_next Next
 * - <a href="/docs/developer-guide/rendering-streams">Rendering Streams</a>:
 *   layouts, hand tracking, the lifecycle of a stream.
 * - <a href="/docs/developer-guide/onboarding-and-sessions#launched-by-qaros-qaros-api-apps">Launched
 *   by QAROS</a>: the contract the cube follows, so an operator can add it to
 *   the Visualizer as a QAROS API app.
 * - <a href="/docs/developer-guide/tutorials/rust/vulkan-rendering">Vulkan
 *   Rendering in Rust</a>: the same cube with the Rust binding.
 */
// clang-format on

#include "renderer.hpp"
#ifndef _WIN32
#include <cerrno>
#include <sys/stat.h>
#include <unistd.h>
#endif
#include <algorithm>
#include <array>
#include <atomic>
#include <chrono>
#include <csignal>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

QAR_IMPLEMENT_DYNAMIC_LOADING()

using Clock = std::chrono::steady_clock;
using PeerKey = std::array<uint8_t, QAR_MAX_ID_LENGTH>;

// Set by Ctrl+C (SIGINT), SIGTERM or a closed stdin pipe. A signal handler can reach nothing but a
// global, and a lock-free atomic is the one thing it may safely write; all SDK
// and GPU cleanup stays on main.
std::atomic<bool> g_stopRequested = false;

extern "C" void
OnStopSignal(int)
{
	g_stopRequested.store(true);
}

// Everything up to main stays internal to this file, which also keeps the
// cube's C++ types out of the C API reference.
namespace
{

bool
StdinIsPipe()
{
#ifdef _WIN32
	return GetFileType(GetStdHandle(STD_INPUT_HANDLE)) == FILE_TYPE_PIPE;
#else
	struct stat status{};
	return fstat(STDIN_FILENO, &status) == 0
		   && (S_ISFIFO(status.st_mode) || S_ISSOCK(status.st_mode));
#endif
}

// QAROS stops an app it started by closing the app's stdin pipe. Whatever
// arrives on the pipe is ignored; its end is the stop request.
void
StopWhenStdinCloses()
{
	std::array<char, 256> buffer{};
#ifdef _WIN32
	const HANDLE input = GetStdHandle(STD_INPUT_HANDLE);
	DWORD read = 0;
	while(ReadFile(input, buffer.data(), static_cast<DWORD>(buffer.size()), &read, nullptr)
		  && read > 0)
	{
	}
#else
	for(;;)
	{
		const ssize_t count = read(STDIN_FILENO, buffer.data(), buffer.size());
		if(count > 0 || (count < 0 && errno == EINTR))
		{
			continue;
		}
		if(count < 0)
		{
			std::cerr << "Reading launcher stdin failed: " << std::strerror(errno) << '\n';
		}
		break;
	}
#endif
	g_stopRequested.store(true);
}

//! [vulkan_waiting]
// A stream that is paused (its receiver's display is not visible), reconnecting
// or has no transfer yet is not broken: keep the sender and try the next frame.
// Recreating it would only start the negotiation over. An app volume that does
// not resolve yet is recoverable too: begin-frame starts no frame then, and a
// frame begins once the volume resolves. qar_result_has_code also matches a
// wrapped cause: begin-frame reports these inside
// QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME.
bool
IsWaiting(QarResult result)
{
	return qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS)
		   || qar_result_has_code(result, QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED);
}
//! [vulkan_waiting]

// Destroys the frame info when it goes out of scope, on every path out of a
// frame.
struct FrameInfoDeleter
{
	void operator()(QarRenderFrameInfo* info) const { qar_render_frame_info_handle_destroy(info); }
};
using FrameInfo = std::unique_ptr<QarRenderFrameInfo, FrameInfoDeleter>;

// What every target renders: one animated scene, seen through each target's
// own cameras.
struct Scene
{
	VulkanDevice* gpu;
	QarSession* session;
	QarAppVolumeId volume;
	Clock::time_point started;
	float Seconds() const { return std::chrono::duration<float>(Clock::now() - started).count(); }
};

// One target app the cube renders for, on a thread of its own. The blocking
// qar_render_sender_begin_frame() paces it: it returns when this target wants
// its next frame, so each target runs at its own display rate and a slow or
// paused one never delays another.
class Target
{
public:
	Target(QarPeerId peer, const Scene& scene)
		: m_peer(peer)
		, m_scene(scene)
		, m_camera(scene.gpu)
	{
		m_ready = Check(qar_cancel_token_create(&m_cancel));
		if(m_ready)
		{
			m_thread = std::thread(&Target::Serve, this);
		}
	}
	Target(const Target&) = delete;
	Target& operator=(const Target&) = delete;
	~Target()
	{
		RequestStop();
		if(m_thread.joinable())
		{
			m_thread.join();
		}
		// The camera drains its own GPU work before the sender's images go.
		m_camera.reset();
		if(m_sender)
		{
			qar_render_stream_handle_destroy(m_sender);
		}
		if(m_cancel)
		{
			qar_cancel_token_handle_destroy(m_cancel);
		}
	}
	// Ends a pending sender creation or begin-frame wait at once.
	void RequestStop()
	{
		m_stop.store(true);
		if(m_cancel)
		{
			Check(qar_cancel_token_cancel(m_cancel));
		}
	}
	bool Finished() const { return not m_ready || m_finished.load(); }
	// The target is gone for good, not just failing for now.
	bool StreamClosed() const { return m_closed.load(); }

private:
	void Serve()
	{
		if(CreateSender() && CreateCamera())
		{
			while(not m_stop.load() && RenderNext())
			{
			}
		}
		m_finished.store(true);
	}

	//! [vulkan_sender]
	bool CreateSender()
	{
		auto vulkan = qar_stream_params_vulkan_default();
		vulkan.physical_device = m_scene.gpu->physical;
		vulkan.device = m_scene.gpu->device;
		auto init = qar_render_sender_init_default();
		init.header.next = &vulkan.header;
		init.graphics_api = QAR_GRAPHICS_API_VULKAN;
		init.peer_id = m_peer;
		init.app_volume_id = &m_scene.volume;
		init.color_format = QAR_PIXEL_FORMAT_R8G8B8A8;
		init.depth_format = QAR_PIXEL_FORMAT_D32_FLOAT;
		init.texture_layout = QAR_FRAME_LAYOUT_SEPARATED_TEXTURES;
		init.frame_views_count = 4;
		for(size_t eye = 0; eye < 2; ++eye)
		{
			for(size_t type = 0; type < 2; ++type)
			{
				const size_t index = eye * 2 + type;
				init.frame_views[index] = qar_render_frame_view_default();
				init.frame_views[index].eye =
					eye == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
				init.frame_views[index].data_type =
					type == 0 ? QAR_VIDEO_FRAME_VIEW_TYPE_COLOR : QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH;
				init.frame_views[index].texture_index = static_cast<uint32_t>(index);
			}
		}
		// Blocks until the target asks for the stream; this thread has
		// nothing else to do meanwhile, and RequestStop() cancels it.
		const QarResult created =
			qar_render_sender_create(m_scene.session, &init, m_cancel, &m_sender);
		if(not m_stop.load())
		{
			NoteClosed(created);
			return Check(created);
		}
		return false;
	}
	//! [vulkan_sender]

	bool CreateCamera()
	{
		if(not m_camera->Create())
		{
			return false;
		}
		char id[64]{};
		if(Check(qar_uuid_to_string(m_peer.data, id, sizeof(id))))
		{
			std::cout << "Rendering for target " << id << '\n';
		}
		return true;
	}

	//! [vulkan_frame]
	// One frame: begin (paced by the target), render, show. Returns false when
	// this target cannot go on.
	bool RenderNext()
	{
		QarRenderFrameInfo* begunInfo = nullptr;
		const QarResult begun = qar_render_sender_begin_frame(m_sender, m_cancel, &begunInfo);
		// The frame info describes the begun frame: keep it until that frame is
		// shown, as the CPU and D3D11 samples do.
		const FrameInfo info(begunInfo);
		if(not qar_result_is_success(begun))
		{
			return ContinueAfterFailedBegin(begun);
		}
		// From here a stop request waits for this frame: a begun frame is
		// shown, even when rendering failed, because the show closes the frame
		// cycle. The show waits for the frame's semaphore, which comes with the
		// record: without the record nothing could signal it, so a failed read
		// ends the target without a show, and its sender goes with it.
		QarVideoFrameVulkan* frame = nullptr;
		if(not Check(qar_render_sender_frame_vulkan(m_sender, &frame)))
		{
			return false;
		}
		if(not RenderInto(*frame, info.get()))
		{
			// The discard signals the semaphore, so the show can close the
			// cycle; the target ends afterwards.
			if(m_camera->Discard(*frame))
			{
				Check(Show());
			}
			else
			{
				std::cerr << "Failed to discard camera frame\n";
			}
			return false;
		}
		const QarResult shown = Show();
		if(IsWaiting(shown))
		{
			return true;
		}
		NoteClosed(shown);
		return Check(shown);
	}

	QarResult Show()
	{
		auto show = qar_render_frame_show_default();
		show.rendered_near_far = { .1f, 10.f };
		return qar_render_sender_show_frame(m_sender, &show);
	}

	bool ContinueAfterFailedBegin(QarResult begun)
	{
		if(m_stop.load())
		{
			// RequestStop() cancelled the wait: not an error to report.
			return false;
		}
		if(IsWaiting(begun))
		{
			// Begin-frame waits through a pause without reporting it, and
			// holds an unresolved app volume about 16 ms before reporting it.
			// A reconnecting or transfer-less stream has no such hold, so wait
			// a moment before the next begin instead of spinning.
			std::this_thread::sleep_for(std::chrono::milliseconds(2));
			return true;
		}
		Check(begun);
		NoteClosed(begun);
		return false;
	}
	//! [vulkan_frame]

	//! [vulkan_views]
	bool RenderInto(QarVideoFrameVulkan& frame, QarRenderFrameInfo* info)
	{
		std::array<CameraView, 2> cameras{};
		for(size_t eye = 0; eye < 2; ++eye)
		{
			// Each eye's pose and field of view belong to the view showing its
			// color. Finding it by eye and type keeps this right whatever order
			// the views were declared in.
			const int view = FindFrameView(
				frame,
				eye == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT,
				QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
			);
			if(view < 0
			   || not Check(qar_render_frame_info_get_view_pose(
				   info, static_cast<size_t>(view), &cameras[eye].pose
			   ))
			   || not Check(qar_render_frame_info_get_view_fov(
				   info, static_cast<size_t>(view), &cameras[eye].fov
			   )))
			{
				return false;
			}
		}
		return m_camera->Render(frame, cameras, m_scene.Seconds());
	}
	//! [vulkan_views]

	void NoteClosed(QarResult result)
	{
		if(qar_result_has_code(result, QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED))
		{
			m_closed.store(true);
		}
	}

	QarPeerId m_peer{};
	Scene m_scene;
	std::optional<CubeCamera> m_camera;
	QarCancelToken* m_cancel = nullptr;
	QarRenderSender* m_sender = nullptr;
	bool m_ready = false;
	std::atomic<bool> m_stop = false;
	std::atomic<bool> m_finished = false;
	std::atomic<bool> m_closed = false;
	std::thread m_thread;
};

//! [vulkan_targets]
// A target can be rendered for unless the hub says it is gone or it announced its leave. UNKNOWN
// is a hub that has not reconciled the peer yet (or an older hub that never does): nothing says
// it is gone, so it counts.
bool
IsReachable(QarPeerPresence presence)
{
	return presence == QAR_PEER_PRESENCE_ONLINE || presence == QAR_PEER_PRESENCE_UNKNOWN;
}

// The cube renders for every running target app in the session: which targets
// to serve is the source app's own choice, and this one serves them all. Each
// sender attaches its target to the cube's app volume, which is what makes the
// target ask for the content.
//
// The peer callback runs on a library thread and hands over IDs only; main
// starts the targets. A plain std::mutex guards the hand-over.
struct Targets
{
	std::mutex lock;
	std::map<PeerKey, QarPeerId> pending;
	// Queues the peer when it is a reachable target app.
	void Offer(QarPeerSpec* spec)
	{
		QarPeerRole role = QAR_PEER_ROLE_INTERNAL;
		if(not Check(qar_peer_spec_get_role(spec, &role)) || role != QAR_PEER_ROLE_TARGET)
		{
			return;
		}
		QarPeerPresence presence = QAR_PEER_PRESENCE_UNKNOWN;
		QarPeerId id{};
		if(not Check(qar_peer_spec_get_presence(spec, &presence)) || not IsReachable(presence)
		   || not Check(qar_peer_spec_get_id(spec, &id)))
		{
			return;
		}
		PeerKey key{};
		std::memcpy(key.data(), id.data, key.size());
		const std::lock_guard<std::mutex> guard(lock);
		pending.insert_or_assign(key, id);
	}
	static void OnPeerUpdate(QarPeerSpec* spec, void* context)
	{
		static_cast<Targets*>(context)->Offer(spec);
	}
	// The peers already in the session when the cube starts.
	bool OfferCurrentPeers(QarSession* session)
	{
		size_t count = 0;
		if(not Check(qar_query_peer_specs_count(session, &count)))
		{
			return false;
		}
		// Right after onboarding the session may know no peer yet; the
		// subscription reports them as they arrive.
		if(count == 0)
		{
			return true;
		}
		std::vector<QarPeerSpec*> specs(count, nullptr);
		size_t written = 0;
		const bool listedAll =
			Check(qar_query_peer_specs(session, specs.data(), specs.size(), &written));
		// A failed query may still have written some specs: destroy those too.
		const size_t owned = std::min(written, specs.size());
		for(size_t index = 0; index < owned; ++index)
		{
			if(specs[index] == nullptr)
			{
				continue;
			}
			if(listedAll)
			{
				Offer(specs[index]);
			}
			qar_peer_spec_handle_destroy(specs[index]);
		}
		return listedAll;
	}
	std::map<PeerKey, QarPeerId> Take()
	{
		std::map<PeerKey, QarPeerId> taken;
		const std::lock_guard<std::mutex> guard(lock);
		taken.swap(pending);
		return taken;
	}
};
//! [vulkan_targets]

// Main only manages targets: starts one for each new target app, restarts one
// that failed after a pause, forgets one whose stream closed for good. The
// frames themselves run on the targets' threads.
bool
Run(QarSession* session, VulkanDevice& gpu, Targets& found)
{
	//! [vulkan_volume]
	auto volumeInit = qar_app_volume_init_default();
	volumeInit.common_name = "vulkan-cube";
	volumeInit.display_name = "Vulkan Cube";
	volumeInit.pose.position.z = -1.5f;
	volumeInit.size = { 1.2f, 1.2f, 1.2f };
	Scene scene{ .gpu = &gpu, .session = session, .started = Clock::now() };
	if(not Check(qar_app_volumes_get_or_create(session, &volumeInit, &scene.volume)))
	{
		return false;
	}
	//! [vulkan_volume]
	//! [vulkan_follow]
	// The SDK requires a token for a subscription. Targets outlives the
	// session, so cancelling at the end of Run is enough. Subscribing before
	// listing the current peers leaves no gap for a target joining in between.
	QarCancelToken* subscription = nullptr;
	if(not Check(qar_cancel_token_create(&subscription)))
	{
		return false;
	}
	if(not Check(qar_peer_subscribe_updates(session, Targets::OnPeerUpdate, &found, subscription))
	   || not found.OfferCurrentPeers(session))
	{
		Check(qar_cancel_token_cancel(subscription));
		qar_cancel_token_handle_destroy(subscription);
		return false;
	}
	std::map<PeerKey, QarPeerId> wanted;
	std::map<PeerKey, std::unique_ptr<Target>> targets;
	std::map<PeerKey, Clock::time_point> retries;
	while(not g_stopRequested.load())
	{
		const auto now = Clock::now();
		for(const auto& [key, id] : found.Take())
		{
			wanted.insert_or_assign(key, id);
		}
		for(const auto& [key, id] : wanted)
		{
			if(not targets.contains(key) && now >= retries[key])
			{
				targets.emplace(key, std::make_unique<Target>(id, scene));
			}
		}
		for(auto it = targets.begin(); it != targets.end();)
		{
			if(not it->second->Finished())
			{
				++it;
				continue;
			}
			if(it->second->StreamClosed())
			{
				wanted.erase(it->first);
			}
			retries[it->first] = now + std::chrono::seconds(3);
			it = targets.erase(it);
		}
		// Control plane only: how soon a new or failed target is noticed.
		std::this_thread::sleep_for(std::chrono::milliseconds(100));
	}
	// Stop every target first so their waits end together, then join them.
	for(auto& [key, target] : targets)
	{
		target->RequestStop();
	}
	targets.clear();
	bool success = Check(qar_cancel_token_cancel(subscription));
	qar_cancel_token_handle_destroy(subscription);
	return success;
	//! [vulkan_follow]
}

//! [vulkan_adapter]
// The GPU the session renders on, as the hex VulkanDevice::Create matches. The
// session is on the GPU of the target QAROS launched this app for, or one the
// library picked; the cube creates its device on the same one so frames stay
// in GPU memory. Empty when the session reports none.
std::string
SessionAdapter(const QarSession* session)
{
	auto id = qar_graphics_device_id_default();
	bool present = false;
	if(not Check(qar_session_get_graphics_device_id(session, &id, &present)) || not present)
	{
		return {};
	}
	std::array<uint8_t, 16> bytes{};
	if(id.id_type == QAR_GPU_DEVICE_ID_TYPE_LUID)
	{
		std::memcpy(bytes.data(), &id.luid, sizeof(id.luid));
		return AdapterIdHex(bytes.data(), sizeof(id.luid));
	}
	std::memcpy(bytes.data(), id.uuid, sizeof(id.uuid));
	return AdapterIdHex(bytes.data(), sizeof(id.uuid));
}
//! [vulkan_adapter]

constexpr const char* kUsage =
	"Usage: qar-vulkan-source\n"
	"\n"
	"Loads the QAROS runtime (the default lookup: the runtime beside this executable,\n"
	"the folder QAROS_RUNTIME_DIR names, the installed QAROS, PATH) and joins the QAROS\n"
	"on this machine as a local app. The first run waits until you click Allow in QAROS.\n"
	"The cube renders on the GPU its session was created on.\n"
	"Stop with Ctrl+C, or by closing stdin when it is a pipe.\n";

void
OnProgress(QarActionSeverity, float, const char* message, void*)
{
	if(message != nullptr && message[0] != '\0')
	{
		std::cout << message << '\n';
	}
}

//! [vulkan_join]
// Cancels the onboarding call when a stop arrives before it finished. Polls
// the stop flag, the one thing a signal handler can set.
void
CancelOnStop(const std::atomic<bool>* finished, QarCancelToken* cancel)
{
	while(not finished->load() && not g_stopRequested.load())
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	if(not finished->load() && not Check(qar_cancel_token_cancel(cancel)))
	{
		std::cerr << "Failed to cancel joining QAROS\n";
	}
}

// Joins the Hub on this machine as a local app: no code, the user approves the
// app once in QAROS. No QarGraphicsDeviceId is chained: the session decides the
// GPU, and the cube creates its device on it. Ctrl+C or a closed stdin cancels
// the call, which can wait minutes for the approval.
QarResult
JoinAsLocalApp(QarRuntime* runtime, QarSession** session)
{
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Vulkan Cube Source";
	init.header.next = &local.header;
	QarCancelToken* cancel = nullptr;
	const QarResult created = qar_cancel_token_create(&cancel);
	if(not qar_result_is_success(created))
	{
		return created;
	}
	std::atomic<bool> finished = false;
	std::thread canceller(CancelOnStop, &finished, cancel);
	QarOnboardingId onboarding{};
	const QarResult result =
		qar_runtime_onboard(runtime, &init, OnProgress, nullptr, cancel, &onboarding, session);
	finished.store(true);
	canceller.join();
	qar_cancel_token_handle_destroy(cancel);
	return result;
}
//! [vulkan_join]

//! [vulkan_explain]
// Tells the user, in plain words, why joining failed and what to do. Returns
// the exit code: each refusal needs a different answer from the user, so each
// has its own code, 3 to 7; anything else is 1.
int
ExplainOnboardingFailure(QarResult result)
{
	int exitCode = 1;
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		std::cerr << "QAROS did not allow this app to join: the approval was denied, or revoked. "
					 "Start the app again to be asked again.\n";
		exitCode = 4;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		std::cerr << "Nobody answered the approval prompt in QAROS in time. It opens in the QAROS "
					 "Visualizer, or as a QAROS tray window. Start the app again.\n";
		exitCode = 5;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		std::cerr
			<< "The program answering on this machine is not the installed QAROS, so this app "
			   "does not trust it and stops. Reinstall QAROS or ask your administrator.\n";
		exitCode = 6;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		std::cerr << "This app no longer matches what QAROS approved: it was moved, copied or "
					 "re-signed. In the QAROS Visualizer, revoke it under Connection > Approved "
					 "apps, then start it again to approve it anew.\n";
		exitCode = 7;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		// The join starts the Hub when it does not run, so this means it could
		// neither start nor reach one.
		std::cerr
			<< "Could not start or reach QAROS on this machine. Start QAROS from the Start menu, "
			   "then start this app again.\n";
		exitCode = 3;
	}
	else
	{
		std::cerr << "Could not join QAROS.\n";
	}
	std::array<char, 512> detail{};
	qar_result_message(result, detail.data(), detail.size());
	std::cerr << "Details: " << detail.data() << '\n';
	return exitCode;
}
//! [vulkan_explain]

//! [vulkan_run_app]
// Everything between loading the library and unloading it. Returns the exit
// code; a stop request is not a failure.
int
RunApp()
{
	VulkanDevice gpu; // Outlives every sender and camera.
	Targets found;	  // Outlives session subscription teardown.
	QarRuntime* runtime = nullptr;
	QarSession* session = nullptr;
	// runtime_binaries_folder_path stays empty: the runtime uses the folder it
	// was loaded from.
	const auto runtimeInit = qar_runtime_init_default();
	int exitCode = Check(qar_runtime_create(&runtimeInit, &runtime)) ? 0 : 1;
	if(exitCode == 0)
	{
		std::cout
			<< "Joining the QAROS on this machine. On the first run, click Allow in the QAROS "
			   "approval prompt.\n";
		const QarResult joined = JoinAsLocalApp(runtime, &session);
		if(not qar_result_is_success(joined) && not g_stopRequested.load())
		{
			exitCode = ExplainOnboardingFailure(joined);
		}
	}
	// Onboard first: the session decides the GPU, and the cube creates its own
	// device on the same one.
	if(session != nullptr && not g_stopRequested.load())
	{
		bool success = gpu.Create(SessionAdapter(session));
		if(success)
		{
			std::cout << "Connected. Waiting for rendering targets. Ctrl+C to quit.\n";
			success = Run(session, gpu, found);
		}
		exitCode = success ? 0 : 1;
	}
	if(session)
	{
		qar_session_handle_destroy(session);
	}
	if(runtime)
	{
		qar_runtime_handle_destroy(runtime);
	}
	return exitCode;
}
//! [vulkan_run_app]

} // namespace

//! [vulkan_main]
int
main(int argc, char** argv)
{
	// Print every line as it happens, also when the output goes to a pipe or a
	// file: the C runtime block-buffers redirected output, and MSVC has no line
	// buffering, so stdout (which std::cout writes through) stays unbuffered.
	std::setvbuf(stdout, nullptr, _IONBF, 0);
	if(argc > 1)
	{
		const bool help =
			std::string_view(argv[1]) == "--help" || std::string_view(argv[1]) == "-h";
		(help ? std::cout : std::cerr) << kUsage;
		return help ? 0 : 1;
	}
	std::signal(SIGINT, OnStopSignal);
	std::signal(SIGTERM, OnStopSignal);
	if(StdinIsPipe())
	{
		std::thread(StopWhenStdinCloses).detach();
	}
	// The header's default lookup: no path to find or pass.
	if(not qar_library_load(nullptr))
	{
		std::cerr
			<< "Could not load QAROS. Install QAROS on this machine, then start this app again.\n";
		return 2;
	}
	auto libraryInit = qar_library_init_default();
	libraryInit.enable_console_logging = true;
	if(not Check(qar_library_init(&libraryInit)))
	{
		qar_library_unload();
		return 1;
	}
	int exitCode = RunApp();
	if(not Check(qar_library_destroy()) && exitCode == 0)
	{
		exitCode = 1;
	}
	qar_library_unload();
	std::signal(SIGINT, SIG_DFL);
	std::signal(SIGTERM, SIG_DFL);
	return exitCode;
}
//! [vulkan_main]
