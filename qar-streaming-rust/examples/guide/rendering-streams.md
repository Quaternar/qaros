## Finding rendering targets

`PeerInfo::identity` carries the role and presence (where the bindings declare them). `PeerInfo` also selects the same targets without them: `is_target_app` is role == target, and the app state, which the header derives from the presence, reads `Running` for an `ONLINE` or `UNKNOWN` peer and `ShuttingDown` for an `OFFLINE` or `LEAVING` one, which may come back.

```rust
{{#include ../tests/guide/rendering_streams.rs:find_targets}}
```

## Creating a render sender

```rust
{{#include ../tests/guide/rendering_streams.rs:create_sender}}
```

## The frame loop

```rust
{{#include ../tests/guide/rendering_streams.rs:frame_loop}}
```

## Beginning a frame without blocking

`RenderSender::start_owned_frame` returns a `PendingFrame` that delivers an `OwnedFrame`, with the CPU, Vulkan and D3D11 access of any other frame.

```rust
{{#include ../tests/guide/rendering_streams.rs:begin_frame_async}}
```

## The D3D11 path

```rust
{{#include ../tests/guide/rendering_streams.rs:d3d11_params}}
```

## Reading back the viewer: hand tracking

Hands are raw only: call `qar_render_sender_last_hands` through `library.raw_api()` with the sender's `as_raw()`.

```rust
{{#include ../tests/guide/rendering_streams.rs:last_hands}}
```
