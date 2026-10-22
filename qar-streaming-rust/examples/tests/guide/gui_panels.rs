use qaros::qaros_sys::{QarGuiPanelId, QarPeerId};
use qaros::{
    GuiPanelConfig, GuiPanelId, GuiPanelSize, GuiPanelState, Library, PeerId, Pose, Session, Vec3,
};

fn get_or_create(session: &Session) -> qaros::Result<GuiPanelId> {
    // ANCHOR: get_or_create
    let config = GuiPanelConfig {
        display_name: Some("Machine Status".to_string()),
        // Pose is the top-edge midpoint, room space, meters.
        pose: Pose::from_position(Vec3::new(0.0, 1.5, -1.0)),
        size: GuiPanelSize {
            width: 0.8,
            height: 0.5,
            ..GuiPanelSize::default()
        },
        ..GuiPanelConfig::new("machine-status.myapp.example.com") // required, stable identity
    };

    // get_or_create: returns the existing panel or creates it. Idempotent.
    let panels = session.gui_panels();
    let panel_id = panels.get_or_create(&config)?;

    panels.navigate_to_uri(panel_id, "https://myapp.local/status-dashboard")?;
    // ANCHOR_END: get_or_create
    Ok(panel_id)
}

fn manipulate(
    session: &Session,
    panel_id: GuiPanelId,
    new_pose: Pose,
    new_size: GuiPanelSize,
) -> qaros::Result<()> {
    // ANCHOR: manipulate
    let panels = session.gui_panels();
    panels.set_pose(panel_id, new_pose)?; // move it
    panels.set_size(panel_id, new_size)?; // resize
    panels.set_state(panel_id, GuiPanelState::Minimized)?;
    panels.navigate_to_uri(panel_id, "https://...")?;
    panels.close(panel_id)?; // remove
    // ANCHOR_END: manipulate
    Ok(())
}

fn update_visible_to(library: &Library, session: &Session, panel_id: GuiPanelId, operator: PeerId) {
    // ANCHOR: update_visible_to
    // Editing the visibility set is raw only: call the raw table with the session's handle.
    let api = library.raw_api();
    let panel = QarGuiPanelId { data: panel_id.0 };
    let operator_peer_id = QarPeerId { data: operator.0 };
    let mut additions = [&raw const operator_peer_id];
    // SAFETY: the session is alive and used on this thread; the ids outlive the call.
    let result = unsafe {
        (api.qar_impl_gui_panels_update_visible_to)(
            session.as_raw(),
            &raw const panel,
            additions.as_mut_ptr(),
            additions.len(),
            std::ptr::null_mut(), // no removals
            0,
        )
    };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // ANCHOR_END: update_visible_to
}
