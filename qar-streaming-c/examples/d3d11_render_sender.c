/** \file d3d11_render_sender.c
 *  \brief Creates a D3D11.1 device on the session's adapter, chains it into a
 *  render sender for the first running target app, and clears each color view
 *  of the shared textures on the GPU.
 *  \example d3d11_render_sender.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_d3d11_rendering D3D11 Rendering
 * \tableofcontents
 *
 * \section d3d11_rendering_build What you build
 * A Windows source application that renders straight into the render sender's
 * shared D3D11 textures, with no CPU copy. It finds the first running target
 * app, creates a D3D11.1 device on the session's adapter, chains that device
 * into a render sender, and clears each eye's color view to a pulsing color:
 * the left eye red, the right eye blue.
 *
 * What you will learn:
 * - how to create the device on the session's adapter
 * - how to hand the device to a sender with QarStreamParamsD3D11, and what
 *   its fields decide
 * - the D3D11 frame cycle: the frame record, the shared textures it lends,
 *   and the status codes that keep it going
 * - which references you hold, and the teardown order
 *
 * Read \ref qar_c_tutorial_cpu_rendering "CPU Rendering with a Visualizer"
 * first. This page assumes its concepts: frame views, targets from the peer
 * list, the frame cycle and the waiting status codes.
 *
 * \section d3d11_rendering_prereq Before you start
 * - **Windows with a D3D11.1-capable GPU**, and QAROS installed on the Hub
 *   machine. The program runs on the Hub machine.
 * - **A target.** At least one target app in the session: open the
 *   Visualizer, or start a headset.
 * - **Joining.** The program joins the Hub on this machine as a local app,
 *   as the CPU program does: click **Allow** in the QAROS approval prompt on
 *   the first run of the executable. Nothing is stored, and later runs join
 *   without a prompt. A pairing code works too; see
 *   \ref d3d11_rendering_code "With a pairing code" at the end of this page.
 * - **This repository built** with CMake: `cmake --preset x64-windows`, then
 *   `cmake --build --preset x64-windows-debug --target d3d11_render_sender`.
 *   The program links `d3d11`, `dxgi` and `dxguid`.
 *
 * \section d3d11_rendering_run Run it
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\d3d11_render_sender.exe --frames=300
 * \endcode
 *
 * The arguments are those of the CPU program: `--frames=N` (300 by default)
 * and `--qaros-library=PATH`. Any other argument prints the usage and exits
 * with 1.
 *
 * Expected output, without the library's own log lines and the approval
 * progress messages:
 * \code{.txt}
 * Loading the QAROS runtime (default lookup)
 * Loaded <path>\qar-streaming-c.dll (the installed QAROS)
 * Runtime <version> (<flavor>, C API <version>)
 * qar_runtime_create: SUCCESS (code=0)
 * No session to rejoin and no pairing code: joining the QAROS Hub on this machine as a local app. On the first run of this executable, click Allow in the QAROS approval prompt (waiting up to 120 s).
 * qar_runtime_onboard: SUCCESS (local app)
 * A local app has no onboarding id to store (it is zero): the next run joins as a local app again, without a prompt while this executable is unchanged.
 * Waiting for a target app in the peer list ...
 * Creating the D3D11 device on <adapter name>
 * qar_render_sender_create: SUCCESS (code=0)
 * Rendering for target <target name> (peer <peer id>)
 * Submitted 300 frames.
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * On the target, the "D3D11 Tutorial Volume" app volume shows the two pulsing
 * eyes.
 *
 * Exit codes are those of the CPU program: 0 when the frames were shown; 1
 * for an unknown argument, a failed startup, a failure after joining (no D3D11 device, no sender),
 * or a frame failure that ended the loop, a stream closed for good included;
 * 2 when no runtime loaded; 3 to 7 for a refused local-app join.
 *
 * \section d3d11_rendering_walkthrough Walkthrough
 *
 * \subsection d3d11_rendering_start 1. Join, and create the app volume
 * The startup is the CPU program's: `example_join_session` from `common.h`
 * rejoins, onboards with a pairing code, or joins as a local app. Then the
 * app volume the stream is shown in:
 * \snippet d3d11_render_sender.c d3d11_volume
 *
 * \subsection d3d11_rendering_target 2. Find the target
 * Like the CPU program, this one finds its target in the peer list, not
 * through a render request: the sender is what attaches a target to your app
 * volume, and only an attached target requests your content. This program
 * serves one target, the first running target app, and reads the list every
 * 50 ms until there is one:
 * \snippet d3d11_render_sender.c d3d11_target
 *
 * A running target app is a peer whose role is `QAR_PEER_ROLE_TARGET` and
 * whose presence is `QAR_PEER_PRESENCE_ONLINE`, or `QAR_PEER_PRESENCE_UNKNOWN`
 * on a Hub that has not reconciled it yet. The CPU program uses the same
 * check from `common.h`:
 * \snippet common.h example_is_running_target
 *
 * Destroy every spec \ref qar_query_peer_specs wrote, also when it returned
 * an error.
 *
 * \subsection d3d11_rendering_device 3. Create the device on the session's adapter
 * The sender's shared textures live on the session's GPU. Create your device
 * on the same adapter, so frames need no copy between GPUs.
 * \ref qar_session_get_graphics_device_id returns the session's adapter, by
 * LUID on Windows. The program finds the DXGI adapter with that LUID:
 * \snippet d3d11_render_sender.c d3d11_adapter
 *
 * When the session names no adapter, the program uses the default hardware
 * adapter. When it names one that DXGI does not list, the program stops: a
 * device on another GPU would cost a copy on every frame.
 *
 * Then it creates a D3D11.1 device on that adapter:
 * \snippet d3d11_render_sender.c d3d11_device
 *
 * `D3D11CreateDevice` takes `D3D_DRIVER_TYPE_UNKNOWN` when you pass an
 * adapter, and `D3D_DRIVER_TYPE_HARDWARE` when you do not. The sender needs
 * the `ID3D11Device1` and `ID3D11DeviceContext1` interfaces, so the program
 * queries the device for them.
 *
 * \subsection d3d11_rendering_sender 4. Hand the device to the sender
 * \snippet d3d11_render_sender.c d3d11_sender
 *
 * - **`graphics_api`** selects `QAR_GRAPHICS_API_D3D11`, and
 *   QarStreamParamsD3D11 goes in `QarRenderSenderInit::header.next`. The
 *   D3D11 path requires exactly that one extension.
 * - **The bind flags** decide how you can use the shared textures. Render
 *   target and shader resource for color let you draw into it and sample it.
 *   Depth stencil for depth lets you use it as your depth buffer. Leave shader
 *   resource off the depth flags: QAROS creates the depth textures as
 *   `DXGI_FORMAT_D32_FLOAT`, which D3D11 cannot bind as a shader resource,
 *   and sender creation then fails with "The parameter is incorrect."
 * - **`acquire_keyed_mutex_sync = true`** lets the library acquire and
 *   release each texture's keyed mutex around the frame cycle. Set it to
 *   false only when you run the keyed-mutex chain yourself; the frame then
 *   lists each texture's keyed mutex with its keys in
 *   `QarVideoFrameD3D11::synchronization`.
 * - **The frame views** are declared as in the CPU program: both eyes'
 *   color side by side in texture 0, depth in texture 1.
 *
 * \ref qar_render_sender_create blocks until the target answers. As in the
 * CPU program, a cancel token with a 10 s timeout bounds that wait: a target
 * that does not answer in time gets no sender, and the program ends with
 * exit code 1.
 *
 * QAROS uses the immediate context you give it, and a D3D11 immediate context
 * is not thread-safe. Use it only on the thread that drives the session and
 * its senders, as this program does, or turn on the device's multithread
 * protection.
 *
 * \subsection d3d11_rendering_frames 5. Begin, clear, show
 * \snippet d3d11_render_sender.c d3d11_frames
 *
 * Each frame is one cycle:
 * \snippet d3d11_render_sender.c d3d11_render_frame
 *
 * The program begins each frame with
 * \ref qar_render_sender_try_begin_frame, as the CPU program does. It does not
 * block: `ready` false means no frame yet, because the target does not want
 * one or the receiver paused the stream. The loop then sleeps 2 ms and polls
 * again, so a target that stops asking for frames never holds the program
 * inside a call. Once a frame is ready,
 * \ref qar_render_sender_frame_d3d11 fills the frame record: the views
 * and the shared textures they lie on. \ref qar_render_sender_show_frame
 * closes the cycle that begin opened, so the program shows even when reading
 * the record failed, as the CPU program does; it then ends the loop and exits
 * with 1. The frame info is destroyed after the show.
 *
 * The program does not read the view poses: a clear needs none. A real
 * renderer reads the pose and field of view of each view with
 * \ref qar_render_frame_info_get_view_pose and
 * \ref qar_render_frame_info_get_view_fov, and draws the scene into the
 * view's rectangle, as the CPU program does.
 *
 * The clear itself:
 * \snippet d3d11_render_sender.c d3d11_clear
 *
 * The textures belong to the sender. The frame lends them for the cycle,
 * with no `AddRef`: do not release them, and do not use them after the show.
 * The program creates a render-target view on the texture and clears the
 * view's rectangle with `ID3D11DeviceContext1::ClearView`. Depth views are
 * left as the sender hands them out. A real renderer writes depth too: QAROS
 * uses it for occlusion and reprojection.
 *
 * Status handling:
 * | Status | What the program does |
 * |---|---|
 * | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED` | keeps the sender; begin-frame waits through a pause, so it mostly shows up as `ready` false |
 * | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING` | keeps the sender, sleeps 2 ms, tries the next frame |
 * | `QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS` | keeps the sender, sleeps 2 ms, tries the next frame |
 * | `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED` | keeps the sender, sleeps 2 ms, tries the next frame |
 * | `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED`, or any other error | prints the error, ends the loop, and exits with 1 |
 *
 * With one target there is nothing to recreate, so a closed stream ends the
 * program. To serve a target again, destroy the sender and create a new one
 * while the target is still in the peer list.
 *
 * \section d3d11_rendering_ownership Ownership and teardown in C
 * \snippet d3d11_render_sender.c d3d11_teardown
 *
 * - **The sender first.** \ref qar_render_stream_handle_destroy ends the
 *   stream. The sender was created with your device and context, so release
 *   them only after it is destroyed.
 * - **Then your device and context**, with `Release`.
 * - **Then the session, the runtime and the library**, in that order.
 * - **One frame info per cycle**, destroyed after the show. The textures in
 *   QarVideoFrameD3D11 are lent: no `Release`.
 * - **The DXGI adapter** you found is yours: release it once the device
 *   exists. The device keeps its own reference.
 *
 * \section d3d11_rendering_troubleshooting Troubleshooting
 * | What you see | Cause | What to do |
 * |---|---|---|
 * | exit 4 or 5 after `joining the QAROS Hub on this machine as a local app` | the approval prompt was denied, closed or not answered within 120 s | run again and click **Allow** |
 * | `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.`, exit 3 | the local-app join could neither start the Hub nor reach it | start QAROS from the Start menu and run again |
 * | stays at `Waiting for a target app in the peer list ...` | no running target app in the session | open the Visualizer or start a headset |
 * | `` The session's adapter (LUID ...) is not among this machine's DXGI adapters `` | the session names an adapter DXGI does not list | check the GPU driver; report it with the LUID |
 * | `D3D11CreateDevice failed (...)` or `D3D11.1 interfaces not available` | the adapter does not support D3D11.1 | use a GPU that does |
 * | `qar_render_sender_create: ERROR` with "The parameter is incorrect." | depth bind flags QAROS cannot create the depth textures with | bind depth as `D3D11_BIND_DEPTH_STENCIL` only |
 * | `qar_render_sender_create: ERROR` with another message | the sender could not be created for this target, or the target did not answer within 10 s | read the message; check that the target is still running |
 * | `qar_render_sender_try_begin_frame: ERROR`, then `Submitted <N> frames.`, exit 1 | the stream closed for good (`QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED`, possibly wrapped in `QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME`) or another failure | read the message; start the program again |
 *
 * \section d3d11_rendering_next Next
 * - <a href="/docs/developer-guide/rendering-streams#the-d3d11-path">The D3D11 path</a>
 *   in Rendering Streams.
 * - <a href="/docs/developer-guide/api-conventions">API Conventions</a>:
 *   handle ownership and the teardown order.
 * - <a href="/docs/developer-guide/tutorials/rust/d3d11-rendering">D3D11 Rendering in Rust</a>:
 *   the same program on the Rust binding.
 *
 * \section d3d11_rendering_code With a pairing code
 * Like the CPU program, it also takes a pairing code as its first argument
 * that does not start with `--`. It passes no Hub address, so the code finds
 * a Hub of your own user, as \ref qar_c_tutorial_onboarding "Onboarding and Rejoin"
 * explains. An app on another machine also names the Hub with a
 * QarOnboardHostExt; that tutorial shows how.
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\d3d11_render_sender.exe <PAIRING-CODE> --frames=300
 * \endcode
 * It stores the onboarding id in `d3d11_render_sender.onboarding-id.txt` in
 * the working directory, and later runs rejoin with it.
 */
