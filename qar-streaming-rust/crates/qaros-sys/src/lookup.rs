//! The default runtime lookup every QAROS binding follows (QAROS developer guide, "Authoring a
//! Language Binding", "The default lookup"; the C header's `qar_library_load(NULL)`).
//!
//! The first match wins:
//!
//! 0. a `qar-streaming-c` already loaded in the process: nothing is loaded, a reference is taken;
//! 1. the runtime beside the binding: the folder of the module this crate is compiled into (the
//!    executable, or the plug-in library that loads QAROS);
//! 2. the folder named by `QAROS_RUNTIME_DIR`; when the variable is set it is the only copy
//!    considered, and a runtime missing there or failing to load ends the lookup with an error;
//! 3. the installed QAROS, `<InstallFolder>\bin`, with `InstallFolder` read from
//!    `HKLM\SOFTWARE\Quaternar\QAROS` (Windows only, never `HKCU`); an installed copy that is
//!    missing or fails to load ends the lookup with an error;
//! 4. Windows: each absolute folder on `PATH`, in order; elsewhere the dynamic linker's own search.
//!
//! The current directory is never searched. The decisions are kept apart from the operating
//! system behind [`RuntimeLocator`], so the order is tested without a registry, an environment or
//! a real library.

use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};

/// The environment variable naming a folder that holds the runtime (step 2).
pub const RUNTIME_DIR_VARIABLE: &str = "QAROS_RUNTIME_DIR";

/// The registry key, under `HKEY_LOCAL_MACHINE`, where the QAROS installer records its folder
/// (step 3).
pub const INSTALL_REGISTRY_KEY: &str = r"SOFTWARE\Quaternar\QAROS";

/// The value under [`INSTALL_REGISTRY_KEY`] that holds the installation folder.
pub const INSTALL_FOLDER_VALUE: &str = "InstallFolder";

/// One place the default lookup considers, in lookup order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LookupStep {
    /// A runtime already loaded in this process.
    AlreadyLoaded,
    /// The folder of the module this binding is compiled into.
    BesideBinding,
    /// The folder `QAROS_RUNTIME_DIR` names.
    RuntimeDirVariable,
    /// The `bin` folder of the installed QAROS.
    InstalledRuntime,
    /// The folders on `PATH`, or the dynamic linker's search outside Windows.
    SearchPath,
}

/// What the lookup met at one place.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LookupOutcome {
    /// Nothing to look at: no runtime loaded, the binding's folder unknown, the variable unset,
    /// QAROS not installed, or no runtime in any folder on `PATH`.
    NotApplicable,
    /// The runtime file is not at this path.
    Missing {
        /// The path looked at.
        path: PathBuf,
    },
    /// The runtime file is there but failed to load.
    FailedToLoad {
        /// The path of the runtime file.
        path: PathBuf,
        /// The operating system's reason.
        message: String,
    },
}

/// One place the default lookup looked at and what it met there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LookupAttempt {
    /// Where the lookup looked.
    pub step: LookupStep,
    /// What it met.
    pub outcome: LookupOutcome,
}

impl fmt::Display for LookupAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let place = match self.step {
            LookupStep::AlreadyLoaded => "already loaded in this process",
            LookupStep::BesideBinding => "beside the binding",
            LookupStep::RuntimeDirVariable => RUNTIME_DIR_VARIABLE,
            LookupStep::InstalledRuntime => "the installed QAROS",
            LookupStep::SearchPath if cfg!(windows) => "PATH",
            LookupStep::SearchPath => "the dynamic linker's search",
        };
        match &self.outcome {
            LookupOutcome::NotApplicable => {
                let reason = match self.step {
                    LookupStep::AlreadyLoaded => "no".to_string(),
                    LookupStep::BesideBinding => "the binding's folder is unknown".to_string(),
                    LookupStep::RuntimeDirVariable => "not set".to_string(),
                    LookupStep::InstalledRuntime if cfg!(windows) => format!(
                        "not installed (no {INSTALL_FOLDER_VALUE} under HKLM\\{INSTALL_REGISTRY_KEY})"
                    ),
                    LookupStep::InstalledRuntime => "looked up on Windows only".to_string(),
                    LookupStep::SearchPath => "not found in any folder".to_string(),
                };
                write!(formatter, "{place}: {reason}")
            }
            LookupOutcome::Missing { path } => {
                write!(formatter, "{place}: {} does not exist", path.display())
            }
            LookupOutcome::FailedToLoad { path, message } => {
                write!(
                    formatter,
                    "{place}: {} failed to load: {message}",
                    path.display()
                )
            }
        }
    }
}

