use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::time::Duration;

use qaros_sys::{self as sys, QarApi, defaults};

use crate::callbacks::BlockingOptions;
use crate::error::{Error, Result, check};
use crate::library::require_handle;
use crate::pending::{self, AbandonedCall, CallSpec, PendingCall, RawOutcome};
use crate::session::SessionInner;
use crate::types::{
    AppVolumeId, Eye, Fov, FrameLayout, PeerId, PixelFormat, Pose, StreamId, ViewKind,
};

const FRAME_ALREADY_OPEN: &str =
    "a frame is already open on this render sender; show or drop it first";
const BEGIN_FRAME_ASYNC: &str = "qar_render_sender_begin_frame_async";

/// A begin-frame in flight from [`RenderSender::start_owned_frame`], delivering an
/// [`OwnedFrame`].
pub type PendingFrame = PendingCall<OwnedFrame>;

/// A render stream request from a peer that wants this application's
/// frames, read inside the callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderRequest {
    /// Pass as [`RenderSenderConfig::target_peer`].
    pub target_peer: PeerId,
    pub stream_id: StreamId,
}

pub(crate) fn read_render_request(
    api: &QarApi,
    request: *mut sys::QarRenderStreamRequest,
) -> Result<RenderRequest> {
    let mut target_peer = defaults::qar_peer_id_default();
    // SAFETY: the request is live for the callback; valid out-pointer.
    let result =
        unsafe { (api.qar_impl_render_request_get_target_peer_id)(request, &raw mut target_peer) };
    check(api, "qar_render_request_get_target_peer_id", result)?;
    let mut stream_id = defaults::qar_stream_id_default();
    // SAFETY: as above.
    let result =
        unsafe { (api.qar_impl_render_request_get_stream_id)(request, &raw mut stream_id) };
    check(api, "qar_render_request_get_stream_id", result)?;
    Ok(RenderRequest {
        target_peer: PeerId::from_raw(target_peer),
        stream_id: StreamId::from_raw(stream_id),
    })
}

/// One view inside the video frame, as declared at sender creation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameViewSpec {
    pub kind: ViewKind,
    pub eye: Eye,
    pub texture_index: u32,
}

/// What a render sender's colour alpha means (`QarColorAlphaMode`), declared
/// through `QarRenderSenderAlphaModeExt`. The stream publishes it and a
/// receiver composites by it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ColorAlphaMode {
    /// Not declared: a receiver keeps its own default. The sender chains no
    /// extension.
    #[default]
    Unspecified,
    /// The alpha channel means nothing; every pixel is fully covered. The
    /// declaration for a `B8G8R8X8` colour, if any.
    Opaque,
    /// The colour is premultiplied by the alpha, which is the pixel's
    /// coverage: what drawing over a transparent-black clear with ordinary
    /// alpha blending produces.
    Premultiplied,
    /// The colour is not multiplied by the alpha. The sender premultiplies it
    /// before the frame leaves the application, at one shader pass per frame,
    /// and the stream publishes [`Self::Premultiplied`].
    Straight,
}

/// What the numbers in a render sender's colour texture mean
/// (`QarColorEncoding`), declared through `QarRenderSenderColorEncodingExt`.
/// Whatever is declared, the stream carries sRGB-encoded colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ColorEncoding {
    /// sRGB-encoded, as a display shows it; nothing is converted. The sender
    /// chains no extension.
    #[default]
    Srgb,
    /// Linear light, as stored in a plain UNORM render target. The sender
    /// encodes it at one GPU pass per frame; 8-bit linear colour bands
    /// slightly in the darks, which [`PixelFormat::R16G16B16A16`] avoids.
    Linear,
}

impl ColorAlphaMode {
    fn to_raw(self) -> sys::QarColorAlphaMode {
        match self {
            Self::Unspecified => sys::QarColorAlphaMode::QAR_COLOR_ALPHA_MODE_UNSPECIFIED,
            Self::Opaque => sys::QarColorAlphaMode::QAR_COLOR_ALPHA_MODE_OPAQUE,
            Self::Premultiplied => sys::QarColorAlphaMode::QAR_COLOR_ALPHA_MODE_PREMULTIPLIED,
            Self::Straight => sys::QarColorAlphaMode::QAR_COLOR_ALPHA_MODE_STRAIGHT,
        }
    }
}

impl ColorEncoding {
    fn to_raw(self) -> sys::QarColorEncoding {
        match self {
            Self::Srgb => sys::QarColorEncoding::QAR_COLOR_ENCODING_SRGB,
            Self::Linear => sys::QarColorEncoding::QAR_COLOR_ENCODING_LINEAR,
        }
    }
}

