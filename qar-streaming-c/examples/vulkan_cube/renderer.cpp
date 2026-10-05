#include "renderer.hpp"
#include "CubeShaders.hpp"
#include <array>
#include <cmath>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>
#include <iostream>
#include <span>

bool
Check(QarResult result)
{
	if(qar_result_is_success(result))
	{
		return true;
	}
	qar_result_log_if_error(result);
	return false;
}
bool
CheckVk(VkResult result, const char* operation)
{
	if(result == VK_SUCCESS)
	{
		return true;
	}
	std::cerr << operation << " failed: VkResult " << result << '\n';
	return false;
}
bool
VulkanDevice::Create()
{
	VkApplicationInfo app{ VK_STRUCTURE_TYPE_APPLICATION_INFO };
	app.pApplicationName = "QAROS Vulkan cube";
	app.apiVersion = VK_API_VERSION_1_1;
	VkInstanceCreateInfo init{ VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO };
	init.pApplicationInfo = &app;
	if(not CheckVk(
		   vkCreateInstance(&init, nullptr, &instance), "vkCreateInstance"
	   ))
	{
		return false;
	}
	uint32_t count = 0;
	if(not CheckVk(
		   vkEnumeratePhysicalDevices(instance, &count, nullptr),
		   "enumerate GPUs"
	   ))
	{
		return false;
	}
	std::vector<VkPhysicalDevice> devices(count);
	if(not CheckVk(
		   vkEnumeratePhysicalDevices(instance, &count, devices.data()),
		   "enumerate GPUs"
	   ))
	{
		return false;
	}
	const std::array<const char*, 2> extensions = {
		VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME,
		VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME
	};
	for(auto candidate : devices)
	{
		VkPhysicalDeviceProperties gpuProperties{};
		vkGetPhysicalDeviceProperties(candidate, &gpuProperties);
		if(gpuProperties.vendorID != 0x10de)
		{
			continue; // Current SDK supports NVIDIA adapters.
		}
		uint32_t extensionCount = 0;
		if(not CheckVk(
			   vkEnumerateDeviceExtensionProperties(
				   candidate, nullptr, &extensionCount, nullptr
			   ),
			   "enumerate extensions"
		   ))
		{
			return false;
		}
		std::vector<VkExtensionProperties> available(extensionCount);
		if(not CheckVk(
			   vkEnumerateDeviceExtensionProperties(
				   candidate, nullptr, &extensionCount, available.data()
			   ),
			   "enumerate extensions"
		   ))
		{
			return false;
		}
		bool supported = true;
		for(auto required : extensions)
		{
			bool found = false;
			for(const auto& extension : available)
			{
				found |= std::string(extension.extensionName) == required;
			}
			supported &= found;
		}
		if(not supported)
		{
			continue;
		}
		uint32_t families = 0;
		vkGetPhysicalDeviceQueueFamilyProperties(candidate, &families, nullptr);
		std::vector<VkQueueFamilyProperties> properties(families);
		vkGetPhysicalDeviceQueueFamilyProperties(
			candidate, &families, properties.data()
		);
		for(uint32_t index = 0; index < families; ++index)
		{
			if(not(properties[index].queueFlags & VK_QUEUE_GRAPHICS_BIT))
			{
				continue;
			}
			physical = candidate;
			family = index;
			break;
		}
		if(physical)
		{
			break;
		}
	}
	if(not physical)
	{
		std::cerr << "No graphics GPU with external memory/semaphore support\n";
		return false;
	}
	const float priority = 1;
	VkDeviceQueueCreateInfo queueInfo{
		VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO
	};
	queueInfo.queueFamilyIndex = family;
	queueInfo.queueCount = 1;
	queueInfo.pQueuePriorities = &priority;
	VkDeviceCreateInfo deviceInfo{ VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO };
	deviceInfo.queueCreateInfoCount = 1;
	deviceInfo.pQueueCreateInfos = &queueInfo;
	deviceInfo.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
	deviceInfo.ppEnabledExtensionNames = extensions.data();
	if(not CheckVk(
		   vkCreateDevice(physical, &deviceInfo, nullptr, &device),
		   "vkCreateDevice"
	   ))
	{
		return false;
	}
	vkGetDeviceQueue(device, family, 0, &queue);
	return true;
}
VulkanDevice::~VulkanDevice()
{
	if(device)
	{
		vkDestroyDevice(device, nullptr);
	}
	if(instance)
	{
		vkDestroyInstance(instance, nullptr);
	}
}

