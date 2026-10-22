use std::cell::Cell;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use qaros_sys::{
    QarPixelFormat, QarPose, QarVideoFrameCpu, QarVideoFrameLayout, QarVideoFrameVulkan,
    QarVideoTextureCpu, defaults,
};

use crate::ffi::{
    ExtRenderReceiver, ExtRenderReceiverInit, ExtRenderRequest, ExtVideoFrameInfo, ExtVideoReceiver,
};
use crate::hub::PeerInner;
use crate::vulkan::DeviceInner;
use crate::{Error, ExtApi, Result, VulkanDevice, stamped_header, structure_types};

/// Caller-owned CPU buffers a receiver writes one frame into, one tightly
/// packed buffer per texture of the receiver's layout.
#[derive(Debug)]
pub struct CpuFrameBuffers {
    layout: QarVideoFrameLayout,
    textures: Vec<Vec<u8>>,
    pitches: Vec<u32>,
}

impl CpuFrameBuffers {
    pub(crate) fn for_layout(layout: &QarVideoFrameLayout) -> Result<Self> {
        let texture_count = layout.textures_count.min(layout.textures.len());
        let mut textures = Vec::with_capacity(texture_count);
        let mut pitches = Vec::with_capacity(texture_count);
        for size in &layout.textures[..texture_count] {
            let pitch = size.width * bytes_per_texel(size.format)?;
            let layer_count = size.array_layers.max(1);
            let byte_count =
                usize::try_from(u64::from(pitch) * u64::from(size.height) * u64::from(layer_count))
                    .map_err(|_| Error::InvalidArgument("texture too large".to_string()))?;
            textures.push(vec![0_u8; byte_count]);
            pitches.push(pitch);
        }
        Ok(Self {
            layout: *layout,
            textures,
            pitches,
        })
    }

    pub fn texture(&self, index: usize) -> Option<&[u8]> {
        self.textures.get(index).map(Vec::as_slice)
    }

    pub fn pitch(&self, index: usize) -> Option<u32> {
        self.pitches.get(index).copied()
    }

    /// The `QarVideoFrameCpu` describing these buffers. The pointers stay
    /// valid while `self` is neither moved nor dropped.
    fn as_raw(&mut self) -> QarVideoFrameCpu {
        // SAFETY: an all-zero QarVideoFrameCpu is a valid empty frame (null
        // pointers, zero counts, zero enum values).
        let mut frame: QarVideoFrameCpu = unsafe { std::mem::zeroed() };
        let view_count = self.layout.views_count.min(self.layout.views.len());
        frame.texture_views[..view_count].copy_from_slice(&self.layout.views[..view_count]);
        frame.texture_views_count = view_count;
        for (index, data) in self.textures.iter_mut().enumerate() {
            frame.textures[index] = QarVideoTextureCpu {
                size: self.layout.textures[index],
                pitch: self.pitches[index],
                texture_data: data.as_mut_ptr(),
                texture_data_size: data.len(),
            };
        }
        frame.textures_count = self.textures.len();
        frame
    }
}

pub(crate) fn bytes_per_texel(format: QarPixelFormat) -> Result<u32> {
    match format {
        QarPixelFormat::QAR_PIXEL_FORMAT_R8G8B8A8
        | QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8
        | QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8X8
        | QarPixelFormat::QAR_PIXEL_FORMAT_R32_FLOAT
        | QarPixelFormat::QAR_PIXEL_FORMAT_D32_FLOAT => Ok(4),
        QarPixelFormat::QAR_PIXEL_FORMAT_R16G16B16A16 => Ok(8),
        other => Err(Error::InvalidArgument(format!(
            "no texel size known for pixel format {}",
            other.0
        ))),
    }
}

/// A render receiver on an extension peer, with the video receiver its
/// frames arrive on. For CPU receivers it owns the buffers it lends, so a
/// lent frame can never outlive them: the receiver is destroyed first.
#[derive(Debug)]
pub struct RenderReceiver {
    handle: NonNull<ExtRenderReceiver>,
    video: NonNull<ExtVideoReceiver>,
    cpu_buffers: Option<CpuFrameBuffers>,
    /// Set while the extension may write into `cpu_buffers`: from an accepted
    /// lend until `receive_frame` reports the frame.
    cpu_frame_lent: Cell<bool>,
    /// A Vulkan receiver's device, kept alive as long as the receiver.
    _vulkan_device: Option<Rc<DeviceInner>>,
    peer: Rc<PeerInner>,
}

