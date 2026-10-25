#include "renderer.hpp"
#ifndef _WIN32
#include <cerrno>
#include <sys/stat.h>
#include <unistd.h>
#endif
#include <array>
#include <atomic>
#include <chrono>
#include <csignal>
#include <cstdint>
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

// A stream that is paused (its receiver's display is not visible), reconnecting
// or has no transfer yet is not broken: keep the sender and try the next frame.
// Recreating it would only start the negotiation over. qar_result_has_code
// also matches a wrapped cause: begin-frame reports these inside
// QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME.
bool
IsWaiting(QarResult result)
{
	return qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS);
}

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
// paused one never delays another. Nothing on the frame path sleeps or polls.
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

	// One frame: begin (paced by the target), render, show. Returns false when
	// this target cannot go on.
	bool RenderNext()
	{
		QarRenderFrameInfo* info = nullptr;
		const QarResult begun = qar_render_sender_begin_frame(m_sender, m_cancel, &info);
		if(m_stop.load())
		{
			qar_render_frame_info_handle_destroy(info);
			return false;
		}
		if(IsWaiting(begun))
		{
			// The receiver is not taking frames right now. No sleep needed:
			// begin-frame waits through a pause and holds a reconnecting or
			// transfer-less stream ~16 ms before reporting it.
			return true;
		}
		if(not Check(begun))
		{
			NoteClosed(begun);
			return false;
		}
		QarVideoFrameVulkan* frame = nullptr;
		bool rendered =
			Check(qar_render_sender_frame_vulkan(m_sender, &frame)) && RenderInto(*frame, info);
		qar_render_frame_info_handle_destroy(info);
		if(not rendered)
		{
			if(frame != nullptr && not m_camera->Discard(*frame))
			{
				std::cerr << "Failed to discard camera frame\n";
			}
			return false;
		}
		auto show = qar_render_frame_show_default();
		show.rendered_near_far = { .1f, 10.f };
		const QarResult shown = qar_render_sender_show_frame(m_sender, &show);
		if(IsWaiting(shown))
		{
			return true;
		}
		NoteClosed(shown);
		return Check(shown);
	}

	bool RenderInto(QarVideoFrameVulkan& frame, QarRenderFrameInfo* info)
	{
		std::array<CameraView, 2> cameras{};
		for(size_t eye = 0; eye < 2; ++eye)
		{
			// The eye's color view: the sender orders the frame's views.
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

// The cube renders for every running target app in the session: which targets
// to serve is the source app's own choice, and this one serves them all. Each
// sender attaches its target to the cube's app volume, which is what makes the
// target ask for the content.
//
// The peer callback hands over IDs only; main starts the targets. A plain
// std::mutex, because this standalone SDK example cannot depend on
// qar::Synchronized and must stay portable to Linux.
// A target can be rendered for unless the hub says it is gone or it announced its leave. UNKNOWN
// is a hub that has not reconciled the peer yet (or an older hub that never does): nothing says
// it is gone, so it counts.
static bool
IsReachable(QarPeerPresence presence)
{
	return presence == QAR_PEER_PRESENCE_ONLINE || presence == QAR_PEER_PRESENCE_UNKNOWN;
}

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
		if(not Check(qar_query_peer_specs(session, specs.data(), specs.size(), &written)))
		{
			return false;
		}
		for(size_t index = 0; index < written; ++index)
		{
			Offer(specs[index]);
			qar_peer_spec_handle_destroy(specs[index]);
		}
		return true;
	}
	std::map<PeerKey, QarPeerId> Take()
	{
		std::map<PeerKey, QarPeerId> taken;
		const std::lock_guard<std::mutex> guard(lock);
		taken.swap(pending);
		return taken;
	}
};

// Main only manages targets: starts one for each new target app, restarts one
// that failed after a pause, forgets one whose stream closed for good. The
// frames themselves run on the targets' threads.
bool
Run(QarSession* session, VulkanDevice& gpu, Targets& found)
{
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
}

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

// The cube's app tag: an identity key, constant across versions, so every
// start of this approved executable gets the same peer id back from the hub.
// Printable ASCII, a DNS-style name scoped by the app.
constexpr const char* kAppTag = "vulkan-cube.examples.qaros";

constexpr const char* kUsage =
	"Usage: qar-vulkan-source\n"
	"\n"
	"Loads the QAROS runtime (qar_library_load(NULL): one beside this executable,\n"
	"else the one of the QAROS instance QAR_INSTANCE names, else the installed\n"
	"QAROS, else one on PATH) and joins the QAROS on this PC as a local app with the\n"
	"app tag "
	"vulkan-cube.examples.qaros. The first run waits until you click Allow in QAROS;\n"
	"later runs join without asking and keep their peer id.\n"
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

