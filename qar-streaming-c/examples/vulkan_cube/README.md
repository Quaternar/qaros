# Vulkan cube source

A standalone Windows source app rendering a rotating, colored cube with stereo color and depth. Each viewer gets its own camera of the same cube.

## Run

1. Start the QAROS Hub on this computer with `bin\qaros-hub-service.exe` and take an onboarding code from the visualizer's onboarding screen.
2. Start the source from its application folder:

   ```powershell
   .\qar-vulkan-source.exe
   ```

3. At `Hub onboarding code:`, enter the code and press Enter. The app connects to the local hub on localhost by default.
4. Open the application's content on a target viewer. Additional viewers automatically get their own rendering streams and cameras.

- Enter an onboarding code each time the app starts.
- `QAR_GPU_ADAPTER_ID` selects the GPU: the lowercase hex LUID (16 digits) or UUID (32 digits) of the adapter. Absent or empty, the first supported NVIDIA GPU is used. A value that matches no supported GPU stops the app with an error. The console names the GPU it renders on.

  ```powershell
  $env:QAR_GPU_ADAPTER_ID = "a1b2000000000000"; .\qar-vulkan-source.exe
  ```

- The QAROS runtime ships this app next to the visualizer, which lists it as the **Vulkan cube** tile in Source Applications (Developer UI mode and up). Started from there, the app gets its onboarding code on stdin and `QAR_GPU_ADAPTER_ID` set to the target's GPU, so no prompt needs an answer. The contract is in the Developer Guide, *Onboarding and Sessions*, "Launched by QAROS".
- Press Ctrl+C to stop the source.
- Requires an NVIDIA GPU supporting Vulkan external memory and semaphores.

## Test your own app in the visualizer

Any executable built on the QAROS C or C# API can be launched from the visualizer the same way as this cube, once it follows the launch contract (code on stdin, `QAR_GPU_ADAPTER_ID`, own app volume, stop on end of stdin; Developer Guide, *Onboarding and Sessions*, "Launched by QAROS").

1. Start the QAROS Hub with `bin\qaros-hub-service.exe`; it opens the visualizer.
2. Settings → **UI mode** → **Developer** (no password). QAROS API apps show from Developer mode up.
3. Source Applications → **+ Add source app** → **QAROS API app**.
4. **Executable**: path to your `.exe` (type or **Browse**). **Arguments**: one per line, optional. **Name** defaults to the executable name.
5. **Add app** saves a preset tile; it does not launch. Click the tile → **Launch**.
6. Your app appears under **Running**; open its content on a target viewer. **Stop** closes its stdin.

- Preset is kept by the Hub on this PC and survives restarts; the executable path must stay reachable.
- An app exiting within 10 s is restarted with backoff and reported as an error notification; its stdout/stderr are in the Hub's log folder.
- Without registering: run the app from a console and type an onboarding code from the Hub, as in [Run](#run).

Details: Operator Guide, *Source Applications*, "Adding your own app".

## Build

From the standalone `qaros` repository root, in a Developer PowerShell with CMake 3.29+, Ninja and Git available:

```powershell
cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
cmake --build --preset x64-windows-debug --target qar-vulkan-source
```

The repository bootstraps vcpkg and installs the Vulkan and glm build dependencies. The executable is `build/x64-windows/qar-streaming-c/examples/vulkan_cube/Debug/qar-vulkan-source.exe`. Shaders are embedded in the executable.

Target discovery uses the existing `qar_render_sender_subscribe_requests()` API. Each requesting target gets an independent sender and stereo camera, including viewers arriving after rendering starts.

Known gaps: [01-improvements.md](01-improvements.md).
