#if NET8_0_OR_GREATER
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using Quaternar.Qaros;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// An application Vulkan device the way a Unity player has one: Vulkan 1.1, no layers, and only
    /// the device extensions asked for. Raw vulkan-1.dll calls, so the tests need no Vulkan
    /// package. It takes the first physical device with a graphics queue, which must be the GPU the
    /// session runs on (the sender refuses a device on another GPU).
    /// </summary>
    internal sealed class VulkanTestDevice : IDisposable
    {
        public const string ExternalMemoryWin32 = "VK_KHR_external_memory_win32";
        public const string ExternalSemaphoreWin32 = "VK_KHR_external_semaphore_win32";

        private const string Vulkan = "vulkan-1";
        private const uint ApiVersion11 = (1u << 22) | (1u << 12);
        private const uint QueueGraphicsBit = 0x1;
        private const uint ImageAspectColor = 0x1;
        private const uint ImageAspectDepth = 0x2;
        private const uint ImageAspectStencil = 0x4;
        private const uint PipelineStageAllCommands = 0x10000;
        private const uint PipelineStageBottomOfPipe = 0x2000;
        private const uint AccessMemoryWrite = 0x10000;
        private const uint CommandBufferUsageOneTimeSubmit = 0x1;

        private IntPtr _instance;
        private IntPtr _device;
        private ulong _commandPool;
        private ulong _fence;

        private VulkanTestDevice()
        {
        }

        public IntPtr PhysicalDevice { get; private set; }
        public IntPtr Device { get { return _device; } }
        public uint QueueFamily { get; private set; }
        public IntPtr Queue { get; private set; }

        public static VulkanTestDevice Create(params string[] deviceExtensions)
        {
            var result = new VulkanTestDevice();
            try
            {
                result.CreateInstanceAndDevice(deviceExtensions);
                return result;
            }
            catch
            {
                result.Dispose();
                throw;
            }
        }

        /// <summary>
        /// Moves every image of the frame to TRANSFER_SRC_OPTIMAL, releases it to
        /// VK_QUEUE_FAMILY_EXTERNAL and signals the frame's semaphore, in one submit on this
        /// device's queue, then waits for it. The contents are left undefined.
        /// </summary>
        public void ReleaseToExternal(VideoFrameVulkan frame)
        {
            var barriers = new ImageMemoryBarrier[frame.Textures.Length];
            for (var i = 0; i < barriers.Length; i++)
            {
                var texture = frame.Textures[i];
                barriers[i] = new ImageMemoryBarrier
                {
                    SType = 45,
                    SrcAccessMask = AccessMemoryWrite,
                    OldLayout = VulkanConstants.ImageLayoutUndefined,
                    NewLayout = VulkanConstants.ImageLayoutTransferSrcOptimal,
                    SrcQueueFamilyIndex = QueueFamily,
                    DstQueueFamilyIndex = VulkanConstants.QueueFamilyExternal,
                    Image = texture.Image,
                    AspectMask = AspectOf(texture.Size.Format),
                    LevelCount = 1,
                    LayerCount = Math.Max(1u, texture.Size.ArrayLayers)
                };
            }
            var allocate = new CommandBufferAllocateInfo { SType = 40, CommandPool = _commandPool, CommandBufferCount = 1 };
            Check(vkAllocateCommandBuffers(_device, ref allocate, out var commandBuffer), "vkAllocateCommandBuffers");
            var begin = new CommandBufferBeginInfo { SType = 42, Flags = CommandBufferUsageOneTimeSubmit };
            Check(vkBeginCommandBuffer(commandBuffer, ref begin), "vkBeginCommandBuffer");
            vkCmdPipelineBarrier(commandBuffer, PipelineStageAllCommands, PipelineStageBottomOfPipe, 0, 0, IntPtr.Zero, 0, IntPtr.Zero, (uint)barriers.Length, barriers);
            Check(vkEndCommandBuffer(commandBuffer), "vkEndCommandBuffer");
            var commandBuffers = GCHandle.Alloc(new[] { commandBuffer }, GCHandleType.Pinned);
            var semaphores = GCHandle.Alloc(new[] { frame.Semaphore }, GCHandleType.Pinned);
            try
            {
                var submit = new SubmitInfo
                {
                    SType = 4,
                    CommandBufferCount = 1,
                    PCommandBuffers = commandBuffers.AddrOfPinnedObject(),
                    SignalSemaphoreCount = 1,
                    PSignalSemaphores = semaphores.AddrOfPinnedObject()
                };
                Check(vkResetFences(_device, 1, new[] { _fence }), "vkResetFences");
                Check(vkQueueSubmit(Queue, 1, new[] { submit }, _fence), "vkQueueSubmit");
                Check(vkWaitForFences(_device, 1, new[] { _fence }, 1, ulong.MaxValue), "vkWaitForFences");
            }
            finally
            {
                commandBuffers.Free();
                semaphores.Free();
            }
            vkFreeCommandBuffers(_device, _commandPool, 1, new[] { commandBuffer });
        }

        public void Dispose()
        {
            if (_device != IntPtr.Zero)
            {
                vkDeviceWaitIdle(_device);
                if (_fence != 0)
                    vkDestroyFence(_device, _fence, IntPtr.Zero);
                if (_commandPool != 0)
                    vkDestroyCommandPool(_device, _commandPool, IntPtr.Zero);
                vkDestroyDevice(_device, IntPtr.Zero);
                _device = IntPtr.Zero;
            }
            if (_instance != IntPtr.Zero)
            {
                vkDestroyInstance(_instance, IntPtr.Zero);
                _instance = IntPtr.Zero;
            }
        }

        private void CreateInstanceAndDevice(string[] deviceExtensions)
        {
            var allocations = new List<IntPtr>();
            try
            {
                var name = Marshal.StringToHGlobalAnsi("QarStreamingSharpVulkanTest");
                allocations.Add(name);
                var application = Alloc(new ApplicationInfo { PApplicationName = name, PEngineName = name, ApiVersion = ApiVersion11 }, allocations);
                var instanceInfo = new InstanceCreateInfo { SType = 1, PApplicationInfo = application };
                Check(vkCreateInstance(ref instanceInfo, IntPtr.Zero, out _instance), "vkCreateInstance");
                uint count = 0;
                Check(vkEnumeratePhysicalDevices(_instance, ref count, null), "vkEnumeratePhysicalDevices");
                var physicalDevices = new IntPtr[count];
                Check(vkEnumeratePhysicalDevices(_instance, ref count, physicalDevices), "vkEnumeratePhysicalDevices");
                foreach (var candidate in physicalDevices)
                {
                    uint familyCount = 0;
                    vkGetPhysicalDeviceQueueFamilyProperties(candidate, ref familyCount, null);
                    var families = new QueueFamilyProperties[familyCount];
                    vkGetPhysicalDeviceQueueFamilyProperties(candidate, ref familyCount, families);
                    var family = Array.FindIndex(families, f => (f.QueueFlags & QueueGraphicsBit) != 0);
                    if (family < 0)
                        continue;
                    PhysicalDevice = candidate;
                    QueueFamily = (uint)family;
                    break;
                }
                if (PhysicalDevice == IntPtr.Zero)
                    throw new InvalidOperationException("No Vulkan physical device with a graphics queue.");
                var priority = Alloc(1.0f, allocations);
                var queueInfo = Alloc(new DeviceQueueCreateInfo { SType = 2, QueueFamilyIndex = QueueFamily, QueueCount = 1, PQueuePriorities = priority }, allocations);
                var extensionNames = new IntPtr[deviceExtensions.Length];
                for (var i = 0; i < deviceExtensions.Length; i++)
                {
                    extensionNames[i] = Marshal.StringToHGlobalAnsi(deviceExtensions[i]);
                    allocations.Add(extensionNames[i]);
                }
                var extensionArray = IntPtr.Zero;
                if (extensionNames.Length > 0)
                {
                    extensionArray = Marshal.AllocHGlobal(IntPtr.Size * extensionNames.Length);
                    allocations.Add(extensionArray);
                    Marshal.Copy(extensionNames, 0, extensionArray, extensionNames.Length);
                }
                var deviceInfo = new DeviceCreateInfo
                {
                    SType = 3,
                    QueueCreateInfoCount = 1,
                    PQueueCreateInfos = queueInfo,
                    EnabledExtensionCount = (uint)extensionNames.Length,
                    PpEnabledExtensionNames = extensionArray
                };
                Check(vkCreateDevice(PhysicalDevice, ref deviceInfo, IntPtr.Zero, out _device), "vkCreateDevice");
                vkGetDeviceQueue(_device, QueueFamily, 0, out var queue);
                Queue = queue;
                var poolInfo = new CommandPoolCreateInfo { SType = 39, QueueFamilyIndex = QueueFamily };
                Check(vkCreateCommandPool(_device, ref poolInfo, IntPtr.Zero, out _commandPool), "vkCreateCommandPool");
                var fenceInfo = new FenceCreateInfo { SType = 8 };
                Check(vkCreateFence(_device, ref fenceInfo, IntPtr.Zero, out _fence), "vkCreateFence");
            }
            finally
            {
                foreach (var allocation in allocations)
                    Marshal.FreeHGlobal(allocation);
            }
        }

        private static uint AspectOf(PixelFormat format)
        {
            switch (format)
            {
                case PixelFormat.D16Unorm:
                case PixelFormat.D32Float:
                    return ImageAspectDepth;
                case PixelFormat.D24UnormS8Uint:
                case PixelFormat.D32FloatS8X24Uint:
                    return ImageAspectDepth | ImageAspectStencil;
                default:
                    return ImageAspectColor;
            }
        }

        private static IntPtr Alloc<T>(T value, List<IntPtr> allocations)
            where T : struct
        {
            var pointer = Marshal.AllocHGlobal(Marshal.SizeOf<T>());
            allocations.Add(pointer);
            Marshal.StructureToPtr(value, pointer, false);
            return pointer;
        }

        private static void Check(int result, string call)
        {
            if (result != 0)
                throw new InvalidOperationException($"{call} failed with VkResult {result}.");
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct ApplicationInfo
        {
            public int SType;
            public IntPtr PNext;
            public IntPtr PApplicationName;
            public uint ApplicationVersion;
            public IntPtr PEngineName;
            public uint EngineVersion;
            public uint ApiVersion;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct InstanceCreateInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
            public IntPtr PApplicationInfo;
            public uint EnabledLayerCount;
            public IntPtr PpEnabledLayerNames;
            public uint EnabledExtensionCount;
            public IntPtr PpEnabledExtensionNames;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct QueueFamilyProperties
        {
            public uint QueueFlags;
            public uint QueueCount;
            public uint TimestampValidBits;
            public uint MinImageTransferGranularityWidth;
            public uint MinImageTransferGranularityHeight;
            public uint MinImageTransferGranularityDepth;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct DeviceQueueCreateInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
            public uint QueueFamilyIndex;
            public uint QueueCount;
            public IntPtr PQueuePriorities;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct DeviceCreateInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
            public uint QueueCreateInfoCount;
            public IntPtr PQueueCreateInfos;
            public uint EnabledLayerCount;
            public IntPtr PpEnabledLayerNames;
            public uint EnabledExtensionCount;
            public IntPtr PpEnabledExtensionNames;
            public IntPtr PEnabledFeatures;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct CommandPoolCreateInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
            public uint QueueFamilyIndex;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct CommandBufferAllocateInfo
        {
            public int SType;
            public IntPtr PNext;
            public ulong CommandPool;
            public int Level;
            public uint CommandBufferCount;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct CommandBufferBeginInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
            public IntPtr PInheritanceInfo;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct ImageMemoryBarrier
        {
            public int SType;
            public IntPtr PNext;
            public uint SrcAccessMask;
            public uint DstAccessMask;
            public int OldLayout;
            public int NewLayout;
            public uint SrcQueueFamilyIndex;
            public uint DstQueueFamilyIndex;
            public ulong Image;
            public uint AspectMask;
            public uint BaseMipLevel;
            public uint LevelCount;
            public uint BaseArrayLayer;
            public uint LayerCount;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct SubmitInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint WaitSemaphoreCount;
            public IntPtr PWaitSemaphores;
            public IntPtr PWaitDstStageMask;
            public uint CommandBufferCount;
            public IntPtr PCommandBuffers;
            public uint SignalSemaphoreCount;
            public IntPtr PSignalSemaphores;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct FenceCreateInfo
        {
            public int SType;
            public IntPtr PNext;
            public uint Flags;
        }

        [DllImport(Vulkan)] private static extern int vkCreateInstance(ref InstanceCreateInfo info, IntPtr allocator, out IntPtr instance);
        [DllImport(Vulkan)] private static extern void vkDestroyInstance(IntPtr instance, IntPtr allocator);
        [DllImport(Vulkan)] private static extern int vkEnumeratePhysicalDevices(IntPtr instance, ref uint count, [Out] IntPtr[]? devices);
        [DllImport(Vulkan)] private static extern void vkGetPhysicalDeviceQueueFamilyProperties(IntPtr physicalDevice, ref uint count, [Out] QueueFamilyProperties[]? properties);
        [DllImport(Vulkan)] private static extern int vkCreateDevice(IntPtr physicalDevice, ref DeviceCreateInfo info, IntPtr allocator, out IntPtr device);
        [DllImport(Vulkan)] private static extern void vkDestroyDevice(IntPtr device, IntPtr allocator);
        [DllImport(Vulkan)] private static extern int vkDeviceWaitIdle(IntPtr device);
        [DllImport(Vulkan)] private static extern void vkGetDeviceQueue(IntPtr device, uint family, uint index, out IntPtr queue);
        [DllImport(Vulkan)] private static extern int vkCreateCommandPool(IntPtr device, ref CommandPoolCreateInfo info, IntPtr allocator, out ulong pool);
        [DllImport(Vulkan)] private static extern void vkDestroyCommandPool(IntPtr device, ulong pool, IntPtr allocator);
        [DllImport(Vulkan)] private static extern int vkAllocateCommandBuffers(IntPtr device, ref CommandBufferAllocateInfo info, out IntPtr commandBuffer);
        [DllImport(Vulkan)] private static extern void vkFreeCommandBuffers(IntPtr device, ulong pool, uint count, IntPtr[] commandBuffers);
        [DllImport(Vulkan)] private static extern int vkBeginCommandBuffer(IntPtr commandBuffer, ref CommandBufferBeginInfo info);
        [DllImport(Vulkan)] private static extern int vkEndCommandBuffer(IntPtr commandBuffer);
        [DllImport(Vulkan)] private static extern void vkCmdPipelineBarrier(IntPtr commandBuffer, uint srcStage, uint dstStage, uint dependencyFlags, uint memoryBarrierCount, IntPtr memoryBarriers, uint bufferBarrierCount, IntPtr bufferBarriers, uint imageBarrierCount, [In] ImageMemoryBarrier[] imageBarriers);
        [DllImport(Vulkan)] private static extern int vkCreateFence(IntPtr device, ref FenceCreateInfo info, IntPtr allocator, out ulong fence);
        [DllImport(Vulkan)] private static extern void vkDestroyFence(IntPtr device, ulong fence, IntPtr allocator);
        [DllImport(Vulkan)] private static extern int vkResetFences(IntPtr device, uint count, ulong[] fences);
        [DllImport(Vulkan)] private static extern int vkWaitForFences(IntPtr device, uint count, ulong[] fences, uint waitAll, ulong timeout);
        [DllImport(Vulkan)] private static extern int vkQueueSubmit(IntPtr queue, uint count, [In] SubmitInfo[] submits, ulong fence);
    }
}
#endif

namespace Quaternar.Qaros.Tests
{
    /// <summary>The Vulkan values a render sender's frame is handed back with.</summary>
    internal static class VulkanConstants
    {
        /// <summary>VK_IMAGE_LAYOUT_UNDEFINED.</summary>
        public const int ImageLayoutUndefined = 0;
        /// <summary>VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL.</summary>
        public const int ImageLayoutTransferSrcOptimal = 6;
        /// <summary>VK_QUEUE_FAMILY_EXTERNAL.</summary>
        public const uint QueueFamilyExternal = 0xFFFFFFFE;
    }
}
