## The SDK and loading the runtime

The Rust binding is the Cargo workspace in `qar-streaming-rust/` of this repository:

- **`qaros-sys`**: the raw bindings to every `qar_impl_*` function, generated from `qar_streaming.h` and committed, with the runtime lookup and ports of the `qar_*_default()` initializers.
- **`qaros`**: the safe API: handles released on drop, `qaros::Result` errors, callbacks that receive owned snapshots.
- **Examples**: Rust ports of the C examples in `qar-streaming-rust/examples/`. The [Rust tutorials](/docs/developer-guide/tutorials/rust/) are built from them.

The crates are not published to crates.io. Add `qaros` to the `[dependencies]` section of your crate's `Cargo.toml` as a git dependency on this repository; Cargo finds the crate by its name inside it:

```toml
[dependencies]
qaros = { git = "https://github.com/Quaternar/qaros" }
```

Pin it with `rev = "<commit>"` or `tag = "<tag>"` on the same line, so the header the binding is generated from stays the one your runtime exports in full (see below).

With a clone of the repository, use a path to its `qar-streaming-rust/crates/qaros` folder instead, relative to your `Cargo.toml`:

```toml
[dependencies]
qaros = { path = "../qaros/qar-streaming-rust/crates/qaros" }
```

The GPU render senders are features: add `features = ["vulkan"]` or `features = ["d3d11"]` (Windows; on macOS only for the tests against `qaros-stub`) to the line.

**Load the installed runtime.** `Library::load_default` finds `qar-streaming-c.dll` with the lookup of `qar_library_load(NULL)` and initializes the library. It is `unsafe` because the runtime it finds must match the bindings. With no runtime found it returns an error naming every place it looked, so your app keeps working without AR/VR. Leave `RuntimeConfig::runtime_binaries_directory` at `None`: the library defaults it to the folder it came from.

```rust
{{#include ../tests/guide/getting_started.rs:load_installed_runtime}}
```

Functions are loaded by symbol name into a table the binding owns (`qaros_sys::QarApi`), all of them or none. The binding therefore needs a QAROS runtime that exports every function of the header it was generated from (this repository's `qar-streaming-c/include/qar_streaming.h`), and that is the runtime of the Hub you join. An older runtime fails to load, and the error names the functions it lacks. Walkthrough: [Dynamic Loading tutorial](/docs/developer-guide/tutorials/rust/dynamic-loading).

**Run the examples** from `qar-streaming-rust/`. With QAROS installed, they find the installed runtime:

```powershell
cargo run -p qaros-examples --bin dynamic_loading
```

To run against a QAROS development hub instead, name its instance with `QAR_INSTANCE`: the hub records its runtime folder under that name, and the examples load that runtime (an app the hub starts receives the variable already):

```powershell
$env:QAR_INSTANCE = "dev-0123456789abcdef" # the development hub's instance name
cargo run -p qaros-examples --bin dynamic_loading
```

## Lifecycle

```rust
{{#include ../tests/guide/getting_started.rs:lifecycle}}
```
