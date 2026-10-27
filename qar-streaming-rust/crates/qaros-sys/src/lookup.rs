//! The default runtime lookup every QAROS binding follows (QAROS developer guide, "Authoring a
//! Language Binding", "The default lookup"; the C header's `qar_library_load(NULL)`).
//!
//! The first match wins:
//!
//! 0. a `qar-streaming-c` already loaded in the process: nothing is loaded, a reference is taken;
//! 1. the runtime beside the binding: the folder of the module this crate is compiled into (the
//!    executable, or the plug-in library that loads QAROS);
//! 2. when `QAR_INSTANCE` names a QAROS instance, that instance's runtime folder: `default` is
//!    the installed QAROS, any other name the folder recorded as `RuntimeFolder` under
//!    `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>` (a QAROS development hub records its build
//!    output there and sets `QAR_INSTANCE` for the apps it starts), else that record's
//!    `<InstallFolder>\bin`. When the variable is set, an invalid name, an unknown instance or a
//!    runtime that does not load there ends the lookup with an error, never a reason to load
//!    another instance's runtime. Windows only: elsewhere there are no instance records and the
//!    variable is not read;
//! 3. the installed QAROS, `<InstallFolder>\bin`, with `InstallFolder` read from
//!    `HKLM\SOFTWARE\Quaternar\QAROS` (Windows only, never `HKCU`); an installed copy that is
//!    missing or fails to load ends the lookup with an error;
//! 4. Windows: each absolute folder on `PATH`, in order; elsewhere the dynamic linker's own search.
//!
//! The current directory is never searched. The decisions are kept apart from the operating
//! system behind [`RuntimeLocator`], so the order is tested without a registry, an environment or
//! a real library.

use std::fmt;
use std::path::{Path, PathBuf};

/// The environment variable naming the QAROS instance an application belongs to (step 2): the
/// runtime is loaded from that instance's folder, and enrols with that instance's hub. A QAROS
/// development hub sets it for the apps it starts. Read on Windows only.
pub const INSTANCE_VARIABLE: &str = "QAR_INSTANCE";

/// The name of the installed (`HKLM`) instance: `QAR_INSTANCE=default` loads the installed
/// QAROS's runtime.
pub const DEFAULT_INSTANCE_NAME: &str = "default";

/// The longest instance name. A name is 1 to 64 characters of `A-Z a-z 0-9 . _ -`.
pub const INSTANCE_NAME_MAX_LENGTH: usize = 64;

/// The key, under `HKEY_CURRENT_USER`, holding one per-user record per named instance (step 2):
/// the same records the runtime reads to pick the hub of a named instance.
pub const INSTANCES_REGISTRY_KEY: &str = r"SOFTWARE\Quaternar\QAROS\Instances";

/// The value of a per-user instance record naming the folder that holds that instance's runtime
/// (a development hub records its build output there).
pub const INSTANCE_RUNTIME_FOLDER_VALUE: &str = "RuntimeFolder";

/// The registry key, under `HKEY_LOCAL_MACHINE`, where the QAROS installer records its folder
/// (step 3, and the `default` instance of step 2).
pub const INSTALL_REGISTRY_KEY: &str = r"SOFTWARE\Quaternar\QAROS";

/// The value under [`INSTALL_REGISTRY_KEY`] that holds the installation folder; a per-user
/// instance record may hold one too.
pub const INSTALL_FOLDER_VALUE: &str = "InstallFolder";

/// One place the default lookup considers, in lookup order. Its `Display` is the name the
/// lookup's messages give the place (`QAR_INSTANCE` for [`LookupStep::Instance`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LookupStep {
    /// A runtime already loaded in this process.
    AlreadyLoaded,
    /// The folder of the module this binding is compiled into.
    BesideBinding,
    /// The runtime folder of the instance `QAR_INSTANCE` names (Windows only).
    Instance,
    /// The `bin` folder of the installed QAROS.
    InstalledRuntime,
    /// The folders on `PATH`, or the dynamic linker's search outside Windows.
    SearchPath,
}

