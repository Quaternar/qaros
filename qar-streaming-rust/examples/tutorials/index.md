---
sidebar_position: 0
title: Rust
description: Step-by-step walkthroughs of the Rust binding's examples, from loading the runtime to rendering into an app volume.
---

These tutorials are for developers who integrate QAROS into a Rust application: a tool that
places panels in the room, an app that streams rendered frames, a service that follows the
session. Each one walks through one example program of the Rust binding, in the order the program
runs, and explains why each call is there.

You do not need to know the C API. Where the Rust binding differs from C, mostly in ownership and
callbacks, the tutorial says so, and each ends with a short mapping to its C counterpart for
readers who know it. The concepts behind
the calls (sessions, peers, panels, volumes, render senders) are explained once, in the guide
pages the tutorials link to.

## The crates

The binding lives in `qar-streaming-rust/` in the QAROS repository, a Cargo workspace.

| Crate | What it is | You use it |
| --- | --- | --- |
| `qaros` | The safe API: library and runtime, onboarding and rejoin, peers, app volumes, GUI panels, the CPU, Vulkan and D3D11 render senders (Vulkan behind the `vulkan` feature, D3D11 behind the `d3d11` feature: Windows; on macOS only for the tests against `qaros-stub`). Handles release themselves on drop; errors are `qaros::Result`. | In your application |
| `qaros-sys` | The raw bindings to every exported `qar_impl_*` function, the runtime lookup and loading, and ports of the `qar_*_default()` initializers. Reached from `qaros` through `library.raw_api()`, with the `as_raw()` handle of a safe value such as a `Session`, for what the safe API leaves out: hands, gestures, world anchors and the rest of the [coverage table](https://github.com/Quaternar/qaros/blob/main/qar-streaming-rust/README.md#coverage) in the binding's README. | When the safe API does not cover a call |
| `qaros-examples` | The example programs these tutorials walk through, and their shared helpers (`examples/src/lib.rs`). | To learn, and to copy from |
| `qaros-stub` | An in-memory stand-in for the runtime library. It exports the same symbols, keeps panels, volumes, peers and senders in memory, and records every call. | To run the tests and most examples without QAROS |
| `qaros-conformance` | Test-only: compiles the header's default initializers with the C compiler and checks the Rust ports against them. | Never directly |

## Add the binding to your crate

The crates are not published on crates.io. Add `qaros` to the `[dependencies]` section of your
crate's `Cargo.toml` as a git dependency on the QAROS repository; Cargo finds the crate by its name
inside the repository:

```toml
[dependencies]
qaros = { git = "https://github.com/Quaternar/qaros" }
```

Pin it with `rev = "<commit>"` or `tag = "<tag>"` on the same line, so the header the binding is
generated from stays the one your runtime exports in full
([On Windows, against QAROS](#on-windows-against-qaros)). If you have cloned the repository, depend on the `qar-streaming-rust/crates/qaros` folder of your clone
instead, with a path relative to your `Cargo.toml`:

```toml
[dependencies]
qaros = { path = "../qaros/qar-streaming-rust/crates/qaros" }
```

The GPU render senders are behind features: add `features = ["vulkan"]` for the Vulkan sender or
`features = ["d3d11"]` for the D3D11 sender (Windows; on macOS only for the tests against
`qaros-stub`) to either line, for example
`qaros = { git = "https://github.com/Quaternar/qaros", features = ["vulkan"] }`.

## Build and run

You need the latest stable Rust. Run every command from the
`qar-streaming-rust` folder of the repository:

```sh
cargo run -p qaros-examples --bin <example> -- <arguments>
```

Everything after `--` goes to the example. Two Cargo features change what is built:

- `--features vulkan` builds `vulkan_cube`, which requires it.
- `--features link` links the import library `qar-streaming-c.lib` (directory in `QAROS_LIB_DIR`)
  instead of loading the runtime at start. The runtime package does not ship that library, so
  you rarely need it.

### On Windows, against QAROS

The binding needs a QAROS runtime that exports every function of the header it was generated
from (the repository's `qar-streaming-c/include/qar_streaming.h`), and that is the runtime of
the Hub you join. It loads all of those functions or none, so an older runtime fails to load,
and the error names the functions it lacks
([Dynamic Loading](/docs/developer-guide/tutorials/rust/dynamic-loading#troubleshooting)).

The examples load the runtime with the default lookup every binding follows
([Authoring a Language Binding](/docs/developer-guide/authoring-a-binding)): a copy beside the
executable, else the folder `QAROS_RUNTIME_DIR` names, else the installed QAROS, else `PATH`. To
load one specific library file instead, pass `--qaros-library=PATH`, set `QAROS_DLL`, or put the
path on the first line of `qaros-library.txt` beside the executable (for a start without
arguments). `vulkan_cube` and `local_app_onboarding` always use the default lookup, as
their C originals do.

`cpu_rendering_visualizer`, `d3d11_render_sender`, `gui_panel_operations` and
`app_volume_management` run on the Hub machine and join the Hub as a local app (on Windows): the
first run of each executable asks you to click **Allow** in QAROS, and a local app stores nothing.
Joining as a local app starts the Hub when it is not running; a running Hub is left as it is, and
the Hub keeps running after the example exits. They also take an optional pairing code as their
first argument: they then store the onboarding id they get as `<example>.onboarding-id.txt` in
the folder you run them from and rejoin with it on later runs. They pass no Hub address, so the
code finds only your own Hub, on the Hub machine; a Hub elsewhere needs its host and port
(`HubAddress`, see [Onboarding and Rejoin](/docs/developer-guide/tutorials/rust/onboarding-and-rejoin)). `onboarding_and_rejoin` is the example of that path: it onboards with a code on
its first run and keeps one id per runtime, `onboarding_and_rejoin.primary-onboarding-id.txt` and
`onboarding_and_rejoin.sibling-onboarding-id.txt`.

### On macOS, against the stub

There is no QAROS runtime for macOS. The examples that take a library path run against
`qaros-stub` instead. Run them from a folder of their own, here `target/stub-run` (Cargo finds
the workspace from any folder inside it):

```sh
cargo build -p qaros-stub
mkdir -p target/stub-run && cd target/stub-run
QAROS_DLL="$PWD/../debug/libqaros_stub.dylib" \
    cargo run -p qaros-examples --bin gui_panel_operations -- ANY-CODE
```

The stub accepts any pairing code except `WRONG` (a pairing error) and `UNREACHABLE` (no Hub).
It runs the program's logic and shows nothing. Like a real run, a stub run stores its onboarding
id in the folder it runs from, and a later run against QAROS from that folder would first try to
rejoin with the stub's id; the folder of its own keeps them apart.

`vulkan_cube` and `local_app_onboarding` take no library path: they find the stub through
`QAROS_RUNTIME_DIR`, as their pages show ([Vulkan Rendering](/docs/developer-guide/tutorials/rust/vulkan-rendering#run-it),
[Local App Onboarding](/docs/developer-guide/tutorials/rust/local-app-onboarding#run-it)).
`d3d11_render_sender` prints that it requires Windows and exits; on Windows, against the stub, it
submits its frames without drawing, since the stub lends no D3D11 textures.

## The tutorials

Read them in order the first time: each builds on the session the earlier ones explain.

1. [Dynamic Loading](/docs/developer-guide/tutorials/rust/dynamic-loading): find, load and
   initialize the runtime library, and tear it down.
2. [Onboarding and Rejoin](/docs/developer-guide/tutorials/rust/onboarding-and-rejoin): join a
   Hub with a pairing code once, then rejoin with the stored onboarding id.
3. [Local App Onboarding](/docs/developer-guide/tutorials/rust/local-app-onboarding): join the
   Hub from the Hub machine as a local app, after the user clicks Allow.
4. [CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering): find the target apps in
   the peer list and stream frames rendered on the CPU to each.
5. [D3D11 Rendering](/docs/developer-guide/tutorials/rust/d3d11-rendering): stream frames
   drawn with Direct3D 11 into the sender's shared textures.
6. [Vulkan Rendering](/docs/developer-guide/tutorials/rust/vulkan-rendering): stream a stereo
   cube rendered with Vulkan into every running target app.
7. [GUI Panels](/docs/developer-guide/tutorials/rust/gui-panels): place a shared web panel in the
   room, follow its updates and end a subscription safely.
8. [App Volumes](/docs/developer-guide/tutorials/rust/app-volumes): create the box your content
   lives in, move the content inside it, and handle an unresolved volume when rendering.
