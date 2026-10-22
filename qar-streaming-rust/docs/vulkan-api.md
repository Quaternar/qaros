# The Vulkan path in `qaros`

The Rust API over QAROS's Vulkan render sender (header `qar_streaming.h`, example `qar-streaming-c/examples/vulkan_cube/`). Enable it with the `vulkan` feature:

```toml
qaros = { path = "<path to the binding>/crates/qaros", features = ["vulkan"] }
```

Handles are `ash` 0.38 types, the version `wgpu-hal` 30 uses, so a renderer on wgpu passes its `vk::PhysicalDevice`, `vk::Device` and the frame's `vk::Image` handles through unchanged.

## Types and functions

Adapter selection (always available, not behind the feature):

| Item | What it does |
| --- | --- |
| `qaros::GraphicsAdapter::{Luid([u8; 8]), Uuid([u8; 16])}` | `QarGraphicsDeviceId`. Windows takes the LUID, other platforms the UUID. |
| `Session::graphics_adapter()` | The session's adapter (`qar_session_get_graphics_device_id`): the one chained at onboard or rejoin, else the one QAROS or the library selected; `None` when the session knows none. Create the device there. |
| `GraphicsAdapter::parse(&str)` | 16 hex digits are a LUID, 32 a UUID, bytes in memory order (the cube's `AdapterIdHex`). |
| `GraphicsAdapter::matches(luid: Option<[u8; 8]>, uuid: [u8; 16])` | Compare with `VkPhysicalDeviceIDProperties` (`device_luid` if `device_luid_valid`, `device_uuid`) to pick the physical device. |
| `Display for GraphicsAdapter` | Lowercase hex, the format `parse` reads. |
| `Runtime::onboard_on_adapter(method, presentation, Option<GraphicsAdapter>, options)` | `onboard` with the adapter chained after the mode extension. |
| `Runtime::rejoin_on_adapter(onboarding_id, presentation, Option<GraphicsAdapter>, options)` | `rejoin` with the adapter chained into `QarRejoinInit`. |

The Vulkan sender and its frame (feature `vulkan`, module `qaros::vulkan`):

| Item | What it does |
| --- | --- |
| `vulkan::VulkanStreamParams { physical_device: vk::PhysicalDevice, device: vk::Device }` | `QarStreamParamsVulkan`. |
| `unsafe Session::create_vulkan_render_sender(&RenderSenderConfig, VulkanStreamParams, &BlockingOptions) -> Result<RenderSender>` | `qar_render_sender_create` with `QAR_GRAPHICS_API_VULKAN`. Same `RenderSender` type as the CPU path. |
| `vulkan::required_device_extensions() -> &'static [&'static CStr]` | `VK_KHR_external_memory_win32` and `VK_KHR_external_semaphore_win32` on Windows, `VK_KHR_external_memory_fd` on Linux, empty elsewhere. |
| `vulkan::assumed_vk_format(PixelFormat) -> Option<vk::Format>` | The header reports no `VkFormat`; UNORM / SFLOAT assumed (an open question for QuaternAR). |
| `vulkan::SHOWN_LAYOUT` | `vk::ImageLayout::TRANSFER_SRC_OPTIMAL`, the layout images are left in before `show`. |
| `RenderSender::vulkan_frame_state(&mut self) -> Result<VulkanFrameState<'_>>` | Read-only frame record outside a frame cycle, readable from creation (build image views up front). Fails while a frame is open or pending. |
| `RenderSender::try_begin_owned_frame(&mut self) -> Result<Option<OwnedFrame>>` | `qar_render_sender_try_begin_frame`, the cube's non-blocking begin. `None`: poll again. |
| `OwnedFrame::vulkan_frame(&mut self) -> Result<VulkanFrame<'_>>`, `Frame::vulkan_frame` | The frame record inside a cycle, writable. Derefs to `VulkanFrameState`. |
| `VulkanFrameState::views() -> Vec<VideoFrameView>` | Eye, color or depth, image index, rectangle, format per view. |
| `VulkanFrameState::view_index(Eye, ViewKind) -> Option<usize>` | The cube's `FindFrameView`. The index also selects the pose and FOV: `frame.view(index)`. |
| `VulkanFrameState::images() -> Vec<VulkanImage>`, `image(index)`, `image_count()` | Snapshots of the image records. |
| `VulkanImage { size: TextureSize, image: vk::Image, memory: vk::DeviceMemory, format_features: vk::FormatFeatureFlags, layout: vk::ImageLayout, queue_family_index: u32 }` | `QarVideoTextureVulkan`. `layout` and `queue_family_index` are the state to transition from. |
| `VulkanFrameState::semaphore() -> vk::Semaphore`, `fence() -> vk::Fence` | `QarSyncFrameVulkan`. |
| `VulkanFrame::set_image_state(index, vk::ImageLayout, queue_family: u32) -> Result<()>` | Write back one image's final state. |
| `VulkanFrame::release_to_external(vk::ImageLayout)` | Write back every image as `layout` + `VK_QUEUE_FAMILY_EXTERNAL`. |
| `VulkanFrame::set_fence(vk::Fence)` | The cube writes `vk::Fence::null()`. |

## Safety rules

- **Device.** `create_vulkan_render_sender` is `unsafe`: the physical device and device must be valid, the device created from that physical device on the adapter the session was onboarded or rejoined with, with `required_device_extensions()` enabled, and the device must outlive the sender and every handle taken from it.
- **Ownership.** QAROS owns the images, their memory and the semaphore. Never destroy, free or re-bind them. Image handles may change between frames (a layout change, for example): read the records every frame and rebuild views when a handle changes.
- **Frame pointer validity.** The C record stays valid "until the next `qar_render_sender_frame_vulkan` call or sender destruction". `VulkanFrameState` and `VulkanFrame` borrow the sender or the open frame mutably, so neither can happen while they live. Handles copied out stay valid while the sender lives.
- **Writes only inside a frame cycle.** Only `VulkanFrame` (from an open frame) writes; `VulkanFrameState` from the sender is read-only.
- **Order.** Submit the GPU work that signals `semaphore()`, then write back, then `show`. The borrow guarantees the write-back happens before `show`; the wrapper cannot see the submission, so submitting after `show` is a bug it cannot catch (a binary semaphore wait needs its signal submitted first).
- **Discarded frames.** A frame that is begun but not rendered must still signal the semaphore (an empty submission is enough) before it is dropped.
- **Threads.** Like every `qaros` handle, the sender and frames are not `Send`. QAROS may submit on its own queues; on the stub, see below.

## One frame

```rust
use ash::vk;
use qaros::vulkan::{SHOWN_LAYOUT, VulkanStreamParams};
use qaros::{BlockingOptions, Eye, FrameLayout, FrameViewSpec, RenderSenderConfig, ShowParams, ViewKind};

// Session onboarded with runtime.onboard(..); the device was created on session.graphics_adapter()?.
let mut config = RenderSenderConfig::new(request.target_peer);
config.layout = FrameLayout::SeparatedTextures;
config.app_volume = Some(volume_id);
config.color_format = qaros::PixelFormat::R8G8B8A8;
config.views = vec![
    FrameViewSpec { kind: ViewKind::Color, eye: Eye::Left, texture_index: 0 },
    FrameViewSpec { kind: ViewKind::Depth, eye: Eye::Left, texture_index: 1 },
    FrameViewSpec { kind: ViewKind::Color, eye: Eye::Right, texture_index: 2 },
    FrameViewSpec { kind: ViewKind::Depth, eye: Eye::Right, texture_index: 3 },
];
// SAFETY: the device was created on the session's adapter with
// required_device_extensions() and outlives the sender.
let mut sender = unsafe {
    session.create_vulkan_render_sender(
        &config,
        VulkanStreamParams { physical_device, device: device.handle() },
        &BlockingOptions::default(),
    )?
};

if let Some(mut frame) = sender.try_begin_owned_frame()? {
    let mut vulkan_frame = frame.vulkan_frame()?;
    let left = vulkan_frame.view_index(Eye::Left, ViewKind::Color).expect("left color view");
    let images = vulkan_frame.images();
    // Record: barrier images[i].layout / images[i].queue_family_index -> attachment layout on
    // my family; render each eye with frame.view(index) pose and FOV (read before or after
    // borrowing vulkan_frame); barrier -> SHOWN_LAYOUT, release to vk::QUEUE_FAMILY_EXTERNAL.
    // Submit, signalling vulkan_frame.semaphore().
    vulkan_frame.release_to_external(SHOWN_LAYOUT);
    vulkan_frame.set_fence(vk::Fence::null());
    frame.show(&ShowParams { near: 0.1, far: 10.0, depth_scale: 1.0 })?;
}
```

`frame.view(index)` needs `&frame`, which the `VulkanFrame` borrows mutably until its last use: read the poses first, or fetch the `VulkanFrame` again afterwards (the second fetch returns the same record).

## Against `qaros-stub`

The stub emulates the path on the caller's device:

- At sender creation it creates one plain image per layout texture on the given device (optimal tiling, one mip, `assumed_vk_format`, color usage `COLOR_ATTACHMENT | SAMPLED | TRANSFER_SRC | TRANSFER_DST`, depth usage `DEPTH_STENCIL_ATTACHMENT | SAMPLED | TRANSFER_SRC | TRANSFER_DST`), dedicated memory, and one binary semaphore. No Win32 external memory.
- Initial records: `layout = UNDEFINED`, `queue_family_index = VK_QUEUE_FAMILY_IGNORED`. Later frames report what the previous frame wrote back. The real runtime (package of 2026-10-12) reports `TRANSFER_SRC_OPTIMAL` with `VK_QUEUE_FAMILY_EXTERNAL` from the first frame on, so code must acquire from the reported state rather than assume either.
- At `show_frame` it logs `vulkan write-back ok` or `ERROR vulkan write-back ...` (every image must be `TRANSFER_SRC_OPTIMAL` + `VK_QUEUE_FAMILY_EXTERNAL`), then consumes the semaphore: it submits a wait on queue 0 of queue family `QAROS_STUB_VULKAN_QUEUE_FAMILY` (default 0) with its own fence and waits up to 5 s for it (`ERROR vulkan semaphore not signalled` on timeout). That submission and wait run on the calling thread: do not submit to that queue from another thread during `show`.
- A frame dropped without `show` leaves the semaphore in an unknown state; at the next begin (or sender destruction) the stub waits for the device to go idle and replaces the semaphore (`vulkan discarded frame: semaphore replaced`).
- It opens the Vulkan loader by the name `ash::Entry::load` (and so wgpu-hal) uses: `vulkan-1.dll` on Windows, `libvulkan.so.1` on Linux, the platform's own loader name elsewhere. The dynamic linker then hands back the loader the caller already loaded, so device-level functions dispatch through the same loader. Override the path with `QAROS_STUB_VULKAN_LIBRARY`.
- No frame-stamp pairing check on the Vulkan path: the stub does not read pixels back.
