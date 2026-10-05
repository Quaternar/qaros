#include "renderer.hpp"
#include <algorithm>
#include <cstring>
#include <iostream>

QAR_IMPLEMENT_DYNAMIC_LOADING()

namespace
{
constexpr uint32_t Size = 128;

uint32_t
MemoryType(VulkanDevice& gpu, uint32_t bits, VkMemoryPropertyFlags flags)
{
	VkPhysicalDeviceMemoryProperties properties{};
	vkGetPhysicalDeviceMemoryProperties(gpu.physical, &properties);
	for(uint32_t i = 0; i < properties.memoryTypeCount; ++i)
	{
		if((bits & (1u << i))
		   && (properties.memoryTypes[i].propertyFlags & flags) == flags)
		{
			return i;
		}
	}
	return UINT32_MAX;
}

struct Images
{
	VulkanDevice* gpu;
	QarVideoFrameVulkan frame{};
	VkSemaphore ready = VK_NULL_HANDLE;
	explicit Images(VulkanDevice* value)
		: gpu(value)
	{
	}
	~Images()
	{
		if(ready)
		{
			vkDestroySemaphore(gpu->device, ready, nullptr);
		}
		for(size_t i = 0; i < frame.textures_count; ++i)
		{
			if(frame.textures[i].image)
			{
				vkDestroyImage(gpu->device, frame.textures[i].image, nullptr);
			}
			if(frame.textures[i].image_memory)
			{
				vkFreeMemory(
					gpu->device, frame.textures[i].image_memory, nullptr
				);
			}
		}
	}
	bool Create()
	{
		frame.textures_count = frame.texture_views_count = 4;
		for(size_t i = 0; i < 4; ++i)
		{
			// The render sender's own order: both color views, then both
			// depth views. The camera must find views by eye and type.
			const bool depth = i >= 2;
			auto& texture = frame.textures[i];
			texture.size = { depth ? QAR_PIXEL_FORMAT_D32_FLOAT
								   : QAR_PIXEL_FORMAT_R8G8B8A8,
							 Size,
							 Size,
							 1 };
			texture.layout = VK_IMAGE_LAYOUT_UNDEFINED;
			texture.queue_family_index = VK_QUEUE_FAMILY_IGNORED;
			auto& view = frame.texture_views[i];
			view.end_x = view.end_y = Size;
			view.texture_index = static_cast<uint32_t>(i);
			view.texture_format = texture.size.format;
			view.data_type = depth ? QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH
								   : QAR_VIDEO_FRAME_VIEW_TYPE_COLOR;
			view.eye = i % 2 == 0 ? QAR_VIDEO_FRAME_VIEW_EYE_LEFT
								  : QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
			VkExternalMemoryImageCreateInfo external{
				VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO
			};
			external.handleTypes =
				VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT;
			VkImageCreateInfo image{ VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO };
			image.pNext = &external;
			image.imageType = VK_IMAGE_TYPE_2D;
			image.format =
				depth ? VK_FORMAT_D32_SFLOAT : VK_FORMAT_R8G8B8A8_UNORM;
			image.extent = { Size, Size, 1 };
			image.mipLevels = image.arrayLayers = 1;
			image.samples = VK_SAMPLE_COUNT_1_BIT;
			image.tiling = VK_IMAGE_TILING_OPTIMAL;
			image.usage = VK_IMAGE_USAGE_TRANSFER_SRC_BIT
						  | (depth ? VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
								   : VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);
			if(not CheckVk(
				   vkCreateImage(gpu->device, &image, nullptr, &texture.image),
				   "test image"
			   ))
			{
				return false;
			}
			VkMemoryRequirements requirements{};
			vkGetImageMemoryRequirements(
				gpu->device, texture.image, &requirements
			);
			VkMemoryDedicatedAllocateInfo dedicated{
				VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO
			};
			dedicated.image = texture.image;
			VkExportMemoryAllocateInfo exportInfo{
				VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO
			};
			exportInfo.pNext = &dedicated;
			exportInfo.handleTypes = external.handleTypes;
			VkMemoryAllocateInfo memory{
				VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO
			};
			memory.pNext = &exportInfo;
			memory.allocationSize = requirements.size;
			memory.memoryTypeIndex = MemoryType(
				*gpu,
				requirements.memoryTypeBits,
				VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT
			);
			if(not CheckVk(
				   vkAllocateMemory(
					   gpu->device, &memory, nullptr, &texture.image_memory
				   ),
				   "test image memory"
			   ))
			{
				return false;
			}
			if(not CheckVk(
				   vkBindImageMemory(
					   gpu->device, texture.image, texture.image_memory, 0
				   ),
				   "bind test image"
			   ))
			{
				return false;
			}
		}
		VkSemaphoreCreateInfo semaphore{
			VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO
		};
		if(not CheckVk(
			   vkCreateSemaphore(gpu->device, &semaphore, nullptr, &ready),
			   "test ready semaphore"
		   ))
		{
			return false;
		}
		frame.synchronization.semaphore = ready;
		return true;
	}
};

struct Readback
{
	VulkanDevice* gpu;
	VkBuffer buffer = VK_NULL_HANDLE;
	VkDeviceMemory memory = VK_NULL_HANDLE;
	VkCommandPool pool = VK_NULL_HANDLE;
	VkCommandBuffer command = VK_NULL_HANDLE;
	VkFence fence = VK_NULL_HANDLE;
	bool pending = false;
	explicit Readback(VulkanDevice* value)
		: gpu(value)
	{
	}
	~Readback()
	{
		if(pending
		   && not CheckVk(
			   vkWaitForFences(gpu->device, 1, &fence, VK_TRUE, UINT64_MAX),
			   "drain test readback"
		   ))
		{
			std::cerr << "Readback drain failed\n";
		}
		if(fence)
		{
			vkDestroyFence(gpu->device, fence, nullptr);
		}
		if(pool)
		{
			vkDestroyCommandPool(gpu->device, pool, nullptr);
		}
		if(buffer)
		{
			vkDestroyBuffer(gpu->device, buffer, nullptr);
		}
		if(memory)
		{
			vkFreeMemory(gpu->device, memory, nullptr);
		}
	}
	bool Create()
	{
		VkBufferCreateInfo info{ VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO };
		info.size = Size * Size * 8;
		info.usage = VK_BUFFER_USAGE_TRANSFER_DST_BIT;
		if(not CheckVk(
			   vkCreateBuffer(gpu->device, &info, nullptr, &buffer),
			   "test buffer"
		   ))
		{
			return false;
		}
		VkMemoryRequirements requirements{};
		vkGetBufferMemoryRequirements(gpu->device, buffer, &requirements);
		VkMemoryAllocateInfo allocation{
			VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO
		};
		allocation.allocationSize = requirements.size;
		allocation.memoryTypeIndex = MemoryType(
			*gpu,
			requirements.memoryTypeBits,
			VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
				| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT
		);
		if(not CheckVk(
			   vkAllocateMemory(gpu->device, &allocation, nullptr, &memory),
			   "test readback memory"
		   ))
		{
			return false;
		}
		if(not CheckVk(
			   vkBindBufferMemory(gpu->device, buffer, memory, 0),
			   "bind test buffer"
		   ))
		{
			return false;
		}
		VkCommandPoolCreateInfo commands{
			VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO
		};
		commands.queueFamilyIndex = gpu->family;
		if(not CheckVk(
			   vkCreateCommandPool(gpu->device, &commands, nullptr, &pool),
			   "test pool"
		   ))
		{
			return false;
		}
		VkCommandBufferAllocateInfo allocate{
			VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
		};
		allocate.commandPool = pool;
		allocate.commandBufferCount = 1;
		allocate.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
		if(not CheckVk(
			   vkAllocateCommandBuffers(gpu->device, &allocate, &command),
			   "test command"
		   ))
		{
			return false;
		}
		VkFenceCreateInfo fenceInfo{ VK_STRUCTURE_TYPE_FENCE_CREATE_INFO };
		return CheckVk(
			vkCreateFence(gpu->device, &fenceInfo, nullptr, &fence),
			"test fence"
		);
	}
	bool
	Copy(Images& images, std::vector<uint8_t>& color, std::vector<float>& depth)
	{
		if(not CheckVk(
			   vkResetCommandPool(gpu->device, pool, 0), "reset test pool"
		   ))
		{
			return false;
		}
		if(not CheckVk(
			   vkResetFences(gpu->device, 1, &fence), "reset test fence"
		   ))
		{
			return false;
		}
		VkCommandBufferBeginInfo begin{
			VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
		};
		if(not CheckVk(
			   vkBeginCommandBuffer(command, &begin), "begin test readback"
		   ))
		{
			return false;
		}
		for(size_t i = 0; i < 2; ++i)
		{
			// The left eye's color, then its depth.
			const auto& texture = images.frame.textures[i == 0 ? 0 : 2];
			if(texture.layout != VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL
			   || texture.queue_family_index != VK_QUEUE_FAMILY_EXTERNAL)
			{
				return false;
			}
			VkImageMemoryBarrier barrier{
				VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER
			};
			barrier.oldLayout = barrier.newLayout = texture.layout;
			barrier.srcQueueFamilyIndex = VK_QUEUE_FAMILY_EXTERNAL;
			barrier.dstQueueFamilyIndex = gpu->family;
			barrier.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
			barrier.image = texture.image;
			barrier.subresourceRange = { static_cast<VkImageAspectFlags>(
											 i == 0 ? VK_IMAGE_ASPECT_COLOR_BIT
													: VK_IMAGE_ASPECT_DEPTH_BIT
										 ),
										 0,
										 1,
										 0,
										 1 };
			vkCmdPipelineBarrier(
				command,
				VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
				VK_PIPELINE_STAGE_TRANSFER_BIT,
				0,
				0,
				nullptr,
				0,
				nullptr,
				1,
				&barrier
			);
			VkBufferImageCopy copy{};
			copy.bufferOffset = i * Size * Size * 4;
			copy.imageSubresource = {
				barrier.subresourceRange.aspectMask, 0, 0, 1
			};
			copy.imageExtent = { Size, Size, 1 };
			vkCmdCopyImageToBuffer(
				command, texture.image, texture.layout, buffer, 1, &copy
			);
			barrier.srcQueueFamilyIndex = gpu->family;
			barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_EXTERNAL;
			barrier.srcAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
			barrier.dstAccessMask = 0;
			vkCmdPipelineBarrier(
				command,
				VK_PIPELINE_STAGE_TRANSFER_BIT,
				VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
				0,
				0,
				nullptr,
				0,
				nullptr,
				1,
				&barrier
			);
		}
		if(not CheckVk(vkEndCommandBuffer(command), "end test readback"))
		{
			return false;
		}
		VkSubmitInfo submit{ VK_STRUCTURE_TYPE_SUBMIT_INFO };
		submit.commandBufferCount = 1;
		submit.pCommandBuffers = &command;
		const VkPipelineStageFlags stage = VK_PIPELINE_STAGE_TRANSFER_BIT;
		submit.waitSemaphoreCount = 1;
		submit.pWaitSemaphores = &images.ready;
		submit.pWaitDstStageMask = &stage;
		if(not CheckVk(
			   vkQueueSubmit(gpu->queue, 1, &submit, fence),
			   "submit test readback"
		   ))
		{
			return false;
		}
		pending = true;
		if(not CheckVk(
			   vkWaitForFences(gpu->device, 1, &fence, VK_TRUE, UINT64_MAX),
			   "wait test readback"
		   ))
		{
			return false;
		}
		pending = false;
		void* data = nullptr;
		if(not CheckVk(
			   vkMapMemory(gpu->device, memory, 0, Size * Size * 8, 0, &data),
			   "map test readback"
		   ))
		{
			return false;
		}
		color.resize(Size * Size * 4);
		depth.resize(Size * Size);
		std::memcpy(color.data(), data, color.size());
		std::memcpy(
			depth.data(),
			static_cast<uint8_t*>(data) + color.size(),
			depth.size() * sizeof(float)
		);
		vkUnmapMemory(gpu->device, memory);
		return true;
	}
};
} // namespace

