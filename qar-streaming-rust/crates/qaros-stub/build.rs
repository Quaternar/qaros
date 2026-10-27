// `qaros_color_extensions`, `qaros_member_hubs` and `qaros_device_identity` are set where
// qaros-sys's bindings declare the render sender's colour extensions, the member-hub types and
// `QarPeerDeviceIdentity` (its build script reports them as `DEP_QAR_STREAMING_C_<GROUP>`), so
// the stub implements and checks them.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    for (cfg, variable) in [
        (
            "qaros_color_extensions",
            "DEP_QAR_STREAMING_C_COLOR_EXTENSIONS",
        ),
        ("qaros_member_hubs", "DEP_QAR_STREAMING_C_MEMBER_HUBS"),
        (
            "qaros_device_identity",
            "DEP_QAR_STREAMING_C_DEVICE_IDENTITY",
        ),
    ] {
        println!("cargo::rustc-check-cfg=cfg({cfg})");
        if std::env::var_os(variable).is_some() {
            println!("cargo::rustc-cfg={cfg}");
        }
    }
}
