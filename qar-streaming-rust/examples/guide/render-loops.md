## Creating a sender in each mode

```rust
{{#include ../tests/guide/render_loops.rs:create_modes}}
```

## Serving several viewers

```rust
{{#include ../tests/guide/render_loops.rs:viewer}}

{{#include ../tests/guide/render_loops.rs:viewers}}

{{#include ../tests/guide/render_loops.rs:update_viewers}}
```

## Blocking: one thread per viewer

`begin_owned_frame` is the blocking begin-frame; `begin_frame` is the same with a frame that borrows the sender.

```rust
{{#include ../tests/guide/render_loops.rs:blocking}}
```

## Non-blocking: one loop for every viewer

`try_begin_owned_frame` refuses while a frame of the sender is open, so the frame an arming call returns is kept in `Viewer::ready` and rendered on the next pass.

```rust
{{#include ../tests/guide/render_loops.rs:non_blocking}}
```

## Implementing the free-running mode

```rust
{{#include ../tests/guide/render_loops.rs:free_running}}
```
