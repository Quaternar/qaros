## Updating the display name

```rust
{{#include ../tests/guide/peers.rs:update_display_name}}
```

## Querying peers

```rust
{{#include ../tests/guide/peers.rs:query_peers}}
```

`PeerInfo::identity` carries the presence, role, category and device identity (`qar_peer_spec_get_presence` and the others), where the bindings declare them; `PeerInfo::app_state` is what the header derives from the presence.

```rust
{{#include ../tests/guide/peers_identity.rs:read_presence}}
```

## Show the user what each target is

The device id and name can change while a peer runs: read them again from a peer update.

```rust
{{#include ../tests/guide/peers_identity.rs:show_targets}}
```

## A source app that is a camera

`PeerPresentation::device_identity` chains the `QarPeerDeviceIdentity` on the presentation. `SourceDeviceCategory` holds only the categories an application may declare, so `QAR_STATUS_PEER_DEVICE_CATEGORY_NOT_ALLOWED` cannot happen; an empty device id fails with `StatusCode::ARGUMENT_NOT_SUPPORTED`.

```rust
{{#include ../tests/guide/peers_identity.rs:onboard_as_camera}}
```

## Refine the identity later

`Session::update_device_identity` calls `qar_peer_update_device_identity`.

```rust
{{#include ../tests/guide/peers_identity.rs:refine_identity}}
```

## Subscribing to updates

```rust
{{#include ../tests/guide/peers.rs:subscribe_updates}}
```
