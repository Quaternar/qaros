## Creating an app volume

```rust
{{#include ../tests/guide/app_volumes.rs:get_or_create}}
```

## World anchors

World anchors are raw only: call them through `library.raw_api()` with the session's `as_raw()`.

```rust
{{#include ../tests/guide/app_volumes.rs:world_anchor}}
```

## Gestures

Gestures are raw only: subscribe through `library.raw_api()` with the session's `as_raw()` and a callback of your own. The subscription belongs to its cancel token: it ends when you call `cancel()` on the token, when the token's last clone drops (the drop destroys the handle, which ends it too), or when the session drops.

```rust
{{#include ../tests/guide/app_volumes.rs:on_gesture}}

{{#include ../tests/guide/app_volumes.rs:subscribe_gestures}}

{{#include ../tests/guide/app_volumes.rs:end_gestures}}
```
