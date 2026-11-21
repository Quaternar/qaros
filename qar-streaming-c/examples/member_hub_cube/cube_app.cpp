#include "cube_app.hpp"

#ifdef _WIN32
#include <windows.h>
#else
#include <cerrno>
#include <sys/stat.h>
#include <unistd.h>
#endif

#include <algorithm>
#include <chrono>
#include <cmath>
#include <csignal>
#include <cstring>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <thread>
#include <vector>

std::atomic<bool> g_stopRequested = false;

namespace
{

using Clock = std::chrono::steady_clock;
using PeerKey = std::array<uint8_t, QAR_MAX_ID_LENGTH>;

// Each eye gets a color and a depth view: four views in all.
constexpr size_t kViewCount = 4;
// How often the peer list is read for targets that came or went.
constexpr std::chrono::seconds kListInterval{ 1 };
// The wait before a target whose sender ended gets a new one.
constexpr std::chrono::seconds kRetryWait{ 2 };

// A signal handler may only write a lock-free atomic; all cleanup stays on main.
void
OnStopSignal(int)
{
	g_stopRequested.store(true);
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
		const ssize_t count = ::read(STDIN_FILENO, buffer.data(), buffer.size());
		if(count > 0 || (count < 0 && errno == EINTR))
		{
			continue;
		}
		break;
	}
#endif
	g_stopRequested.store(true);
}

void
CancelOnStop(const std::atomic<bool>* finished, QarCancelToken* cancel)
{
	while(not finished->load() && not g_stopRequested.load())
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	if(not finished->load())
	{
		Check(qar_cancel_token_cancel(cancel));
	}
}

PeerKey
KeyOf(const QarPeerId& peer)
{
	PeerKey key{};
	std::memcpy(key.data(), peer.data, key.size());
	return key;
}

std::string
PeerText(const QarPeerId& peer)
{
	std::array<char, 64> text{};
	return Check(qar_uuid_to_string(peer.data, text.data(), text.size())) ? text.data() : "?";
}

// Paused, reconnecting, no transfer yet or an app volume that does not
// resolve yet: the target is not broken, try the next frame.
bool
IsWaiting(QarResult result)
{
	return qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS)
		   || qar_result_has_code(result, QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED);
}

struct Scene
{
	VulkanDevice* gpu;
	QarSession* session;
	QarAppVolumeId volume;
	std::array<float, 3> color;
	Clock::time_point started;
};

