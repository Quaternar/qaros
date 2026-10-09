#include "renderer.hpp"
#include <array>
#include <atomic>
#include <chrono>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <thread>

QAR_IMPLEMENT_DYNAMIC_LOADING()

using Clock = std::chrono::steady_clock;
using PeerKey = std::array<uint8_t, QAR_MAX_ID_LENGTH>;

std::wstring
StopEventName()
{
	return L"Local\\QarosVulkanCubeStop-"
		   + std::to_wstring(GetCurrentProcessId());
}

// Stop requests only signal an event; all SDK and GPU cleanup stays on main.
void
SignalStop()
{
	auto stop = OpenEventW(EVENT_MODIFY_STATE, FALSE, StopEventName().c_str());
	if(stop)
	{
		SetEvent(stop);
		CloseHandle(stop);
	}
}

BOOL WINAPI
OnConsoleEvent(DWORD event)
{
	if(event != CTRL_C_EVENT && event != CTRL_BREAK_EVENT)
	{
		return FALSE;
	}
	SignalStop();
	return TRUE;
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
	while(ReadFile(
			  input,
			  buffer.data(),
			  static_cast<DWORD>(buffer.size()),
			  &read,
			  nullptr
		  )
		  && read > 0)
	{
	}
	SignalStop();
}

struct Target
{
	QarPeerId peer{};
	QarRenderSenderInit init{};
	QarStreamParamsVulkan vulkan{};
	QarCancelToken* cancel = nullptr;
	QarRenderSender* sender = nullptr;
	QarResult result{};
	QarResult frameResult{};
	std::atomic<bool> created = false;
	bool showing = false;
	std::optional<CubeCamera> camera;
	explicit Target(QarPeerId id)
		: peer(id)
	{
	}
	~Target()
	{
		// Camera drains its own work before framebuffer attachments disappear.
		camera.reset();
		if(sender)
		{
			qar_render_stream_handle_destroy(sender);
		}
		if(cancel)
		{
			qar_cancel_token_handle_destroy(cancel);
		}
	}
	static void
	OnCreated(QarResult status, QarRenderSender* stream, void* context)
	{
		auto& target = *static_cast<Target*>(context);
		target.result = status;
		target.sender = stream;
		target.created.store(true, std::memory_order_release);
	}
	bool
	Start(QarSession* session, VulkanDevice& gpu, const QarAppVolumeId& volume)
	{
		if(not Check(qar_cancel_token_create(&cancel)))
		{
			return false;
		}
		init = qar_render_sender_init_default();
		vulkan = qar_stream_params_vulkan_default();
		vulkan.physical_device = gpu.physical;
		vulkan.device = gpu.device;
		init.header.next = &vulkan.header;
		init.graphics_api = QAR_GRAPHICS_API_VULKAN;
		init.peer_id = peer;
		init.app_volume_id = &volume;
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
					eye == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT
							 : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
				init.frame_views[index].data_type =
					type == 0 ? QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
							  : QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH;
				init.frame_views[index].texture_index =
					static_cast<uint32_t>(index);
			}
		}
		return Check(qar_render_sender_create_async(
			session, &init, OnCreated, this, cancel
		));
	}
	bool Step(VulkanDevice& gpu, float seconds)
	{
		if(not created.load(std::memory_order_acquire))
		{
			return true;
		}
		if(not Check(result))
		{
			return false;
		}
		if(not camera.has_value())
		{
			camera.emplace(&gpu);
			if(not camera->Create())
			{
				return false;
			}
			char id[64]{};
			if(not Check(qar_uuid_to_string(peer.data, id, sizeof(id))))
			{
				return false;
			}
			std::cout << "Camera and Vulkan render sender ready for " << id
					  << '\n';
		}
		if(showing)
		{
			bool ready = false;
			if(not camera->Complete(ready))
			{
				return false;
			}
			if(not ready)
			{
				return true;
			}
			showing = false;
			auto show = qar_render_frame_show_default();
			show.rendered_near_far = { .1f, 10.f };
			frameResult = qar_render_sender_show_frame(sender, &show);
			return Check(frameResult);
		}
		QarRenderFrameInfo* info = nullptr;
		bool ready = false;
		frameResult = qar_render_sender_try_begin_frame(sender, &info, &ready);
		if(qar_result_has_code(
			   frameResult, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING
		   )
		   || qar_result_has_code(
			   frameResult, QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS
		   ))
		{
			return true;
		}
		if(not Check(frameResult))
		{
			return false;
		}
		if(not ready)
		{
			return true;
		}
		QarVideoFrameVulkan* frame = nullptr;
		bool success = Check(qar_render_sender_frame_vulkan(sender, &frame));
		std::array<CameraView, 2> cameras{};
		for(size_t eye = 0; success && eye < 2; ++eye)
		{
			// The eye's color view: the sender orders the frame's views.
			const int view = FindFrameView(
				*frame,
				eye == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT
						 : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT,
				QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
			);
			success = view >= 0
					  && Check(qar_render_frame_info_get_view_pose(
						  info, static_cast<size_t>(view), &cameras[eye].pose
					  ))
					  && Check(qar_render_frame_info_get_view_fov(
						  info, static_cast<size_t>(view), &cameras[eye].fov
					  ));
		}
		if(success)
		{
			success = camera->Submit(*frame, cameras, seconds);
		}
		if(not success && frame && not camera->Discard(*frame))
		{
			std::cerr << "Failed to discard camera frame\n";
		}
		qar_render_frame_info_handle_destroy(info);
		showing = success;
		return success;
	}
};

