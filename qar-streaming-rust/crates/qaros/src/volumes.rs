use std::ffi::{c_char, c_void};
use std::ptr;
use std::rc::Rc;

use qaros_sys::{self as sys, QarApi, QarResult, defaults};

use crate::error::{Result, check};
use crate::session::{CallbackState, SessionInner, Subscription, query_owned_handles};
use crate::strings;
use crate::types::{AppVolumeId, AppVolumeLifetime, PeerId, Pose};

/// Box extents in meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AppVolumeSize {
    pub width: f32,
    pub length: f32,
    pub height: f32,
}

impl Default for AppVolumeSize {
    /// `qar_app_volume_size_default()`: one meter cube.
    fn default() -> Self {
        Self::from_raw(defaults::qar_app_volume_size_default())
    }
}

impl AppVolumeSize {
    fn to_raw(self) -> sys::QarAppVolumeSize {
        sys::QarAppVolumeSize {
            width_meters: self.width,
            length_meters: self.length,
            height_meters: self.height,
        }
    }

    fn from_raw(raw: sys::QarAppVolumeSize) -> Self {
        Self {
            width: raw.width_meters,
            length: raw.length_meters,
            height: raw.height_meters,
        }
    }
}

/// Options for `qar_app_volumes_get_or_create`.
#[derive(Clone, Debug)]
pub struct AppVolumeConfig {
    /// Stable identity; the volume id derives from it, so the same name
    /// reattaches to the same volume.
    pub common_name: String,
    pub display_name: Option<String>,
    /// Pose of the cuboid's center in room space.
    pub pose: Pose,
    pub size: AppVolumeSize,
    pub initial_peers: Vec<PeerId>,
    /// Where the app origin sits, in volume meters (not scaled).
    pub app_pose: Pose,
    /// Room meters per app meter.
    pub app_scale: f32,
}

impl AppVolumeConfig {
    /// The values of `qar_app_volume_init_default()` with this common name.
    pub fn new(common_name: impl Into<String>) -> Self {
        Self {
            common_name: common_name.into(),
            display_name: None,
            pose: Pose::default(),
            size: AppVolumeSize::default(),
            initial_peers: Vec::new(),
            app_pose: Pose::default(),
            app_scale: 1.0,
        }
    }
}

/// Snapshot of a `QarAppVolume`.
#[derive(Clone, Debug, PartialEq)]
pub struct AppVolumeInfo {
    pub id: AppVolumeId,
    pub display_name: String,
    pub pose: Pose,
    pub size: AppVolumeSize,
    pub lifetime: AppVolumeLifetime,
    pub app_pose: Pose,
    pub app_scale: f32,
}

/// App volume operations of a session (`session.app_volumes()`).
#[derive(Debug)]
pub struct AppVolumes<'session> {
    session: &'session Rc<SessionInner>,
}

