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
mod rendering_streams;
mod troubleshooting;

// The peer presence, role, category and device identity functions are in the bindings generated
// from the public header, not in those of the header of runtime 0.9.0-dev.2916, so their snippets
// are compiled where the generated function list declares `qar_impl_peer_update_device_identity`.
macro_rules! peers_identity_module_when_declared {
    (qar_impl_peer_update_device_identity) => {
        mod peers_identity;
    };
    ($other_function:ident) => {};
}

macro_rules! declare_peers_identity_module {
    ($(
        pub fn $name:ident($($argument:ident : $argument_type:ty),* $(,)?) $(-> $return_type:ty)?;
    )*) => {
        $(peers_identity_module_when_declared!($name);)*
    };
}

qaros::qaros_sys::qar_impl_function_list!(declare_peers_identity_module);