// clang-format on

#define COBJMACROS
#include "common.h"

#include <windows.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

#define DEFAULT_FRAME_COUNT 300u
#define TARGET_POLL_INTERVAL_MS 50u
/* How long creating the sender may block before the program gives up. */
#define SENDER_CREATION_TIMEOUT_MS 10000u
/* How long the loop sleeps when no frame was shown, so polling does not spin. */
#define IDLE_WAIT_MS 2u
/* The animation period, in frames. */
#define PULSE_FRAMES 120u
#define NEAR_PLANE 0.1f
#define FAR_PLANE 10.0f
#define TARGET_NAME_SIZE 128u

/* Color of both eyes side by side in texture 0, depth side by side in texture
 * 1, as in the CPU program. */
static const QarRenderFrameView FRAME_VIEWS[] = {
	{ QAR_VIDEO_FRAME_VIEW_TYPE_COLOR, QAR_VIDEO_FRAME_VIEW_EYE_LEFT, 0 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_COLOR, QAR_VIDEO_FRAME_VIEW_EYE_RIGHT, 0 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH, QAR_VIDEO_FRAME_VIEW_EYE_LEFT, 1 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH, QAR_VIDEO_FRAME_VIEW_EYE_RIGHT, 1 },
};

typedef enum FrameOutcome
{
	FRAME_SHOWN,
	FRAME_NOT_SHOWN,
	FRAME_FAILED
} FrameOutcome;

