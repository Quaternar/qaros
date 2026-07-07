---
sidebar_position: 8
title: Language Bindings
description: Using QarOS from C++, C# / .NET, and Unity.
---

# Language Bindings

The C API (`qar_streaming.h`) is the stable ABI; every binding is a thin layer over it. Because the DLL exports plain named functions and all structs grow only through the extension-chain mechanism, bindings stay compatible across runtime updates.

## C++

The single header compiles cleanly as C++ — use it directly. Recommended idioms:

- wrap each handle in a small RAII type calling the matching `qar_*_handle_destroy`,
- wrap `QarResult` in a checker that throws or logs via `qar_result_log_if_error`,
- keep all calls on one thread; marshal callback data through your own queue.

## C# / .NET and Unity

C# bindings live in `qar-streaming-sharp`, with Unity integration (prefabs, scripts, components) in the QarOS Unity package.

:::note TODO
This section should document, once the C# bindings for the v0 onboarding API are published into this repository:
- installing the binding (NuGet package name / Unity package),
- the mapping rules from C to C# (handles → `SafeHandle` classes, `QarResult` → exceptions or result types, callbacks → delegates and their GC-pinning rules),
- the onboarding/rejoin flow in idiomatic C# (persisting `OnboardingId` in `PlayerPrefs`/settings),
- Unity specifics: which calls are main-thread-only, how the render sender integrates with Unity's render pipeline (native texture pointers), and the lifecycle relative to `MonoBehaviour` events,
- a compiled C# example project mirrored after the C examples, so C# snippets in these docs are build-verified the same way the C ones are.

Until then, the C# tabs in the tutorials are placeholders marked TODO.
:::

## Other languages

Any language with C FFI can consume the API directly from the single header (Rust `bindgen`, Python `cffi`, etc.). The conventions that matter for FFI users: results returned **by value**, init structs with `{type, next}` headers that must be stamped (replicate the `qar_*_default()` helpers), and callbacks invoked from library threads.
