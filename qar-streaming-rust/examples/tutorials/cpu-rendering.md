---
sidebar_position: 4
title: CPU Rendering
description: Find the target apps in the peer list, give each one a CPU render sender, and stream a stereo color-and-depth frame rendered from that target's own eye poses.
---

## What you build

A source application that floats a lit sphere, 25 cm in radius, in the middle of its own app volume. It finds every target app in the session (a visualizer, a headset), gives each one a render sender, and renders every frame on the CPU from that target's eye poses and fields of view, with color and depth for both eyes.

What you will learn:

- why a source app finds its targets in the peer list instead of waiting for render requests;
- how to declare the frame views of a stereo color-and-depth stream;
- the frame cycle: begin a frame, read the pose and field of view of each view, write pixels, show;
- which status codes mean "keep the sender and try again" and which mean "this sender is finished";
- what the Rust handles own and in which order they are released.

The program is `examples/src/bin/cpu_rendering_visualizer.rs` in the Rust binding. This page includes its code; the file is the authority.

## Before you start

- **Platform.** Windows for the real runtime. On macOS and Linux the example runs against `qaros-stub`, the in-memory stand-in for the runtime that ships with the binding's tests. The stub is enough to follow the frame cycle; it shows no picture.
- **QAROS.** Installed on the Hub machine, where the example runs, or a runtime folder named by `QAROS_RUNTIME_DIR`, or a library file named with `--qaros-library=PATH`. The example finds the runtime the way every binding does ([Language Bindings](/docs/developer-guide/language-bindings#rust)).
- **A target.** At least one target app in the session: open the Visualizer, or start a headset. Joining as a local app starts the Hub when it is not running.
- **Joining.** On Windows the example joins the Hub as a local app ([Local app](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code)): the first run of the executable opens the QAROS approval prompt, you click **Allow** once, and later runs of the same executable join without a prompt. A local app has no onboarding id, so nothing is stored.[^code]
- **The latest stable Rust.**

## Run it

From the `qar-streaming-rust` folder of the repository, on Windows:

```powershell
# Join the Hub as a local app: click Allow in QAROS on the first run.
cargo run --release -p qaros-examples --bin cpu_rendering_visualizer -- --frames=300
```

`--frames=N` (or `--frames N`) stops the example after N shown frames, counted over all targets; without it, the example stops after 300. An unsigned executable asks for **Allow** again after every rebuild that changes it.

Without QAROS, run it against the stub. Build the stub first; it is a test library, so `cargo run` does not build it:

```sh
cargo build -p qaros-stub
mkdir -p target/stub-run && cd target/stub-run
# Windows: qaros_stub.dll; Linux: libqaros_stub.so
QAROS_DLL="$PWD/../debug/libqaros_stub.dylib" cargo run -p qaros-examples --bin cpu_rendering_visualizer -- ANY-CODE --frames=3
```

Outside Windows the example does not try the local-app join, so the stub run passes a code: the stub accepts any code except `WRONG` (a rejected code) and `UNREACHABLE` (no Hub). The stub run stores `cpu_rendering_visualizer.onboarding-id.txt` in its folder, which a later run against QAROS from that folder would try to rejoin with: keep stub runs in a folder of their own ([On macOS, against the stub](/docs/developer-guide/tutorials/rust/#on-macos-against-the-stub)). A first run and then a second one, which rejoins, print (with your own path):

```text
Loading qar-streaming-c from: .../target/stub-run/../debug/libqaros_stub.dylib
Loaded .../target/debug/libqaros_stub.dylib
qar_runtime_create: SUCCESS
qar_runtime_onboard: SUCCESS
Looking for target apps in the peer list (open a visualizer or start a headset) ...
Rendering for Stub Visualizer (peer 56:56:56:56:56:56:56:56:56:56:56:56:56:56:56:56)
Submitted 3 frames.
```

```text
Loading qar-streaming-c from: .../target/stub-run/../debug/libqaros_stub.dylib
Loaded .../target/debug/libqaros_stub.dylib
qar_runtime_create: SUCCESS
qar_runtime_rejoin: SUCCESS
Looking for target apps in the peer list (open a visualizer or start a headset) ...
Rendering for Stub Visualizer (peer 56:56:56:56:56:56:56:56:56:56:56:56:56:56:56:56)
Submitted 3 frames.
```

Without `QAROS_DLL` (or `--qaros-library`) the example uses the default lookup: it prints `Loading the QAROS runtime (default lookup)`, and the `Loaded` line names the step that found the runtime, for example `(the installed QAROS)`. Against a real Hub you then see the sphere in the "CPU Tutorial Volume" app volume on each target.

## Walkthrough

### 1. Declare the frame views

A render sender streams one video frame per display frame. The frame views say what is in it and where: which eye, color or depth, and in which texture. The Rust example declares four views: both eyes' color side by side in texture 0, both eyes' depth side by side in texture 1.

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:frame_views}}
```

The order matters. Every frame lists its views in the order you declared them, and the view index you pass to `OwnedFrame::view` is that position: view 0 is the left eye, view 1 the right eye ([Rendering Streams](/docs/developer-guide/rendering-streams#creating-a-sender), "Frame views"). A sender created with no views at all hands out frames without poses: reading the pose of view 0 then fails with `QAR_STATUS_LOGIC_ERROR`.

Send depth whenever you can. QAROS uses it for occlusion between app volumes and to warp your frame to the viewer's latest head pose.

### 2. Load the runtime and get a session

Every session comes from onboarding: there is no "connect" call. The helpers in `examples/src/lib.rs` load the runtime, create a `Runtime`, then rejoin with the stored onboarding id, onboard with the pairing code, or join as a local app.

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:session}}
```

`join_session` tries the three in that order:

```rust
{{#include ../src/lib.rs:join_session}}
```

- **Rejoin.** With a stored id it calls `Runtime::rejoin`. When the Hub has no session for that id, the error carries `StatusCode::ONBOARDING_SESSION_NOT_FOUND`, which is normal flow: it goes on to the next way. Any other rejoin error is real and ends the run.
- **Pairing code.** With a code it calls `Runtime::onboard` with `OnboardMethod::Code` and stores the new id, as the C example does. A rejected or expired code fails with `StatusCode::PAKE_ERROR`; read the current code and try again.
- **Local app.** With neither, on Windows, it calls `Runtime::onboard` with `OnboardMethod::LocalApp`, as [Local App Onboarding](/docs/developer-guide/tutorials/rust/local-app-onboarding) explains, and waits up to 120 s for **Allow**. The returned onboarding id is the zero id: there is nothing to store, and the next run joins the same way. A refusal ends the run with the exit codes of `local_app_onboarding`, 3 to 7. Elsewhere it stops with "No persisted onboarding id".

### 3. Create the app volume

A render sender needs an app volume. The mixer composites the stream inside that box, and a sender for a target is what attaches the target to it ([App Volumes](/docs/developer-guide/app-volumes#creating-a-volume-idempotent)).

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:volume}}
```

`get_or_create` is idempotent: the common name identifies the volume, so a second run gets the same one back. The pose puts its center 1 m in front of the room origin, in room space; the size is in metres. The [App Volumes](/docs/developer-guide/tutorials/rust/app-volumes) tutorial covers volumes in full: the box and the content inside it, and their updates.

### 4. Follow the target apps in the peer list

Your application decides which targets it renders for; QAROS does not choose for it ([Rendering Streams](/docs/developer-guide/rendering-streams#finding-rendering-targets)). This example serves every running target app. The C tutorial waits for a render request instead, but a target requests your content only once it is attached to your app volume, and a sender is what attaches it. So a new application that waits for requests waits forever.

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:follow_targets}}
```

A peer is a target to render for when `PeerInfo::is_target_app` is true and `PeerInfo::app_state` is `AppState::Running`:

```rust
{{#include ../src/lib.rs:is_running_target}}
```

The guide picks targets by their presence (`qar_peer_spec_get_presence`): online, or unknown on a Hub that has not reconciled the peer ([Working with Peers](/docs/developer-guide/peers)). The safe API reads the app state instead; the presence getter is raw only, through `library.raw_api()` with the session's `as_raw()`. The app state is deprecated but computed from the presence, so for every current peer `Running` selects the same targets. On every listing, `follow_targets` drops the senders of targets that left or stopped running, and creates a sender for every new one.

The main loop subscribes to peer updates before it lists the peers, so a target that joins in between is not missed. Peer updates arrive on a library thread; `qaros::channel` forwards them into a channel the loop drains. The loop re-reads the list after any update, and at least every 2 s.

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:render_loop}}
```

One thread does everything. `qaros` handles are not `Send`: the SDK is designed for single-threaded use per runtime. So the loop serves all targets in turn with the non-blocking begin (step 6), and sleeps 2 ms only when no target had a frame ready.

### 5. Create a render sender per target

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:create_sender}}
```

