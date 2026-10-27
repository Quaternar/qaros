## Creating a panel

```rust
{{#include ../tests/guide/gui_panels.rs:get_or_create}}
```

## Manipulating panels

```rust
{{#include ../tests/guide/gui_panels.rs:manipulate}}
```

## Per-peer visibility

Editing the visibility set is raw only: call `qar_gui_panels_update_visible_to` through `library.raw_api()` with the session's `as_raw()`.

```rust
{{#include ../tests/guide/gui_panels.rs:update_visible_to}}
```