typedef struct D3D11Objects
{
	ID3D11Device1* device;
	ID3D11DeviceContext1* context;
} D3D11Objects;

//! [d3d11_target]
/** \brief The first running target app in the session's peer list.
 *  *out_found stays false when there is none yet. Returns false after printing
 *  why when the list cannot be read. */
static bool
find_running_target(
	QarSession* session, bool* out_found, QarPeerId* out_target, char* out_name, size_t name_size
)
{
	*out_found = false;
	size_t peer_count = 0;
	QarResult counted = qar_query_peer_specs_count(session, &peer_count);
	if(qar_result_is_error(counted))
	{
		log_result("qar_query_peer_specs_count", counted);
		return false;
	}
	if(peer_count == 0)
	{
		return true;
	}

	QarPeerSpec** peers = (QarPeerSpec**)calloc(peer_count, sizeof(*peers));
	if(peers == NULL)
	{
		fprintf(stderr, "Out of memory listing %zu peers\n", peer_count);
		return false;
	}
	size_t written = 0;
	QarResult queried = qar_query_peer_specs(session, peers, peer_count, &written);
	const bool listed_all = qar_result_is_success(queried);
	if(!listed_all)
	{
		log_result("qar_query_peer_specs", queried);
	}
	/* A failed query may still have written some specs: destroy those too. */
	const size_t owned = written < peer_count ? written : peer_count;
	for(size_t i = 0; i < owned; ++i)
	{
		if(peers[i] == NULL)
		{
			continue;
		}
		if(listed_all && !*out_found && example_is_running_target(peers[i])
		   && qar_result_is_success(qar_peer_spec_get_id(peers[i], out_target)))
		{
			*out_found = true;
			if(qar_result_is_error(qar_peer_spec_get_display_name(peers[i], out_name, name_size)))
			{
				out_name[0] = '\0';
			}
		}
		qar_peer_spec_handle_destroy(peers[i]);
	}
	free(peers);
	return listed_all;
}