`RenderSenderConfig::new(peer)` starts from the C defaults: auto-reconnect on, side-by-side layout, BGRA8 color, D32 depth. The example sets its views, its app volume, RGBA8 color and the side-by-side layout. `create_render_sender` blocks until the target answers. A cancel token with a 10 s timeout bounds that wait. A target that does not answer yields an error, `follow_targets` prints `No sender for ... yet`, and the next listing tries again.

### 6. Begin, render, show

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:render_next_frame}}
```

`try_begin_owned_frame` starts the library's begin-frame wait on its first call and polls it on later calls. `Ok(None)` means no frame yet: the target does not want one, or the receiver paused the stream. Poll again later.

A frame you get carries the pose and field of view to render each view with. They are in your app volume's app content space, the space you render in ([Coordinate Systems](/docs/developer-guide/coordinate-systems)). `frame.view(0)` and `frame.view(1)` return them for the two eyes, as declared in step 1. The example turns each into rays through the pixels:

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:eye_ray}}
```

`cpu_frame` hands out the frame's textures as byte slices with their pitch, and the views with their pixel rectangles. A view whose start lies past its end is mirrored. The example writes color views as RGBA or BGRA bytes and depth views as 32-bit floats:

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:write_pixel}}
```

The header does not specify the depth encoding. The example writes forward depth, 0 at the near plane and 1 at the far plane, as the Vulkan cube does:

```rust
{{#include ../src/bin/cpu_rendering_visualizer.rs:forward_depth}}
```

`show` presents the frame with the near and far planes the frame was rendered with, in app units. QAROS reprojects with exactly this metadata and your depth. Wrong values show up as swimming or misplaced content, never as an error code ([Rendering Streams](/docs/developer-guide/rendering-streams#the-frame-loop)).

### 7. Keep the sender, or end it

Some failures are part of the normal flow. The stream may be reconnecting, may have no transfer yet, or your app volume may not resolve yet where the receiver runs. For these, keep the sender and try the next frame ([Troubleshooting](/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow)):

```rust
{{#include ../src/lib.rs:is_waiting}}
```

| Status (`qaros::StatusCode`) | C name | What the example does |
| --- | --- | --- |
| `MEDIA_STREAMING_STREAM_IS_PAUSED` | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED` | keeps the sender; begin-frame waits through a pause, so it mostly shows up as `Ok(None)` |
| `MEDIA_STREAMING_STREAM_IS_RECONNECTING` | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING` | keeps the sender, tries the next frame |
| `MEDIA_STREAMING_NO_ACTIVE_TRANSFERS` | `QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS` | keeps the sender, tries the next frame |
| `RENDERING_APP_VOLUME_UNRESOLVED` | `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED` | keeps the sender; the call holds about 16 ms, so the loop does not spin |
| `RENDERING_STREAM_IS_CLOSED` | `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED` | drops the sender; the next listing creates a new one if the target still runs |
| anything else | | drops the sender, prints `Stopped streaming to ...`, as above |

Begin-frame reports the waiting codes wrapped inside `RENDERING_UNABLE_TO_BEGIN_FRAME`. `Error::has_code` compares only the outer code; `Error::has_cause` also matches a wrapped one, like the C `qar_result_has_code`. Use `has_cause` for these checks.

## Ownership and teardown

In C you destroy every handle yourself, in the right order: the sender before its session, the session before the runtime, the runtime before the library. In Rust each handle releases itself on drop and keeps its parent alive:

| Value | Owns | Keeps alive |
| --- | --- | --- |
| `Library` | `qar_library_init` for the process | |
| `Runtime` | the `QarRuntime` | the library |
| `Session` | the `QarSession`, the subscriptions' callback state | the runtime |
| `Subscription` (`_peer_subscription`) | the subscription's cancel token | |
| `RenderSender` (in each `Target`) | the `QarRenderSender` | the session |
| `OwnedFrame` | the frame info, until shown or dropped | the sender |
| `CpuFrame` | borrowed texture bytes | |

So the C order holds whatever order Rust drops them in. At the end of `run`, locals drop in reverse order of declaration: the targets and their senders, the subscription, the session, the runtime, then the library.

What the wrapper guarantees:

- **One open frame per sender.** A second begin while a frame is open fails before it reaches the library.
- **Texture bytes cannot outlive the frame.** The slices from `cpu_frame` borrow the frame, so the compiler rejects any use after `show`.
- **A dropped frame is released.** An `OwnedFrame` dropped without `show` destroys its frame info and closes the cycle without presenting.
- **The library stays loaded.** Dropping the `Library` calls `qar_library_destroy` but never unloads the DLL, because library threads may still run its code.

What it does not guarantee:

- **Your metadata.** It cannot check that you rendered with the poses `begin` gave you, or that `show` reports the planes you used.
- **Threads.** Handles are not `Send`, so the compiler keeps them on one thread. Callbacks still run on library threads; keep them short and never call blocking QAROS functions from them.
- **A dropped subscription guard stops delivery but frees nothing.** The callback state lives until the session is destroyed, so the receiver from `qaros::channel` disconnects only then.

## Troubleshooting

| Symptom | Cause | What to do |
| --- | --- | --- |
| `No persisted onboarding id in '...'` | not on Windows, no code, and no id file in the working directory | pass the Hub's pairing code |
| exit 4 or 5 after `joining the QAROS Hub on this machine as a local app` | the approval prompt was denied, closed or not answered within 120 s | run again and click **Allow** |
| exit 3 | the local-app join, which starts a Hub that is not running, reached no Hub | start QAROS and run again |
| `Pairing code rejected or expired` (`QAR_STATUS_PAKE_ERROR`) | wrong code, or the code changed (they live about 10 s) | read the current code and run again at once |
| `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE` | with a code: no Hub of your user answered; the example finds only your own Hub, on the Hub machine | start QAROS and run again |
| a not-found error naming every place it looked | no runtime found | install QAROS, set `QAROS_RUNTIME_DIR`, or pass `--qaros-library=PATH` |
| stays at `Looking for target apps ...` | no running target app in the session | open the Visualizer or start a headset |
| `No sender for <target> yet: ...`, repeated | the target does not answer the sender creation within 10 s | check the target is running; each attempt blocks the loop for up to 10 s, so the Vulkan tutorial's example retries less often |
| `Stopped streaming to <target>: ...` | a failure that is not part of the normal flow, `RENDERING_STREAM_IS_CLOSED` included | the next listing creates a new sender when the target still runs |
| content swims or sits at the wrong depth | near and far in `show` do not match the frame, or poses not taken from this frame | render with this frame's `view` results and report the planes you used |

## From C

The C tutorial is [CPU Rendering with a Visualizer](/docs/developer-guide/tutorials/c/cpu-rendering). What corresponds:

| C | Rust |
| --- | --- |
| `qar_library_load`, `qar_library_init` | `Library::load_default`, `Library::load` |
| `qar_runtime_create` | `Library::create_runtime` |
| `example_obtain_session` (rejoin, else code) | `join_session` in `examples/src/lib.rs` (rejoin, else code, else local app) |
| `qar_app_volumes_get_or_create` | `session.app_volumes().get_or_create` |
| `qar_render_sender_subscribe_requests` | not used: `session.subscribe_peer_updates` and `session.peers()` |
| `qar_render_sender_create` with `QAR_GRAPHICS_API_CPU` | `Session::create_render_sender` |
| `qar_render_sender_begin_frame` | `RenderSender::try_begin_owned_frame` (non-blocking), or `begin_frame` / `begin_owned_frame` (blocking) |
| `qar_render_frame_info_get_view_pose`, `_get_view_fov` | `OwnedFrame::view`, one `ViewPose` |
| `qar_render_sender_frame_cpu` | `OwnedFrame::cpu_frame` |
| `qar_render_sender_show_frame` | `OwnedFrame::show` |
| `qar_render_frame_info_handle_destroy`, `qar_render_stream_handle_destroy` | drop |
| `qar_session_handle_destroy`, `qar_runtime_handle_destroy`, `qar_library_destroy` | drop the values; the library is destroyed, not unloaded |
| `qar_result_has_code` | `Error::has_cause` |

The Rust example also differs in what it does: without a code it joins as a local app, it finds its targets in the peer list, serves all of them, declares its views, renders from each frame's poses with depth, and keeps senders through waiting statuses. The C example waits for one render request and writes a gradient into three frames.

## Next

- [D3D11 Rendering](/docs/developer-guide/tutorials/rust/d3d11-rendering): the same frame cycle with shared GPU textures, on a D3D11 device of your own.
- [Vulkan Rendering](/docs/developer-guide/tutorials/rust/vulkan-rendering): the same frame cycle with sender-owned Vulkan images, with a device of your own on the session's GPU.
- [Rendering Streams](/docs/developer-guide/rendering-streams): layouts, layout changes, hand tracking.

[^code]: The example also takes a pairing code, as its first argument that does not start with `--`; it passes no Hub address, so the code finds only your own Hub, on the Hub machine (a Hub elsewhere needs its host and port, `HubAddress`): `cargo run --release -p qaros-examples --bin cpu_rendering_visualizer -- <PAIRING-CODE> --frames=300`. It then stores the onboarding id it gets back in `cpu_rendering_visualizer.onboarding-id.txt` in the working directory, and later runs rejoin with it ([Onboarding and Rejoin](/docs/developer-guide/tutorials/rust/onboarding-and-rejoin)).