// The callback hands over IDs only. Sender creation and GPU work stay on main.
// SRWLOCK is used because this standalone SDK example cannot depend on
// qar::Synchronized.
struct Requests
{
	SRWLOCK lock = SRWLOCK_INIT;
	std::map<PeerKey, QarPeerId> pending;
	static void OnRequest(QarRenderStreamRequest* request, void* context)
	{
		QarPeerId id{};
		const bool valid =
			Check(qar_render_request_get_target_peer_id(request, &id));
		qar_render_request_handle_destroy(request);
		if(not valid)
		{
			return;
		}
		auto& requests = *static_cast<Requests*>(context);
		PeerKey key{};
		std::memcpy(key.data(), id.data, key.size());
		AcquireSRWLockExclusive(&requests.lock);
		requests.pending.insert_or_assign(key, id);
		ReleaseSRWLockExclusive(&requests.lock);
	}
	std::map<PeerKey, QarPeerId> Take()
	{
		std::map<PeerKey, QarPeerId> taken;
		AcquireSRWLockExclusive(&lock);
		taken.swap(pending);
		ReleaseSRWLockExclusive(&lock);
		return taken;
	}
};

bool
Run(QarSession* session, VulkanDevice& gpu, HANDLE stop, Requests& requests)
{
	auto volumeInit = qar_app_volume_init_default();
	volumeInit.common_name = "vulkan-cube";
	volumeInit.display_name = "Vulkan Cube";
	volumeInit.pose.position.z = -1.5f;
	volumeInit.size = { 1.2f, 1.2f, 1.2f };
	QarAppVolumeId volume{};
	if(not Check(qar_app_volumes_get_or_create(session, &volumeInit, &volume)))
	{
		return false;
	}
	// The SDK requires a token for a subscription. Requests outlives the
	// session, so cancelling at the end of Run is enough.
	QarCancelToken* subscription = nullptr;
	if(not Check(qar_cancel_token_create(&subscription)))
	{
		return false;
	}
	if(not Check(qar_render_sender_subscribe_requests(
		   session, Requests::OnRequest, &requests, subscription
	   )))
	{
		qar_cancel_token_handle_destroy(subscription);
		return false;
	}
	std::map<PeerKey, QarPeerId> wanted;
	std::map<PeerKey, std::unique_ptr<Target>> targets;
	std::map<PeerKey, Clock::time_point> retries;
	const auto started = Clock::now();
	bool success = true;
	while(WaitForSingleObject(stop, 0) == WAIT_TIMEOUT)
	{
		const auto now = Clock::now();
		for(const auto& [key, id] : requests.Take())
		{
			wanted.insert_or_assign(key, id);
		}
		for(const auto& [key, id] : wanted)
		{
			if(targets.contains(key) || now < retries[key])
			{
				continue;
			}
			auto target = std::make_unique<Target>(id);
			if(not target->Start(session, gpu, volume))
			{
				success = false;
				break;
			}
			targets.emplace(key, std::move(target));
		}
		const float seconds =
			std::chrono::duration<float>(now - started).count();
		for(auto it = targets.begin(); it != targets.end();)
		{
			auto& target = *it->second;
			if(not target.created.load(std::memory_order_acquire))
			{
				++it;
				continue;
			}
			const bool healthy = target.Step(gpu, seconds);
			if(healthy)
			{
				++it;
				continue;
			}
			if(qar_result_has_code(
				   target.result, QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED
			   )
			   || qar_result_has_code(
				   target.frameResult,
				   QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED
			   ))
			{
				wanted.erase(it->first);
			}
			retries[it->first] = now + std::chrono::seconds(3);
			it = targets.erase(it);
		}
		if(not success)
		{
			break;
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
	}
	// Cancellation completion owns Target's callback state until its last
	// callback.
	for(auto& [key, target] : targets)
	{
		if(not Check(qar_cancel_token_cancel(target->cancel)))
		{
			success = false;
		}
	}
	for(auto& [key, target] : targets)
	{
		while(not target->created.load(std::memory_order_acquire))
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
		}
	}
	targets.clear();
	if(not Check(qar_cancel_token_cancel(subscription)))
	{
		success = false;
	}
	qar_cancel_token_handle_destroy(subscription);
	return success;
}