// One target app the cube renders for. Its sender is created on a thread of
// its own, because qar_render_sender_create blocks until the target answers.
// From then on the render loop gives it a frame whenever it asks for one
// (request-driven: qar_render_sender_try_begin_frame never blocks).
class CubeTarget
{
public:
	CubeTarget(QarPeerId peer, std::string name, const Scene& scene)
		: m_peer(peer)
		, m_name(std::move(name))
		, m_scene(scene)
	{
		if(Check(qar_cancel_token_create(&m_cancel)))
		{
			m_creation = std::thread(&CubeTarget::Create, this);
		}
		else
		{
			m_finished.store(true);
		}
	}
	CubeTarget(const CubeTarget&) = delete;
	CubeTarget& operator=(const CubeTarget&) = delete;
	~CubeTarget()
	{
		m_stop.store(true);
		if(m_cancel)
		{
			Check(qar_cancel_token_cancel(m_cancel));
		}
		if(m_creation.joinable())
		{
			m_creation.join();
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

	bool Finished() const { return m_finished.load(); }

	// A frame, when the target asked for one. Returns whether one was shown.
	bool RenderIfAsked()
	{
		if(not m_created.load() || Finished())
		{
			return false;
		}
		QarRenderFrameInfo* info = nullptr;
		bool ready = false;
		const QarResult tried = qar_render_sender_try_begin_frame(m_sender, &info, &ready);
		if(not qar_result_is_success(tried))
		{
			if(not IsWaiting(tried))
			{
				End(tried);
			}
			return false;
		}
		if(not ready)
		{
			return false;
		}
		const bool shown = RenderAndShow(info);
		qar_render_frame_info_handle_destroy(info);
		if(shown && ++m_framesShown == 1)
		{
			std::cout << "First frame shown to \"" << m_name << "\"\n";
		}
		return shown;
	}

private:
	void Create()
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
		init.frame_views_count = kViewCount;
		for(size_t index = 0; index < kViewCount; ++index)
		{
			init.frame_views[index] = qar_render_frame_view_default();
			init.frame_views[index].eye =
				index < 2 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
			init.frame_views[index].data_type =
				index % 2 == 0 ? QAR_VIDEO_FRAME_VIEW_TYPE_COLOR : QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH;
			init.frame_views[index].texture_index = static_cast<uint32_t>(index);
		}
		// Blocks until the target asks for the stream; the destructor cancels it.
		const QarResult created =
			qar_render_sender_create(m_scene.session, &init, m_cancel, &m_sender);
		if(m_stop.load() || not Check(created))
		{
			m_finished.store(true);
			return;
		}
		m_camera.emplace(m_scene.gpu, m_scene.color);
		if(not m_camera->Create())
		{
			m_finished.store(true);
			return;
		}
		std::cout << "Rendering for target \"" << m_name << "\" (" << PeerText(m_peer) << ")\n";
		m_created.store(true);
	}

	// From here the frame is shown, also when rendering failed: the show closes
	// the frame cycle. A frame whose record cannot be read is not shown: nothing
	// could signal its semaphore.
	bool RenderAndShow(QarRenderFrameInfo* info)
	{
		QarVideoFrameVulkan* frame = nullptr;
		if(not Check(qar_render_sender_frame_vulkan(m_sender, &frame)))
		{
			m_finished.store(true);
			return false;
		}
		const bool rendered = RenderInto(*frame, info);
		if(not rendered && not m_camera->Discard(*frame))
		{
			m_finished.store(true);
			return false;
		}
		auto show = qar_render_frame_show_default();
		show.rendered_near_far = { .1f, 10.f };
		const QarResult shown = qar_render_sender_show_frame(m_sender, &show);
		if(not rendered)
		{
			m_finished.store(true);
			return false;
		}
		if(not qar_result_is_success(shown) && not IsWaiting(shown))
		{
			End(shown);
		}
		return true;
	}

	// Each eye's pose and field of view are those of the view showing its color.
	bool RenderInto(QarVideoFrameVulkan& frame, QarRenderFrameInfo* info)
	{
		std::array<CameraView, 2> cameras{};
		for(size_t eye = 0; eye < 2; ++eye)
		{
			const int view = FindFrameView(
				frame,
				eye == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT,
				QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
			);
			if(view < 0)
			{
				return false;
			}
			const auto index = static_cast<size_t>(view);
			if(not Check(qar_render_frame_info_get_view_pose(info, index, &cameras[eye].pose))
			   || not Check(qar_render_frame_info_get_view_fov(info, index, &cameras[eye].fov)))
			{
				return false;
			}
		}
		const float seconds = std::chrono::duration<float>(Clock::now() - m_scene.started).count();
		return m_camera->Render(frame, cameras, seconds);
	}

	void End(QarResult result)
	{
		if(not m_stop.load())
		{
			Check(result);
		}
		m_finished.store(true);
	}

	QarPeerId m_peer{};
	std::string m_name;
	Scene m_scene;
	std::optional<CubeCamera> m_camera;
	QarCancelToken* m_cancel = nullptr;
	QarRenderSender* m_sender = nullptr;
	std::atomic<bool> m_stop = false;
	std::atomic<bool> m_created = false;
	std::atomic<bool> m_finished = false;
	size_t m_framesShown = 0;
	std::thread m_creation;
};

struct ListedTarget
{
	QarPeerId peer{};
	std::string name;
};

// The running target apps whose display name contains `filter`, or nothing
// when the list cannot be read. In a session of several hubs this lists the
// targets of every hub: the filter is what keeps the cube to its own.
std::optional<std::vector<ListedTarget>>
ListTargets(QarSession* session, const std::string& filter)
{
	size_t count = 0;
	if(not Check(qar_query_peer_specs_count(session, &count)))
	{
		return std::nullopt;
	}
	std::vector<QarPeerSpec*> specs(count, nullptr);
	size_t written = 0;
	const bool listed =
		count == 0 || Check(qar_query_peer_specs(session, specs.data(), specs.size(), &written));
	std::vector<ListedTarget> targets;
	for(size_t index = 0; index < std::min(written, specs.size()); ++index)
	{
		QarPeerSpec* spec = specs[index];
		if(spec == nullptr)
		{
			continue;
		}
		QarPeerRole role = QAR_PEER_ROLE_INTERNAL;
		QarPeerPresence presence = QAR_PEER_PRESENCE_UNKNOWN;
		ListedTarget target;
		std::array<char, 256> name{};
		const bool running =
			Check(qar_peer_spec_get_role(spec, &role)) && role == QAR_PEER_ROLE_TARGET
			&& Check(qar_peer_spec_get_presence(spec, &presence))
			&& (presence == QAR_PEER_PRESENCE_ONLINE || presence == QAR_PEER_PRESENCE_UNKNOWN)
			&& Check(qar_peer_spec_get_id(spec, &target.peer))
			&& Check(qar_peer_spec_get_display_name(spec, name.data(), name.size()));
		target.name = name.data();
		if(running && (filter.empty() || target.name.find(filter) != std::string::npos))
		{
			targets.push_back(std::move(target));
		}
		qar_peer_spec_handle_destroy(spec);
	}
	if(not listed)
	{
		return std::nullopt;
	}
	return targets;
}

// The targets the cube serves: one for each listed target app, none for one
// that left, and a new sender after a short wait for one whose sender ended.
struct TargetBook
{
	std::map<PeerKey, std::unique_ptr<CubeTarget>> served;
	std::map<PeerKey, Clock::time_point> retryAt;

