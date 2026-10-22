//! The library is never unloaded, and a failed `qar_library_destroy` keeps the process slot
//! reserved for good, so this runs in a process of its own. Until the first drop, the
//! `Library` is the only holder of the stub, so the stub's call log surviving that drop shows
//! the DLL was not unmapped. macOS dyld keeps the stub mapped after its last `dlclose` anyway,
//! so there that check holds either way; it can only catch an unload where the loader honours
//! one.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use qaros::{Error, Library};

#[allow(clippy::wildcard_imports)]
use support::*;

fn count_of(calls: &[String], entry: &str) -> usize {
    calls.iter().filter(|call| *call == entry).count()
}

#[test]
fn the_library_stays_mapped_and_a_failed_destroy_keeps_the_slot() {
    let first = load_stub();
    drop(first);

    let control = StubControl::open();
    assert_eq!(
        control.calls(),
        vec!["library_init".to_string(), "library_destroy".to_string()],
        "the stub's state must survive the drop: the DLL stays mapped"
    );

    let second = load_stub();
    assert_eq!(count_of(&control.calls(), "library_init"), 2);
    control.fail_next("library_destroy", 7);
    drop(second);
    assert_eq!(count_of(&control.calls(), "library_destroy"), 2);

    // SAFETY: the stub matches the C header by construction.
    let third = unsafe { Library::load(stub_path(), &quiet()) };
    assert!(
        matches!(third, Err(Error::LibraryAlreadyLoaded)),
        "a failed destroy keeps the slot reserved: {third:?}"
    );
    assert_eq!(count_of(&control.calls(), "library_init"), 2);
}
