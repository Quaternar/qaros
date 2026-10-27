//! Port of `local_app_onboarding.c`: the smallest source app that joins the QAROS installed on
//! this PC as a local app. No code: QAROS asks the user once to allow the app, and an app tag
//! keeps its peer id across restarts.
//!
//! ```text
//! cargo run -p qaros-examples --bin local_app_onboarding [-- approval-timeout-ms [app-tag]]
//! ```
//!
//! It loads the runtime with the default lookup (`Library::load_default`, the C sample's
//! `qar_library_load(NULL)`) and joins with `OnboardMethod::LocalApp`. The first run opens
//! "Allow `local_app_onboarding.exe` to join QAROS?" in the QAROS Visualizer or as a QAROS tray
//! window; later runs join silently while the executable is unchanged, and print the same peer id
//! while no other instance with the same tag runs. Without a timeout, or with 0, the hub's
//! default applies. The tag is `main.local-app-onboarding.examples.quaternar` when absent.
//!
//! Exit codes, as in the C sample:
//!
//! | Code | Meaning |
//! |---|---|
//! | 0 | joined |
//! | 1 | other error |
//! | 2 | QAROS is not installed (runtime not loaded) |
//! | 3 | no QAROS hub running on this PC (1802) |
//! | 4 | approval denied or revoked (1805) |
//! | 5 | nobody answered the approval in time (1806) |
//! | 6 | the hub is not the installed QAROS (1807) |
//! | 7 | the app no longer matches its approval (1808) |

use std::process::ExitCode;
use std::time::Duration;

use qaros::{
    BlockingOptions, Library, LibraryConfig, OnboardMethod, PeerPresentation, Progress,
    RuntimeConfig, Session, StatusCode,
};

/// An app tag is an identity key: printable ASCII, constant across versions, scoped by the app
/// (DNS style).
const DEFAULT_APP_TAG: &str = "main.local-app-onboarding.examples.quaternar";

fn main() -> ExitCode {
    let approval_timeout = match parse_approval_timeout(std::env::args().nth(1).as_deref()) {
        Ok(timeout) => timeout,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };
    let app_tag = std::env::args()
        .nth(2)
        .unwrap_or_else(|| DEFAULT_APP_TAG.to_string());

    // ANCHOR: load
    // SAFETY: the runtime found is the installed QAROS (or that of the instance QAR_INSTANCE
    // names), which matches these bindings.
    let loaded = unsafe {
        Library::load_default(&LibraryConfig {
            console_logging: false,
            ..LibraryConfig::default()
        })
    };
    let library = match loaded {
        Ok(library) => library,
        Err(qaros::Error::Load(load_error)) => {
            eprintln!("QAROS is not installed on this PC.\n{load_error}");
            return ExitCode::from(2);
        }
        Err(error) => {
            eprintln!("The QAROS runtime failed to initialize: {error}");
            return ExitCode::from(1);
        }
    };
    // ANCHOR_END: load

    // ANCHOR: runtime
    let runtime = match library.create_runtime(&RuntimeConfig::default()) {
        Ok(runtime) => runtime,
        Err(error) => {
            println!("qar_runtime_create: ERROR {error}");
            return ExitCode::from(1);
        }
    };
    // ANCHOR_END: runtime

    // ANCHOR: onboard
    let (progress, progress_reports) = std::sync::mpsc::channel::<Progress>();
    let printer = std::thread::spawn(move || {
        for report in progress_reports {
            if !report.message.is_empty() {
                println!("{}", report.message);
            }
        }
    });
    let presentation = PeerPresentation {
        display_name: Some("Local App Onboarding Example".to_string()),
        ..PeerPresentation::default()
    };
    let joined = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout,
            // Optional: without it the app has the default tag.
            app_tag: Some(&app_tag),
        },
        &presentation,
        &BlockingOptions {
            progress: Some(progress),
            cancel: None,
        },
    );
    if printer.join().is_err() {
        eprintln!("the progress printer panicked");
    }
    // ANCHOR_END: onboard
    // ANCHOR: teardown
    let exit_code = explain_result(joined.as_ref().map(|_| ()));
    if let Ok(onboarded) = &joined {
        // The same tag prints the same peer id on the next start.
        print_my_peer_id(&onboarded.session);
    }
    // The session, the runtime and the library are destroyed in that order as they drop.
    drop(joined);
    drop(runtime);
    drop(library);
    ExitCode::from(exit_code)
    // ANCHOR_END: teardown
}

/// Prints the peer id the hub gave this app. Joining succeeded either way, so a failure here is
/// only reported.
fn print_my_peer_id(session: &Session) {
    let peer = session
        .my_peer()
        .and_then(|me| session.library().uuid_to_string(&me.id.0));
    match peer {
        Ok(text) => println!("This app joined as peer {text}"),
        Err(error) => eprintln!("Could not read this app's peer id: {error}"),
    }
}

// ANCHOR: timeout
/// The optional first argument in milliseconds; absent or 0 is the hub's default.
fn parse_approval_timeout(argument: Option<&str>) -> Result<Option<Duration>, String> {
    let Some(text) = argument else {
        return Ok(None);
    };
    let milliseconds: u32 = text.parse().map_err(|error| {
        format!("approval timeout '{text}' is not a number of milliseconds: {error}")
    })?;
    Ok((milliseconds > 0).then(|| Duration::from_millis(u64::from(milliseconds))))
}
// ANCHOR_END: timeout

// ANCHOR: explain
/// Maps the onboarding result to the documented exit code and explains it.
fn explain_result(joined: Result<(), &qaros::Error>) -> u8 {
    let error = match joined {
        Ok(()) => {
            println!("Joined QAROS.");
            return 0;
        }
        Err(error) => error,
    };
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
            eprintln!("{explanation}");
            return exit_code;
        }
    }
    println!("qar_runtime_onboard: ERROR {error}");
    1
}
// ANCHOR_END: explain

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn no_argument_and_zero_ask_for_the_hubs_default_timeout() {
        assert_eq!(parse_approval_timeout(None), Ok(None));
        assert_eq!(parse_approval_timeout(Some("0")), Ok(None));
        assert_eq!(
            parse_approval_timeout(Some("90000")),
            Ok(Some(Duration::from_secs(90)))
        );
    }

    #[test]
    fn a_timeout_that_is_not_a_number_is_refused() {
        let refusal = parse_approval_timeout(Some("soon")).expect_err("not a number");
        assert!(
            refusal.starts_with("approval timeout 'soon' is not a number"),
            "{refusal}"
        );
    }
}
