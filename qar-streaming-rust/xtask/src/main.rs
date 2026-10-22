//! `cargo xtask bindgen [target...]` regenerates the committed bindings in
//! `crates/qaros-sys/src/generated/<target>/` from the C header.
//!
//! Each target gets two files:
//! - `types.rs`: structs, enums, constants and callback typedefs, with
//!   bindgen's compile-time layout assertions.
//! - `functions.rs`: the `qar_impl_*` declarations wrapped in the
//!   `qar_impl_function_list!` macro, which `qaros-sys` expands into the
//!   runtime-loading table and the `link`-mode externs, and `qaros-stub`
//!   expands into its exported symbols.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bindgen::{EnumVariation, RustEdition, RustTarget};

const DEFAULT_TARGETS: &[&str] = &[
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "aarch64-apple-darwin",
];

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let Some((command, command_arguments)) = arguments.split_first() else {
        print_usage();
        return ExitCode::FAILURE;
    };
    if command == "bindgen" {
        regenerate_bindings(command_arguments)
    } else {
        print_usage();
        ExitCode::FAILURE
    }
}

fn regenerate_bindings(requested_targets: &[String]) -> ExitCode {
    let targets: Vec<&str> = if requested_targets.is_empty() {
        DEFAULT_TARGETS.to_vec()
    } else {
        requested_targets.iter().map(String::as_str).collect()
    };

    for target in targets {
        match generate_bindings(target) {
            Ok(output_directory) => println!("{target}: wrote {}", output_directory.display()),
            Err(message) => {
                eprintln!("{target}: {message}");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}

fn print_usage() {
    eprintln!("usage: cargo xtask bindgen [target...]");
    eprintln!("default targets: {}", DEFAULT_TARGETS.join(", "));
    eprintln!("set LIBCLANG_PATH if libclang is not found automatically");
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

/// Where and how one generated binding set is written.
struct BindingOutput<'a> {
    command: &'a str,
    directory: PathBuf,
    header_label: &'a str,
    macro_name: &'a str,
}

/// Clang and bindgen settings.
fn common_builder(target: &str, root: &Path) -> Result<bindgen::Builder, String> {
    // The header applies __declspec(dllimport) on every platform; clang
    // accepts it outside Windows only with -fdeclspec.
    let declspec_arguments: &[&str] = if target.contains("windows") {
        &[]
    } else {
        &["-fdeclspec"]
    };
    // The header enables its D3D11 types and functions on Windows only. The development
    // target has no runtime, so its bindings serve qaros-stub alone; binding D3D11 there too
    // lets the stub emulate the D3D11 render sender and qaros test its `d3d11` feature
    // without Windows. Linux, which has a runtime, keeps the header's own choice.
    let d3d11_arguments: &[&str] = if target.contains("apple") {
        &["-DQAR_ENABLE_D3D11"]
    } else {
        &[]
    };
    let rust_target = RustTarget::stable(85, 0).map_err(|error| error.to_string())?;
    Ok(bindgen::Builder::default()
        .clang_args(declspec_arguments)
        .clang_args(d3d11_arguments)
        .clang_arg(format!("--target={target}"))
        .clang_arg("-ffreestanding")
        .clang_arg("-isystem")
        .clang_arg(root.join("xtask/shim").to_string_lossy())
        .clang_arg("-isystem")
        .clang_arg(root.join("xtask/shim-d3d11").to_string_lossy())
        .clang_arg("-isystem")
        .clang_arg(root.join("xtask/shim-vulkan").to_string_lossy())
        // Binds the Vulkan types and functions on every target; qaros puts
        // its Vulkan API behind the `vulkan` feature.
        .clang_arg("-DQAR_ENABLE_VULKAN")
        .clang_arg("-Wno-gnu-empty-initializer")
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_debug(true)
        .derive_partialeq(true)
        .derive_default(false)
        .layout_tests(true)
        .rust_target(rust_target)
        .rust_edition(RustEdition::Edition2024))
}

fn generate_bindings(target: &str) -> Result<PathBuf, String> {
    let root = workspace_root();
    let header = root.join("../qar-streaming-c/include/qar_streaming.h");
    let builder = common_builder(target, &root)?
        .header(header.to_string_lossy())
        .allowlist_function("qar_impl_.*")
        .allowlist_type("Qar.*")
        .allowlist_type("qar_.*_callback_t")
        .allowlist_var("QAR_.*")
        .blocklist_type("qar_.*_func_t")
        .generate_comments(true);
    write_bindings(
        builder,
        &BindingOutput {
            command: "bindgen",
            directory: root
                .join("crates/qaros-sys/src/generated")
                .join(target.replace('-', "_")),
            header_label: "qar-streaming-c/include/qar_streaming.h",
            macro_name: "qar_impl_function_list",
        },
        target,
    )
}

fn write_bindings(
    builder: bindgen::Builder,
    output: &BindingOutput<'_>,
    target: &str,
) -> Result<PathBuf, String> {
    let bindings = builder
        .generate()
        .map_err(|error| format!("bindgen failed: {error}"))?;
    let parsed = syn::parse_file(&bindings.to_string())
        .map_err(|error| format!("bindgen output does not parse: {error}"))?;
    let (type_items, function_items) = split_functions_from_types(parsed.items);
    if function_items.is_empty() {
        return Err("no qar_impl_* functions found; check the allowlist".to_string());
    }

    std::fs::create_dir_all(&output.directory)
        .map_err(|error| format!("cannot create {}: {error}", output.directory.display()))?;
    let types_path = output.directory.join("types.rs");
    write_file(
        &types_path,
        &(banner(target, output) + &render_types(type_items)),
    )?;
    write_file(
        &output.directory.join("functions.rs"),
        &render_function_list(target, output, function_items),
    )?;
    format_with_rustfmt(&types_path)?;
    Ok(output.directory.clone())
}

/// `cargo fmt` reaches the generated modules too, so the committed files are
/// kept in rustfmt's layout and regeneration produces no formatting diff.
fn format_with_rustfmt(path: &Path) -> Result<(), String> {
    let status = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path)
        .status()
        .map_err(|error| format!("cannot run rustfmt: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("rustfmt failed on {}", path.display()))
    }
}

fn split_functions_from_types(items: Vec<syn::Item>) -> (Vec<syn::Item>, Vec<syn::ForeignItemFn>) {
    let mut type_items = Vec::new();
    let mut function_items = Vec::new();
    for item in items {
        match item {
            syn::Item::ForeignMod(foreign_module) => {
                for foreign_item in foreign_module.items {
                    if let syn::ForeignItem::Fn(mut function) = foreign_item {
                        function.attrs.clear();
                        function_items.push(function);
                    }
                }
            }
            other => type_items.push(other),
        }
    }
    function_items.sort_by_key(|function| function.sig.ident.to_string());
    (type_items, function_items)
}

fn banner(target: &str, output: &BindingOutput<'_>) -> String {
    format!(
        "// Generated by `cargo xtask {}` from {} for {target}.\n\
         // Do not edit by hand; regenerate instead.\n\n",
        output.command, output.header_label
    )
}

fn render_types(items: Vec<syn::Item>) -> String {
    let file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    };
    prettyplease::unparse(&file)
}

fn render_function_list(
    target: &str,
    output_settings: &BindingOutput<'_>,
    functions: Vec<syn::ForeignItemFn>,
) -> String {
    let extern_block: syn::ItemForeignMod = syn::parse_quote! {
        unsafe extern "C" {}
    };
    let extern_block = syn::ItemForeignMod {
        items: functions.into_iter().map(syn::ForeignItem::Fn).collect(),
        ..extern_block
    };
    let formatted = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: vec![syn::Item::ForeignMod(extern_block)],
    });
    let declaration_lines: Vec<&str> = formatted.lines().collect();
    let inner_lines = &declaration_lines[1..declaration_lines.len().saturating_sub(1)];

    let mut output = banner(target, output_settings);
    output.push_str("/// Invokes `$callback!` with every exported `qar_impl_*` declaration.\n");
    output.push_str("#[macro_export]\n");
    output.push_str("macro_rules! ");
    output.push_str(output_settings.macro_name);
    output.push_str(" {\n");
    output.push_str("    ($callback:ident) => {\n");
    output.push_str("        $callback! {\n");
    for line in inner_lines {
        if line.is_empty() {
            output.push('\n');
        } else {
            output.push_str("        ");
            output.push_str(line);
            output.push('\n');
        }
    }
    output.push_str("        }\n");
    output.push_str("    };\n");
    output.push_str("}\n");
    output
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    std::fs::write(path, content)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))
}
