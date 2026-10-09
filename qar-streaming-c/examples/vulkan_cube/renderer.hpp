#pragma once
#include <array>
#include <glm/glm.hpp>
#include <map>
#include <mutex>
#include <qar_streaming.h>
#include <string>
#include <string_view>
#include <utility>
#include <vector>
#include <vulkan/vulkan.h>

struct CameraView
{
	QarPose pose;
	QarFov fov;
};

bool Check(QarResult result);
bool CheckVk(VkResult result, const char* operation);

// Owns the application's device. All cameras and SDK senders must die before
// it.
struct VulkanDevice
{
	VkInstance instance = VK_NULL_HANDLE;
	VkPhysicalDevice physical = VK_NULL_HANDLE;
	VkDevice device = VK_NULL_HANDLE;
	VkQueue queue = VK_NULL_HANDLE;
	uint32_t family = 0;
	// requestedAdapter: lowercase hex LUID (16 digits) or UUID (32 digits) of
	// the GPU to render on. Empty picks the first supported GPU; a value that
	// matches no supported GPU fails.
	bool Create(std::string_view requestedAdapter = {});
	// Every target renders on a thread of its own and they all share this one
	// queue, which Vulkan requires to be externally synchronized. Only the
	// submit itself is under the lock; recording runs in parallel.
	bool Submit(const VkSubmitInfo& submit, VkFence fence, const char* what);
	~VulkanDevice();

private:
	std::mutex m_queueLock;
};

// Index in `frame.texture_views` of the view showing `type` for `eye`, or -1.
// The sender lays the frame out itself, so its views need not follow the order
// QarRenderSenderInit::frame_views asked for: find views by what they show.
int FindFrameView(
	const QarVideoFrameVulkan& frame,
	QarVideoFrameViewEye eye,
	QarVideoFrameViewType type
);

// Lowercase hex of `bytes`, the format Create() takes.
std::string AdapterIdHex(const uint8_t* bytes, size_t count);

// Whether `requested` (lowercase hex) names this GPU's LUID or UUID.
bool MatchesAdapterId(
	const VkPhysicalDeviceIDProperties& id, std::string_view requested
);

// One target's camera. It keeps up to kFramesInFlight frames on the GPU and
// waits for the GPU only when it reuses the oldest of them - never for the
// frame it has just submitted: that frame's semaphore hands the work to the
// sender, which waits for it on the GPU. So the caller shows the frame right
// after Render() and can begin the next one at once.
class CubeCamera
{
public:
	static constexpr size_t kFramesInFlight = 2;

	explicit CubeCamera(VulkanDevice* gpu)
		: m_gpu(gpu)
	{
	}
	CubeCamera(const CubeCamera&) = delete;
	CubeCamera& operator=(const CubeCamera&) = delete;
	~CubeCamera();
	bool Create();
	// Records and submits one stereo frame into the sender's images and
	// signals frame.synchronization.semaphore when the GPU is done with it.
	bool Render(
		QarVideoFrameVulkan& frame,
		const std::array<CameraView, 2>& views,
		float seconds
	);
	// Signals the semaphore of a begun frame that could not be rendered, so
	// the sender can still consume it.
	bool Discard(QarVideoFrameVulkan& frame);
	// Waits for every submitted frame. Teardown only.
	bool Drain();

private:
	struct Slot
	{
		VkCommandBuffer commands = VK_NULL_HANDLE;
		VkFence fence = VK_NULL_HANDLE;
	};
	// One eye's attachments, made the first time an image pair is seen. The
	// sender's images live as long as the sender, so after the first frame
	// nothing is created per frame.
	struct EyeTarget
	{
		std::array<VkImageView, 2> views{};
		VkFramebuffer framebuffer = VK_NULL_HANDLE;
	};
	using ImagePair = std::pair<VkImage, VkImage>;

	bool CreatePipeline();
	bool NextSlot(Slot*& slot);
	const EyeTarget* TargetFor(
		const QarVideoTextureVulkan& color,
		const QarVideoTextureVulkan& depth,
		uint32_t width,
		uint32_t height
	);
	void DestroyTargets();

	VulkanDevice* m_gpu;
	VkCommandPool m_pool = VK_NULL_HANDLE;
	std::array<Slot, kFramesInFlight> m_slots{};
	size_t m_nextSlot = 0;
	VkRenderPass m_pass = VK_NULL_HANDLE;
	VkPipelineLayout m_layout = VK_NULL_HANDLE;
	VkPipeline m_pipeline = VK_NULL_HANDLE;
	std::map<ImagePair, EyeTarget> m_targets;
};
