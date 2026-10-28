//! The D3D11 render sender path (feature `d3d11`).
//!
//! The application creates a D3D11.1 device and immediate context on the adapter the session
//! uses ([`crate::Session::graphics_adapter`]) and hands both to QAROS
//! ([`D3D11StreamParams`], chained into `QarRenderSenderInit` as `QarStreamParamsD3D11`). QAROS
//! then creates the frame's shared textures on that device and hands them out with the views
//! laid out on them, as `QarVideoFrameD3D11`.
//!
//! One frame, in the order of QuaternAR's `d3d11_render_sender.c`:
//!
//! 1. Begin a frame ([`crate::RenderSender::begin_owned_frame`],
//!    [`crate::RenderSender::try_begin_owned_frame`], ...).
//! 2. [`crate::OwnedFrame::d3d11_frame`]: the views ([`D3D11Frame::view_index`] finds each
//!    eye's color and depth view) and the shared textures they lie on.
//! 3. When the sender was created with `acquire_keyed_mutex_sync` false: acquire each
//!    texture's keyed mutex with its acquire key ([`D3D11Frame::keyed_mutexes`]).
//! 4. Render each eye into its view's rectangle with the frame's view pose and field of view.
//! 5. With `acquire_keyed_mutex_sync` false: release each keyed mutex with its release key.
//! 6. [`crate::OwnedFrame::show`].
//!
//! # Safety rules
//!
//! These come from the header (`QarStreamParamsD3D11`, `QarSyncFrameD3D11`,
//! `qar_render_sender_frame_d3d11`) and the C sample; the wrapper enforces what a borrow or a
//! reference count can express and documents the rest.
//!
//! Items that take or lend `windows` crate types (`D3D11StreamParams::new`,
//! `D3D11Texture::texture`, `D3D11KeyedMutex::mutex`) exist on Windows only; macOS builds the
//! rest against `qaros-stub`, with raw pointers.
//!
//! - **Device lifetime.** `D3D11StreamParams::new` takes a reference on the device and the
//!   context (`AddRef`), and the sender keeps both until it is destroyed, and its session with
//!   it when the sender held the session's last reference. A creation that never calls back
//!   (on a runtime older than the header's exactly-once rule for async calls) keeps them for
//!   the rest of the process. [`D3D11StreamParams::from_raw`] takes no
//!   reference; the caller then keeps both valid that long.
//! - **The immediate context.** QAROS uses the context it is given. When and on which thread
//!   is not documented, and a D3D11 immediate context is not thread-safe. That is why sender
//!   creation is `unsafe`: use the context only on the thread that drives the sender (every
//!   `qaros` handle stays on one thread), or turn on the device's multithread protection
//!   (`ID3D11Multithread::SetMultithreadProtected`).
//! - **Ownership of the textures.** The shared textures and their keyed mutexes are the
//!   sender's. `D3D11Texture::texture` and `D3D11KeyedMutex::mutex` lend them for the
//!   borrow of the frame, without `AddRef` or `Release`; clone the interface to keep one
//!   longer. Read the frame every cycle: the header does not say the textures stay the same.
//! - **Writes only inside a frame cycle.** The frame record is readable from creation
//!   ([`crate::RenderSender::d3d11_frame_state`], for creating views up front); the textures
//!   may be rendered to only between a begin and its show (the header's "writable only inside
//!   a frame cycle"). The wrapper cannot see GPU work, so this rule is the caller's.
//! - **Keyed mutexes.** With `acquire_keyed_mutex_sync` true (the header's default) the
//!   library acquires and releases them around the cycle; do not touch them. With it false
//!   the application drives the chain with the record's keys; the header gives the keys and
//!   says the entries match the textures, an empty list meaning no external synchronization.
//! - **Device.** D3D11.1 (`ID3D11Device1`, `ID3D11DeviceContext1`), on the session's adapter:
//!   the header creates the session's devices, and with them every sender's frame textures,
//!   there.

use std::ffi::c_void;
use std::marker::PhantomData;

use qaros_sys::{self as sys, defaults};
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{ID3D11Device1, ID3D11DeviceContext1, ID3D11Texture2D};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::IDXGIKeyedMutex;
#[cfg(windows)]
use windows::core::Interface as _;

use crate::render::{TextureSize, VideoFrameView};
use crate::types::{Eye, ViewKind};

