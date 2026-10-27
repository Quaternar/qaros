# Vulkan cube source

A Windows source app that renders a rotating, colored cube in stereo color and depth. Each viewer gets its own camera of the same cube.

It is the reference for a source app on the Hub machine: it loads the installed QAROS runtime and joins as a **local app**, with no code. Step-by-step guide: Developer Guide, *Getting Started*. Walkthrough of the code: Developer Guide, *Tutorials*, *C*, *Vulkan Rendering*.

## Requirements

- Windows 10/11 x64 with QAROS installed (Operator Guide, *Installation*).
- An NVIDIA GPU whose Vulkan driver offers `VK_KHR_external_memory_win32` and `VK_KHR_external_semaphore_win32`.
- Visual Studio 2022 Build Tools ("Desktop development with C++" with its recommended components, which bring CMake and Ninja), Git. No Vulkan SDK: the Vulkan headers, loader and shader compiler come from vcpkg.

## Build

From the `qaros` repository root, in a **Developer PowerShell for VS 2022**:

```powershell
cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
cmake --build --preset x64-windows-release --target qar-vulkan-source
```

- The first configure clones vcpkg into `build/` and builds the dependencies; it takes a while.
- Output: `build/x64-windows/qar-streaming-c/examples/vulkan_cube/Release/qar-vulkan-source.exe` (`Debug/` with `x64-windows-debug`). Shaders are embedded.
- `-DBUILD_VULKAN_EXAMPLE_TESTS=ON` adds `vulkan_cube_gpu_test`, which renders two cameras into images of its own and reads them back. It needs a GPU, not QAROS.

## Run

```powershell
.\build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe
```

1. The app loads the runtime with `qar_library_load(NULL)`, the default lookup: a `qar-streaming-c.dll` beside the executable, else the runtime of the QAROS instance `QAR_INSTANCE` names, else the installed QAROS, else one on `PATH` (Developer Guide, *Authoring a Language Binding*). Do not ship a copy of the runtime beside the app: it would win over the installed one. It then joins the Hub on this machine as a local app (`QarOnboardLocalAppExt`) with the app tag `vulkan-cube.examples.qaros` (`QarOnboardLocalAppTagExt`). When the Hub does not run, the join starts it. The app takes no arguments and looks for no file itself.
2. The console shows `Joining the QAROS on this machine...`, then `Waiting for approval in QAROS hub`.
3. First run: QAROS asks **"Allow qar-vulkan-source to join QAROS?"** in a window in front of all others (an open Visualizer asks the same; answer either). Publisher reads "Unknown publisher": your build is not signed. Click **Allow** within 120 s.
4. The console shows `Joined QAROS as peer <id>`, then `Connected. Waiting for rendering targets.`
5. Open a viewer: the Visualizer opens when QAROS starts (if you closed it, left-click the QAROS tray icon), or join with a headset. The cube appears in its own app volume. Each additional viewer gets its own stream and camera.
6. Later runs join without a prompt while the executable is unchanged, under the same peer id (the same `Joined QAROS as peer` line): the Hub remembers it per approval and app tag.

- Unsigned build: every rebuild asks again; a copy elsewhere is refused until its approval is revoked. See *Onboarding and Sessions*, "The trust record".
- Ctrl+C stops the app. Ctrl+C while it waits for approval cancels the wait.
- QAROS also installs a copy, `C:\Program Files\Quaternar\QAROS\bin\qar-vulkan-source.exe`: run it to see the flow before you build. It is approved on its own.

| Console message | Status | Exit code | What to do |
|---|---|---|---|
| `Could not load QAROS. Install QAROS on this machine...` | library load failed | 2 | install QAROS, or fix or unset `QAR_INSTANCE` |
| `Could not start or reach QAROS on this machine...` | 1802 | 3 | start QAROS from the Start menu, run again |
| `QAROS did not allow this app to join...` | 1805 | 4 | someone clicked Deny or revoked it: run again to be asked again. The `Details:` line names any other reason, such as QAROS running for another Windows user |
| `Nobody answered the approval prompt in QAROS in time...` | 1806 | 5 | run again, and answer the prompt |
| `The program answering on this machine is not the installed QAROS...` | 1807 | 6 | reinstall QAROS; the app does not retry |
| `This app no longer matches what QAROS approved...` | 1808 | 7 | revoke it in the Visualizer (Source Applications, Approved apps), run again |

Exit code 0 follows a stop. Exit code 1 is a bad argument, a failed `qar_library_init` or `qar_runtime_create`, or any other failure (the GPU, the app volume, the peer list).

## GPU

The app joins first, then creates its Vulkan device on the session's GPU (`qar_session_get_graphics_device_id`): the GPU of the target named in the launch invite when QAROS started the app, else the one the library picked. A session GPU that matches no supported Vulkan device stops the app with an error. The console names the GPU it renders on. The app reads no environment variable.

## Started by QAROS

- QAROS ships this app next to the Visualizer, which lists it as the **Vulkan cube** tile in Source Applications (Developer UI mode and up). Started there it runs with no arguments and joins as a local app under the peer id the Hub planned for it.
- **Stop** closes the app's stdin pipe. The app treats a closed stdin pipe as a stop request and shuts down cleanly. Started from a console, only Ctrl+C stops it.
- Contract: Developer Guide, *Onboarding and Sessions*, "Launched by QAROS".

## Test your own app in the Visualizer

Any executable built on a QAROS API can be launched from the Visualizer the same way as this cube, once it follows the launch contract (joins as a local app, own app volume, targets from the peer list, stop on end of stdin; Developer Guide, *Onboarding and Sessions*, "Launched by QAROS").

1. Open the Visualizer: left-click the QAROS tray icon (start **QAROS** from the Start menu if there is none).
2. Settings → **UI mode** → **Developer** (no password). QAROS API apps show from Developer mode up.
3. Source Applications → **+ Add source app** → **QAROS API app**.
4. **Executable**: path to your `.exe` (type or **Browse**). **Arguments**: one per line, optional. **Name** defaults to the executable name.
5. **Add app** saves a preset tile; it does not launch. Click the tile → **Launch**.
6. Your app appears under **Running**; open its content on a target viewer. **Stop** closes its stdin.

- The Hub on this machine keeps the preset across restarts; the executable path must stay reachable.
- An app exiting within 10 s is restarted with backoff and reported as an error notification; its stdout/stderr are in the Hub's log folder.
- Without registering: run the app from a console, as in [Run](#run).

Details: Operator Guide, *Source Applications*, "Adding your own app".

## Notes

- The cube renders for every running target app in the peer list (`qar_peer_subscribe_updates()` plus `qar_peer_spec_get_role()` == `QAR_PEER_ROLE_TARGET`): which targets to serve is a source app's own choice, and the cube serves them all. Each target gets an independent sender and stereo camera on a thread of its own, including viewers arriving after rendering starts. The sender attaches its target to the cube's app volume, which is what makes the target ask for the content.
- Each frame keeps its frame info from `qar_render_sender_begin_frame()` until `qar_render_sender_show_frame()`. A begun frame is always shown, as the other C samples do; when rendering fails, the cube first discards it by signaling its semaphore. Only a frame whose record could not be read is not shown: the show would wait for a semaphore nothing signals. A stop request waits for the frame.
- Known gaps: [01-improvements.md](01-improvements.md).
