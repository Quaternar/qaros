#include "renderer.hpp"
#include <array>
#include <atomic>
#include <charconv>
#include <chrono>
#include <csignal>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
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
	return GetFileType(GetStdHandle(STD_INPUT_HANDLE)) == FILE_TYPE_PIPE;
}

// Whether a person can answer a question on stdin.
bool
StdinIsConsole()
{
	DWORD mode = 0;
	return GetConsoleMode(GetStdHandle(STD_INPUT_HANDLE), &mode) != 0;
}

// QAROS stops an app it started by closing the app's stdin pipe. Whatever
// arrives on the pipe is ignored; its end is the stop request.
void
StopWhenStdinCloses(HANDLE input)
{
	std::array<char, 256> buffer{};
	DWORD read = 0;
	while(ReadFile(input, buffer.data(), static_cast<DWORD>(buffer.size()), &read, nullptr)
		  && read > 0)
	{
	}
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
struct Targets
{
	std::mutex lock;
	std::map<PeerKey, QarPeerId> pending;
	// Queues the peer when it is a running target app.
	void Offer(QarPeerSpec* spec)
	{
		bool isTargetApp = false;
		if(not Check(qar_peer_spec_is_target_app(spec, &isTargetApp)) || not isTargetApp)
		{
			return;
		}
		QarAppState state = QAR_APP_STATE_UNKNOWN;
		QarPeerId id{};
		if(not Check(qar_peer_spec_get_app_state(spec, &state)) || state != QAR_APP_STATE_RUNNING
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

// A path as UTF-8, which is what every QAROS C API path is. string() would give
// the ANSI code page instead.
std::string
Utf8(const std::filesystem::path& path)
{
	const std::u8string text = path.u8string();
	return std::string(text.begin(), text.end());
}

// A hub address as host:port. There is no default port: a hub claims its
// onboarding port from a range, so the port must be given.
bool
ParseHubAddress(const std::string& address, std::string& host, uint16_t& port)
{
	const auto colon = address.rfind(':');
	if(colon == std::string::npos || colon == 0)
	{
		return false;
	}
	const std::string portText = address.substr(colon + 1);
	unsigned long value = 0;
	const auto [end, error] =
		std::from_chars(portText.data(), portText.data() + portText.size(), value);
	if(error != std::errc{} || end != portText.data() + portText.size() || value == 0
	   || value > 65535)
	{
		return false;
	}
	host = address.substr(0, colon);
	port = static_cast<uint16_t>(value);
	return true;
}

constexpr const char* kUsage =
	"Usage: qar-vulkan-source [--code [<code>]] [--host <hub-host:port>]\n"
	"                         [--runtime <qar-streaming-c.dll>]\n"
	"\n"
	"Without arguments the app loads the QAROS installed on this PC and joins\n"
	"it as a local app. The first run waits until you click Allow in QAROS.\n"
	"\n"
	"  --code [<code>]    Join with an onboarding code or a full invite (one\n"
	"                     line of JSON) instead, for a hub on another PC.\n"
	"                     Without a value, the app reads it from stdin.\n"
	"  --host <host:port> Hub a typed code joins. Default: this user's hub.\n"
	"  --runtime <path>   Load this qar-streaming-c.dll instead of the\n"
	"                     installed one (development builds of QAROS).\n"
	"\n"
	"The cube renders on the GPU its session was created on.\n"
	"Stop with Ctrl+C, or by closing stdin when it is a pipe.\n";

struct Options
{
	bool help = false;
	bool useCode = false;
	std::string code;
	std::string host;
	std::string runtime;
};

// The value following argv[index], when there is one that is not an option.
bool
HasValue(int argc, char** argv, int index)
{
	return index + 1 < argc && std::string_view(argv[index + 1]).rfind("--", 0) != 0;
}

std::optional<Options>
ParseOptions(int argc, char** argv)
{
	Options options;
	for(int index = 1; index < argc; ++index)
	{
		const std::string_view argument = argv[index];
		const bool hasValue = HasValue(argc, argv, index);
		if(argument == "--help" || argument == "-h")
		{
			options.help = true;
			continue;
		}
		if(argument == "--code")
		{
			options.useCode = true;
			options.code = hasValue ? argv[++index] : "";
			continue;
		}
		if(argument == "--host" && hasValue)
		{
			options.host = argv[++index];
			continue;
		}
		if(argument == "--runtime" && hasValue)
		{
			options.runtime = argv[++index];
			continue;
		}
		std::cerr << "Unknown or incomplete argument: " << argument << "\n\n" << kUsage;
		return std::nullopt;
	}
	if(not options.host.empty() && not options.useCode)
	{
		std::cerr << "--host needs --code: a local app joins the QAROS on this PC.\n";
		return std::nullopt;
	}
	return options;
}

// Loads the installed QAROS runtime, or the library --runtime names.
// `binaries` stays empty for the installed runtime, which finds its own bin/.
bool
LoadRuntime(const Options& options, std::string& binaries)
{
	if(options.runtime.empty())
	{
		if(qar_library_load(nullptr))
		{
			return true;
		}
		std::cerr << "Could not load QAROS. Install QAROS on this PC, then start this app again.\n";
		return false;
	}
	std::error_code error;
	const auto library = std::filesystem::absolute(options.runtime, error);
	if(error)
	{
		std::cerr << "Invalid runtime path: " << error.message() << '\n';
		return false;
	}
	binaries = Utf8(library.parent_path());
	if(qar_library_load(Utf8(library).c_str()))
	{
		return true;
	}
	std::cerr << "Could not load the QAROS runtime " << Utf8(library) << '\n';
	return false;
}

// Reads one line the user typed, false when stdin ended or a stop was
// requested.
bool
AskLine(const char* question, std::string& answer)
{
	std::cout << question << std::flush;
	const bool read = static_cast<bool>(std::getline(std::cin, answer));
	return read && not g_stopRequested.load();
}

// The extension chain of one onboarding call. Members point at each other, so
// it is filled in place and never copied. No QarGraphicsDeviceId is chained:
// the session decides the GPU, and the cube creates its device on it.
struct OnboardRequest
{
	QarOnboardInit init = qar_onboard_init_default();
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardCodeExt code = qar_onboard_code_ext_default();
	QarOnboardInviteExt invite = qar_onboard_invite_ext_default();
	QarOnboardHostExt host = qar_onboard_host_ext_default();
	QarOnboardingInvite* inviteHandle = nullptr;
	std::string hostname;
	OnboardRequest() = default;
	OnboardRequest(const OnboardRequest&) = delete;
	OnboardRequest& operator=(const OnboardRequest&) = delete;
	~OnboardRequest()
	{
		if(inviteHandle != nullptr)
		{
			qar_onboarding_invite_handle_destroy(inviteHandle);
		}
	}
};

// Local app by default: no code, the user approves the app once in QAROS. With
// --code: a full invite, one line of JSON with the hub's address, port and a
// one-time code, what QAROS hands an app it launches with a code (the session
// is then created on the GPU QAROS chose for the app). Anything else is a code
// typed by hand, which finds this user's hub on its own unless --host names
// one.
bool
ChainRequest(OnboardRequest& request, const Options& options)
{
	request.init.presentation.display_name = "Vulkan Cube Source";
	if(not options.useCode)
	{
		request.init.header.next = &request.local.header;
		return true;
	}
	const std::string& line = options.code;
	if(line.front() == '{')
	{
		if(not Check(qar_onboarding_invite_deserialize(
			   reinterpret_cast<const uint8_t*>(line.data()), line.size(), &request.inviteHandle
		   )))
		{
			return false;
		}
		request.invite.invite = request.inviteHandle;
		request.init.header.next = &request.invite.header;
		return true;
	}
	request.code.code = line.c_str();
	request.init.header.next = &request.code.header;
	if(options.host.empty())
	{
		return true;
	}
	if(not ParseHubAddress(options.host, request.hostname, request.host.port))
	{
		std::cerr << "--host must be host:port, for example 192.168.1.20:19121\n";
		return false;
	}
	request.host.hostname = request.hostname.c_str();
	request.code.header.next = &request.host.header;
	return true;
}

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

// One onboarding call that Ctrl+C or a closed stdin can cancel. The approval
// prompt can keep it waiting for minutes.
QarResult
Onboard(QarRuntime* runtime, const OnboardRequest& request, QarSession** session)
{
	QarCancelToken* cancel = nullptr;
	const QarResult created = qar_cancel_token_create(&cancel);
	if(not qar_result_is_success(created))
	{
		return created;
	}
	std::atomic<bool> finished = false;
	std::thread canceller(CancelOnStop, &finished, cancel);
	QarOnboardingId onboarding{};
	const QarResult result = qar_runtime_onboard(
		runtime, &request.init, OnProgress, nullptr, cancel, &onboarding, session
	);
	finished.store(true);
	canceller.join();
	qar_cancel_token_handle_destroy(cancel);
	return result;
}

// Tells the user, in plain words, why joining failed and what to do. Returns
// whether asking again right away can succeed.
bool
ExplainOnboardingFailure(QarResult result, bool usesCode)
{
	bool retry = false;
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		std::cerr << "QAROS did not allow this app to join: the approval was denied, or revoked. "
					 "Start the app again to be asked again.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		std::cerr << "Nobody answered the approval prompt in QAROS in time. It opens in the QAROS "
					 "Visualizer, or as a QAROS tray window.\n";
		retry = true;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		std::cerr << "The program answering on this PC is not the installed QAROS, so this app "
					 "does not trust it and stops. Reinstall QAROS or ask your administrator.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		std::cerr << "This app no longer matches what QAROS approved: it was moved, copied or "
					 "re-signed. In the QAROS Visualizer, revoke it under Connection > Approved "
					 "apps, then start it again to approve it anew.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		std::cerr
			<< (usesCode ? "Could not reach the QAROS hub. Check that it runs, and the "
						   "host:port given with --host.\n"
						 : "QAROS is not running on this PC. Start QAROS from the Start "
						   "menu, then start this app again.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_PAKE_ERROR))
	{
		std::cerr << "The hub rejected the onboarding code. Check the code on the hub, or get a "
					 "new one, and try again.\n";
	}
	else
	{
		std::cerr << "Could not join QAROS.\n";
	}
	std::array<char, 512> detail{};
	qar_result_message(result, detail.data(), detail.size());
	std::cerr << "Details: " << detail.data() << '\n';
	return retry;
}

enum class JoinOutcome
{
	Joined,
	Stopped,
	Failed
};

// Joins QAROS, asking again after an approval timeout while a person is at the
// console.
JoinOutcome
Join(
	QarRuntime* runtime, const OnboardRequest& request, const Options& options, QarSession** session
)
{
	while(true)
	{
		if(not options.useCode)
		{
			std::cout << "Joining the QAROS on this PC. On the first run, click Allow in the QAROS "
						 "approval prompt.\n";
		}
		const QarResult result = Onboard(runtime, request, session);
		if(g_stopRequested.load())
		{
			return JoinOutcome::Stopped;
		}
		if(qar_result_is_success(result))
		{
			return JoinOutcome::Joined;
		}
		const bool retry = ExplainOnboardingFailure(result, options.useCode);
		std::string answer;
		if(not retry || not StdinIsConsole()
		   || not AskLine("Press Enter to ask again, Ctrl+C to quit: ", answer))
		{
			return JoinOutcome::Failed;
		}
	}
}

// The onboarding code or invite from --code, or read from stdin when --code
// has no value.
bool
ReadCode(Options& options)
{
	if(not options.useCode || not options.code.empty())
	{
		return true;
	}
	if(AskLine("Hub onboarding code: ", options.code) && not options.code.empty())
	{
		return true;
	}
	std::cerr << "No onboarding code given.\n";
	return false;
}

// Everything between loading the library and unloading it. False on failure;
// a stop request is not a failure.
bool
RunApp(const Options& options, const std::string& binaries)
{
	VulkanDevice gpu; // Outlives every sender and camera.
	Targets found;	  // Outlives session subscription teardown.
	OnboardRequest request;
	QarRuntime* runtime = nullptr;
	QarSession* session = nullptr;
	auto runtimeInit = qar_runtime_init_default();
	runtimeInit.runtime_binaries_folder_path = binaries.c_str();
	bool success =
		ChainRequest(request, options) && Check(qar_runtime_create(&runtimeInit, &runtime));
	const JoinOutcome joined =
		success ? Join(runtime, request, options, &session) : JoinOutcome::Failed;
	success = joined != JoinOutcome::Failed;
	// Onboard first: the session decides the GPU, and the cube creates its own
	// device on the same one.
	if(joined == JoinOutcome::Joined)
	{
		success = gpu.Create(SessionAdapter(session));
	}
	if(joined == JoinOutcome::Joined && success)
	{
		std::cout << "Connected. Waiting for rendering targets. Ctrl+C to quit.\n";
		success = Run(session, gpu, found);
	}
	if(session)
	{
		qar_session_handle_destroy(session);
	}
	if(runtime)
	{
		qar_runtime_destroy(runtime);
	}
	return success;
}

int
main(int argc, char** argv)
{
	auto parsed = ParseOptions(argc, argv);
	if(not parsed.has_value())
	{
		return 1;
	}
	Options options = parsed.value();
	if(options.help)
	{
		std::cout << kUsage;
		return 0;
	}
	std::signal(SIGINT, OnStopSignal);
	std::signal(SIGTERM, OnStopSignal);
	// Read before the stdin watcher starts, which takes over the pipe.
	if(not ReadCode(options))
	{
		return 1;
	}
	if(StdinIsPipe())
	{
		std::thread(StopWhenStdinCloses, GetStdHandle(STD_INPUT_HANDLE)).detach();
	}
	std::string binaries;
	if(not LoadRuntime(options, binaries))
	{
		return 2;
	}
	auto libraryInit = qar_library_init_default();
	libraryInit.enable_console_logging = true;
	if(not Check(qar_library_init(&libraryInit)))
	{
		qar_library_unload();
		return 3;
	}
	bool success = RunApp(options, binaries);
	if(not Check(qar_library_destroy()))
	{
		success = false;
	}
	qar_library_unload();
	std::signal(SIGINT, SIG_DFL);
	std::signal(SIGTERM, SIG_DFL);
	return success ? 0 : 4;
}
