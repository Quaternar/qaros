use qaros::{PeerInfo, Session, Subscription};

fn update_display_name(session: &Session) -> qaros::Result<()> {
    // ANCHOR: update_display_name
    session.set_display_name("My CAD Tool — Assembly B")?;
    // ANCHOR_END: update_display_name
    Ok(())
}

fn query_peers(session: &Session) -> qaros::Result<()> {
    // ANCHOR: query_peers
    // peers() does the count-then-fetch and releases every handle: you get owned snapshots.
    for peer in session.peers()? {
        let name: &str = &peer.display_name;
        // also: peer.app_version, peer.custom_info, peer.app_state,
        //       peer.version_id, peer.room_tag, peer.id, peer.is_target_app
        println!("{name}");
    }
    // ANCHOR_END: query_peers
    Ok(())
}

fn subscribe_updates(session: &Session) -> qaros::Result<Subscription> {
    // ANCHOR: subscribe_updates
    // Runs on a library thread: forward to your own thread before touching app state.
    let subscription = session.subscribe_peer_updates(|peer: PeerInfo| {
        // peer joined / left / changed presentation or app state
        println!("{} is {:?}", peer.display_name, peer.app_state);
    })?;

    // Or into a std::sync::mpsc receiver you drain on your own thread:
    let (forward, updates) = qaros::channel();
    let forwarding = session.subscribe_peer_updates(forward)?;
    for peer in updates.try_iter() {
        println!("{}", peer.display_name);
    }

    // Dropping a Subscription stops delivery; detach() keeps it for the session's lifetime.
    // ANCHOR_END: subscribe_updates
    drop(forwarding);
    Ok(subscription)
}
