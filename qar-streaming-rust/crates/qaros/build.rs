// `qaros_d3d11` is set where the feature `d3d11` is on and the bindings carry the D3D11 types:
// Windows, the runtime's platform, and macOS, whose bindings serve qaros-stub. Elsewhere the
// feature compiles to nothing.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(qaros_d3d11)");
    let feature_enabled = std::env::var_os("CARGO_FEATURE_D3D11").is_some();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let bindings_have_d3d11 = target_os == "windows" || target_os == "macos";
    if feature_enabled && bindings_have_d3d11 {
        println!("cargo::rustc-cfg=qaros_d3d11");
    }
}
