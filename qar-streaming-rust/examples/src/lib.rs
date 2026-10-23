//! Helpers shared by the example programs, the Rust counterpart of the C
//! examples' `common.h`.
//!
//! The runtime is found with `Library::load_default`, as the C examples' `qar_library_load(NULL)`
//! does: the installed QAROS, or the folder `QAROS_RUNTIME_DIR` names (see the README). An
//! explicit library file, for a development build of QAROS or the `qaros-stub` library cargo
//! builds in `target/debug/`, comes from `--qaros-library=PATH`, else `QAROS_DLL`, else the first
//! line of `qaros-library.txt` beside the executable. `RUST_LOG`-style filtering is not provided;
//! warnings from the wrapper go to stderr.

use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use qaros::qaros_sys::LookupStep;
use qaros::{
    BlockingOptions, Library, LibraryConfig, OnboardMethod, OnboardingId, PeerInfo,
    PeerPresentation, Pose, Progress, Runtime, RuntimeConfig, Session, StatusCode,
};

pub type ExampleResult<T = ()> = Result<T, Box<dyn Error>>;

/// Flags that take a value, written either `--name=value` or as `--name`
/// followed by the value as the next argument (the visualizer's argument
/// list puts one argument per line, so both spellings occur).
const FLAGS_WITH_VALUE: [&str; 2] = ["--qaros-library", "--frames"];

/// Command-line arguments shared by the examples: an optional pairing code
/// (first run only) and flags starting with `--`.
#[derive(Debug, Default)]
pub struct Arguments {
    pub pairing_code: Option<String>,
    pub flags: Vec<String>,
}

impl Arguments {
    pub fn parse() -> Self {
        Self::from_arguments(std::env::args().skip(1))
    }

    pub fn from_arguments(command_line: impl IntoIterator<Item = String>) -> Self {
        let mut arguments = Self::default();
        let mut command_line = command_line.into_iter();
        while let Some(argument) = command_line.next() {
            if FLAGS_WITH_VALUE.contains(&argument.as_str()) {
                let value = command_line.next().unwrap_or_default();
                arguments.flags.push(format!("{argument}={value}"));
            } else if argument.starts_with("--") {
                arguments.flags.push(argument);
            } else if arguments.pairing_code.is_none() {
                arguments.pairing_code = Some(argument);
            }
        }
        arguments
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|candidate| candidate == flag)
    }

    /// Value of `--name=value`.
    pub fn flag_value(&self, name: &str) -> Option<&str> {
        self.flags
            .iter()
            .find_map(|flag| flag.strip_prefix(name)?.strip_prefix('='))
    }
}

/// Name of the optional file beside the executable whose first line is the
/// library path, for a start that passes no arguments or environment.
const LIBRARY_PATH_FILE: &str = "qaros-library.txt";

// ANCHOR: explicit_library_path
/// The explicit library file: `--qaros-library=PATH`, else `QAROS_DLL`, else the first line of
/// `qaros-library.txt` beside the executable. `None` loads the runtime with the default lookup.
pub fn explicit_library_path() -> Option<PathBuf> {
    if let Some(path) = Arguments::parse().flag_value("--qaros-library") {
        return Some(PathBuf::from(path));
    }
    if let Some(path) = std::env::var_os("QAROS_DLL") {
        return Some(PathBuf::from(path));
    }
    library_path_from_file()
}
// ANCHOR_END: explicit_library_path

fn library_path_from_file() -> Option<PathBuf> {
    let executable = std::env::current_exe().ok()?;
    let file = executable.parent()?.join(LIBRARY_PATH_FILE);
    let contents = match std::fs::read_to_string(&file) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
        Err(error) => {
            eprintln!("warning: cannot read {}: {error}", file.display());
            return None;
        }
    };
    let first_line = contents.lines().next()?.trim();
    (!first_line.is_empty()).then(|| PathBuf::from(first_line))
}