impl RenderReceiver {
    pub(crate) fn create(
        peer: &Rc<PeerInner>,
        layout: &QarVideoFrameLayout,
        vulkan: Option<&VulkanDevice>,
    ) -> Result<Self> {
        let api = &peer.hub.api;
        let cpu_buffers = if vulkan.is_none() {
            Some(CpuFrameBuffers::for_layout(layout)?)
        } else {
            None
        };
        let mut vulkan_params = vulkan.map(VulkanDevice::params).transpose()?;
        let mut init = ExtRenderReceiverInit {
            header: stamped_header(structure_types::RENDER_RECEIVER_INIT),
            optimal_layout: *layout,
        };
        if let Some(params) = vulkan_params.as_mut() {
            init.header.next = (&raw mut params.header).cast();
        }
        let mut handle: *mut ExtRenderReceiver = ptr::null_mut();
        // SAFETY: live peer; init and its chain outlive the call.
        let result = unsafe {
            (api.qar_impl_ext_render_receiver_create)(
                peer.handle.as_ptr(),
                &raw const init,
                &raw mut handle,
            )
        };
        api.check("qar_ext_render_receiver_create", result)?;
        let handle =
            NonNull::new(handle).ok_or(Error::MissingHandle("qar_ext_render_receiver_create"))?;
        let mut video: *mut ExtVideoReceiver = ptr::null_mut();
        // SAFETY: live receiver; valid out-pointer.
        let result =
            unsafe { (api.qar_impl_ext_render_receiver_video)(handle.as_ptr(), &raw mut video) };
        let checked = api
            .check("qar_ext_render_receiver_video", result)
            .and_then(|()| {
                NonNull::new(video).ok_or(Error::MissingHandle("qar_ext_render_receiver_video"))
            });
        let video = match checked {
            Ok(video) => video,
            Err(error) => {
                // SAFETY: created above, not yet owned by a value.
                unsafe { (api.qar_impl_ext_render_receiver_handle_destroy)(handle.as_ptr()) };
                return Err(error);
            }
        };
        Ok(Self {
            handle,
            video,
            cpu_buffers,
            cpu_frame_lent: Cell::new(false),
            _vulkan_device: vulkan.map(|device| Rc::clone(&device.inner)),
            peer: Rc::clone(peer),
        })
    }

    fn api(&self) -> &ExtApi {
        &self.peer.hub.api
    }

    /// Waits until a sender has answered; `false` on timeout.
    pub fn wait_connected(&self, timeout: Duration) -> Result<bool> {
        let mut connected = false;
        // SAFETY: live video receiver; valid out-pointer.
        let result = unsafe {
            (self.api().qar_impl_ext_video_receiver_wait_connected)(
                self.video.as_ptr(),
                milliseconds(timeout),
                &raw mut connected,
            )
        };
        self.api()
            .check("qar_ext_video_receiver_wait_connected", result)?;
        Ok(connected)
    }

    /// The layout the sender writes, once its stream record states one.
    pub fn source_layout(&self) -> Result<Option<QarVideoFrameLayout>> {
        let mut layout = defaults::qar_video_frame_layout_default();
        let mut known = false;
        // SAFETY: live video receiver; valid out-pointers.
        let result = unsafe {
            (self.api().qar_impl_ext_video_receiver_source_layout)(
                self.video.as_ptr(),
                &raw mut layout,
                &raw mut known,
            )
        };
        self.api()
            .check("qar_ext_video_receiver_source_layout", result)?;
        Ok(known.then_some(layout))
    }

    /// Publishes a head pose and a render request; the sender renders one
    /// frame for it. `frame_count` is echoed back by the sender.
    pub fn request_render(&self, head_pose: QarPose, frame_count: Option<u64>) -> Result<()> {
        let request = ExtRenderRequest {
            head_pose,
            frame_count: frame_count.unwrap_or(u64::MAX),
        };
        // SAFETY: live receiver; the request is read during the call.
        let result = unsafe {
            (self.api().qar_impl_ext_render_receiver_request_render)(
                self.handle.as_ptr(),
                &raw const request,
            )
        };
        self.api()
            .check("qar_ext_render_receiver_request_render", result)
    }

