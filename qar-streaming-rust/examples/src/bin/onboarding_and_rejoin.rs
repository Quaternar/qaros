//! Port of `onboarding_and_rejoin.c`: obtain a session by rejoin-or-onboard,
//! mint an invite, onboard a sibling runtime from it, and tear down with
//! destroy (keeps the identity) or `--forget` (erases it).
//!
//! ```text
//! cargo run -p qaros-examples --bin onboarding_and_rejoin -- [pairing-code] [--forget]
//! ```

use std::path::Path;
use std::process::ExitCode;

use qaros::{BlockingOptions, OnboardMethod, PeerPresentation};
use qaros_examples::{
    Arguments, ExampleResult, create_runtime, load_library, obtain_session, report,
    save_onboarding_id,
};

const PRIMARY_STATE: &str = "onboarding_and_rejoin.primary-state";
const SIBLING_STATE: &str = "onboarding_and_rejoin.sibling-state";
const PRIMARY_ID_FILE: &str = "onboarding_and_rejoin.primary-onboarding-id.txt";
const SIBLING_ID_FILE: &str = "onboarding_and_rejoin.sibling-onboarding-id.txt";

fn main() -> ExitCode {
    report(run())
}

fn run() -> ExampleResult {
    // ANCHOR: arguments
    let arguments = Arguments::parse();
    let forget_on_exit = arguments.has_flag("--forget");
    // ANCHOR_END: arguments

    // ANCHOR: primary
    let library = load_library()?;
    let primary_runtime = create_runtime(&library, Some(Path::new(PRIMARY_STATE)))?;
    let (primary_id, primary_session) = obtain_session(
        &library,
        &primary_runtime,
        arguments.pairing_code.as_deref(),
        Path::new(PRIMARY_ID_FILE),
        "Primary Tutorial Peer",
    )?;
    println!(
        "Primary session obtained with onboarding id {}",
        library.uuid_to_string(&primary_id.0)?
    );
    // ANCHOR_END: primary

    // ANCHOR: mint_invite
    let invite = primary_session.request_onboarding_invite(&BlockingOptions::default())?;
    let wire_bytes = invite.serialize()?;
    println!(
        "Minted sibling invite ({} bytes, expires at unix {}).",
        wire_bytes.len(),
        invite.expires_unix()?
    );
    drop(invite);
    // ANCHOR_END: mint_invite

    // ANCHOR: sibling
    let sibling_runtime = create_runtime(&library, Some(Path::new(SIBLING_STATE)))?;
    let sibling_invite = library.deserialize_invite(&wire_bytes)?;
    let sibling = sibling_runtime.onboard(
        &OnboardMethod::Invite(&sibling_invite),
        &PeerPresentation {
            display_name: Some("Sibling Tutorial Peer".to_string()),
            ..PeerPresentation::default()
        },
        &BlockingOptions::default(),
    )?;
    drop(sibling_invite);
    let sibling_id = sibling.onboarding_id;
    println!(
        "Sibling runtime onboarded with onboarding id {}",
        library.uuid_to_string(&sibling_id.0)?
    );
    save_onboarding_id(&library, Path::new(SIBLING_ID_FILE), sibling_id)?;
    // ANCHOR_END: sibling

    // ANCHOR: destroy
    println!("Destroying sibling session handle keeps its persisted onboarding state for rejoin.");
    drop(sibling.session);

    println!("Destroying primary session handle keeps its persisted onboarding state for rejoin.");
    drop(primary_session);
    // ANCHOR_END: destroy

    // ANCHOR: forget
    if forget_on_exit {
        sibling_runtime.forget(sibling_id)?;
        println!("qar_runtime_forget(sibling): SUCCESS");
        std::fs::remove_file(SIBLING_ID_FILE)?;
        primary_runtime.forget(primary_id)?;
        println!("qar_runtime_forget(primary): SUCCESS");
        std::fs::remove_file(PRIMARY_ID_FILE)?;
    }
    // ANCHOR_END: forget
    Ok(())
}