/// Options for `qar_render_sender_create` on the CPU path.
#[derive(Clone, Debug)]
pub struct RenderSenderConfig {
    /// The requesting peer, from [`RenderRequest::target_peer`].
    pub target_peer: PeerId,
    pub auto_reconnect: bool,
    pub layout: FrameLayout,
    /// At most `QAR_MAX_FRAME_VIEWS` (8). The docs do not specify which view
    /// set a given layout expects; query [`RenderSender::layout`] after
    /// creation and write pixels where the frame's views say.
    pub views: Vec<FrameViewSpec>,
    /// Composites the stream inside this app volume.
    pub app_volume: Option<AppVolumeId>,
    pub color_format: PixelFormat,
    /// [`PixelFormat::D16Unorm`] on every graphics API;
    /// [`PixelFormat::D24UnormS8Uint`] and [`PixelFormat::D32FloatS8X24Uint`]
    /// on Vulkan only. A format the graphics API cannot take is refused with
    /// [`crate::StatusCode::RENDERING_DEPTH_FORMAT_NOT_SUPPORTED`].
    pub depth_format: PixelFormat,
    /// Chains `QarRenderSenderAlphaModeExt` unless
    /// [`ColorAlphaMode::Unspecified`].
    pub alpha_mode: ColorAlphaMode,
    /// Chains `QarRenderSenderColorEncodingExt` unless
    /// [`ColorEncoding::Srgb`].
    pub color_encoding: ColorEncoding,
}

impl RenderSenderConfig {
    /// The values of `qar_render_sender_init_default()`: auto-reconnect,
    /// side-by-side, no views, BGRA8 color, D32 depth; no colour extension,
    /// so the alpha mode is unspecified and the colour sRGB.
    pub fn new(target_peer: PeerId) -> Self {
        Self {
            target_peer,
            auto_reconnect: true,
            layout: FrameLayout::SideBySide,
            views: Vec::new(),
            app_volume: None,
            color_format: PixelFormat::B8G8R8A8,
            depth_format: PixelFormat::D32Float,
            alpha_mode: ColorAlphaMode::Unspecified,
            color_encoding: ColorEncoding::Srgb,
        }
    }
}

/// Per-view pose and field of view to render with, from `begin_frame`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewPose {
    pub pose: Pose,
    pub fov: Fov,
}

/// What `show_frame` reports about the frame just rendered. The default is
/// `qar_render_frame_show_default()`: near 0.1, far 100, depth scale 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShowParams {
    /// The near plane actually used.
    pub near: f32,
    /// The far plane actually used.
    pub far: f32,
    pub depth_scale: f32,
}

impl Default for ShowParams {
    fn default() -> Self {
        let raw = defaults::qar_render_frame_show_default();
        Self {
            near: raw.rendered_near_far.near_plane,
            far: raw.rendered_near_far.far_plane,
            depth_scale: raw.depth_scale,
        }
    }
}

/// A view rectangle inside one of the frame's textures. End coordinates are
/// exclusive and may be smaller than the start (flips).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoFrameView {
    pub start_x: u32,
    pub start_y: u32,
    pub end_x: u32,
    pub end_y: u32,
    pub texture_index: u32,
    pub array_layer_index: u32,
    pub format: PixelFormat,
    pub kind: ViewKind,
    pub eye: Eye,
}

impl VideoFrameView {
    pub(crate) fn from_raw(raw: &sys::QarVideoFrameView) -> Self {
        Self {
            start_x: raw.start_x,
            start_y: raw.start_y,
            end_x: raw.end_x,
            end_y: raw.end_y,
            texture_index: raw.texture_index,
            array_layer_index: raw.array_layer_index,
            format: PixelFormat::from_raw(raw.texture_format),
            kind: ViewKind::from_raw(raw.data_type),
            eye: Eye::from_raw(raw.eye),
        }
    }
}

/// Format and size of one texture of the frame layout (`QarTextureSize`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureSize {
    pub format: PixelFormat,
    pub width: u32,
    pub height: u32,
    pub array_layers: u32,
}

impl TextureSize {
    pub(crate) fn from_raw(raw: &sys::QarTextureSize) -> Self {
        Self {
            format: PixelFormat::from_raw(raw.format),
            width: raw.width,
            height: raw.height,
            array_layers: raw.array_layers,
        }
    }
}

/// The negotiated layout (`qar_render_sender_layout`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VideoFrameLayout {
    pub views: Vec<VideoFrameView>,
    pub textures: Vec<TextureSize>,
}

/// A CPU texture of the current frame, writable until the frame is shown.
#[derive(Debug)]
pub struct CpuTexture<'frame> {
    pub size: TextureSize,
    /// Bytes per row.
    pub pitch: u32,
    pub data: &'frame mut [u8],
}

/// The frame's CPU textures and the views mapping eyes and color/depth onto
/// them (`qar_render_sender_frame_cpu`).
#[derive(Debug)]
pub struct CpuFrame<'frame> {
    pub views: Vec<VideoFrameView>,
    pub textures: Vec<CpuTexture<'frame>>,
}

/// A CPU render stream sender. Keeps its session alive; destroyed with
/// `qar_render_stream_handle_destroy` after the last open frame.
#[derive(Debug)]
pub struct RenderSender {
    inner: Rc<RenderSenderInner>,
}

