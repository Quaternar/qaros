## Updating the display name

```rust
{{#include ../tests/guide/peers.rs:update_display_name}}
```

## Querying peers

```rust
{{#include ../tests/guide/peers.rs:query_peers}}
```

The binding does not wrap the presence, role, category and device identity getters (`qar_peer_spec_get_presence` and the others): they are raw only, through `library.raw_api()` with the session's `as_raw()`. `PeerInfo` carries the app state, which the header derives from the presence. Reading the presence itself, raw:

```rust
{{#include ../tests/guide/peers_identity.rs:read_presence}}
```

The role, category and device getters follow in the next sections.

## Show the user what each target is

The role, category and device getters are raw only: call them through `library.raw_api()` with the session's `as_raw()`.

```rust
{{#include ../tests/guide/peers_identity.rs:show_targets}}
```

## A source app that is a camera

Declaring your device is raw only: onboard through `library.raw_api()` with the runtime's `as_raw()`. The session it returns is a raw `QarSession`, not a `qaros::Session`.

```rust
{{#include ../tests/guide/peers_identity.rs:onboard_as_camera}}
```

## Refine the identity later

Raw only: call `qar_peer_update_device_identity` through `library.raw_api()` with the session's `as_raw()`.

```rust
{{#include ../tests/guide/peers_identity.rs:refine_identity}}
```

## Subscribing to updates

```rust
{{#include ../tests/guide/peers.rs:subscribe_updates}}
```
