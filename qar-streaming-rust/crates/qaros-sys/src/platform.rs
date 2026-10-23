//! What the loader needs from the operating system: opening a library with the runtime's
//! search rules, the module an address belongs to, and the places of the default lookup.

use std::ffi::c_void;
use std::path::{Path, PathBuf};

use crate::lookup::{RUNTIME_DIR_VARIABLE, RuntimeLocator, SearchPath};

/// The folder `QAROS_RUNTIME_DIR` names, when it is set and not empty.
pub fn runtime_dir_from_environment() -> Option<PathBuf> {
    std::env::var_os(RUNTIME_DIR_VARIABLE)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

/// `<InstallFolder>\bin` of the QAROS installed on this PC, read from
/// `HKLM\SOFTWARE\Quaternar\QAROS`. `None` when QAROS is not installed, and always outside
/// Windows.
pub fn installed_bin_folder() -> Option<PathBuf> {
    os::install_folder().map(|folder| folder.join("bin"))
}

/// Opens the library file at `path` the way every QAROS binding does: from its absolute path,
/// with its dependencies resolved from its own folder and System32 only, never from the
/// application's folder or the current directory.
///
/// # Safety
///
/// Opening a library runs its initialization code.
pub(crate) unsafe fn open_library_file(path: &Path) -> Result<libloading::Library, String> {
    let absolute_path = std::path::absolute(path)
        .map_err(|error| format!("cannot make {} absolute: {error}", path.display()))?;
    // SAFETY: forwarded to the caller.
    unsafe { os::open_with_runtime_search_rules(&absolute_path) }
        .map_err(|error| describe_open_error(&error))
}

/// libloading's message for a failed open omits the operating system's reason (for example a
/// missing dependent DLL), which sits in its source.
fn describe_open_error(error: &libloading::Error) -> String {
    match std::error::Error::source(error) {
        Some(reason) => format!("{error}: {reason}"),
        None => error.to_string(),
    }
}

/// The file of the module (executable or library) that contains `address`.
pub(crate) fn module_path_containing(address: *const c_void) -> Option<PathBuf> {
    os::module_path_containing(address)
}

/// The real process, environment and file system.
pub(crate) struct SystemLocator;

impl RuntimeLocator for SystemLocator {
    type Library = libloading::Library;

    fn already_loaded(&self) -> Option<libloading::Library> {
        os::open_already_loaded(crate::LIBRARY_FILE_NAME)
    }

    fn binding_folder(&self) -> Option<PathBuf> {
        let address_in_this_module = module_path_containing as *const c_void;
        module_path_containing(address_in_this_module)?
            .parent()
            .map(Path::to_path_buf)
    }

    fn runtime_dir_variable(&self) -> Option<PathBuf> {
        runtime_dir_from_environment()
    }

    fn installed_bin_folder(&self) -> Option<PathBuf> {
        installed_bin_folder()
    }

    fn search_path(&self) -> SearchPath {
        if cfg!(windows) {
            SearchPath::Folders(
                std::env::var_os("PATH")
                    .map(|path_value| crate::lookup::search_path_folders(&path_value))
                    .unwrap_or_default(),
            )
        } else {
            SearchPath::DynamicLinker
        }
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn load(&self, path: &Path) -> Result<libloading::Library, String> {
        // SAFETY: the default lookup is `unsafe` for its caller, who accepts that the runtime
        // found runs its initialization code.
        unsafe { open_library_file(path) }
    }

    fn load_by_linker_search(&self, file_name: &str) -> Result<libloading::Library, String> {
        // SAFETY: as `load`.
        unsafe { os::open_by_linker_search(file_name) }
    }
}

#[cfg(windows)]
mod os {
    use std::ffi::{OsStr, OsString, c_void};
    use std::os::windows::ffi::{OsStrExt as _, OsStringExt as _};
    use std::path::{Path, PathBuf};

    use crate::lookup::{INSTALL_FOLDER_VALUE, INSTALL_REGISTRY_KEY};

    type ModuleHandle = *mut c_void;
    type RegistryKey = isize;

    const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x0000_0002;
    const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x0000_0004;
    #[allow(clippy::cast_possible_wrap)]
    const HKEY_LOCAL_MACHINE: RegistryKey = 0x8000_0002_u32 as i32 as RegistryKey;
    const RRF_RT_REG_SZ: u32 = 0x0000_0002;
    const ERROR_SUCCESS: i32 = 0;
    /// The longest path the wide Windows API accepts.
    const LONGEST_PATH: usize = 32_768;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleExW(
            flags: u32,
            module_name: *const u16,
            module: *mut ModuleHandle,
        ) -> i32;
        fn GetModuleFileNameW(module: ModuleHandle, file_name: *mut u16, size: u32) -> u32;
    }

    #[link(name = "advapi32")]
    unsafe extern "system" {
        fn RegGetValueW(
            key: RegistryKey,
            sub_key: *const u16,
            value: *const u16,
            flags: u32,
            value_type: *mut u32,
            data: *mut c_void,
            data_size: *mut u32,
        ) -> i32;
    }

    fn wide(text: &str) -> Vec<u16> {
        OsStr::new(text).encode_wide().chain(Some(0)).collect()
    }

    pub(super) unsafe fn open_with_runtime_search_rules(
        absolute_path: &Path,
    ) -> Result<libloading::Library, libloading::Error> {
        use libloading::os::windows::{
            LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LOAD_LIBRARY_SEARCH_SYSTEM32,
        };
        // SAFETY: forwarded to the caller.
        let library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                absolute_path,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        }?;
        Ok(library.into())
    }

    /// GetModuleHandleExW without flags takes a reference, which keeps unloading balanced.
    pub(super) fn open_already_loaded(file_name: &str) -> Option<libloading::Library> {
        libloading::os::windows::Library::open_already_loaded(file_name)
            .ok()
            .map(Into::into)
    }

    /// Never reached: on Windows step 4 searches the folders on `PATH`.
    pub(super) unsafe fn open_by_linker_search(
        file_name: &str,
    ) -> Result<libloading::Library, String> {
        Err(format!(
            "{file_name}: Windows searches the folders on PATH instead"
        ))
    }

    pub(super) fn module_path_containing(address: *const c_void) -> Option<PathBuf> {
        let mut module: ModuleHandle = std::ptr::null_mut();
        // SAFETY: with FROM_ADDRESS the name argument is an address inside a loaded module;
        // UNCHANGED_REFCOUNT takes no reference, so nothing needs releasing.
        let found = unsafe {
            GetModuleHandleExW(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                    | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                address.cast(),
                &raw mut module,
            )
        };
        if found == 0 {
            return None;
        }
        let mut capacity = 260;
        while capacity <= LONGEST_PATH {
            let mut buffer = vec![0_u16; capacity];
            let capacity_u32 = u32::try_from(capacity).ok()?;
            // SAFETY: the buffer is writable for `capacity` characters.
            let length = unsafe { GetModuleFileNameW(module, buffer.as_mut_ptr(), capacity_u32) };
            if length == 0 {
                return None;
            }
            if length < capacity_u32 {
                buffer.truncate(length as usize);
                return Some(PathBuf::from(OsString::from_wide(&buffer)));
            }
            capacity *= 2;
        }
        None
    }

    pub(super) fn install_folder() -> Option<PathBuf> {
        let key = wide(INSTALL_REGISTRY_KEY);
        let value = wide(INSTALL_FOLDER_VALUE);
        let mut size_in_bytes: u32 = 0;
        // SAFETY: a size query: no data pointer, a valid size out-pointer.
        let status = unsafe {
            RegGetValueW(
                HKEY_LOCAL_MACHINE,
                key.as_ptr(),
                value.as_ptr(),
                RRF_RT_REG_SZ,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut size_in_bytes,
            )
        };
        if status != ERROR_SUCCESS || size_in_bytes == 0 {
            return None;
        }
        let mut buffer = vec![0_u16; (size_in_bytes as usize).div_ceil(2)];
        // SAFETY: the buffer holds `size_in_bytes` bytes.
        let status = unsafe {
            RegGetValueW(
                HKEY_LOCAL_MACHINE,
                key.as_ptr(),
                value.as_ptr(),
                RRF_RT_REG_SZ,
                std::ptr::null_mut(),
                buffer.as_mut_ptr().cast(),
                &raw mut size_in_bytes,
            )
        };
        if status != ERROR_SUCCESS {
            return None;
        }
        let length = buffer
            .iter()
            .position(|&character| character == 0)
            .unwrap_or(buffer.len());
        (length > 0).then(|| PathBuf::from(OsString::from_wide(&buffer[..length])))
    }
}

