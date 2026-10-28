use std::ffi::{OsStr, c_void};
use std::fmt;
use std::path::{Path, PathBuf};

use crate::lookup::{LookupAttempt, LookupStep, find_runtime};
use crate::platform::{SystemLocator, module_path_containing, open_library_file};
#[allow(clippy::wildcard_imports)]
use crate::types::*;

/// Why [`QarApi::load`] failed.
#[derive(Debug)]
pub enum LoadError {
    /// The operating system could not open the library (missing file,
    /// missing dependent DLL, wrong architecture).
    Open {
        /// The path given to the loader.
        path: PathBuf,
        /// The operating system's reason.
        message: String,
    },
    /// The library opened but lacks symbols this binding needs. All missing
    /// names are listed; nothing was loaded.
    MissingSymbols {
        /// The path given to the loader.
        path: PathBuf,
        /// Every required symbol the library does not export.
        symbols: Vec<&'static str>,
    },
    /// The default lookup ([`QarApi::load_default`]) found no runtime it could load. The
    /// attempts name every place looked at, in lookup order; a set `QAR_INSTANCE` that names no
    /// valid or known instance, or an instance or installed QAROS whose runtime is missing or
    /// fails to load, ends the lookup there.
    NotFound {
        /// Each place looked at and what was met there.
        attempts: Vec<LookupAttempt>,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open { path, message } => {
                write!(formatter, "cannot open {}: {message}", path.display())
            }
            Self::MissingSymbols { path, symbols } => write!(
                formatter,
                "{} lacks {} of {} required symbols: {}",
                path.display(),
                symbols.len(),
                SYMBOL_NAMES.len(),
                symbols.join(", ")
            ),
            Self::NotFound { attempts } => {
                write!(
                    formatter,
                    "no loadable QAROS runtime ({}); looked at:",
                    crate::LIBRARY_FILE_NAME
                )?;
                for attempt in attempts {
                    write!(formatter, "\n  - {attempt}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for LoadError {}

macro_rules! define_api_table {
    ($(
        pub fn $name:ident($($argument:ident : $argument_type:ty),* $(,)?) $(-> $return_type:ty)?;
    )*) => {
        /// Every `qar_impl_*` symbol this binding requires, in table order.
        pub const SYMBOL_NAMES: &[&str] = &[$(stringify!($name)),*];

        /// Table of the exported `qar_impl_*` functions.
        ///
        /// In runtime-loading mode the table owns the opened library, which
        /// stays loaded until the table is dropped. The function pointers are
        /// valid only while the table is alive: do not copy one out of it.
        pub struct QarApi {
            $(pub $name: unsafe extern "C" fn($($argument_type),*) $(-> $return_type)?,)*
            library: Option<libloading::Library>,
            library_path: Option<PathBuf>,
            lookup_step: Option<LookupStep>,
        }

        impl QarApi {
            fn resolve(library: libloading::Library, path: &Path) -> Result<Self, LoadError> {
                let mut missing: Vec<&'static str> = Vec::new();
                $(
                    // SAFETY: the symbol type is the declaration generated from
                    // qar_streaming.h for this target.
                    let $name = match unsafe {
                        library.get::<unsafe extern "C" fn($($argument_type),*) $(-> $return_type)?>(
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
                let assemble = || Some(Self { $($name: $name?,)* library: None, library_path: None, lookup_step: None });
                match assemble() {
                    Some(table) if missing.is_empty() => {
                        let library_path =
                            module_path_containing(table.qar_impl_library_init as *const c_void);
                        Ok(Self { library: Some(library), library_path, ..table })
                    }
                    _ => Err(LoadError::MissingSymbols { path: path.to_path_buf(), symbols: missing }),
                }
            }

            /// Builds the table from the import-library declarations.
            #[cfg(feature = "link")]
            pub fn linked() -> Self {
                Self { $($name: linked::$name,)* library: None, library_path: None, lookup_step: None }
            }
        }

        #[cfg(feature = "link")]
        mod linked {
            #[allow(clippy::wildcard_imports)]
            use crate::types::*;

            #[link(name = "qar-streaming-c")]
            unsafe extern "C" {
                $(pub fn $name($($argument: $argument_type),*) $(-> $return_type)?;)*
            }
        }
    };
}

qar_impl_function_list!(define_api_table);

impl QarApi {
    /// Opens the C API library file at `path` and resolves every `qar_impl_*` symbol. Only that
    /// file is tried, from its absolute path (a relative path is taken from the current
    /// directory), with its dependencies resolved from its own folder and System32. Either all
    /// symbols resolve or none are kept.
    ///
    /// Explicit paths are for development builds of QAROS itself; a distributed application uses
    /// [`QarApi::load_default`].
    ///
    /// # Safety
    ///
    /// `path` must name a `qar-streaming-c` build whose exports match these bindings. Opening a
    /// library runs its initialization code, and a library exporting these names with different
    /// signatures causes undefined behaviour on the first call.
    pub unsafe fn load(path: impl AsRef<OsStr>) -> Result<Self, LoadError> {
        let path = Path::new(path.as_ref());
        // SAFETY: forwarded to the caller.
        let library = unsafe { open_package_library(path) }?;
        Self::resolve(library, path)
    }

    /// Finds the runtime with the default lookup every QAROS binding follows and resolves every
    /// `qar_impl_*` symbol. The first match wins:
    ///
    /// 0. a `qar-streaming-c` already loaded in the process (a reference is taken);
    /// 1. `qar-streaming-c.dll` beside the binding: in the folder of the executable or library
    ///    this crate is compiled into;
    /// 2. when `QAR_INSTANCE` names a QAROS instance (Windows only), that instance's runtime
    ///    folder: `default` is the installed QAROS, any other name the `RuntimeFolder` recorded
    ///    under `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>` (a QAROS development hub
    ///    records its build output there and sets `QAR_INSTANCE` for the apps it starts), else
    ///    that record's `<InstallFolder>\bin`. When the variable is set it is the only copy
    ///    considered: an invalid name, an unknown instance, or a runtime missing there or failing
    ///    to load is an error, never a reason to load another instance's runtime;
    /// 3. the installed QAROS, `<InstallFolder>\bin`, from `HKLM\SOFTWARE\Quaternar\QAROS`
    ///    (Windows); an installed runtime missing or failing to load is an error;
    /// 4. each folder on `PATH`, in order (elsewhere: the dynamic linker's search).
    ///
    /// The current directory is never searched, and the process's search path is not changed.
    /// Every candidate loads from its absolute path, its dependencies from its own folder and
    /// System32. With no runtime found, [`LoadError::NotFound`] names every place looked at.
    /// [`QarApi::library_path`] tells which file was loaded.
    ///
    /// # Safety
    ///
    /// The runtime found must be a `qar-streaming-c` build whose exports match these bindings;
    /// see [`QarApi::load`].
    pub unsafe fn load_default() -> Result<Self, LoadError> {
        let found = find_runtime(&SystemLocator, crate::LIBRARY_FILE_NAME)
            .map_err(|attempts| LoadError::NotFound { attempts })?;
        let table = Self::resolve(found.library, &found.path)?;
        Ok(Self {
            lookup_step: Some(found.step),
            ..table
        })
    }

    /// Whether this table owns a library opened by [`QarApi::load`] or
    /// [`QarApi::load_default`].
    pub fn is_runtime_loaded(&self) -> bool {
        self.library.is_some()
    }

    /// The file of the library this table was loaded from, for diagnostics. `None` for an
    /// import-linked table or when the operating system does not name the module.
    pub fn library_path(&self) -> Option<&Path> {
        self.library_path.as_deref()
    }

    /// The step of the default lookup that found the runtime; `None` when it was not loaded by
    /// [`QarApi::load_default`].
    pub fn lookup_step(&self) -> Option<LookupStep> {
        self.lookup_step
    }
}

/// Opens a library file with the search rules [`QarApi::load`] uses: from its absolute path, its
/// dependencies from its own folder and System32. For other libraries of a QAROS build, such as
/// the test-only counterparts library.
///
/// # Safety
///
/// Opening a library runs its initialization code.
pub unsafe fn open_package_library(path: &Path) -> Result<libloading::Library, LoadError> {
    // SAFETY: forwarded to the caller.
    unsafe { open_library_file(path) }.map_err(|message| LoadError::Open {
        path: path.to_path_buf(),
        message,
    })
}

impl fmt::Debug for QarApi {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QarApi")
            .field("symbols", &SYMBOL_NAMES.len())
            .field("runtime_loaded", &self.library.is_some())
            .field("library_path", &self.library_path)
            .field("lookup_step", &self.lookup_step)
            .finish_non_exhaustive()
    }
}