bool
CubeCamera::Create()
{
	VkCommandPoolCreateInfo pool{ VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO };
	pool.queueFamilyIndex = m_gpu->family;
	if(not CheckVk(
		   vkCreateCommandPool(m_gpu->device, &pool, nullptr, &m_pool),
		   "create camera pool"
	   ))
	{
		return false;
	}
	VkCommandBufferAllocateInfo allocation{
		VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
	};
	allocation.commandPool = m_pool;
	allocation.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
	allocation.commandBufferCount = 1;
	if(not CheckVk(
		   vkAllocateCommandBuffers(m_gpu->device, &allocation, &m_commands),
		   "allocate camera commands"
	   ))
	{
		return false;
	}
	VkFenceCreateInfo fence{ VK_STRUCTURE_TYPE_FENCE_CREATE_INFO };
	if(not CheckVk(
		   vkCreateFence(m_gpu->device, &fence, nullptr, &m_fence),
		   "create camera fence"
	   ))
	{
		return false;
	}
	return CreatePipeline();
}

bool
CubeCamera::CreatePipeline()
{
	std::array<VkAttachmentDescription, 2> attachments{};
	attachments[0].format = VK_FORMAT_R8G8B8A8_UNORM;
	attachments[1].format = VK_FORMAT_D32_SFLOAT;
	for(size_t i = 0; i < 2; ++i)
	{
		auto& a = attachments[i];
		a.samples = VK_SAMPLE_COUNT_1_BIT;
		a.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
		a.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
		a.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
		a.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
		a.initialLayout =
			i == 0 ? VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
				   : VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
		a.finalLayout = a.initialLayout;
	}
	const VkAttachmentReference color{
		0, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
	};
	const VkAttachmentReference depth{
		1, VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
	};
	VkSubpassDescription subpass{};
	subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
	subpass.colorAttachmentCount = 1;
	subpass.pColorAttachments = &color;
	subpass.pDepthStencilAttachment = &depth;
	VkRenderPassCreateInfo pass{ VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO };
	pass.attachmentCount = 2;
	pass.pAttachments = attachments.data();
	pass.subpassCount = 1;
	pass.pSubpasses = &subpass;
	if(not CheckVk(
		   vkCreateRenderPass(m_gpu->device, &pass, nullptr, &m_pass),
		   "create cube render pass"
	   ))
	{
		return false;
	}
	const VkPushConstantRange push{ VK_SHADER_STAGE_VERTEX_BIT,
									0,
									sizeof(glm::mat4) };
	VkPipelineLayoutCreateInfo layout{
		VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO
	};
	layout.pushConstantRangeCount = 1;
	layout.pPushConstantRanges = &push;
	if(not CheckVk(
		   vkCreatePipelineLayout(m_gpu->device, &layout, nullptr, &m_layout),
		   "create cube layout"
	   ))
	{
		return false;
	}
	std::array<VkShaderModule, 2> modules{};
	bool loaded = true;
	const std::array<std::span<const uint32_t>, 2> shaders{
		cube_shaders::vert, cube_shaders::frag
	};
	for(size_t i = 0; i < 2; ++i)
	{
		VkShaderModuleCreateInfo shader{
			VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO
		};
		shader.codeSize = shaders[i].size_bytes();
		shader.pCode = shaders[i].data();
		if(not CheckVk(
			   vkCreateShaderModule(
				   m_gpu->device, &shader, nullptr, &modules[i]
			   ),
			   "create cube shader"
		   ))
		{
			loaded = false;
			break;
		}
	}
	std::array<VkPipelineShaderStageCreateInfo, 2> stages{};
	for(size_t i = 0; i < 2; ++i)
	{
		stages[i].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
		stages[i].stage =
			i == 0 ? VK_SHADER_STAGE_VERTEX_BIT : VK_SHADER_STAGE_FRAGMENT_BIT;
		stages[i].module = modules[i];
		stages[i].pName = "main";
	}
	VkPipelineVertexInputStateCreateInfo vertex{
		VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO
	};
	VkPipelineInputAssemblyStateCreateInfo assembly{
		VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO
	};
	assembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
	VkPipelineViewportStateCreateInfo viewport{
		VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO
	};
	viewport.viewportCount = viewport.scissorCount = 1;
	VkPipelineRasterizationStateCreateInfo raster{
		VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO
	};
	raster.polygonMode = VK_POLYGON_MODE_FILL;
	raster.cullMode = VK_CULL_MODE_NONE;
	raster.lineWidth = 1;
	VkPipelineMultisampleStateCreateInfo samples{
		VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO
	};
	samples.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
	VkPipelineDepthStencilStateCreateInfo z{
		VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO
	};
	z.depthTestEnable = z.depthWriteEnable = VK_TRUE;
	z.depthCompareOp = VK_COMPARE_OP_LESS;
	VkPipelineColorBlendAttachmentState blend{};
	blend.colorWriteMask = 15;
	VkPipelineColorBlendStateCreateInfo blends{
		VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO
	};
	blends.attachmentCount = 1;
	blends.pAttachments = &blend;
	const std::array<VkDynamicState, 2> dynamic{ VK_DYNAMIC_STATE_VIEWPORT,
												 VK_DYNAMIC_STATE_SCISSOR };
	VkPipelineDynamicStateCreateInfo dynamics{
		VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO
	};
	dynamics.dynamicStateCount = 2;
	dynamics.pDynamicStates = dynamic.data();
	VkGraphicsPipelineCreateInfo pipeline{
		VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO
	};
	pipeline.stageCount = 2;
	pipeline.pStages = stages.data();
	pipeline.pVertexInputState = &vertex;
	pipeline.pInputAssemblyState = &assembly;
	pipeline.pViewportState = &viewport;
	pipeline.pRasterizationState = &raster;
	pipeline.pMultisampleState = &samples;
	pipeline.pDepthStencilState = &z;
	pipeline.pColorBlendState = &blends;
	pipeline.pDynamicState = &dynamics;
	pipeline.layout = m_layout;
	pipeline.renderPass = m_pass;
	const bool success = loaded
						 && CheckVk(
							 vkCreateGraphicsPipelines(
								 m_gpu->device,
								 VK_NULL_HANDLE,
								 1,
								 &pipeline,
								 nullptr,
								 &m_pipeline
							 ),
							 "create cube pipeline"
						 );
	for(auto module : modules)
	{
		if(module)
		{
			vkDestroyShaderModule(m_gpu->device, module, nullptr);
		}
	}
	return success;
}