int
main()
{
	VulkanDevice gpu;
	if(not gpu.Create())
	{
		return 1;
	}
	Images first(&gpu), second(&gpu);
	Readback readback(&gpu);
	// Destruction order: cameras drain before images are freed, all before
	// device.
	CubeCamera cameraA(&gpu), cameraB(&gpu);
	if(not first.Create() || not second.Create() || not readback.Create()
	   || not cameraA.Create() || not cameraB.Create())
	{
		return 1;
	}
	std::array<CameraView, 2> centered{}, shifted{};
	for(size_t eye = 0; eye < 2; ++eye)
	{
		centered[eye].pose.orientation.w = shifted[eye].pose.orientation.w = 1;
		centered[eye].pose.position = { 0, 0, 1.5f };
		shifted[eye].pose.position = { .3f, 0, 1.5f };
		centered[eye].fov = shifted[eye].fov = { -.7f, .7f, .7f, -.7f };
	}
	for(size_t cycle = 0; cycle < 2; ++cycle)
	{
		if(not cameraA.Submit(first.frame, centered, .4f)
		   || not cameraB.Submit(second.frame, shifted, .4f))
		{
			return 1;
		}
		if(not cameraA.Drain() || not cameraB.Drain())
		{
			return 1;
		}
		std::vector<uint8_t> a, b;
		std::vector<float> zA, zB;
		if(not readback.Copy(first, a, zA) || not readback.Copy(second, b, zB))
		{
			return 1;
		}
		size_t colored = 0, differences = 0;
		for(size_t pixel = 0; pixel < Size * Size; ++pixel)
		{
			colored += a[pixel * 4] > 128 || a[pixel * 4 + 1] > 128
					   || a[pixel * 4 + 2] > 128;
			differences += a[pixel * 4] != b[pixel * 4]
						   || a[pixel * 4 + 1] != b[pixel * 4 + 1]
						   || a[pixel * 4 + 2] != b[pixel * 4 + 2];
		}
		const float center = zA[(Size / 2) * Size + Size / 2];
		if(colored < 100 || colored > Size * Size / 2 || differences < 100
		   || center <= 0 || center >= 1 || zA[0] != 1)
		{
			std::cerr << "Cube camera readback failed: colored=" << colored
					  << " differences=" << differences << " depth=" << center
					  << '\n';
			return 1;
		}
	}
	std::cout << "Two independent cube cameras: color, forward depth, distinct "
				 "views and resource reuse passed\n";
	return 0;
}