/** \brief Read the peer list until it shows a running target app. */
static bool
wait_for_target(QarSession* session, QarPeerId* out_target, char* out_name, size_t name_size)
{
	bool found = false;
	while(!found)
	{
		if(!find_running_target(session, &found, out_target, out_name, name_size))
		{
			return false;
		}
		if(!found)
		{
			Sleep(TARGET_POLL_INTERVAL_MS);
		}
	}
	return true;
}
//! [d3d11_target]

//! [d3d11_adapter]
/** \brief The DXGI adapter the session's graphics devices live on. Sets
 *  *out_adapter to NULL when the session names no adapter by LUID: the default
 *  hardware adapter is then used. Returns false after printing why. */
static bool
find_session_adapter(QarSession* session, IDXGIAdapter1** out_adapter)
{
	*out_adapter = NULL;
	QarGraphicsDeviceId device_id = qar_graphics_device_id_default();
	bool present = false;
	QarResult result = qar_session_get_graphics_device_id(session, &device_id, &present);
	if(qar_result_is_error(result))
	{
		log_result("qar_session_get_graphics_device_id", result);
		return false;
	}
	if(!present || device_id.id_type != QAR_GPU_DEVICE_ID_TYPE_LUID)
	{
		printf("The session names no adapter: using the default hardware adapter.\n");
		return true;
	}

	IDXGIFactory1* factory = NULL;
	HRESULT hr = CreateDXGIFactory1(&IID_IDXGIFactory1, (void**)&factory);
	if(FAILED(hr))
	{
		fprintf(stderr, "CreateDXGIFactory1 failed (0x%08lX)\n", (unsigned long)hr);
		return false;
	}
	IDXGIAdapter1* adapter = NULL;
	for(UINT index = 0; SUCCEEDED(IDXGIFactory1_EnumAdapters1(factory, index, &adapter)); ++index)
	{
		DXGI_ADAPTER_DESC1 description;
		if(SUCCEEDED(IDXGIAdapter1_GetDesc1(adapter, &description))
		   && description.AdapterLuid.LowPart == device_id.luid.LowPart
		   && description.AdapterLuid.HighPart == device_id.luid.HighPart)
		{
			printf("Creating the D3D11 device on %ls\n", description.Description);
			*out_adapter = adapter;
			break;
		}
		IDXGIAdapter1_Release(adapter);
	}
	IDXGIFactory1_Release(factory);
	if(*out_adapter == NULL)
	{
		fprintf(
			stderr,
			"The session's adapter (LUID %08lX:%08lX) is not among this machine's DXGI adapters\n",
			(unsigned long)device_id.luid.HighPart,
			(unsigned long)device_id.luid.LowPart
		);
		return false;
	}
	return true;
}
//! [d3d11_adapter]