    /// Lends the receiver's own CPU buffers for the next frame; `false`
    /// while not connected, paused, or while a frame is lent already.
    pub fn provide_cpu_frame(&mut self) -> Result<bool> {
        let video = self.video.as_ptr();
        let api = Arc::clone(&self.peer.hub.api);
        let buffers = self
            .cpu_buffers
            .as_mut()
            .ok_or_else(|| Error::InvalidArgument("not a CPU receiver".to_string()))?;
        let frame = buffers.as_raw();
        let mut accepted = false;
        // SAFETY: live video receiver; the buffers live in self and outlive
        // the lend, because the receiver is destroyed before they drop.
        let result = unsafe {
            (api.qar_impl_ext_video_receiver_try_provide_frame_cpu)(
                video,
                &raw const frame,
                &raw mut accepted,
            )
        };
        api.check("qar_ext_video_receiver_try_provide_frame_cpu", result)?;
        if accepted {
            self.cpu_frame_lent.set(true);
        }
        Ok(accepted)
    }

    /// Lends a Vulkan frame (from [`crate::VulkanTestFrame::lend`]).
    ///
    /// # Safety
    ///
    /// The frame's images must stay alive until the frame is received or the
    /// receiver is dropped.
    pub unsafe fn provide_vulkan_frame(&self, frame: &QarVideoFrameVulkan) -> Result<bool> {
        let mut accepted = false;
        // SAFETY: live video receiver; the caller keeps the images alive.
        let result = unsafe {
            (self
                .api()
                .qar_impl_ext_video_receiver_try_provide_frame_vulkan)(
                self.video.as_ptr(),
                frame,
                &raw mut accepted,
            )
        };
        self.api()
            .check("qar_ext_video_receiver_try_provide_frame_vulkan", result)?;
        Ok(accepted)
    }

    /// Waits for the lent frame; `None` on timeout (the frame stays lent).
    pub fn receive_frame(&self, timeout: Duration) -> Result<Option<ExtVideoFrameInfo>> {
        let mut info = ExtVideoFrameInfo {
            valid_frame: false,
            sequence_number: 0,
            creation_time_ns: 0,
        };
        let mut received = false;
        // SAFETY: live video receiver; valid out-pointers.
        let result = unsafe {
            (self.api().qar_impl_ext_video_receiver_receive_frame)(
                self.video.as_ptr(),
                milliseconds(timeout),
                &raw mut info,
                &raw mut received,
            )
        };
        self.api()
            .check("qar_ext_video_receiver_receive_frame", result)?;
        if received {
            self.cpu_frame_lent.set(false);
        }
        Ok(received.then_some(info))
    }

    /// The CPU buffers last written, for reading pixels after
    /// [`Self::receive_frame`]. `None` for a Vulkan receiver, and while a frame
    /// is lent and not yet received: the extension may be writing into them.
    pub fn cpu_buffers(&self) -> Option<&CpuFrameBuffers> {
        if self.cpu_frame_lent.get() {
            return None;
        }
        self.cpu_buffers.as_ref()
    }

    /// The last Vulkan frame handed back, with the image layouts and queue
    /// families it was left in and its completion semaphore.
    pub fn last_frame_vulkan(&self) -> Result<QarVideoFrameVulkan> {
        // SAFETY: an all-zero frame record is a valid empty value.
        let mut frame: QarVideoFrameVulkan = unsafe { std::mem::zeroed() };
        // SAFETY: live video receiver; valid out-pointer.
        let result = unsafe {
            (self.api().qar_impl_ext_video_receiver_last_frame_vulkan)(
                self.video.as_ptr(),
                &raw mut frame,
            )
        };
        self.api()
            .check("qar_ext_video_receiver_last_frame_vulkan", result)?;
        Ok(frame)
    }

    pub fn pause(&self) -> Result<()> {
        // SAFETY: live video receiver.
        let result = unsafe { (self.api().qar_impl_ext_video_receiver_pause)(self.video.as_ptr()) };
        self.api().check("qar_ext_video_receiver_pause", result)
    }

    pub fn resume(&self) -> Result<()> {
        // SAFETY: live video receiver.
        let result =
            unsafe { (self.api().qar_impl_ext_video_receiver_resume)(self.video.as_ptr()) };
        self.api().check("qar_ext_video_receiver_resume", result)
    }
}

impl Drop for RenderReceiver {
    fn drop(&mut self) {
        // SAFETY: owned handle; this also destroys the video receiver and
        // releases a lent frame before the CPU buffers drop.
        unsafe {
            (self
                .peer
                .hub
                .api
                .qar_impl_ext_render_receiver_handle_destroy)(self.handle.as_ptr());
        }
    }
}

fn milliseconds(timeout: Duration) -> u32 {
    u32::try_from(timeout.as_millis()).unwrap_or(u32::MAX)
}