/// Where the default lookup took the runtime from.
#[derive(Debug)]
pub(crate) struct FoundRuntime<Library> {
    pub(crate) library: Library,
    pub(crate) step: LookupStep,
    /// The file loaded; the bare file name for a runtime already loaded or found by the
    /// dynamic linker.
    pub(crate) path: PathBuf,
}

/// How step 4 searches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SearchPath {
    /// Windows: these folders, in order.
    Folders(Vec<PathBuf>),
    /// Elsewhere: the dynamic linker's search for the bare file name.
    DynamicLinker,
}

/// The operating system as the default lookup sees it.
pub(crate) trait RuntimeLocator {
    type Library;

    /// Step 0: a reference to a runtime already loaded in the process.
    fn already_loaded(&self) -> Option<Self::Library>;
    /// Step 1: the folder of the module this binding is compiled into.
    fn binding_folder(&self) -> Option<PathBuf>;
    /// Step 2: the folder `QAROS_RUNTIME_DIR` names, when set and not empty.
    fn runtime_dir_variable(&self) -> Option<PathBuf>;
    /// Step 3: `<InstallFolder>\bin` of the installed QAROS.
    fn installed_bin_folder(&self) -> Option<PathBuf>;
    /// Step 4.
    fn search_path(&self) -> SearchPath;
    fn is_file(&self, path: &Path) -> bool;
    /// Loads the runtime file at `path`, its dependencies from its own folder and System32.
    fn load(&self, path: &Path) -> Result<Self::Library, String>;
    /// Loads the runtime by its bare file name through the dynamic linker's search.
    fn load_by_linker_search(&self, file_name: &str) -> Result<Self::Library, String>;
}

/// Runs the default lookup for `file_name` (`qar-streaming-c.dll`). On failure the attempts name
/// every place looked at, in order.
pub(crate) fn find_runtime<Locator: RuntimeLocator>(
    locator: &Locator,
    file_name: &str,
) -> Result<FoundRuntime<Locator::Library>, Vec<LookupAttempt>> {
    let mut attempts = Vec::new();
    let found = |library, step, path| {
        Ok(FoundRuntime {
            library,
            step,
            path,
        })
    };

    if let Some(library) = locator.already_loaded() {
        return found(library, LookupStep::AlreadyLoaded, PathBuf::from(file_name));
    }
    attempts.push(not_applicable(LookupStep::AlreadyLoaded));

    match locator.binding_folder() {
        Some(folder) => match load_from_folder(locator, &folder, file_name) {
            Ok(library) => {
                return found(library, LookupStep::BesideBinding, folder.join(file_name));
            }
            Err(outcome) => attempts.push(LookupAttempt {
                step: LookupStep::BesideBinding,
                outcome,
            }),
        },
        None => attempts.push(not_applicable(LookupStep::BesideBinding)),
    }

    // A set variable or an installation is the only copy considered at its step: a failure
    // there ends the lookup instead of falling through to a copy the user did not choose.
    for (step, folder) in [
        (
            LookupStep::RuntimeDirVariable,
            locator.runtime_dir_variable(),
        ),
        (LookupStep::InstalledRuntime, locator.installed_bin_folder()),
    ] {
        let Some(folder) = folder else {
            attempts.push(not_applicable(step));
            continue;
        };
        return match load_from_folder(locator, &folder, file_name) {
            Ok(library) => found(library, step, folder.join(file_name)),
            Err(outcome) => {
                attempts.push(LookupAttempt { step, outcome });
                Err(attempts)
            }
        };
    }

    match locator.search_path() {
        SearchPath::Folders(folders) => {
            for folder in folders {
                match load_from_folder(locator, &folder, file_name) {
                    Ok(library) => {
                        return found(library, LookupStep::SearchPath, folder.join(file_name));
                    }
                    Err(LookupOutcome::Missing { .. }) => {}
                    Err(outcome) => attempts.push(LookupAttempt {
                        step: LookupStep::SearchPath,
                        outcome,
                    }),
                }
            }
            attempts.push(not_applicable(LookupStep::SearchPath));
        }
        SearchPath::DynamicLinker => match locator.load_by_linker_search(file_name) {
            Ok(library) => {
                return found(library, LookupStep::SearchPath, PathBuf::from(file_name));
            }
            Err(message) => attempts.push(LookupAttempt {
                step: LookupStep::SearchPath,
                outcome: LookupOutcome::FailedToLoad {
                    path: PathBuf::from(file_name),
                    message,
                },
            }),
        },
    }
    Err(attempts)
}