macro_rules! volume_setter {
    ($(#[$documentation:meta])* $method:ident, $function:ident, $value_type:ty, |$value:ident| $raw_value:expr, $pass:ident) => {
        $(#[$documentation])*
        pub fn $method(&self, volume: AppVolumeId, $value: $value_type) -> Result<()> {
            let api = self.session.api();
            let volume_id = volume.to_raw();
            let raw_value = $raw_value;
            // SAFETY: live session; arguments live for the call.
            let result = unsafe {
                (api.$function)(self.session.handle.as_ptr(), &raw const volume_id, volume_setter!(@pass $pass raw_value))
            };
            check(api, stringify!($function), result)
        }
    };
    (@pass by_pointer $raw_value:ident) => { &raw const $raw_value };
    (@pass by_value $raw_value:ident) => { $raw_value };
}

macro_rules! volume_latest_getter {
    ($(#[$documentation:meta])* $method:ident, $function:ident, $value_type:ty, $initial:expr, |$raw:ident| $convert:expr) => {
        $(#[$documentation])*
        pub fn $method(&self, volume: AppVolumeId) -> Result<$value_type> {
            let api = self.session.api();
            let volume_id = volume.to_raw();
            let mut $raw = $initial;
            // SAFETY: live session; valid pointers for the call.
            let result = unsafe {
                (api.$function)(self.session.handle.as_ptr(), &raw const volume_id, &raw mut $raw)
            };
            check(api, stringify!($function), result)?;
            Ok($convert)
        }
    };
}

impl<'session> AppVolumes<'session> {
    pub(crate) fn new(session: &'session Rc<SessionInner>) -> Self {
        Self { session }
    }

    /// Returns the volume named by `config.common_name`, creating it if
    /// needed. Idempotent.
    pub fn get_or_create(&self, config: &AppVolumeConfig) -> Result<AppVolumeId> {
        let common_name = strings::to_c_string(&config.common_name, "common name")?;
        let display_name =
            strings::optional_c_string(config.display_name.as_deref(), "display name")?;
        let initial_peers: Vec<sys::QarPeerId> = config
            .initial_peers
            .iter()
            .map(|peer| peer.to_raw())
            .collect();
        let mut initial_peer_pointers: Vec<*const sys::QarPeerId> =
            initial_peers.iter().map(ptr::from_ref).collect();

        let mut init = defaults::qar_app_volume_init_default();
        init.common_name = common_name.as_ptr();
        init.display_name = strings::pointer_or_null(display_name.as_ref());
        init.pose = config.pose.to_raw();
        init.size = config.size.to_raw();
        if !initial_peer_pointers.is_empty() {
            init.initial_peers = initial_peer_pointers.as_mut_ptr();
            init.initial_peer_count = initial_peer_pointers.len();
        }
        init.app_pose = config.app_pose.to_raw();
        init.app_scale = config.app_scale;

        let api = self.session.api();
        let mut volume_id = defaults::qar_app_volume_id_default();
        // SAFETY: init and everything it points to live for the call.
        let result = unsafe {
            (api.qar_impl_app_volumes_get_or_create)(
                self.session.handle.as_ptr(),
                &raw const init,
                &raw mut volume_id,
            )
        };
        check(api, "qar_app_volumes_get_or_create", result)?;
        Ok(AppVolumeId::from_raw(volume_id))
    }

    volume_setter!(
        /// Moves the box (pose of its center, room space).
        set_pose, qar_impl_app_volumes_change_pose, Pose, |pose| pose.to_raw(), by_pointer
    );
    volume_setter!(
        /// Resizes the box.
        set_size,
        qar_impl_app_volumes_change_size,
        AppVolumeSize,
        |size| size.to_raw(),
        by_pointer
    );
    volume_setter!(
        /// Moves the content inside the box (volume meters).
        set_app_pose, qar_impl_app_volumes_change_app_pose, Pose, |app_pose| app_pose.to_raw(), by_pointer
    );
    volume_setter!(
        /// Room meters per app meter.
        set_app_scale, qar_impl_app_volumes_change_app_scale, f32, |app_scale| app_scale, by_value
    );

    /// Renames the volume (UTF-8).
    pub fn set_display_name(&self, volume: AppVolumeId, display_name: &str) -> Result<()> {
        let api = self.session.api();
        let volume_id = volume.to_raw();
        let display_name = strings::to_c_string(display_name, "display name")?;
        // SAFETY: live session; arguments live for the call.
        let result = unsafe {
            (api.qar_impl_app_volumes_change_display_name)(
                self.session.handle.as_ptr(),
                &raw const volume_id,
                display_name.as_ptr(),
            )
        };
        check(api, "qar_app_volumes_change_display_name", result)
    }

    /// Closes the volume (`qar_app_volumes_close_volume`).
    pub fn close(&self, volume: AppVolumeId) -> Result<()> {
        let api = self.session.api();
        let volume_id = volume.to_raw();
        // SAFETY: live session; argument lives for the call.
        let result = unsafe {
            (api.qar_impl_app_volumes_close_volume)(
                self.session.handle.as_ptr(),
                &raw const volume_id,
            )
        };
        check(api, "qar_app_volumes_close_volume", result)
    }

    volume_latest_getter!(
        /// Latest locally known pose, including changes by other peers and gestures.
        latest_pose, qar_impl_app_volume_get_latest_pose, Pose, defaults::qar_pose_default(), |raw| Pose::from_raw(raw)
    );
    volume_latest_getter!(
        /// Latest locally known size.
        latest_size,
        qar_impl_app_volume_get_latest_size,
        AppVolumeSize,
        defaults::qar_app_volume_size_default(),
        |raw| AppVolumeSize::from_raw(raw)
    );
    volume_latest_getter!(
        /// Latest locally known pose of the content inside the box.
        latest_app_pose,
        qar_impl_app_volume_get_latest_app_pose,
        Pose,
        defaults::qar_pose_default(),
        |raw| Pose::from_raw(raw)
    );
    volume_latest_getter!(
        /// Latest locally known scale, room meters per app meter.
        latest_app_scale,
        qar_impl_app_volume_get_latest_app_scale,
        f32,
        1.0_f32,
        |raw| raw
    );

    /// Snapshot of every volume known to the session.
    pub fn list(&self) -> Result<Vec<AppVolumeInfo>> {
        let api = self.session.api();
        let session = self.session.handle.as_ptr();
        query_owned_handles(
            api,
            // SAFETY: live session; valid out-pointer.
            |count| unsafe { (api.qar_impl_query_app_volumes_count)(session, count) },
            // SAFETY: the buffer holds `capacity` handle slots.
            |handles, capacity, written| unsafe {
                (api.qar_impl_query_app_volumes)(session, handles, capacity, written)
            },
            api.qar_impl_app_volume_handle_destroy,
            "qar_query_app_volumes",
            |handle| read_app_volume_info(api, handle),
        )
    }

    /// Calls `callback` with a snapshot whenever a volume changes. Runs on
    /// library threads.
    pub fn subscribe(
        &self,
        callback: impl Fn(AppVolumeInfo) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.session.handle.as_ptr();
        self.session.subscribe(
            Box::new(callback),
            "qar_app_volumes_subscribe_updates",
            |api, user_state, token| {
                // SAFETY: live session; user_state stays valid until the
                // session handle is destroyed.
                unsafe {
                    (api.qar_impl_app_volumes_subscribe_updates)(
                        session,
                        Some(app_volume_update_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }
}

fn read_app_volume_info(api: &QarApi, handle: *mut sys::QarAppVolume) -> Result<AppVolumeInfo> {
    let mut volume_id = defaults::qar_app_volume_id_default();
    let mut pose = defaults::qar_pose_default();
    let mut size = defaults::qar_app_volume_size_default();
    let mut lifetime = sys::QarAppVolumeLifetimeStatus::QAR_APP_VOLUME_ACTIVE;
    let mut app_pose = defaults::qar_pose_default();
    let mut app_scale = 1.0_f32;
    // SAFETY (all getters below): handle is live, out-pointers are valid.
    let results: [(&'static str, QarResult); 6] = unsafe {
        [
            (
                "qar_app_volume_get_id",
                (api.qar_impl_app_volume_get_id)(handle, &raw mut volume_id),
            ),
            (
                "qar_app_volume_get_pose",
                (api.qar_impl_app_volume_get_pose)(handle, &raw mut pose),
            ),
            (
                "qar_app_volume_get_size",
                (api.qar_impl_app_volume_get_size)(handle, &raw mut size),
            ),
            (
                "qar_app_volume_get_lifetime_status",
                (api.qar_impl_app_volume_get_lifetime_status)(handle, &raw mut lifetime),
            ),
            (
                "qar_app_volume_get_app_pose",
                (api.qar_impl_app_volume_get_app_pose)(handle, &raw mut app_pose),
            ),
            (
                "qar_app_volume_get_app_scale",
                (api.qar_impl_app_volume_get_app_scale)(handle, &raw mut app_scale),
            ),
        ]
    };
    for (function, result) in results {
        check(api, function, result)?;
    }
    let display_name = strings::read_string(|buffer: *mut c_char, size: usize| {
        // SAFETY: handle is live; the buffer is writable for size bytes.
        let result = unsafe { (api.qar_impl_app_volume_get_display_name)(handle, buffer, size) };
        check(api, "qar_app_volume_get_display_name", result)
    })?;
    Ok(AppVolumeInfo {
        id: AppVolumeId::from_raw(volume_id),
        display_name,
        pose: Pose::from_raw(pose),
        size: AppVolumeSize::from_raw(size),
        lifetime: AppVolumeLifetime::from_raw(lifetime),
        app_pose: Pose::from_raw(app_pose),
        app_scale,
    })
}

unsafe extern "C" fn app_volume_update_trampoline(
    handle: *mut sys::QarAppVolume,
    user_state: *mut c_void,
) {
    // SAFETY: registered by AppVolumes::subscribe with this event type.
    let state = unsafe { CallbackState::<AppVolumeInfo>::from_user_state(user_state) };
    if handle.is_null() {
        return;
    }
    // Borrowed for the duration of the callback: read, never destroy.
    match read_app_volume_info(state.api(), handle) {
        Ok(volume) => state.deliver(volume, "app volume update"),
        Err(error) => log::warn!("reading an app volume update: {error}"),
    }
}