/// `QarStreamParamsD3D11`: the application's D3D11.1 device and immediate context, on which
/// QAROS creates the frame's shared textures, and how it creates and synchronizes them.
#[derive(Clone, Debug)]
pub struct D3D11StreamParams {
    device: DeviceReferences,
    /// `D3D11_BIND_FLAG` bits of the color textures; the C sample asks for
    /// `D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE`. The header's default is 0.
    pub color_bind_flags: u32,
    /// `D3D11_BIND_FLAG` bits of the depth textures, for example `D3D11_BIND_DEPTH_STENCIL`.
    /// QAROS creates them as `DXGI_FORMAT_D32_FLOAT`, a typed depth format, so
    /// `D3D11_BIND_SHADER_RESOURCE` makes sender creation fail with `E_INVALIDARG`. The header's
    /// default is 0.
    pub depth_bind_flags: u32,
    /// True (the header's default): the library acquires and releases each texture's keyed
    /// mutex around the frame cycle. False: the application drives the keyed-mutex chain with
    /// the keys in [`D3D11Frame::keyed_mutexes`], for example when it copies through a chain of
    /// shared textures.
    pub acquire_keyed_mutex_sync: bool,
}

/// The device and context pointers handed to QAROS, and on Windows the references that keep
/// them alive while a sender may use them.
#[derive(Clone, Debug)]
pub(crate) struct DeviceReferences {
    device: *mut sys::ID3D11Device1,
    context: *mut sys::ID3D11DeviceContext1,
    #[cfg(windows)]
    _held: Option<(ID3D11Device1, ID3D11DeviceContext1)>,
}

impl D3D11StreamParams {
    /// The header's defaults (`qar_stream_params_d3d11_default`: no bind flags, the library
    /// drives the keyed mutexes) for `device` and its immediate `context`. Takes a reference
    /// on both, which every sender created with these parameters keeps.
    #[cfg(windows)]
    pub fn new(device: &ID3D11Device1, context: &ID3D11DeviceContext1) -> Self {
        Self::with_defaults(DeviceReferences {
            device: device.as_raw().cast::<sys::ID3D11Device1>(),
            context: context.as_raw().cast::<sys::ID3D11DeviceContext1>(),
            _held: Some((device.clone(), context.clone())),
        })
    }

    /// The header's defaults for a device and context given as raw pointers, from another
    /// D3D11 binding or a test. `None` when either is null. No reference is taken.
    ///
    /// # Safety
    ///
    /// `device` is an `ID3D11Device1` and `context` its immediate `ID3D11DeviceContext1`, and
    /// both stay valid for as long as a sender created with these parameters may use them:
    /// until its session is dropped, and for a creation started with
    /// [`crate::Session::start_d3d11_render_sender`] as described there.
    pub unsafe fn from_raw(
        device: *mut sys::ID3D11Device1,
        context: *mut sys::ID3D11DeviceContext1,
    ) -> Option<Self> {
        if device.is_null() || context.is_null() {
            return None;
        }
        Some(Self::with_defaults(DeviceReferences {
            device,
            context,
            #[cfg(windows)]
            _held: None,
        }))
    }

    fn with_defaults(device: DeviceReferences) -> Self {
        let raw = defaults::qar_stream_params_d3d11_default();
        Self {
            device,
            color_bind_flags: raw.color_bind_flags,
            depth_bind_flags: raw.depth_bind_flags,
            acquire_keyed_mutex_sync: raw.acquire_keyed_mutex_sync,
        }
    }

    /// The device pointer handed to QAROS.
    pub fn device_raw(&self) -> *mut sys::ID3D11Device1 {
        self.device.device
    }

    /// The immediate context pointer handed to QAROS.
    pub fn context_raw(&self) -> *mut sys::ID3D11DeviceContext1 {
        self.device.context
    }

    pub(crate) fn to_raw(&self) -> sys::QarStreamParamsD3D11 {
        let mut raw = defaults::qar_stream_params_d3d11_default();
        raw.d3d11_device = self.device.device;
        raw.d3d11_context = self.device.context;
        raw.color_bind_flags = self.color_bind_flags;
        raw.depth_bind_flags = self.depth_bind_flags;
        raw.acquire_keyed_mutex_sync = self.acquire_keyed_mutex_sync;
        raw
    }

    pub(crate) fn references(&self) -> DeviceReferences {
        self.device.clone()
    }
}

/// One shared texture of the frame (`QarVideoTextureD3D11`), lent for the borrow of the frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11Texture<'frame> {
    pub size: TextureSize,
    raw: *mut c_void,
    _frame: PhantomData<&'frame ()>,
}

