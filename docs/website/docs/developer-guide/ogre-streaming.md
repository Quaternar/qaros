---
sidebar_position: 8.5
title: Streaming an Ogre Scene to QAROS
description: How to stream an Ogre 1.12 application into the room with the qar_ogre_adapter library — what it does for each peer, the three ways to use it, the conventions your scene follows and where the library stops.
---

# Streaming an Ogre Scene to QAROS

If your C++ application renders with Ogre 1.12, the library **`qar_ogre_adapter`** streams its scene to QAROS. For every peer that asks for a frame, it renders the scene for each eye into a color and a depth texture and hands the textures to the C API, so your application needs no call of the C API for the stream. This page describes how a frame travels, the three ways to use the library, the conventions your scene follows and where the library stops. The installation steps, the API reference and the troubleshooting tables stay in the repository [github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre); this site links to them and does not copy them.

:::info Which SDK generation
This page describes the library `qar_ogre_adapter` as it was built and verified against the QAROS SDK drop of 2026-09-28. The C API that this site documents, release 0.9.0, is a later generation, and the library has not been rebuilt against it. Release 0.9.0 replaces the session start-up, in which the library creates a session and invites a Visualizer, by onboarding to the Hub. The sections that depend on the session start-up say so, and [Session start-up](#session-start-up) collects them; for the calls of 0.9.0, start with [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions). The document `docs/SDK_COMPATIBILITY.md` of the repository lists the supported SDK drops, how to identify the drop you have and what differs in the later generation.
:::

## What the library does for you

For each peer that asks for a frame, `qar_ogre_adapter` takes the pose and the field of view of each eye from QAROS, renders your Ogre scene for each eye into a **color** texture and a **depth** texture, and copies the textures into the textures that QAROS hands out. It renders the peers that are due in one batch under a **render budget**. Your application keeps its Ogre scene and its render loop: the library creates its eye cameras in your scene manager and its render textures on the first frame of a stream, and it never creates or destroys the scene manager.

Three modules of C++17 static libraries make up the layer. Link only the ones you need. Nothing in them depends on Qt, ROS or RViz; the ROS 2 Humble distribution is only the source of Ogre.

| Module | What it holds | Calls the C API |
|---|---|---|
| `qar_ar_core` | The core that the other two build on: the panel and pointer types, the pointing ray, the spatial frames, the pick model, an opt-in timing log and the one wrapper of the vendored C header. | No |
| `qar_ogre_streaming` | The Ogre side: the stream manager, the batched render scheduler, the render budget, the eye render targets, the pose conversion, the scene lighting and the diagnostics. It reaches QAROS only through the interfaces `IQarRenderSenderFactory` and `IQarRenderSenderAdapter`. | No |
| `qar_qar_adapter` | The adapter of the C API, in five components: `capi`, `render`, `panels`, `url` and `gestures`. Streaming a 3D scene needs `render`, which holds the session factory and the render sender of every peer. | Yes, the only module that does |

[Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) says when to use which of the three libraries and what to install.

## How a frame travels

The render loop of your application calls `OgreQarStreamingManager::render_tick()` on its render thread. One call renders the streams that are due, in one batch:

1. **A peer asks for a frame.** For every stream that the scheduler admits on this tick, the library polls `qar_render_sender_try_begin_frame`, which never blocks. While QAROS has no render request from the viewer, the stream skips the tick and records it as `SenderNotReady`. That is the normal state while the viewer paces the stream below your tick rate.
2. **The view arrives.** When a frame is wanted, the library reads the pose and the field of view of the two eyes from the frame information and pairs them with the near and far distances of its configuration. The stream turns them into a camera placement and an off-axis projection for each eye.
3. **The library renders both eyes.** It renders your scene for each eye into a color and a depth render texture. The scheduler takes your render lock, if you pass one, once for the whole batch, so every peer samples the same snapshot of the scene.
4. **The library hands the textures to QAROS.** It asks QAROS for the textures of the frame with `qar_render_sender_frame_opengl`, copies the rendered textures into them with a framebuffer blit, waits with `glFinish` until the copy is complete and shows the frame with `qar_render_sender_show_frame`.
5. **The next wait starts at once.** By default the library starts the wait for the next frame right after it shows a frame. The wait runs in the gap between two ticks, so a frame costs one tick.

Every peer costs a color and a depth render of your scene for each eye in every frame, so on every tick the scheduler decides which streams render. A stream is due when its next-due time has passed. The scheduler orders the due streams, most overdue first, admits a stream under floor protection unconditionally and admits the others while the sum of their estimated costs stays within the batch budget and the cap. A due stream that does not fit is shed for this tick and counted as a skipped frame with the reason `BudgetShed`. These settings control the decision:

| Setting | Default | Meaning |
|---|---|---|
| `StereoStreamConfig::max_fps` | `72` | The target rate of one stream. A value of `0` is refused. |
| `RenderBudgetConfig::max_batch_milliseconds` | `16.0` | The soft time budget of one batch for all peers. `0` means unlimited. |
| `RenderBudgetConfig::max_streams_per_tick` | `0` | A cap on the streams rendered in one tick; a stream under floor protection renders beyond it. `0` means unlimited. |
| `RenderBudgetConfig::floor_fps` | `30` | A due stream that has not rendered within 1 / `floor_fps` seconds is always admitted. `0` switches the protection off. |
| `RenderBudgetConfig::default_stream_cost_ms` | `2.0` | The cost assumed for a stream that has never rendered. The measured cost replaces it after the first render. |
| `QarCApiSenderFactory::Config::max_peers` | `8` | The ceiling on concurrently streamed peers. A further peer is refused with a logged warning. |

## Three ways in

Take them in this order:

| Way in | What it does | Needs |
|---|---|---|
| The headless sample | Renders the scene for synthetic peers and writes their stereo images. | Ogre only, no SDK |
| The live example | Streams the same scene to a QAROS Visualizer. | The SDK and a local desktop session |
| Your own application | Replaces the scene of the example with yours. | Ogre to build, the SDK to connect |

### The headless sample

The program `qar_ogre_standalone_streaming_sample` brings up Ogre with a hidden OpenGL window and renders the scene for synthetic stereo peers that live in the same process, through the same manager as the live example. It loads no QAROS library and sends nothing anywhere. Run it with `scripts\run_ogre_sample.ps1`: by default it renders `120` ticks for three peers at `1280` by `720` pixels for each eye and writes the stereo images of each peer. For each peer it checks that both eyes contain content, that the depth is present and not uniform, that the two eyes differ, and that the nearer part of the scene shifts more between the eyes than the farther part. It exits with `0` when every check passes, with `1` when a check fails and with `2` for a wrong argument or when Ogre cannot start.

### The live example

The program `qar_ogre_live_stream_example` streams the same scene to a QAROS Visualizer. Its option `--sdk-bin` is required and names the `bin` folder of the SDK.

- **With `--connect-only`**, the example opens a QAROS session, prints the identifier of the app volume that it created and exits. It opens no window and invites nobody (session start-up).
- **Without it**, the example brings up Ogre with a hidden window, connects, invites a Visualizer into the session (session start-up) and renders the scene for the Visualizer for the number of seconds that `--seconds` gives, `60` by default. QAROS starts the Visualizer itself from the SDK folder, and the Visualizer opens its own window. The example prints a line when the stream opens and when it closes.

Set `QAR_DEPTH_ENCODING=ndc` and `QAR_POSE_WORLD=zup` in the shell before you start the example. As far as the authors of `qar_ogre_adapter` observed, the Visualizer needs the first; the scene of the sample is Z-up, so it needs the second. [Conventions your renderer must follow](#conventions-your-renderer-must-follow) says why. The library reads each variable once for each process.

The authors of `qar_ogre_adapter` have not run this example in a session with a Visualizer. The library ran in live sessions of the RViz2 plugin, which streams the scene of RViz2 to a Visualizer through it.

### Your own application

Your application replaces only the scene of the example, which takes its scene from the sample.

**The objects.** Link the target `qar_ogre_adapter::render`; it brings `qar_ogre_streaming`, `qar_ar_core`, `capi` and Ogre. A consumer of the installed package asks `find_package` for the components that it uses. Then create:

- **The manager.** `OgreQarStreamingManager` is the entry point. It owns the scheduler and one stereo stream for each peer. Its configuration, `OgreQarStreamingConfig`, needs a `scene_manager` (your Ogre `SceneManager`), a `render_thread_executor` and a `qar_sender_factory`; the constructor throws `std::invalid_argument` when one of them is missing. A `diagnostics` sink, a `render_budget`, a `render_lock`, a `clock`, a `fence_hook` and a `scene_lighting` rig are optional.
- **The executor.** `IRenderThreadExecutor` is how the library asks whether the calling thread is the render thread (`is_render_thread`) and posts work to it (`post_to_render_thread`). The example renders on its main thread, so its executor reports the main thread.
- **The factory.** `QarCApiSenderFactory` is the sender factory of QAROS (session start-up). The manager asks it for a sender when it creates the stream of a peer. Its configuration needs the copy hook `make_gl_copy_image_hook()`.

**The frame loop.** Each pass of your render loop runs on the render thread, and the code below follows the source of the example. `capture_gl_context()` comes first, because the factory creates a sender only after it has latched the OpenGL context that is current on the thread. The sender of a peer is created asynchronously, so the first attempts of `create_stream` fail with `InvalidState`; `on_render_request` queues the peer again, so that the next pass retries it. `render_tick()` renders one batch of the streams that are due; called on another thread, it returns `RenderThreadRequired` without touching Ogre. For a peer whose stream QAROS closed, call `destroy_stream` and then `forget`, which frees the place of the peer under `max_peers`. The two blocks marked (session start-up) take the peers from the factory.

```cpp
qar_ogre_streaming::OgreQarStreamingConfig streaming;
streaming.scene_manager = scene_manager;              // your Ogre::SceneManager
streaming.render_thread_executor = executor;          // your IRenderThreadExecutor
streaming.qar_sender_factory = factory;               // (session start-up)
auto manager = std::make_unique<qar_ogre_streaming::OgreQarStreamingManager>(streaming);
streaming.qar_sender_factory.reset();                 // the manager holds the factory now
manager->set_streaming_active(true);

while (running) {
  factory->capture_gl_context();                      // latch the GL context of this thread

  // (session start-up) the peers that asked for a stream
  std::vector<QarPeerId> requested;
  factory->poll_new_peer_requests(requested);
  for (const QarPeerId & id : requested) {
    qar_ogre_streaming::StereoStreamConfig stream;
    stream.peer_id = id;
    const qar_ogre_streaming::Result created = manager->create_stream(stream);
    if (created.ok()) {
      manager->get_stream(id)->set_active(true);
    } else if (created.status != qar_ogre_streaming::Status::AlreadyExists) {
      factory->on_render_request(id);                 // the sender is still being created
    }
  }

  manager->render_tick();                             // one batch of the streams that are due

  // (session start-up) the peers whose stream QAROS closed
  std::vector<QarPeerId> closed;
  factory->poll_closed_peers(closed);
  for (const QarPeerId & id : closed) {
    manager->destroy_stream(id);
    factory->forget(id);
  }
}
```

The manager keeps its own reference to the factory, so the code clears the factory from its configuration object: no other reference may outlive the shutdown. Shut down in this order: the manager first, because the senders of its streams belong to the QAROS session; then the factory, which destroys the session, the runtime and the library; then Ogre.

**The threads.**

| Thread | What runs there | Rule |
|---|---|---|
| The render thread of your application | `render_tick`, the creation of the eye targets, `capture_gl_context`, `create_sender`, the begin-frame polling and the submit of each frame, `poll_new_peer_requests` and `poll_closed_peers` | It is the only thread of your application that touches Ogre and OpenGL. If you pass a render lock, the scheduler holds it around each batch. |
| A graphics thread of QAROS | It reads the textures of a sender with the WGL context that the factory created for that sender. | The factory supplies the context and its callbacks, and it never makes that context current on your render thread. |
| The callback threads of QAROS | The peer updates (session start-up) and the result of the asynchronous sender creation | A callback converts, queues and returns. It never blocks and never renders. |
| A worker thread of your application, optional | The construction of the factory (session start-up) | The constructor blocks for seconds, and the factory is not usable before `ready()`. |

**The configuration.** These are the fields that most hosts set. The configuration document of the repository lists every field with its default.

- `QarCApiSenderFactory::Config::runtime_binaries_folder_path`: the `bin` folder of the SDK. The library loads `qar-streaming-c.dll` from it unless `sdk_library_path` names the file.
- `copy_hook`: set it to `make_gl_copy_image_hook()`. QAROS owns the textures of a frame and nothing else writes them, so without the hook the library skips the copy and the Visualizer shows a black scene.
- `near_far`: the near and far clip distances in meters, reported with every frame. The default is `{0.1, 500.0}`. The far distance matters for the window-space depth encoding.
- `depth_scale`: reported with every frame as the depth scale. The default is `1.0`.
- `log_severity` and `enable_console_logging`: the log of the QAROS library. The defaults are trace and on. A trace log is large, so a host in normal operation sets `QAR_LOG_SEVERITY_WARN` and off, as the live example does.
- `StereoStreamConfig`, one for each peer: `peer_id`; `width` and `height` of each eye, `1024` by default; `max_fps`; `color_format`; `depth_format`, the format that you request; and `layout`, which accepts only separated textures.

**Lighting.** The library renders the lights of your scene for every peer. A host such as RViz keeps a headlight that follows the camera of the local operator, so the shading that a remote peer sees swings whenever the local operator orbits the view. The optional rig `QarSceneLighting`, passed in `scene_lighting`, lights the scene with lights of its own, world-fixed by default, while streaming is active and restores your lighting when streaming stops. Its configuration field `suppress_light_name` names a light of your scene, such as the headlight, that the rig hides while it is applied. No example of the repository sets a rig, so among the checks of the repository only the unit tests cover it.

## Conventions your renderer must follow

### World basis and app content space

- **The camera frames agree.** QAROS follows OpenXR in every 3D space: right-handed, `+Y` up and `-Z` forward, and a pose is a position vector plus an orientation quaternion stored x, y, z, w. An Ogre camera looks along its local `-Z` with `+Y` up as well. The library therefore copies a position unchanged and an orientation component for component, and it only reorders the quaternion to the w, x, y, z storage of Ogre.
- **View poses are in app content space.** The pose of each view is in the app content space of the app volume that the stream is keyed to, the space your application renders in. Use it as it comes: the library applies no part of the placement of the volume (its pose, its app pose or its app scale) to a view pose. [Coordinate Systems](/docs/developer-guide/coordinate-systems#app-content-space--your-applications-world-inside-the-box) defines the space.
- **The world axes can differ.** QAROS describes the world with `+Y` up, while a ROS scene, such as the one of RViz, is Z-up with `+X` forward (REP 103). A Y-up pose fed into a Z-up world tips the viewer on its side. By default the library maps the two worlds one to one, which is right for Y-up content. For Z-up content, set `QAR_POSE_WORLD=zup`, exactly in that spelling. The library then rotates the QAROS world into the Z-up world with `ros = (-z, -x, y)`, so that QAROS up becomes up and QAROS forward becomes forward. In that mode a viewer anchor follows the change of axes: a yaw about the vertical axis, `QAR_POSE_YAW_DEG`, `180` degrees by default, and an eye height and a stand-off behind the facing direction, `QAR_POSE_HEIGHT_M` and `QAR_POSE_BACK_M`, both `0` by default. Leave those two at `0`: QAROS reports the viewer in its own frame, so a translation that you apply only on your side is invisible to QAROS, and the two worlds would disagree about where the viewer stands. Set these variables before the program starts: the camera conversion reads them once for each process.

### Stereo layout and projection

- **One frame, four textures.** The library declares four views in this order: left color, right color, left depth and right depth. Their layout is `QAR_FRAME_LAYOUT_SEPARATED_TEXTURES`, the only layout it accepts, and the texture index of a view names its texture. [Rendering Streams](/docs/developer-guide/rendering-streams) describes the layouts of the C API.
- **Formats.** Color is `QAR_PIXEL_FORMAT_R8G8B8A8`. Depth is always `QAR_PIXEL_FORMAT_R32_FLOAT`, whatever depth format the stream configuration requests.
- **Size.** Each eye renders at the `width` and `height` of its `StereoStreamConfig`. The copy into the textures of QAROS is a blit, which rescales when the sizes differ.
- **Off-axis projection.** Each eye has its own signed half-angles. The library builds an asymmetric perspective matrix for each eye from the field of view that QAROS reports, and the eye camera uses it as its custom projection.

### Depth

The GL3+ render system of Ogre 1.12 cannot expose a metric, sampleable hardware depth texture through its public API, so the library writes depth as color. It renders the scene a second time through a depth material scheme whose shaders replace the material of every object, into a single-channel 32-bit float texture. The format that it produces and sends is therefore always `QAR_PIXEL_FORMAT_R32_FLOAT`. The texture holds one of two encodings, chosen by `QAR_DEPTH_ENCODING`:

| Encoding | Value written | Where nothing was drawn | Chosen by |
|---|---|---|---|
| Metric, the default | The linear eye-space distance in meters, multiplied by `QAR_DEPTH_SHADER_SCALE` if you set it | `0` | `QAR_DEPTH_ENCODING` unset, or any value other than `ndc` |
| Window space | `gl_FragCoord.z`, the value of an OpenGL depth buffer in the range 0 to 1, derived from the projection that the camera renders with | `1`, the far plane | `QAR_DEPTH_ENCODING=ndc`, exactly in that spelling |

`QAR_DEPTH_SHADER_SCALE` applies to the metric encoding only and must be greater than 0; a malformed value keeps meters. The library reads both variables once for each process, when it creates the first depth target.

The SDK documents the units of the depth values and not their encoding. `qar_render_sender_show_frame` takes the near and far distances that the frame was rendered with, in app units, and a `depth_scale`, the factor from the depth buffer's units to the view poses' units. The header says nothing about the encoding of the values or about what a receiver does with these fields. What the authors of `qar_ogre_adapter` know about receivers comes from one observation:

- **Observed with an SDK drop earlier than that of 2026-09-28:** the Visualizer reads the depth texture as the raw window-space value in the range 0 to 1, and changing `depth_scale` or the near and far distances that you report does not change what it shows. Metric depth therefore displays wrongly there: every surface farther than one meter saturates, and the scene shows in one flat color. Set `QAR_DEPTH_ENCODING=ndc` for the Visualizer. The observation has not been repeated with the drop of 2026-09-28 or with release 0.9.0.
- What any other peer does with the depth values is not documented by the SDK.

The window-space value is reciprocal in distance. A far plane that is much wider than your content leaves little of the range for the near surfaces, and everything beyond the far plane is clipped away. The far distance is `near_far` of the factory configuration, `500 m` by default, so set it to fit your content.

## Session start-up

This section describes the part of the library that release 0.9.0 replaces by onboarding to the Hub. For the calls of 0.9.0, see [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions).

- **One object does it.** The constructor of `QarCApiSenderFactory` runs the whole start-up and blocks until it is done, so an application with a window constructs it on a worker thread. A failed start-up throws no exception: `ready()` says whether the start-up worked, and `last_error()` says what failed.
- **What it does.** It loads `qar-streaming-c.dll` from the SDK folder, initializes the library, creates a runtime and a session, joins the session, reads its own peer id, creates the one app volume that every stream is keyed to, subscribes to peer updates and invites the peers that its configuration names, none by default. The live example then invites a Visualizer with `invite_additional_visualizer`.
- **The app volume.** The SDK rejects a stream without an app volume. The library creates a cube of `40 m`, centered `1 m` above the floor, with the app scale `1.0`; the fields are `create_app_volume`, `app_volume_name`, `app_volume_width_m`, `app_volume_length_m`, `app_volume_height_m`, `app_volume_center_y_m`, `app_volume_scale` and `app_volume_pick_gestures`. Nothing clips the content to the box. If the volume cannot be created, the session stays usable, but the factory refuses to create a sender and says why.
- **The peers.** The peer subscription of the SDK reports everything that joins. With an SDK drop earlier than that of 2026-09-28, one invited Visualizer appeared as about five peer ids. The library therefore streams only to the peers that it admits: with the default policy `kInvitedOnly`, the peers that the factory invited itself, and never more than `max_peers`. The other fields of the configuration that belong to the start-up are `invite_peers`, `auto_invite_visualizers`, `visualizer_conn_suffix`, `admission`, `admission_name_filter`, `baseline_settle_ms`, `invite_flow_admission` and `launcher_config_json`.

## Where it stops

### What to design around

- **The cost grows with the peers.** Each peer needs both eyes rendered in every frame, and the library admits at most `8` peers at the same time by default. The render budget sheds a due stream that does not fit in `max_batch_milliseconds`, but a stream under floor protection renders even when the batch is over budget. Keep your scene cheap enough for the peers you expect.
- **NVIDIA adapters only.** The header states that the C API currently supports NVIDIA adapters only. On a machine with an integrated and an NVIDIA adapter, run your program on the NVIDIA adapter. Otherwise Windows may render on the integrated adapter, the frame rate can be low, and each frame is copied between adapters.
- **Creating a sender can take arbitrarily long.** Creating a render sender negotiates a transfer with the receiver. As far as the authors of `qar_ogre_adapter` observed with an SDK drop earlier than that of 2026-09-28, it has no bound on its duration, and the library does not cancel the wait. The library creates the sender asynchronously, so your render loop keeps running, and it logs once, after `5` seconds, that a peer is still creating.
- **The frame has no synchronization field.** The OpenGL frame of the SDK carries no field for synchronization, so the library waits with `glFinish` after the copy.
- **What the header leaves open.** [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) collects what the header of the C API leaves open about streams, sizes and the position of the operator.

### What has been verified

- The unit tests of the library need no SDK, and they pass. The headless sample passes. With an SDK of the drop of 2026-09-28, `--connect-only` opened a session and reported the app volume.
- The live example has not been run with a Visualizer. The library ran in live sessions of the RViz2 plugin, which streams the scene of RViz2 to a Visualizer through it. No live session has run the current code of the library, and how the pictures looked in the Visualizer was not judged in those sessions.
- No device check has been run with the current code of the library, because no headset was available.

### Where to look when it does not work

- **The troubleshooting document of the repository** ([github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)) lists symptoms, causes and fixes, among them a black scene in the Visualizer, a flat depth color, a scene tipped on its side, a QAROS graphics thread without an OpenGL context, a sender that is not created and a peer that gets no stream.
- **The diagnostics of the manager.** `diagnostics_snapshot()` returns a copy of all counters, with the submitted and skipped frames of each stream, the totals of the four skip reasons (`SenderNotReady`, `SenderError`, `TargetInitFailed` and `BudgetShed`) and the account of the adapter for the latest skip in `last_skip_detail`. A stall shows as `SenderNotReady` forever while the submit counter stays frozen. For `SenderError`, `last_skip_detail` names the failed call.
- **The logs.** The library writes its own state to the standard error stream with the prefix `[qar-adapter]`. By default QAROS writes its library log into the folder `quaternar` below your `TEMP` folder. The switches `QAR_ADAPTER_TRACE`, `QAR_ADAPTER_FRAME_TRACE` and `QAR_VIDEO_TIMING_DIR` add a trace of the calls of the SDK, a per-frame trace and stage timings.
- **Two lines that look like errors.** When the streams close at the end of a run, the adapter can print `wglMakeCurrent FAILED`. While a peer still negotiates, it prints `still creating after` and a number of seconds. Neither line is a failure by itself.

## Related pages

- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries and how they stack.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender, the frame loop and the layouts of the C API.
- [Coordinate Systems](/docs/developer-guide/coordinate-systems) — room, app volume and app content space.
- [App Volumes and Gestures](/docs/developer-guide/app-volumes) — the volume that every stream is keyed to.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the session start-up of release 0.9.0.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — the panel route of the same adapter.
- [Troubleshooting](/docs/developer-guide/troubleshooting) — status codes and frequent integration mistakes.
- [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) — the RViz2 plugin that streams the scene of RViz2 through this library.
