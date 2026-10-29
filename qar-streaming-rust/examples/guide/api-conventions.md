## Opaque handles and value IDs

- **Handles** are owned values that release their C handle on drop: `Library`, `Runtime`, `Session`, `RenderSender`, `CancelToken`, `OnboardingInvite`. Each keeps its parent alive, so the C teardown order holds.
- **Value IDs** are `Copy` newtypes over the 16 UUID bytes: `PeerId`, `SessionId`, `GuiPanelId`, `AppVolumeId`, `StreamId`, `OnboardingId`. They compare with `==`, and `library.uuid_to_string` renders one for persisting.

```rust
{{#include ../tests/guide/api_conventions.rs:handles_and_ids}}
```

## Results and status

Every fallible call returns **`qaros::Result`**. A C failure is `Error::Api`, which keeps the C function's name, the `StatusCode` and the text of `qar_result_message`; `?` propagates it.

```rust
{{#include ../tests/guide/api_conventions.rs:results}}
```

`error.has_code` matches the status code. `error.has_cause` also sees the causes that begin-frame wraps inside `RENDERING_UNABLE_TO_BEGIN_FRAME`, as `qar_result_has_code` does in C.

## Extensible init structs

The safe API builds the init structs, stamps them with ports of the `qar_*_default()` helpers and chains the extensions. You pass Rust values: an enum for the onboarding mode, an `Option` for an optional extension such as the hub address. For raw calls through `library.raw_api()`, `qaros_sys::defaults` has the same helpers.

```rust
{{#include ../tests/guide/api_conventions.rs:extension_chain}}
```

## Blocking / async pairs

- **Blocking**: `Runtime::onboard`, `Runtime::rejoin` and `Session::create_render_sender` block until done. Their `BlockingOptions` take an optional progress channel, fed from a library thread while you wait, and an optional `CancelToken` (`library.create_cancel_token`, with or without a timeout), which the borrow keeps alive for the call. Its `cancel_handle()` cancels it from another thread.
- **Async**: `Session::start_render_sender` returns a `PendingRenderSender` at once, and its `poll()` hands over the outcome once the library called back. The async onboard, rejoin and begin-frame calls are not wrapped: `library.raw_api()` reaches them, on raw handles created through it.

```rust
{{#include ../tests/guide/api_conventions.rs:blocking_call}}
```

```rust
{{#include ../tests/guide/api_conventions.rs:async_creation}}
```

## Enumeration and string getters

Enumerations return an owned snapshot, a `Vec` of plain Rust values: the binding runs the count-then-fetch and releases every handle it fetched. String getters return `String`.

```rust
{{#include ../tests/guide/api_conventions.rs:enumeration}}
```

## Ownership and lifetime

- Every handle is released on drop and keeps its parent alive (render sender, session, runtime, library), so teardown runs in the documented order whatever order your values drop in.
- Callbacks receive owned snapshots (`PeerInfo`, `AppVolumeInfo`, `GuiPanelInfo`, `RenderRequest`), never a borrowed C handle: keep them as long as you like.
- A subscription returns a `Subscription` guard: dropping it stops delivery, and `detach()` keeps it until the session ends.
- Handles are not `Send`: use a runtime from one thread; callbacks are the only cross-thread surface. A `CancelToken` is cancelled from another thread through its `cancel_handle()`.
- One `Library` per process: a second load is refused while one lives, and the runtime stays loaded until the process exits.
