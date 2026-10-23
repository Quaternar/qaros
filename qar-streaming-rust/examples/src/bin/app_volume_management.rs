//! Port of `app_volume_management.c`: get or create an app volume, subscribe to volume updates,
//! move the content inside it, read its latest transforms and list the session's volumes.
//!
//! ```text
//! cargo run -p qaros-examples --bin app_volume_management -- [pairing-code]
//! ```
//!
//! Without a stored onboarding id or a pairing code it joins the QAROS on this PC as a local app
//! (Windows; Allow once in QAROS), which the C sample, onboarding by code only, does not.

use std::path::Path;
use std::process::ExitCode;

use qaros::{AppVolumeConfig, AppVolumeInfo, AppVolumeSize, Pose, Vec3};
use qaros_examples::{
    Arguments, ExampleResult, FIRST_UPDATE_TIMEOUT, SAME_FACTOR, SAME_LENGTH_METERS,
    create_runtime, join_session, load_library, nearly_equal, report, same_pose,
    updates_until_quiet,
};

fn main() -> ExitCode {
    report(run())
}

// ANCHOR: same_update
/// Whether two updates show the same volume: poses, size and scale at the precision of
/// `same_pose`, the lifetime exactly.
fn same_volume_update(first: &AppVolumeInfo, second: &AppVolumeInfo) -> bool {
    same_pose(&first.pose, &second.pose)
        && nearly_equal(first.size.width, second.size.width, SAME_LENGTH_METERS)
        && nearly_equal(first.size.length, second.size.length, SAME_LENGTH_METERS)
        && nearly_equal(first.size.height, second.size.height, SAME_LENGTH_METERS)
        && same_pose(&first.app_pose, &second.app_pose)
        && nearly_equal(first.app_scale, second.app_scale, SAME_FACTOR)
        && first.lifetime == second.lifetime
}
// ANCHOR_END: same_update

fn print_volume_update(update: &AppVolumeInfo) {
    println!(
        "Update: center({:.2}, {:.2}, {:.2}) size({:.2} x {:.2} x {:.2}) app position({:.2}, {:.2}, {:.2}) app scale {:.2} lifetime={:?}",
        update.pose.position.x,
        update.pose.position.y,
        update.pose.position.z,
        update.size.width,
        update.size.length,
        update.size.height,
        update.app_pose.position.x,
        update.app_pose.position.y,
        update.app_pose.position.z,
        update.app_scale,
        update.lifetime
    );
}

fn run() -> ExampleResult {
    // ANCHOR: session
    let arguments = Arguments::parse();
    let library = load_library()?;
    let runtime = create_runtime(&library, None)?;
    let session = join_session(
        &library,
        &runtime,
        arguments.pairing_code.as_deref(),
        Path::new("app_volume_management.onboarding-id.txt"),
        "App Volume Peer",
    )?;
    // ANCHOR_END: session

    // ANCHOR: create
    let volumes = session.app_volumes();
    let mut config = AppVolumeConfig::new("tutorial-volume.examples.qaros");
    config.display_name = Some("Tutorial Volume".to_string());
    config.pose = Pose::from_position(Vec3::new(0.0, 0.0, -1.0));
    config.size = AppVolumeSize {
        width: 1.0,
        length: 1.0,
        height: 1.0,
    };
    let volume_id = volumes.get_or_create(&config)?;
    println!("App volume id: {volume_id}");
    // ANCHOR_END: create

    // ANCHOR: subscribe
    let (forward_update, volume_updates) = qaros::channel();
    // subscribe reports every volume of the session: forward only this one's updates.
    let subscription = volumes.subscribe(move |update| {
        if update.id == volume_id {
            forward_update(update);
        }
    })?;
    // ANCHOR_END: subscribe

    // ANCHOR: transforms
    volumes.set_app_pose(volume_id, Pose::from_position(Vec3::new(0.0, -0.25, 0.0)))?;
    volumes.set_app_scale(volume_id, 0.5)?;
    // ANCHOR_END: transforms

    // ANCHOR: receive
    let mut received_count = 0;
    let mut printed_count = 0;
    let mut last_printed: Option<AppVolumeInfo> = None;
    for update in updates_until_quiet(&volume_updates) {
        received_count += 1;
        // The Hub repeats updates; print only one that differs from the last one printed.
        if last_printed
            .as_ref()
            .is_some_and(|last| same_volume_update(last, &update))
        {
            continue;
        }
        printed_count += 1;
        print_volume_update(&update);
        last_printed = Some(update);
    }
    if received_count == 0 {
        println!(
            "No update of the volume arrived within {} s of the changes.",
            FIRST_UPDATE_TIMEOUT.as_secs()
        );
    }
    let skipped_repeats = if printed_count < received_count {
        ": the others matched the update printed before them"
    } else {
        ""
    };
    println!(
        "Received {received_count} updates of this volume, printed {printed_count}{skipped_repeats}."
    );
    if received_count == 0 {
        return Err("no update of the volume arrived".into());
    }
    // ANCHOR_END: receive

    // ANCHOR: latest
    let pose = volumes.latest_pose(volume_id)?;
    let size = volumes.latest_size(volume_id)?;
    let app_pose = volumes.latest_app_pose(volume_id)?;
    let app_scale = volumes.latest_app_scale(volume_id)?;
    println!(
        "Latest: center({:.2}, {:.2}, {:.2}) size({:.2} x {:.2} x {:.2}) app position({:.2}, {:.2}, {:.2}) app scale {:.2}",
        pose.position.x,
        pose.position.y,
        pose.position.z,
        size.width,
        size.length,
        size.height,
        app_pose.position.x,
        app_pose.position.y,
        app_pose.position.z,
        app_scale
    );
    // ANCHOR_END: latest

    // ANCHOR: list
    for volume in volumes.list()? {
        println!(
            "- Volume id: {} name='{}' size({:.2} x {:.2} x {:.2}) position({:.2}, {:.2}, {:.2})",
            volume.id,
            volume.display_name,
            volume.size.width,
            volume.size.length,
            volume.size.height,
            volume.pose.position.x,
            volume.pose.position.y,
            volume.pose.position.z
        );
    }
    // ANCHOR_END: list

    // ANCHOR: end
    subscription.cancel()?;
    // ANCHOR_END: end
    Ok(())
}