// The GPU QAROS asks this app to render on, set when QAROS launches it. Empty
// when the variable is absent or empty.
std::string
RequestedAdapter()
{
	char* value = nullptr;
	size_t length = 0;
	if(_dupenv_s(&value, &length, "QAR_GPU_ADAPTER_ID") != 0
	   || value == nullptr)
	{
		return {};
	}
	std::string adapter(value);
	free(value);
	return adapter;
}

constexpr const char* kUsage =
	"Usage: qar-vulkan-source [--code [<code>]] [--host <hub-host>]\n"
	"                         [--runtime <qar-streaming-c.dll>]\n"
	"\n"
	"Without arguments the app loads the QAROS installed on this PC and joins\n"
	"it as a local app. The first run waits until you click Allow in QAROS.\n"
	"\n"
	"  --code [<code>]    Join with an onboarding code instead, for a hub on\n"
	"                     another PC. Without a value, the app asks for it.\n"
	"  --host <hub-host>  Host name or IP of that hub. Default: this PC.\n"
	"  --runtime <path>   Load this qar-streaming-c.dll instead of the\n"
	"                     installed one (development builds of QAROS).\n"
	"\n"
	"QAR_GPU_ADAPTER_ID=<LUID or UUID hex> selects the GPU.\n"
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
	return index + 1 < argc
		   && std::string_view(argv[index + 1]).rfind("--", 0) != 0;
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
		std::cerr << "Unknown or incomplete argument: " << argument << "\n\n"
				  << kUsage;
		return std::nullopt;
	}
	if(not options.host.empty() && not options.useCode)
	{
		std::cerr << "--host needs --code: a local app joins the QAROS on "
					 "this PC.\n";
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
		std::cerr << "Could not load QAROS. Install QAROS on this PC, then "
					 "start this app again.\n";
		return false;
	}
	std::error_code error;
	const auto library = std::filesystem::absolute(options.runtime, error);
	if(error)
	{
		std::cerr << "Invalid runtime path: " << error.message() << '\n';
		return false;
	}
	binaries = library.parent_path().string();
	if(qar_library_load(library.string().c_str()))
	{
		return true;
	}
	std::cerr << "Could not load the QAROS runtime " << library.string()
			  << '\n';
	return false;
}

// Reads one line the user typed, false when stdin ended or Ctrl+C was
// pressed.
bool
AskLine(const char* question, HANDLE stop, std::string& answer)
{
	std::cout << question << std::flush;
	const bool read = static_cast<bool>(std::getline(std::cin, answer));
	return read && WaitForSingleObject(stop, 0) == WAIT_TIMEOUT;
}

