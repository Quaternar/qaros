// The same sources build against two binding sets: the public header's, and the header of
// runtime 0.9.0-dev.2916, which lacks some types. A cfg is set where the bindings for the target
// declare a group of them, and passed on as `DEP_QAR_STREAMING_C_<KEY>` to the crates whose build
// scripts turn it into the same cfg:
//
// - `qaros_color_extensions` (`COLOR_EXTENSIONS`): the render sender's colour extensions
//   (`QarRenderSenderAlphaModeExt`, `QarRenderSenderColorEncodingExt`).
// - `qaros_member_hubs` (`MEMBER_HUBS`): the member-hub inits and the remote-app grant
//   extensions (`QarMemberHubInvitationInit`, `QarOnboardRemoteAppGrantExt`, ...).
// - `qaros_device_identity` (`DEVICE_IDENTITY`): `QarPeerDeviceIdentity`.
const TYPE_GROUPS: [(&str, &str, &str); 3] = [
    (
        "qaros_color_extensions",
        "color_extensions",
        "pub struct QarRenderSenderAlphaModeExt",
    ),
    (
        "qaros_member_hubs",
        "member_hubs",
        "pub struct QarOnboardRemoteAppGrantExt",
    ),
    (
        "qaros_device_identity",
        "device_identity",
        "pub struct QarPeerDeviceIdentity",
    ),
];

fn main() {
    println!("cargo::rerun-if-env-changed=QAROS_LIB_DIR");
    println!("cargo::rerun-if-changed=build.rs");
    let bindings = generated_types();
    for (cfg, metadata, declaration) in TYPE_GROUPS {
        println!("cargo::rustc-check-cfg=cfg({cfg})");
        if bindings.contains(declaration) {
            println!("cargo::rustc-cfg={cfg}");
            println!("cargo::metadata={metadata}=1");
        }
    }
    if std::env::var_os("CARGO_FEATURE_LINK").is_none() {
        return;
    }
    if let Some(library_directory) = std::env::var_os("QAROS_LIB_DIR") {
        println!(
            "cargo::rustc-link-search=native={}",
            library_directory.to_string_lossy()
        );
    }
}

fn generated_types() -> String {
    let target = std::env::var("TARGET")
        .unwrap_or_default()
        .replace('-', "_");
    let types = format!("src/generated/{target}/types.rs");
    println!("cargo::rerun-if-changed={types}");
    std::fs::read_to_string(&types).unwrap_or_default()
}