#[cfg(not(windows))]
mod os {
    use std::ffi::{CStr, OsStr, c_char, c_int, c_void};
    use std::os::unix::ffi::OsStrExt as _;
    use std::path::{Path, PathBuf};

    use libloading::os::unix::{RTLD_LOCAL, RTLD_NOW};

    #[cfg(target_os = "macos")]
    const RTLD_NOLOAD: c_int = 0x10;
    #[cfg(not(target_os = "macos"))]
    const RTLD_NOLOAD: c_int = 0x4;

    #[repr(C)]
    struct DynamicLinkInfo {
        file_name: *const c_char,
        base_address: *mut c_void,
        symbol_name: *const c_char,
        symbol_address: *mut c_void,
    }

    unsafe extern "C" {
        fn dladdr(address: *const c_void, info: *mut DynamicLinkInfo) -> c_int;
    }

    pub(super) unsafe fn open_with_runtime_search_rules(
        absolute_path: &Path,
    ) -> Result<libloading::Library, libloading::Error> {
        // SAFETY: forwarded to the caller.
        let library = unsafe {
            libloading::os::unix::Library::open(Some(absolute_path), RTLD_NOW | RTLD_LOCAL)
        }?;
        Ok(library.into())
    }

    pub(super) fn install_folder() -> Option<PathBuf> {
        None
    }

