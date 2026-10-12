# Vulkan cube source

Status: Implemented, end-to-end multi-headset validation pending.

- Source: `qar-streaming-c/examples/vulkan_cube/`.
- Optional `BUILD_VULKAN_EXAMPLE` CMake target, Windows Vulkan SDK and glm.
- Target `qar-vulkan-source`, also built from Quaternar with `QAR_BUILD_VULKAN_SRC_APP` and included in the launcher's copy/install set.
- Compile and embed shaders into the executable; default SDK path is beside the executable, independent of working directory. The main build copies the dynamic SDK and transfer-plugin dependencies beside the app.
- Runtime-load the C SDK, read onboarding code or invite from stdin, onboard, then create the Vulkan device on the session's GPU.
- GPU: onboard without chaining a `QarGraphicsDeviceId`; the SDK creates the session on the launch invite's GPU (Quaternar hub ADR-0274), else on the library's pick. Read it with `qar_session_get_graphics_device_id` and create the Vulkan device on the matching physical device; no match fails with an error. No environment variable. The console names the GPU used.
- Find targets in the peer list: `qar_peer_subscribe_updates()` and `qar_query_peer_specs()`, keeping running peers for which `qar_peer_spec_is_target_app()` is true. Render requests cannot discover targets: a target requests content only once a sender attached it to the cube's app volume.
- One independently polled sender and stereo camera per target app, all drawing the same colored cube and real depth.
- Protect callback-to-render-thread ID handoff, deduplicate requests and retain callback state through teardown.
- Publish actual final image layouts and external queue ownership; signal each sender's semaphore. Each camera drains its own submitted work before releasing resources.
- Documentation entry: `docs/website/docs/developer-guide/rendering-streams.mdx`; usage and known gaps in the example folder.
- Validation: isolated standalone Debug build in `build/qaros-vulkan-debug`, dynamic debug SDK load/device startup and teardown from another working directory, and CTest GPU readback for two distinct cameras over two cycles passed with Vulkan synchronization validation enabled. SDK DLL is absent from the executable's import table.
