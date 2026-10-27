---
sidebar_position: 6
title: Vulkan Rendering
description: Join QAROS as a local app, create your own Vulkan device on the session's GPU, and hand QAROS a stereo color-and-depth frame per target through the Vulkan render sender.
---

## What you build

The Rust port of QAROS's Vulkan cube: a rotating cube with one flat color per face, rendered on the GPU in stereo color and depth for every target app in the session. It is the reference for a GPU source app in Rust. It runs on the Hub machine and joins the Hub as a local app, creates its own Vulkan device on the GPU the session uses, and hands each frame to QAROS through the Vulkan render sender without a copy.

Take this path when your renderer already runs on Vulkan. The API uses the types of the `ash` crate, version 0.38. If your renderer uses `ash` 0.38 too (as `wgpu-hal` 30, the Vulkan backend of wgpu 30, does), pass its `vk::PhysicalDevice` and `vk::Device` to the sender and use the frame's `vk::Image`s directly, with no conversion. Otherwise convert your raw Vulkan handles with `ash`'s `Handle::from_raw` and `as_raw`.

What you will learn:

- how to join as a local app, and how to cancel the approval wait on a stop request;
- how to create your Vulkan device on the session's GPU with the extensions QAROS needs;
- the safety contract of `Session::create_vulkan_render_sender`;
- the frame hand-off: acquire each image from the state QAROS reports, render, release it to the external queue family, signal the frame's semaphore, write the state back, show;
- why the device must outlive the session, not only the senders.

