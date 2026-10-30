//! Raw bindings to the QAROS streaming C API (`qar_streaming.h`).
//!
//! The header's public `qar_*` functions are `static inline` wrappers that
//! forward to the exported `qar_impl_*` symbols, so these bindings target the
//! `qar_impl_*` symbols directly. All of them are reached through a
//! [`QarApi`] function table, which comes from one of two loading modes:
//!
//! - [`QarApi::load_default`] (default): finds `qar-streaming-c.dll` with the runtime lookup
//!   every QAROS binding follows (already loaded, beside the binding, the runtime of the QAROS
//!   instance `QAR_INSTANCE` names, the installed QAROS, `PATH`), opens it at runtime and
//!   resolves every symbol up front.
//!   [`QarApi::load`] opens one given file instead. A missing symbol is a [`LoadError`] listing
//!   all missing names, instead of the header's `abort()` at the first call.
//! - `QarApi::linked` (feature `link`): the symbols are declared
//!   `extern "C"` and resolved by the import library `qar-streaming-c.lib`;
//!   the DLL must be present at process start.
//!
//! The header's `qar_*_default()` initializers are also `static inline` and
//! absent from the binary; [`defaults`] ports them, including the
//! `QarStructureHeader` stamping every init structure needs.
//!
//! Bindings are generated per target by `cargo xtask bindgen` and committed:
//! `x86_64-pc-windows-msvc` (the runtime's platform, with the D3D11 types as
//! opaque structs), `x86_64-unknown-linux-gnu` (`libqar-streaming-c.so`), and
//! one development target without a runtime, for building and testing against
//! `qaros-stub` (`aarch64-apple-darwin`, used on every macOS host), which carries the D3D11
//! types and functions too so that the stub can emulate the D3D11 render sender.
//! `cargo xtask bindgen <target>` and a
//! `cfg` entry below add another target.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::pub_underscore_fields,
    clippy::unreadable_literal,
    clippy::too_many_arguments,
    clippy::struct_field_names,
    clippy::missing_safety_doc,
    rustdoc::invalid_html_tags
)]

#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
#[path = "generated/x86_64_pc_windows_msvc/types.rs"]
mod types;
#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
#[macro_use]
#[path = "generated/x86_64_pc_windows_msvc/functions.rs"]
mod function_list;

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
#[path = "generated/x86_64_unknown_linux_gnu/types.rs"]
mod types;
#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
#[macro_use]
#[path = "generated/x86_64_unknown_linux_gnu/functions.rs"]
mod function_list;

#[cfg(target_os = "macos")]
#[path = "generated/aarch64_apple_darwin/types.rs"]
mod types;
#[cfg(target_os = "macos")]
#[macro_use]
#[path = "generated/aarch64_apple_darwin/functions.rs"]
mod function_list;

#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"),
    all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"),
    target_os = "macos"
)))]
compile_error!(
    "qaros-sys has no committed bindings for this target; run `cargo xtask bindgen <target>` and add it to src/lib.rs"
);

pub mod defaults;
mod loader;
mod lookup;
mod platform;

pub use loader::{LoadError, QarApi, SYMBOL_NAMES, open_package_library};
pub use lookup::{
    DEFAULT_INSTANCE_NAME, INSTALL_FOLDER_VALUE, INSTALL_REGISTRY_KEY, INSTANCE_NAME_MAX_LENGTH,
    INSTANCE_RUNTIME_FOLDER_VALUE, INSTANCE_VARIABLE, INSTANCES_REGISTRY_KEY, LookupAttempt,
    LookupOutcome, LookupStep, is_valid_instance_name,
};
pub use platform::{installed_bin_folder, instance_runtime_folder, requested_instance};
pub use types::*;

/// File name of the C API library inside the binary package's `bin/`.
pub const LIBRARY_FILE_NAME: &str = if cfg!(target_os = "windows") {
    "qar-streaming-c.dll"
} else if cfg!(target_os = "macos") {
    "libqar-streaming-c.dylib"
} else {
    "libqar-streaming-c.so"
};