// Fields drop in declaration order after `drop` destroyed the handle: the session (destroyed
// here when this sender held its last reference) goes before the graphics references, so a
// D3D11 device outlives the session as well as the sender.
#[derive(Debug)]
struct RenderSenderInner {
    handle: NonNull<sys::QarRenderSender>,
    session: Rc<SessionInner>,
    /// A frame is open, or a begin-frame is in flight (pending or abandoned).
    frame_open: Cell<bool>,
    /// An async begin-frame whose [`PendingFrame`] was dropped before the callback; the frame
    /// stays "open" until it is settled.
    abandoned_frame: RefCell<Option<AbandonedCall>>,
    // Read only by the Vulkan and D3D11 frame access, to refuse a sender of another API.
    #[cfg_attr(not(any(feature = "vulkan", qaros_d3d11)), expect(dead_code))]
    graphics_api: sys::QarGraphicsAPI,
    _graphics: GraphicsReferences,
}

/// The graphics path a render sender is created for.
#[derive(Clone, Debug)]
pub(crate) enum SenderGraphics {
    Cpu,
    /// The caller vouches for the device handles (see
    /// `Session::create_vulkan_render_sender`).
    #[cfg(feature = "vulkan")]
    Vulkan(crate::vulkan::VulkanStreamParams),
    /// The parameters hold references on the device and context, or the caller vouches for
    /// them (see `Session::create_d3d11_render_sender`).
    #[cfg(qaros_d3d11)]
    D3D11(crate::d3d11::D3D11StreamParams),
}

impl SenderGraphics {
    /// What the sender has to keep alive for as long as the library may use it.
    pub(crate) fn references(&self) -> GraphicsReferences {
        match self {
            #[cfg(qaros_d3d11)]
            Self::D3D11(params) => GraphicsReferences {
                _d3d11: Some(params.references()),
            },
            _ => GraphicsReferences::default(),
        }
    }
}

/// Graphics objects the library may use while a sender lives: the D3D11 device and context
/// references of `D3D11StreamParams::new`. Empty for the CPU and Vulkan paths, whose handles
/// carry no reference count.
#[derive(Debug, Default)]
pub(crate) struct GraphicsReferences {
    #[cfg(qaros_d3d11)]
    _d3d11: Option<crate::d3d11::DeviceReferences>,
}

/// A `QarRenderSenderInit` together with the app volume id and the
/// extensions it points to, so that one allocation keeps all of it valid for
/// as long as the library may read it.
pub(crate) struct RenderSenderInitStorage {
    init: sys::QarRenderSenderInit,
    app_volume: Option<sys::QarAppVolumeId>,
    vulkan_params: Option<sys::QarStreamParamsVulkan>,
    #[cfg(qaros_d3d11)]
    d3d11_params: Option<sys::QarStreamParamsD3D11>,
    alpha_mode: Option<sys::QarRenderSenderAlphaModeExt>,
    color_encoding: Option<sys::QarRenderSenderColorEncodingExt>,
}

impl RenderSenderInitStorage {
    pub(crate) fn new(config: &RenderSenderConfig, graphics: &SenderGraphics) -> Result<Box<Self>> {
        let maximum_views = sys::QAR_MAX_FRAME_VIEWS as usize;
        if config.views.len() > maximum_views {
            return Err(Error::InvalidArgument(format!(
                "{} frame views requested, the C API holds at most {maximum_views}",
                config.views.len()
            )));
        }

        let mut init = defaults::qar_render_sender_init_default();
        init.peer_id = config.target_peer.to_raw();
        init.enable_auto_reconnects = config.auto_reconnect;
        init.texture_layout = config.layout.to_raw();
        for (slot, view) in init.frame_views.iter_mut().zip(&config.views) {
            slot.data_type = view.kind.to_raw();
            slot.eye = view.eye.to_raw();
            slot.texture_index = view.texture_index;
        }
        init.frame_views_count = config.views.len();
        init.color_format = config.color_format.to_raw();
        init.depth_format = config.depth_format.to_raw();
        let mut storage = Box::new(Self {
            init,
            app_volume: config.app_volume.map(AppVolumeId::to_raw),
            vulkan_params: None,
            #[cfg(qaros_d3d11)]
            d3d11_params: None,
            alpha_mode: alpha_mode_extension(config.alpha_mode),
            color_encoding: color_encoding_extension(config.color_encoding),
        });
        match graphics {
            SenderGraphics::Cpu => {
                storage.init.graphics_api = sys::QarGraphicsAPI::QAR_GRAPHICS_API_CPU;
            }
            #[cfg(feature = "vulkan")]
            SenderGraphics::Vulkan(params) => {
                storage.init.graphics_api = sys::QarGraphicsAPI::QAR_GRAPHICS_API_VULKAN;
                storage.vulkan_params = Some(params.to_raw());
            }
            #[cfg(qaros_d3d11)]
            SenderGraphics::D3D11(params) => {
                storage.init.graphics_api = sys::QarGraphicsAPI::QAR_GRAPHICS_API_D3D11;
                storage.d3d11_params = Some(params.to_raw());
            }
        }
        Ok(storage)
    }

    pub(crate) fn graphics_api(&self) -> sys::QarGraphicsAPI {
        self.init.graphics_api
    }

