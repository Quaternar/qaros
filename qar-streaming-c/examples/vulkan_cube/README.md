# Vulkan cube source

A standalone Windows source app rendering a rotating, colored cube with stereo color and depth. Each viewer gets its own camera of the same cube.

It is the reference for a source app on the QAROS PC: it loads the installed QAROS runtime and joins as a **local app**, with no code. Step-by-step guide: Developer Guide, *Getting Started*.

## Requirements

- Windows 10/11 x64 with QAROS installed (Operator Guide, *Installation*).
- NVIDIA GPU supporting Vulkan external memory and semaphores.
- Visual Studio 2022 Build Tools ("Desktop development with C++" with its recommended components, which bring CMake and Ninja), Git. No Vulkan SDK: the Vulkan headers, loader and shader compiler come from vcpkg.

## Build

From the `qaros` repository root, in a **Developer PowerShell for VS 2022**:

```powershell
cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
cmake --build --preset x64-windows-release --target qar-vulkan-source
```

- The first configure clones vcpkg into `build/` and builds the dependencies; it takes a while.
- Output: `build/x64-windows/qar-streaming-c/examples/vulkan_cube/Release/qar-vulkan-source.exe` (`Debug/` with `x64-windows-debug`). Shaders are embedded.

## Run

```powershell
.\build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe
```

1. The app loads the runtime with `qar_library_load(NULL)`, which finds it on its own: a `qar-streaming-c.dll` beside the executable, else the folder `QAROS_RUNTIME_DIR` names, else the installed QAROS, else one on `PATH` (Developer Guide, *Authoring a Language Binding*). Do not ship a copy of the runtime beside the app: it would win over the installed one. It then joins the QAROS on this PC as a local app (`QarOnboardLocalAppExt`) with the app tag `vulkan-cube.examples.qaros` (`QarOnboardLocalAppTagExt`). The app takes no arguments and looks for no file itself.
2. The console shows `Joining the QAROS on this PC...`, then `Waiting for approval in QAROS hub`.
3. First run: QAROS asks **"Allow qar-vulkan-source to join QAROS?"** in a window in front of all others (an open Visualizer asks the same; answer either). Publisher reads "Unknown publisher": your build is not signed. Click **Allow** within 120 s.
4. The console shows `Joined QAROS as peer <id>`, the GPU (`Rendering on ...`), then `Connected. Waiting for rendering targets.`
5. Open a viewer: the Visualizer opens when QAROS starts (if you closed it, left-click the QAROS tray icon), or join with a headset. The console shows `Rendering for target ...` per viewer, and the cube appears in its own app volume. Each additional viewer gets its own stream and camera.
6. Later runs join without a prompt while the executable is unchanged, under the same peer id (the same `Joined QAROS as peer` line): the hub remembers it per approval and app tag.

- Unsigned build: every rebuild asks again; a copy elsewhere is refused until its approval is revoked. See *Onboarding and Sessions*, "The trust record".
- Ctrl+C stops the app. Ctrl+C while it waits for approval cancels the wait.
- QAROS also installs a copy, `C:\Program Files\Quaternar\QAROS\bin\qar-vulkan-source.exe`: run it to see the flow before you build. It is approved on its own.

| Console message | Status | What to do |
|---|---|---|
| `Could not load QAROS. Install QAROS on this PC...` | library load failed | install QAROS |
| `QAROS is not running on this PC...` | 1802 | start QAROS from the Start menu, run again. With QAROS installed but its Hub not running, the message takes up to a minute: QAROS waits for a Hub to start |
| `QAROS did not allow this app to join...` | 1805 | someone clicked Deny or revoked it: run again to be asked again. The `Details:` line names any other reason, such as QAROS running for another Windows user |
| `Nobody answered the approval prompt in QAROS in time...` | 1806 | run again, and answer the prompt |
| `The program answering on this PC is not the installed QAROS...` | 1807 | reinstall QAROS; the app does not retry |
| `This app no longer matches what QAROS approved...` | 1808 | revoke it in the Visualizer (Source Applications, Approved apps), run again |

## GPU

- The app joins first and then creates its Vulkan device on the session's GPU (`qar_session_get_graphics_device_id`), the GPU the library picks. A session GPU that matches no supported Vulkan device stops the app with an error. The console names the GPU it renders on. The app reads no environment variable.

## Started by QAROS

- QAROS ships this app next to the Visualizer, which lists it as the **Vulkan cube** tile in Source Applications (Developer UI mode and up). Started there it runs with no arguments and joins as a local app under the peer id the hub planned for it.
- **Stop** closes the app's stdin pipe. The app treats a closed stdin pipe as a stop request and shuts down cleanly. Started from a console, only Ctrl+C stops it.
- Contract: Developer Guide, *Onboarding and Sessions*, "Launched by QAROS".

## Test your own app in the visualizer

Any executable built on the QAROS C or C# API can be launched from the visualizer the same way as this cube, once it follows the launch contract (joins as a local app, own app volume, targets from the peer list, stop on end of stdin; Developer Guide, *Onboarding and Sessions*, "Launched by QAROS").

1. Open the Visualizer: left-click the QAROS tray icon (start **QAROS** from the Start menu if there is none).
2. Settings → **UI mode** → **Developer** (no password). QAROS API apps show from Developer mode up.
3. Source Applications → **+ Add source app** → **QAROS API app**.
4. **Executable**: path to your `.exe` (type or **Browse**). **Arguments**: one per line, optional. **Name** defaults to the executable name.
5. **Add app** saves a preset tile; it does not launch. Click the tile → **Launch**.
6. Your app appears under **Running**; open its content on a target viewer. **Stop** closes its stdin.

- Preset is kept by the Hub on this PC and survives restarts; the executable path must stay reachable.
- An app exiting within 10 s is restarted with backoff and reported as an error notification; its stdout/stderr are in the Hub's log folder.
- Without registering: run the app from a console, as in [Run](#run).

Details: Operator Guide, *Source Applications*, "Adding your own app".

## Notes

- The cube renders for every running target app in the peer list (`qar_peer_subscribe_updates()` plus `qar_peer_spec_get_role()` == `QAR_PEER_ROLE_TARGET`): which targets to serve is a source app's own choice, and the cube serves them all. Each target gets an independent sender and stereo camera, including viewers arriving after rendering starts. The sender attaches its target to the cube's app volume, which is what makes the target ask for the content.
- Known gaps: [01-improvements.md](01-improvements.md).
