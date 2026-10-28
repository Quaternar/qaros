// `qaros_member_hubs` is set where qaros-sys's bindings declare the member-hub types (its build
// script reports them as `DEP_QAR_STREAMING_C_MEMBER_HUBS`), so `remote_app_onboarding` builds
// its real program there and a stub that says why elsewhere.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(qaros_member_hubs)");
    if std::env::var_os("DEP_QAR_STREAMING_C_MEMBER_HUBS").is_some() {
        println!("cargo::rustc-cfg=qaros_member_hubs");
    }
}