/// The runtime binaries folder stays empty, so the runtime uses the folder it was loaded from.
pub fn runtime_config(storage_directory: Option<&Path>) -> RuntimeConfig {
    RuntimeConfig {
        runtime_binaries_directory: None,
        storage_directory: storage_directory.map(Path::to_path_buf),
    }
}

// ANCHOR: load_library
/// Loads and initializes the library, printing what it does like the C examples: the explicit
/// library file when one is given ([`explicit_library_path`]), else the runtime the default
/// lookup finds.
#[cfg(not(feature = "link"))]
pub fn load_library() -> ExampleResult<Library> {
    install_stderr_logger();
    let library = if let Some(path) = explicit_library_path() {
        println!("Loading qar-streaming-c from: {}", path.display());
        // SAFETY: the user named a qar-streaming-c build; a mismatching library is the user's
        // configuration error, as in the C examples.
        unsafe { Library::load(&path, &LibraryConfig::default()) }?
    } else {
        println!("Loading the QAROS runtime (default lookup)");
        // SAFETY: the runtime found is the installed QAROS or one the user chose with
        // QAROS_RUNTIME_DIR; a mismatching one is a configuration error, as in the C examples.
        unsafe { Library::load_default(&LibraryConfig::default()) }.map_err(|error| match error {
            qaros::Error::Load(qaros::qaros_sys::LoadError::NotFound { .. }) => format!(
                "{error}\nInstall QAROS, set QAROS_RUNTIME_DIR to the folder of qar-streaming-c.dll, or pass --qaros-library=PATH (the qaros-stub library in target/debug/ runs without QAROS)."
            ),
            other => other.to_string(),
        })?
    };
    println!("Loaded {}", describe_loaded_runtime(&library));
    Ok(library)
}
// ANCHOR_END: load_library

/// The file the runtime came from and, after the default lookup, the step that found it.
pub fn describe_loaded_runtime(library: &Library) -> String {
    let file = library.library_path().map_or_else(
        || "qar-streaming-c".to_string(),
        |path| path.display().to_string(),
    );
    match library.lookup_step() {
        Some(step) => format!("{file} ({})", describe_lookup_step(step)),
        None => file,
    }
}

fn describe_lookup_step(step: LookupStep) -> &'static str {
    match step {
        LookupStep::AlreadyLoaded => "already loaded in this process",
        LookupStep::BesideBinding => "beside the binding",
        LookupStep::RuntimeDirVariable => "QAROS_RUNTIME_DIR",
        LookupStep::InstalledRuntime => "the installed QAROS",
        LookupStep::SearchPath if cfg!(windows) => "PATH",
        LookupStep::SearchPath => "the dynamic linker's search",
    }
}

/// Initializes the import-linked library (feature `link`).
#[cfg(feature = "link")]
pub fn load_library() -> ExampleResult<Library> {
    install_stderr_logger();
    println!("Using the import-linked qar-streaming-c");
    Ok(Library::linked(&LibraryConfig::default())?)
}

pub fn create_runtime(
    library: &Library,
    storage_directory: Option<&Path>,
) -> ExampleResult<Runtime> {
    let runtime = library.create_runtime(&runtime_config(storage_directory))?;
    println!("qar_runtime_create: SUCCESS");
    Ok(runtime)
}

// ANCHOR: onboarding_id_file
/// The onboarding id a previous run stored in `id_file`, if it holds a readable one.
pub fn load_onboarding_id(library: &Library, id_file: &Path) -> Option<OnboardingId> {
    let text = std::fs::read_to_string(id_file).ok()?;
    match library.uuid_from_string(text.trim()) {
        Ok(bytes) => Some(OnboardingId(bytes)),
        Err(error) => {
            println!(
                "Ignoring unreadable onboarding id in '{}': {error}",
                id_file.display()
            );
            None
        }
    }
}

pub fn save_onboarding_id(
    library: &Library,
    id_file: &Path,
    onboarding_id: OnboardingId,
) -> ExampleResult {
    let text = library.uuid_to_string(&onboarding_id.0)?;
    std::fs::write(id_file, format!("{text}\n"))?;
    Ok(())
}
// ANCHOR_END: onboarding_id_file