	bool Follow(const Scene& scene, const std::string& filter)
	{
		const std::optional<std::vector<ListedTarget>> listed = ListTargets(scene.session, filter);
		if(not listed)
		{
			return false;
		}
		const Clock::time_point now = Clock::now();
		std::map<PeerKey, const ListedTarget*> current;
		for(const ListedTarget& target : *listed)
		{
			current.emplace(KeyOf(target.peer), &target);
		}
		for(auto it = served.begin(); it != served.end();)
		{
			const bool left = not current.contains(it->first);
			if(not left && not it->second->Finished())
			{
				++it;
				continue;
			}
			retryAt[it->first] = now + kRetryWait;
			it = served.erase(it);
		}
		for(const auto& [key, target] : current)
		{
			if(not served.contains(key) && now >= retryAt[key])
			{
				served.emplace(
					key, std::make_unique<CubeTarget>(target->peer, target->name, scene)
				);
			}
		}
		return true;
	}

	bool RenderPass()
	{
		bool rendered = false;
		for(auto& [key, target] : served)
		{
			rendered = target->RenderIfAsked() || rendered;
		}
		return rendered;
	}
};

// Prints where the shared volume and its content are whenever either moves,
// whoever moved them: this app, the other app, or a user in any Visualizer or
// headset of the session. The callback runs on a library thread; the handle is
// the library's and valid during the call only.
struct VolumeWatch
{
	QarAppVolumeId volume = qar_app_volume_id_default();
	std::mutex lock;
	std::optional<QarPose> pose;
	std::optional<QarPose> appPose;

	static bool Moved(const std::optional<QarPose>& before, const QarPose& now)
	{
		constexpr float kMinimumMove = 0.005f; // metres; below this is noise
		return not before || std::abs(before->position.x - now.position.x) > kMinimumMove
			   || std::abs(before->position.y - now.position.y) > kMinimumMove
			   || std::abs(before->position.z - now.position.z) > kMinimumMove;
	}

	static void Print(const char* what, const QarPose& pose)
	{
		std::cout << what << " at x=" << pose.position.x << " y=" << pose.position.y
				  << " z=" << pose.position.z << '\n';
	}

	static void OnUpdate(QarAppVolume* handle, void* state)
	{
		auto* watch = static_cast<VolumeWatch*>(state);
		QarAppVolumeId id = qar_app_volume_id_default();
		QarPose pose = qar_pose_default();
		QarPose appPose = qar_pose_default();
		if(not qar_result_is_success(qar_app_volume_get_id(handle, &id))
		   || not qar_app_volume_id_equals(&id, &watch->volume)
		   || not qar_result_is_success(qar_app_volume_get_pose(handle, &pose))
		   || not qar_result_is_success(qar_app_volume_get_app_pose(handle, &appPose)))
		{
			return;
		}
		const std::lock_guard<std::mutex> guard(watch->lock);
		if(Moved(watch->pose, pose))
		{
			Print("The shared app volume is", pose);
			watch->pose = pose;
		}
		if(Moved(watch->appPose, appPose))
		{
			Print("The cube's app pose in the shared volume is", appPose);
			watch->appPose = appPose;
		}
	}
};

} // namespace

void
WatchForStop()
{
	std::signal(SIGINT, OnStopSignal);
	std::signal(SIGTERM, OnStopSignal);
#ifdef _WIN32
	const bool pipe = GetFileType(GetStdHandle(STD_INPUT_HANDLE)) == FILE_TYPE_PIPE;
#else
	struct stat status{};
	const bool pipe =
		fstat(STDIN_FILENO, &status) == 0 && (S_ISFIFO(status.st_mode) || S_ISSOCK(status.st_mode));
#endif
	if(pipe)
	{
		std::thread(StopWhenStdinCloses).detach();
	}
}

void
PrintProgress(QarActionSeverity, float, const char* message, void*)
{
	if(message != nullptr && message[0] != '\0')
	{
		std::cout << message << '\n';
	}
}

