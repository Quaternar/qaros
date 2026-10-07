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
  - **Developer Guide** (`docs/website/docs/developer-guide/`) — for engineers integrating the API in **C or C#**: getting started, concepts, API conventions, coordinate systems, onboarding and sessions, peers, GUI panels, rendering streams, app volumes, language bindings, troubleshooting, and the compiled **Tutorials**. Code samples are selectable per language via a site-wide switcher.
  - **Administrator & Operator Guide** (`docs/website/docs/operator-guide/`) — for people who install, secure, configure, run, and maintain a Hub: system requirements, installation, running the Hub, launcher configuration, networking and federation, the security model (and an auditor-level deep dive), device management, the mixer, and maintenance.
  - **C API Reference** — generated with Doxygen from the headers and the example sources in `qar-streaming-c/examples/`. The examples are compiled by this repository's build, so every tutorial snippet is guaranteed-valid code and doubles as the source for the Developer Guide Tutorials. The examples take an optional `[pairing-code]` argument for the first run; later runs rejoin with the persisted onboarding id.
- Deferred documentation work (unshipped features, items needing team input) is tracked in [`docs/TODO.md`](docs/TODO.md), not inside the published pages.

## Repository structure

- `cmake/` - CMake helper files, including the vcpkg toolchain integration.
- `docs/website/` - Documentation site source, authored with Docusaurus.
- `qar-streaming-c/` - C API headers, the generated single header, and compiled usage examples.
- `build/` - Generated build artifacts (created by CMake; not committed).
- `CMakeLists.txt`, `CMakePresets.json`, `vcpkg.json` - Top-level project configuration and dependency manifest.

## Build and run

1. Launch a Developer PowerShell or command prompt configured for MSVC (the Visual Studio Build Tools installer provides shortcuts).
2. Configure the project with CMake presets: `cmake --preset x64-windows`.
3. Build the desired configuration, for example Debug: `cmake --build --preset x64-windows-debug` (use `x64-windows-release` for Release binaries).
4. Run the samples from `build/x64-windows/qar-streaming-c/examples/<Config>/`, for example `./build/x64-windows/qar-streaming-c/examples/Debug/dynamic_loading.exe installed`.

The optional [Vulkan cube source](qar-streaming-c/examples/vulkan_cube/README.md) loads the installed QAROS, joins as a local app with one approval in QAROS (no code), and runs a separate stereo camera and sender for each rendering target discovered through render requests.

## Support channels

- Use GitHub Issues to report bugs, request features, or track regressions.
- Start a GitHub Discussion when you need help with integration details or want to collaborate on implementation approaches.
