//! (a) The real package: library load, init and destroy, with the extension
//! next to it, twice in one process.

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use std::time::Instant;

#[test]
fn the_package_loads_initializes_and_tears_down_twice() {
    let Some(bin) = common::native_bin() else {
        return;
    };
    for round in 1..=2 {
        let started = Instant::now();
        let loaded = common::load(&bin, &format!("load-round{round}"));
        assert!(loaded.library.raw_api().is_runtime_loaded());
        let runtime = common::create_runtime(&loaded.library, &bin, &loaded.scratch, "load");
        let hub = loaded
            .extension
            .create_hub(false)
            .expect("create an extension hub");
        let hub_peer = hub.peer_id().expect("hub peer id");
        assert_ne!(hub_peer.data, [0; 16], "the hub session has a peer id");
        let layout = loaded
            .extension
            .layout_side_by_side(64, 32)
            .expect("side-by-side layout");
        assert!(layout.textures_count >= 1, "the layout has textures");
        eprintln!(
            "round {round}: layout views {} textures {}; first texture {}x{} format {}",
            layout.views_count,
            layout.textures_count,
            layout.textures[0].width,
            layout.textures[0].height,
            layout.textures[0].format.0
        );
        drop(hub);
        drop(loaded.extension);
        drop(runtime);
        drop(loaded.library);
        eprintln!("round {round}: load to teardown in {:?}", started.elapsed());
    }
}
