//! Web pages a session serves to served GUI panels, and the JSON messages between those pages
//! and the session (`qar_panel_content_*`).

use std::ffi::{c_char, c_void};
use std::path::Path;
use std::rc::Rc;

use qaros_sys as sys;

use crate::error::{Error, Result, check};
use crate::session::{CallbackState, SessionInner, Subscription};
use crate::strings;
use crate::types::{GuiPanelId, PeerId};

/// Longest bundle name, in bytes (`QAR_PANEL_CONTENT_BUNDLE_NAME_MAX`).
pub const PANEL_CONTENT_BUNDLE_NAME_MAX: usize = sys::QAR_PANEL_CONTENT_BUNDLE_NAME_MAX as usize;

/// Names one content bundle (`QarPanelContentBundleId`): the session that published it and
/// serves its files, and its name, unique per publisher.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PanelContentBundleId {
    /// The publishing peer: the content's owner.
    pub publisher: PeerId,
    /// ASCII `A-Z a-z 0-9 . _ -`, not starting with `.`, at most
    /// [`PANEL_CONTENT_BUNDLE_NAME_MAX`] bytes.
    pub name: String,
}

impl PanelContentBundleId {
    /// The C id. A name too long or holding NUL is `Error::InvalidArgument`; the C API checks
    /// the remaining rules.
    pub(crate) fn to_raw(&self) -> Result<sys::QarPanelContentBundleId> {
        let bytes = self.name.as_bytes();
        if bytes.len() > PANEL_CONTENT_BUNDLE_NAME_MAX || bytes.contains(&0) {
            return Err(Error::InvalidArgument(format!(
                "bundle name {:?} is longer than {PANEL_CONTENT_BUNDLE_NAME_MAX} bytes or holds NUL",
                self.name
            )));
        }
        let mut raw = sys::QarPanelContentBundleId {
            publisher: self.publisher.to_raw(),
            name: [0; PANEL_CONTENT_BUNDLE_NAME_MAX + 1],
        };
        for (slot, byte) in raw.name.iter_mut().zip(bytes) {
            *slot = c_char::from_ne_bytes([*byte]);
        }
        Ok(raw)
    }

    pub(crate) fn from_raw(raw: &sys::QarPanelContentBundleId) -> Self {
        let bytes: Vec<u8> = raw
            .name
            .iter()
            .map(|byte| u8::from_ne_bytes(byte.to_ne_bytes()))
            .take_while(|byte| *byte != 0)
            .collect();
        Self {
            publisher: PeerId::from_raw(raw.publisher),
            name: String::from_utf8_lossy(&bytes).into_owned(),
        }
    }
}

/// What a served GUI panel shows: one entry page of a content bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuiPanelServedContent {
    pub bundle: PanelContentBundleId,
    /// Path of the page inside the bundle, for example `inspector/index.html`.
    pub entry: String,
}

/// One file of [`PanelContent::publish_files`] (`QarPanelContentFile`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanelContentFile {
    /// Relative, `/`-separated path inside the bundle.
    pub path: String,
    /// `None`: taken from the path's extension. Otherwise it must be the type that extension
    /// is served as.
    pub mime_type: Option<String>,
    pub data: Vec<u8>,
}

/// A message a served panel sent to its content owner, stamped by the mixer showing it
/// (`QarPanelContentMessage`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanelContentMessage {
    pub panel: GuiPanelId,
    /// The user (the mixer's target peer) whose panel sent it.
    pub user: PeerId,
    /// Counts up per sending mixer and panel, from 1.
    pub sequence: u64,
    /// One JSON value.
    pub json: String,
}

/// Panel content operations of a session (`session.panel_content()`).
///
/// A served GUI panel shows a page of a content bundle this session publishes. Every user's
/// mixer loads the page in its own browser and fetches the files over the session; the page
/// sends with `qaros.panel.send(msg)` and receives with `qaros.panel.onMessage(cb)`. Publishing
/// copies every byte, so an edit needs another publication, which reloads every panel showing
/// the bundle. Only the publisher may put its bundle on a panel, receives the panels' messages
/// and posts to them. Limits: 16 MiB per file, 64 MiB and 1024 files per bundle, 64 KiB per
/// message.
#[derive(Debug)]
pub struct PanelContent<'session> {
    session: &'session Rc<SessionInner>,
}

impl<'session> PanelContent<'session> {
    pub(crate) fn new(session: &'session Rc<SessionInner>) -> Self {
        Self { session }
    }

    /// Publishes `files` as bundle `name` (`qar_panel_content_publish_files`), replacing an
    /// earlier publication of that name. Nothing is published when it fails:
    /// `PANEL_CONTENT_BUNDLE_REFUSED`, `PANEL_CONTENT_PATH_REFUSED`,
    /// `PANEL_CONTENT_MIME_REFUSED` or `PANEL_CONTENT_SIZE_REFUSED`.
    pub fn publish_files(
        &self,
        name: &str,
        files: &[PanelContentFile],
    ) -> Result<PanelContentBundleId> {
        let name = strings::to_c_string(name, "bundle name")?;
        let paths = files
            .iter()
            .map(|file| strings::to_c_string(&file.path, "file path"))
            .collect::<Result<Vec<_>>>()?;
        let mime_types = files
            .iter()
            .map(|file| strings::optional_c_string(file.mime_type.as_deref(), "MIME type"))
            .collect::<Result<Vec<_>>>()?;
        let raw_files: Vec<sys::QarPanelContentFile> = files
            .iter()
            .zip(paths.iter().zip(&mime_types))
            .map(|(file, (path, mime_type))| sys::QarPanelContentFile {
                path: path.as_ptr(),
                mime_type: strings::pointer_or_null(mime_type.as_ref()),
                data: file.data.as_ptr(),
                size: file.data.len(),
            })
            .collect();
        let api = self.session.api();
        let mut bundle = sys::QarPanelContentBundleId {
            publisher: PeerId::default().to_raw(),
            name: [0; PANEL_CONTENT_BUNDLE_NAME_MAX + 1],
        };
        // SAFETY: live session; the files, their strings and bytes live for the call.
        let result = unsafe {
            (api.qar_impl_panel_content_publish_files)(
                self.session.handle.as_ptr(),
                name.as_ptr(),
                raw_files.as_ptr(),
                raw_files.len(),
                &raw mut bundle,
            )
        };
        check(api, "qar_panel_content_publish_files", result)?;
        Ok(PanelContentBundleId::from_raw(&bundle))
    }

