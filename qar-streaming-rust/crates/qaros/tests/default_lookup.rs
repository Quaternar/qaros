//! `Library::load_default` against a real library: a copy of `qaros-stub` under the runtime's
//! file name. On Windows it sets `QAR_INSTANCE` to an instance whose per-user record
//! (`HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>`, as a development hub writes it) names the
//! runtime folder; the name is unique to the run and the record is removed afterwards. It keeps
//! the library mapped for the rest of the process, so it is the only test in its binary. The
//! order of the lookup's steps is tested without a real library in `qaros-sys` (`lookup.rs`).

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use qaros::qaros_sys::{LoadError, LookupOutcome, LookupStep};
use qaros::{Error, Library};

#[allow(clippy::wildcard_imports)]
use support::*;

fn set_instance(instance: &str) {
    // SAFETY: the only test of this binary; no other thread reads the environment meanwhile.
    unsafe { std::env::set_var("QAR_INSTANCE", instance) };
}

/// The lookup's attempts, after checking that step 2 ended it.
#[cfg(windows)]
fn attempts_ending_at_the_instance(error: &Error) -> &[qaros::qaros_sys::LookupAttempt] {
    let Error::Load(LoadError::NotFound { attempts }) = error else {
        panic!("expected NotFound, got {error}");
    };
    let steps: Vec<LookupStep> = attempts.iter().map(|attempt| attempt.step).collect();
    assert_eq!(
        steps,
        vec![
            LookupStep::AlreadyLoaded,
            LookupStep::BesideBinding,
            LookupStep::Instance
        ],
        "a set QAR_INSTANCE ends the lookup at step 2: {error}"
    );
    attempts
}

#[cfg(windows)]
#[test]
fn qar_instance_names_the_runtime_and_a_loaded_runtime_is_found_first() {
    use qaros::qaros_sys::LIBRARY_FILE_NAME;

    let scratch = std::env::temp_dir().join(format!("qaros-default-lookup-{}", std::process::id()));
    let empty_folder = scratch.join("empty");
    let runtime_folder = scratch.join("runtime");
    std::fs::create_dir_all(&empty_folder).expect("empty folder");
    std::fs::create_dir_all(&runtime_folder).expect("runtime folder");
    let runtime_copy = runtime_folder.join(LIBRARY_FILE_NAME);
    std::fs::copy(stub_path(), &runtime_copy).expect("copy the stub as the runtime");
    let record = registry::InstanceRecord::unique();
    let instance = record.name().to_string();

    // Not an instance name: an error, whatever is installed.
    set_instance("not a name");
    // SAFETY: nothing is found, so nothing is loaded.
    let error = unsafe { Library::load_default(&quiet()) }.expect_err("an invalid name");
    let attempts = attempts_ending_at_the_instance(&error);
    assert_eq!(
        attempts[2].outcome,
        LookupOutcome::InvalidInstanceName {
            name: "not a name".to_string()
        }
    );

    // No record for the instance: an error naming it, never the installed runtime.
    set_instance(&instance);
    // SAFETY: as above.
    let error = unsafe { Library::load_default(&quiet()) }.expect_err("an unknown instance");
    let attempts = attempts_ending_at_the_instance(&error);
    assert_eq!(
        attempts[2].outcome,
        LookupOutcome::UnknownInstance {
            name: instance.clone()
        }
    );
    let message = error.to_string();
    assert!(
        message.contains(&format!(
            "QAR_INSTANCE is '{instance}', but no runtime folder is recorded for that instance"
        )),
        "{message}"
    );

    // The record names a folder without the runtime: the lookup stops there.
    record.set_runtime_folder(&empty_folder);
    // SAFETY: as above.
    let error = unsafe { Library::load_default(&quiet()) }.expect_err("no runtime there");
    let attempts = attempts_ending_at_the_instance(&error);
    assert_eq!(
        attempts[2].outcome,
        LookupOutcome::InstanceRuntimeNotLoadable {
            name: instance.clone(),
            path: empty_folder.join(LIBRARY_FILE_NAME),
            message: None,
        }
    );

    // The record names the folder with the runtime.
    record.set_runtime_folder(&runtime_folder);
    // SAFETY: the copy is qaros-stub, which matches these bindings by construction.
    let library = unsafe { Library::load_default(&quiet()) }.expect("load the instance's runtime");
    let loaded_from = library
        .library_path()
        .expect("the loaded file")
        .to_path_buf();
    assert_eq!(canonical(&loaded_from), canonical(&runtime_copy));
    assert_eq!(library.lookup_step(), Some(LookupStep::Instance));
    drop(library);

    // The runtime stays mapped after the drop, so step 0 finds it before the instance's folder.
    // Windows matches a loaded module by its file name.
    record.set_runtime_folder(&empty_folder);
    // SAFETY: as above; the mapped copy is reused.
    let library = unsafe { Library::load_default(&quiet()) }.expect("the runtime already loaded");
    let loaded_from = library
        .library_path()
        .expect("the loaded file")
        .to_path_buf();
    assert_eq!(canonical(&loaded_from), canonical(&runtime_copy));
    assert_eq!(library.lookup_step(), Some(LookupStep::AlreadyLoaded));
    drop(library);

    drop(record);
    if let Err(error) = std::fs::remove_dir_all(&empty_folder) {
        eprintln!("cannot remove {}: {error}", empty_folder.display());
    }
}

