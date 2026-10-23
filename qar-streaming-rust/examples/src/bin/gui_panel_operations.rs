//! Port of `gui_panel_operations.c`: create a panel, subscribe to its updates, move, resize,
//! minimize and navigate it, list panels, end the subscription and close the panel.
//!
//! ```text
//! cargo run -p qaros-examples --bin gui_panel_operations -- [pairing-code]
//! ```
//!
//! Without a stored onboarding id or a pairing code it joins the QAROS on this PC as a local app
//! (Windows; Allow once in QAROS), which the C sample, onboarding by code only, does not.

use std::path::Path;
use std::process::ExitCode;

use qaros::{GuiPanelConfig, GuiPanelInfo, GuiPanelSize, GuiPanelState, Pose, Vec3};
use qaros_examples::{
    Arguments, ExampleResult, FIRST_UPDATE_TIMEOUT, SAME_FACTOR, SAME_LENGTH_METERS,
    create_runtime, join_session, load_library, nearly_equal, report, same_pose,
    updates_until_quiet,
};

fn main() -> ExitCode {
    report(run())
}

// ANCHOR: same_update
/// Whether two updates show the same panel: pose and size at the precision of `same_pose`, state
/// and address exactly.
fn same_panel_update(first: &GuiPanelInfo, second: &GuiPanelInfo) -> bool {
    same_pose(&first.pose, &second.pose)
        && nearly_equal(first.size.width, second.size.width, SAME_LENGTH_METERS)
        && nearly_equal(first.size.height, second.size.height, SAME_LENGTH_METERS)
        && nearly_equal(
            first.size.content_scale,
            second.size.content_scale,
            SAME_FACTOR,
        )
        && first.state == second.state
        && first.content_uri == second.content_uri
}
// ANCHOR_END: same_update

fn run() -> ExampleResult {
    // ANCHOR: session
    let arguments = Arguments::parse();
    let library = load_library()?;
    let runtime = create_runtime(&library, None)?;
    let session = join_session(
        &library,
        &runtime,
        arguments.pairing_code.as_deref(),
        Path::new("gui_panel_operations.onboarding-id.txt"),
        "GUI Panel Peer",
    )?;
    // ANCHOR_END: session

    // ANCHOR: create
    let panels = session.gui_panels();
    let mut config = GuiPanelConfig::new("tutorial-panel.examples.qaros");
    config.display_name = Some("Tutorial Panel".to_string());
    config.pose = Pose::from_position(Vec3::new(0.5, 1.5, -1.2));
    config.size = GuiPanelSize {
        width: 1.2,
        height: 0.7,
        ..GuiPanelSize::default()
    };
    let panel_id = panels.get_or_create(&config)?;
    println!("GUI panel id: {panel_id}");
    // ANCHOR_END: create

    // ANCHOR: subscribe
    let (forward_update, panel_updates) = qaros::channel();
    let subscription = panels.subscribe_panel(panel_id, forward_update)?;
    // ANCHOR_END: subscribe

    // ANCHOR: update
    panels.set_pose(panel_id, Pose::from_position(Vec3::new(0.2, 1.6, -1.0)))?;
    panels.set_size(
        panel_id,
        GuiPanelSize {
            width: 1.0,
            height: 0.6,
            ..GuiPanelSize::default()
        },
    )?;
    panels.set_state(panel_id, GuiPanelState::Minimized)?;
    panels.navigate_to_uri(panel_id, "https://example.com/tutorial")?;
    // ANCHOR_END: update

    // ANCHOR: receive
    let mut received_count = 0;
    let mut printed_count = 0;
    let mut last_printed: Option<GuiPanelInfo> = None;
    for update in updates_until_quiet(&panel_updates) {
        received_count += 1;
        // The Hub repeats updates; print only one that differs from the last one printed.
        if last_printed
            .as_ref()
            .is_some_and(|last| same_panel_update(last, &update))
        {
            continue;
        }
        printed_count += 1;
        println!(
            "Update: position({:.2}, {:.2}, {:.2}) size({:.2} x {:.2}) state={:?} uri='{}'",
            update.pose.position.x,
            update.pose.position.y,
            update.pose.position.z,
            update.size.width,
            update.size.height,
            update.state,
            update.content_uri
        );
        last_printed = Some(update);
    }
    if received_count == 0 {
        println!(
            "No update of the panel arrived within {} s of the changes.",
            FIRST_UPDATE_TIMEOUT.as_secs()
        );
    }
    let skipped_repeats = if printed_count < received_count {
        ": the others matched the update printed before them"
    } else {
        ""
    };
    println!("Received {received_count} updates, printed {printed_count}{skipped_repeats}.");
    // ANCHOR_END: receive

    // ANCHOR: list
    for panel in panels.list()? {
        println!(
            "- Panel id: {} name='{}' size({:.2} x {:.2}) position({:.2}, {:.2}, {:.2}) state={:?} uri='{}'",
            panel.id,
            panel.display_name,
            panel.size.width,
            panel.size.height,
            panel.pose.position.x,
            panel.pose.position.y,
            panel.pose.position.z,
            panel.state,
            panel.content_uri
        );
    }
    // ANCHOR_END: list

    // ANCHOR: close
    subscription.cancel()?;
    panels.close(panel_id)?;
    println!("qar_gui_panels_close_panel: SUCCESS");
    // ANCHOR_END: close
    if received_count == 0 {
        return Err("no update of the panel arrived".into());
    }
    Ok(())
}
