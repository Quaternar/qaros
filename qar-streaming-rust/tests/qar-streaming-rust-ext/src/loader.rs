use std::ffi::c_char;
use std::fmt;
use std::path::{Path, PathBuf};

#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

#[allow(clippy::wildcard_imports)]
use crate::ffi::*;
use crate::{Error, Result};

/// File name of the extension inside the package's `bin` folder.
pub const EXTENSION_FILE_NAME: &str = if cfg!(target_os = "windows") {
    "qar-streaming-c-ext.dll"
} else if cfg!(target_os = "macos") {
    "libqar-streaming-c-ext.dylib"
} else {
    "libqar-streaming-c-ext.so"
};

type ResultMessageFunction = unsafe extern "C" fn(QarResult, *mut c_char, usize);

const RESULT_MESSAGE_BUFFER_SIZE: usize = 4096;

macro_rules! define_extension_table {
    ($(
        pub fn $name:ident($($argument:ident : $argument_type:ty),* $(,)?) $(-> $return_type:ty)?;
    )*) => {
        /// Every `qar_impl_ext_*` symbol, in table order.
        pub const EXTENSION_SYMBOL_NAMES: &[&str] = &[$(stringify!($name)),*];

        /// The extension's functions, plus the base library's
        /// `qar_result_message` for reading the results they return. Owns
        /// the extension library; the pointers are valid while the table
        /// lives.
        ///
        /// The table also takes a reference on the base library and never
        /// releases it: once an extension hub has run, unloading the base
        /// (after `qar_library_destroy`) makes the process crash at exit
        /// (`STATUS_ACCESS_VIOLATION`, package of 2026-10-12). The base
        /// stays mapped for the rest of the process; `qaros` can still
        /// destroy and re-initialize it.
        pub struct ExtApi {
            $(pub $name: unsafe extern "C" fn($($argument_type),*) $(-> $return_type)?,)*
            result_message: ResultMessageFunction,
            _extension_library: libloading::Library,
            _base_library: std::mem::ManuallyDrop<libloading::Library>,
        }

        impl ExtApi {
            fn resolve(
                extension_library: libloading::Library,
                base_library: libloading::Library,
                path: &Path,
            ) -> Result<Self> {
                let mut missing: Vec<&'static str> = Vec::new();
                $(
                    // SAFETY: the symbol type is the extension's
                    // declaration, mirrored in `ffi`.
                    let $name = match unsafe {
                        extension_library.get::<unsafe extern "C" fn($($argument_type),*) $(-> $return_type)?>(
                            concat!(stringify!($name), "\0").as_bytes(),
                        )
                    } {
                        Ok(symbol) => Some(*symbol),
                        Err(_) => {
                            missing.push(stringify!($name));
                            None
                        }
                    };
                )*
                // SAFETY: the base library's declaration in qaros-sys.
                let result_message = unsafe {
                    base_library.get::<ResultMessageFunction>(b"qar_impl_result_message\0")
                }
                .map(|symbol| *symbol)
                .map_err(|error| Error::Load {
                    path: path.to_path_buf(),
                    message: format!("the base library lacks qar_impl_result_message: {error}"),
                })?;
                let assemble = || {
                    Some(Self {
                        $($name: $name?,)*
                        result_message,
                        _extension_library: extension_library,
                        _base_library: std::mem::ManuallyDrop::new(base_library),
                    })
                };
                assemble().ok_or_else(|| Error::Load {
                    path: path.to_path_buf(),
                    message: format!("missing symbols: {}", missing.join(", ")),
                })
            }
        }
    };
}

crate::ffi::ext_function_list!(define_extension_table);

impl ExtApi {
    /// Opens the extension from `bin_directory` and takes a reference on the
    /// base library there, which the process has already loaded.
    ///
    /// # Safety
    ///
    /// See [`crate::Extension::load`].
    pub unsafe fn load(bin_directory: &Path) -> Result<Self> {
        let base_path = bin_directory.join(LIBRARY_FILE_NAME);
        let extension_path = bin_directory.join(EXTENSION_FILE_NAME);
        let to_error = |error: LoadError, path: PathBuf| match error {
            LoadError::Open { message, .. } => Error::Load { path, message },
            other => Error::Load {
                path,
                message: other.to_string(),
            },
        };
        // SAFETY: forwarded to the caller; the base is already loaded, so
        // opening it again only adds a reference.
        let base_library = unsafe { open_package_library(&base_path) }
            .map_err(|error| to_error(error, base_path.clone()))?;
        // SAFETY: forwarded to the caller.
        let extension_library = unsafe { open_package_library(&extension_path) }
            .map_err(|error| to_error(error, extension_path.clone()))?;
        Self::resolve(extension_library, base_library, &extension_path)
    }

    /// Turns an extension result into `Ok` or an [`Error::Api`] with the
    /// library's message.
    pub fn check(&self, function: &'static str, result: QarResult) -> Result<()> {
        if result.code == QarStatusCode::QAR_STATUS_SUCCESS {
            return Ok(());
        }
        let mut buffer = vec![0 as c_char; RESULT_MESSAGE_BUFFER_SIZE];
        // SAFETY: the buffer is writable for its length.
        unsafe { (self.result_message)(result, buffer.as_mut_ptr(), buffer.len()) };
        Err(Error::Api {
            function,
            code: result.code,
            message: text_from_buffer(&buffer),
        })
    }
}

impl fmt::Debug for ExtApi {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExtApi")
            .field("symbols", &EXTENSION_SYMBOL_NAMES.len())
            .finish_non_exhaustive()
    }
}

pub(crate) fn text_from_buffer(buffer: &[c_char]) -> String {
    let bytes: Vec<u8> = buffer
        .iter()
        .take_while(|&&character| character != 0)
        .map(|&character| character.to_ne_bytes()[0])
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
