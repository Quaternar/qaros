# QarOS

## Prerequisites

### Windows
- CMake 3.29 or newer. Download the installer from https://cmake.org/download/ and make sure it is available from your PATH.
- Visual Studio Build Tools 2022 with the "Desktop development with C++" workload (MSVC, Windows SDK, and CMake integration). Install from https://visualstudio.microsoft.com/downloads/.
- [Optional] npm if you want to build the documentation site
- [Optional] Clangd, clang-format and clang-tidy if you want to develop in Vscode
```
winget install --id Kitware.CMake ---exact --version "3.31.8"
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id OpenJS.NodeJS.LTS ---exact
winget install --id LLVM.LLVM ---exact
```

## Install QAROS

Install QAROS on the PC with `QAROS-Setup-<version>.exe` (per machine, one administrator prompt). The examples load its runtime at run time (`qar_library_load(NULL)`); this repository needs no runtime download. Step by step: Developer Guide, *Getting Started* (`docs/website/docs/developer-guide/getting-started.mdx`).

The core examples take `installed` as their first argument to load the installed QAROS, or the path of a `qar-streaming-c.dll`. The Vulkan cube loads the installed QAROS by default.

## Documentation

- The authoritative, always up-to-date documentation is hosted at https://docs.quaternar.com/qaros/.
- A local copy of the documentation lives in `docs/website/` and can be built with the Docusaurus toolchain for offline browsing and authoring (`cd docs/website && npm install && npm run build`).
- The Knowledge Base is split into three self-contained guides by audience, each with its own navigation:
  - **User Guide** (`docs/website/docs/user-guide/`) — for people who join a Hub from a device, no code: the QAROS world, getting started, connecting to the Hub, onboarding devices, the shared space, working in a session, and troubleshooting.
  - **Developer Guide** (`docs/website/docs/developer-guide/`) — for engineers integrating the API in **C, C# or Rust**: getting started, concepts, API conventions, coordinate systems, onboarding and sessions, peers, GUI panels, rendering streams, app volumes, language bindings, troubleshooting, and the compiled **Tutorials**. Code samples are selectable per language via a site-wide switcher.
  - **Administrator & Operator Guide** (`docs/website/docs/operator-guide/`) — for people who install, secure, configure, run, and maintain a Hub: system requirements, installation, running the Hub, launcher configuration, networking and federation, the security model (and an auditor-level deep dive), device management, the mixer, and maintenance.
  - **C API Reference** — generated with Doxygen from the headers and the example sources in `qar-streaming-c/examples/`. The examples are compiled by this repository's build, so every tutorial snippet is guaranteed-valid code and doubles as the source for the Developer Guide Tutorials. The examples take an optional `[pairing-code]` argument for the first run; later runs rejoin with the persisted onboarding id.
- Deferred documentation work (unshipped features, items needing team input) is tracked in [`docs/TODO.md`](docs/TODO.md), not inside the published pages.

## Repository structure

- `cmake/` - CMake helper files, including the vcpkg toolchain integration.
- `docs/website/` - Documentation site source, authored with Docusaurus.
- `package/` - Drop-in location for an SDK build the C# tests run against (`package/bin`, see C# binding).
- `qar-streaming-c/` - C API headers, the generated single header, and compiled usage examples.
- `qar-streaming-sharp/` - the C# binding (`Quaternar.Qaros`: assembly, namespace and NuGet) and its xUnit suites.
- `qar-streaming-rust/` - the Rust binding (crates `qaros-sys` and `qaros`), Rust ports of the C examples and their tests.
- `build/` - Generated build artifacts (created by CMake; not committed).
- `CMakeLists.txt`, `CMakePresets.json`, `vcpkg.json` - Top-level project configuration and dependency manifest.

## Build and run

1. Launch a Developer PowerShell or command prompt configured for MSVC (the Visual Studio Build Tools installer provides shortcuts).
2. Configure the project with CMake presets: `cmake --preset x64-windows`.
3. Build the desired configuration, for example Debug: `cmake --build --preset x64-windows-debug` (use `x64-windows-release` for Release binaries).
4. Run the samples from `build/x64-windows/qar-streaming-c/examples/<Config>/`, for example `./build/x64-windows/qar-streaming-c/examples/Debug/dynamic_loading.exe installed`.

## C# binding

`qar-streaming-sharp/` holds the C# API and its tests. Build and test with the .NET 8 SDK:

```
cd qar-streaming-sharp
dotnet build Quaternar.Qaros.sln
dotnet test Quaternar.Qaros.sln --no-build
```

The integration tests drive the C API against `qar-streaming-c-ext`, a test-only DLL with the
counterparts an application talks to (an in-process hub, peers, receivers), so no running QAROS
is needed. It is built only in Debug and RelWithDebInfo SDK builds and never ships in a release
package. The tests copy the native binaries from `QarStreamingNativeBinDir`:

- default `package/bin`: put a Debug or RelWithDebInfo SDK build there, including
  `qar-streaming-c-ext.dll`;
- inside the Quaternar repository, its `qaros-native.props` points at that repository's own build
  (build the `qar-streaming-c-ext_gtests` target first);
- `-p:QarStreamingNativeBinDir=<folder>` overrides both.

The optional [Vulkan cube source](qar-streaming-c/examples/vulkan_cube/README.md) loads the installed QAROS, joins as a local app with one approval in QAROS (no code), and runs a separate stereo camera and sender for each target app in the session's peer list.

## Rust binding

`qar-streaming-rust/` holds the Rust binding (`qaros-sys`, the raw layer generated from
`qar-streaming-c/include/qar_streaming.h`, and `qaros`, the safe layer), Rust ports of the C
examples and the tests. Build and test with the latest stable Rust:

```
cd qar-streaming-rust
cargo build --workspace
cargo test --workspace
```

The tests run against `qaros-stub`, an in-memory stand-in for the runtime, so no QAROS is needed.
The integration tests drive the C API against `qar-streaming-c-ext`, as the C# tests do: set
`QAROS_PACKAGE_BIN` to the `bin` folder of a Debug or RelWithDebInfo SDK build, then
`cargo test -p qar-streaming-rust-ext --features e2e`. The runtime is found the way every binding
finds it, with `QAROS_RUNTIME_DIR` to point a run at another one. Details:
[qar-streaming-rust/README.md](qar-streaming-rust/README.md).

## Support channels

- Use GitHub Issues to report bugs, request features, or track regressions.
- Start a GitHub Discussion when you need help with integration details or want to collaborate on implementation approaches.
