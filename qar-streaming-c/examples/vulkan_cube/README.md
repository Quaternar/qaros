# Vulkan cube source

A standalone Windows source app rendering a rotating, colored cube with stereo color and depth. Each viewer gets its own camera of the same cube.

It is the reference for a source app on the QAROS PC: it loads the installed QAROS runtime and joins as a **local app**, with no code. Step-by-step guide: Developer Guide, *Getting Started*.

## Requirements

- Windows 10/11 x64 with QAROS installed (Operator Guide, *Installation*).
- NVIDIA GPU supporting Vulkan external memory and semaphores.
- Visual Studio 2022 Build Tools ("Desktop development with C++"), CMake 3.29+, Git. The Vulkan headers, loader and shader compiler come from vcpkg.

## Build

From the `qaros` repository root, in a Developer PowerShell for VS 2022:

```powershell
cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
cmake --build --preset x64-windows-release --target qar-vulkan-source
```

- The first configure bootstraps vcpkg and builds the dependencies; it takes a while.
- Output: `build/x64-windows/qar-streaming-c/examples/vulkan_cube/Release/qar-vulkan-source.exe` (`Debug/` with `x64-windows-debug`). Shaders are embedded.

## Run (local app, the default)

```powershell
.\build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe
```

1. The app loads the installed QAROS (`qar_library_load(NULL)`) and asks it to join (`QarOnboardLocalAppExt`).
2. First run: QAROS asks **"Allow qar-vulkan-source.exe to join QAROS?"**, in the Visualizer or as a tray window. Click **Allow** within 120 s. Meanwhile the console shows `Waiting for approval in QAROS hub`.
3. `Connected. Waiting for rendering targets.` Open the cube's app volume on a viewer (Visualizer or headset). Each additional viewer gets its own stream and camera.
4. Later runs join without a prompt while the executable is unchanged.

- Unsigned build: every rebuild asks again; a copy elsewhere is refused until its approval is revoked. See *Onboarding and Sessions*, "The trust record".
- Ctrl+C stops the app. Ctrl+C while it waits for approval cancels the wait.

| Console message | Status | What to do |
|---|---|---|
| `Could not load QAROS. Install QAROS on this PC...` | library load failed | install QAROS |
| `QAROS is not running on this PC...` | 1802 | start QAROS from the Start menu, run again |
| `QAROS did not allow this app to join...` | 1804 | someone clicked Deny or revoked it: run again to be asked again |
| `Nobody answered the approval prompt in QAROS in time...` | 1805 | press Enter to ask again |
| `The program answering on this PC is not the installed QAROS...` | 1806 | reinstall QAROS; the app does not retry |
| `This app no longer matches what QAROS approved...` | 1807 | revoke it in the Visualizer (Connection panel, Approved apps), run again |

## Run with an onboarding code (hub on another PC)

```powershell
.\qar-vulkan-source.exe --code --host hub-pc.example.local   # asks: Hub onboarding code:
.\qar-vulkan-source.exe --code 12345678 --host 192.168.1.20
```

- `--code` without a value asks for the code on stdin. `--host` defaults to this PC.
- QAROS must still be installed on this PC: the app loads its runtime.
- A code is single-use. Every start needs a new one; nothing is persisted.

## Options

| Option / variable | Effect |
|---|---|
| `--code [<code>]` | join with an onboarding code instead of as a local app |
| `--host <hub-host>` | hub host name or IP for `--code` |
| `--runtime <path>` | load this `qar-streaming-c.dll` instead of the installed one (QAROS development builds) |
| `QAR_GPU_ADAPTER_ID` | lowercase hex LUID (16 digits) or UUID (32 digits) of the GPU. Absent or empty: first supported NVIDIA GPU. No match: the app stops with an error. The console names the GPU used. |

```powershell
$env:QAR_GPU_ADAPTER_ID = "a1b2000000000000"; .\qar-vulkan-source.exe
```

## Started by QAROS

- QAROS ships this app next to the Visualizer, which lists it as the **Vulkan cube** tile in Source Applications (Debug UI mode). Started there it runs with no arguments, joins as a local app under the peer id the hub planned for it, and gets `QAR_GPU_ADAPTER_ID` set to the target's GPU.
- **Stop** closes the app's stdin pipe. The app treats a closed stdin pipe as a stop request and shuts down cleanly. Started from a console, only Ctrl+C stops it.
- A hub configured to hand API apps a code on stdin needs the argument `--code`: the app then reads the code from the first stdin line.
- Contract: Developer Guide, *Onboarding and Sessions*, "Launched by QAROS".

## Notes

- Target discovery uses `qar_render_sender_subscribe_requests()`. Each requesting target gets an independent sender and stereo camera, including viewers arriving after rendering starts.
- Known gaps: [01-improvements.md](01-improvements.md).