std::optional<LUID>
DeviceLuid(const VulkanDevice& gpu)
{
	VkPhysicalDeviceIDProperties id{
		VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES
	};
	VkPhysicalDeviceProperties2 properties{
		VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2
	};
	properties.pNext = &id;
	vkGetPhysicalDeviceProperties2(gpu.physical, &properties);
	if(not id.deviceLUIDValid)
	{
		return std::nullopt;
	}
	LUID luid{};
	std::memcpy(&luid, id.deviceLUID, VK_LUID_SIZE);
	return luid;
}

// The extension chain of one onboarding call. Members point at each other, so
// it is filled in place and never copied.
struct OnboardRequest
{
	QarOnboardInit init = qar_onboard_init_default();
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardCodeExt code = qar_onboard_code_ext_default();
	QarOnboardHostExt host = qar_onboard_host_ext_default();
	QarGraphicsDeviceId adapter = qar_graphics_device_id_default();
	OnboardRequest() = default;
	OnboardRequest(const OnboardRequest&) = delete;
	OnboardRequest& operator=(const OnboardRequest&) = delete;
};

// Local app by default: no code, the user approves the app once in QAROS. With
// --code: the onboarding code, and the hub's host when one is given.
void
ChainRequest(
	OnboardRequest& request,
	const Options& options,
	const std::string& code,
	const LUID& luid
)
{
	request.init.presentation.display_name = "Vulkan Cube Source";
	request.adapter.id_type = QAR_GPU_DEVICE_ID_TYPE_LUID;
	request.adapter.luid = luid;
	QarStructureHeader* mode = &request.local.header;
	if(options.useCode)
	{
		request.code.code = code.c_str();
		mode = &request.code.header;
	}
	request.init.header.next = mode;
	mode->next = &request.adapter.header;
	if(options.useCode && not options.host.empty())
	{
		request.host.hostname = options.host.c_str();
		request.adapter.header.next = &request.host.header;
	}
}

void
OnProgress(QarActionSeverity, float, const char* message, void*)
{
	if(message != nullptr && message[0] != '\0')
	{
		std::cout << message << '\n';
	}
}

// Cancels the onboarding call when a stop arrives before it finished.
void
CancelOnStop(HANDLE stop, HANDLE finished, QarCancelToken* cancel)
{
	const std::array<HANDLE, 2> events{ stop, finished };
	const DWORD signaled = WaitForMultipleObjects(
		static_cast<DWORD>(events.size()), events.data(), FALSE, INFINITE
	);
	if(signaled == WAIT_OBJECT_0 && not Check(qar_cancel_token_cancel(cancel)))
	{
		std::cerr << "Failed to cancel joining QAROS\n";
	}
}

