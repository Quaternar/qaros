//! The code of the developer guide's Rust tabs. `cargo xtask guide` copies the anchored regions of
//! these modules into the guide's pages, through the tab sources in `examples/guide/`, so the Rust
//! a reader sees compiles against the current API. The snippets are compiled with the tests and
//! never run: they need a QAROS runtime.

#![allow(
    dead_code,
    reason = "the snippets are compiled for the developer guide, not called"
)]

mod api_conventions;
mod app_volumes;
mod getting_started;
mod gui_panels;
mod onboarding_and_sessions;
mod peers;
mod peers_identity;
mod rendering_streams;
mod troubleshooting;