#[cfg(windows)]
fn canonical(path: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(path)
        .unwrap_or_else(|error| panic!("cannot resolve {}: {error}", path.display()))
}

/// Outside Windows there are no instance records: the lookup, like the C loader, does not read
/// `QAR_INSTANCE`, so even a value that is no instance name is no error.
#[cfg(not(windows))]
#[test]
fn qar_instance_is_not_read_outside_windows() {
    set_instance("not a name");
    // SAFETY: a runtime found by the dynamic linker's search is a qar-streaming-c the developer
    // put on the library search path; nothing is loaded otherwise.
    match unsafe { Library::load_default(&quiet()) } {
        Ok(library) => assert_ne!(library.lookup_step(), Some(LookupStep::Instance)),
        Err(Error::Load(LoadError::NotFound { attempts })) => {
            let instance = attempts
                .iter()
                .find(|attempt| attempt.step == LookupStep::Instance)
                .expect("step 2 is listed");
            assert_eq!(instance.outcome, LookupOutcome::NotApplicable);
            assert_eq!(
                attempts.last().map(|attempt| attempt.step),
                Some(LookupStep::SearchPath)
            );
        }
        Err(error) => panic!("expected NotFound, got {error}"),
    }
}

/// The per-user instance record a QAROS development hub writes, for one test run.
#[cfg(windows)]
mod registry {
    use std::ffi::{OsStr, c_void};
    use std::os::windows::ffi::OsStrExt as _;
    use std::path::Path;

    use qaros::qaros_sys::{INSTANCE_RUNTIME_FOLDER_VALUE, INSTANCES_REGISTRY_KEY};

    type RegistryKey = isize;
    #[allow(clippy::cast_possible_wrap)]
    const HKEY_CURRENT_USER: RegistryKey = 0x8000_0001_u32 as i32 as RegistryKey;
    const REG_SZ: u32 = 1;

    #[link(name = "advapi32")]
    unsafe extern "system" {
        fn RegSetKeyValueW(
            key: RegistryKey,
            sub_key: *const u16,
            value_name: *const u16,
            value_type: u32,
            data: *const c_void,
            data_size: u32,
        ) -> i32;
        fn RegDeleteTreeW(key: RegistryKey, sub_key: *const u16) -> i32;
    }

    fn wide(text: &OsStr) -> Vec<u16> {
        text.encode_wide().chain(Some(0)).collect()
    }

    /// `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>`, removed on drop.
    pub(super) struct InstanceRecord {
        name: String,
    }

    impl InstanceRecord {
        /// A name unique to this run: the process id and the time.
        pub(super) fn unique() -> Self {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time after the epoch")
                .as_nanos();
            Self {
                name: format!("rust-test-{}-{nanos:x}", std::process::id()),
            }
        }

        pub(super) fn name(&self) -> &str {
            &self.name
        }

        fn key(&self) -> Vec<u16> {
            wide(OsStr::new(&format!(
                "{INSTANCES_REGISTRY_KEY}\\{}",
                self.name
            )))
        }

        pub(super) fn set_runtime_folder(&self, folder: &Path) {
            let data = wide(folder.as_os_str());
            let data_size = u32::try_from(data.len() * 2).expect("a short folder");
            let value = wide(OsStr::new(INSTANCE_RUNTIME_FOLDER_VALUE));
            // SAFETY: NUL-terminated key and value names; data holds data_size bytes, its NUL
            // included, as REG_SZ wants.
            let status = unsafe {
                RegSetKeyValueW(
                    HKEY_CURRENT_USER,
                    self.key().as_ptr(),
                    value.as_ptr(),
                    REG_SZ,
                    data.as_ptr().cast(),
                    data_size,
                )
            };
            assert_eq!(status, 0, "write the instance record");
        }
    }

    impl Drop for InstanceRecord {
        fn drop(&mut self) {
            // SAFETY: a NUL-terminated key name under HKCU.
            let status = unsafe { RegDeleteTreeW(HKEY_CURRENT_USER, self.key().as_ptr()) };
            // 2: ERROR_FILE_NOT_FOUND, never written.
            if status != 0 && status != 2 {
                eprintln!("cannot remove the instance record {}: {status}", self.name);
            }
        }
    }
}