// ANCHOR: obtain_session
/// The canonical startup of an app that onboards with a pairing code: rejoin with the persisted
/// onboarding id, otherwise onboard with the code shown on the hub and persist the new id.
pub fn obtain_session(
    library: &Library,
    runtime: &Runtime,
    pairing_code: Option<&str>,
    id_file: &Path,
    display_name: &str,
) -> ExampleResult<(OnboardingId, Session)> {
    let presentation = presentation(display_name);
    if let Some(rejoined) = rejoin_with_stored_id(library, runtime, id_file, &presentation)? {
        return Ok(rejoined);
    }
    let Some(pairing_code) = pairing_code else {
        return Err(missing_pairing_code(id_file));
    };
    onboard_with_code(library, runtime, pairing_code, id_file, &presentation)
}

fn presentation(display_name: &str) -> PeerPresentation {
    PeerPresentation {
        display_name: Some(display_name.to_string()),
        ..PeerPresentation::default()
    }
}

/// `None` when `id_file` holds no id, or the hub has no session for it any more.
fn rejoin_with_stored_id(
    library: &Library,
    runtime: &Runtime,
    id_file: &Path,
    presentation: &PeerPresentation,
) -> ExampleResult<Option<(OnboardingId, Session)>> {
    let Some(saved_id) = load_onboarding_id(library, id_file) else {
        return Ok(None);
    };
    match runtime.rejoin(saved_id, presentation, &BlockingOptions::default()) {
        Ok(session) => {
            println!("qar_runtime_rejoin: SUCCESS");
            Ok(Some((saved_id, session)))
        }
        Err(error) if error.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND) => {
            println!("qar_runtime_rejoin: {error}; onboarding instead");
            Ok(None)
        }
        Err(error) => Err(error.into()),
    }
}

fn onboard_with_code(
    library: &Library,
    runtime: &Runtime,
    pairing_code: &str,
    id_file: &Path,
    presentation: &PeerPresentation,
) -> ExampleResult<(OnboardingId, Session)> {
    let onboarded = runtime
        .onboard(
            &OnboardMethod::Code {
                code: pairing_code,
                hub: None,
            },
            presentation,
            &BlockingOptions::default(),
        )
        .map_err(|error| -> Box<dyn Error> {
            if error.has_code(StatusCode::PAKE_ERROR) {
                format!("{error}\nPairing code rejected or expired: read the current code off the hub screen and retry.")
                    .into()
            } else {
                error.into()
            }
        })?;
    println!("qar_runtime_onboard: SUCCESS");
    save_onboarding_id(library, id_file, onboarded.onboarding_id)?;
    Ok((onboarded.onboarding_id, onboarded.session))
}

fn missing_pairing_code(id_file: &Path) -> Box<dyn Error> {
    format!(
        "No persisted onboarding id in '{}'. Pass the pairing code shown on the hub's onboarding screen.",
        id_file.display()
    )
    .into()
}
// ANCHOR_END: obtain_session

// ANCHOR: join_session
/// The startup of the CPU, D3D11, GUI panel and app volume examples: rejoin with the persisted
/// onboarding id, else onboard with the pairing code when one is passed (as `obtain_session`),
/// else, on Windows, join the Hub on this machine as a local app. Their C originals onboard by code
/// only.
pub fn join_session(
    library: &Library,
    runtime: &Runtime,
    pairing_code: Option<&str>,
    id_file: &Path,
    display_name: &str,
) -> ExampleResult<Session> {
    let presentation = presentation(display_name);
    if let Some((_, session)) = rejoin_with_stored_id(library, runtime, id_file, &presentation)? {
        return Ok(session);
    }
    if let Some(pairing_code) = pairing_code {
        let (_, session) =
            onboard_with_code(library, runtime, pairing_code, id_file, &presentation)?;
        return Ok(session);
    }
    if cfg!(windows) {
        join_as_local_app(runtime, &presentation)
    } else {
        Err(missing_pairing_code(id_file))
    }
}
// ANCHOR_END: join_session