    /// Points the init at the app volume id and the extensions stored beside
    /// it, chained as alpha mode, colour encoding, graphics extension (the
    /// header allows any order). The returned pointer, and those inside it,
    /// stay valid while the storage is neither moved nor accessed in any other
    /// way.
    pub(crate) fn wired_init(&mut self) -> *mut sys::QarRenderSenderInit {
        self.init.app_volume_id = self.app_volume.as_ref().map_or(ptr::null(), ptr::from_ref);
        let vulkan_extension = self
            .vulkan_params
            .as_mut()
            .map(|params| ptr::from_mut(&mut params.header).cast::<c_void>());
        #[cfg(qaros_d3d11)]
        let d3d11_extension = self
            .d3d11_params
            .as_mut()
            .map(|params| ptr::from_mut(&mut params.header).cast::<c_void>());
        #[cfg(not(qaros_d3d11))]
        let d3d11_extension = None;
        let graphics_extension = vulkan_extension
            .or(d3d11_extension)
            .unwrap_or(ptr::null_mut());
        let graphics_extension = self.chain_color_extensions_before(graphics_extension);
        self.init.header.next = graphics_extension;
        &raw mut self.init
    }

    /// Chains the colour extensions that are set in front of `next` and
    /// returns the start of the chain.
    fn chain_color_extensions_before(&mut self, mut next: *mut c_void) -> *mut c_void {
        if let Some(extension) = self.color_encoding.as_mut() {
            extension.header.next = next;
            next = ptr::from_mut(&mut extension.header).cast::<c_void>();
        }
        if let Some(extension) = self.alpha_mode.as_mut() {
            extension.header.next = next;
            next = ptr::from_mut(&mut extension.header).cast::<c_void>();
        }
        next
    }
}

fn alpha_mode_extension(alpha_mode: ColorAlphaMode) -> Option<sys::QarRenderSenderAlphaModeExt> {
    (alpha_mode != ColorAlphaMode::Unspecified).then(|| {
        let mut extension = defaults::qar_render_sender_alpha_mode_ext_default();
        extension.alpha_mode = alpha_mode.to_raw();
        extension
    })
}

fn color_encoding_extension(
    color_encoding: ColorEncoding,
) -> Option<sys::QarRenderSenderColorEncodingExt> {
    (color_encoding != ColorEncoding::Srgb).then(|| {
        let mut extension = defaults::qar_render_sender_color_encoding_ext_default();
        extension.color_encoding = color_encoding.to_raw();
        extension
    })
}

