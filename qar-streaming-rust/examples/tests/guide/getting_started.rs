mod load_installed_runtime {
    // ANCHOR: load_installed_runtime
    use qaros::{Library, LibraryConfig};

    fn main() {
        // SAFETY: the runtime the lookup finds (the installed QAROS) matches these bindings.
        let library = match unsafe { Library::load_default(&LibraryConfig::default()) } {
            Ok(library) => library,
            Err(error) => {
                // QAROS not installed: run without AR/VR features
                eprintln!("{error}"); // names every place the lookup tried
                return;
            }
        };
        // ... use the API ...
        drop(library); // qar_library_destroy
    }
    // ANCHOR_END: load_installed_runtime
}

mod lifecycle {
    // ANCHOR: lifecycle
    use qaros::{Library, LibraryConfig, RuntimeConfig};

    fn main() -> qaros::Result<()> {
        // 1. Library load and init (process-wide, once)
        let library_config = LibraryConfig {
            console_logging: true,
            ..LibraryConfig::default()
        };
        // SAFETY: the runtime the lookup finds matches these bindings.
        let library = unsafe { Library::load_default(&library_config)? };

        // 2. Runtime create (your process's QAROS context)
        let runtime = library.create_runtime(&RuntimeConfig::default())?;

        // 3. Obtain a session: on the QAROS PC, join as a local app
        //    (see Onboarding and Sessions)

        // 4. Do work: app volume, render streams, panels

        // 5. Teardown, in reverse. Each handle keeps its parent alive,
        //    so any drop order still tears down in this one.
        drop(runtime);
        drop(library);
        Ok(())
    }
    // ANCHOR_END: lifecycle
}
