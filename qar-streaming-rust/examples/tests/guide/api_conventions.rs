use std::sync::mpsc;
use std::time::Duration;

use qaros::{
    BlockingOptions, Library, OnboardMethod, Onboarded, PeerId, PeerPresentation, RenderSender,
    RenderSenderConfig, Runtime, Session, StatusCode,
};

fn handles_and_ids(
    library: &Library,
    session: Session,
    first_peer: PeerId,
    second_peer: PeerId,
) -> qaros::Result<()> {
    // ANCHOR: handles_and_ids
    // Dropping the Session tears down the live session; a local app joins again on its next start:
    drop(session);

    // IDs are Copy values that compare by their bytes; the library renders them as text:
    let same_peer = first_peer == second_peer;
    let peer_text = library.uuid_to_string(&first_peer.0)?;
    // ANCHOR_END: handles_and_ids
    println!("{same_peer} {peer_text}");
    Ok(())
}

fn results(runtime: &Runtime, presentation: &PeerPresentation) -> qaros::Result<Option<Session>> {
    // ANCHOR: results
    let method = OnboardMethod::LocalApp {
        approval_timeout: None,
        app_tag: None,
    };
    let session = match runtime.onboard(&method, presentation, &BlockingOptions::default()) {
        Ok(onboarded) => Some(onboarded.session),
        // Expected control flow, not a bug: nobody answered the approval prompt in time.
        Err(error) if error.has_code(StatusCode::ONBOARDING_APPROVAL_TIMEOUT) => None,
        // Anything else: the error names the C function, the status code and the message.
        Err(error) => return Err(error),
    };
    // ANCHOR_END: results
    Ok(session)
}

fn extension_chain(runtime: &Runtime, presentation: &PeerPresentation) -> qaros::Result<Onboarded> {
    // ANCHOR: extension_chain
    // The local-app and app-tag extensions, chained and stamped by the binding:
    let method = OnboardMethod::LocalApp {
        approval_timeout: Some(Duration::from_secs(60)),
        app_tag: Some("left.my-viewer.vendor"),
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
            app_tag: None,
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
