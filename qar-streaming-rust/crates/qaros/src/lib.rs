//! Safe Rust API over the QAROS streaming C API.
//!
//! Covers what a streaming application needs: loading and initializing the
//! library, the loaded runtime's version, runtimes, onboarding and rejoin
//! (pairing code, remote hub, sibling invites, local app approved in the hub,
//! adapter choice), peers, the CPU render sender, the
//! Vulkan render sender (feature `vulkan`, module `vulkan`), the D3D11 render
//! sender (feature `d3d11`, module `d3d11`: Windows; on macOS only for the
//! tests against `qaros-stub`), app volumes and GUI panels. Hands, gestures and world anchors
//! stay in [`qaros_sys`]; reach them, and every other raw call, through
//! [`Library::raw_api`].
//!
//! # Design
//!
//! - Every fallible call returns [`Result`]; C errors carry the
//!   [`StatusCode`] and the message from `qar_result_message`.
//! - Init structures are built from the ported `qar_*_default()`
//!   initializers, so every `QarStructureHeader` is stamped.
//! - Handles release themselves on drop, and each keeps its parent alive, so
//!   the C teardown order is always render sender, session, runtime,
//!   library, whatever order Rust drops them in.
//! - None of the handle types is `Send`: the SDK is designed for
//!   single-threaded use per runtime, and callbacks are its only
//!   cross-thread surface.
//! - Subscription callbacks run on library threads. They receive owned
//!   snapshots read from the borrowed C handle during the callback; use
//!   [`channel`] to forward them into a `std::sync::mpsc` receiver. Their
//!   state lives until the session is destroyed; a [`Subscription`] guard
//!   only cancels.
//!
//! ```no_run
//! use qaros::{BlockingOptions, Library, LibraryConfig, OnboardMethod, PeerPresentation, RuntimeConfig};
//!
//! # fn main() -> qaros::Result<()> {
//! // SAFETY: the runtime found (the installed QAROS, or that of the instance QAR_INSTANCE
//! // names) matches these bindings.
//! let library = unsafe { Library::load_default(&LibraryConfig::default())? };
//! let runtime = library.create_runtime(&RuntimeConfig::default())?;
//! let onboarded = runtime.onboard(
//!     &OnboardMethod::Code { code: "ABCD-EFGH", hub: None },
//!     &PeerPresentation::default(),
//!     &BlockingOptions::default(),
//! )?;
//! let onboarding_text = library.uuid_to_string(&onboarded.onboarding_id.0)?;
//! # let _ = onboarding_text;
//! # Ok(()) }
//! ```

mod adapter;
mod callbacks;
mod cancel;
#[cfg(qaros_d3d11)]
pub mod d3d11;
mod error;
mod invite;
mod library;
mod panels;
mod peers;
mod pending_sender;
mod render;
mod runtime;
mod session;
mod strings;
mod types;
mod volumes;
#[cfg(feature = "vulkan")]
pub mod vulkan;

pub use adapter::GraphicsAdapter;
pub use callbacks::{BlockingOptions, Progress, channel};
pub use cancel::{CancelHandle, CancelToken};
pub use error::{Error, Result, StatusCode};
pub use invite::OnboardingInvite;
pub use library::{Library, LibraryConfig, RuntimeInfo};
pub use panels::{GuiPanelConfig, GuiPanelInfo, GuiPanelSize, GuiPanels};
pub use peers::PeerInfo;
pub use pending_sender::{CANCELLED_CREATION_WAIT, PendingRenderSender};
pub use render::{
    ColorAlphaMode, ColorEncoding, CpuFrame, CpuTexture, Frame, FrameViewSpec, OwnedFrame,
    RenderRequest, RenderSender, RenderSenderConfig, ShowParams, TextureSize, VideoFrameLayout,
    VideoFrameView, ViewPose,
};
pub use runtime::{HubAddress, OnboardMethod, Onboarded, PeerPresentation, Runtime, RuntimeConfig};
pub use session::{Session, Subscription};
pub use types::{
    ActionSeverity, AppState, AppVolumeId, AppVolumeLifetime, Eye, Fov, FrameLayout, GuiPanelId,
    GuiPanelState, LogSeverity, OnboardingId, OnboardingMethod, PeerId, PixelFormat, Pose, Quat,
    SessionChangeCause, SessionId, SessionLifecycle, SessionLifecycleState, StreamId, Vec3,
    ViewKind,
};
pub use volumes::{AppVolumeConfig, AppVolumeInfo, AppVolumeSize, AppVolumes};

pub use qaros_sys;
pub use qaros_sys::{installed_bin_folder, instance_runtime_folder, requested_instance};

/// The binding's README as doc tests, so its "Use" example keeps compiling against the API.
#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
struct ReadmeDocTests;