    pub(super) fn open_already_loaded(file_name: &str) -> Option<libloading::Library> {
        // SAFETY: RTLD_NOLOAD opens nothing new; it only takes a reference on a loaded library.
        unsafe {
            libloading::os::unix::Library::open(
                Some(file_name),
                RTLD_NOW | RTLD_NOLOAD | RTLD_LOCAL,
            )
        }
        .ok()
        .map(Into::into)
    }

    pub(super) unsafe fn open_by_linker_search(
        file_name: &str,
    ) -> Result<libloading::Library, String> {
        // SAFETY: forwarded to the caller.
        unsafe { libloading::os::unix::Library::open(Some(file_name), RTLD_NOW | RTLD_LOCAL) }
            .map(Into::into)
            .map_err(|error| error.to_string())
    }

    pub(super) fn module_path_containing(address: *const c_void) -> Option<PathBuf> {
        let mut info = DynamicLinkInfo {
            file_name: std::ptr::null(),
            base_address: std::ptr::null_mut(),
            symbol_name: std::ptr::null(),
            symbol_address: std::ptr::null_mut(),
        };
        // SAFETY: a valid out-pointer; dladdr only reads the address.
        let found = unsafe { dladdr(address, &raw mut info) };
        if found == 0 || info.file_name.is_null() {
            return None;
        }
        // SAFETY: dladdr returned a NUL-terminated name that stays valid while the module is
        // loaded, which it is, since `address` lies in it.
        let file_name = unsafe { CStr::from_ptr(info.file_name) };
        let path = PathBuf::from(OsStr::from_bytes(file_name.to_bytes()));
        // The main executable can be reported by the relative name it was started with.
        if path.is_absolute() {
            Some(path)
        } else {
            std::env::current_exe().ok()
        }
    }
}
