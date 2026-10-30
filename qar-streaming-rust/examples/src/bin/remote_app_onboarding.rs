//! Port of `remote_app_onboarding.c`: an app on the primary hub's PC grants one app on another
//! PC. The invitation joins that PC's hub as a member and admits the app there with no prompt,
//! under a peer id the primary remembers.
//!
//! ```text
//! # On the primary hub's PC (the app's approval needs "May bring in member hubs"):
//! cargo run -p qaros-examples --bin remote_app_onboarding -- issue | <your channel>
//! # On the other PC, first start:
//! <your channel> | cargo run -p qaros-examples --bin remote_app_onboarding -- join
//! # On the other PC, every later start:
//! cargo run -p qaros-examples --bin remote_app_onboarding -- start
//! ```
//!
//! `issue` writes the invitation to stdout and refuses to print it on a console; its messages go
//! to stderr. Exit codes, as in the C sample: 0 done, 1 error, 2 QAROS is not installed.

use std::process::ExitCode;

mod remote_app {
    use std::io::{IsTerminal, Read, Write};
    use std::process::ExitCode;
    use std::sync::mpsc;

    use qaros::{
        BlockingOptions, Error, JoinHubAsMemberConfig, Library, LibraryConfig, MemberHubInvitation,
        MemberHubInvitationConfig, OnboardExtensions, OnboardMethod, PeerPresentation, Progress,
        Result, Runtime, RuntimeConfig, Session, StatusCode,
    };

    /// The issuing app's own tag on the primary, and the tag the grant admits the remote app
    /// under. One grant admits one app: issue one tag per remote app.
    const ISSUER_TAG: &str = "server.remote-app-onboarding.examples.quaternar";
    const GRANT_TAG: &str = "client.remote-app-onboarding.examples.quaternar";
    /// A serialized invitation is at most 16 KiB.
    const MAX_INVITATION_BYTES: u64 = 16 * 1024;

