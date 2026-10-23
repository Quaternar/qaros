use std::sync::mpsc;
use std::time::Duration;

use qaros::{
    BlockingOptions, HubAddress, Library, OnboardMethod, Onboarded, OnboardingId, PeerId,
    PeerPresentation, RenderSender, RenderSenderConfig, Runtime, Session, StatusCode,
};

fn handles_and_ids(
    library: &Library,
    runtime: &Runtime,
    session: Session,
    saved_id: OnboardingId,
    first_peer: PeerId,
    second_peer: PeerId,
) -> qaros::Result<()> {
    // ANCHOR: handles_and_ids
    // Dropping the Session tears down the live session but KEEPS the identity:
    drop(session);

    // Erasing the persisted identity is a separate, explicit call on the runtime:
    runtime.forget(saved_id)?;

    // IDs are Copy values that compare by their bytes; the library renders them as text:
    let same_peer = first_peer == second_peer;
    let saved_text = library.uuid_to_string(&saved_id.0)?;
    // ANCHOR_END: handles_and_ids
    println!("{same_peer} {saved_text}");
    Ok(())
}

fn results(
    runtime: &Runtime,
    saved_id: OnboardingId,
    presentation: &PeerPresentation,
) -> qaros::Result<Option<Session>> {
    // ANCHOR: results
    let session = match runtime.rejoin(saved_id, presentation, &BlockingOptions::default()) {
        Ok(session) => Some(session),
        // Expected control flow, not an error: no persisted slot, so onboard instead.
        Err(error) if error.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND) => None,
        // Anything else: the error names the C function, the status code and the message.
        Err(error) => return Err(error),
    };
    // ANCHOR_END: results
    Ok(session)
}

fn extension_chain(
    runtime: &Runtime,
    presentation: &PeerPresentation,
    hub_port: u16,
) -> qaros::Result<Onboarded> {
    // ANCHOR: extension_chain
    // The code and host extensions, chained and stamped by the binding:
    let method = OnboardMethod::Code {
        code: "ABCD-EFGH",
        hub: Some(HubAddress {
            hostname: "192.168.1.50",
            port: hub_port, // shown with the hub's code
        }),
    };
    let onboarded = runtime.onboard(&method, presentation, &BlockingOptions::default())?;
    // ANCHOR_END: extension_chain
    Ok(onboarded)
}

fn blocking_call(library: &Library, runtime: &Runtime) -> qaros::Result<Onboarded> {
    // ANCHOR: blocking_call
    let token = library.create_cancel_token(Some(Duration::from_secs(120)))?;
    let (progress, reports) = mpsc::channel();
    let onboarded = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
        },
        &PeerPresentation::default(),
        &BlockingOptions {
            progress: Some(progress), // reports sent from a library thread while the call blocks
            cancel: Some(&token),     // borrowed, so it outlives the call
        },
    )?;
    for report in reports.try_iter() {
        println!("{:.0}% {}", report.percent, report.message);
    }
    // ANCHOR_END: blocking_call
    Ok(onboarded)
}

fn async_creation(
    session: &Session,
    config: &RenderSenderConfig,
) -> qaros::Result<Option<RenderSender>> {
    // ANCHOR: async_creation
    // Returns at once; the creation is cancelled after 10 s.
    let mut pending = session.start_render_sender(config, Some(Duration::from_secs(10)), || {})?;

    // On each frame of your loop: None while the library creates the sender,
    // then the sender or the error the library delivered.
    let created: Option<RenderSender> = pending.poll().transpose()?;
    // ANCHOR_END: async_creation
    Ok(created)
}

fn enumeration(session: &Session) -> qaros::Result<()> {
    // ANCHOR: enumeration
    for peer in session.peers()? {
        // peer is a PeerInfo snapshot: owned, safe to keep
        println!("{} {}", peer.display_name, peer.app_version);
    }
    // ANCHOR_END: enumeration
    Ok(())
}
