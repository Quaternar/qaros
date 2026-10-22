//! `Library::load_default` against a real library: a copy of `qaros-stub` under the runtime's
//! file name. It sets `QAROS_RUNTIME_DIR` and keeps the library mapped for the rest of the
//! process, so it is the only test in its binary. The order of the lookup's steps is tested
//! without a real library in `qaros-sys` (`lookup.rs`).

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use std::path::{Path, PathBuf};

use qaros::qaros_sys::{LIBRARY_FILE_NAME, LoadError, LookupOutcome, LookupStep};
use qaros::{Error, Library};

#[allow(clippy::wildcard_imports)]
use support::*;

fn set_runtime_dir(folder: &Path) {
    // SAFETY: the only test of this binary; no other thread reads the environment meanwhile.
    unsafe { std::env::set_var("QAROS_RUNTIME_DIR", folder) };
}

fn canonical(path: &Path) -> PathBuf {
    std::fs::canonicalize(path)
        .unwrap_or_else(|error| panic!("cannot resolve {}: {error}", path.display()))
}

#[test]
fn the_variable_names_the_runtime_and_a_loaded_runtime_is_found_first() {
    let scratch = std::env::temp_dir().join(format!("qaros-default-lookup-{}", std::process::id()));
    let empty_folder = scratch.join("empty");
    let runtime_folder = scratch.join("runtime");
    std::fs::create_dir_all(&empty_folder).expect("empty folder");
    std::fs::create_dir_all(&runtime_folder).expect("runtime folder");
    let runtime_copy = runtime_folder.join(LIBRARY_FILE_NAME);
    std::fs::copy(stub_path(), &runtime_copy).expect("copy the stub as the runtime");

    // Nothing loaded, nothing beside the test binary, and the variable names a folder without
    // the runtime: the lookup stops there and names each place.
    set_runtime_dir(&empty_folder);
    // SAFETY: nothing is found, so nothing is loaded.
    let error = unsafe { Library::load_default(&quiet()) }.expect_err("no runtime there");
    let Error::Load(LoadError::NotFound { attempts }) = &error else {
        panic!("expected NotFound, got {error}");
    };
    let steps: Vec<LookupStep> = attempts.iter().map(|attempt| attempt.step).collect();
    assert_eq!(
        steps,
        vec![
            LookupStep::AlreadyLoaded,
            LookupStep::BesideBinding,
            LookupStep::RuntimeDirVariable
        ]
    );
    assert_eq!(
        attempts[2].outcome,
        LookupOutcome::Missing {
            path: empty_folder.join(LIBRARY_FILE_NAME)
        }
    );
    let message = error.to_string();
    assert!(
        message.contains(&format!(
            "QAROS_RUNTIME_DIR: {} does not exist",
            empty_folder.join(LIBRARY_FILE_NAME).display()
        )),
        "{message}"
    );

    // The variable names the folder with the runtime.
    set_runtime_dir(&runtime_folder);
    // SAFETY: the copy is qaros-stub, which matches these bindings by construction.
    let library = unsafe { Library::load_default(&quiet()) }.expect("load from QAROS_RUNTIME_DIR");
    let loaded_from = library
        .library_path()
        .expect("the loaded file")
        .to_path_buf();
    assert_eq!(canonical(&loaded_from), canonical(&runtime_copy));
    assert_eq!(library.lookup_step(), Some(LookupStep::RuntimeDirVariable));
    drop(library);

    reuse_of_the_mapped_runtime(&empty_folder, &runtime_copy);

    if let Err(error) = std::fs::remove_dir_all(&empty_folder) {
        eprintln!("cannot remove {}: {error}", empty_folder.display());
    }
}

/// The runtime stays mapped after the drop, so step 0 finds it before the variable's folder.
/// Windows matches a loaded module by its file name. Elsewhere the loader matches the library's
/// own name (install name or soname), which for the copy is still the stub's, so this part runs
/// on Windows only.
#[cfg(windows)]
fn reuse_of_the_mapped_runtime(empty_folder: &Path, runtime_copy: &Path) {
    set_runtime_dir(empty_folder);
    // SAFETY: as above; the mapped copy is reused.
    let library = unsafe { Library::load_default(&quiet()) }.expect("the runtime already loaded");
    let loaded_from = library
        .library_path()
        .expect("the loaded file")
        .to_path_buf();
    assert_eq!(canonical(&loaded_from), canonical(runtime_copy));
    assert_eq!(library.lookup_step(), Some(LookupStep::AlreadyLoaded));
    drop(library);
}

#[cfg(not(windows))]
fn reuse_of_the_mapped_runtime(_empty_folder: &Path, _runtime_copy: &Path) {}
