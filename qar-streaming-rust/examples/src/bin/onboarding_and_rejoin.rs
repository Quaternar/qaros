//! Obtain a session by rejoin-or-onboard with a pairing code, and tear down with destroy (keeps
//! the identity) or `--forget` (erases it). The C sample of the same name pairs a camera on a PC
//! without a hub the same way.
//!
//! ```text
//! cargo run -p qaros-examples --bin onboarding_and_rejoin -- [pairing-code] [--forget]
//! ```

use std::path::Path;
use std::process::ExitCode;

use qaros_examples::{
    Arguments, ExampleResult, create_runtime, load_library, obtain_session, report,
};

const PRIMARY_STATE: &str = "onboarding_and_rejoin.primary-state";
const PRIMARY_ID_FILE: &str = "onboarding_and_rejoin.primary-onboarding-id.txt";

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

    // ANCHOR: destroy
    println!("Destroying primary session handle keeps its persisted onboarding state for rejoin.");
    drop(primary_session);
    // ANCHOR_END: destroy

    // ANCHOR: forget
    if forget_on_exit {
        primary_runtime.forget(primary_id)?;
        println!("qar_runtime_forget(primary): SUCCESS");
        std::fs::remove_file(PRIMARY_ID_FILE)?;
    }
    // ANCHOR_END: forget
    Ok(())
}
