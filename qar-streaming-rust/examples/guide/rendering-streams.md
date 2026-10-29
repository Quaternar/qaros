## Finding rendering targets

The role and presence getters are raw only (through `library.raw_api()` with the session's `as_raw()`; [Enumerating peers](/docs/developer-guide/peers#enumerating-peers) reads the presence that way). `PeerInfo` selects the same targets: `is_target_app` is role == target, and the app state, which the header derives from the presence, reads `Running` for an `ONLINE` or `UNKNOWN` peer and `ShuttingDown` for an `OFFLINE` or `LEAVING` one, which may come back.

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

## The D3D11 path

```rust
{{#include ../tests/guide/rendering_streams.rs:d3d11_params}}
```

## Reading back the viewer: hand tracking

Hands are raw only: call `qar_render_sender_last_hands` through `library.raw_api()` with the sender's `as_raw()`.

```rust
{{#include ../tests/guide/rendering_streams.rs:last_hands}}
```
