// `qaros_color_extensions` is set where qaros-sys's bindings declare the render sender's colour
// extensions (its build script reports them as `DEP_QAR_STREAMING_C_COLOR_EXTENSIONS`), so the
// stub checks them in a render sender's chain.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(qaros_color_extensions)");
    if std::env::var_os("DEP_QAR_STREAMING_C_COLOR_EXTENSIONS").is_some() {
        println!("cargo::rustc-cfg=qaros_color_extensions");
    }
}
