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

// Console handler only signals an event; all SDK and GPU cleanup stays on main.
BOOL WINAPI
OnConsoleEvent(DWORD event)
{
	if(event != CTRL_C_EVENT && event != CTRL_BREAK_EVENT)
	{
		return FALSE;
	}
	auto stop = OpenEventW(EVENT_MODIFY_STATE, FALSE, StopEventName().c_str());
	if(stop)
	{
		SetEvent(stop);
		CloseHandle(stop);
	}
	return TRUE;
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
			success = Check(qar_render_frame_info_get_view_pose(
						  info, eye * 2, &cameras[eye].pose
					  ))
					  && Check(qar_render_frame_info_get_view_fov(
						  info, eye * 2, &cameras[eye].fov
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

int
main(int argc, char** argv)
{
	if(argc > 1 && std::string(argv[1]) == "--help")
	{
		std::cout
			<< "Usage: qar-vulkan-source [qar-streaming-c.dll] [hub-host]\n"
			   "Defaults to the SDK beside this executable. Enter onboarding "
			   "code on stdin. QAR_GPU_ADAPTER_ID=<LUID or UUID hex> selects "
			   "the GPU.\n";
		return 0;
	}
	std::array<wchar_t, 32768> executable{};
	const auto length = GetModuleFileNameW(
		nullptr, executable.data(), static_cast<DWORD>(executable.size())
	);
	if(length == 0 || length >= executable.size())
	{
		std::cerr << "Cannot locate executable directory\n";
		return 1;
	}
	const auto directory =
		std::filesystem::path(executable.data()).parent_path();
	std::error_code error;
	const auto library = argc > 1 ? std::filesystem::absolute(argv[1], error)
								  : directory / "qar-streaming-c.dll";
	if(error)
	{
		std::cerr << "Invalid SDK path: " << error.message() << '\n';
		return 1;
	}
	const auto binaries = library.parent_path().string();
	if(not qar_library_load(library.string().c_str()))
	{
		std::cerr << "Failed to load SDK DLL (check header/runtime versions)\n";
		return 2;
	}
	auto libraryInit = qar_library_init_default();
	libraryInit.enable_console_logging = true;
	if(not Check(qar_library_init(&libraryInit)))
	{
		qar_library_unload();
		return 3;
	}
	bool success = false;
	{
		VulkanDevice gpu;
		Requests requests; // Outlives session subscription teardown.
		QarRuntime* runtime = nullptr;
		QarSession* session = nullptr;
		auto runtimeInit = qar_runtime_init_default();
		runtimeInit.runtime_binaries_folder_path = binaries.c_str();
		if(gpu.Create(RequestedAdapter())
		   && Check(qar_runtime_create(&runtimeInit, &runtime)))
		{
			std::string code;
			std::cout << "Hub onboarding code: " << std::flush;
			if(std::getline(std::cin, code) && not code.empty())
			{
				auto init = qar_onboard_init_default();
				auto codeExt = qar_onboard_code_ext_default();
				auto host = qar_onboard_host_ext_default();
				auto adapter = qar_graphics_device_id_default();
				VkPhysicalDeviceIDProperties id{
					VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES
				};
				VkPhysicalDeviceProperties2 properties{
					VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2
				};
				properties.pNext = &id;
				vkGetPhysicalDeviceProperties2(gpu.physical, &properties);
				adapter.id_type = QAR_GPU_DEVICE_ID_TYPE_LUID;
				std::memcpy(&adapter.luid, id.deviceLUID, VK_LUID_SIZE);
				init.presentation.display_name = "Vulkan Cube Source";
				codeExt.code = code.c_str();
				init.header.next = &codeExt.header;
				codeExt.header.next = &adapter.header;
				if(argc > 2)
				{
					host.hostname = argv[2];
					adapter.header.next = &host.header;
				}
				QarOnboardingId onboarding{};
				if(id.deviceLUIDValid
				   && Check(qar_runtime_onboard(
					   runtime,
					   &init,
					   nullptr,
					   nullptr,
					   nullptr,
					   &onboarding,
					   &session
				   )))
				{
					auto stop = CreateEventW(
						nullptr, TRUE, FALSE, StopEventName().c_str()
					);
					if(stop && SetConsoleCtrlHandler(OnConsoleEvent, TRUE))
					{
						std::cout << "Connected. Waiting for rendering "
									 "targets. Ctrl+C to quit.\n";
						success = Run(session, gpu, stop, requests);
						SetConsoleCtrlHandler(OnConsoleEvent, FALSE);
					}
					if(stop)
					{
						CloseHandle(stop);
					}
				}
				else if(not id.deviceLUIDValid)
				{
					std::cerr << "GPU has no valid Windows LUID\n";
				}
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
	}
	if(not Check(qar_library_destroy()))
	{
		success = false;
	}
	qar_library_unload();
	return success ? 0 : 4;
}
