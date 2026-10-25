---
sidebar_position: 10.5
title: Authoring a Language Binding
description: The rules every QAROS binding follows — how it finds and loads the runtime, binds the C ABI and keeps its conventions — so C, C#, Unity, Rust and any later binding behave the same.
---

# Authoring a Language Binding

A QAROS binding is a thin layer over the C ABI of `qar-streaming-c`
([Language Bindings](/docs/developer-guide/language-bindings)). The C header (`qar_streaming.h`) and
the C# binding (`Quaternar.Qaros`) are the reference implementations. A new binding (Rust, Python,
…) follows the same rules, so an application behaves the same whichever language it is written in.

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
| 2 | The runtime of the QAROS instance named by `QAR_INSTANCE` | Windows. `default` is the installed QAROS (step 3's folder). Any other name: the folder in the value `RuntimeFolder` under `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>`, else that record's `<InstallFolder>\bin`; a folder that is not a full path counts as none. A QAROS development hub records its own build output there and sets `QAR_INSTANCE` for the apps it starts, so they run the runtime of the hub they belong to. **When the variable is set, it is the only copy considered:** an invalid name (1–64 of `A-Z a-z 0-9 . _ -`), a name with no recorded folder, or a library missing there or failing to load is reported, naming the instance, never a reason to load the installed runtime. |
| 3 | The installed QAROS: `<InstallFolder>\bin\qar-streaming-c.dll` | Windows. `InstallFolder` is the value under `HKLM\SOFTWARE\Quaternar\QAROS`. An installation is read from `HKLM` only, never `HKCU`. **An installed copy that is missing or fails to load is reported, never skipped.** |
| 4 | Each folder on `PATH`, in order | Linux: the dynamic linker's own search (`LD_LIBRARY_PATH`, runpath, the ld.so cache). |

```mermaid
flowchart TD
    A["load(NULL)"] --> B{"qar-streaming-c already in the process?"}
    B -- yes --> Z["use it"]
    B -- no --> C{"beside the binding?"}
    C -- yes --> Z
    C -- no --> E{"QAR_INSTANCE set?"}
    E -- yes --> F["that instance's RuntimeFolder\qar-streaming-c.dll, or fail"]
    F --> Z
    E -- no --> G{"HKLM InstallFolder set?"}
    G -- yes --> H["InstallFolder\bin\qar-streaming-c.dll, or fail"]
    H --> Z
    G -- no --> I{"in a folder on PATH?"}
    I -- yes --> Z
    I -- no --> X["fail: no QAROS runtime found"]
```

- **Never search the current directory.** A library planted there would be loaded.
- **`QAR_INSTANCE` and its `RuntimeFolder` are per-user settings.** Every process of the user can
  set the variable and write the `HKCU` record, so step 2 carries the trust of an environment
  variable naming a folder: read the record only when `QAR_INSTANCE` names that instance. No step
  verifies the runtime's signature before loading it.
- **Load every candidate from its absolute path.** Resolve its dependencies from the library's own
  folder and System32 only, never from the application's folder:
  `LoadLibraryExW(path, NULL, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32)`.
- **Do not change the process's search path.** No `SetDllDirectory`, no `PATH` edits: they change
  how every later library of the host process is found.
- **Nothing found:** fail with a result whose message names every place looked at (beside the
  binding, `QAR_INSTANCE`, the installation, `PATH`). Never let a missing library surface
  later as a crash or a loader exception.
- **A runtime beside the binding wins over the installation.** That is what lets a test binary or
  a development tree run its own build on a PC with QAROS installed. A hub only verifies the
  version and flavor of the runtime it installed, so a distributed application that ships its own
  copy runs an unverified runtime.

### An explicit path

- The binding also accepts the path of the library **file**. Only that file is tried, with no
  fallback, using the same `LoadLibraryExW` flags.
- Explicit paths are for development builds of QAROS itself. A distributed application uses the
  default lookup; to run against the runtime of a QAROS development hub, set `QAR_INSTANCE` to
  that hub's instance name, as the hub does for the apps it starts.

### After loading

- Leave `QarRuntimeInit::runtime_binaries_folder_path` empty. The runtime then uses the folder it
  was loaded from, which is where it finds `qar-runtime-launcher` and its other processes.
- Expose the installed `bin` folder and the requested instance's runtime folder for diagnostics
  (C: `qar_installed_bin_folder`; C#: `InstalledRuntime.TryGetBinFolder`,
  `InstalledRuntime.TryGetRequestedInstance`, `InstalledRuntime.TryGetInstanceRuntimeFolder`),
  and the path actually loaded (C#: `InstalledRuntime.LoadedLibraryPath`).

### Reference implementations

| Binding | Default lookup | Explicit path |
|---|---|---|
| C | `qar_library_load(NULL)` in `qar_streaming.h` | `qar_library_load(path)` |
| C# | `InstalledRuntime.Load()`, called by `Library.Init` | `InstalledRuntime.Load(path)` |

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

- [ ] The default lookup follows steps 0–4 above, in that order. A set `QAR_INSTANCE` that names
  no usable runtime, or an installed QAROS that does not load, is reported, never skipped.
- [ ] The explicit path loads only that file.
- [ ] Candidates load from their absolute path, with dependencies from their own folder and
  System32. The current directory is never searched, and the process search path is never
  changed.
- [ ] A missing runtime is a result whose message names every place looked at, not a crash.
- [ ] `runtime_binaries_folder_path` is left empty by default.
- [ ] Every symbol resolves at load, or the load fails as a whole.
- [ ] Strings are UTF-8, results are values, handles are released, callbacks are marshalled.
