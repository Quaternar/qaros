# Vulkan cube source

Status: Implemented, end-to-end multi-headset validation pending.

- Source: `qar-streaming-c/examples/vulkan_cube/`.
- Optional `BUILD_VULKAN_EXAMPLE` CMake target, Windows Vulkan SDK and glm.
- Target `qar-vulkan-source`, also built from Quaternar with `QAR_BUILD_VULKAN_SRC_APP` and included in the launcher's copy/install set.
- Compile and embed shaders into the executable. Loads the installed runtime (`qar_library_load(NULL)`), else the `qar-streaming-c.dll` beside its executable (an uninstalled QAROS build); `--runtime <dll>` loads a given build. The main build copies the dynamic SDK and transfer-plugin dependencies beside the app.
- Default: joins as a local app (`QarOnboardLocalAppExt`, GitHub #195/#223), no code; a plain-words message per status (1802, 1805-1808, not installed); an approval timeout offers a retry at an interactive console. `--code [<code>]` (+ `--host host:port`) keeps code onboarding for a hub on another PC; a code line starting with `{` is a full launch invite (`qar_onboarding_invite_deserialize` + `QarOnboardInviteExt`).
- Onboarding is cancellable: Ctrl+C or a closed stdin pipe cancels the approval wait. A closed stdin pipe is a stop request (launcher Stop).
- GPU: onboard without chaining a `QarGraphicsDeviceId`; the SDK creates the session on the launch invite's GPU, else on the library's pick (a local-app join always takes the library's pick until QAROS selects one, Quaternar #231). Read it with `qar_session_get_graphics_device_id` and create the Vulkan device on the matching physical device; no match fails with an error. No environment variable. The console names the GPU used.
- Find targets in the peer list: `qar_peer_subscribe_updates()` and `qar_query_peer_specs()`, keeping running peers for which `qar_peer_spec_is_target_app()` is true. Render requests cannot discover targets: a target requests content only once a sender attached it to the cube's app volume.
- One independently polled sender and stereo camera per target app, all drawing the same colored cube and real depth.
- Protect callback-to-render-thread ID handoff, deduplicate requests and retain callback state through teardown.
- Publish actual final image layouts and external queue ownership; signal each sender's semaphore. Each camera drains its own submitted work before releasing resources.
- Documentation entry: `docs/website/docs/developer-guide/getting-started.mdx` (outside-user walkthrough), `rendering-streams.mdx`; usage and known gaps in the example folder.
- Validation: isolated standalone Debug build in `build/qaros-vulkan-debug`, dynamic debug SDK load/device startup and teardown from another working directory, and CTest GPU readback for two distinct cameras over two cycles passed with Vulkan synchronization validation enabled. SDK DLL is absent from the executable's import table.
