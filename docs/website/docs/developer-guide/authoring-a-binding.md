---
sidebar_position: 10.5
title: Authoring a Language Binding
description: The rules every QAROS binding follows — how it finds and loads the runtime, binds the C ABI and keeps its conventions — so C, C#, Unity, Rust and any later binding behave the same.
---

# Authoring a Language Binding

A QAROS binding is a thin layer over the C ABI of `qar-streaming-c`
([Language Bindings](/docs/developer-guide/language-bindings)). The C header (`qar_streaming.h`) and
the C# binding (`Quaternar.Qaros`) are the reference implementations. A new binding (Python, …)
follows the same rules, so an application behaves the same whichever language it is written in.

## Loading the runtime

The runtime is one shared library, `qar-streaming-c.dll` (Linux: `libqar-streaming-c.so`). A
binding loads it at run time and never links it. Distributed applications run against the QAROS
installed on the PC and do not ship a copy of the runtime: a copy beside the application would be
loaded first.

### The default lookup

Called with no path, the load takes the **first** match:

| Step | Where | Notes |
|---|---|---|
| 0 | A `qar-streaming-c` already loaded in the process | A host (a game engine, a plug-in host, a test fixture) or another binding loaded it first. Load nothing; take a reference so that unloading stays balanced. |
| 1 | Beside the binding | The folder of the module the binding's code lives in: for C the executable or plug-in DLL that includes the header (`GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, …)`), for a managed binding its assembly's folder, then the application's base directory. Covers test binaries, an unpacked development ZIP and an app built into a QAROS tree. |
| 2 | The folder named by `QAROS_RUNTIME_DIR` | For running against a chosen runtime, such as a development build, without copying it. **When the variable is set, it is the only copy considered:** if the library is missing there or fails to load, report that and stop. |
| 3 | The installed QAROS: `<InstallFolder>\bin\qar-streaming-c.dll` | Windows. `InstallFolder` is the value under `HKLM\SOFTWARE\Quaternar\QAROS`. Read `HKLM` only, never `HKCU`, which every process of the user can write. **An installed copy that is missing or fails to load is reported, never skipped.** |
| 4 | Each folder on `PATH`, in order | Linux: the dynamic linker's own search (`LD_LIBRARY_PATH`, runpath, the ld.so cache). |

```mermaid
flowchart TD
    A["load(NULL)"] --> B{"qar-streaming-c already in the process?"}
    B -- yes --> Z["use it"]
    B -- no --> C{"beside the binding?"}
    C -- yes --> Z
    C -- no --> E{"QAROS_RUNTIME_DIR set?"}
    E -- yes --> F["QAROS_RUNTIME_DIR\qar-streaming-c.dll, or fail"]
    F --> Z
    E -- no --> G{"HKLM InstallFolder set?"}
    G -- yes --> H["InstallFolder\bin\qar-streaming-c.dll, or fail"]
    H --> Z
    G -- no --> I{"in a folder on PATH?"}
    I -- yes --> Z
    I -- no --> X["fail: no QAROS runtime found"]
```

- **Never search the current directory.** A library planted there would be loaded.
- **Load every candidate from its absolute path.** Resolve its dependencies from the library's own
  folder and System32 only, never from the application's folder:
  `LoadLibraryExW(path, NULL, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32)`.
- **Do not change the process's search path.** No `SetDllDirectory`, no `PATH` edits: they change
  how every later library of the host process is found.
- **Nothing found:** fail with a result whose message names every place looked at (beside the
  binding, `QAROS_RUNTIME_DIR`, the installation, `PATH`). Never let a missing library surface
  later as a crash or a loader exception.
- **A runtime beside the binding wins over the installation.** That is what lets a test binary or
  a development tree run its own build on a PC with QAROS installed. A hub only verifies the
  version and flavor of the runtime it installed, so a distributed application that ships its own
  copy runs an unverified runtime.

### An explicit path

- The binding also accepts the path of the library **file**. Only that file is tried, with no
  fallback, using the same `LoadLibraryExW` flags.
- Explicit paths are for development builds of QAROS itself. A distributed application uses the
  default lookup; to point a run at another runtime without code, set `QAROS_RUNTIME_DIR`.

### After loading

- Leave `QarRuntimeInit::runtime_binaries_folder_path` empty. The runtime then uses the folder it
  was loaded from, which is where it finds `qar-runtime-launcher` and its other processes.
- Expose the installed `bin` folder and the `QAROS_RUNTIME_DIR` folder for diagnostics (C:
  `qar_installed_bin_folder`, `qar_runtime_env_folder`; C#: `InstalledRuntime.TryGetBinFolder`,
  `InstalledRuntime.TryGetRuntimeDirFromEnvironment`), and the path actually loaded (C#:
  `InstalledRuntime.LoadedLibraryPath`).

### Reference implementations

| Binding | Default lookup | Explicit path |
|---|---|---|
| C | `qar_library_load(NULL)` in `qar_streaming.h` | `qar_library_load(path)` |
| C# | `InstalledRuntime.Load()`, called by `Library.Init` | `InstalledRuntime.Load(path)` |
| Rust | `Library::load_default(&config)` in `qaros`, over `qaros_sys::QarApi::load_default()` | `Library::load(path, &config)`, over `QarApi::load(path)` |

The [Vulkan cube example](https://github.com/Quaternar/qaros/tree/main/qar-streaming-c/examples/vulkan_cube/main.cpp)
shows the application side: it calls `qar_library_load(NULL)` and does no searching of its own.

## Binding the C ABI

- **Symbols:** resolve the exported `qar_impl_<name>` functions by name. Loading is all or nothing:
  if one symbol is missing, unload the library and fail, so a half-bound API is never observable.
- **Stability:** the ABI only grows. Functions are appended and never change their signature, and
  structs grow through their extension chain. A binding built against an older header keeps
  working with a newer runtime.
- **Init structs** start with a `{type, next}` header. Fill each one exactly as the header's
  `qar_<struct>_default()` does: the type tag and the default values.
- **Extensions** are chained through `header.next`. Keep a chain alive for the whole call that
  reads it.
- **Results** are returned by value (`QarResult`). Map them to the language's result type rather
  than throwing for expected failures, and keep the status code and message.
- **Strings and paths** are UTF-8 in both directions. Never use the platform's ANSI code page.
- **Handles:** each one has its `qar_<type>_handle_destroy`. Tie it to the language's lifetime
  mechanism (RAII, `IDisposable`, `Drop`).
- **Callbacks** run on library threads. Hand their data to the application's thread; never call
  back into a UI toolkit from them.

## Checklist for a new binding

- [ ] The default lookup follows steps 0–4 above, in that order. A set `QAROS_RUNTIME_DIR` or an
  installed QAROS that does not load is reported, never skipped.
- [ ] The explicit path loads only that file.
- [ ] Candidates load from their absolute path, with dependencies from their own folder and
  System32. The current directory is never searched, and the process search path is never
  changed.
- [ ] A missing runtime is a result whose message names every place looked at, not a crash.
- [ ] `runtime_binaries_folder_path` is left empty by default.
- [ ] Every symbol resolves at load, or the load fails as a whole.
- [ ] Strings are UTF-8, results are values, handles are released, callbacks are marshalled.
