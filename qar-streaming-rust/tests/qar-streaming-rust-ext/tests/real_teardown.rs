//! Process exit after a full teardown: the base library, the extension, a
//! runtime and an extension hub are set up and destroyed, and the process
//! must then exit cleanly. The verdict is the process exit code, so run this
//! binary on its own.

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

#[test]
fn the_process_exits_cleanly_after_teardown() {
    let Some(bin) = common::package_bin() else {
        return;
    };
    let loaded = common::load(&bin, "teardown");
    let runtime = common::create_runtime(&loaded.library, &bin, &loaded.scratch, "teardown");
    let hub = loaded.extension.create_hub(false).expect("create a hub");
    drop(hub);
    drop(loaded.extension);
    drop(runtime);
    drop(loaded.library);
    eprintln!("teardown complete");
}
