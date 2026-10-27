# QAROS Rust binding

Rust bindings for the QAROS streaming C API (`qar_streaming.h`): raw FFI in `qaros-sys`, a safe
API in `qaros`, and Rust ports of the C examples. The bindings load the same
`qar-streaming-c.dll` as the C and C# APIs.

## Requirements

- The latest stable Rust, through [rustup](https://rustup.rs).
- To run against QAROS: Windows with the MSVC toolchain (`stable-x86_64-pc-windows-msvc`, Visual
  Studio Build Tools 2022 with "Desktop development with C++") and QAROS installed, or a QAROS
  development hub whose instance `QAR_INSTANCE` names ([Loading the runtime](#loading-the-runtime)).
- A QAROS runtime that exports every function of the header the bindings are generated from
  ([qar-streaming-c/include/qar_streaming.h](../qar-streaming-c/include/qar_streaming.h) in this
  repository), and that is the runtime of the Hub you join. The binding loads all of the
  header's functions or none, so an
  older runtime fails to load, and the error names the functions it lacks.
- To build and test without QAROS: nothing more. The tests run against `qaros-stub`, an
  in-memory stand-in for the runtime.

## Build

```sh
cargo build --workspace
```

Committed bindings cover `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu` and
`aarch64-apple-darwin` (macOS on Apple silicon), a development target without a QAROS runtime,
where the tests and examples run against `qaros-stub`. The Linux build is checked by
cross-compilation. On any other target
the build stops with a message naming the command that adds it (see
[Regenerating the bindings](#regenerating-the-bindings)).

## Use

The crates are not published to crates.io. Add `qaros` to the `[dependencies]` section of your
crate's `Cargo.toml` as a git dependency on the QAROS repository; Cargo finds the crate by its
name inside it:

```toml
[dependencies]
qaros = { git = "https://github.com/Quaternar/qaros" }
```

Pin it with `rev = "<commit>"` or `tag = "<tag>"` on the same line, so the header the binding is
generated from stays the one your runtime exports in full ([Requirements](#requirements)).

With a clone of the repository, use a path to its `qar-streaming-rust/crates/qaros` folder
instead, relative to your `Cargo.toml`:

```toml
[dependencies]
qaros = { path = "../qaros/qar-streaming-rust/crates/qaros" }
```

Add `features = ["vulkan"]` for the Vulkan render sender or `features = ["d3d11"]` for the D3D11
render sender (Windows) to either line.

```rust,no_run
use qaros::{AppState, AppVolumeConfig, BlockingOptions, Eye, FrameViewSpec, Library, LibraryConfig, OnboardMethod, PeerPresentation, PixelFormat, RenderSenderConfig, RuntimeConfig, ShowParams, ViewKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: the runtime found (the installed QAROS, or that of the instance QAR_INSTANCE
    // names) matches these bindings.
    let library = unsafe { Library::load_default(&LibraryConfig::default())? };
    let runtime = library.create_runtime(&RuntimeConfig::default())?;
    // On the Hub machine (Windows), join as a local app: the first start waits until the user
    // clicks Allow in QAROS, and there is nothing to persist.
    let session = runtime
        .onboard(
            &OnboardMethod::LocalApp { approval_timeout: None, app_tag: Some("main.my-app.example") },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )?
        .session;

    // Targets are the running target apps in the peer list;
    // session.subscribe_peer_updates(forward) reports each change to the list.
    let target = session
        .peers()?
        .into_iter()
        .find(|peer| peer.is_target_app && peer.app_state == AppState::Running)
        .ok_or("no running target app in the session")?;

    // A sender needs an app volume, and frames carry view poses only for declared views.
    let volume = session.app_volumes().get_or_create(&AppVolumeConfig::new("my-app"))?;
    let mut config = RenderSenderConfig::new(target.id);
    config.app_volume = Some(volume);
    config.color_format = PixelFormat::R8G8B8A8;
    config.depth_format = PixelFormat::D32Float;
    config.views = vec![
        FrameViewSpec { kind: ViewKind::Color, eye: Eye::Left, texture_index: 0 },
        FrameViewSpec { kind: ViewKind::Color, eye: Eye::Right, texture_index: 0 },
        FrameViewSpec { kind: ViewKind::Depth, eye: Eye::Left, texture_index: 1 },
        FrameViewSpec { kind: ViewKind::Depth, eye: Eye::Right, texture_index: 1 },
    ];
    let mut sender = session.create_render_sender(&config, &BlockingOptions::default())?;
    let mut frame = sender.begin_frame(&BlockingOptions::default())?;
    // frame.view(0)? is the pose and field of view to render the first declared view with.
    {
        let mut textures = frame.cpu_frame()?.textures;
        // Texture 0 holds both color views: RGBA8 rows, textures[0].pitch bytes apart.
        textures[0].data.fill(0x80);
        // Texture 1 holds both depth views: one f32 per pixel, 0 at near and 1 at far.
        for depth in textures[1].data.chunks_exact_mut(4) {
            depth.copy_from_slice(&1.0_f32.to_ne_bytes());
        }
    }
    frame.show(&ShowParams { near: 0.1, far: 10.0, depth_scale: 1.0 })?;
    Ok(())
}
```

Design rules, following the SDK's API conventions:

- Every fallible call returns `qaros::Result`. A C failure becomes `Error::Api` with the function
  name, the `StatusCode` and the text of `qar_result_message`. Check expected outcomes with
  `error.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND)` and similar. Begin-frame wraps a
  paused, reconnecting or transfer-less stream and an unresolved app volume inside
  `RENDERING_UNABLE_TO_BEGIN_FRAME`; `error.has_cause(...)` sees those causes, as
  `qar_result_has_code` does in C.
- Init structures come from ports of the `qar_*_default()` initializers, so every
  `QarStructureHeader` is stamped.
- Handles release themselves on drop. Each keeps its parent alive (render sender, session,
  runtime, library), so the C teardown order holds whatever order Rust drops them in.
  `qar_library_init` is process-wide: a second `Library` is refused while one is alive. The DLL
  stays mapped for the rest of the process. After a clean `qar_library_destroy`, a later load
  initializes it again; after a failed one, it returns `Error::LibraryAlreadyLoaded`.
- Teardown never blocks on a cancelled asynchronous call. A session dropped with cancelled sender
  creations in flight waits at most `CANCELLED_CREATION_WAIT` (500 ms) for their callbacks. A
  session still waiting after that is leaked with its runtime and the library instead of
  destroyed, since destroying it would block.
- Runtime, session and the other handles are not `Send`: the SDK is single-threaded per runtime.
  A `CancelToken` is cancelled from another thread (a Ctrl+C handler, a stdin watcher) through
  its `cancel_handle()`.
- One frame is open per sender, from `begin_frame` until `show` or drop. `begin_frame` returns a
  `Frame` that borrows the sender; `begin_owned_frame` returns an `OwnedFrame` that keeps the
  sender alive, for renderers that fill the frame several UI frames later.
  `try_begin_owned_frame` polls for a frame without blocking. The async
  `qar_render_sender_begin_frame_async` is not wrapped, since the runtime never calls back for a
  cancelled one; it is reachable through `raw_api()` with the sender's `as_raw()`.
- Several users are several senders: one `RenderSender` per target peer.
- A sender declares its colour through `RenderSenderConfig`: `alpha_mode` (unspecified by
  default) and `color_encoding` (sRGB by default) chain their extension only when set to
  something else. `PixelFormat::D16Unorm` works on every graphics API, the depth-stencil formats
  on Vulkan only; a format the API cannot take is
  `StatusCode::RENDERING_DEPTH_FORMAT_NOT_SUPPORTED`. With bindings from a header without the
  colour extensions, such as that of 0.9.0-dev.2916, setting either is `Error::InvalidArgument`.
- Subscription callbacks run on library threads and receive owned snapshots. `qaros::channel()`
  forwards them into a `std::sync::mpsc` receiver.
- The Vulkan render sender is behind the feature `vulkan`, with `ash` 0.38 types:
  `Session::create_vulkan_render_sender` and `OwnedFrame::vulkan_frame`. API and safety rules are
  in [docs/vulkan-api.md](docs/vulkan-api.md).
- The D3D11 render sender is behind the feature `d3d11`, with `windows` crate 0.62 types:
  `Session::create_d3d11_render_sender` and `OwnedFrame::d3d11_frame`. It runs on Windows; on
  macOS the feature builds it only for the tests against `qaros-stub`, and on Linux the feature
  adds nothing. The sender keeps a reference on the device and context. API and safety rules
  are in [docs/d3d11-api.md](docs/d3d11-api.md).
- Targets are selected by `PeerInfo::app_state`, which the header derives from the peer
  presence: `Running` for an `ONLINE` peer or one the Hub has not reconciled (`UNKNOWN`),
  `ShuttingDown` for an `OFFLINE` or `LEAVING` one, which may come back, and `Retired` for one
  that never does. The developer guide reads the presence itself, which is raw only:
  `qar_peer_spec_get_presence` through `library.raw_api()` with the session's `as_raw()`.
- Hands, gestures, world anchors and the other areas the [coverage table](#coverage) lists as raw
  only are reached through `library.raw_api()`, the `qaros_sys::QarApi` table, with the C handle
  of a safe value: `as_raw()` on `Runtime`, `Session`, `RenderSender`, `CancelToken`,
  `OnboardingInvite` and `MemberHubInvitation`. Taking the pointer is safe; using it is `unsafe`: it is valid while the safe
  value lives, the safe value destroys it (never the caller), and the session's threading rules
  apply.

`cargo doc --no-deps --open` builds the API reference.

## Coverage

The areas of the C header (its sections, and the groups of functions inside the larger ones),
what the safe `qaros` API covers, what is reached only through `library.raw_api()`, and whether
the C# binding (`qar-streaming-sharp/src`, at the time of writing) has it in its public API. Raw
only means the function is in `qaros_sys::QarApi` for the header the bindings are generated
from, called with the `as_raw()` handle of the safe value it takes (a `Session`, `Runtime`,
`RenderSender`, `CancelToken`, `OnboardingInvite` or `MemberHubInvitation`).

\* These bindings are generated from the public header in this repository
(`qar-streaming-c/include/qar_streaming.h`), which declares the functions marked \* (peer
presence, role, category and device identity, member hubs, the Android Java VM) and the types
of the render sender's colour and depth declarations. A runtime without those functions, such
as 0.9.0-dev.2916, does not load. Bindings from such a header build without the safe member-hub
functions (`MemberHubInvitation` then has no value, so `OnboardExtensions::remote_app_grant`
stays `None`) and without `Session::update_device_identity`; a set
`PeerPresentation::device_identity` is `Error::InvalidArgument` there.

| Area | Safe in Rust | Raw only | C# has it |
| --- | --- | --- | --- |
| Dynamic loading | `Library::load_default`, `Library::load`, `Library::linked` (feature `link`), `installed_bin_folder` | | yes (`InstalledRuntime`) |
| Library initialization, runtime info | `Library` (initialized on load, destroyed on drop), `Library::runtime_info` | | yes |
| Android Java VM | | `qar_platform_set_android_java_vm` \* | no |
| Error handling | `qaros::Error` with function, `StatusCode`, message and wrapped causes (`has_code`, `has_cause`) | the `qar_result_*` constructors and predicates, `qar_error_wrap_result`, `qar_result_log_if_error` | yes (`ResultInfo`, `StatusCode`) |
| Cancellation | `CancelToken` with or without a timeout, `is_timed_out`, `CancelHandle` from another thread | `qar_cancel_token_cancel_after` | partly: a `CancellationToken` bridged to an internal native token; no timeout token or timeout query |
| Runtime | `Runtime` | | yes |
| Onboarding: pairing code, remote Hub, invite, local app and its app tag; rejoin; forget | blocking: `Runtime::onboard`, `onboard_with` (`OnboardExtensions`: the adapter, the planned peer id, re-enrolling a slot, a remote-app grant \*), `rejoin`, `forget`, `OnboardMethod`, `RuntimeConfig::ephemeral_storage` | | yes |
| Co-process invites and the invite handle | `Session::request_onboarding_invite`, `OnboardingInvite` (serialize, deserialize, expiry, method, `code`) | `qar_onboarding_invite_handle_is_valid` | yes |
| Onboarding with a declared device | `PeerPresentation::device_identity` (`DeviceIdentity`, `SourceDeviceCategory`) at onboard and rejoin, `Session::update_device_identity` \* | | yes (`PeerPresentationOptions.WithDeviceIdentity`) |
| Member hubs | `Runtime::create_member_hub_invitation` (`MemberHubInvitationConfig`, `MemberHubPermissions`, the remote-app grant as `app_grant_tag`), `Runtime::join_hub_as_member` (`JoinHubAsMemberConfig`), `MemberHubInvitation` (serialize, destroyed on drop), `Library::deserialize_member_hub_invitation`, redeeming a grant with `OnboardExtensions::remote_app_grant` \* | | yes |
| Async calls | `Runtime::start_onboard`, `Runtime::start_rejoin` with `PendingOnboard`; `Session::start_render_sender` and its Vulkan and D3D11 forms, with `PendingRenderSender` | `qar_session_request_onboarding_invite_async`, `qar_session_invite_target_app_async`, `qar_session_kill_target_app_async`, `qar_render_sender_begin_frame_async` | yes (`Task` forms) |
| Session | `Session::id`, `Session::graphics_adapter` | `qar_session_find_render_target` | yes |
| Session lifecycle | `Session::lifecycle`, `Session::subscribe_lifecycle`, `SessionLifecycle` | | yes |
| Target apps | | invite and kill a target app, `qar_target_app_connection_string_*` | yes |
| Peers: list, own spec, display name, updates | `Session::peers`, `my_peer`, `set_display_name`, `subscribe_peer_updates`; `PeerInfo` with the app state and `is_target_app` | | yes |
| Peers: presence, role, category, device identity | | `qar_peer_spec_get_presence`, `_get_role`, `_get_category`, `_get_device_id`, `_get_device_family`, `_get_device_name` \* | yes |
| Render requests | `Session::subscribe_render_requests`, `RenderRequest` | | yes |
| CPU render sender | creation (blocking and polled), layout, begin, try-begin, `CpuFrame`, show | | yes |
| Vulkan render sender | feature `vulkan`: `create_vulkan_render_sender`, `VulkanFrame` | | yes |
| D3D11 render sender | feature `d3d11` (Windows; on macOS only for the tests against `qaros-stub`): `create_d3d11_render_sender`, `D3D11Frame` | | yes |
| OpenGL render sender | not bound: the bindings are generated without `QAR_ENABLE_OPENGL` | | yes |
| Render sender colour and depth declarations | `RenderSenderConfig::alpha_mode` (`ColorAlphaMode`) and `color_encoding` (`ColorEncoding`), chained as `QarRenderSenderAlphaModeExt` and `QarRenderSenderColorEncodingExt`; the depth formats `PixelFormat::D16Unorm`, `D24UnormS8Uint`, `D32FloatS8X24Uint` \* | | yes |
| More of the render sender | | `qar_render_sender_stream_id`, `qar_render_sender_change_layout`, view overrides at show | yes |
| Hands and gestures | | `qar_render_sender_last_hands`; the gesture events, configuration and modes of GUI panels and app volumes | yes |
| Video senders (primaryless) | | every function but the OpenGL frame, which is not bound | yes |
| GUI panels | `GuiPanels`: get or create, pose, size, state, navigate, close, list (with the peers a panel is visible to), updates | `qar_gui_panels_update_visible_to`, `qar_gui_panel_get_content_type` | yes |
| GUI panel streams | | `qar_gui_panels_set_stream`, `qar_gui_panel_get_content_stream_id` | yes |
| App volumes | `AppVolumes`: get or create, close, rename, pose, size, app pose and scale, latest values, list, updates | the peers using a volume | yes |
| World anchors | | change, clear and read an app volume's world anchor | yes |
| Editing state | | start and stop editing, the editing status | yes |
| Time points | | `QarTimePoint` inside raw records; the header has no time function | yes (`TimePoint`) |
| UUIDs and ids | `Library::uuid_to_string`, `uuid_from_string`, the id types | `qar_peer_id_unique`, `qar_session_id_unique`, `qar_gui_panel_id_unique` | yes |

The C# binding P/Invokes every exported function but `qar_peer_spec_is_target_app`, which it
derives from the role, and `qar_platform_set_android_java_vm`.

## Loading the runtime

`Library::load_default` finds the runtime the way every QAROS binding does (QAROS developer
guide, "Authoring a Language Binding"; the C header's `qar_library_load(NULL)`). The first match
wins:

| Step | Where | Notes |
| --- | --- | --- |
| 0 | A `qar-streaming-c` already loaded in the process | Nothing is loaded; a reference is taken. |
| 1 | Beside the binding | The folder of the executable or library the binding is compiled into: a test binary, an unpacked development ZIP, an app built into a QAROS tree. |
| 2 | The runtime folder of the QAROS instance `QAR_INSTANCE` names | Windows; elsewhere there are no instance records and the variable is not read. `default` is the installed QAROS (step 3's folder); any other name is the `RuntimeFolder` recorded under `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>` (a QAROS development hub records its build output there and sets `QAR_INSTANCE` for the apps it starts), else that record's `<InstallFolder>\bin`; only fully qualified folders count. When the variable is set, a name that is not 1-64 of `A-Z a-z 0-9 . _ -`, an unknown instance, or a runtime missing there or failing to load is an error, never a reason to load another instance's runtime. |
| 3 | The installed QAROS, `<InstallFolder>\bin` | Windows. `InstallFolder` is read from `HKLM\SOFTWARE\Quaternar\QAROS`, never `HKCU`. An installed runtime missing or failing to load is an error. |
| 4 | Each folder on `PATH`, in order | Elsewhere: the dynamic linker's own search. |

- The current directory is never searched, and the process's DLL search path is not changed.
  Every candidate loads from its absolute path with
  `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32`, so the runtime's
  dependencies come from its own folder and System32 only.
- Nothing found is `Error::Load(LoadError::NotFound { attempts })`, whose message names every
  place looked at and what was there.
- `Library::load(path)` loads one library file and nothing else, with the same flags. It is for
  development builds of QAROS; to point a run at another QAROS instance's runtime without code,
  set `QAR_INSTANCE` (an app a QAROS hub starts receives it).
- `RuntimeConfig::runtime_binaries_directory` stays `None` by default: the runtime then uses the
  folder it was loaded from, where it finds `qar-runtime-launcher`.
- Diagnostics: `qaros::installed_bin_folder()`, `qaros::requested_instance()` (the value of
  `QAR_INSTANCE`), `qaros::instance_runtime_folder(name)` (the folder step 2 loads from),
  `Library::library_path()` (the file loaded) and `Library::lookup_step()` (the step that found
  it).

A distributed application uses the default lookup and ships no copy of the runtime: a copy beside
it would be loaded first, and a Hub verifies only the runtime it installed.

## Examples

`examples/` ports the C examples: `dynamic_loading`, `onboarding_and_rejoin`,
`app_volume_management`, `gui_panel_operations`, `cpu_rendering_visualizer`,
`local_app_onboarding`, `vulkan_cube` (feature `vulkan`) and `d3d11_render_sender` (Windows; the
examples enable `qaros/d3d11`). They load the runtime with the default lookup. A development build or the stub
is named with `--qaros-library=PATH`, else `QAROS_DLL`, else the first line of
`qaros-library.txt` beside the executable (`vulkan_cube` and `local_app_onboarding` take no
path, as their C originals). The developer guide's Rust tutorials walk through each of them
([Tutorials](#tutorials)).

On Windows, on the Hub machine, with QAROS installed, or with `QAR_INSTANCE` naming the instance
of a running QAROS development hub. Joining as a local app starts the Hub when it is not
running; a running Hub is left as it is, and the Hub keeps running after the example exits. The
rendering examples also need a target app
in the session, such as the Visualizer:

```powershell
# Loads the runtime, resolves every qar_impl_* symbol, prints the runtime version.
cargo run -p qaros-examples --bin dynamic_loading

# Join as a local app: the first run of each executable asks for Allow in QAROS.
cargo run -p qaros-examples --bin local_app_onboarding -- [approval-timeout-ms]
cargo run -p qaros-examples --bin gui_panel_operations
cargo run -p qaros-examples --bin app_volume_management
cargo run --release -p qaros-examples --bin cpu_rendering_visualizer -- --frames=300
cargo run --release -p qaros-examples --bin d3d11_render_sender -- --frames=300

# Onboard with the Hub's pairing code once; later runs rejoin. No Hub address is passed, so
# the code finds only your own Hub, on the Hub machine (see the Onboarding and Rejoin tutorial).
cargo run -p qaros-examples --bin onboarding_and_rejoin -- <PAIRING-CODE>
cargo run -p qaros-examples --bin onboarding_and_rejoin -- --forget
```

`gui_panel_operations`, `app_volume_management`, `cpu_rendering_visualizer` and
`d3d11_render_sender` join the Hub as a local app on Windows (`join_session` in
`examples/src/lib.rs`). A local app has the zero onboarding id, so nothing is stored. A refused
local-app join exits with `local_app_onboarding`'s codes: 2 when no runtime loads (that sample
only; the others exit 1), 3 to 7 for a Hub that cannot be reached, an approval denied,
unanswered, a Hub that is not the installed QAROS, and an app that no longer matches its approval.
They also take a pairing code as their first argument, as their C originals do: they rejoin with
a stored onboarding id first, then onboard with the code and store the new id as
`<example>.onboarding-id.txt` in the working directory. Like `onboarding_and_rejoin`, they pass
no Hub address, so the code finds only your own Hub, on the Hub machine: through the
`hub-endpoint.json` file the Hub writes there, then through a discovery beacon whose owner is
your user. A Hub elsewhere needs its host and port, `HubAddress` in `OnboardMethod::Code`.
`onboarding_and_rejoin` keeps one per
runtime, `onboarding_and_rejoin.primary-onboarding-id.txt` and
`onboarding_and_rejoin.sibling-onboarding-id.txt`.

The pairing code comes from a Hub onboarding screen when one shows it, or from the Hub's
operators, who mint pairing codes and addressed codes (`qar-code:...`); an app cannot mint one.
`OnboardingInvite::code` (`qar_onboarding_invite_get_code`) reads the code text of a pairing-code
invite. A local app mints co-process invites instead (`Session::request_onboarding_invite`), for a
process it starts on the same PC, which onboards with `OnboardMethod::Invite`.

`vulkan_cube` ports the C `vulkan_cube` source app:

```powershell
cargo run --release -p qaros-examples --features vulkan --bin vulkan_cube
```

- It takes no options. It loads the runtime with the default lookup and joins the Hub as a local
  app on the Hub machine; on the first run, click **Allow** in the QAROS prompt. Later runs join
  without a prompt while the executable is unchanged.
- It creates its own `ash` device on the session's GPU (`Session::graphics_adapter`; the GPU
  needs the external memory and semaphore extensions) and renders a spinning cube in stereo
  color and depth for every running target app, inside the "Vulkan Cube" app volume. Each frame
  goes through the hand-off in [docs/vulkan-api.md](docs/vulkan-api.md). The shaders are the C
  sample's GLSL, checked in as SPIR-V (`examples/shaders/`), so the build needs no shader
  compiler.
- Ctrl+C stops it, and so does the end of stdin when stdin is a pipe (QAROS's Stop for an app it
  started). A stop during the approval wait cancels the wait. Exit codes as in C: 2 when no
  runtime loads, 3 when the library fails to initialize, 4 on any other failure.
- It serves all targets from one loop with `try_begin_owned_frame`, since `qaros` handles are
  not `Send`; the C sample runs a thread per target.

`cpu_rendering_visualizer` and `d3d11_render_sender` start from a console, as their C originals
do, join as above and end after `--frames=N` frames (300 by default).
`cpu_rendering_visualizer` floats a sphere 25 cm in radius in the "CPU Tutorial Volume" app
volume for every running target app. `d3d11_render_sender` ports `d3d11_render_sender.c` on the
safe D3D11 sender (`qaros::d3d11`, with the `windows` crate for the device): it renders for the
first running target app, on a D3D11.1 device on the session's adapter, inside the "D3D11
Tutorial Volume", the left eye pulsing red and the right eye blue. Elsewhere it prints that it
requires Windows and exits 0.

Import-library mode needs `qar-streaming-c.lib`, which the runtime package does not ship:

```powershell
$env:QAROS_LIB_DIR = "<directory of qar-streaming-c.lib>"
$env:PATH = "<runtime bin folder>;$env:PATH"
cargo run -p qaros-examples --features link --bin dynamic_loading
```

## Tests

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

The tests need no QAROS package and run on every target the bindings cover. They load
`qaros-stub`, a library exporting every `qar_impl_*` symbol with in-memory behaviour. It logs
each call and flags double destroys, wrong destroy functions, unstamped headers and out-of-order
teardown. `crates/qaros-conformance` compiles the header's inline default initializers with the
C compiler (MSVC in C11 mode on Windows; on macOS with the D3D11 stand-in headers, so the D3D11
initializers are checked on both) and compares them with the Rust ports. The runtime lookup's
order is tested against a simulated process, environment and registry
(`crates/qaros-sys/src/lookup.rs`), and `Library::load_default` once against a copy of the stub
(`crates/qaros/tests/default_lookup.rs`): on Windows through `QAR_INSTANCE` and a per-user
instance record unique to the run, removed afterwards; elsewhere it checks that `QAR_INSTANCE`
is not read.

- The D3D11 tests (`crates/qaros/tests/stub/d3d11_path.rs`, with the feature `d3d11`, which the
  examples turn on for the workspace: on Windows, and on macOS for these tests) run against the
  stub's D3D11
  emulation, which creates no Direct3D object; on Windows one of them counts the references on a
  real WARP device. What the emulation proves is in [docs/d3d11-api.md](docs/d3d11-api.md).
- The Vulkan tests (`cargo test -p qaros --features vulkan`) create a real device. Without a
  Vulkan loader and driver they print `skipped` and pass; `QAROS_REQUIRE_VULKAN=1` makes that a
  failure. On macOS the Vulkan loader is not on the library path the tests search, and a
  `DYLD_LIBRARY_PATH` set for `cargo test` does not reach them: build the test binary with
  `--no-run` and run it yourself, as shown for `vulkan_cube` below. `--all-features` also
  enables `link`, which needs the import library.
- On a host without Windows, check the Windows build as well:
  `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc --all-features -- -D warnings`
  (after `rustup target add x86_64-pc-windows-msvc`).
- The examples run against the stub. Build it first with `cargo build -p qaros-stub`, then set
  `QAROS_DLL` to the absolute path of the library it builds: `target/debug/libqaros_stub.dylib`
  on macOS, `target\debug\qaros_stub.dll` on Windows, `target/debug/libqaros_stub.so` on Linux.
  Pass any pairing code, for example
  `cargo run -p qaros-examples --bin cpu_rendering_visualizer -- ANY-CODE --frames=3`. The stub
  refuses the codes `WRONG` (PAKE error) and `UNREACHABLE` (hub unreachable). The stub's D3D11
  emulation lends placeholder addresses, not textures, so `d3d11_render_sender` submits its
  frames without drawing there. A stub run stores its onboarding id in the working directory
  as a real run does, and a later run against QAROS from the same folder tries to rejoin with
  it first: run the stub from a folder of its own, or delete `<example>.onboarding-id.txt` (and
  `onboarding_and_rejoin`'s `*-state` folders) before a real run.
- `vulkan_cube` has a test of its own that renders one frame through the stub on the first
  supported GPU and reads both eyes back:
  `QAROS_REQUIRE_VULKAN=1 cargo test -p qaros-examples --features vulkan --bin vulkan_cube`.
  On macOS, build the test binary, then run it with `DYLD_LIBRARY_PATH` naming the folder of
  `libvulkan.dylib` (`/opt/homebrew/lib` for the Homebrew Vulkan loader):

  ```sh
  cargo test -p qaros-examples --features vulkan --bin vulkan_cube --no-run
  # The last line names the binary: target/debug/deps/vulkan_cube-<hash>
  QAROS_REQUIRE_VULKAN=1 DYLD_LIBRARY_PATH=/opt/homebrew/lib target/debug/deps/vulkan_cube-<hash>
  ```
- Test drivers can add and remove peers (`qar_stub_peer_join`, `qar_stub_peer_leave`, or
  `QAROS_STUB_SCENARIO="join:headset-b@30,leave:visualizer@150"`), retire a peer
  (`qar_stub_peer_retire`), pause a peer's stream (`qar_stub_set_peer_paused`), make a listed
  peer stop answering sender creations (`qar_stub_set_peer_unanswering`), relist a peer under a
  new version id as a restarted app (`qar_stub_set_peer_version_id`), delay the begin-frame
  wait of `try_begin_frame` (`qar_stub_set_try_begin_frame_delay_ms`), delay callbacks and inject
  failures. `QAROS_STUB_REPORT=<file>` writes the call log at `qar_library_destroy`.

### Integration tests against `qar-streaming-c-ext`

The integration tests drive the C API against `qar-streaming-c-ext`, a test-only DLL with the
counterparts an application talks to (an in-process hub that mints invites, peers, render and
video receivers that collect frames, a Vulkan test device), as the C# suites do. No running QAROS,
visualizer or window is needed, except for `real_local_app` (below); runtime state goes to a
scratch directory under `%TEMP%\qaros-e2e`. `qar-streaming-c-ext` is built only in Debug and RelWithDebInfo SDK builds
and never ships in a release package.

`tests/qar-streaming-rust-ext` holds the Rust layer of that DLL, mirrored by hand like the C#
tests' `Ext` layer, and the tests. It is test-only: not a default member of the workspace, never
published, and no other crate depends on it. Point the tests at the `bin` folder of a Debug or
RelWithDebInfo SDK build that includes `qar-streaming-c-ext.dll`; without the variable they print
`skipped` and pass.

```powershell
$env:QAROS_PACKAGE_BIN = "<SDK build>\bin"
cargo test -p qar-streaming-rust-ext --features e2e -- --nocapture
```

| Test | What it checks |
| --- | --- |
| `real_load` | Load, `qar_library_init`, a runtime, an extension hub, teardown, twice in one process |
| `real_teardown` | The process exits cleanly after teardown |
| `real_cpu_render` | Onboarding with a minted invite, the target app found in the peer list, the CPU render sender, every view of every frame matching a known picture at the receiver |
| `real_vulkan_render` | The Vulkan render sender on the extension's device |
| `real_vulkan_own_device` | The Vulkan render sender on the application's own `ash` device, chosen by `Session::graphics_adapter` |
| `real_multi_user` | Two targets, two senders on one session; one user leaves |
| `real_local_app` | `vulkan_cube` (`QAROS_E2E_APP`) started with a piped stdin, as QAROS starts it: it joins the Hub running on this machine as a local app, renders for a target when `QAROS_E2E_REQUIRE_TARGET=1`, and exits 0 when its stdin closes. It needs that Hub and the Allow click on the cube's first run. The cube inherits `QAR_INSTANCE`, which picks the Hub and the runtime (a development hub's instance name; unset, the installed QAROS) |

`QAROS_E2E_EYE=<width>x<height>` and `QAROS_E2E_FRAMES` size the runs. Each test prints `RATE`
and `PHASES` lines.

Not covered by these tests: pairing-code onboarding and rejoin against a Hub, frames on screen or
in a headset, GUI panels on screen, the `link` feature, and eye poses (the extension's receivers
report identity poses).

## Tutorials

`examples/tutorials/` holds the sources of the developer guide's Rust tutorials, one Markdown file
per example plus the overview (`index.md`). A source pulls its code from the example with
`{{#include <path>:<anchor>}}` on a line of its own inside a fenced code block; the region is the
lines between `// ANCHOR: <anchor>` and `// ANCHOR_END: <anchor>` in the example, which may not
nest or overlap. Edit the sources and the examples, never the generated pages.

```sh
cargo xtask tutorials          # writes the pages to docs/website/docs/developer-guide/tutorials/rust/
cargo xtask tutorials --check  # fails on a stale page, an unresolved include or a bad anchor
```

## Code tabs of the developer guide

Every code sample of the developer guide with a C tab also has a Rust tab. `examples/guide/` holds
their sources, one Markdown file per guide page: a `## ` heading per C tab of the page, in page
order, then the tab's content. The code comes from anchored regions of `examples/tests/guide/`,
included as in the tutorials. That test target is compiled with the tests and never runs, so
every snippet builds against the current API. A sample the safe API cannot show calls the raw
function through `library.raw_api()` with an `as_raw()` handle; the snippets of functions only
the public header declares (the peer role, category and device identity) are in
`peers_identity.rs`, which `main.rs` compiles where the bindings declare them.

```sh
cargo xtask guide          # expands every tab; in qaros it also writes them into the guide's pages
cargo xtask guide --check  # fails on an unresolved include, and in qaros on a stale page
```

In qaros, CI runs the check on every developer guide change: a new code group with a C tab fails
until its Rust tab is added here and `cargo xtask guide` is run, and the message says how.

## Regenerating the bindings

```sh
cargo xtask bindgen
```

This regenerates `crates/qaros-sys/src/generated/<target>/` for the committed targets. It needs
libclang; set `LIBCLANG_PATH` if bindgen does not find it. `cargo xtask bindgen <target>` plus a
`cfg` entry in `crates/qaros-sys/src/lib.rs` adds a target. The macOS bindings are generated with
`QAR_ENABLE_D3D11`, which the header sets on Windows only, so that the stub can emulate the D3D11
sender there; the Linux bindings keep the header's choice. Shim headers stand in for the Windows
SDK (`xtask/shim/`) and the D3D11 headers (`xtask/shim-d3d11/`, also used by the macOS
conformance check), and `xtask/shim-vulkan/` declares the Vulkan types the
header uses. A change to `qar-streaming-c/include/qar_streaming.h` and the regenerated
bindings land in one commit; run the conformance tests after it.

## Repository layout

| Path | What it is |
| --- | --- |
| `crates/qaros-sys` | Raw bindings to every exported `qar_impl_*` function, generated per target and committed. The runtime lookup, runtime loading (default) or import-library linking (feature `link`). Ports of the `qar_*_default()` initializers. |
| `crates/qaros` | The safe API: library and runtime, onboarding and rejoin, peers, CPU, Vulkan and D3D11 render senders, app volumes, GUI panels. |
| `crates/qaros-stub` | The in-memory stand-in library for tests. |
| `crates/qaros-conformance` | Test-only: checks the ported initializers against the C header. |
| `tests/qar-streaming-rust-ext` | Test-only: the Rust layer of `qar-streaming-c-ext` and the integration tests. Not a default member. |
| `examples` | Ports of the C examples, the tutorial sources in `examples/tutorials/`, and the developer guide's Rust tabs in `examples/guide/` with their snippets in `examples/tests/guide/`. |
| `xtask` | `cargo xtask bindgen`, `cargo xtask tutorials` and `cargo xtask guide`. |
| `docs/` | [The Vulkan path](docs/vulkan-api.md), [the D3D11 path](docs/d3d11-api.md). |

## License

MIT, under the repository's root [LICENSE](../LICENSE).
