use std::ffi::{c_char, c_void};
use std::rc::Rc;

use qaros_sys::{self as sys, QarApi, QarResult, defaults};

use crate::error::{Result, check};
use crate::session::{CallbackState, SessionInner, Subscription, query_owned_handles};
use crate::strings;
use crate::types::{GuiPanelId, GuiPanelState, PeerId, Pose};

/// Physical size in meters and the scale mapping web content onto it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuiPanelSize {
    pub width: f32,
    pub height: f32,
    pub content_scale: f32,
}

impl Default for GuiPanelSize {
    /// `qar_gui_panel_size_default()`: 1 m by 1 m, content scale 1.
    fn default() -> Self {
        Self::from_raw(defaults::qar_gui_panel_size_default())
    }
}

impl GuiPanelSize {
    fn to_raw(self) -> sys::QarGuiPanelSize {
        sys::QarGuiPanelSize {
            width_meters: self.width,
            height_meters: self.height,
            content_scale: self.content_scale,
        }
    }

    fn from_raw(raw: sys::QarGuiPanelSize) -> Self {
        Self {
            width: raw.width_meters,
            height: raw.height_meters,
            content_scale: raw.content_scale,
        }
    }
}

/// Options for `qar_gui_panels_get_or_create`.
#[derive(Clone, Debug)]
pub struct GuiPanelConfig {
    /// Stable identity, for example `status.myapp.example.com`; the panel id
    /// derives from it.
    pub common_name: String,
    pub display_name: Option<String>,
    /// Pose of panel space in room space. Its origin is the midpoint of the
    /// panel's top edge, and the surface hangs below it (x in ±width/2, y in
    /// [-height, 0]). +X runs along the top edge to the viewer's left, +Y up,
    /// and -Z is the front normal pointing at the viewer, so a panel with the
    /// identity orientation faces room -Z (`qar_streaming.h` "Panel space").
    pub pose: Pose,
    pub size: GuiPanelSize,
    /// Empty: visible to everyone.
    pub visible_to: Vec<PeerId>,
}

impl GuiPanelConfig {
    /// The values of `qar_gui_panel_init_default()` with this common name.
    pub fn new(common_name: impl Into<String>) -> Self {
        Self {
            common_name: common_name.into(),
            display_name: None,
            pose: Pose::default(),
            size: GuiPanelSize::default(),
            visible_to: Vec::new(),
        }
    }
}

/// Snapshot of a `QarGuiPanel`.
#[derive(Clone, Debug, PartialEq)]
pub struct GuiPanelInfo {
    pub id: GuiPanelId,
    pub display_name: String,
    pub pose: Pose,
    pub size: GuiPanelSize,
    pub content_uri: String,
    pub state: GuiPanelState,
    pub visible_to: Vec<PeerId>,
}

/// GUI panel operations of a session (`session.gui_panels()`).
#[derive(Debug)]
pub struct GuiPanels<'session> {
    session: &'session Rc<SessionInner>,
}

impl<'session> GuiPanels<'session> {
    pub(crate) fn new(session: &'session Rc<SessionInner>) -> Self {
        Self { session }
    }

