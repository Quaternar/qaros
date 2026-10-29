---
sidebar_position: 5
title: D3D11 Rendering
description: Chain your own D3D11.1 device, created on the session's adapter, into a render sender through the safe D3D11 API, and clear each color view of its shared textures on the GPU.
---

## What you build

A Windows source application that renders straight into the render sender's shared D3D11 textures, with no CPU copy. It finds the first running target app, creates a D3D11.1 device on the session's adapter, chains that device into a render sender, and clears each eye's color view to a pulsing color: the left eye red, the right eye blue.

What you will learn:

- how to create the device on the session's adapter;
- how to hand the device to a sender with `qaros::d3d11::D3D11StreamParams` and what its fields decide;
- the D3D11 frame cycle: the frame record, the shared textures it lends, and the status codes that keep it going;
- what the sender keeps alive for you, and the one rule the wrapper cannot check.

The program is `examples/src/bin/d3d11_render_sender.rs` in the Rust binding, a port of the C example `d3d11_render_sender.c` on the safe API (feature `d3d11` of `qaros`). This page includes its code; the file is the authority. Read the [CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering) tutorial first: this one assumes its concepts.

## Before you start

- **Platform.** Windows with a D3D11.1-capable GPU. On any other system the example prints that it requires Windows and exits with 0.
- **QAROS.** Installed on the Hub machine, where the example runs, or a runtime folder named by `QAROS_RUNTIME_DIR`, or a library file named with `--qaros-library=PATH`.
- **A target.** At least one target app in the session (the Visualizer, a headset). Joining as a local app starts the Hub when it is not running.
- **Joining.** The example joins the Hub as a local app, as the CPU example does: click **Allow** in the QAROS approval prompt on the first run of the executable; nothing is stored, and later runs join without a prompt ([Local app](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code)).[^code]
- **The latest stable Rust.** The D3D11 types come from the `windows` crate 0.62, a Windows-only dependency of `qaros` (feature `d3d11`) and of the examples.

In your own application, add `qaros` with the `d3d11` feature to the `[dependencies]` section of your crate's `Cargo.toml`. The crates are not on crates.io, so the dependency names the QAROS repository, and Cargo finds the crate inside it:

```toml
[dependencies]
qaros = { git = "https://github.com/Quaternar/qaros", features = ["d3d11"] }
```

With a clone of the repository, point a path at its `qar-streaming-rust/crates/qaros` folder instead, relative to your `Cargo.toml`:

```toml
[dependencies]
qaros = { path = "../qaros/qar-streaming-rust/crates/qaros", features = ["d3d11"] }
```

The device and context you pass are `windows` crate 0.62 types, so your own D3D11 code depends on `windows` 0.62 as well.

## Run it

From the `qar-streaming-rust` folder of the repository:

```powershell
# Join the Hub as a local app: click Allow in QAROS on the first run.
cargo run --release -p qaros-examples --bin d3d11_render_sender -- --frames=300
```

The arguments are those of the CPU example: `--frames=N` (300 by default) and `--qaros-library=PATH`.

The program prints the runtime it loaded, `qar_runtime_create: SUCCESS`, the local-app join, `Waiting for a target app in the peer list ...`, `qar_render_sender_create: SUCCESS`, `Rendering for target <target name> (peer <peer id>)`, and `Submitted <N> frames.` at the end. On the target, the "D3D11 Tutorial Volume" app volume shows the two pulsing eyes.

