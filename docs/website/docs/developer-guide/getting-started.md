---
sidebar_position: 1
title: Getting Started
description: Set up the QarOS SDK — the single header, static vs. dynamic loading, and your first build.
---

# Getting Started

The QarOS C API is designed so the *simplest possible integration is genuinely simple*: one header, one DLL, plain C functions. Everything else — C++ helpers, C# bindings, dynamic loading — is layered on top of that.

## What you get

The SDK consists of:

- **`qar_streaming.h`** — a single amalgamated header containing the complete v0 C API. It is auto-generated from the authoritative API sources on every runtime build, so the header in a given package always matches its binaries exactly. Never edit it.
- **`qar-streaming-c.dll` / `.so`** — the runtime library your app links against (or loads at runtime).
- **Runtime binaries** — the services the library spawns/uses (delivered in the binary package's `bin/` folder).
- **Examples** — small, self-contained C programs in `qar-streaming-c/examples/` of this repository. They are compiled as part of the repository build, so they are guaranteed to be valid against the current header. Tutorials in this guide reference their code.

## Static linking vs. dynamic loading

The API supports two consumption modes, with identical function names in both:

| | Static/import-library linking | Dynamic loading |
|---|---|---|
| How | Link `qar-streaming-c.lib`, call functions directly | Define `QAR_ENABLE_DYNAMIC_LOADING`, load the DLL at runtime |
| Startup dependency | DLL must be present at process start | Your app starts without QarOS; loads it on demand |
| Failure mode | Loader error dialog | You handle a `bool` load failure gracefully |
| Recommended for | Tools that always ship with QarOS | Plugins and applications where AR/VR is optional |

Dynamic loading is the recommended mode for integrating into an existing application: your software keeps working on machines without QarOS installed.

```c
// Dynamic-loading mode: one macro before the include, one macro in ONE .c file.
#define QAR_ENABLE_DYNAMIC_LOADING
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING() // exactly once per application

int main(int argc, char** argv)
{
    if (!qar_library_load("path/to/qar-streaming-c.dll"))
    {
        // QarOS not present — run without AR/VR features
        return 1;
    }
    // ... use the API exactly as in static mode ...
    qar_library_unload();
    return 0;
}
```

Under the hood, functions are loaded from the DLL *by symbol name* into application-owned function tables — there is no loader struct in the ABI, so newer DLLs keep working with older applications and vice versa. The full walkthrough is the compiled tutorial: [Dynamic Loading tutorial](/api/qar-streaming-c/pages/qar-c-tutorial-dynamic-loading/).

## Building the examples in this repository

```powershell
# 1. Drop the QarOS binary package into ./package (see Deployment and Installation)
# 2. Configure and build
cmake --preset x64-windows
cmake --build --preset x64-windows-debug

# 3. Run an example
./build/x64-windows/Debug/dynamic_loading.exe ./package/bin/qar-streaming-c.dll
```

## The canonical program skeleton

Every QarOS application has the same shape:

```c
#include <qar_streaming.h>

int main(void)
{
    // 1. Library init (process-wide, once)
    QarLibraryInit lib_init = qar_library_init_default();
    lib_init.enable_console_logging = true;
    qar_result_log_if_error(qar_library_init(&lib_init));

    // 2. Runtime create (your process's QarOS context)
    QarRuntimeInit rt_init = qar_runtime_init_default();
    QarRuntime* runtime = NULL;
    qar_result_log_if_error(qar_runtime_create(&rt_init, &runtime));

    // 3. Obtain a session: rejoin if we onboarded before, otherwise onboard
    //    (see Onboarding and Sessions)

    // 4. Do work: streams, panels, volumes, peers

    // 5. Teardown, in reverse
    qar_runtime_destroy(runtime);
    qar_result_log_if_error(qar_library_destroy());
    return 0;
}
```

Steps 3 and 4 are what the rest of this guide is about. Before diving in, read [API Conventions](/docs/developer-guide/api-conventions) once — the API follows a handful of strict patterns, and knowing them makes every module predictable.

## Language bindings

- **C** — the native surface, documented throughout this guide and in the [C API Reference](/api/qar-streaming-c/).
- **C++** — the C header is C++-clean; use it directly (RAII wrappers are trivial to write over the `*_handle_destroy` functions).
- **C# / .NET (Unity)** — bindings exist in `qar-streaming-sharp`. See [Language Bindings](/docs/developer-guide/language-bindings).