impl RenderSender {
    pub(crate) fn create(
        session: &Rc<SessionInner>,
        config: &RenderSenderConfig,
        graphics: &SenderGraphics,
        options: &BlockingOptions<'_>,
    ) -> Result<Self> {
        let mut storage = RenderSenderInitStorage::new(config, graphics)?;
        let graphics_api = storage.graphics_api();
        let init = storage.wired_init();
        let api = session.api();
        let mut handle: *mut sys::QarRenderSender = ptr::null_mut();
        // SAFETY: live session; the storage behind init lives until the
        // blocking call returns; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_sender_create)(
                session.handle.as_ptr(),
                init,
                options.cancel_pointer(),
                &raw mut handle,
            )
        };
        check(api, "qar_render_sender_create", result)?;
        let handle = require_handle(handle, "qar_render_sender_create")?;
        Ok(Self::from_created_handle(
            session,
            handle,
            graphics_api,
            graphics.references(),
        ))
    }

    /// Takes ownership of a sender the library created on `session`, with the graphics
    /// references it keeps until it is destroyed.
    pub(crate) fn from_created_handle(
        session: &Rc<SessionInner>,
        handle: NonNull<sys::QarRenderSender>,
        graphics_api: sys::QarGraphicsAPI,
        graphics: GraphicsReferences,
    ) -> Self {
        Self {
            inner: Rc::new(RenderSenderInner {
                handle,
                session: Rc::clone(session),
                frame_open: Cell::new(false),
                abandoned_frame: RefCell::new(None),
                graphics_api,
                _graphics: graphics,
            }),
        }
    }

    /// The sender's D3D11 frame record outside a frame cycle (`qar_render_sender_frame_d3d11`),
    /// readable from creation: create views on the shared textures here, before the first
    /// frame, but render into them only inside a cycle. Fails while a frame is open on this
    /// sender; inside a cycle use [`Frame::d3d11_frame`] or [`OwnedFrame::d3d11_frame`]. Fails
    /// for a sender not created for D3D11.
    #[cfg(qaros_d3d11)]
    pub fn d3d11_frame_state(&self) -> Result<crate::d3d11::D3D11Frame<'_>> {
        self.inner.refuse_if_frame_open(
            "a frame is open on this render sender; read its D3D11 frame through the frame",
        )?;
        let record = self.inner.fetch_d3d11_frame()?;
        Ok(crate::d3d11::D3D11Frame::from_record(&record))
    }

    /// The sender-owned Vulkan frame outside a frame cycle
    /// (`qar_render_sender_frame_vulkan`), readable from creation: create
    /// image views and framebuffers against the real images here, before the
    /// first frame. Fails while a frame is open on this sender;
    /// inside a cycle use [`Frame::vulkan_frame`] or
    /// [`OwnedFrame::vulkan_frame`], which also allow the write-back.
    #[cfg(feature = "vulkan")]
    pub fn vulkan_frame_state(&mut self) -> Result<crate::vulkan::VulkanFrameState<'_>> {
        self.inner.refuse_if_frame_open(
            "a frame is open on this render sender; read its Vulkan frame through the frame",
        )?;
        self.inner.fetch_vulkan_frame()
    }

    /// The `QarRenderSender` handle, for the functions only the raw table of
    /// [`crate::Library::raw_api`] reaches, such as `qar_render_sender_last_hands`,
    /// `qar_render_sender_stream_id` and `qar_render_sender_change_layout`.
    ///
    /// Taking the pointer is safe; using it is `unsafe`, under the rules of
    /// [`crate::Session::as_raw`] for a sender: valid while this `RenderSender` lives, never
    /// destroyed by the caller, used on the thread that drives this sender. The frame cycle stays
    /// with the safe API: never begin or show a frame through the raw table.
    pub fn as_raw(&self) -> *mut sys::QarRenderSender {
        self.inner.handle.as_ptr()
    }

    /// The layout negotiated with the receiver (`qar_render_sender_layout`).
    pub fn layout(&self) -> Result<VideoFrameLayout> {
        let api = self.inner.session.api();
        let mut layout = defaults::qar_video_frame_layout_default();
        // SAFETY: live sender; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_sender_layout)(self.inner.handle.as_ptr(), &raw mut layout)
        };
        check(api, "qar_render_sender_layout", result)?;
        let view_count = layout.views_count.min(layout.views.len());
        let texture_count = layout.textures_count.min(layout.textures.len());
        Ok(VideoFrameLayout {
            views: layout.views[..view_count]
                .iter()
                .map(VideoFrameView::from_raw)
                .collect(),
            textures: layout.textures[..texture_count]
                .iter()
                .map(TextureSize::from_raw)
                .collect(),
        })
    }

    /// Begins a frame (`qar_render_sender_begin_frame`). Render with the
    /// returned views' poses and fields of view, then [`Frame::show`].
    pub fn begin_frame(&mut self, options: &BlockingOptions<'_>) -> Result<Frame<'_>> {
        Ok(Frame {
            owned: self.begin_owned_frame(options)?,
            _sender: PhantomData,
        })
    }

    /// Like [`RenderSender::begin_frame`], for a frame that stays open while
    /// the caller does other work, for example across several UI frames
    /// until a GPU readback arrives. The frame keeps the sender alive. One
    /// frame is open per sender at a time; a second call before the first
    /// frame is shown or dropped fails without calling the library.
    pub fn begin_owned_frame(&mut self, options: &BlockingOptions<'_>) -> Result<OwnedFrame> {
        self.inner.refuse_if_frame_open(FRAME_ALREADY_OPEN)?;
        let api = self.inner.session.api();
        let mut info: *mut sys::QarRenderFrameInfo = ptr::null_mut();
        // SAFETY: live sender; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_sender_begin_frame)(
                self.inner.handle.as_ptr(),
                options.cancel_pointer(),
                &raw mut info,
            )
        };
        check(api, "qar_render_sender_begin_frame", result)?;
        let info = require_handle(info, "qar_render_sender_begin_frame")?;
        self.inner.frame_open.set(true);
        Ok(OwnedFrame {
            info,
            sender: Rc::clone(&self.inner),
        })
    }

    /// Starts a frame without blocking (`qar_render_sender_begin_frame_async`) and returns at
    /// once; [`PendingCall::poll`] or [`PendingCall::wait`] picks up the [`OwnedFrame`], which
    /// behaves like one from [`RenderSender::begin_owned_frame`] (CPU, Vulkan and D3D11 access
    /// alike). Same poses, failures and wait through a paused stream as the blocking begin.
    ///
    /// The frame counts as open from the start: another begin on this sender fails without a
    /// library call until the pending frame is taken and shown or dropped. `timeout` cancels
    /// the call through its token, after which the library calls back with
    /// [`crate::StatusCode::TIMEOUT`]. `wake` runs on a library thread right after the result
    /// arrived.
    ///
    /// Dropping the pending frame cancels it and returns at once. The next begin on this sender
    /// first waits for its callback and destroys a frame info it delivered; destroying the sender
    /// cancels it and waits for its callback. The
    /// frame info the callback hands over is owned by the caller, as the runtime's tests use it;
    /// the header does not say so.
    pub fn start_owned_frame(
        &mut self,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingFrame> {
        self.inner.refuse_if_frame_open(FRAME_ALREADY_OPEN)?;
        let finish_sender = Rc::clone(&self.inner);
        let abandon_sender = Rc::clone(&self.inner);
        let poll_sender = Rc::clone(&self.inner);
        let spec = CallSpec {
            function: BEGIN_FRAME_ASYNC,
            progress: None,
            timeout,
            wake: Box::new(wake),
            keep: None,
            finish: Box::new(move |outcome: RawOutcome| {
                let api = finish_sender.session.api();
                let delivered = check(api, BEGIN_FRAME_ASYNC, outcome.result).and_then(|()| {
                    require_handle(
                        outcome.handle as *mut sys::QarRenderFrameInfo,
                        BEGIN_FRAME_ASYNC,
                    )
                });
                match delivered {
                    Ok(info) => Ok(OwnedFrame {
                        info,
                        sender: finish_sender,
                    }),
                    Err(error) => {
                        release_frame_info(api, outcome.handle);
                        finish_sender.frame_open.set(false);
                        Err(error)
                    }
                }
            }),
            abandon: Box::new(move |call| {
                *abandon_sender.abandoned_frame.borrow_mut() = Some(call);
            }),
            before_poll: Box::new(move || {
                poll_sender.session.release_delivered_abandoned_calls();
            }),
            destroy: Some(release_frame_info),
        };
        let sender = self.inner.handle.as_ptr();
        let api = self.inner.session.api();
        let pending = pending::start(
            self.inner.session.library(),
            spec,
            |user_state, _progress, token| {
                // SAFETY: live sender; the token outlives the call (the pending frame or the
                // sender keeps it).
                unsafe {
                    (api.qar_impl_render_sender_begin_frame_async)(
                        sender,
                        Some(pending::handle_result_trampoline::<sys::QarRenderFrameInfo>),
                        user_state,
                        token,
                    )
                }
            },
        )?;
        self.inner.frame_open.set(true);
        Ok(pending)
    }

    /// Polls for a frame without blocking (`qar_render_sender_try_begin_frame`,
    /// the call QuaternAR's Vulkan cube uses). The first call starts the
    /// library's begin-frame wait; `Ok(None)` means not ready yet, poll again
    /// later on the same sender. A frame it returns behaves like one from
    /// [`RenderSender::begin_owned_frame`]. Fails without calling the library
    /// while a frame is open on this sender.
    pub fn try_begin_owned_frame(&mut self) -> Result<Option<OwnedFrame>> {
        self.inner.refuse_if_frame_open(FRAME_ALREADY_OPEN)?;
        let api = self.inner.session.api();
        let mut info: *mut sys::QarRenderFrameInfo = ptr::null_mut();
        let mut ready = false;
        // SAFETY: live sender; valid out-pointers.
        let result = unsafe {
            (api.qar_impl_render_sender_try_begin_frame)(
                self.inner.handle.as_ptr(),
                &raw mut info,
                &raw mut ready,
            )
        };
        check(api, "qar_render_sender_try_begin_frame", result)?;
        if !ready {
            return Ok(None);
        }
        let info = require_handle(info, "qar_render_sender_try_begin_frame")?;
        self.inner.frame_open.set(true);
        Ok(Some(OwnedFrame {
            info,
            sender: Rc::clone(&self.inner),
        }))
    }
}