impl D3D11Texture<'_> {
    /// The `ID3D11Texture2D` pointer the record carries. The sender owns it.
    pub fn as_raw(&self) -> *mut sys::ID3D11Texture2D {
        self.raw.cast()
    }

    /// The texture, borrowed: no `AddRef`, no `Release`. `None` when the record carries a null
    /// pointer.
    #[cfg(windows)]
    pub fn texture(&self) -> Option<&ID3D11Texture2D> {
        // SAFETY: the record's texture is a live ID3D11Texture2D owned by the sender, which
        // the frame borrow keeps alive.
        unsafe { ID3D11Texture2D::from_raw_borrowed(&self.raw) }
    }
}

/// The keyed mutex of one shared texture and the keys of its acquire and release
/// (`QarDXGIKeyedMutexSync`), for an application that drives the chain itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11KeyedMutex<'frame> {
    /// The key to pass to `IDXGIKeyedMutex::AcquireSync` before rendering.
    pub acquire_key: u32,
    /// The key to pass to `IDXGIKeyedMutex::ReleaseSync` after rendering.
    pub release_key: u32,
    raw: *mut c_void,
    _frame: PhantomData<&'frame ()>,
}

impl D3D11KeyedMutex<'_> {
    /// The `IDXGIKeyedMutex` pointer the record carries. The sender owns it.
    pub fn as_raw(&self) -> *mut sys::IDXGIKeyedMutex {
        self.raw.cast()
    }

    /// The keyed mutex, borrowed: no `AddRef`, no `Release`. `None` when the record carries
    /// a null pointer.
    #[cfg(windows)]
    pub fn mutex(&self) -> Option<&IDXGIKeyedMutex> {
        // SAFETY: the record's mutex is a live IDXGIKeyedMutex owned by the sender, which the
        // frame borrow keeps alive.
        unsafe { IDXGIKeyedMutex::from_raw_borrowed(&self.raw) }
    }
}

/// A copy of the sender's D3D11 frame record (`qar_render_sender_frame_d3d11`): its views, its
/// shared textures and their keyed-mutex synchronization. From
/// [`crate::OwnedFrame::d3d11_frame`] or [`crate::Frame::d3d11_frame`] inside a frame cycle,
/// or [`crate::RenderSender::d3d11_frame_state`] outside one. The textures it lends live as
/// long as the borrow it was taken from.
#[derive(Debug)]
pub struct D3D11Frame<'borrow> {
    record: sys::QarVideoFrameD3D11,
    _borrow: PhantomData<&'borrow ()>,
}

impl D3D11Frame<'_> {
    pub(crate) fn from_record(record: &sys::QarVideoFrameD3D11) -> Self {
        Self {
            record: *record,
            _borrow: PhantomData,
        }
    }

    /// The record as the library wrote it, for C helpers that take it.
    pub fn as_raw(&self) -> &sys::QarVideoFrameD3D11 {
        &self.record
    }

    /// The frame's views: which texture, which rectangle, which eye, color or depth.
    pub fn views(&self) -> Vec<VideoFrameView> {
        let count = self
            .record
            .texture_views_count
            .min(self.record.texture_views.len());
        self.record.texture_views[..count]
            .iter()
            .map(VideoFrameView::from_raw)
            .collect()
    }

    /// Index of the first view showing `kind` for `eye`. The same index selects the view's
    /// pose and field of view on the frame ([`crate::OwnedFrame::view`]).
    pub fn view_index(&self, eye: Eye, kind: ViewKind) -> Option<usize> {
        self.views()
            .iter()
            .position(|view| view.eye == eye && view.kind == kind)
    }

    /// Number of shared textures in the frame.
    pub fn texture_count(&self) -> usize {
        self.record.textures_count.min(self.record.textures.len())
    }

    /// The texture a view's `texture_index` names.
    pub fn texture(&self, index: usize) -> Option<D3D11Texture<'_>> {
        self.record.textures[..self.texture_count()]
            .get(index)
            .map(texture_from_raw)
    }

    /// Every shared texture of the frame, in `texture_index` order.
    pub fn textures(&self) -> Vec<D3D11Texture<'_>> {
        self.record.textures[..self.texture_count()]
            .iter()
            .map(texture_from_raw)
            .collect()
    }

    /// One entry per texture when the application drives the keyed mutexes
    /// (`acquire_keyed_mutex_sync` false); empty when no external synchronization is needed.
    pub fn keyed_mutexes(&self) -> Vec<D3D11KeyedMutex<'_>> {
        let synchronization = &self.record.synchronization;
        let count = synchronization
            .texture_sync_count
            .min(synchronization.texture_sync.len());
        synchronization.texture_sync[..count]
            .iter()
            .map(|sync| D3D11KeyedMutex {
                acquire_key: sync.acquire,
                release_key: sync.release,
                raw: sync.mutex.cast(),
                _frame: PhantomData,
            })
            .collect()
    }
}

