use std::cell::Cell;
use std::fmt;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use qaros_sys::{QarCancelToken, QarResult, QarStatusCode};

use crate::error::{Error, Result, StatusCode, check};
use crate::library::{LibraryInner, require_handle};

/// A `QarCancelToken`.
///
/// Pass it to blocking calls (through [`crate::BlockingOptions`]) and cancel
/// it from a callback, or create it with a timeout. The token itself is not
/// `Send` because it shares the library handle; to cancel from another thread
/// (a Ctrl+C handler, a watcher of stdin), hand that thread a
/// [`CancelToken::cancel_handle`].
#[derive(Clone, Debug)]
pub struct CancelToken {
    pub(crate) inner: Rc<CancelTokenInner>,
}

#[derive(Debug)]
pub(crate) struct CancelTokenInner {
    pub(crate) handle: NonNull<QarCancelToken>,
    library: Rc<LibraryInner>,
    destroy_at_library_teardown: Cell<bool>,
    cross_thread: Arc<CrossThreadToken>,
}

type CancelFunction = unsafe extern "C" fn(*mut QarCancelToken) -> QarResult;

/// The token as other threads see it: the handle while its owner lives, `None` after.
struct CrossThreadToken {
    handle: Mutex<Option<TokenPointer>>,
    cancel: CancelFunction,
}

struct TokenPointer(NonNull<QarCancelToken>);

// SAFETY: the C API allows `qar_cancel_token_cancel` from any thread; the pointer is only used
// under the mutex, which the owner takes before it destroys the handle.
unsafe impl Send for TokenPointer {}

impl fmt::Debug for CrossThreadToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CrossThreadToken")
            .finish_non_exhaustive()
    }
}

/// Cancels a [`CancelToken`] from any thread (`qar_cancel_token_cancel`). It does not keep the
/// token alive: once the token's last owner is gone, [`CancelHandle::cancel`] does nothing and
/// returns `Ok(false)`.
#[derive(Clone, Debug)]
pub struct CancelHandle {
    token: Arc<CrossThreadToken>,
}

impl CancelHandle {
    /// Cancels every call that was given the token. `Ok(true)` when the token was cancelled,
    /// `Ok(false)` when its owner had already dropped it.
    pub fn cancel(&self) -> Result<bool> {
        let handle = self
            .token
            .handle
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let Some(token) = handle.as_ref() else {
            return Ok(false);
        };
        // SAFETY: the owner destroys the handle only after taking this lock and clearing it.
        let result = unsafe { (self.token.cancel)(token.0.as_ptr()) };
        if result.code == QarStatusCode::QAR_STATUS_SUCCESS {
            Ok(true)
        } else {
            Err(Error::Api {
                function: "qar_cancel_token_cancel",
                code: StatusCode::from_raw(result.code),
                message: String::new(),
                causes: Vec::new(),
            })
        }
    }
}

impl CancelToken {
    pub(crate) fn create(library: &Rc<LibraryInner>, timeout: Option<Duration>) -> Result<Self> {
        Ok(Self {
            inner: Rc::new(CancelTokenInner::create(library, timeout)?),
        })
    }

    /// Cancels every call that was given this token (`qar_cancel_token_cancel`).
    pub fn cancel(&self) -> Result<()> {
        self.inner.cancel()
    }

    /// Whether [`Self::cancel`] has been called on this token.
    pub fn is_cancelled(&self) -> bool {
        let api = &self.inner.library.api;
        // SAFETY: the handle is live until CancelTokenInner drops.
        unsafe { (api.qar_impl_cancel_token_is_cancelled)(self.inner.handle.as_ptr()) }
    }

    /// Whether the token's timeout has passed.
    pub fn is_timed_out(&self) -> bool {
        let api = &self.inner.library.api;
        // SAFETY: the handle is live until CancelTokenInner drops.
        unsafe { (api.qar_impl_cancel_token_is_timeout)(self.inner.handle.as_ptr()) }
    }

    /// A handle that cancels this token from any thread, for as long as the token lives.
    pub fn cancel_handle(&self) -> CancelHandle {
        CancelHandle {
            token: Arc::clone(&self.inner.cross_thread),
        }
    }

    /// The `QarCancelToken` handle, for a raw call (through [`crate::Library::raw_api`]) that
    /// takes a cancel token, or for `qar_cancel_token_cancel_after`.
    ///
    /// Taking the pointer is safe; using it is `unsafe`, under the rules of
    /// [`crate::Session::as_raw`] for a token: valid while this `CancelToken` (or a clone of it)
    /// lives, never destroyed by the caller, used on the thread that owns the token. To cancel
    /// from another thread, use [`CancelToken::cancel_handle`].
    pub fn as_raw(&self) -> *mut QarCancelToken {
        self.inner.handle.as_ptr()
    }

    pub(crate) fn as_ptr(&self) -> *mut QarCancelToken {
        self.inner.handle.as_ptr()
    }
}

impl CancelTokenInner {
    pub(crate) fn create(library: &Rc<LibraryInner>, timeout: Option<Duration>) -> Result<Self> {
        let api = &library.api;
        let mut handle: *mut QarCancelToken = ptr::null_mut();
        let (function, result) = match timeout {
            None => (
                "qar_cancel_token_create",
                // SAFETY: handle is a valid out-pointer.
                unsafe { (api.qar_impl_cancel_token_create)(&raw mut handle) },
            ),
            Some(duration) => {
                let milliseconds = u32::try_from(duration.as_millis()).map_err(|_| {
                    Error::InvalidArgument("cancel timeout exceeds u32 milliseconds".to_string())
                })?;
                (
                    "qar_cancel_token_create_with_timeout",
                    // SAFETY: handle is a valid out-pointer.
                    unsafe {
                        (api.qar_impl_cancel_token_create_with_timeout)(
                            &raw mut handle,
                            milliseconds,
                        )
                    },
                )
            }
        };
        check(api, function, result)?;
        let handle = require_handle(handle, function)?;
        Ok(Self {
            handle,
            library: Rc::clone(library),
            destroy_at_library_teardown: Cell::new(false),
            cross_thread: Arc::new(CrossThreadToken {
                handle: Mutex::new(Some(TokenPointer(handle))),
                cancel: api.qar_impl_cancel_token_cancel,
            }),
        })
    }

    pub(crate) fn cancel(&self) -> Result<()> {
        let api = &self.library.api;
        // SAFETY: the handle is live until this value drops.
        let result = unsafe { (api.qar_impl_cancel_token_cancel)(self.handle.as_ptr()) };
        check(api, "qar_cancel_token_cancel", result)
    }
}

impl CancelTokenInner {
    /// For an operation that may still use the token after its last owner
    /// is gone: the handle is then destroyed right before the library.
    pub(crate) fn destroy_at_library_teardown(&self) {
        self.destroy_at_library_teardown.set(true);
    }
}

impl Drop for CancelTokenInner {
    fn drop(&mut self) {
        // From here on no other thread reaches the handle.
        self.cross_thread
            .handle
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if self.destroy_at_library_teardown.get() {
            self.library
                .tokens_destroyed_at_teardown
                .borrow_mut()
                .push(self.handle);
            return;
        }
        // SAFETY: last owner; no call using the token is in progress because
        // blocking calls borrow the token and subscriptions keep their own
        // reference until the session handle is destroyed.
        unsafe { (self.library.api.qar_impl_cancel_token_handle_destroy)(self.handle.as_ptr()) };
    }
}