void
CubeCamera::ClearTargets()
{
	for(auto buffer : m_buffers)
	{
		vkDestroyFramebuffer(m_gpu->device, buffer, nullptr);
	}
	for(auto view : m_views)
	{
		vkDestroyImageView(m_gpu->device, view, nullptr);
	}
	m_buffers.clear();
	m_views.clear();
}

// OpenXR signs, forward Vulkan Z [0,1]. View poses are already in app content
// space.
static glm::mat4
ViewProjection(const QarPose& pose, const QarFov& fov)
{
	const float l = std::tan(fov.angle_left), r = std::tan(fov.angle_right);
	const float b = std::tan(fov.angle_down), t = std::tan(fov.angle_up);
	const float n = .1f, f = 10.f;
	glm::mat4 projection(0);
	projection[0][0] = 2 / (r - l);
	projection[1][1] = 2 / (t - b);
	projection[2][0] = (r + l) / (r - l);
	projection[2][1] = (t + b) / (t - b);
	projection[2][2] = -f / (f - n);
	projection[2][3] = -1;
	projection[3][2] = -f * n / (f - n);
	const auto& q = pose.orientation;
	const glm::mat4 camera =
		glm::translate(
			glm::mat4(1),
			glm::vec3(pose.position.x, pose.position.y, pose.position.z)
		)
		* glm::mat4_cast(glm::quat(q.w, q.x, q.y, q.z));
	return projection * glm::inverse(camera);
}