static void
release_d3d11(D3D11Objects* d3d)
{
	if(d3d->context)
	{
		ID3D11DeviceContext1_Release(d3d->context);
	}
	if(d3d->device)
	{
		ID3D11Device1_Release(d3d->device);
	}
	d3d->context = NULL;
	d3d->device = NULL;
}

//! [d3d11_device]
/** \brief A D3D11.1 device and immediate context on `adapter`, or on the
 *  default hardware adapter when it is NULL. */
static bool
create_d3d11(IDXGIAdapter1* adapter, D3D11Objects* out_d3d)
{
	const D3D_FEATURE_LEVEL levels[] = { D3D_FEATURE_LEVEL_11_1 };
	ID3D11Device* device = NULL;
	ID3D11DeviceContext* context = NULL;
	HRESULT hr = D3D11CreateDevice(
		(IDXGIAdapter*)adapter,
		adapter != NULL ? D3D_DRIVER_TYPE_UNKNOWN : D3D_DRIVER_TYPE_HARDWARE,
		NULL,
		D3D11_CREATE_DEVICE_BGRA_SUPPORT,
		levels,
		1,
		D3D11_SDK_VERSION,
		&device,
		NULL,
		&context
	);
	if(FAILED(hr))
	{
		fprintf(stderr, "D3D11CreateDevice failed (0x%08lX)\n", (unsigned long)hr);
		return false;
	}

	out_d3d->device = NULL;
	out_d3d->context = NULL;
	HRESULT device_hr =
		ID3D11Device_QueryInterface(device, &IID_ID3D11Device1, (void**)&out_d3d->device);
	HRESULT context_hr = ID3D11DeviceContext_QueryInterface(
		context, &IID_ID3D11DeviceContext1, (void**)&out_d3d->context
	);
	ID3D11DeviceContext_Release(context);
	ID3D11Device_Release(device);
	if(FAILED(device_hr) || FAILED(context_hr))
	{
		fprintf(stderr, "D3D11.1 interfaces not available\n");
		release_d3d11(out_d3d);
		return false;
	}
	return true;
}
//! [d3d11_device]