impl RenderSenderInner {
    fn refuse_if_frame_open(&self, message: &str) -> Result<()> {
        self.settle_abandoned_frame();
        if self.frame_open.get() {
            return Err(Error::InvalidArgument(message.to_string()));
        }
        Ok(())
    }

    /// Ends a dropped async begin-frame: waits for its callback, which the cancel brings
    /// promptly and the header guarantees, then destroys a frame info it delivered and closes the
    /// frame. A cancel arriving as the begin finishes may still deliver a frame, so a new begin
    /// must not start before this.
    fn settle_abandoned_frame(&self) {
        if let Some(call) = self.abandoned_frame.borrow_mut().take() {
            call.settle(self.session.api());
            self.frame_open.set(false);
        }
    }

    #[cfg(feature = "vulkan")]
    fn fetch_vulkan_frame<'borrow>(&self) -> Result<crate::vulkan::VulkanFrameState<'borrow>> {
        if self.graphics_api != sys::QarGraphicsAPI::QAR_GRAPHICS_API_VULKAN {
            return Err(Error::InvalidArgument(
                "this render sender was not created for Vulkan".to_string(),
            ));
        }
        let api = self.session.api();
        let mut frame: *mut sys::QarVideoFrameVulkan = ptr::null_mut();
        // SAFETY: live sender; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_sender_frame_vulkan)(self.handle.as_ptr(), &raw mut frame)
        };
        check(api, "qar_render_sender_frame_vulkan", result)?;
        let frame = require_handle(frame, "qar_render_sender_frame_vulkan")?;
        // SAFETY: the library keeps the frame valid until the next
        // frame_vulkan call or sender destruction; callers tie 'borrow to a
        // mutable borrow that rules out both (see the call sites).
        Ok(unsafe { crate::vulkan::VulkanFrameState::from_raw(frame) })
    }

    /// A copy of the D3D11 frame record. The pointers in it name the sender's own textures
    /// and keyed mutexes; callers lend them for a borrow of the sender or the open frame.
    #[cfg(qaros_d3d11)]
    fn fetch_d3d11_frame(&self) -> Result<sys::QarVideoFrameD3D11> {
        if self.graphics_api != sys::QarGraphicsAPI::QAR_GRAPHICS_API_D3D11 {
            return Err(Error::InvalidArgument(
                "this render sender was not created for D3D11".to_string(),
            ));
        }
        let api = self.session.api();
        let mut frame = defaults::qar_video_frame_d3d11_default();
        // SAFETY: live sender; valid out-pointer to a record the library fills.
        let result = unsafe {
            (api.qar_impl_render_sender_frame_d3d11)(self.handle.as_ptr(), &raw mut frame)
        };
        check(api, "qar_render_sender_frame_d3d11", result)?;
        Ok(frame)
    }
}

