# Vulkan cube source

Status: Implemented, end-to-end multi-headset validation pending.

- Source: `qar-streaming-c/examples/vulkan_cube/`.
- Optional `BUILD_VULKAN_EXAMPLE` CMake target, Windows Vulkan SDK and glm.
- Target `qar-vulkan-source`, also built from Quaternar with `QAR_BUILD_VULKAN_SRC_APP` and included in the launcher's copy/install set.
- Compile and embed shaders into the executable. Loads the installed runtime (`qar_library_load(NULL)`); `--runtime <dll>` loads a development build.
- Default: joins as a local app (`QarOnboardLocalAppExt`, GitHub #195/#223), no code; a plain-words message per status (1802, 1804-1807, not installed); an approval timeout offers a retry at an interactive console. `--code [<code>]` (+ `--host`) keeps code onboarding for a hub on another PC. Session GPU pinned to the Vulkan device's LUID.
- Onboarding is cancellable: Ctrl+C or a closed stdin pipe cancels the approval wait. A closed stdin pipe is a stop request (launcher Stop).
- `QAR_GPU_ADAPTER_ID` (lowercase hex LUID or UUID) selects the GPU; absent or empty keeps the first supported NVIDIA GPU, no match fails with an error. QAROS sets it when it launches the app; on Windows it writes no code (local enrollment). The console names the GPU used.
- Use existing `qar_render_sender_subscribe_requests()` for target discovery; add no C API.
- One independently polled sender and stereo camera per requesting target, all drawing the same colored cube and real depth.
- Protect callback-to-render-thread ID handoff, deduplicate requests and retain callback state through teardown.
- Publish actual final image layouts and external queue ownership; signal each sender's semaphore. Each camera drains its own submitted work before releasing resources.
- Documentation entry: `docs/website/docs/developer-guide/getting-started.mdx` (outside-user walkthrough), `rendering-streams.mdx`; usage and known gaps in the example folder.
- Validation: isolated standalone Debug build in `build/qaros-vulkan-debug`, dynamic debug SDK load/device startup and teardown from another working directory, and CTest GPU readback for two distinct cameras over two cycles passed with Vulkan synchronization validation enabled. SDK DLL is absent from the executable's import table.