// ANCHOR: join_as_local_app
/// How long the first run of an executable waits for someone to click Allow in QAROS.
const LOCAL_APP_APPROVAL_TIMEOUT: Duration = Duration::from_secs(120);

/// Joins this machine's Hub with `OnboardMethod::LocalApp`, as `local_app_onboarding` does. The
/// join is stateless: its onboarding id is the zero id, so nothing is stored and every run joins
/// this way again.
fn join_as_local_app(runtime: &Runtime, presentation: &PeerPresentation) -> ExampleResult<Session> {
    println!(
        "No session to rejoin and no pairing code: joining the QAROS Hub on this machine as a local app. On the first run of this executable, click Allow in the QAROS approval prompt (waiting up to {} s).",
        LOCAL_APP_APPROVAL_TIMEOUT.as_secs()
    );
    let (progress, progress_reports) = std::sync::mpsc::channel::<Progress>();
    let printer = std::thread::spawn(move || {
        for report in progress_reports {
            if !report.message.is_empty() {
                println!("{}", report.message);
            }
        }
    });
    let joined = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: Some(LOCAL_APP_APPROVAL_TIMEOUT),
        },
        presentation,
        &BlockingOptions {
            progress: Some(progress),
            cancel: None,
        },
    );
    if printer.join().is_err() {
        eprintln!("the progress printer panicked");
    }
    let onboarded = joined.map_err(explain_local_app_refusal)?;
    println!("qar_runtime_onboard: SUCCESS (local app)");
    println!(
        "A local app has no onboarding id to store (it is zero): the next run joins as a local app again, without a prompt while this executable is unchanged."
    );
    Ok(onboarded.session)
}

/// A failed local-app join, with the exit code `local_app_onboarding` (and its C original) uses
/// for it; [`report`] exits with that code.
#[derive(Debug)]
pub struct LocalAppRefusal {
    pub exit_code: u8,
    explanation: String,
}

impl std::fmt::Display for LocalAppRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.explanation)
    }
}

impl Error for LocalAppRefusal {}

fn explain_local_app_refusal(error: qaros::Error) -> Box<dyn Error> {
    let explained = [
        (
            StatusCode::ONBOARDING_HUB_UNREACHABLE,
            3,
            "No QAROS hub is running on this PC. Start QAROS (tray icon) and try again.",
        ),
        (
            StatusCode::ONBOARDING_APPROVAL_DENIED,
            4,
            "QAROS did not allow this app: the approval was denied or revoked.",
        ),
        (
            StatusCode::ONBOARDING_APPROVAL_TIMEOUT,
            5,
            "Nobody answered the approval prompt in QAROS in time. Start the app again.",
        ),
        (
            StatusCode::ONBOARDING_HUB_NOT_AUTHENTIC,
            6,
            "The program answering on this PC is not the installed QAROS. Reinstall QAROS.",
        ),
        (
            StatusCode::ONBOARDING_APP_IDENTITY_MISMATCH,
            7,
            "This app no longer matches what QAROS approved. Revoke it in QAROS and approve it again.",
        ),
    ];
    for (code, exit_code, explanation) in explained {
        if error.has_code(code) {
            return Box::new(LocalAppRefusal {
                exit_code,
                explanation: format!("{error}\n{explanation}"),
            });
        }
    }
    error.into()
}
// ANCHOR_END: join_as_local_app

/// `--frames=N`, else `default_frame_count`.
pub fn frame_limit(arguments: &Arguments, default_frame_count: usize) -> ExampleResult<usize> {
    match arguments.flag_value("--frames") {
        Some(text) => Ok(text.parse::<usize>()?),
        None => Ok(default_frame_count),
    }
}

