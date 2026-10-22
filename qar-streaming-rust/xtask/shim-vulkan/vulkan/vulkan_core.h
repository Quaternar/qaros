/* Minimal stand-in for vulkan_core.h, used to generate bindings and to compile
 * the conformance checks without the Vulkan SDK. Declares only what
 * qar_streaming.h uses, with the layouts the real header has on 64-bit
 * targets: dispatchable handles are pointers, non-dispatchable handles are
 * 64-bit integers (the real header's VK_USE_64_BIT_PTR_DEFINES=0 form, which
 * has the same size and alignment as its pointer form), enums are 32-bit.
 * The integer form matches the representation of the `ash` crate's handle
 * types, so callers pass `ash::vk` handles through unchanged. */
#ifndef QAROS_SHIM_VULKAN_CORE_H
#define QAROS_SHIM_VULKAN_CORE_H

#include <stdint.h>

#define VK_NULL_HANDLE 0

typedef struct VkPhysicalDevice_T* VkPhysicalDevice;
typedef struct VkDevice_T* VkDevice;

typedef uint64_t VkImage;
typedef uint64_t VkDeviceMemory;
typedef uint64_t VkSemaphore;
typedef uint64_t VkFence;

typedef uint32_t VkFlags;
typedef VkFlags VkFormatFeatureFlags;
typedef int32_t VkImageLayout;

#endif
