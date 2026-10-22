use std::ffi::{CStr, c_char};
use std::sync::mpsc;

use qaros::qaros_sys::{
    QarOnboardingInvite, QarRequestOnboardingCodesInit, QarStructureType, defaults,
};
use qaros::{
    BlockingOptions, CancelToken, GraphicsAdapter, HubAddress, Library, OnboardMethod,
    OnboardingId, PeerPresentation, Progress, Runtime, Session, StatusCode,
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

fn mint_invite(session: &Session) -> qaros::Result<Vec<u8>> {
    // ANCHOR: mint_invite
    // In the onboarded instance:
    let invite = session.request_onboarding_invite(&BlockingOptions::default())?;
    let wire_bytes = invite.serialize()?;
    // ship `wire_bytes` over your IPC of choice; the invite is destroyed when it drops
    // ANCHOR_END: mint_invite
    Ok(wire_bytes)
}

fn onboard_sibling(
    library: &Library,
    runtime: &Runtime,
    wire_bytes: &[u8],
    who: &PeerPresentation,
) -> qaros::Result<(Session, OnboardingId)> {
    // ANCHOR: onboard_sibling
    // In the sibling instance:
    let invite = library.deserialize_invite(wire_bytes)?;
    let onboarded = runtime.onboard(
        &OnboardMethod::Invite(&invite),
        who,
        &BlockingOptions::default(),
    )?;
    let my_id = onboarded.onboarding_id; // persist it for later rejoins
    drop(invite); // the caller keeps ownership of the invite
    // ANCHOR_END: onboard_sibling
    Ok((onboarded.session, my_id))
}

fn one_time_codes(library: &Library, session: &Session) -> String {
    // ANCHOR: one_time_codes
    // One-time codes are raw only: call the raw table with the session's handle.
    let api = library.raw_api();
    let request = QarRequestOnboardingCodesInit {
        header: defaults::structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_SESSION_REQUEST_ONBOARDING_CODES_INIT,
        ),
        count: 3,              // 1 to 64
        lifetime_seconds: 300, // 0 = 120 s, at most 600 s
        planned_peer_id: defaults::qar_peer_id_default(),
    };
    let mut invites: [*mut QarOnboardingInvite; 3] = [std::ptr::null_mut(); 3];
    let mut code: [c_char; 128] = [0; 128];
    // SAFETY: the session is alive and used on this thread; the array holds `count` handles,
    // which are ours to destroy; the code buffer is writable for its length.
    unsafe {
        let result = (api.qar_impl_session_request_onboarding_codes)(
            session.as_raw(),
            &raw const request,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            invites.as_mut_ptr(),
        );
        if (api.qar_impl_result_is_success)(result) {
            (api.qar_impl_onboarding_invite_get_code)(
                invites[0],
                code.as_mut_ptr(),
                code.len(),
                std::ptr::null_mut(),
            );
            for invite in invites {
                (api.qar_impl_onboarding_invite_handle_destroy)(invite);
            }
        }
    }
    // SAFETY: the library writes a terminated string into the zeroed buffer.
    let code = unsafe { CStr::from_ptr(code.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    // hand `code` to one device; it onboards with OnboardMethod::Code
    // ANCHOR_END: one_time_codes
    code
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

fn teardown(runtime: &Runtime, session: Session, saved_id: OnboardingId) -> qaros::Result<()> {
    // ANCHOR: teardown
    drop(session); // qar_session_handle_destroy
    // only for an explicit un-pair:
    runtime.forget(saved_id)?;
    // ANCHOR_END: teardown
    Ok(())
}
