//! The C ABI of `qar-streaming-c-ext`, mirrored by hand like the C# tests' `ExtNativeMethods.cs`:
//! the extension's own structures, enumerations and opaque handles, and its exported functions.
//! Base types (results, ids, poses, layouts, frames) are the `qaros-sys` ones.
//!
//! The layouts must match the extension library's build; the compile-time checks at the end pin
//! the sizes and offsets of every structure for 64-bit targets.

#![allow(clippy::wildcard_imports)]

use std::ffi::{c_char, c_int};

use qaros_sys::*;

/// Capacity of the extension's fixed text fields, NUL included.
pub const EXT_MAX_TEXT_LENGTH: usize = 256;

/// The extension's own range of `QarStructureType` values.
pub mod structure_types {
    use qaros_sys::QarStructureType;

    pub const HUB_INIT: QarStructureType = QarStructureType(0xE001);
    pub const PEER_INIT: QarStructureType = QarStructureType(0xE002);
    pub const RENDER_RECEIVER_INIT: QarStructureType = QarStructureType(0xE003);
}

macro_rules! opaque_handles {
    ($($name:ident),* $(,)?) => {$(
        /// An opaque handle of the extension, only ever used behind a pointer.
        #[repr(C)]
        #[derive(Debug)]
        pub struct $name {
            _private: [u8; 0],
        }
    )*};
}

opaque_handles!(
    ExtHub,
    ExtPeer,
    ExtRenderReceiver,
    ExtVideoReceiver,
    ExtTargetAppResponder,
    ExtVulkanDevice,
    ExtVulkanFrame,
);

macro_rules! c_enumeration {
    ($(#[$attribute:meta])* $name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        $(#[$attribute])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub struct $name(pub c_int);

        impl $name {
            $(pub const $variant: Self = Self($value);)*
        }
    };
}

c_enumeration!(
    /// How a hub invite onboards: with a PAKE code or with a password.
    ExtInviteMethod { PAKE = 0, PASSWORD = 1 }
);

c_enumeration!(
    /// Which stream of a sender a video receiver takes.
    ExtVideoReceiverRole { AUTO = 0, PRIMARY = 1, SECONDARY = 2 }
);

c_enumeration!(
    /// What a registered media stream carries.
    ExtMediaStreamKind { VIDEO = 0, AUDIO = 1, AUDIO_VIDEO = 2 }
);

c_enumeration!(
    /// Where a registered media stream is in its life.
    ExtMediaStreamLifetime { ACTIVE = 0, CLOSED = 1, REMOVED = 2 }
);

/// Creates an in-process hub.
#[repr(C)]
#[derive(Debug)]
pub struct ExtHubInit {
    pub header: QarStructureHeader,
    pub enable_password_onboarding: bool,
}

/// Creates an in-process peer on a hub.
#[repr(C)]
#[derive(Debug)]
pub struct ExtPeerInit {
    pub header: QarStructureHeader,
    pub display_name: *const c_char,
}

/// Creates a video receiver on a peer.
#[repr(C)]
pub struct ExtVideoReceiverInit {
    pub header: QarStructureHeader,
    pub optimal_layout: QarVideoFrameLayout,
    pub has_stream_id: bool,
    pub stream_id: QarStreamId,
    pub role: ExtVideoReceiverRole,
    pub start_paused: bool,
}

/// Creates a render receiver (a target that requests rendered frames) on a peer.
#[repr(C)]
pub struct ExtRenderReceiverInit {
    pub header: QarStructureHeader,
    pub optimal_layout: QarVideoFrameLayout,
}

/// One render request a render receiver sends.
#[repr(C)]
#[derive(Debug)]
pub struct ExtRenderRequest {
    pub head_pose: QarPose,
    pub frame_count: u64,
}

/// What a video receiver got with a frame.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtVideoFrameInfo {
    pub valid_frame: bool,
    pub sequence_number: u64,
    pub creation_time_ns: i64,
}

/// Creates a responder that answers the hub's target-app invites like a headset.
#[repr(C)]
#[derive(Debug)]
pub struct ExtTargetAppResponderInit {
    pub header: QarStructureHeader,
    pub machine_id: *const c_char,
}