impl Drop for RenderSenderInner {
    fn drop(&mut self) {
        let api = self.session.api();
        let abandoned = self.abandoned_frame.get_mut().take();
        if let Some(call) = &abandoned {
            call.release_if_delivered(api);
        }
        // SAFETY: open frames and pending begin-frames hold an Rc to this value, so none is
        // alive. Per the header, the destroy cancels an abandoned begin-frame still running and
        // returns only after its callback has returned.
        unsafe { (api.qar_impl_render_stream_handle_destroy)(self.handle.as_ptr()) };
        if let Some(call) = abandoned {
            call.leak_late_delivery();
        }
    }
}

/// Destroys a frame info an async begin-frame delivered that nobody takes.
fn release_frame_info(api: &QarApi, info: usize) {
    if info != 0 {
        // SAFETY: delivered to us, used by nobody, and its sender is alive.
        unsafe {
            (api.qar_impl_render_frame_info_handle_destroy)(info as *mut sys::QarRenderFrameInfo);
        };
    }
}

/// A frame between `begin_frame` and `show_frame`, borrowing its sender.
/// Dropping it without [`Frame::show`] releases the frame info without
/// presenting.
#[derive(Debug)]
pub struct Frame<'sender> {
    owned: OwnedFrame,
    _sender: PhantomData<&'sender mut RenderSender>,
}

impl Frame<'_> {
    /// Pose and field of view for view `view_index`. The C API offers no
    /// view count; use the number of views the sender was created with.
    pub fn view(&self, view_index: usize) -> Result<ViewPose> {
        self.owned.view(view_index)
    }

    /// The CPU textures to write this frame's pixels into
    /// (`qar_render_sender_frame_cpu`).
    pub fn cpu_frame(&mut self) -> Result<CpuFrame<'_>> {
        self.owned.cpu_frame()
    }

    /// The sender-owned Vulkan images of this frame, with the write-back of
    /// layouts, queue family and fence; see [`OwnedFrame::vulkan_frame`].
    #[cfg(feature = "vulkan")]
    pub fn vulkan_frame(&mut self) -> Result<crate::vulkan::VulkanFrame<'_>> {
        self.owned.vulkan_frame()
    }

    /// The shared D3D11 textures of this frame and the views on them; see
    /// [`OwnedFrame::d3d11_frame`].
    #[cfg(qaros_d3d11)]
    pub fn d3d11_frame(&self) -> Result<crate::d3d11::D3D11Frame<'_>> {
        self.owned.d3d11_frame()
    }

    /// Presents the frame (`qar_render_sender_show_frame`) with the near and
    /// far planes actually used.
    pub fn show(self, params: &ShowParams) -> Result<()> {
        self.owned.show(params)
    }
}

/// A frame between `begin_frame` and `show_frame` that owns a reference to
/// its sender, from [`RenderSender::begin_owned_frame`]. Dropping it without
/// [`OwnedFrame::show`] releases the frame info without presenting.
#[derive(Debug)]
pub struct OwnedFrame {
    info: NonNull<sys::QarRenderFrameInfo>,
    sender: Rc<RenderSenderInner>,
}

