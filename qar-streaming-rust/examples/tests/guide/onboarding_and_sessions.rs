use std::sync::mpsc;

use qaros::{
    BlockingOptions, CancelToken, GraphicsAdapter, HubAddress, Library, OnboardMethod,
    OnboardingId, PeerPresentation, Progress, Runtime, RuntimeConfig, Session, SessionId,
    SessionLifecycle, SessionLifecycleState, StatusCode, Subscription,
};

fn local_app(runtime: &Runtime, cancel: &CancelToken) -> qaros::Result<Session> {
    // ANCHOR: local_app
    let (progress, reports) = mpsc::channel::<Progress>();
    let printer = std::thread::spawn(move || {
        for report in reports {
            println!("{}", report.message); // "Waiting for approval in QAROS hub"
        }
    });

    let presentation = PeerPresentation {
        display_name: Some("Vendor Capture".to_string()),
        ..PeerPresentation::default()
    };
    let joined = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            app_tag: None, // the default tag; see "App tags"
        },
        &presentation,
        &BlockingOptions {
            progress: Some(progress),
            cancel: Some(cancel),
        },
    );
    let _ = printer.join(); // the channel closes when the call returns

    // A local app's onboarding id is the zero id: nothing to persist.
    let session = joined?.session;
    // ANCHOR_END: local_app
    Ok(session)
}

fn local_app_with_tag(runtime: &Runtime) -> qaros::Result<Session> {
    // ANCHOR: app_tag
    let who = PeerPresentation {
        display_name: Some("TerraExplorer – left view".to_string()), // any UTF-8
        ..PeerPresentation::default()
    };
    let onboarded = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            app_tag: Some("left.terraexplorer.skyline"),
        },
        &who,
        &BlockingOptions::default(),
    )?;
    // ANCHOR_END: app_tag
    Ok(onboarded.session)
}

// ANCHOR: obtain_session
fn obtain_session(
    runtime: &Runtime,
    saved_id: &mut OnboardingId,  // loaded from your app's settings
    code_from_user: Option<&str>, // None until you prompted the user
    hub_host: &str,               // the hub PC's host name or IP
    hub_port: u16,                // shown with the hub's code
) -> qaros::Result<Option<Session>> {
    let who = PeerPresentation {
        display_name: Some("My CAD Tool".to_string()),
        app_version: Some("2.4.0".to_string()),
        ..PeerPresentation::default()
    };

    if !saved_id.is_zero() {
        match runtime.rejoin(*saved_id, &who, &BlockingOptions::default()) {
            Ok(session) => return Ok(Some(session)), // silent reconnect: the normal path
            Err(error) if error.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND) => {
                // no persisted slot -> fall through to onboarding
            }
            Err(error) => return Err(error), // real error (hub unreachable, ...)
        }
    }

    let Some(code) = code_from_user else {
        return Ok(None); // prompt the user for the code shown by the hub, then call again
    };
    let method = OnboardMethod::Code {
        code,
        hub: Some(HubAddress {
            hostname: hub_host,
            port: hub_port,
        }),
    };
    match runtime.onboard(&method, &who, &BlockingOptions::default()) {
        Ok(onboarded) => {
            *saved_id = onboarded.onboarding_id; // persist it before returning
            Ok(Some(onboarded.session))
        }
        // Wrong or expired code: re-prompt the user and call again.
        Err(error) if error.has_code(StatusCode::PAKE_ERROR) => Ok(None),
        Err(error) => Err(error),
    }
}
// ANCHOR_END: obtain_session