/// One target-app invite a responder received.
#[repr(C)]
pub struct ExtTargetAppInviteRecord {
    pub producer: QarPeerId,
    pub hostname: [c_char; EXT_MAX_TEXT_LENGTH],
    pub device: [c_char; EXT_MAX_TEXT_LENGTH],
    pub answered: bool,
    pub target: QarPeerId,
}

/// One target-app shutdown request a responder received.
#[repr(C)]
pub struct ExtTargetAppShutdownRecord {
    pub peer_to_shutdown: QarPeerId,
    pub answered: bool,
}

/// One media stream in the hub's or a peer's register.
#[repr(C)]
pub struct ExtMediaStreamRecord {
    pub id: QarStreamId,
    pub kind: ExtMediaStreamKind,
    pub lifetime: ExtMediaStreamLifetime,
    pub has_video: bool,
    pub sender_peer: QarPeerId,
    pub receiver_count: usize,
    pub has_source_layout: bool,
    pub source_layout: QarVideoFrameLayout,
}

/// A GUI panel gesture a peer sends, in panel pixels.
#[repr(C)]
pub struct ExtGuiPanelGesture {
    pub panel: QarGuiPanelId,
    pub kind: QarGuiPanelGestureKind,
    pub phase: QarGesturePhase,
    pub gesture_id: u64,
    pub handedness: QarHandedness,
    pub x: i32,
    pub y: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub tap_count: u8,
}