impl LookupStep {
    /// The name the lookup's messages give this place.
    pub fn place(self) -> &'static str {
        match self {
            Self::AlreadyLoaded => "already loaded in this process",
            Self::BesideBinding => "beside the binding",
            Self::Instance => INSTANCE_VARIABLE,
            Self::InstalledRuntime => "the installed QAROS",
            Self::SearchPath if cfg!(windows) => "PATH",
            Self::SearchPath => "the dynamic linker's search",
        }
    }
}

impl fmt::Display for LookupStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.place())
    }
}

/// What the lookup met at one place.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LookupOutcome {
    /// Nothing to look at: no runtime loaded, the binding's folder unknown, `QAR_INSTANCE` unset
    /// (or not read, outside Windows), QAROS not installed, or no runtime in any folder on
    /// `PATH`.
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
    /// `QAR_INSTANCE` is set to something that is not an instance name (see
    /// [`is_valid_instance_name`]). No runtime is loaded.
    InvalidInstanceName {
        /// The value of `QAR_INSTANCE`, as set (read lossily when it is not Unicode).
        name: String,
    },
    /// `QAR_INSTANCE` names an instance without a runtime folder: for `default`, QAROS is not
    /// installed; for any other name, its per-user record holds neither a fully qualified
    /// `RuntimeFolder` nor a fully qualified `InstallFolder`. No other runtime is loaded.
    UnknownInstance {
        /// The instance `QAR_INSTANCE` names.
        name: String,
    },
    /// `QAR_INSTANCE` names an instance whose runtime folder holds no loadable runtime. No other
    /// runtime is loaded.
    InstanceRuntimeNotLoadable {
        /// The instance `QAR_INSTANCE` names.
        name: String,
        /// The runtime file looked at, in the instance's runtime folder.
        path: PathBuf,
        /// The operating system's reason when the file is there but failed to load; `None` when
        /// it is not there.
        message: Option<String>,
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
        let place = self.step.place();
        match &self.outcome {
            LookupOutcome::NotApplicable => {
                let reason = match self.step {
                    LookupStep::AlreadyLoaded => "no".to_string(),
                    LookupStep::BesideBinding => "the binding's folder is unknown".to_string(),
                    LookupStep::Instance if cfg!(windows) => "not set".to_string(),
                    LookupStep::Instance => {
                        "read on Windows only (there are no instance records here)".to_string()
                    }
                    LookupStep::InstalledRuntime if cfg!(windows) => format!(
                        "not installed (no {INSTALL_FOLDER_VALUE} under HKLM\\{INSTALL_REGISTRY_KEY})"
                    ),
                    LookupStep::InstalledRuntime => "looked up on Windows only".to_string(),
                    LookupStep::SearchPath => "not found in any folder".to_string(),
                };
                write!(formatter, "{place}: {reason}")
            }
            LookupOutcome::Missing { path } if self.step == LookupStep::InstalledRuntime => write!(
                formatter,
                "{place}: the installed QAROS runtime in '{}' could not be loaded ({} does not \
                 exist). Repair or reinstall QAROS",
                folder_of(path).display(),
                path.display()
            ),
            LookupOutcome::Missing { path } => {
                write!(formatter, "{place}: {} does not exist", path.display())
            }
            LookupOutcome::FailedToLoad { path, message }
                if self.step == LookupStep::InstalledRuntime =>
            {
                write!(
                    formatter,
                    "{place}: the installed QAROS runtime in '{}' could not be loaded ({} failed \
                     to load: {message}). Repair or reinstall QAROS",
                    folder_of(path).display(),
                    path.display()
                )
            }
            LookupOutcome::FailedToLoad { path, message } => {
                write!(
                    formatter,
                    "{place}: {} failed to load: {message}",
                    path.display()
                )
            }
            LookupOutcome::InvalidInstanceName { name } => write!(
                formatter,
                "{INSTANCE_VARIABLE}='{name}' is not a QAROS instance name (1-\
                 {INSTANCE_NAME_MAX_LENGTH} of A-Z a-z 0-9 . _ -); no runtime is loaded"
            ),
            LookupOutcome::UnknownInstance { name } if name == DEFAULT_INSTANCE_NAME => write!(
                formatter,
                "{INSTANCE_VARIABLE} is '{DEFAULT_INSTANCE_NAME}', but QAROS is not installed \
                 (no {INSTALL_FOLDER_VALUE} under HKLM\\{INSTALL_REGISTRY_KEY})"
            ),
            LookupOutcome::UnknownInstance { name } => write!(
                formatter,
                "{INSTANCE_VARIABLE} is '{name}', but no runtime folder is recorded for that \
                 instance (no full-path {INSTANCE_RUNTIME_FOLDER_VALUE} or \
                 {INSTALL_FOLDER_VALUE} under HKCU\\{INSTANCES_REGISTRY_KEY}\\{name}). Start its \
                 hub, or unset {INSTANCE_VARIABLE}"
            ),
            LookupOutcome::InstanceRuntimeNotLoadable {
                name,
                path,
                message,
            } => {
                let file_name = path.file_name().map_or_else(
                    || path.display().to_string(),
                    |file_name| file_name.to_string_lossy().into_owned(),
                );
                write!(
                    formatter,
                    "{INSTANCE_VARIABLE} is '{name}', but no loadable {file_name} is in its \
                     runtime folder '{}'",
                    folder_of(path).display()
                )?;
                match message {
                    Some(message) => write!(formatter, " ({file_name} failed to load: {message})"),
                    None => write!(formatter, " ({file_name} does not exist there)"),
                }
            }
        }
    }
}

