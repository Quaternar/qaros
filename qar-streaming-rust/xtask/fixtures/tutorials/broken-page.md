---
title: Broken Fixture Page
description: A fixture page with one problem per paragraph.
---

An include outside a code fence:
{{#include ../src/regions.rs:top_level_region}}

```rust
{{#include ../src/regions.rs:missing_region}}
```

```rust
{{#include ../src/absent.rs:any_region}}
```

```rust
{{#include ../src/fence-in-region.rs:fenced_region}}
```

Prose that compares a < b outside code.

Prose that opens {an expression} outside code.

```rust
let fixture = "{{#rustdoc_include}}";
{{#include ../src/regions.rs}}