fn not_applicable(step: LookupStep) -> LookupAttempt {
    LookupAttempt {
        step,
        outcome: LookupOutcome::NotApplicable,
    }
}

fn load_from_folder<Locator: RuntimeLocator>(
    locator: &Locator,
    folder: &Path,
    file_name: &str,
) -> Result<Locator::Library, LookupOutcome> {
    let path = folder.join(file_name);
    if !locator.is_file(&path) {
        return Err(LookupOutcome::Missing { path });
    }
    locator
        .load(&path)
        .map_err(|message| LookupOutcome::FailedToLoad { path, message })
}

/// The folders of a Windows `PATH` value, in order: separated by `;`, each one unquoted, empty
/// and relative entries left out (a relative entry would search the current directory).
pub(crate) fn search_path_folders(path_value: &OsStr) -> Vec<PathBuf> {
    path_value
        .to_string_lossy()
        .split(';')
        .map(|entry| {
            let entry = entry.trim();
            entry
                .strip_prefix('"')
                .and_then(|unquoted| unquoted.strip_suffix('"'))
                .unwrap_or(entry)
        })
        .filter(|entry| !entry.is_empty())
        .map(PathBuf::from)
        .filter(|folder| folder.is_absolute())
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ffi::OsString;

    use super::*;

    const FILE_NAME: &str = "qar-streaming-c.dll";

    /// What a fake library file does when loaded.
    #[derive(Clone, Copy)]
    enum FakeFile {
        Loads,
        FailsToLoad,
    }

    /// A process, environment and file system made of plain data. The "library" a load returns
    /// is the path it came from, and every load is recorded.
    #[derive(Default)]
    struct FakeSystem {
        already_loaded: bool,
        binding_folder: Option<PathBuf>,
        runtime_dir_variable: Option<PathBuf>,
        installed_bin_folder: Option<PathBuf>,
        search_path_folders: Option<Vec<PathBuf>>,
        linker_finds_runtime: bool,
        files: HashMap<PathBuf, FakeFile>,
        loads: RefCell<Vec<PathBuf>>,
    }

    impl FakeSystem {
        fn with_file(mut self, folder: &Path, file: FakeFile) -> Self {
            self.files.insert(folder.join(FILE_NAME), file);
            self
        }
    }

    impl RuntimeLocator for FakeSystem {
        type Library = PathBuf;

        fn already_loaded(&self) -> Option<PathBuf> {
            self.already_loaded.then(|| PathBuf::from("already loaded"))
        }

        fn binding_folder(&self) -> Option<PathBuf> {
            self.binding_folder.clone()
        }

        fn runtime_dir_variable(&self) -> Option<PathBuf> {
            self.runtime_dir_variable.clone()
        }

        fn installed_bin_folder(&self) -> Option<PathBuf> {
            self.installed_bin_folder.clone()
        }

        fn search_path(&self) -> SearchPath {
            self.search_path_folders
                .clone()
                .map_or(SearchPath::DynamicLinker, SearchPath::Folders)
        }

        fn is_file(&self, path: &Path) -> bool {
            self.files.contains_key(path)
        }

        fn load(&self, path: &Path) -> Result<PathBuf, String> {
            self.loads.borrow_mut().push(path.to_path_buf());
            match self.files.get(path) {
                Some(FakeFile::Loads) => Ok(path.to_path_buf()),
                Some(FakeFile::FailsToLoad) => Err("a dependent DLL is missing".to_string()),
                None => panic!("loaded {} without checking it exists", path.display()),
            }
        }

        fn load_by_linker_search(&self, file_name: &str) -> Result<PathBuf, String> {
            self.loads.borrow_mut().push(PathBuf::from(file_name));
            if self.linker_finds_runtime {
                Ok(PathBuf::from(format!("linker:{file_name}")))
            } else {
                Err(format!("{file_name}: cannot open shared object file"))
            }
        }
    }

    fn folder(name: &str) -> PathBuf {
        std::env::temp_dir().join("qaros-lookup-test").join(name)
    }

    fn runtime_in(name: &str) -> PathBuf {
        folder(name).join(FILE_NAME)
    }

    /// Every folder of the lookup holds a loadable runtime; tests take away what they need.
    fn system_with_a_runtime_everywhere() -> FakeSystem {
        FakeSystem {
            already_loaded: true,
            binding_folder: Some(folder("binding")),
            runtime_dir_variable: Some(folder("variable")),
            installed_bin_folder: Some(folder("installed")),
            search_path_folders: Some(vec![folder("path-1")]),
            ..FakeSystem::default()
        }
        .with_file(&folder("binding"), FakeFile::Loads)
        .with_file(&folder("variable"), FakeFile::Loads)
        .with_file(&folder("installed"), FakeFile::Loads)
        .with_file(&folder("path-1"), FakeFile::Loads)
    }

    fn found_at(system: &FakeSystem) -> (LookupStep, PathBuf) {
        let found = find_runtime(system, FILE_NAME).expect("a runtime");
        let file_loaded = if found.library.is_absolute() {
            found.library.clone()
        } else {
            PathBuf::from(FILE_NAME)
        };
        assert_eq!(
            found.path, file_loaded,
            "the path reported is the file loaded"
        );
        (found.step, found.library)
    }

    fn attempts_of(system: &FakeSystem) -> Vec<LookupAttempt> {
        find_runtime(system, FILE_NAME).expect_err("no runtime")
    }

    fn attempt(step: LookupStep, outcome: LookupOutcome) -> LookupAttempt {
        LookupAttempt { step, outcome }
    }

    #[test]
    fn a_runtime_already_in_the_process_wins_and_nothing_is_loaded() {
        let system = system_with_a_runtime_everywhere();
        assert_eq!(
            found_at(&system),
            (LookupStep::AlreadyLoaded, PathBuf::from("already loaded"))
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn the_runtime_beside_the_binding_wins_over_the_variable_and_the_installation() {
        let system = FakeSystem {
            already_loaded: false,
            ..system_with_a_runtime_everywhere()
        };
        assert_eq!(
            found_at(&system),
            (LookupStep::BesideBinding, runtime_in("binding"))
        );
    }

    #[test]
    fn the_variable_wins_over_the_installation() {
        let system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            ..system_with_a_runtime_everywhere()
        };
        assert_eq!(
            found_at(&system),
            (LookupStep::RuntimeDirVariable, runtime_in("variable"))
        );
    }

    #[test]
    fn a_runtime_beside_the_binding_that_fails_to_load_is_reported_and_the_lookup_goes_on() {
        let mut system = FakeSystem {
            already_loaded: false,
            ..system_with_a_runtime_everywhere()
        };
        system
            .files
            .insert(runtime_in("binding"), FakeFile::FailsToLoad);
        assert_eq!(
            found_at(&system),
            (LookupStep::RuntimeDirVariable, runtime_in("variable"))
        );
        assert_eq!(
            *system.loads.borrow(),
            vec![runtime_in("binding"), runtime_in("variable")]
        );
    }

    #[test]
    fn a_set_variable_without_a_runtime_ends_the_lookup_before_the_installation() {
        let system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            runtime_dir_variable: Some(folder("empty")),
            ..system_with_a_runtime_everywhere()
        };
        assert_eq!(
            attempts_of(&system),
            vec![
                attempt(LookupStep::AlreadyLoaded, LookupOutcome::NotApplicable),
                attempt(LookupStep::BesideBinding, LookupOutcome::NotApplicable),
                attempt(
                    LookupStep::RuntimeDirVariable,
                    LookupOutcome::Missing {
                        path: runtime_in("empty")
                    }
                ),
            ]
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn a_set_variable_whose_runtime_fails_to_load_ends_the_lookup() {
        let mut system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            ..system_with_a_runtime_everywhere()
        };
        system
            .files
            .insert(runtime_in("variable"), FakeFile::FailsToLoad);
        let attempts = attempts_of(&system);
        assert_eq!(
            attempts.last(),
            Some(&attempt(
                LookupStep::RuntimeDirVariable,
                LookupOutcome::FailedToLoad {
                    path: runtime_in("variable"),
                    message: "a dependent DLL is missing".to_string(),
                }
            ))
        );
        assert_eq!(*system.loads.borrow(), vec![runtime_in("variable")]);
    }

    #[test]
    fn without_the_variable_the_installation_wins_over_path() {
        let system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            runtime_dir_variable: None,
            ..system_with_a_runtime_everywhere()
        };
        assert_eq!(
            found_at(&system),
            (LookupStep::InstalledRuntime, runtime_in("installed"))
        );
    }

    #[test]
    fn an_installation_without_its_runtime_is_reported_and_path_is_not_searched() {
        let system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            runtime_dir_variable: None,
            installed_bin_folder: Some(folder("broken-installation")),
            ..system_with_a_runtime_everywhere()
        };
        assert_eq!(
            attempts_of(&system).last(),
            Some(&attempt(
                LookupStep::InstalledRuntime,
                LookupOutcome::Missing {
                    path: runtime_in("broken-installation")
                }
            ))
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn an_installed_runtime_that_fails_to_load_is_reported_and_path_is_not_searched() {
        let mut system = FakeSystem {
            already_loaded: false,
            binding_folder: None,
            runtime_dir_variable: None,
            ..system_with_a_runtime_everywhere()
        };
        system
            .files
            .insert(runtime_in("installed"), FakeFile::FailsToLoad);
        assert_eq!(
            attempts_of(&system).last(),
            Some(&attempt(
                LookupStep::InstalledRuntime,
                LookupOutcome::FailedToLoad {
                    path: runtime_in("installed"),
                    message: "a dependent DLL is missing".to_string(),
                }
            ))
        );
        assert_eq!(*system.loads.borrow(), vec![runtime_in("installed")]);
    }

    #[test]
    fn path_folders_are_tried_in_order_past_ones_without_the_runtime_or_failing_to_load() {
        let system = FakeSystem {
            search_path_folders: Some(vec![folder("path-a"), folder("path-b"), folder("path-c")]),
            ..FakeSystem::default()
        }
        .with_file(&folder("path-b"), FakeFile::FailsToLoad)
        .with_file(&folder("path-c"), FakeFile::Loads);
        assert_eq!(
            found_at(&system),
            (LookupStep::SearchPath, runtime_in("path-c"))
        );
        assert_eq!(
            *system.loads.borrow(),
            vec![runtime_in("path-b"), runtime_in("path-c")]
        );
    }

    #[test]
    fn outside_windows_step_four_is_the_dynamic_linkers_search() {
        let system = FakeSystem {
            linker_finds_runtime: true,
            ..FakeSystem::default()
        };
        assert_eq!(
            found_at(&system),
            (
                LookupStep::SearchPath,
                PathBuf::from(format!("linker:{FILE_NAME}"))
            )
        );
    }

    #[test]
    fn nothing_found_names_every_place_looked_at_in_order() {
        let system = FakeSystem {
            binding_folder: Some(folder("binding")),
            search_path_folders: Some(vec![folder("path-a"), folder("path-b")]),
            ..FakeSystem::default()
        }
        .with_file(&folder("path-b"), FakeFile::FailsToLoad);
        let attempts = attempts_of(&system);
        assert_eq!(
            attempts,
            vec![
                attempt(LookupStep::AlreadyLoaded, LookupOutcome::NotApplicable),
                attempt(
                    LookupStep::BesideBinding,
                    LookupOutcome::Missing {
                        path: runtime_in("binding")
                    }
                ),
                attempt(LookupStep::RuntimeDirVariable, LookupOutcome::NotApplicable),
                attempt(LookupStep::InstalledRuntime, LookupOutcome::NotApplicable),
                attempt(
                    LookupStep::SearchPath,
                    LookupOutcome::FailedToLoad {
                        path: runtime_in("path-b"),
                        message: "a dependent DLL is missing".to_string(),
                    }
                ),
                attempt(LookupStep::SearchPath, LookupOutcome::NotApplicable),
            ]
        );
        let lines: Vec<String> = attempts.iter().map(ToString::to_string).collect();
        assert_eq!(lines[0], "already loaded in this process: no");
        assert_eq!(
            lines[1],
            format!(
                "beside the binding: {} does not exist",
                runtime_in("binding").display()
            )
        );
        assert_eq!(lines[2], "QAROS_RUNTIME_DIR: not set");
        assert!(
            lines[3].starts_with("the installed QAROS: "),
            "{}",
            lines[3]
        );
        assert!(
            lines[4].ends_with("failed to load: a dependent DLL is missing"),
            "{}",
            lines[4]
        );
        assert!(
            lines[5].ends_with(": not found in any folder"),
            "{}",
            lines[5]
        );
    }

    #[test]
    fn path_entries_are_unquoted_and_empty_or_relative_ones_are_dropped() {
        let separator = OsString::from(";");
        let mut path_value = OsString::new();
        for entry in [
            folder("first").into_os_string(),
            OsString::new(),
            OsString::from("relative\\folder"),
            OsString::from("."),
            {
                let mut quoted = OsString::from("\"");
                quoted.push(folder("second with space"));
                quoted.push("\"");
                quoted
            },
        ] {
            path_value.push(&entry);
            path_value.push(&separator);
        }
        assert_eq!(
            search_path_folders(&path_value),
            vec![folder("first"), folder("second with space")]
        );
    }
}
