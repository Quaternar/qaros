# Vulkan cube source

A standalone Windows source app rendering a rotating, colored cube with stereo color and depth. Each viewer gets its own camera of the same cube.

## Run

1. Have the QAROS service running on this computer and obtain an onboarding code from it.
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

- Started from QAROS (Visualizer, Source Applications), the app gets its onboarding code on stdin and `QAR_GPU_ADAPTER_ID` set to the target's GPU, so no prompt needs an answer.
- Press Ctrl+C to stop the source.
- Requires an NVIDIA GPU supporting Vulkan external memory and semaphores.

## Build

From the standalone `qaros` repository root, in a Developer PowerShell with CMake 3.29+, Ninja and Git available:

```powershell
cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
cmake --build --preset x64-windows-debug --target qar-vulkan-source
```

The repository bootstraps vcpkg and installs the Vulkan and glm build dependencies. The executable is `build/x64-windows/qar-streaming-c/examples/vulkan_cube/Debug/qar-vulkan-source.exe`. Shaders are embedded in the executable.

Target discovery uses the existing `qar_render_sender_subscribe_requests()` API. Each requesting target gets an independent sender and stereo camera, including viewers arriving after rendering starts.

Known gaps: [01-improvements.md](01-improvements.md).