    /// Returns the panel named by `config.common_name`, creating it if
    /// needed. Idempotent.
    pub fn get_or_create(&self, config: &GuiPanelConfig) -> Result<GuiPanelId> {
        let common_name = strings::to_c_string(&config.common_name, "common name")?;
        let display_name =
            strings::optional_c_string(config.display_name.as_deref(), "display name")?;
        let visible_to: Vec<sys::QarPeerId> =
            config.visible_to.iter().map(|peer| peer.to_raw()).collect();

        let mut init = defaults::qar_gui_panel_init_default();
        init.common_name = common_name.as_ptr();
        init.display_name = strings::pointer_or_null(display_name.as_ref());
        init.pose = config.pose.to_raw();
        init.size = config.size.to_raw();
        if !visible_to.is_empty() {
            init.visible_to_peers = visible_to.as_ptr();
            init.visible_to_peer_count = visible_to.len();
        }

        let api = self.session.api();
        let mut panel_id = defaults::qar_gui_panel_id_default();
        // SAFETY: init and everything it points to live for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_get_or_create)(
                self.session.handle.as_ptr(),
                &raw const init,
                &raw mut panel_id,
            )
        };
        check(api, "qar_gui_panels_get_or_create", result)?;
        Ok(GuiPanelId::from_raw(panel_id))
    }

    /// Moves the panel (top-edge midpoint, room space; see
    /// [`GuiPanelConfig::pose`]).
    pub fn set_pose(&self, panel: GuiPanelId, pose: Pose) -> Result<()> {
        let api = self.session.api();
        let panel_id = panel.to_raw();
        let pose = pose.to_raw();
        // SAFETY: live session; arguments live for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_update_pose)(
                self.session.handle.as_ptr(),
                &raw const panel_id,
                &raw const pose,
            )
        };
        check(api, "qar_gui_panels_update_pose", result)
    }

    /// Changes the panel's size and content scale (`qar_gui_panels_change_size`).
    pub fn set_size(&self, panel: GuiPanelId, size: GuiPanelSize) -> Result<()> {
        let api = self.session.api();
        let panel_id = panel.to_raw();
        let size = size.to_raw();
        // SAFETY: live session; arguments live for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_change_size)(
                self.session.handle.as_ptr(),
                &raw const panel_id,
                &raw const size,
            )
        };
        check(api, "qar_gui_panels_change_size", result)
    }

    /// Sets the panel's state: visible, minimized or hidden (`qar_gui_panels_set_state`).
    pub fn set_state(&self, panel: GuiPanelId, state: GuiPanelState) -> Result<()> {
        let api = self.session.api();
        let panel_id = panel.to_raw();
        // SAFETY: live session; arguments live for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_set_state)(
                self.session.handle.as_ptr(),
                &raw const panel_id,
                state.to_raw(),
            )
        };
        check(api, "qar_gui_panels_set_state", result)
    }

    /// Points the panel at web content.
    pub fn navigate_to_uri(&self, panel: GuiPanelId, uri: &str) -> Result<()> {
        let api = self.session.api();
        let panel_id = panel.to_raw();
        let uri = strings::to_c_string(uri, "URI")?;
        // SAFETY: live session; arguments live for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_navigate_to_uri)(
                self.session.handle.as_ptr(),
                &raw const panel_id,
                uri.as_ptr(),
            )
        };
        check(api, "qar_gui_panels_navigate_to_uri", result)
    }

    /// Closes and removes the panel (`qar_gui_panels_close_panel`).
    pub fn close(&self, panel: GuiPanelId) -> Result<()> {
        let api = self.session.api();
        let panel_id = panel.to_raw();
        // SAFETY: live session; argument lives for the call.
        let result = unsafe {
            (api.qar_impl_gui_panels_close_panel)(self.session.handle.as_ptr(), &raw const panel_id)
        };
        check(api, "qar_gui_panels_close_panel", result)
    }

    /// Snapshot of every panel in the session.
    pub fn list(&self) -> Result<Vec<GuiPanelInfo>> {
        let api = self.session.api();
        let session = self.session.handle.as_ptr();
        query_owned_handles(
            api,
            // SAFETY: live session; valid out-pointer.
            |count| unsafe { (api.qar_impl_query_gui_panels_count)(session, count) },
            // SAFETY: the buffer holds `capacity` handle slots.
            |handles, capacity, written| unsafe {
                (api.qar_impl_query_gui_panels)(session, handles, capacity, written)
            },
            api.qar_impl_gui_panel_handle_destroy,
            "qar_query_gui_panels",
            |handle| read_gui_panel_info(api, handle),
        )
    }

    /// Calls `callback` with a snapshot whenever any panel changes. Runs on
    /// library threads.
    pub fn subscribe(
        &self,
        callback: impl Fn(GuiPanelInfo) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.session.handle.as_ptr();
        self.session.subscribe(
            Box::new(callback),
            "qar_gui_panels_subscribe_updates",
            |api, user_state, token| {
                // SAFETY: live session; user_state stays valid until the
                // session handle is destroyed.
                unsafe {
                    (api.qar_impl_gui_panels_subscribe_updates)(
                        session,
                        Some(gui_panel_update_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Like [`GuiPanels::subscribe`], for one panel.
    pub fn subscribe_panel(
        &self,
        panel: GuiPanelId,
        callback: impl Fn(GuiPanelInfo) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.session.handle.as_ptr();
        let panel_id = panel.to_raw();
        self.session.subscribe(
            Box::new(callback),
            "qar_gui_panels_subscribe_panel_updates",
            |api, user_state, token| {
                // SAFETY: as in subscribe; panel_id lives for the call.
                unsafe {
                    (api.qar_impl_gui_panels_subscribe_panel_updates)(
                        session,
                        &raw const panel_id,
                        Some(gui_panel_update_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }
}

fn read_gui_panel_info(api: &QarApi, handle: *mut sys::QarGuiPanel) -> Result<GuiPanelInfo> {
    let mut panel_id = defaults::qar_gui_panel_id_default();
    let mut pose = defaults::qar_pose_default();
    let mut size = defaults::qar_gui_panel_size_default();
    let mut state = sys::QarGuiPanelState::QAR_GUI_PANEL_STATE_VISIBLE;
    let mut visible_count = 0_usize;
    // SAFETY (all getters below): handle is live, out-pointers are valid.
    let results: [(&'static str, QarResult); 5] = unsafe {
        [
            (
                "qar_gui_panel_get_id",
                (api.qar_impl_gui_panel_get_id)(handle, &raw mut panel_id),
            ),
            (
                "qar_gui_panel_get_pose",
                (api.qar_impl_gui_panel_get_pose)(handle, &raw mut pose),
            ),
            (
                "qar_gui_panel_get_size",
                (api.qar_impl_gui_panel_get_size)(handle, &raw mut size),
            ),
            (
                "qar_gui_panel_get_state",
                (api.qar_impl_gui_panel_get_state)(handle, &raw mut state),
            ),
            (
                "qar_gui_panel_get_visible_to_peers_count",
                (api.qar_impl_gui_panel_get_visible_to_peers_count)(handle, &raw mut visible_count),
            ),
        ]
    };
    for (function, result) in results {
        check(api, function, result)?;
    }

    let mut visible_to = vec![defaults::qar_peer_id_default(); visible_count];
    let mut written = 0_usize;
    if visible_count > 0 {
        // SAFETY: the buffer holds visible_count ids.
        let result = unsafe {
            (api.qar_impl_gui_panel_get_visible_to_peers)(
                handle,
                visible_to.as_mut_ptr(),
                visible_to.len(),
                &raw mut written,
            )
        };
        check(api, "qar_gui_panel_get_visible_to_peers", result)?;
    }
    visible_to.truncate(written);

    let display_name = strings::read_string(|buffer: *mut c_char, buffer_size: usize| {
        // SAFETY: handle is live; the buffer is writable for buffer_size bytes.
        let result =
            unsafe { (api.qar_impl_gui_panel_get_display_name)(handle, buffer, buffer_size) };
        check(api, "qar_gui_panel_get_display_name", result)
    })?;
    let content_uri = strings::read_string(|buffer: *mut c_char, buffer_size: usize| {
        // SAFETY: as above.
        let result =
            unsafe { (api.qar_impl_gui_panel_get_content_uri)(handle, buffer, buffer_size) };
        check(api, "qar_gui_panel_get_content_uri", result)
    })?;

    Ok(GuiPanelInfo {
        id: GuiPanelId::from_raw(panel_id),
        display_name,
        pose: Pose::from_raw(pose),
        size: GuiPanelSize::from_raw(size),
        content_uri,
        state: GuiPanelState::from_raw(state),
        visible_to: visible_to.into_iter().map(PeerId::from_raw).collect(),
    })
}

unsafe extern "C" fn gui_panel_update_trampoline(
    handle: *mut sys::QarGuiPanel,
    user_state: *mut c_void,
) {
    // SAFETY: registered by GuiPanels::subscribe* with this event type.
    let state = unsafe { CallbackState::<GuiPanelInfo>::from_user_state(user_state) };
    if handle.is_null() {
        return;
    }
    // Borrowed for the duration of the callback: read, never destroy.
    match read_gui_panel_info(state.api(), handle) {
        Ok(panel) => state.deliver(panel, "GUI panel update"),
        Err(error) => log::warn!("reading a GUI panel update: {error}"),
    }
}