QarResult
JoinAsLocalApp(
	QarRuntime* runtime,
	const char* displayName,
	const char* tag,
	const QarMemberHubInvitation* grant,
	QarSession** session
)
{
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = displayName;
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tagExt = qar_onboard_local_app_tag_ext_default();
	QarOnboardRemoteAppGrantExt redeem = qar_onboard_remote_app_grant_ext_default();
	init.header.next = &local.header;
	if(grant != nullptr)
	{
		// Borrowed for the call. The grant's tag applies, so no tag is chained.
		redeem.invitation = grant;
		local.header.next = &redeem.header;
	}
	else if(tag != nullptr)
	{
		tagExt.app_tag = tag;
		local.header.next = &tagExt.header;
	}
	QarCancelToken* cancel = nullptr;
	const QarResult created = qar_cancel_token_create(&cancel);
	if(not qar_result_is_success(created))
	{
		return created;
	}
	std::atomic<bool> finished = false;
	std::thread canceller(CancelOnStop, &finished, cancel);
	// A local app stores nothing: the onboarding id stays zero.
	QarOnboardingId unused = qar_onboarding_id_default();
	const QarResult result =
		qar_runtime_onboard(runtime, &init, PrintProgress, nullptr, cancel, &unused, session);
	finished.store(true);
	canceller.join();
	qar_cancel_token_handle_destroy(cancel);
	return result;
}

int
ExplainJoinFailure(QarResult result)
{
	int exitCode = 1;
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		std::cerr << "QAROS did not allow this app to join: the approval was denied or revoked, or "
					 "the grant already admitted another app.\n";
		exitCode = 4;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		std::cerr << "Nobody answered the approval prompt in QAROS in time. Start the app again.\n";
		exitCode = 5;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		std::cerr << "The program answering on this machine is not the installed QAROS.\n";
		exitCode = 6;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		std::cerr << "This app no longer matches what QAROS approved: revoke it in the QAROS "
					 "Visualizer, then start it again.\n";
		exitCode = 7;
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		std::cerr << "Could not start or reach QAROS on this machine. Start QAROS from the Start "
					 "menu, then start this app again.\n";
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

void
PrintMyPeerId(const QarSession* session)
{
	QarPeerSpec* me = nullptr;
	if(not Check(qar_session_get_my_spec(session, &me)))
	{
		return;
	}
	QarPeerId id = qar_peer_id_default();
	if(Check(qar_peer_spec_get_id(me, &id)))
	{
		std::cout << "Joined QAROS as peer " << PeerText(id) << '\n';
	}
	qar_peer_spec_handle_destroy(me);
}

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

bool
RenderCube(QarSession* session, VulkanDevice& gpu, const CubeOptions& options)
{
	// The first app asks for the volume and creates it; the other gets the same
	// volume back, where it is now: the pose here only counts for the first.
	auto volumeInit = qar_app_volume_init_default();
	volumeInit.common_name = kSharedVolumeName;
	volumeInit.display_name = kSharedVolumeDisplayName;
	volumeInit.pose.position.z = -1.0f;
	volumeInit.size = { 1.2f, 1.2f, 1.2f };
	Scene scene{ .gpu = &gpu, .session = session, .color = options.color, .started = Clock::now() };
	if(not Check(qar_app_volumes_get_or_create(session, &volumeInit, &scene.volume)))
	{
		return false;
	}
	VolumeWatch watch;
	watch.volume = scene.volume;
	QarCancelToken* subscription = nullptr;
	if(not Check(qar_cancel_token_create(&subscription))
	   || not Check(
		   qar_app_volumes_subscribe_updates(session, VolumeWatch::OnUpdate, &watch, subscription)
	   ))
	{
		if(subscription != nullptr)
		{
			qar_cancel_token_handle_destroy(subscription);
		}
		return false;
	}
	if(options.targetFilter.empty())
	{
		std::cout << "Waiting for targets: rendering for every target app in the session.\n";
	}
	else
	{
		std::cout << "Waiting for targets whose name contains \"" << options.targetFilter
				  << "\".\n";
	}
	TargetBook book;
	bool success = true;
	Clock::time_point nextList = Clock::now();
	while(success && not g_stopRequested.load())
	{
		if(Clock::now() >= nextList)
		{
			success = book.Follow(scene, options.targetFilter);
			nextList = Clock::now() + kListInterval;
		}
		if(not book.RenderPass())
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(2));
		}
	}
	book.served.clear();
	// Ends the subscription and waits for a running callback: `watch` may go.
	qar_cancel_token_handle_destroy(subscription);
	return success;
}
