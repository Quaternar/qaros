---
sidebar_position: 1
title: Fixture Page
description: A fixture tutorial page for the generator tests.
---

## What this fixture shows

A region included at the top level, with `inline <code>` and an escaped \{ brace:

```rust
{{#include ../src/regions.rs:indented_region}}
```

1. A region included inside a list item keeps the item's indentation:

    ```rust
    {{#include ../src/regions.rs:top_level_region}}
    ```
