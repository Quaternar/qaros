// The header's inline initializers are compiled with the target's C compiler:
// clang or gcc elsewhere, MSVC in C11 mode on Windows (as QAROS builds its C
// samples), where the header also enables its D3D11 types. On macOS, whose
// bindings carry the D3D11 types for qaros-stub, the D3D11 initializers are
// compiled too, against the same stand-in headers the bindings are generated
// with. A Windows target checked from another host (cargo check or clippy)
// compiles nothing: the header needs the Windows SDK there, and nothing is
// linked.
fn main() {
    // The ports of types only the public header declares exist where qaros-sys's bindings
    // declare them (see qaros-sys's build script).
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
    println!("cargo::rerun-if-changed=src/defaults.c");
    println!("cargo::rerun-if-changed=../../../qar-streaming-c/include/qar_streaming.h");
    println!("cargo::rerun-if-changed=../../xtask/shim-vulkan/vulkan/vulkan_core.h");
    println!("cargo::rerun-if-changed=../../xtask/shim-d3d11");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_is_windows = target_os == "windows";
    let host_is_windows = std::env::var("HOST").is_ok_and(|host| host.contains("windows"));
    if target_is_windows && !host_is_windows {
        return;
    }
    let mut build = cc::Build::new();
    build
        .file("src/defaults.c")
        .include("../../../qar-streaming-c/include")
        // vulkan_core.h without the Vulkan SDK; the same stand-in the
        // bindings are generated with.
        .include("../../xtask/shim-vulkan")
        .std("c11");
    if target_os == "macos" {
        build
            .include("../../xtask/shim-d3d11")
            .define("QAR_ENABLE_D3D11", None)
            // qar_sync_frame_d3d11_default initializes its array of structures with a
            // bare NULL, which C's brace elision accepts and clang warns about.
            .flag_if_supported("-Wno-missing-braces");
    }
    if !target_is_windows {
        build
            // __declspec(dllimport) is applied on every platform by the header.
            .flag("-fdeclspec")
            .flag_if_supported("-Wno-ignored-attributes")
            .flag_if_supported("-Wno-c23-extensions")
            .flag_if_supported("-Wno-gnu-empty-initializer")
            .flag_if_supported("-Wno-missing-field-initializers");
    }
    build.compile("qaros_conformance_defaults");
}