// Joins the QAROS on this PC as a local app: no code, the user approves the app
// once in QAROS. The app tag makes the hub hand back the same peer id on every
// start. No QarGraphicsDeviceId is chained: the session decides the GPU, and
// the cube creates its device on it. Ctrl+C or a closed stdin cancels the call,
// which can wait minutes for the approval.
QarResult
JoinAsLocalApp(QarRuntime* runtime, QarSession** session)
{
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tag = qar_onboard_local_app_tag_ext_default();
	tag.app_tag = kAppTag;
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Vulkan Cube Source";
	init.header.next = &local.header;
	local.header.next = &tag.header;
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

// Prints the peer id the hub gave this app. Thanks to the app tag it is the
// same on every start, so the user (and a test) can see the restart kept it.
void
PrintMyPeerId(const QarSession* session)
{
	QarPeerSpec* me = nullptr;
	if(not Check(qar_session_get_my_spec(session, &me)))
	{
		return;
	}
	QarPeerId id = qar_peer_id_default();
	std::array<char, 64> text{};
	if(Check(qar_peer_spec_get_id(me, &id))
	   && Check(qar_uuid_to_string(id.data, text.data(), text.size())))
	{
		std::cout << "Joined QAROS as peer " << text.data() << '\n';
	}
	qar_peer_spec_handle_destroy(me);
}

// Tells the user, in plain words, why joining failed and what to do.
void
ExplainOnboardingFailure(QarResult result)
{
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		std::cerr << "QAROS did not allow this app to join: the approval was denied, or revoked. "
					 "Start the app again to be asked again.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		std::cerr << "Nobody answered the approval prompt in QAROS in time. It opens in the QAROS "
					 "Visualizer, or as a QAROS tray window. Start the app again.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		std::cerr << "The program answering on this PC is not the installed QAROS, so this app "
					 "does not trust it and stops. Reinstall QAROS or ask your administrator.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		std::cerr << "This app no longer matches what QAROS approved: it was moved, copied or "
					 "re-signed. In the QAROS Visualizer, revoke it under Source Applications > "
					 "Approved apps, then start it again to approve it anew.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		std::cerr << "QAROS is not running on this PC. Start QAROS from the Start menu, then start "
					 "this app again.\n";
	}
	else
	{
		std::cerr << "Could not join QAROS.\n";
	}
	std::array<char, 512> detail{};
	qar_result_message(result, detail.data(), detail.size());
	std::cerr << "Details: " << detail.data() << '\n';
}

// Everything between loading the library and unloading it. False on failure;
// a stop request is not a failure.
bool
RunApp()
{
	VulkanDevice gpu; // Outlives every sender and camera.
	Targets found;	  // Outlives session subscription teardown.
	QarRuntime* runtime = nullptr;
	QarSession* session = nullptr;
	// runtime_binaries_folder_path stays empty: the runtime uses the folder it
	// was loaded from.
	const auto runtimeInit = qar_runtime_init_default();
	bool success = Check(qar_runtime_create(&runtimeInit, &runtime));
	if(success)
	{
		std::cout << "Joining the QAROS on this PC. On the first run, click Allow in the QAROS "
					 "approval prompt.\n";
		const QarResult joined = JoinAsLocalApp(runtime, &session);
		success = qar_result_is_success(joined) || g_stopRequested.load();
		if(not success)
		{
			ExplainOnboardingFailure(joined);
		}
	}
	// Onboard first: the session decides the GPU, and the cube creates its own
	// device on the same one.
	if(session != nullptr && not g_stopRequested.load())
	{
		PrintMyPeerId(session);
		success = gpu.Create(SessionAdapter(session));
		if(success)
		{
			std::cout << "Connected. Waiting for rendering targets. Ctrl+C to quit.\n";
			success = Run(session, gpu, found);
		}
	}
	if(session)
	{
		qar_session_handle_destroy(session);
	}
	if(runtime)
	{
		qar_runtime_handle_destroy(runtime);
	}
	return success;
}

int
main(int argc, char** argv)
{
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
		std::cerr << "Could not load QAROS. Install QAROS on this PC, then start this app again.\n";
		return 2;
	}
	auto libraryInit = qar_library_init_default();
	libraryInit.enable_console_logging = true;
	if(not Check(qar_library_init(&libraryInit)))
	{
		qar_library_unload();
		return 3;
	}
	bool success = RunApp();
	if(not Check(qar_library_destroy()))
	{
		success = false;
	}
	qar_library_unload();
	std::signal(SIGINT, SIG_DFL);
	std::signal(SIGTERM, SIG_DFL);
	return success ? 0 : 4;
}
