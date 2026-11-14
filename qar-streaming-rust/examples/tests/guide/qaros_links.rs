use qaros::{QarosDeviceKind, QarosView, Session};

fn open_qaros_view(session: &Session) -> qaros::Result<()> {
    // ANCHOR: open_qaros_view
    // Opens the setup of a Meta Quest in this session's QAROS; QAROS starts if it is not running.
    session.open_qaros_view(QarosView::DeviceSetup(QarosDeviceKind::MetaQuest))?;
    // The other views: QarosView::PairHeadsetQr, QarosView::RoomOriginQr.
    // ANCHOR_END: open_qaros_view
    Ok(())
}

#[cfg(windows)]
fn open_qaros_link() {
    // ANCHOR: open_qaros_link
    use std::process::Command;

    // Opens the setup of a Meta Quest in QAROS; QAROS starts if it is not running.
    let link = "qaros://visualizer/v1/devices/setup?kind=quest";
    // Explorer hands the link to the program registered for qaros:// links.
    if let Err(error) = Command::new("explorer").arg(link).spawn() {
        eprintln!("could not open {link}: {error}");
    }
    // ANCHOR_END: open_qaros_link
}
