// `qaros_color_extensions` is set where the bindings for the target declare the render sender's
// colour extensions (`QarRenderSenderAlphaModeExt`, `QarRenderSenderColorEncodingExt`): the
// public header does, the header of runtime 0.9.0-dev.2916 does not, and the same sources
// build against both binding sets. The crates that use the extensions read the answer from
// `DEP_QAR_STREAMING_C_COLOR_EXTENSIONS` in their build scripts.
fn main() {
    println!("cargo::rerun-if-env-changed=QAROS_LIB_DIR");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(qaros_color_extensions)");
    if bindings_declare_color_extensions() {
        println!("cargo::rustc-cfg=qaros_color_extensions");
        println!("cargo::metadata=color_extensions=1");
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

fn bindings_declare_color_extensions() -> bool {
    let target = std::env::var("TARGET")
        .unwrap_or_default()
        .replace('-', "_");
    let types = format!("src/generated/{target}/types.rs");
    println!("cargo::rerun-if-changed={types}");
    std::fs::read_to_string(&types)
        .is_ok_and(|bindings| bindings.contains("pub struct QarRenderSenderAlphaModeExt"))
}
