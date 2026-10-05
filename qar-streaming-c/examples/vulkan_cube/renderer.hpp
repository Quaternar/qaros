#pragma once
#include <array>
#include <glm/glm.hpp>
#include <qar_streaming.h>
#include <string>
#include <string_view>
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
	// the GPU to render on, as QAR_GPU_ADAPTER_ID gives it. Empty picks the
	// first supported GPU; a value that matches no supported GPU fails.
	bool Create(std::string_view requestedAdapter = {});
	~VulkanDevice();
};

// Index in `frame.texture_views` of the view showing `type` for `eye`, or -1.
// The sender lays the frame out itself, so its views need not follow the order
// QarRenderSenderInit::frame_views asked for: find views by what they show.
int FindFrameView(
	const QarVideoFrameVulkan& frame,
	QarVideoFrameViewEye eye,
	QarVideoFrameViewType type
);

// Lowercase hex of `bytes`, the QAR_GPU_ADAPTER_ID format.
std::string AdapterIdHex(const uint8_t* bytes, size_t count);

// Whether `requested` (QAR_GPU_ADAPTER_ID format) names this GPU's LUID or
// UUID.
bool MatchesAdapterId(
	const VkPhysicalDeviceIDProperties& id, std::string_view requested
);

// One camera's resources and one submitted frame. Never waits for other
// cameras.
class CubeCamera
{
	VulkanDevice* m_gpu;
	VkCommandPool m_pool = VK_NULL_HANDLE;
	VkCommandBuffer m_commands = VK_NULL_HANDLE;
	VkFence m_fence = VK_NULL_HANDLE;
	VkRenderPass m_pass = VK_NULL_HANDLE;
	VkPipelineLayout m_layout = VK_NULL_HANDLE;
	VkPipeline m_pipeline = VK_NULL_HANDLE;
	std::vector<VkImageView> m_views;
	std::vector<VkFramebuffer> m_buffers;
	bool m_pending = false;
	bool CreatePipeline();
	void ClearTargets();

public:
	explicit CubeCamera(VulkanDevice* gpu)
		: m_gpu(gpu)
	{
	}
	CubeCamera(const CubeCamera&) = delete;
	CubeCamera& operator=(const CubeCamera&) = delete;
	~CubeCamera();
	bool Create();
	bool Submit(
		QarVideoFrameVulkan& frame,
		const std::array<CameraView, 2>& views,
		float seconds
	);
	bool Discard(QarVideoFrameVulkan& frame);
	bool Complete(bool& ready);
	bool Drain();
};