/// Invokes `$callback!` with every exported `qar_impl_ext_*` function.
macro_rules! ext_function_list {
    ($callback:ident) => {
        $callback! {
            pub fn qar_impl_ext_hub_create(init: *const ExtHubInit, out_hub: *mut *mut ExtHub) -> QarResult;
            pub fn qar_impl_ext_hub_find_peer_display_name(
                hub: *mut ExtHub,
                peer_id: *const QarPeerId,
                out_buffer: *mut c_char,
                buffer_size: usize,
                out_size: *mut usize,
                out_found: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_hub_get_peer_id(hub: *mut ExtHub, out_peer_id: *mut QarPeerId) -> QarResult;
            pub fn qar_impl_ext_hub_handle_destroy(hub: *mut ExtHub);
            pub fn qar_impl_ext_hub_mint_invite(
                hub: *mut ExtHub,
                method: ExtInviteMethod,
                out_buffer: *mut c_char,
                buffer_size: usize,
                out_size: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_hub_query_media_streams(
                hub: *mut ExtHub,
                out_records: *mut ExtMediaStreamRecord,
                capacity: usize,
                out_count: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_layout_side_by_side(
                eye_width: u32,
                eye_height: u32,
                out_layout: *mut QarVideoFrameLayout,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_app_volume_add_peer(
                peer: *mut ExtPeer,
                volume: *const QarAppVolumeId,
                added: *const QarPeerId,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_app_volume_remove_peer(
                peer: *mut ExtPeer,
                volume: *const QarAppVolumeId,
                removed: *const QarPeerId,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_app_volume_set_focused(
                peer: *mut ExtPeer,
                volume: *const QarAppVolumeId,
                focused: bool,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_app_volume_start_gesturing(
                peer: *mut ExtPeer,
                volume: *const QarAppVolumeId,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_app_volume_stop_gesturing(
                peer: *mut ExtPeer,
                volume: *const QarAppVolumeId,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_create(
                hub: *mut ExtHub,
                init: *const ExtPeerInit,
                out_peer: *mut *mut ExtPeer,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_get_id(peer: *mut ExtPeer, out_peer_id: *mut QarPeerId) -> QarResult;
            pub fn qar_impl_ext_peer_gui_panel_gesture(
                peer: *mut ExtPeer,
                gesture: *const ExtGuiPanelGesture,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_handle_destroy(peer: *mut ExtPeer);
            pub fn qar_impl_ext_peer_query_media_streams(
                peer: *mut ExtPeer,
                out_records: *mut ExtMediaStreamRecord,
                capacity: usize,
                out_count: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_peer_send_device_pose(peer: *mut ExtPeer, head_pose: *const QarPose) -> QarResult;
            pub fn qar_impl_ext_peer_send_hand_joints(
                peer: *mut ExtPeer,
                hands: *const QarDeviceHandsWithJoints,
            ) -> QarResult;
            pub fn qar_impl_ext_render_receiver_create(
                peer: *mut ExtPeer,
                init: *const ExtRenderReceiverInit,
                out_receiver: *mut *mut ExtRenderReceiver,
            ) -> QarResult;
            pub fn qar_impl_ext_render_receiver_handle_destroy(receiver: *mut ExtRenderReceiver);
            pub fn qar_impl_ext_render_receiver_request_render(
                receiver: *mut ExtRenderReceiver,
                request: *const ExtRenderRequest,
            ) -> QarResult;
            pub fn qar_impl_ext_render_receiver_send_pose(
                receiver: *mut ExtRenderReceiver,
                head_pose: *const QarPose,
            ) -> QarResult;
            pub fn qar_impl_ext_render_receiver_video(
                receiver: *mut ExtRenderReceiver,
                out_video: *mut *mut ExtVideoReceiver,
            ) -> QarResult;
            pub fn qar_impl_ext_scale_ms(milliseconds: u64) -> u64;
            pub fn qar_impl_ext_target_app_responder_create(
                hub: *mut ExtHub,
                init: *const ExtTargetAppResponderInit,
                out_responder: *mut *mut ExtTargetAppResponder,
            ) -> QarResult;
            pub fn qar_impl_ext_target_app_responder_get_invite(
                responder: *mut ExtTargetAppResponder,
                index: usize,
                out_record: *mut ExtTargetAppInviteRecord,
            ) -> QarResult;
            pub fn qar_impl_ext_target_app_responder_get_shutdown(
                responder: *mut ExtTargetAppResponder,
                index: usize,
                out_record: *mut ExtTargetAppShutdownRecord,
            ) -> QarResult;
            pub fn qar_impl_ext_target_app_responder_handle_destroy(responder: *mut ExtTargetAppResponder);
            pub fn qar_impl_ext_target_app_responder_invite_count(
                responder: *mut ExtTargetAppResponder,
                out_count: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_target_app_responder_running_count(
                responder: *mut ExtTargetAppResponder,
                out_count: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_target_app_responder_shutdown_count(
                responder: *mut ExtTargetAppResponder,
                out_count: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_time_scale() -> f64;
            pub fn qar_impl_ext_video_receiver_create(
                peer: *mut ExtPeer,
                init: *const ExtVideoReceiverInit,
                out_receiver: *mut *mut ExtVideoReceiver,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_handle_destroy(receiver: *mut ExtVideoReceiver);
            pub fn qar_impl_ext_video_receiver_last_frame_vulkan(
                receiver: *mut ExtVideoReceiver,
                out_frame: *mut QarVideoFrameVulkan,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_pause(receiver: *mut ExtVideoReceiver) -> QarResult;
            pub fn qar_impl_ext_video_receiver_receive_frame(
                receiver: *mut ExtVideoReceiver,
                timeout_ms: u32,
                out_info: *mut ExtVideoFrameInfo,
                out_received: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_resume(receiver: *mut ExtVideoReceiver) -> QarResult;
            pub fn qar_impl_ext_video_receiver_source_layout(
                receiver: *mut ExtVideoReceiver,
                out_layout: *mut QarVideoFrameLayout,
                out_known: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_stream_id(
                receiver: *mut ExtVideoReceiver,
                out_stream_id: *mut QarStreamId,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_try_provide_frame_cpu(
                receiver: *mut ExtVideoReceiver,
                frame: *const QarVideoFrameCpu,
                out_accepted: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_try_provide_frame_vulkan(
                receiver: *mut ExtVideoReceiver,
                frame: *const QarVideoFrameVulkan,
                out_accepted: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_video_receiver_wait_connected(
                receiver: *mut ExtVideoReceiver,
                timeout_ms: u32,
                out_connected: *mut bool,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_device_create(out_device: *mut *mut ExtVulkanDevice) -> QarResult;
            pub fn qar_impl_ext_vulkan_device_handle_destroy(device: *mut ExtVulkanDevice);
            pub fn qar_impl_ext_vulkan_device_params(
                device: *mut ExtVulkanDevice,
                out_params: *mut QarStreamParamsVulkan,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_frame_create(
                device: *mut ExtVulkanDevice,
                layout: *const QarVideoFrameLayout,
                out_frame: *mut *mut ExtVulkanFrame,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_frame_handle_destroy(frame: *mut ExtVulkanFrame);
            pub fn qar_impl_ext_vulkan_frame_lend(
                frame: *mut ExtVulkanFrame,
                out_frame: *mut QarVideoFrameVulkan,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_read_texture(
                device: *mut ExtVulkanDevice,
                frame: *mut QarVideoFrameVulkan,
                texture_index: usize,
                wait_frame_semaphore: bool,
                out_data: *mut u8,
                buffer_size: usize,
                out_size: *mut usize,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_release_frame(
                device: *mut ExtVulkanDevice,
                frame: *mut QarVideoFrameVulkan,
            ) -> QarResult;
            pub fn qar_impl_ext_vulkan_write_texture(
                device: *mut ExtVulkanDevice,
                frame: *mut QarVideoFrameVulkan,
                texture_index: usize,
                data: *const u8,
                data_size: usize,
            ) -> QarResult;
        }
    };
}

pub(crate) use ext_function_list;

/// Sizes and field offsets of the mirrored structures on 64-bit targets, checked at compile time.
#[cfg(target_pointer_width = "64")]
const _: () = {
    use std::mem::{offset_of, size_of};

    assert!(size_of::<ExtHubInit>() == 24);
    assert!(offset_of!(ExtHubInit, enable_password_onboarding) == 16);
    assert!(size_of::<ExtPeerInit>() == 24);
    assert!(offset_of!(ExtPeerInit, display_name) == 16);
    assert!(size_of::<ExtVideoReceiverInit>() == 416);
    assert!(offset_of!(ExtVideoReceiverInit, optimal_layout) == 16);
    assert!(offset_of!(ExtVideoReceiverInit, has_stream_id) == 384);
    assert!(offset_of!(ExtVideoReceiverInit, stream_id) == 385);
    assert!(offset_of!(ExtVideoReceiverInit, role) == 404);
    assert!(offset_of!(ExtVideoReceiverInit, start_paused) == 408);
    assert!(size_of::<ExtRenderReceiverInit>() == 384);
    assert!(offset_of!(ExtRenderReceiverInit, optimal_layout) == 16);
    assert!(size_of::<ExtRenderRequest>() == 40);
    assert!(offset_of!(ExtRenderRequest, frame_count) == 32);
    assert!(size_of::<ExtVideoFrameInfo>() == 24);
    assert!(offset_of!(ExtVideoFrameInfo, sequence_number) == 8);
    assert!(offset_of!(ExtVideoFrameInfo, creation_time_ns) == 16);
    assert!(size_of::<ExtTargetAppResponderInit>() == 24);
    assert!(offset_of!(ExtTargetAppResponderInit, machine_id) == 16);
    assert!(size_of::<ExtTargetAppInviteRecord>() == 545);
    assert!(offset_of!(ExtTargetAppInviteRecord, hostname) == 16);
    assert!(offset_of!(ExtTargetAppInviteRecord, device) == 272);
    assert!(offset_of!(ExtTargetAppInviteRecord, answered) == 528);
    assert!(offset_of!(ExtTargetAppInviteRecord, target) == 529);
    assert!(size_of::<ExtTargetAppShutdownRecord>() == 17);
    assert!(offset_of!(ExtTargetAppShutdownRecord, answered) == 16);
    assert!(size_of::<ExtMediaStreamRecord>() == 432);
    assert!(offset_of!(ExtMediaStreamRecord, kind) == 16);
    assert!(offset_of!(ExtMediaStreamRecord, lifetime) == 20);
    assert!(offset_of!(ExtMediaStreamRecord, has_video) == 24);
    assert!(offset_of!(ExtMediaStreamRecord, sender_peer) == 25);
    assert!(offset_of!(ExtMediaStreamRecord, receiver_count) == 48);
    assert!(offset_of!(ExtMediaStreamRecord, has_source_layout) == 56);
    assert!(offset_of!(ExtMediaStreamRecord, source_layout) == 64);
    assert!(size_of::<ExtGuiPanelGesture>() == 56);
    assert!(offset_of!(ExtGuiPanelGesture, kind) == 16);
    assert!(offset_of!(ExtGuiPanelGesture, phase) == 20);
    assert!(offset_of!(ExtGuiPanelGesture, gesture_id) == 24);
    assert!(offset_of!(ExtGuiPanelGesture, handedness) == 32);
    assert!(offset_of!(ExtGuiPanelGesture, x) == 36);
    assert!(offset_of!(ExtGuiPanelGesture, start_y) == 48);
    assert!(offset_of!(ExtGuiPanelGesture, tap_count) == 52);
};