fn folder_of(path: &Path) -> &Path {
    path.parent().unwrap_or(path)
}

/// Whether `name` is a QAROS instance name: 1 to [`INSTANCE_NAME_MAX_LENGTH`] characters of
/// `A-Z a-z 0-9 . _ -`.
pub fn is_valid_instance_name(name: &str) -> bool {
    (1..=INSTANCE_NAME_MAX_LENGTH).contains(&name.len())
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

/// Whether `folder` is a fully qualified Windows folder: `X:\...` (or `X:/...`) or a UNC
/// `\\server\...`. A relative or drive-relative folder would load against the current
/// directory, which the lookup never searches, so a record holding one counts as no record.
pub(crate) fn is_fully_qualified_folder(folder: &str) -> bool {
    let bytes = folder.as_bytes();
    let drive = bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'\\' | b'/');
    let unc = bytes.starts_with(br"\\");
    drive || unc
}

/// The folder holding the runtime of instance `instance`, as step 2 picks it: for `default` the
/// installed QAROS's `<InstallFolder>\bin`; for any other name the value `RuntimeFolder` of its
/// per-user record, else that record's `<InstallFolder>\bin`. Only fully qualified folders
/// count. `None` when the name is not valid or no folder is recorded.
///
/// The per-user record carries the same trust as an environment variable naming a folder (every
/// process of the user can write both), and is read only when `QAR_INSTANCE` names that instance.
/// No step of the lookup checks the runtime's signature before loading it.
pub(crate) fn instance_runtime_folder<Locator: RuntimeLocator + ?Sized>(
    locator: &Locator,
    instance: &str,
) -> Option<PathBuf> {
    if !is_valid_instance_name(instance) {
        return None;
    }
    if instance == DEFAULT_INSTANCE_NAME {
        return locator.installed_bin_folder();
    }
    let recorded_folder = |value| {
        locator
            .instance_record_value(instance, value)
            .filter(|folder| is_fully_qualified_folder(folder))
            .map(PathBuf::from)
    };
    recorded_folder(INSTANCE_RUNTIME_FOLDER_VALUE)
        .or_else(|| recorded_folder(INSTALL_FOLDER_VALUE).map(|folder| folder.join("bin")))
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
    /// Step 2: the value of `QAR_INSTANCE`, as set, when it is set and not empty; always `None`
    /// where the lookup does not read it (outside Windows).
    fn requested_instance(&self) -> Option<String>;
    /// Step 2: value `value` of the per-user record of `instance`
    /// (`HKCU\SOFTWARE\Quaternar\QAROS\Instances\<instance>`), as stored.
    fn instance_record_value(&self, instance: &str, value: &str) -> Option<String>;
    /// Step 3 (and the `default` instance): `<InstallFolder>\bin` of the installed QAROS.
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

    // A set QAR_INSTANCE, or an installation, is the only copy considered at its step: a failure
    // there ends the lookup instead of falling through to a copy the user did not choose.
    if let Some(instance) = locator.requested_instance() {
        return match load_requested_instance(locator, &instance, file_name) {
            Ok((library, path)) => found(library, LookupStep::Instance, path),
            Err(outcome) => {
                attempts.push(LookupAttempt {
                    step: LookupStep::Instance,
                    outcome,
                });
                Err(attempts)
            }
        };
    }
    attempts.push(not_applicable(LookupStep::Instance));

    if let Some(folder) = locator.installed_bin_folder() {
        return match load_from_folder(locator, &folder, file_name) {
            Ok(library) => found(
                library,
                LookupStep::InstalledRuntime,
                folder.join(file_name),
            ),
            Err(outcome) => {
                attempts.push(LookupAttempt {
                    step: LookupStep::InstalledRuntime,
                    outcome,
                });
                Err(attempts)
            }
        };
    }
    attempts.push(not_applicable(LookupStep::InstalledRuntime));

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

/// Step 2: the runtime of the instance `QAR_INSTANCE` names, or why there is none. Never another
/// instance's runtime.
fn load_requested_instance<Locator: RuntimeLocator>(
    locator: &Locator,
    instance: &str,
    file_name: &str,
) -> Result<(Locator::Library, PathBuf), LookupOutcome> {
    let name = instance.to_string();
    if !is_valid_instance_name(instance) {
        return Err(LookupOutcome::InvalidInstanceName { name });
    }
    let Some(folder) = instance_runtime_folder(locator, instance) else {
        return Err(LookupOutcome::UnknownInstance { name });
    };
    let path = folder.join(file_name);
    match load_from_folder(locator, &folder, file_name) {
        Ok(library) => Ok((library, path)),
        Err(LookupOutcome::FailedToLoad { message, .. }) => {
            Err(LookupOutcome::InstanceRuntimeNotLoadable {
                name,
                path,
                message: Some(message),
            })
        }
        Err(_) => Err(LookupOutcome::InstanceRuntimeNotLoadable {
            name,
            path,
            message: None,
        }),
    }
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
pub(crate) fn search_path_folders(path_value: &std::ffi::OsStr) -> Vec<PathBuf> {
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
    const DEV_INSTANCE: &str = "dev-0123456789abcdef";

    /// What a fake library file does when loaded.
    #[derive(Clone, Copy)]
    enum FakeFile {
        Loads,
        FailsToLoad,
    }

    /// A process, environment, registry and file system made of plain data. The "library" a
    /// load returns is the path it came from, and every load is recorded.
    #[derive(Default)]
    struct FakeSystem {
        already_loaded: bool,
        binding_folder: Option<PathBuf>,
        requested_instance: Option<String>,
        /// `(instance, value)` to the string stored.
        instance_records: HashMap<(String, String), String>,
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

        fn with_record(mut self, instance: &str, value: &str, folder: &str) -> Self {
            self.instance_records.insert(
                (instance.to_string(), value.to_string()),
                folder.to_string(),
            );
            self
        }

        fn without_records(mut self) -> Self {
            self.instance_records.clear();
            self
        }

        fn requesting(mut self, instance: &str) -> Self {
            self.requested_instance = Some(instance.to_string());
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

        fn requested_instance(&self) -> Option<String> {
            self.requested_instance.clone()
        }

        fn instance_record_value(&self, instance: &str, value: &str) -> Option<String> {
            self.instance_records
                .get(&(instance.to_string(), value.to_string()))
                .cloned()
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

    /// A fully qualified Windows folder as a registry record holds it; the instance records are
    /// read on Windows only, so the tests spell them the Windows way on every host.
    fn recorded(name: &str) -> String {
        format!(r"C:\qaros-lookup-test\{name}")
    }

    fn recorded_runtime(name: &str) -> PathBuf {
        PathBuf::from(recorded(name)).join(FILE_NAME)
    }

    /// Every folder of the lookup holds a loadable runtime, and the development instance records
    /// one too; tests take away what they need.
    fn system_with_a_runtime_everywhere() -> FakeSystem {
        FakeSystem {
            already_loaded: true,
            binding_folder: Some(folder("binding")),
            installed_bin_folder: Some(folder("installed")),
            search_path_folders: Some(vec![folder("path-1")]),
            ..FakeSystem::default()
        }
        .with_file(&folder("binding"), FakeFile::Loads)
        .with_file(&folder("installed"), FakeFile::Loads)
        .with_file(&folder("path-1"), FakeFile::Loads)
        .with_record(
            DEV_INSTANCE,
            INSTANCE_RUNTIME_FOLDER_VALUE,
            &recorded("dev-build"),
        )
        .with_file(Path::new(&recorded("dev-build")), FakeFile::Loads)
    }

    /// Nothing loaded and nothing beside the binding: step 2 decides.
    fn system_from_step_two() -> FakeSystem {
        FakeSystem {
            already_loaded: false,
            binding_folder: None,
            ..system_with_a_runtime_everywhere()
        }
    }

    fn found_at(system: &FakeSystem) -> (LookupStep, PathBuf) {
        let found = find_runtime(system, FILE_NAME).expect("a runtime");
        let by_bare_name = found.step == LookupStep::AlreadyLoaded
            || found.library.starts_with(format!("linker:{FILE_NAME}"));
        let file_loaded = if by_bare_name {
            PathBuf::from(FILE_NAME)
        } else {
            found.library.clone()
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

    /// The last attempt, which ended the lookup at step 2.
    fn instance_outcome(system: &FakeSystem) -> LookupOutcome {
        let attempts = attempts_of(system);
        let steps: Vec<LookupStep> = attempts.iter().map(|attempt| attempt.step).collect();
        assert_eq!(
            steps,
            vec![
                LookupStep::AlreadyLoaded,
                LookupStep::BesideBinding,
                LookupStep::Instance
            ],
            "a set QAR_INSTANCE ends the lookup at step 2"
        );
        attempts.last().expect("step 2").outcome.clone()
    }

    fn attempt(step: LookupStep, outcome: LookupOutcome) -> LookupAttempt {
        LookupAttempt { step, outcome }
    }

    #[test]
    fn a_runtime_already_in_the_process_wins_and_nothing_is_loaded() {
        let system = system_with_a_runtime_everywhere().requesting(DEV_INSTANCE);
        assert_eq!(
            found_at(&system),
            (LookupStep::AlreadyLoaded, PathBuf::from("already loaded"))
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn the_runtime_beside_the_binding_wins_over_the_instance_and_the_installation() {
        let system = FakeSystem {
            already_loaded: false,
            ..system_with_a_runtime_everywhere().requesting(DEV_INSTANCE)
        };
        assert_eq!(
            found_at(&system),
            (LookupStep::BesideBinding, runtime_in("binding"))
        );
    }

    #[test]
    fn a_runtime_beside_the_binding_that_fails_to_load_is_reported_and_the_lookup_goes_on() {
        let mut system = FakeSystem {
            already_loaded: false,
            ..system_with_a_runtime_everywhere().requesting(DEV_INSTANCE)
        };
        system
            .files
            .insert(runtime_in("binding"), FakeFile::FailsToLoad);
        assert_eq!(
            found_at(&system),
            (LookupStep::Instance, recorded_runtime("dev-build"))
        );
        assert_eq!(
            *system.loads.borrow(),
            vec![runtime_in("binding"), recorded_runtime("dev-build")]
        );
    }

    #[test]
    fn the_runtime_folder_of_the_named_instance_wins_over_the_installation() {
        let system = system_from_step_two().requesting(DEV_INSTANCE);
        assert_eq!(
            found_at(&system),
            (LookupStep::Instance, recorded_runtime("dev-build"))
        );
    }

    #[test]
    fn the_default_instance_is_the_installed_qaros() {
        let system = system_from_step_two().requesting(DEFAULT_INSTANCE_NAME);
        assert_eq!(
            found_at(&system),
            (LookupStep::Instance, runtime_in("installed"))
        );
    }

    #[test]
    fn the_default_instance_without_an_installation_is_an_error_and_path_is_not_searched() {
        let system = FakeSystem {
            installed_bin_folder: None,
            ..system_from_step_two().requesting(DEFAULT_INSTANCE_NAME)
        };
        let outcome = instance_outcome(&system);
        assert_eq!(
            outcome,
            LookupOutcome::UnknownInstance {
                name: DEFAULT_INSTANCE_NAME.to_string()
            }
        );
        assert_eq!(
            attempt(LookupStep::Instance, outcome).to_string(),
            r"QAR_INSTANCE is 'default', but QAROS is not installed (no InstallFolder under HKLM\SOFTWARE\Quaternar\QAROS)"
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn a_named_instance_without_a_runtime_folder_falls_back_to_its_install_folders_bin() {
        let bin = PathBuf::from(recorded("registered")).join("bin");
        let system = system_from_step_two()
            .without_records()
            .requesting("registered")
            .with_record("registered", INSTALL_FOLDER_VALUE, &recorded("registered"))
            .with_file(&bin, FakeFile::Loads);
        assert_eq!(
            found_at(&system),
            (LookupStep::Instance, bin.join(FILE_NAME))
        );
    }

    #[test]
    fn the_runtime_folder_wins_over_the_install_folder_of_the_same_record() {
        let system = system_from_step_two().requesting(DEV_INSTANCE).with_record(
            DEV_INSTANCE,
            INSTALL_FOLDER_VALUE,
            &recorded("registered"),
        );
        assert_eq!(
            found_at(&system),
            (LookupStep::Instance, recorded_runtime("dev-build"))
        );
    }

    #[test]
    fn an_unknown_instance_is_an_error_naming_it_never_the_installed_runtime() {
        let system = system_from_step_two().requesting("nobody-recorded-this");
        let outcome = instance_outcome(&system);
        assert_eq!(
            outcome,
            LookupOutcome::UnknownInstance {
                name: "nobody-recorded-this".to_string()
            }
        );
        assert_eq!(
            attempt(LookupStep::Instance, outcome).to_string(),
            r"QAR_INSTANCE is 'nobody-recorded-this', but no runtime folder is recorded for that instance (no full-path RuntimeFolder or InstallFolder under HKCU\SOFTWARE\Quaternar\QAROS\Instances\nobody-recorded-this). Start its hub, or unset QAR_INSTANCE"
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn relative_or_drive_relative_recorded_folders_count_as_no_record() {
        for folder in [
            r"relative\bin",
            r"C:bin",
            r"\rooted-without-drive",
            "",
            "C:",
        ] {
            let system = system_from_step_two()
                .without_records()
                .requesting(DEV_INSTANCE)
                .with_record(DEV_INSTANCE, INSTANCE_RUNTIME_FOLDER_VALUE, folder)
                .with_record(DEV_INSTANCE, INSTALL_FOLDER_VALUE, folder);
            assert_eq!(
                instance_outcome(&system),
                LookupOutcome::UnknownInstance {
                    name: DEV_INSTANCE.to_string()
                },
                "{folder:?}"
            );
        }
    }

    #[test]
    fn drive_and_unc_folders_are_fully_qualified() {
        for folder in [r"C:\QAROS\bin", "d:/qaros/bin", r"\\server\share\bin"] {
            assert!(is_fully_qualified_folder(folder), "{folder:?}");
        }
        for folder in [
            r"relative\bin",
            r"C:bin",
            r"\rooted",
            "/usr/lib",
            "",
            r"1:\x",
        ] {
            assert!(!is_fully_qualified_folder(folder), "{folder:?}");
        }
    }

    #[test]
    fn an_invalid_instance_name_is_an_error_and_nothing_is_loaded() {
        let too_long = "a".repeat(INSTANCE_NAME_MAX_LENGTH + 1);
        for name in [
            "with space",
            r"..\escape",
            "dev/x",
            "d\u{e9}v",
            too_long.as_str(),
        ] {
            let system = system_from_step_two().requesting(name);
            let outcome = instance_outcome(&system);
            assert_eq!(
                outcome,
                LookupOutcome::InvalidInstanceName {
                    name: name.to_string()
                }
            );
            assert_eq!(
                attempt(LookupStep::Instance, outcome).to_string(),
                format!(
                    "QAR_INSTANCE='{name}' is not a QAROS instance name (1-64 of A-Z a-z 0-9 . _ -); no runtime is loaded"
                )
            );
            assert!(system.loads.borrow().is_empty());
        }
    }

    #[test]
    fn instance_names_are_short_portable_tokens() {
        let longest = "a".repeat(INSTANCE_NAME_MAX_LENGTH);
        for name in ["default", DEV_INSTANCE, "A.b_c-9", longest.as_str()] {
            assert!(is_valid_instance_name(name), "{name:?}");
        }
        let too_long = "a".repeat(INSTANCE_NAME_MAX_LENGTH + 1);
        for name in [
            "",
            "with space",
            r"..\escape",
            "d\u{e9}v",
            too_long.as_str(),
        ] {
            assert!(!is_valid_instance_name(name), "{name:?}");
        }
    }

    #[test]
    fn an_instance_folder_without_the_runtime_ends_the_lookup_before_the_installation() {
        let system = system_from_step_two()
            .without_records()
            .requesting(DEV_INSTANCE)
            .with_record(
                DEV_INSTANCE,
                INSTANCE_RUNTIME_FOLDER_VALUE,
                &recorded("empty"),
            );
        let outcome = instance_outcome(&system);
        assert_eq!(
            outcome,
            LookupOutcome::InstanceRuntimeNotLoadable {
                name: DEV_INSTANCE.to_string(),
                path: recorded_runtime("empty"),
                message: None,
            }
        );
        assert_eq!(
            attempt(LookupStep::Instance, outcome).to_string(),
            format!(
                "QAR_INSTANCE is '{DEV_INSTANCE}', but no loadable {FILE_NAME} is in its runtime folder '{}' ({FILE_NAME} does not exist there)",
                PathBuf::from(recorded("empty")).display()
            )
        );
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn an_instance_runtime_that_fails_to_load_ends_the_lookup() {
        let mut system = system_from_step_two().requesting(DEV_INSTANCE);
        system
            .files
            .insert(recorded_runtime("dev-build"), FakeFile::FailsToLoad);
        let outcome = instance_outcome(&system);
        assert_eq!(
            outcome,
            LookupOutcome::InstanceRuntimeNotLoadable {
                name: DEV_INSTANCE.to_string(),
                path: recorded_runtime("dev-build"),
                message: Some("a dependent DLL is missing".to_string()),
            }
        );
        let line = attempt(LookupStep::Instance, outcome).to_string();
        assert!(
            line.ends_with(&format!(
                "({FILE_NAME} failed to load: a dependent DLL is missing)"
            )),
            "{line}"
        );
        assert_eq!(*system.loads.borrow(), vec![recorded_runtime("dev-build")]);
    }

    #[test]
    fn without_the_variable_the_installation_wins_over_path() {
        let system = system_from_step_two();
        assert_eq!(
            found_at(&system),
            (LookupStep::InstalledRuntime, runtime_in("installed"))
        );
    }

    #[test]
    fn an_installation_without_its_runtime_is_reported_and_path_is_not_searched() {
        let system = FakeSystem {
            installed_bin_folder: Some(folder("broken-installation")),
            ..system_from_step_two()
        };
        let attempts = attempts_of(&system);
        assert_eq!(
            attempts.last(),
            Some(&attempt(
                LookupStep::InstalledRuntime,
                LookupOutcome::Missing {
                    path: runtime_in("broken-installation")
                }
            ))
        );
        let line = attempts.last().expect("step 3").to_string();
        assert!(
            line.starts_with(&format!(
                "the installed QAROS: the installed QAROS runtime in '{}' could not be loaded",
                folder("broken-installation").display()
            )),
            "{line}"
        );
        assert!(line.ends_with("Repair or reinstall QAROS"), "{line}");
        assert!(system.loads.borrow().is_empty());
    }

    #[test]
    fn an_installed_runtime_that_fails_to_load_is_reported_and_path_is_not_searched() {
        let mut system = system_from_step_two();
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
                attempt(LookupStep::Instance, LookupOutcome::NotApplicable),
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
        let expected_instance_line = if cfg!(windows) {
            "QAR_INSTANCE: not set"
        } else {
            "QAR_INSTANCE: read on Windows only (there are no instance records here)"
        };
        assert_eq!(lines[2], expected_instance_line);
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
    fn the_steps_print_their_place() {
        assert_eq!(LookupStep::Instance.to_string(), "QAR_INSTANCE");
        assert_eq!(LookupStep::BesideBinding.to_string(), "beside the binding");
        assert_eq!(
            LookupStep::InstalledRuntime.to_string(),
            "the installed QAROS"
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