bool
CubeCamera::Submit(
	QarVideoFrameVulkan& frame,
	const std::array<CameraView, 2>& cameras,
	float seconds
)
{
	if(m_pending)
	{
		std::cerr << "Camera still has submitted work\n";
		return false;
	}
	ClearTargets();
	if(not CheckVk(
		   vkResetCommandPool(m_gpu->device, m_pool, 0), "reset camera pool"
	   ))
	{
		return false;
	}
	VkCommandBufferBeginInfo begin{
		VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
	};
	begin.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
	if(not CheckVk(
		   vkBeginCommandBuffer(m_commands, &begin), "begin cube commands"
	   ))
	{
		return false;
	}
	std::vector<VkImageMemoryBarrier> acquire, release;
	for(size_t i = 0; i < frame.textures_count; ++i)
	{
		auto& texture = frame.textures[i];
		const bool depth = texture.size.format == QAR_PIXEL_FORMAT_D32_FLOAT;
		VkImageMemoryBarrier barrier{ VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER };
		barrier.oldLayout = texture.layout;
		barrier.newLayout =
			depth ? VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
				  : VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
		barrier.srcQueueFamilyIndex = texture.queue_family_index;
		barrier.dstQueueFamilyIndex = m_gpu->family;
		if(texture.queue_family_index == VK_QUEUE_FAMILY_IGNORED
		   || texture.queue_family_index == m_gpu->family)
		{
			barrier.srcQueueFamilyIndex = barrier.dstQueueFamilyIndex =
				VK_QUEUE_FAMILY_IGNORED;
		}
		barrier.dstAccessMask =
			depth ? VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
				  : VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
		barrier.image = texture.image;
		barrier.subresourceRange = { static_cast<VkImageAspectFlags>(
										 depth ? VK_IMAGE_ASPECT_DEPTH_BIT
											   : VK_IMAGE_ASPECT_COLOR_BIT
									 ),
									 0,
									 1,
									 0,
									 1 };
		acquire.push_back(barrier);
		barrier.srcAccessMask = barrier.dstAccessMask;
		barrier.dstAccessMask = 0;
		barrier.oldLayout = barrier.newLayout;
		barrier.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
		barrier.srcQueueFamilyIndex = m_gpu->family;
		barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_EXTERNAL;
		release.push_back(barrier);
	}
	vkCmdPipelineBarrier(
		m_commands,
		VK_PIPELINE_STAGE_ALL_COMMANDS_BIT,
		VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
			| VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
		0,
		0,
		nullptr,
		0,
		nullptr,
		static_cast<uint32_t>(acquire.size()),
		acquire.data()
	);
	// Explicit separated textures, color/depth pairs for left and right eyes.
	if(frame.texture_views_count != 4 || frame.textures_count != 4)
	{
		std::cerr << "Expected separated stereo color/depth layout\n";
		return false;
	}
	for(size_t eye = 0; eye < 2; ++eye)
	{
		const size_t colorIndex = eye * 2, depthIndex = colorIndex + 1;
		const auto& colorView = frame.texture_views[colorIndex];
		const auto& depthView = frame.texture_views[depthIndex];
		std::array<VkImageView, 2> views{};
		const std::array<size_t, 2> indices{ colorView.texture_index,
											 depthView.texture_index };
		for(size_t i = 0; i < 2; ++i)
		{
			VkImageViewCreateInfo view{
				VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO
			};
			view.image = frame.textures[indices[i]].image;
			view.viewType = VK_IMAGE_VIEW_TYPE_2D;
			view.format =
				i == 0 ? VK_FORMAT_R8G8B8A8_UNORM : VK_FORMAT_D32_SFLOAT;
			view.subresourceRange = { static_cast<VkImageAspectFlags>(
										  i == 0 ? VK_IMAGE_ASPECT_COLOR_BIT
												 : VK_IMAGE_ASPECT_DEPTH_BIT
									  ),
									  0,
									  1,
									  0,
									  1 };
			if(not CheckVk(
				   vkCreateImageView(m_gpu->device, &view, nullptr, &views[i]),
				   "create camera image view"
			   ))
			{
				return false;
			}
			m_views.push_back(views[i]);
		}
		const uint32_t width = colorView.end_x - colorView.start_x,
					   height = colorView.end_y - colorView.start_y;
		VkFramebufferCreateInfo framebuffer{
			VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO
		};
		framebuffer.renderPass = m_pass;
		framebuffer.attachmentCount = 2;
		framebuffer.pAttachments = views.data();
		framebuffer.width = width;
		framebuffer.height = height;
		framebuffer.layers = 1;
		VkFramebuffer buffer = VK_NULL_HANDLE;
		if(not CheckVk(
			   vkCreateFramebuffer(
				   m_gpu->device, &framebuffer, nullptr, &buffer
			   ),
			   "create camera framebuffer"
		   ))
		{
			return false;
		}
		m_buffers.push_back(buffer);
		const std::array<VkClearValue, 2> clears{
			VkClearValue{ .color = { { .08f, .08f, .1f, 1 } } },
			VkClearValue{ .depthStencil = { 1, 0 } }
		};
		VkRenderPassBeginInfo pass{ VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO };
		pass.renderPass = m_pass;
		pass.framebuffer = buffer;
		pass.renderArea.extent = { width, height };
		pass.clearValueCount = 2;
		pass.pClearValues = clears.data();
		const glm::mat4 placed =
			glm::translate(glm::mat4(1), glm::vec3(0, 0, 0));
		const glm::mat4 model = glm::scale(
			glm::rotate(
				placed, seconds * .8f, glm::normalize(glm::vec3(1, 1, 0))
			),
			glm::vec3(.3f)
		);
		const glm::mat4 mvp =
			ViewProjection(cameras[eye].pose, cameras[eye].fov) * model;
		const VkViewport viewport{ 0,
								   static_cast<float>(height),
								   static_cast<float>(width),
								   -static_cast<float>(height),
								   0,
								   1 };
		const VkRect2D scissor{ { 0, 0 }, { width, height } };
		vkCmdBeginRenderPass(m_commands, &pass, VK_SUBPASS_CONTENTS_INLINE);
		vkCmdSetViewport(m_commands, 0, 1, &viewport);
		vkCmdSetScissor(m_commands, 0, 1, &scissor);
		vkCmdBindPipeline(
			m_commands, VK_PIPELINE_BIND_POINT_GRAPHICS, m_pipeline
		);
		vkCmdPushConstants(
			m_commands,
			m_layout,
			VK_SHADER_STAGE_VERTEX_BIT,
			0,
			sizeof(mvp),
			&mvp
		);
		vkCmdDraw(m_commands, 36, 1, 0, 0);
		vkCmdEndRenderPass(m_commands);
	}
	vkCmdPipelineBarrier(
		m_commands,
		VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
			| VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
			| VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
		VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
		0,
		0,
		nullptr,
		0,
		nullptr,
		static_cast<uint32_t>(release.size()),
		release.data()
	);
	if(not CheckVk(vkEndCommandBuffer(m_commands), "end cube commands"))
	{
		return false;
	}
	if(not CheckVk(
		   vkResetFences(m_gpu->device, 1, &m_fence), "reset camera fence"
	   ))
	{
		return false;
	}
	VkSubmitInfo submit{ VK_STRUCTURE_TYPE_SUBMIT_INFO };
	submit.commandBufferCount = 1;
	submit.pCommandBuffers = &m_commands;
	if(frame.synchronization.semaphore)
	{
		submit.signalSemaphoreCount = 1;
		submit.pSignalSemaphores = &frame.synchronization.semaphore;
	}
	if(not CheckVk(
		   vkQueueSubmit(m_gpu->queue, 1, &submit, m_fence), "submit cube frame"
	   ))
	{
		return false;
	}
	m_pending = true;
	for(size_t i = 0; i < frame.textures_count; ++i)
	{
		frame.textures[i].layout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
		frame.textures[i].queue_family_index = VK_QUEUE_FAMILY_EXTERNAL;
	}
	frame.synchronization.fence = VK_NULL_HANDLE;
	return true;
}
bool
CubeCamera::Discard(QarVideoFrameVulkan& frame)
{
	// A begun cycle that could not render must still signal the borrowed
	// semaphore so the sender can consume it on its discard/teardown path.
	if(m_pending)
	{
		return Drain();
	}
	if(not CheckVk(
		   vkResetFences(m_gpu->device, 1, &m_fence), "reset discard fence"
	   ))
	{
		return false;
	}
	VkSubmitInfo submit{ VK_STRUCTURE_TYPE_SUBMIT_INFO };
	if(frame.synchronization.semaphore)
	{
		submit.signalSemaphoreCount = 1;
		submit.pSignalSemaphores = &frame.synchronization.semaphore;
	}
	if(not CheckVk(
		   vkQueueSubmit(m_gpu->queue, 1, &submit, m_fence),
		   "signal discarded camera frame"
	   ))
	{
		return false;
	}
	m_pending = true;
	return Drain();
}
bool
CubeCamera::Complete(bool& ready)
{
	ready = false;
	const auto status = vkGetFenceStatus(m_gpu->device, m_fence);
	if(status == VK_NOT_READY)
	{
		return true;
	}
	if(not CheckVk(status, "poll cube frame"))
	{
		return false;
	}
	ready = true;
	m_pending = false;
	return true;
}
bool
CubeCamera::Drain()
{
	if(not m_pending)
	{
		return true;
	}
	if(not CheckVk(
		   vkWaitForFences(m_gpu->device, 1, &m_fence, VK_TRUE, UINT64_MAX),
		   "drain camera submission"
	   ))
	{
		return false;
	}
	m_pending = false;
	return true;
}
CubeCamera::~CubeCamera()
{
	if(not Drain())
	{
		std::cerr << "Camera drain failed during teardown\n";
	}
	ClearTargets();
	if(m_pipeline)
	{
		vkDestroyPipeline(m_gpu->device, m_pipeline, nullptr);
	}
	if(m_layout)
	{
		vkDestroyPipelineLayout(m_gpu->device, m_layout, nullptr);
	}
	if(m_pass)
	{
		vkDestroyRenderPass(m_gpu->device, m_pass, nullptr);
	}
	if(m_fence)
	{
		vkDestroyFence(m_gpu->device, m_fence, nullptr);
	}
	if(m_pool)
	{
		vkDestroyCommandPool(m_gpu->device, m_pool, nullptr);
	}
}