//! [d3d11_clear]
/** \brief Clear every color view of the frame to an animated color: the left
 *  eye red-ish, the right eye blue-ish, both pulsing. */
static void
clear_color_views(
	ID3D11Device1* device,
	ID3D11DeviceContext1* context,
	const QarVideoFrameD3D11* frame,
	size_t frame_index
)
{
	const float pulse = (float)(frame_index % PULSE_FRAMES) / (float)PULSE_FRAMES;
	for(size_t i = 0; i < frame->texture_views_count; ++i)
	{
		const QarVideoFrameView* view = &frame->texture_views[i];
		if(view->data_type != QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
		   || view->texture_index >= frame->textures_count)
		{
			continue;
		}

		/* Lent by the sender for this frame: no AddRef, no Release. */
		ID3D11Texture2D* texture = frame->textures[view->texture_index].texture;
		ID3D11RenderTargetView* render_target = NULL;
		HRESULT hr = ID3D11Device1_CreateRenderTargetView(
			device, (ID3D11Resource*)texture, NULL, &render_target
		);
		if(FAILED(hr))
		{
			fprintf(stderr, "CreateRenderTargetView failed (0x%08lX)\n", (unsigned long)hr);
			continue;
		}

		const bool right_eye = view->eye == QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
		const float color[4] = { right_eye ? 0.1f : pulse, 0.2f, right_eye ? pulse : 0.1f, 1.0f };
		const D3D11_RECT rectangle = {
			(LONG)view->start_x, (LONG)view->start_y, (LONG)view->end_x, (LONG)view->end_y
		};
		ID3D11DeviceContext1_ClearView(context, (ID3D11View*)render_target, color, &rectangle, 1);
		ID3D11RenderTargetView_Release(render_target);
	}
}
//! [d3d11_clear]