fn co_process(library: &Library, session: &Session, blob: &[u8]) -> qaros::Result<Session> {
    // ANCHOR: co_process
    // In your app (joined as a local app):
    let invite = session.request_onboarding_invite(
        "companion.terraexplorer.skyline",
        &BlockingOptions::default(),
    )?;
    let mut wire_bytes = invite.serialize()?;
    drop(invite);
    // write `wire_bytes` to the co-process's stdin, then clear it
    wire_bytes.fill(0);

    // In the co-process:
    let runtime = library.create_runtime(&RuntimeConfig {
        ephemeral_storage: true, // identity in memory only
        ..RuntimeConfig::default()
    })?;
    let received = library.deserialize_invite(blob)?;
    let who = PeerPresentation {
        display_name: Some("TerraExplorer companion".to_string()),
        ..PeerPresentation::default()
    };
    let onboarded = runtime.onboard(
        &OnboardMethod::Invite(&received),
        &who,
        &BlockingOptions::default(),
    )?;
    // onboarded.onboarding_id stays zero; the invite is destroyed when it drops
    // ANCHOR_END: co_process
    Ok(onboarded.session)
}

fn onboard_on_adapter(
    runtime: &Runtime,
    who: &PeerPresentation,
    my_adapter_luid: [u8; 8],
) -> qaros::Result<Session> {
    // ANCHOR: onboard_on_adapter
    // e.g. VkPhysicalDeviceIDProperties::deviceLUID; other platforms GraphicsAdapter::Uuid
    let adapter = GraphicsAdapter::Luid(my_adapter_luid);
    // The adapter is chained after the mode extension, as in C.
    let onboarded = runtime.onboard_on_adapter(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            app_tag: None,
        },
        who,
        Some(adapter),
        &BlockingOptions::default(),
    )?;
    // ANCHOR_END: onboard_on_adapter
    Ok(onboarded.session)
}

fn session_adapter(session: &Session) -> qaros::Result<()> {
    // ANCHOR: session_adapter
    if let Some(gpu) = session.graphics_adapter()? {
        // match gpu (Luid on Windows, Uuid elsewhere) against VkPhysicalDeviceIDProperties
        // with gpu.matches(device_luid, device_uuid)
        println!("render on adapter {gpu}");
    }
    // ANCHOR_END: session_adapter
    Ok(())
}

fn onboard_from_stdin_line(
    library: &Library,
    runtime: &Runtime,
    line: &str,
    who: &PeerPresentation,
) -> qaros::Result<Session> {
    // ANCHOR: launch_invite
    let onboarded = if line.starts_with('{') {
        let invite = library.deserialize_invite(line.as_bytes())?;
        runtime.onboard(
            &OnboardMethod::Invite(&invite),
            who,
            &BlockingOptions::default(),
        )?
        // the invite is destroyed when it drops
    } else {
        let method = OnboardMethod::Code {
            code: line,
            hub: None,
        };
        runtime.onboard(&method, who, &BlockingOptions::default())?
    };
    // ANCHOR_END: launch_invite
    Ok(onboarded.session)
}

/// What the app does with the lifecycle states on its own thread.
fn get_or_create_owned_objects(_session: &Session) {}

fn follow_lifecycle(session: &Session) -> qaros::Result<Subscription> {
    // ANCHOR: session_lifecycle
    let (posted, states) = mpsc::channel::<SessionLifecycle>();
    // Library thread: hand the snapshot to the app's thread and return.
    let lifecycle = session.subscribe_lifecycle(move |state| {
        let _ = posted.send(state);
    })?; // without it the app never learns of RePairRequired

    // On the app's thread, for each posted state:
    let mut last_id: SessionId = session.id()?;
    for state in states.try_iter() {
        if state.session_id != last_id {
            last_id = state.session_id; // re-read whatever you keyed by the session id
            get_or_create_owned_objects(session); // app volumes, panels, ... per common name
        }
        if state.state == SessionLifecycleState::RePairRequired {
            // drop `lifecycle` and the session, onboard again, then get_or_create_owned_objects()
            // and recreate your senders
        }
    }
    // ANCHOR_END: session_lifecycle
    Ok(lifecycle)
}

fn teardown(runtime: &Runtime, session: Session, saved_id: OnboardingId) -> qaros::Result<()> {
    // ANCHOR: teardown
    drop(session); // qar_session_handle_destroy
    // only for an explicit un-pair:
    runtime.forget(saved_id)?;
    // ANCHOR_END: teardown
    Ok(())
}