// One onboarding call that Ctrl+C or a closed stdin can cancel. The approval
// prompt can keep it waiting for minutes.
QarResult
Onboard(
	QarRuntime* runtime,
	const OnboardRequest& request,
	HANDLE stop,
	QarSession** session
)
{
	QarCancelToken* cancel = nullptr;
	const QarResult created = qar_cancel_token_create(&cancel);
	if(not qar_result_is_success(created))
	{
		return created;
	}
	HANDLE finished = CreateEventW(nullptr, TRUE, FALSE, nullptr);
	std::thread canceller(CancelOnStop, stop, finished, cancel);
	QarOnboardingId onboarding{};
	const QarResult result = qar_runtime_onboard(
		runtime,
		&request.init,
		OnProgress,
		nullptr,
		cancel,
		&onboarding,
		session
	);
	if(finished)
	{
		SetEvent(finished);
	}
	canceller.join();
	if(finished)
	{
		CloseHandle(finished);
	}
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
		std::cerr << "QAROS did not allow this app to join: the approval was "
					 "denied, or revoked. Start the app again to be asked "
					 "again.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		std::cerr << "Nobody answered the approval prompt in QAROS in time. "
					 "It opens in the QAROS Visualizer, or as a QAROS tray "
					 "window.\n";
		retry = true;
	}
	else if(qar_result_has_code(
				result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC
			))
	{
		std::cerr << "The program answering on this PC is not the installed "
					 "QAROS, so this app does not trust it and stops. "
					 "Reinstall QAROS or ask your administrator.\n";
	}
	else if(qar_result_has_code(
				result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH
			))
	{
		std::cerr << "This app no longer matches what QAROS approved: it was "
					 "moved, copied or re-signed. In the QAROS Visualizer, "
					 "revoke it under Connection > Approved apps, then start "
					 "it again to approve it anew.\n";
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		std::cerr
			<< (usesCode ? "Could not reach the QAROS hub. Check that "
						   "it runs, and the host name given with "
						   "--host.\n"
						 : "QAROS is not running on this PC. Start "
						   "QAROS from the Start menu, then start this "
						   "app again.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_PAKE_ERROR))
	{
		std::cerr << "The hub rejected the onboarding code. Check the code "
					 "on the hub, or get a new one, and try again.\n";
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
	QarRuntime* runtime,
	const OnboardRequest& request,
	const Options& options,
	HANDLE stop,
	QarSession** session
)
{
	while(true)
	{
		if(not options.useCode)
		{
			std::cout << "Joining the QAROS on this PC. On the first run, "
						 "click Allow in the QAROS approval prompt.\n";
		}
		const QarResult result = Onboard(runtime, request, stop, session);
		if(WaitForSingleObject(stop, 0) != WAIT_TIMEOUT)
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
		   || not AskLine(
			   "Press Enter to ask again, Ctrl+C to quit: ", stop, answer
		   ))
		{
			return JoinOutcome::Failed;
		}
	}
}

// The onboarding code from --code, or asked for when --code has no value.
bool
ReadCode(Options& options, HANDLE stop)
{
	if(not options.useCode || not options.code.empty())
	{
		return true;
	}
	if(AskLine("Hub onboarding code: ", stop, options.code)
	   && not options.code.empty())
	{
		return true;
	}
	std::cerr << "No onboarding code given.\n";
	return false;
}

// Everything between loading the library and unloading it. False on failure;
// a stop request is not a failure.
bool
RunApp(const Options& options, const std::string& binaries, HANDLE stop)
{
	VulkanDevice gpu;
	Requests requests; // Outlives session subscription teardown.
	QarRuntime* runtime = nullptr;
	QarSession* session = nullptr;
	auto runtimeInit = qar_runtime_init_default();
	runtimeInit.runtime_binaries_folder_path = binaries.c_str();
	bool success = gpu.Create(RequestedAdapter())
				   && Check(qar_runtime_create(&runtimeInit, &runtime));
	const auto luid = success ? DeviceLuid(gpu) : std::nullopt;
	if(success && not luid.has_value())
	{
		std::cerr << "GPU has no valid Windows LUID\n";
		success = false;
	}
	if(success)
	{
		OnboardRequest request;
		ChainRequest(request, options, options.code, luid.value());
		const JoinOutcome joined =
			Join(runtime, request, options, stop, &session);
		success = joined != JoinOutcome::Failed;
		if(joined == JoinOutcome::Joined)
		{
			std::cout << "Connected. Waiting for rendering targets. Ctrl+C to "
						 "quit.\n";
			success = Run(session, gpu, stop, requests);
		}
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
	auto stop = CreateEventW(nullptr, TRUE, FALSE, StopEventName().c_str());
	if(not stop || not SetConsoleCtrlHandler(OnConsoleEvent, TRUE))
	{
		std::cerr << "Failed to set up the stop handling\n";
		return 1;
	}
	// Read before the stdin watcher starts, which takes over the pipe.
	if(not ReadCode(options, stop))
	{
		return 1;
	}
	if(StdinIsPipe())
	{
		std::thread(StopWhenStdinCloses, GetStdHandle(STD_INPUT_HANDLE))
			.detach();
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
	bool success = RunApp(options, binaries, stop);
	if(not Check(qar_library_destroy()))
	{
		success = false;
	}
	qar_library_unload();
	SetConsoleCtrlHandler(OnConsoleEvent, FALSE);
	CloseHandle(stop);
	return success ? 0 : 4;
}
