//! Port of `dynamic_loading.c`: load `qar-streaming-c` at runtime,
//! initialize the library and tear it down.
//!
//! ```text
//! cargo run -p qaros-examples --bin dynamic_loading [-- --qaros-library=PATH]
//! ```
//!
//! Without a path it loads the runtime the default lookup finds (the C sample's `installed`).

use std::process::ExitCode;

use qaros_examples::{ExampleResult, load_library, report};

fn main() -> ExitCode {
    report(run())
}

fn run() -> ExampleResult {
    // ANCHOR: load
    let library = load_library()?;
    println!(
        "Library initialized with {} symbols. Runtime binaries directory: (folder of the loaded library)",
        qaros::qaros_sys::SYMBOL_NAMES.len(),
    );
    // ANCHOR_END: load
    // ANCHOR: runtime_info
    let runtime_info = library.runtime_info()?;
    println!(
        "Runtime {} ({}, C API {})",
        runtime_info.version, runtime_info.flavor, runtime_info.c_api
    );
    // ANCHOR_END: runtime_info
    // ANCHOR: teardown
    drop(library);
    println!(
        "Library destroyed; qar-streaming-c stays loaded until the process exits. Dynamic loading tutorial complete."
    );
    // ANCHOR_END: teardown
    Ok(())
}
