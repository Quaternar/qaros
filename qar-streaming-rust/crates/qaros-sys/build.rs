fn main() {
    println!("cargo::rerun-if-env-changed=QAROS_LIB_DIR");
    println!("cargo::rerun-if-changed=build.rs");
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
