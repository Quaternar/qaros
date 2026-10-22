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
use std::time::Duration;

use qaros::{AppVolumeConfig, AppVolumeSize, Pose, Vec3};
use qaros_examples::{
    Arguments, ExampleResult, create_runtime, join_session, load_library, report,
};

/// How long the update loop waits for one more update before it moves on.
const UPDATE_QUIET_PERIOD: Duration = Duration::from_millis(500);

fn main() -> ExitCode {
    report(run())
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
    println!("Created app volume with id: {volume_id}");
    // ANCHOR_END: create

    // ANCHOR: subscribe
    let (forward_update, volume_updates) = qaros::channel();
    let subscription = volumes.subscribe(forward_update)?;
    // ANCHOR_END: subscribe

    // ANCHOR: transforms
    volumes.set_app_pose(volume_id, Pose::from_position(Vec3::new(0.0, -0.25, 0.0)))?;
    volumes.set_app_scale(volume_id, 0.5)?;
    // ANCHOR_END: transforms

    // ANCHOR: receive
    while let Ok(update) = volume_updates.recv_timeout(UPDATE_QUIET_PERIOD) {
        if update.id != volume_id {
            continue;
        }
        println!(
            "Update: app position({:.2}, {:.2}, {:.2}) app scale {:.2} lifetime={:?}",
            update.app_pose.position.x,
            update.app_pose.position.y,
            update.app_pose.position.z,
            update.app_scale,
            update.lifetime
        );
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