Against the stub on Windows (`QAROS_DLL` naming `target\debug\qaros_stub.dll`, and any code as the first argument, as in [CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering#run-it)), the example follows the same steps but prints `qaros-stub lends no D3D11 textures: submitting the frames without drawing.` and draws nothing: the stub's D3D11 emulation lends placeholder addresses, not textures, and Direct3D would crash on them. The stub run stores `d3d11_render_sender.onboarding-id.txt` in its folder, which a later run against QAROS from that folder would try to rejoin with: keep stub runs in a folder of their own.

## Walkthrough

### 1. Windows only

```rust
{{#include ../src/bin/d3d11_render_sender.rs:main}}
```

The D3D11 code lives in a `#[cfg(windows)]` module. Elsewhere `main` prints one line. The `d3d11` feature of `qaros` follows the header the same way: it compiles where the bindings carry the D3D11 types (Windows, and on macOS only for the tests against `qaros-stub`) and to nothing on Linux.

### 2. The program, top to bottom

```rust
{{#include ../src/bin/d3d11_render_sender.rs:run}}
```

The session comes from the same helper as the CPU example (`join_session` in `examples/src/lib.rs`: rejoin, else the pairing code, else a local-app join), and the app volume from `session.app_volumes().get_or_create` with the common name `d3d11-volume.examples.qaros`. The `draws` check matters only against `qaros-stub` (runtime flavor `stub`); against QAROS every frame is drawn, and your own application needs no such check.

### 3. Find the target

Like the CPU example, this one finds its target in the peer list, not through a render request ([Rendering Streams](/docs/developer-guide/rendering-streams#finding-rendering-targets)). It serves one target, the first running target app, and polls the list every 50 ms until there is one:

```rust
{{#include ../src/bin/d3d11_render_sender.rs:targets}}
```

`is_running_target` is the CPU example's: a peer for which `qar_peer_spec_is_target_app` is true and whose app state is `AppState::Running`. The header deprecates the app state for `qar_peer_spec_get_presence`, but computes it from the presence: for every current peer, running means online, or unknown on a Hub that has not reconciled the peer, the targets the guide renders for ([Working with Peers](/docs/developer-guide/peers)). The safe API reads the app state; the presence getter is raw only, through `library.raw_api()` with the session's `as_raw()`.

### 4. Create the device on the session's adapter

The sender's shared textures live on the session's GPU. Create your device on the same adapter, so frames need no copy between GPUs ([Choosing the GPU](/docs/developer-guide/onboarding-and-sessions#choosing-the-gpu)). `Session::graphics_adapter` returns the session's adapter; the example finds the DXGI adapter with that LUID and creates a D3D11.1 device on it. When the session names no adapter, it uses the default hardware adapter.

```rust
{{#include ../src/bin/d3d11_render_sender.rs:device}}
```

`D3D11CreateDevice` takes `D3D_DRIVER_TYPE_UNKNOWN` when you pass an adapter, and `D3D_DRIVER_TYPE_HARDWARE` when you do not. The sender needs the `ID3D11Device1` and `ID3D11DeviceContext1` interfaces, so the device is queried for them.

### 5. Hand the device to the sender

```rust
{{#include ../src/bin/d3d11_render_sender.rs:create_sender}}
```

- `D3D11StreamParams::new` starts from the header's defaults (`qar_stream_params_d3d11_default`) and takes a reference on the device and the context. The sender keeps those references until it is destroyed, so the device cannot go away under it, whatever order your own values drop in.
- `color_bind_flags` and `depth_bind_flags` decide how you can use the shared textures. Render target and shader resource for color let you draw into it; depth stencil for depth lets you use it as your depth buffer. Leave shader resource off the depth flags: QAROS creates the depth textures as `DXGI_FORMAT_D32_FLOAT`, which D3D11 cannot bind as a shader resource, and sender creation then fails with `DxgiError` "The parameter is incorrect." They are `D3D11_BIND_FLAG` bits as `u32`, as in the header.
- `acquire_keyed_mutex_sync = true` lets the library acquire and release each texture's keyed mutex around the frame cycle. Set it to false only when you run the keyed-mutex chain yourself; the frame then lists each texture's keyed mutex with its acquire and release keys (`D3D11Frame::keyed_mutexes`, and [The D3D11 path](/docs/developer-guide/rendering-streams#the-d3d11-path)).
- `create_d3d11_render_sender` chains the parameters as `QarStreamParamsD3D11` and selects `QAR_GRAPHICS_API_D3D11`. It returns the same `RenderSender` type as the CPU path; `start_d3d11_render_sender` is the form that does not block.
- The frame views are declared as in the CPU tutorial: both eyes' color side by side in texture 0, depth in texture 1.

The creation is `unsafe` for one reason the wrapper cannot check: QAROS uses the immediate context you give it, the header does not say when or on which thread, and a D3D11 immediate context is not thread-safe. Use it only on the thread that drives the session and its senders, as this example does, or turn on the device's multithread protection.

### 6. Begin, clear, show

```rust
{{#include ../src/bin/d3d11_render_sender.rs:render_frame}}
```

`begin_frame` blocks until the target wants a frame. Then `d3d11_frame` reads the frame record (`qar_render_sender_frame_d3d11`): the views and the shared textures they lie on. `show` closes the frame cycle that begin opened, so the example shows even when reading the record failed. The frame info is destroyed after `show`, as in the C example; `Frame` does that when it is consumed.

The example does not read the view poses: a clear needs none. A real renderer reads the pose and field of view of each view with `frame.view(index)` (the index `D3D11Frame::view_index` finds for an eye and color or depth) and draws the scene into the view's rectangle, as the CPU and Vulkan examples do.

The clear itself:

```rust
{{#include ../src/bin/d3d11_render_sender.rs:clear_color_views}}
```

The textures belong to the sender. `D3D11Texture::texture` lends each one for the borrow of the frame, with no `AddRef` or `Release`, so it cannot be used after `show`; clone the interface if you need to keep one. The example creates a render-target view on the texture and clears the view's rectangle with `ID3D11DeviceContext1::ClearView`. Depth views are left as the sender hands them out, as in the C example. A real renderer writes depth too: QAROS uses it for occlusion and reprojection.

Status handling, by `is_waiting`:

| Status (`qaros::StatusCode`) | C name | What the example does |
| --- | --- | --- |
| `MEDIA_STREAMING_STREAM_IS_PAUSED`, `_IS_RECONNECTING`, `_NO_ACTIVE_TRANSFERS` | `QAR_STATUS_MEDIA_STREAMING_...` | skips the frame and begins the next one; begin-frame holds a reconnecting or transfer-less stream about 16 ms, so the loop does not spin |
| `RENDERING_APP_VOLUME_UNRESOLVED` | `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED` | the same; begin-frame holds about 16 ms |
| `RENDERING_STREAM_IS_CLOSED`, or any other error | `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED`, ... | prints the error and ends the loop |

With one target there is nothing to recreate, so a closed stream ends the program. It still exits with 0. To serve a target again after `STREAM_IS_CLOSED`, drop the sender and create a new one while the target is still in the peer list ([Rendering Streams](/docs/developer-guide/rendering-streams#lifecycle-and-failure)).

## Ownership and teardown

| Value | Owns | Keeps alive |
| --- | --- | --- |
| `Library` | `qar_library_init` for the process | |
| `Runtime` | the `QarRuntime` | the library |
| `Session` | the `QarSession` | the runtime |
| `RenderSender` | the `QarRenderSender` | the session, and a reference on the D3D11 device and context |
| `Frame` | the frame info, until shown or dropped | borrows the sender |
| `D3D11Frame` | a copy of the frame record | borrows the frame; lends the textures |

`run` drops in the C example's order, explicitly: the sender, which releases its device references after the sender is destroyed; the example's own context and device; the session; the runtime (released with `qar_runtime_handle_destroy`, as the C samples release it); the library. The ownership above would keep the order whatever the drop order: a sender keeps its session, a session its runtime, and the device lives as long as the sender holds its reference. When the sender holds the last reference to its session, the session goes before the device references.

On the Vulkan path your device must outlive the session ([Vulkan Rendering](/docs/developer-guide/tutorials/rust/vulkan-rendering#ownership-and-teardown)). A D3D11 device is reference counted, and the sender's references keep it alive until the sender is gone.

## Troubleshooting

| Symptom | Cause | What to do |
| --- | --- | --- |
| `d3d11_render_sender requires Windows (QAR_ENABLE_D3D11).` | built for another system | run it on Windows |
| exit 4 or 5 after `joining the QAROS Hub on this machine as a local app` | the approval prompt was denied, closed or not answered within 120 s | run again and click **Allow** |
| exit 3 | the local-app join, which starts a Hub that is not running, reached no Hub | start QAROS and run again |
| `Pairing code rejected or expired` (`QAR_STATUS_PAKE_ERROR`) | wrong or expired code | read the current code and run again at once |
| stays at `Waiting for a target app in the peer list ...` | no running target app in the session | open the Visualizer or start a headset |
| `the session's adapter (LUID ...) is not among this machine's DXGI adapters` | the session names an adapter DXGI does not list | check the GPU driver; report it with the LUID |
| `D3D11CreateDevice failed (...)` or `D3D11.1 interfaces not available` | the adapter does not support D3D11.1 | use a GPU that does |
| `qar_render_sender_create failed with ...` | the sender could not be created for this target | read the message; check the target is still running |
| `qar_render_sender_begin_frame failed with ...`, then `Submitted <N> frames.` | the stream closed for good (`QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED`, possibly wrapped in `QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME`) or another failure | read the message; start the example again |

## From C

The C tutorial is [D3D11 Rendering](/docs/developer-guide/tutorials/c/d3d11-rendering). The two programs do the same thing: join with a stored onboarding id, a pairing code or as a local app, take the first running target app from the peer list, create the device on the session's adapter, bind depth as depth stencil only, declare color and depth of both eyes, and skip the frame on paused, reconnecting, transfer-less and unresolved-volume statuses. The Rust example makes the same C calls in the same order through the safe API, except where the last rows say otherwise:

| C example | Rust example |
| --- | --- |
| `example_join_session` | `join_session` |
| `example_is_running_target`: role `QAR_PEER_ROLE_TARGET`, presence `ONLINE` or `UNKNOWN` | `is_running_target`: the app state `AppState::Running`, computed from the presence, so the same targets |
| `qar_session_get_graphics_device_id`, then the DXGI adapter with that LUID | `Session::graphics_adapter` |
| `QarStreamParamsD3D11` filled by hand | `D3D11StreamParams::new`, which also keeps the device and context alive for the sender |
| `qar_render_sender_frame_d3d11` into a `QarVideoFrameD3D11` | `Frame::d3d11_frame`, a copy of the record that lends the textures for the frame |
| `example_is_waiting` | `is_waiting` |
| `log_result` | `qaros::Error`, with the function, the status and the message |
| `qar_*_handle_destroy` on every exit path | handles released on drop, with explicit drops for the order |
| `qar_runtime_handle_destroy` | dropping the `Runtime` |
| D3D11 through COBJMACROS | D3D11 through the `windows` crate |
| `qar_render_sender_create` with a 10 s timeout token | `create_d3d11_render_sender` with `BlockingOptions::default()`, no timeout |
| `qar_render_sender_try_begin_frame`, polled with a 2 ms sleep | `begin_frame`, which blocks until the target wants a frame |
| exit code 1 when a frame failure ends the loop | exit code 0 |

## Next

- [Vulkan Rendering](/docs/developer-guide/tutorials/rust/vulkan-rendering): the other GPU path, with the same device-on-the-session's-adapter rule.
- [Rendering Streams](/docs/developer-guide/rendering-streams#the-d3d11-path): the D3D11 path in the guide.
- [API Conventions](/docs/developer-guide/api-conventions#6-ownership-and-lifetime): handle ownership and the teardown order on the C API.

[^code]: Like the CPU example, it also takes a pairing code as its first argument; it passes no Hub address, so the code finds only your own Hub, on the Hub machine (a Hub elsewhere needs its host and port, `HubAddress`): `cargo run --release -p qaros-examples --bin d3d11_render_sender -- <PAIRING-CODE> --frames=300`. It then stores the onboarding id in `d3d11_render_sender.onboarding-id.txt` in the working directory and rejoins with it later ([Onboarding and Rejoin](/docs/developer-guide/tutorials/rust/onboarding-and-rejoin)).