fn texture_from_raw<'frame>(raw: &sys::QarVideoTextureD3D11) -> D3D11Texture<'frame> {
    D3D11Texture {
        size: TextureSize::from_raw(&raw.size),
        raw: raw.texture.cast(),
        _frame: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_with_two_textures() -> sys::QarVideoFrameD3D11 {
        let mut record = defaults::qar_video_frame_d3d11_default();
        record.textures_count = 2;
        record.textures[0].texture = 0x1000 as *mut sys::ID3D11Texture2D;
        record.textures[0].size.width = 128;
        record.textures[1].texture = 0x2000 as *mut sys::ID3D11Texture2D;
        record.texture_views_count = 2;
        record.texture_views[0].eye = sys::QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_LEFT;
        record.texture_views[0].data_type =
            sys::QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH;
        record.texture_views[0].texture_index = 1;
        record.texture_views[1].eye = sys::QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_LEFT;
        record.texture_views[1].data_type =
            sys::QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_COLOR;
        record.synchronization.texture_sync_count = 2;
        record.synchronization.texture_sync[0].mutex = 0x3000 as *mut sys::IDXGIKeyedMutex;
        record.synchronization.texture_sync[0].acquire = 7;
        record.synchronization.texture_sync[0].release = 8;
        record
    }

    #[test]
    fn views_textures_and_keys_are_read_within_their_counts() {
        let frame = D3D11Frame::from_record(&record_with_two_textures());
        assert_eq!(frame.view_index(Eye::Left, ViewKind::Color), Some(1));
        assert_eq!(frame.view_index(Eye::Right, ViewKind::Color), None);
        let textures = frame.textures();
        assert_eq!(textures.len(), 2);
        assert_eq!(textures[0].as_raw() as usize, 0x1000);
        assert_eq!(textures[0].size.width, 128);
        assert_eq!(
            frame.texture(1).map(|texture| texture.as_raw() as usize),
            Some(0x2000)
        );
        assert_eq!(frame.texture(2), None);
        let keyed_mutexes = frame.keyed_mutexes();
        assert_eq!(keyed_mutexes.len(), 2);
        assert_eq!(keyed_mutexes[0].as_raw() as usize, 0x3000);
        assert_eq!(
            (keyed_mutexes[0].acquire_key, keyed_mutexes[0].release_key),
            (7, 8)
        );
    }

    #[test]
    fn counts_beyond_the_arrays_are_clamped() {
        let mut record = record_with_two_textures();
        record.textures_count = usize::MAX;
        record.texture_views_count = usize::MAX;
        record.synchronization.texture_sync_count = usize::MAX;
        let frame = D3D11Frame::from_record(&record);
        assert_eq!(frame.texture_count(), sys::QAR_MAX_FRAME_TEXTURES as usize);
        assert_eq!(frame.views().len(), sys::QAR_MAX_FRAME_VIEWS as usize);
        assert_eq!(
            frame.keyed_mutexes().len(),
            sys::QAR_MAX_FRAME_TEXTURES as usize
        );
    }

    #[test]
    fn stream_params_carry_the_pointers_flags_and_stamp() {
        // SAFETY: the pointers are only copied into the record, never used.
        let created = unsafe {
            D3D11StreamParams::from_raw(
                0x4000 as *mut sys::ID3D11Device1,
                0x5000 as *mut sys::ID3D11DeviceContext1,
            )
        };
        let Some(mut params) = created else {
            unreachable!("non-null pointers are accepted");
        };
        assert!(params.acquire_keyed_mutex_sync);
        assert_eq!((params.color_bind_flags, params.depth_bind_flags), (0, 0));
        params.color_bind_flags = 0x28;
        params.acquire_keyed_mutex_sync = false;
        let raw = params.to_raw();
        assert_eq!(
            raw.header.type_,
            sys::QarStructureType::QAR_STRUCTURE_TYPE_STREAM_D3D11_PARAMS_EXT
        );
        assert!(raw.header.next.is_null());
        assert_eq!(raw.d3d11_device as usize, 0x4000);
        assert_eq!(raw.d3d11_context as usize, 0x5000);
        assert_eq!(raw.color_bind_flags, 0x28);
        assert!(!raw.acquire_keyed_mutex_sync);
        // SAFETY: null pointers are refused before anything else.
        let refused = unsafe {
            D3D11StreamParams::from_raw(
                std::ptr::null_mut(),
                0x5000 as *mut sys::ID3D11DeviceContext1,
            )
        };
        assert!(refused.is_none());
    }
}