The program is `examples/src/bin/vulkan_cube.rs` in the Rust binding, behind the `vulkan` feature. This page includes its code; the file is the authority. The [CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering) tutorial explains peers, app volumes and the frame cycle; this one builds on it. The Vulkan API of the binding is summarized in [`docs/vulkan-api.md`](https://github.com/Quaternar/qaros/blob/main/qar-streaming-rust/docs/vulkan-api.md).

## Before you start

- **Platform.** Windows 10 or 11 for the real runtime. Joining as a local app is available on Windows; elsewhere the runtime refuses it. Without QAROS, the example and its test run against `qaros-stub`, which emulates the Vulkan path on your device.
- **GPU.** A GPU whose Vulkan driver offers `VK_KHR_external_memory_win32` and `VK_KHR_external_semaphore_win32` on Windows. QAROS supports only NVIDIA GPUs ([Choosing the GPU](/docs/developer-guide/onboarding-and-sessions#choosing-the-gpu)). The Vulkan loader comes with the GPU driver.
- **QAROS.** Installed on the Hub machine, where the example runs. When the Hub is not running, joining as a local app starts it, and the Hub keeps running after the app exits. The example takes no path: it finds the runtime with the default lookup, so the installed QAROS, unless a runtime sits beside the executable or `QAR_INSTANCE` names a QAROS instance, such as a development hub, whose runtime it then loads.
- **What you do.** On the first run, click **Allow** in the QAROS approval prompt, in the Visualizer or in a tray window, within 120 s ([The approval prompt](/docs/developer-guide/onboarding-and-sessions#the-approval-prompt)). An unsigned build asks again after every rebuild.
- **A target.** Open the Visualizer, or start a headset.
- **The latest stable Rust.** No shader compiler: the shaders are QAROS's GLSL, checked in as SPIR-V.

To use the Vulkan path in your own crate, add `qaros` with the `vulkan` feature to the `[dependencies]` section of your crate's `Cargo.toml`. The crates are not on crates.io, so the dependency names the QAROS repository, and Cargo finds the crate inside it:

```toml
[dependencies]
qaros = { git = "https://github.com/Quaternar/qaros", features = ["vulkan"] }
```

With a clone of the repository, point a path at its `qar-streaming-rust/crates/qaros` folder instead, relative to your `Cargo.toml`:

```toml
[dependencies]
qaros = { path = "../qaros/qar-streaming-rust/crates/qaros", features = ["vulkan"] }
```

For your own `ash` types to be the binding's, depend on `ash` 0.38 as well.

## Run it

From the `qar-streaming-rust` folder of the repository:

```powershell
cargo run --release -p qaros-examples --features vulkan --bin vulkan_cube
```

The example takes no arguments; `--help` prints its usage, and any other argument prints it to stderr and exits with 1. Stop it with Ctrl+C. When its stdin is a pipe, the end of stdin stops it too: that is how QAROS stops an app it started.

With QAROS installed and the Visualizer open, a run that is later stopped prints this (the runtime may add log lines of its own; the GPU, the target and the peer id are those of your session):

```text
Loaded <QAROS folder>\bin\qar-streaming-c.dll (the installed QAROS)
Joining the QAROS on this PC. On the first run, click Allow in the QAROS approval prompt.
Connecting to the local QAROS hub.
Certificate signed.
Onboarding complete.
Rendering on <GPU name> (LUID <adapter LUID>), the session's GPU
Connected. Waiting for rendering targets. Ctrl+C to quit.
Rendering for target <target name> (peer <peer id>)
Stopping.
```

It exits with 0. The lines between `Joining` and `Onboarding complete.` are the runtime's progress messages. While the approval prompt is open, one more appears, `Waiting for approval in QAROS hub`. In the Visualizer, the cube appears in the "Vulkan Cube" app volume.

Without QAROS, run the example's own test. It renders one frame through the stub on the first supported GPU and reads both eyes back:

```sh
cargo build -p qaros-stub
QAROS_REQUIRE_VULKAN=1 cargo test -p qaros-examples --features vulkan --bin vulkan_cube
```

It needs a Vulkan loader and driver. Without one it prints `skipped` and passes, unless `QAROS_REQUIRE_VULKAN` is set. On macOS the loader is not on the library path the test searches, and a `DYLD_LIBRARY_PATH` set for `cargo test` does not reach the test. Build the test binary, then run it yourself with `DYLD_LIBRARY_PATH` naming the folder of `libvulkan.dylib`, `/opt/homebrew/lib` for the Homebrew Vulkan loader:

```sh
cargo test -p qaros-examples --features vulkan --bin vulkan_cube --no-run
# The last line names the binary: Executable unittests src/bin/vulkan_cube.rs (target/debug/deps/vulkan_cube-<hash>)
QAROS_REQUIRE_VULKAN=1 DYLD_LIBRARY_PATH=/opt/homebrew/lib target/debug/deps/vulkan_cube-<hash>
```

The example itself also runs against the stub. It takes no library path, so put a copy of the stub under the runtime's file name in a folder and let the dynamic linker's search find it there, the last step of the default lookup outside Windows: name the folder with `DYLD_LIBRARY_PATH` (`LD_LIBRARY_PATH` on Linux). `target/stub-runtime` keeps the copy inside the build folder, which git ignores. On macOS, with the Homebrew Vulkan loader and MoltenVK:

```sh
cargo build -p qaros-stub
cargo build -p qaros-examples --features vulkan --bin vulkan_cube
mkdir -p target/stub-runtime
cp target/debug/libqaros_stub.dylib target/stub-runtime/libqar-streaming-c.dylib
DYLD_LIBRARY_PATH="$PWD/target/stub-runtime:/opt/homebrew/lib" target/debug/vulkan_cube
```

Stopped by closing a piped stdin, it prints (with your own path and GPU):

```text
Loaded .../target/stub-runtime/libqar-streaming-c.dylib (the dynamic linker's search)
Joining the QAROS on this PC. On the first run, click Allow in the QAROS approval prompt.
stub onboarding complete
Rendering on <GPU name> (LUID <adapter LUID>), the first supported GPU
Connected. Waiting for rendering targets. Ctrl+C to quit.
Rendering for target Stub Visualizer (peer 56:56:56:56:56:56:56:56:56:56:56:56:56:56:56:56)
Stopping.
```

The stub accepts the local-app join without a prompt and names no GPU, so the example takes the first supported one.

## Walkthrough

### 1. Start, load the runtime, map failures to exit codes

```rust
{{#include ../src/bin/vulkan_cube.rs:main}}
```

`stop::watch_for_stop_requests` installs a Ctrl+C handler (SIGINT and SIGTERM outside Windows) and, when stdin is a pipe, a thread that waits for its end. A signal handler may safely do nothing but write an atomic, so all QAROS and GPU cleanup stays on the main thread, which polls `stop::requested()`.

`Library::load_default` is the C `qar_library_load(NULL)`: the runtime beside the executable, the runtime of the QAROS instance `QAR_INSTANCE` names (Windows), the installed QAROS, then `PATH`. It is `unsafe` because a library file that does not match these bindings is undefined behavior. The exit codes: 0 after a stop, 2 when no runtime loads, 3 when the library fails to initialize, 4 for any other failure.

### 2. The order of the whole program

```rust
{{#include ../src/bin/vulkan_cube.rs:run_app}}
```

Join first: the session decides the GPU, and the device is created there. Read the drop order at the end now; [Ownership and teardown](#ownership-and-teardown) explains it.

### 3. Join as a local app

A source app on the Hub machine joins as a local app: the user approves it once in QAROS ([Local app](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code)). Nothing is stored; every start onboards again.

```rust
{{#include ../src/bin/vulkan_cube.rs:join}}
```

The call can wait minutes for the approval, so it takes a cancel token. `CancelToken::cancel_handle` gives a `Send` handle that another thread can cancel with while the token lives; here a thread cancels when a stop is requested. Progress messages, such as `Waiting for approval in QAROS hub`, arrive on a library thread and go through a channel to a printing thread. `approval_timeout: None` keeps the Hub's default of 120 s.

When joining fails, the example tells the user what happened and what to do, by status code:

```rust
{{#include ../src/bin/vulkan_cube.rs:explain_failure}}
```

These are the codes of [Status codes and what to tell the user](/docs/developer-guide/onboarding-and-sessions#status-codes-and-what-to-tell-the-user). Retry none of them silently.

### 4. Create the device on the session's GPU

QAROS creates the frame images on your device, and the receiver reads them on the session's GPU. Render on that GPU, so frames need no copy between GPUs. `Session::graphics_adapter()` returns the adapter the session uses: the one chained at onboarding, else the one QAROS or the library selected. The example matches it against each physical device's `VkPhysicalDeviceIDProperties`:

```rust
{{#include ../src/bin/vulkan_cube.rs:pick_device}}
```

`GraphicsAdapter::matches` compares a LUID on Windows and a UUID elsewhere. `qaros::vulkan::required_device_extensions()` lists what QAROS needs on your device: Win32 external memory and semaphores on Windows, file-descriptor external memory on Linux. When the session names an adapter and no physical device matches it with those extensions, the example stops with an error rather than render on another GPU. When the session names none, it takes the first supported GPU.

The sender gets the device as two raw handles:

```rust
{{#include ../src/bin/vulkan_cube.rs:stream_params}}
```

### 5. One sender and one camera per target

The cube declares one texture per view: each eye gets a color and a depth image of its own, the `SeparatedTextures` layout.

```rust
{{#include ../src/bin/vulkan_cube.rs:frame_views}}
```

The app volume is the cube's own; QAROS registers none for an app:

```rust
{{#include ../src/bin/vulkan_cube.rs:create_volume}}
```

Each target gets a `Target`: a sender and a camera, the per-target GPU state.

```rust
{{#include ../src/bin/vulkan_cube.rs:target}}
```

`create_vulkan_render_sender` is `unsafe`. You promise:

- the physical device and device are valid, and the device was created from that physical device;
- the device is on the adapter the session was onboarded with, and enables `required_device_extensions()`;
- the device outlives the sender and every handle taken from it.

The wrapper cannot check any of it. The sender is the same `RenderSender` type as on the CPU path.

The serving loop follows the peer list as the CPU example does, with one addition: a target whose sender cannot be created is tried again after a delay that starts at 3 s and doubles up to 60 s (`RetryPlan`). Creating a sender blocks the loop until the target answers, so a target that never answers would otherwise stall every other target.

```rust
{{#include ../src/bin/vulkan_cube.rs:serve_targets}}
```

The C cube serves each target on a thread of its own. `qaros` handles are not `Send`, so this port serves all targets from one loop with the non-blocking `try_begin_owned_frame`, as the CPU example does.

### 6. One frame

```rust
{{#include ../src/bin/vulkan_cube.rs:render_next_frame}}
```

Every begun frame must signal its semaphore before it is dropped, even one that is not rendered: QAROS waits for that signal. So when rendering fails, `Camera::discard` submits an empty batch that only signals it.

The pose and field of view come from the view showing each eye's color. `view_index` finds the view by eye and kind; the same index selects the pose with `frame.view(index)`:

```rust
{{#include ../src/bin/vulkan_cube.rs:eye_views}}
```

The poses are in the app volume's app content space, the space the cube renders in. The projection uses the OpenXR field-of-view angles and forward Vulkan depth, 0 at the near plane and 1 at the far plane:

```rust
{{#include ../src/bin/vulkan_cube.rs:view_projection}}
```

The hand-off itself, in the order the safety rules require: submit the GPU work that signals the frame's semaphore, write the images' new state back, then show.

```rust
{{#include ../src/bin/vulkan_cube.rs:render_frame}}
```

`vulkan_frame()` borrows the open frame mutably, so the frame record cannot change while you read it, and the write-back cannot happen after `show`. The submission is outside the wrapper's view: submitting after `show` is a bug it cannot catch. `set_fence(vk::Fence::null())` is what the C cube writes.

### 7. Acquire and release the images

QAROS owns the images, their memory and the semaphore; never destroy, free or re-bind them. Each frame reports every image's current layout and owning queue family. Transition from exactly that state: the real runtime reports `TRANSFER_SRC_OPTIMAL` owned by `VK_QUEUE_FAMILY_EXTERNAL` from the first frame on, the stub `UNDEFINED` and `VK_QUEUE_FAMILY_IGNORED` at first. After rendering, leave every image in `SHOWN_LAYOUT` (`TRANSFER_SRC_OPTIMAL`), released to `VK_QUEUE_FAMILY_EXTERNAL`.

```rust
{{#include ../src/bin/vulkan_cube.rs:camera_render}}
```

```rust
{{#include ../src/bin/vulkan_cube.rs:acquire_release}}
```

Image handles can change between frames, after a layout change for example. Read the image records every frame. The camera caches its image views and framebuffers by image handle, so a new handle gets new ones.

The camera keeps two frames in flight. It waits on a slot's fence only when it reuses the slot, never for the frame just submitted: the frame's semaphore hands that work to QAROS, which waits for it on the GPU.

### 8. Keep the sender, or end it

```rust
{{#include ../src/lib.rs:is_waiting}}
```

This is the CPU tutorial's helper, shared in `examples/src/lib.rs`: paused, reconnecting, transfer-less streams and an unresolved app volume keep the sender; anything else, `RENDERING_STREAM_IS_CLOSED` included, drops the target, and the next listing creates a new one when the target still runs ([Troubleshooting](/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow)). The C cube's `IsWaiting` keeps the sender for the unresolved app volume too, as the header calls that status recoverable.

## Ownership and teardown

The `qaros` handles keep each other alive as on the CPU path: a sender keeps its session, a session its runtime, a runtime the library. The Vulkan objects are different: the wrapper does not know your device, so their order is yours.

- **Camera before sender.** `Target` declares the camera before the sender, and Rust drops struct fields in declaration order. The camera's `Drop` waits for its submissions, then destroys its views and framebuffers over the sender's images, while those images still exist.
- **Targets before the pipeline and the device.** `serve_targets` drops every target before it returns.
- **Session before device.** `run_app` drops the pipeline, then the session, then the device. QAROS keeps using your device until the session is gone, not only until the senders are: a device destroyed before the session can crash the runtime's teardown (exit `0xC0000005`). The C cube keeps its device until after the session too.
- **Device last.** `CubeDevice`'s `Drop` waits for the device to go idle, then destroys the device and the instance:

```rust
{{#include ../src/bin/vulkan_cube.rs:device_drop}}
```

- **Library.** `main` drops the `Library` after `run_app`. It destroys the library but never unloads the DLL.

What the wrapper guarantees on this path: the frame record is borrowed, so it cannot be used after another frame call or the sender's destruction; writes to it happen only inside a frame cycle; one frame is open per sender. What it cannot see: your device's lifetime, your submissions, the semaphore signal, and the barriers.

## Troubleshooting

| The example says | Cause | What to do |
| --- | --- | --- |
| `Could not load QAROS. Install QAROS on this PC, then start this app again.` (exit 2) | the next line says which: no runtime found by the default lookup, one that does not open, or one that lacks functions these bindings need | install QAROS, or set `QAR_INSTANCE` to a QAROS instance whose runtime matches the binding's header; a set `QAR_INSTANCE` that names no instance, or one without a loadable runtime, is reported on the next line |
| `The QAROS runtime failed to initialize: ...` (exit 3) | the runtime loaded, but `qar_library_init` failed | read the message; check the QAROS installation |
| `QAROS is not running on this PC. ...` | `ONBOARDING_HUB_UNREACHABLE`: no Hub answered, and none could be started | start QAROS from the Start menu, then the example |
| `QAROS did not allow this app to join ...` | `ONBOARDING_APPROVAL_DENIED`: Deny, a closed prompt, or a revoked approval | start it again and click **Allow** |
| `Nobody answered the approval prompt in QAROS in time. ...` | `ONBOARDING_APPROVAL_TIMEOUT` | start it again and answer within 120 s |
| `The program answering on this PC is not the installed QAROS ...` | `ONBOARDING_HUB_NOT_AUTHENTIC` | reinstall QAROS; do not retry |
| `This app no longer matches what QAROS approved ...` | `ONBOARDING_APP_IDENTITY_MISMATCH`: the executable moved, was copied or re-signed | revoke it in the Visualizer, then start it again |
| `the session's GPU (...) names no graphics GPU with the external memory and semaphore support QAROS needs`, or the same without `the session's GPU` | no Vulkan device on the session's adapter (or on this machine) has a graphics queue and the required extensions | update the GPU driver; check that the GPU is supported |
| `No sender for <target> yet (...); trying again in N s` | the target did not answer the sender creation within 10 s | wait; the delay doubles up to 60 s |
| `Stopped streaming to <target>: ...` | a failure outside the normal flow, a closed stream included | nothing; the next listing serves the target again if it still runs |
| `the sender reports ... where the cube renders ...` | QAROS reported a pixel format the cube's pipeline does not render | the header reports no `VkFormat`; `assumed_vk_format` maps QAROS formats to UNORM and SFLOAT ones |
| your own app crashes at exit (`0xC0000005`) | the Vulkan device was destroyed before the session | drop the session first, the device last |

## From C

The C tutorial is [Vulkan Rendering](/docs/developer-guide/tutorials/c/vulkan-rendering); the C original is the [`vulkan_cube` source](https://github.com/Quaternar/qaros/tree/main/qar-streaming-c/examples/vulkan_cube), and [Getting Started](/docs/developer-guide/getting-started) builds and runs it. What corresponds:

| C cube (`main.cpp`) | Rust |
| --- | --- |
| `qar_library_load(NULL)` | `Library::load_default` |
| `JoinAsLocalApp`, `CancelOnStop` | `join_as_local_app`, `CancelToken::cancel_handle` |
| `ExplainOnboardingFailure` | `explain_onboarding_failure` |
| exit codes 0, 1 (a bad argument, a failed `qar_library_init`, any other failure), 2, and 3 to 7 for a refused join | exit codes 0, 1 (a bad argument), 2, 3 (a failed `qar_library_init`), 4 for any other failure |
| `SessionAdapter` (`qar_session_get_graphics_device_id`) | `Session::graphics_adapter` |
| `QarStreamParamsVulkan` chained into `QarRenderSenderInit` | `VulkanStreamParams`, `unsafe Session::create_vulkan_render_sender` |
| `qar_peer_spec_get_role` is `QAR_PEER_ROLE_TARGET` | `PeerInfo::is_target_app` (`qar_peer_spec_is_target_app`, defined as the same test) |
| `IsReachable`: `qar_peer_spec_get_presence` is `ONLINE` or `UNKNOWN` | `PeerInfo::app_state` is `AppState::Running` (`is_running_target`): the deprecated app state, computed from the presence, so the same targets for every current peer ([CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering#4-follow-the-target-apps-in-the-peer-list)) |
| one thread per target, blocking `qar_render_sender_begin_frame` | one loop, `RenderSender::try_begin_owned_frame` |
| `qar_render_sender_frame_vulkan` | `OwnedFrame::vulkan_frame` |
| `FindFrameView` | `VulkanFrameState::view_index` |
| writing `layout`, `queue_family_index` and `fence` into the record | `VulkanFrame::release_to_external`, `set_image_state`, `set_fence` |
| `IsWaiting`, the unresolved app volume included | `is_waiting` |
| frame info kept until `show_frame` (a `std::unique_ptr` with a deleter) | kept until the frame is shown or dropped |
| retry a failed target after 3 s | after 3 s, doubling up to 60 s |
| `qar_session_handle_destroy`, then `qar_runtime_handle_destroy`, the device after the session | drop the session, then the device; the runtime goes when `run_app` returns, the library when `main` drops it |

## Next

- [`docs/vulkan-api.md`](https://github.com/Quaternar/qaros/blob/main/qar-streaming-rust/docs/vulkan-api.md): every type of the Vulkan path, its safety rules, and how the stub emulates it.
- [Rendering Streams](/docs/developer-guide/rendering-streams): layouts, hand tracking, the lifecycle of a stream.
- [Launched by QAROS](/docs/developer-guide/onboarding-and-sessions#launched-by-qaros-qaros-api-apps): the contract this example follows, so an operator can add it to the Visualizer as a QAROS API app.