//! [d3d11_render_frame]
/** \brief One frame when the sender has one ready: begin, clear the color
 *  views, show. FRAME_NOT_SHOWN while none is ready or the stream is waiting;
 *  FRAME_FAILED when the sender is finished. */
static FrameOutcome
render_frame(QarRenderSender* sender, const D3D11Objects* d3d, size_t frame_index)
{
	QarRenderFrameInfo* frame_info = NULL;
	bool ready = false;
	QarResult begun = qar_render_sender_try_begin_frame(sender, &frame_info, &ready);
	if(example_is_waiting(begun))
	{
		return FRAME_NOT_SHOWN;
	}
	if(qar_result_is_error(begun))
	{
		log_result("qar_render_sender_try_begin_frame", begun);
		return FRAME_FAILED;
	}
	if(!ready)
	{
		return FRAME_NOT_SHOWN;
	}

	QarVideoFrameD3D11 frame = qar_video_frame_d3d11_default();
	QarResult frame_result = qar_render_sender_frame_d3d11(sender, &frame);
	const bool readable = qar_result_is_success(frame_result);
	if(readable)
	{
		clear_color_views(d3d->device, d3d->context, &frame, frame_index);
	}
	else
	{
		log_result("qar_render_sender_frame_d3d11", frame_result);
	}

	/* Show closes the frame cycle that begin-frame opened, so it runs even when
	 * the frame could not be read. The frame info lives until then. */
	QarRenderFrameShow show = qar_render_frame_show_default();
	show.rendered_near_far.near_plane = NEAR_PLANE;
	show.rendered_near_far.far_plane = FAR_PLANE;
	QarResult shown = qar_render_sender_show_frame(sender, &show);
	qar_render_frame_info_handle_destroy(frame_info);
	if(!readable)
	{
		return FRAME_FAILED;
	}
	if(example_is_waiting(shown))
	{
		return FRAME_NOT_SHOWN;
	}
	if(qar_result_is_error(shown))
	{
		log_result("qar_render_sender_show_frame", shown);
		return FRAME_FAILED;
	}
	return FRAME_SHOWN;
}
//! [d3d11_render_frame]