    /// Publishes every regular file under `root` as bundle `name`
    /// (`qar_panel_content_publish_directory`). A link inside the folder is refused, as is every
    /// file [`PanelContent::publish_files`] refuses; a missing folder is
    /// `PANEL_CONTENT_NOT_FOUND`.
    pub fn publish_directory(&self, name: &str, root: &Path) -> Result<PanelContentBundleId> {
        let name = strings::to_c_string(name, "bundle name")?;
        let root_text = root.to_str().ok_or_else(|| {
            Error::InvalidArgument(format!("folder {} is not valid UTF-8", root.display()))
        })?;
        let root = strings::to_c_string(root_text, "folder")?;
        let api = self.session.api();
        let mut bundle = sys::QarPanelContentBundleId {
            publisher: PeerId::default().to_raw(),
            name: [0; PANEL_CONTENT_BUNDLE_NAME_MAX + 1],
        };
        // SAFETY: live session; the strings live for the call.
        let result = unsafe {
            (api.qar_impl_panel_content_publish_directory)(
                self.session.handle.as_ptr(),
                name.as_ptr(),
                root.as_ptr(),
                &raw mut bundle,
            )
        };
        check(api, "qar_panel_content_publish_directory", result)?;
        Ok(PanelContentBundleId::from_raw(&bundle))
    }

    /// Stops serving a bundle this session published (`qar_panel_content_remove`). Panels
    /// showing it stay and show nothing.
    pub fn remove(&self, bundle: &PanelContentBundleId) -> Result<()> {
        let raw = bundle.to_raw()?;
        let api = self.session.api();
        // SAFETY: live session; raw lives for the call.
        let result = unsafe {
            (api.qar_impl_panel_content_remove)(self.session.handle.as_ptr(), &raw const raw)
        };
        check(api, "qar_panel_content_remove", result)
    }

    /// Posts one JSON value to the panels showing a bundle this session published
    /// (`qar_panel_content_post_message`): to one panel or all (`None`), for one user or all.
    pub fn post_message(
        &self,
        bundle: &PanelContentBundleId,
        json: &str,
        panel: Option<GuiPanelId>,
        user: Option<PeerId>,
    ) -> Result<()> {
        let raw = bundle.to_raw()?;
        let json = strings::to_c_string(json, "message")?;
        let panel = panel.map(GuiPanelId::to_raw);
        let user = user.map(PeerId::to_raw);
        let api = self.session.api();
        // SAFETY: live session; every argument lives for the call.
        let result = unsafe {
            (api.qar_impl_panel_content_post_message)(
                self.session.handle.as_ptr(),
                &raw const raw,
                panel.as_ref().map_or(std::ptr::null(), std::ptr::from_ref),
                user.as_ref().map_or(std::ptr::null(), std::ptr::from_ref),
                json.as_ptr(),
            )
        };
        check(api, "qar_panel_content_post_message", result)
    }

    /// Calls `callback` with every message a panel showing the bundle sends
    /// (`qar_panel_content_subscribe_messages`). Runs on library threads; validate each message
    /// before acting on it and authorise per [`PanelContentMessage::user`].
    pub fn subscribe_messages(
        &self,
        bundle: &PanelContentBundleId,
        callback: impl Fn(PanelContentMessage) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let raw = bundle.to_raw()?;
        let session = self.session.handle.as_ptr();
        self.session.subscribe(
            Box::new(callback),
            "qar_panel_content_subscribe_messages",
            |api, user_state, token| {
                // SAFETY: live session; user_state stays valid until the session handle is
                // destroyed; raw lives for the call.
                unsafe {
                    (api.qar_impl_panel_content_subscribe_messages)(
                        session,
                        &raw const raw,
                        Some(panel_content_message_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }
}

unsafe extern "C" fn panel_content_message_trampoline(
    message: *const sys::QarPanelContentMessage,
    user_state: *mut c_void,
) {
    // SAFETY: registered by PanelContent::subscribe_messages with this event type.
    let state = unsafe { CallbackState::<PanelContentMessage>::from_user_state(user_state) };
    if message.is_null() {
        return;
    }
    // SAFETY: the message is valid for the duration of the callback.
    let message = unsafe { &*message };
    let json = if message.json.is_null() {
        String::new()
    } else {
        // SAFETY: json points at json_size bytes during the callback.
        let bytes =
            unsafe { std::slice::from_raw_parts(message.json.cast::<u8>(), message.json_size) };
        String::from_utf8_lossy(bytes).into_owned()
    };
    state.deliver(
        PanelContentMessage {
            panel: GuiPanelId::from_raw(message.panel),
            user: PeerId::from_raw(message.user),
            sequence: message.sequence,
            json,
        },
        "panel content message",
    );
}
