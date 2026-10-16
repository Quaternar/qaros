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

## Run

```powershell
.\build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe
```

1. The app loads the runtime with `qar_library_load(NULL)`, which finds it on its own: the installed QAROS, else the runtime beside the executable, else one on `PATH` (Developer Guide, *Authoring a language binding*). It then joins the QAROS on this PC as a local app (`QarOnboardLocalAppExt`). The app takes no arguments and looks for no file itself.
2. First run: QAROS asks **"Allow qar-vulkan-source.exe to join QAROS?"**, in the Visualizer or as a tray window. Click **Allow** within 120 s. Meanwhile the console shows `Waiting for approval in QAROS hub`.
3. `Connected. Waiting for rendering targets.` Open the cube's app volume on a viewer (Visualizer or headset). Each additional viewer gets its own stream and camera.
4. Later runs join without a prompt while the executable is unchanged.

- Unsigned build: every rebuild asks again; a copy elsewhere is refused until its approval is revoked. See *Onboarding and Sessions*, "The trust record".
- Ctrl+C stops the app. Ctrl+C while it waits for approval cancels the wait.

| Console message | Status | What to do |
|---|---|---|
| `Could not load QAROS. Install QAROS on this PC...` | library load failed | install QAROS |
| `QAROS is not running on this PC...` | 1802 | start QAROS from the Start menu, run again |
| `QAROS did not allow this app to join...` | 1805 | someone clicked Deny or revoked it: run again to be asked again |
| `Nobody answered the approval prompt in QAROS in time...` | 1806 | run again, and answer the prompt |
| `The program answering on this PC is not the installed QAROS...` | 1807 | reinstall QAROS; the app does not retry |
| `This app no longer matches what QAROS approved...` | 1808 | revoke it in the Visualizer (Connection panel, Approved apps), run again |

## GPU

- The app joins first and then creates its Vulkan device on the session's GPU (`qar_session_get_graphics_device_id`), the GPU the library picks. A session GPU that matches no supported Vulkan device stops the app with an error. The console names the GPU it renders on. The app reads no environment variable.

## Started by QAROS

- QAROS ships this app next to the Visualizer, which lists it as the **Vulkan cube** tile in Source Applications (Developer UI mode and up). Started there it runs with no arguments and joins as a local app under the peer id the hub planned for it.
- **Stop** closes the app's stdin pipe. The app treats a closed stdin pipe as a stop request and shuts down cleanly. Started from a console, only Ctrl+C stops it.
- Contract: Developer Guide, *Onboarding and Sessions*, "Launched by QAROS".

## Test your own app in the visualizer

Any executable built on the QAROS C or C# API can be launched from the visualizer the same way as this cube, once it follows the launch contract (joins as a local app, own app volume, targets from the peer list, stop on end of stdin; Developer Guide, *Onboarding and Sessions*, "Launched by QAROS").

1. Start QAROS (Start menu, or `bin\qaros-hub-service.exe`); it opens the visualizer.
2. Settings → **UI mode** → **Developer** (no password). QAROS API apps show from Developer mode up.
3. Source Applications → **+ Add source app** → **QAROS API app**.
4. **Executable**: path to your `.exe` (type or **Browse**). **Arguments**: one per line, optional. **Name** defaults to the executable name.
5. **Add app** saves a preset tile; it does not launch. Click the tile → **Launch**.
6. Your app appears under **Running**; open its content on a target viewer. **Stop** closes its stdin.

- Preset is kept by the Hub on this PC and survives restarts; the executable path must stay reachable.
- An app exiting within 10 s is restarted with backoff and reported as an error notification; its stdout/stderr are in the Hub's log folder.
- Without registering: run the app from a console, as in [Run](#run-local-app-the-default).

Details: Operator Guide, *Source Applications*, "Adding your own app".

## Notes

- The cube renders for every running target app in the peer list (`qar_peer_subscribe_updates()` plus `qar_peer_spec_is_target_app()`): which targets to serve is a source app's own choice, and the cube serves them all. Each target gets an independent sender and stereo camera, including viewers arriving after rendering starts. The sender attaches its target to the cube's app volume, which is what makes the target ask for the content.
- Known gaps: [01-improvements.md](01-improvements.md).