static void
teardown(QarRuntime* runtime, QarSession* session)
{
	if(session != NULL)
	{
		/* Destroying the handle leaves the session. */
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	example_unload_library();
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	static const char* const known_flags[] = { "--frames=", "--qaros-library=" };
	if(!example_arguments_are_known(
		   argc,
		   argv,
		   known_flags,
		   sizeof(known_flags) / sizeof(known_flags[0]),
		   "[pairing-code] [--frames=N] [--qaros-library=PATH]"
	   ))
	{
		return 1;
	}
	size_t frame_limit = 0;
	if(!example_frame_limit_argument(argc, argv, DEFAULT_FRAME_COUNT, &frame_limit))
	{
		fprintf(stderr, "--frames takes a whole number of frames, as in --frames=300\n");
		return 1;
	}
	const char* library_path = example_library_path(argc, argv);
	const char* pairing_code = example_pairing_code_argument(argc, argv);

	int exit_code = example_load_library(library_path);
	if(exit_code != 0)
	{
		return exit_code;
	}
	QarRuntime* runtime = NULL;
	if(!example_create_runtime(NULL, &runtime))
	{
		example_unload_library();
		return 1;
	}
	QarSession* session = NULL;
	exit_code = example_join_session(
		runtime, pairing_code, "d3d11_render_sender.onboarding-id.txt", "D3D11 Renderer", &session
	);
	if(exit_code != 0)
	{
		teardown(runtime, session);
		return exit_code;
	}

	//! [d3d11_volume]
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "d3d11-volume.examples.qaros";
	volume_init.display_name = "D3D11 Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();

	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult volume_result = qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	if(qar_result_is_error(volume_result))
	{
		log_result("qar_app_volumes_get_or_create", volume_result);
		teardown(runtime, session);
		return 1;
	}
	//! [d3d11_volume]

	QarPeerId target_peer = qar_peer_id_default();
	char target_name[TARGET_NAME_SIZE] = { 0 };
	printf("Waiting for a target app in the peer list ...\n");
	if(!wait_for_target(session, &target_peer, target_name, sizeof(target_name)))
	{
		teardown(runtime, session);
		return 1;
	}

	IDXGIAdapter1* adapter = NULL;
	D3D11Objects d3d = { NULL, NULL };
	const bool device_created =
		find_session_adapter(session, &adapter) && create_d3d11(adapter, &d3d);
	if(adapter != NULL)
	{
		IDXGIAdapter1_Release(adapter); /* the device holds its own reference */
	}
	if(!device_created)
	{
		teardown(runtime, session);
		return 1;
	}

	//! [d3d11_sender]
	QarStreamParamsD3D11 d3d11_params = qar_stream_params_d3d11_default();
	d3d11_params.d3d11_device = d3d.device;
	d3d11_params.d3d11_context = d3d.context;
	d3d11_params.color_bind_flags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;
	/* Depth stencil only: QAROS creates the depth textures as the typed
	 * DXGI_FORMAT_D32_FLOAT, which D3D11 cannot bind as a shader resource. With
	 * D3D11_BIND_SHADER_RESOURCE added, sender creation fails. */
	d3d11_params.depth_bind_flags = D3D11_BIND_DEPTH_STENCIL;
	d3d11_params.acquire_keyed_mutex_sync = true;

	QarRenderSenderInit sender_init = qar_render_sender_init_default();
	sender_init.graphics_api = QAR_GRAPHICS_API_D3D11;
	sender_init.peer_id = target_peer;
	sender_init.app_volume_id = &volume_id;
	sender_init.texture_layout = QAR_FRAME_LAYOUT_SIDE_BY_SIDE;
	memcpy(sender_init.frame_views, FRAME_VIEWS, sizeof(FRAME_VIEWS));
	sender_init.frame_views_count = sizeof(FRAME_VIEWS) / sizeof(FRAME_VIEWS[0]);
	sender_init.header.next = &d3d11_params.header;

	/* Creation blocks until the target answers; the timeout keeps a target that
	 * does not answer from holding the program for good. */
	QarRenderSender* sender = NULL;
	QarCancelToken* timeout = NULL;
	QarResult sender_result =
		qar_cancel_token_create_with_timeout(&timeout, SENDER_CREATION_TIMEOUT_MS);
	if(qar_result_is_success(sender_result))
	{
		sender_result = qar_render_sender_create(session, &sender_init, timeout, &sender);
		qar_cancel_token_handle_destroy(timeout);
	}
	log_result("qar_render_sender_create", sender_result);
	if(qar_result_is_error(sender_result) || sender == NULL)
	{
		release_d3d11(&d3d);
		teardown(runtime, session);
		return 1;
	}
	printf(
		"Rendering for target %s (peer ", target_name[0] != '\0' ? target_name : "without a name"
	);
	print_hex_id(target_peer.data, QAR_MAX_ID_LENGTH);
	printf(")\n");
	//! [d3d11_sender]

	//! [d3d11_frames]
	size_t frames_shown = 0;
	bool frame_failed = false;
	while(frames_shown < frame_limit && !frame_failed)
	{
		/* The shown frames drive the animation, not the polls. */
		const FrameOutcome outcome = render_frame(sender, &d3d, frames_shown);
		if(outcome == FRAME_SHOWN)
		{
			++frames_shown;
		}
		else if(outcome == FRAME_NOT_SHOWN)
		{
			Sleep(IDLE_WAIT_MS);
		}
		else
		{
			frame_failed = true;
		}
	}
	printf("Submitted %zu frames.\n", frames_shown);
	//! [d3d11_frames]

	//! [d3d11_teardown]
	qar_render_stream_handle_destroy(sender);
	release_d3d11(&d3d);
	teardown(runtime, session);
	return frame_failed ? 1 : 0;
	//! [d3d11_teardown]
}