impl OwnedFrame {
    /// Pose and field of view for view `view_index`. The C API offers no
    /// view count; use the number of views the sender was created with.
    pub fn view(&self, view_index: usize) -> Result<ViewPose> {
        let api = self.sender.session.api();
        let mut pose = defaults::qar_pose_default();
        // SAFETY: live frame info; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_frame_info_get_view_pose)(
                self.info.as_ptr(),
                view_index,
                &raw mut pose,
            )
        };
        check(api, "qar_render_frame_info_get_view_pose", result)?;
        let mut fov = defaults::qar_fov_default();
        // SAFETY: as above.
        let result = unsafe {
            (api.qar_impl_render_frame_info_get_view_fov)(
                self.info.as_ptr(),
                view_index,
                &raw mut fov,
            )
        };
        check(api, "qar_render_frame_info_get_view_fov", result)?;
        Ok(ViewPose {
            pose: Pose::from_raw(pose),
            fov: Fov::from_raw(fov),
        })
    }

    /// The CPU textures to write this frame's pixels into
    /// (`qar_render_sender_frame_cpu`).
    pub fn cpu_frame(&mut self) -> Result<CpuFrame<'_>> {
        let api = self.sender.session.api();
        let mut frame = defaults::qar_video_frame_cpu_default();
        // SAFETY: live sender; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_render_sender_frame_cpu)(self.sender.handle.as_ptr(), &raw mut frame)
        };
        check(api, "qar_render_sender_frame_cpu", result)?;

        let view_count = frame.texture_views_count.min(frame.texture_views.len());
        let texture_count = frame.textures_count.min(frame.textures.len());
        let views = frame.texture_views[..view_count]
            .iter()
            .map(VideoFrameView::from_raw)
            .collect();
        let raw_textures = &frame.textures[..texture_count];
        reject_overlapping_textures(raw_textures)?;
        let textures = raw_textures
            .iter()
            .map(|texture| CpuTexture {
                size: TextureSize::from_raw(&texture.size),
                pitch: texture.pitch,
                // SAFETY: the library hands out texture_data_size writable
                // bytes that stay valid until show_frame; the returned slice
                // borrows this frame mutably, so it cannot outlive show, and
                // only one frame is open per sender.
                data: unsafe { texture_bytes(texture.texture_data, texture.texture_data_size) },
            })
            .collect();
        Ok(CpuFrame { views, textures })
    }

    /// The sender-owned Vulkan images of this frame
    /// (`qar_render_sender_frame_vulkan`): views, image records and the
    /// semaphore to signal, plus the write-back that must happen before
    /// [`OwnedFrame::show`]. The borrow keeps the library's frame pointer
    /// valid: no other `frame_vulkan` call can happen on this sender while it
    /// lives. Image records are snapshots; re-read them every frame.
    #[cfg(feature = "vulkan")]
    pub fn vulkan_frame(&mut self) -> Result<crate::vulkan::VulkanFrame<'_>> {
        self.sender
            .fetch_vulkan_frame()
            .map(crate::vulkan::VulkanFrame::new)
    }

    /// The shared D3D11 textures of this frame, the views laid out on them and, when the
    /// sender was created with `acquire_keyed_mutex_sync` false, their keyed mutexes and keys
    /// (`qar_render_sender_frame_d3d11`). Render into each view's rectangle before
    /// [`OwnedFrame::show`]; the borrow ends before `show` can consume the frame, so the
    /// textures it lends cannot be used past the cycle. Read it every frame. Fails for a
    /// sender not created for D3D11.
    #[cfg(qaros_d3d11)]
    pub fn d3d11_frame(&self) -> Result<crate::d3d11::D3D11Frame<'_>> {
        let record = self.sender.fetch_d3d11_frame()?;
        Ok(crate::d3d11::D3D11Frame::from_record(&record))
    }

    /// Presents the frame (`qar_render_sender_show_frame`) with the near and
    /// far planes actually used.
    pub fn show(self, params: &ShowParams) -> Result<()> {
        let api = self.sender.session.api();
        let mut show = defaults::qar_render_frame_show_default();
        show.rendered_near_far.near_plane = params.near;
        show.rendered_near_far.far_plane = params.far;
        show.depth_scale = params.depth_scale;
        // SAFETY: live sender; show is stamped and lives for the call.
        let result = unsafe {
            (api.qar_impl_render_sender_show_frame)(self.sender.handle.as_ptr(), &raw const show)
        };
        check(api, "qar_render_sender_show_frame", result)
    }
}

impl Drop for OwnedFrame {
    fn drop(&mut self) {
        let destroy_frame_info = self
            .sender
            .session
            .api()
            .qar_impl_render_frame_info_handle_destroy;
        // SAFETY: the info handle is owned by this frame and destroyed once,
        // after show_frame when the frame was shown (the C sample's order).
        unsafe { destroy_frame_info(self.info.as_ptr()) };
        self.sender.frame_open.set(false);
    }
}

/// Two `&mut` slices over the same bytes would be undefined behaviour, so
/// texture buffers reported by the library must be disjoint.
fn reject_overlapping_textures(textures: &[sys::QarVideoTextureCpu]) -> Result<()> {
    let mut ranges: Vec<(usize, usize)> = textures
        .iter()
        .filter(|texture| !texture.texture_data.is_null() && texture.texture_data_size > 0)
        .map(|texture| {
            let start = texture.texture_data as usize;
            (start, start.saturating_add(texture.texture_data_size))
        })
        .collect();
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[1].0 < pair[0].1) {
        return Err(Error::Api {
            function: "qar_render_sender_frame_cpu",
            code: crate::error::StatusCode::UNCLASSIFIED,
            message: "the frame's CPU textures share memory; this wrapper needs disjoint buffers"
                .to_string(),
            causes: Vec::new(),
        });
    }
    Ok(())
}

/// # Safety
///
/// `data` must be null or point to `size` writable bytes valid for `'frame`.
unsafe fn texture_bytes<'frame>(data: *mut u8, size: usize) -> &'frame mut [u8] {
    if data.is_null() || size == 0 {
        return &mut [];
    }
    // SAFETY: forwarded to the caller.
    unsafe { std::slice::from_raw_parts_mut(data, size) }
}