// ANCHOR: is_running_target
/// A target app to render for: its app state is `AppState::Running`. The public header deprecates
/// the app state for the peer presence (`qar_peer_spec_get_presence`) and computes it from the
/// presence: an offline or leaving peer reads `ShuttingDown`, a retired one `Retired`, an online
/// or not yet reconciled one what it claimed, and every current peer claims `Running` once it
/// joins. Running is then the presence's online or unknown, the targets the guide renders for.
pub fn is_running_target(peer: &PeerInfo) -> bool {
    peer.is_target_app && peer.app_state == qaros::AppState::Running
}
// ANCHOR_END: is_running_target

// ANCHOR: is_waiting
/// A stream that is paused, reconnecting or has no transfer yet is not broken: keep the sender
/// and try the next frame. Like the C cube's `IsWaiting` this matches a wrapped cause too
/// (begin-frame reports these inside `RENDERING_UNABLE_TO_BEGIN_FRAME`). An app volume that
/// does not resolve yet is waiting as well: the header calls it recoverable. Any other error
/// ends the sender.
pub fn is_waiting(error: &qaros::Error) -> bool {
    [
        StatusCode::MEDIA_STREAMING_STREAM_IS_PAUSED,
        StatusCode::MEDIA_STREAMING_STREAM_IS_RECONNECTING,
        StatusCode::MEDIA_STREAMING_NO_ACTIVE_TRANSFERS,
        StatusCode::RENDERING_APP_VOLUME_UNRESOLVED,
    ]
    .into_iter()
    .any(|code| error.has_cause(code))
}
// ANCHOR_END: is_waiting

// ANCHOR: updates_until_quiet
/// The Hub delivers the first update on its own schedule, which can take a few seconds after the
/// subscription. Ten seconds leaves room for that and still ends a run in which no update comes.
pub const FIRST_UPDATE_TIMEOUT: Duration = Duration::from_secs(10);
/// After the first update, how long the wait lasts without one more.
pub const UPDATE_QUIET_PERIOD: Duration = Duration::from_millis(500);

/// The updates `updates` receives: the first within [`FIRST_UPDATE_TIMEOUT`], then each next one
/// until none has arrived for [`UPDATE_QUIET_PERIOD`]. Empty when no update arrived in time.
pub fn updates_until_quiet<Update>(
    updates: &Receiver<Update>,
) -> impl Iterator<Item = Update> + '_ {
    let mut deadline = Instant::now() + FIRST_UPDATE_TIMEOUT;
    std::iter::from_fn(move || {
        let update = updates
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .ok()?;
        deadline = Instant::now() + UPDATE_QUIET_PERIOD;
        Some(update)
    })
}
// ANCHOR_END: updates_until_quiet

// ANCHOR: same_pose
/// The Hub reports a value back that can differ from the one you set, and from its own earlier
/// report, below the precision anyone sees. Compared exactly, such an echo looks like a change.
/// So compare at the precision your app cares about. These examples print centimeters; a
/// millimeter is far below what a viewer notices.
pub const SAME_LENGTH_METERS: f32 = 0.001;
/// For each component of a unit quaternion: 0.0001 is about a hundredth of a degree.
pub const SAME_ROTATION: f32 = 0.0001;
/// For a factor such as a scale: one part in a thousand.
pub const SAME_FACTOR: f32 = 0.001;

pub fn nearly_equal(first: f32, second: f32, tolerance: f32) -> bool {
    (first - second).abs() <= tolerance
}

/// Whether two poses are the same at the precision above.
pub fn same_pose(first: &Pose, second: &Pose) -> bool {
    nearly_equal(first.position.x, second.position.x, SAME_LENGTH_METERS)
        && nearly_equal(first.position.y, second.position.y, SAME_LENGTH_METERS)
        && nearly_equal(first.position.z, second.position.z, SAME_LENGTH_METERS)
        && nearly_equal(first.orientation.x, second.orientation.x, SAME_ROTATION)
        && nearly_equal(first.orientation.y, second.orientation.y, SAME_ROTATION)
        && nearly_equal(first.orientation.z, second.orientation.z, SAME_ROTATION)
        && nearly_equal(first.orientation.w, second.orientation.w, SAME_ROTATION)
}
// ANCHOR_END: same_pose