    pub(crate) fn main(role: &str) -> ExitCode {
        // SAFETY: the runtime found is the installed QAROS (or that of the instance QAR_INSTANCE
        // names), which matches these bindings.
        let loaded = unsafe {
            Library::load_default(&LibraryConfig {
                console_logging: false, // keep stdout for the invitation
                ..LibraryConfig::default()
            })
        };
        let library = match loaded {
            Ok(library) => library,
            Err(Error::Load(load_error)) => {
                eprintln!("QAROS is not installed on this PC.\n{load_error}");
                return ExitCode::from(2);
            }
            Err(error) => {
                eprintln!("qar_library_init: {error}");
                return ExitCode::from(1);
            }
        };
        let runtime = match library.create_runtime(&RuntimeConfig::default()) {
            Ok(runtime) => runtime,
            Err(error) => {
                eprintln!("qar_runtime_create: {error}");
                return ExitCode::from(1);
            }
        };
        let done = match role {
            "issue" => issue(&runtime),
            "join" => join(&library, &runtime),
            _ => start(&runtime),
        };
        match done {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(1)
            }
        }
    }

    /// A local onboard; `grant` redeems a remote-app grant, whose tag then applies.
    fn onboard_local(
        runtime: &Runtime,
        tag: Option<&str>,
        grant: Option<&MemberHubInvitation>,
    ) -> Result<Session> {
        let (progress, reports) = mpsc::channel::<Progress>();
        let printer = std::thread::spawn(move || {
            for report in reports {
                eprintln!("{}", report.message);
            }
        });
        let onboarded = runtime.onboard_with(
            &OnboardMethod::LocalApp {
                approval_timeout: None,
                app_tag: if grant.is_some() { None } else { tag },
            },
            &PeerPresentation {
                display_name: Some("Remote App Onboarding Example".to_string()),
                ..PeerPresentation::default()
            },
            &OnboardExtensions {
                remote_app_grant: grant, // borrowed for the call
                ..OnboardExtensions::default()
            },
            &BlockingOptions {
                progress: Some(progress),
                cancel: None,
            },
        );
        // The options, and with them the sender, are gone: the printer ends.
        let _ = printer.join();
        // A local app's onboarding id is zero: nothing to store.
        Ok(onboarded?.session)
    }

    fn print_own_peer(session: &Session) -> Result<()> {
        let peer = session.my_peer()?;
        eprintln!("Remote app joined as peer {}", peer.id);
        Ok(())
    }

    /// Role "issue", on the primary hub's PC.
    fn issue(runtime: &Runtime) -> Result<()> {
        if std::io::stdout().is_terminal() {
            return Err(Error::InvalidArgument(
                "refusing to print the invitation on a console: pipe stdout into your channel"
                    .to_string(),
            ));
        }
        // The issuing app normally is in the session anyway; joining first also lets the user
        // approve it.
        let session = onboard_local(runtime, Some(ISSUER_TAG), None)?;

        // ANCHOR: issue
        let config = MemberHubInvitationConfig {
            label: Some("Remote app example".to_string()), // shown to the primary's operator
            app_grant_tag: Some(GRANT_TAG.to_string()),
            // Defaults: valid 24 h, one hub. Keep both as small as your flow allows.
            ..MemberHubInvitationConfig::default()
        };
        let invitation = runtime
            .create_member_hub_invitation(&config, None)
            .inspect_err(|error| {
                if error.has_code(StatusCode::MEMBER_SPONSOR_NOT_ALLOWED) {
                    eprintln!("Ask the operator to allow this app to bring in member hubs.");
                }
            })?;
        let mut bytes = invitation.serialize()?;
        drop(invitation); // destroys the handle
        // stdout stands in for your channel to the other PC; never log the bytes.
        let written = std::io::stdout()
            .write_all(&bytes)
            .and_then(|()| std::io::stdout().flush());
        bytes.fill(0); // clear the secret before freeing it
        written.map_err(|error| {
            Error::InvalidArgument(format!("the invitation was not handed over: {error}"))
        })?;
        eprintln!("qar_runtime_create_member_hub_invitation: ok");
        // ANCHOR_END: issue

        drop(session);
        Ok(())
    }

    /// Role "join", on the other PC, first start.
    fn join(library: &Library, runtime: &Runtime) -> Result<()> {
        let mut bytes = Vec::new();
        let read = std::io::stdin()
            .take(MAX_INVITATION_BYTES + 1)
            .read_to_end(&mut bytes);
        if read.is_err() || bytes.is_empty() || bytes.len() as u64 > MAX_INVITATION_BYTES {
            return Err(Error::InvalidArgument("no invitation on stdin".to_string()));
        }
        // Checks the format only; the join checks the signature.
        let parsed = library.deserialize_member_hub_invitation(&bytes);
        bytes.fill(0); // the handle keeps its own copy
        let invitation = parsed?;

        // ANCHOR: join
        let (progress, reports) = mpsc::channel::<Progress>();
        let printer = std::thread::spawn(move || {
            for report in reports {
                eprintln!("{}", report.message);
            }
        });
        let joined = runtime.join_hub_as_member(
            &invitation, // borrowed for the call
            &JoinHubAsMemberConfig::default(),
            &BlockingOptions {
                progress: Some(progress),
                cancel: None,
            },
        );
        let _ = printer.join();
        if let Err(error) = &joined
            && error.has_code(StatusCode::TIMEOUT)
        {
            eprintln!("The join is committed; run join again once the hub is back.");
        }
        joined?;
        // ANCHOR_END: join

        // ANCHOR: redeem
        let session = onboard_local(runtime, None, Some(&invitation)).inspect_err(|error| {
            if error.has_code(StatusCode::ONBOARDING_APPROVAL_DENIED) {
                eprintln!("The grant already admitted another app, or it was revoked.");
            }
        })?;
        // ANCHOR_END: redeem
        print_own_peer(&session)
    }

    /// Role "start", on the other PC, every later start.
    fn start(runtime: &Runtime) -> Result<()> {
        // ANCHOR: start
        // No tag needed: the primary remembers a granted app under the grant's tag, whatever
        // tag the app sends.
        let session = onboard_local(runtime, None, None)?;
        // ANCHOR_END: start
        print_own_peer(&session)
    }
}

fn main() -> ExitCode {
    let role = std::env::args().nth(1).unwrap_or_default();
    if !matches!(role.as_str(), "issue" | "join" | "start") {
        eprintln!("Usage: remote_app_onboarding issue|join|start");
        return ExitCode::from(1);
    }
    remote_app::main(&role)
}