/// Prints `main`'s error and exits non-zero: with a [`LocalAppRefusal`]'s own code, else 1.
pub fn report(result: ExampleResult) -> std::process::ExitCode {
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            let exit_code = error
                .downcast_ref::<LocalAppRefusal>()
                .map_or(1, |refusal| refusal.exit_code);
            std::process::ExitCode::from(exit_code)
        }
    }
}

struct StderrLogger;

impl log::Log for StderrLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: StderrLogger = StderrLogger;

fn install_stderr_logger() {
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(log::LevelFilter::Info);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use qaros::{Pose, Quat, Vec3};

    use super::{Arguments, UPDATE_QUIET_PERIOD, same_pose, updates_until_quiet};

    fn parsed(command_line: &[&str]) -> Arguments {
        Arguments::from_arguments(command_line.iter().map(ToString::to_string))
    }

    #[test]
    fn library_path_with_equals_sign() {
        let arguments = parsed(&[
            "--forget",
            r"--qaros-library=D:\package\bin\qar-streaming-c.dll",
        ]);
        assert_eq!(
            arguments.flag_value("--qaros-library"),
            Some(r"D:\package\bin\qar-streaming-c.dll")
        );
        assert!(arguments.has_flag("--forget"));
        assert_eq!(arguments.pairing_code, None);
    }

    #[test]
    fn library_path_as_the_next_argument_is_not_a_pairing_code() {
        let arguments = parsed(&[
            "--qaros-library",
            r"D:\package\bin\qar-streaming-c.dll",
            "--forget",
        ]);
        assert_eq!(
            arguments.flag_value("--qaros-library"),
            Some(r"D:\package\bin\qar-streaming-c.dll")
        );
        assert!(arguments.has_flag("--forget"));
        assert_eq!(arguments.pairing_code, None);
    }

    #[test]
    fn frames_as_the_next_argument_and_a_typed_code() {
        let arguments = parsed(&["123456", "--frames", "60"]);
        assert_eq!(arguments.flag_value("--frames"), Some("60"));
        assert_eq!(arguments.pairing_code.as_deref(), Some("123456"));
    }

    #[test]
    fn the_first_update_may_come_later_than_the_quiet_period() {
        let (sender, receiver) = mpsc::channel();
        let late_sender = thread::spawn(move || {
            thread::sleep(UPDATE_QUIET_PERIOD + Duration::from_millis(300));
            sender.send("first").ok();
            sender.send("second").ok();
            thread::sleep(UPDATE_QUIET_PERIOD * 3);
        });
        let delivered_updates: Vec<&str> = updates_until_quiet(&receiver).collect();
        assert_eq!(delivered_updates, ["first", "second"]);
        late_sender.join().ok();
    }

    #[test]
    fn a_closed_channel_ends_the_updates() {
        let (sender, receiver) = mpsc::channel::<u32>();
        drop(sender);
        assert_eq!(updates_until_quiet(&receiver).count(), 0);
    }

    #[test]
    fn poses_within_a_millimeter_are_the_same() {
        let pose = Pose::from_position(Vec3::new(0.2, 1.6, -1.0));
        let echo = Pose::from_position(Vec3::new(0.2004, 1.6, -1.0009));
        let moved = Pose::from_position(Vec3::new(0.202, 1.6, -1.0));
        assert!(same_pose(&pose, &echo));
        assert!(!same_pose(&pose, &moved));
    }

    #[test]
    fn a_rotation_beyond_the_tolerance_is_a_change() {
        let pose = Pose::default();
        let mut turned = pose;
        turned.orientation = Quat {
            x: 0.0,
            y: 0.001,
            z: 0.0,
            w: 1.0,
        };
        let mut echo = pose;
        echo.orientation.y = 0.00005;
        assert!(!same_pose(&pose, &turned));
        assert!(same_pose(&pose, &echo));
    }
}
