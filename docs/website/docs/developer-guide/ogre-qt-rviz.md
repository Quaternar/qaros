---
sidebar_position: 10.5
title: Ogre, Qt and RViz2 on QAROS
description: Three C++ libraries that build on QAROS — an Ogre integration layer, a standalone Qt panel package that turns widgets into GUI panels and an RViz2 plugin — how they depend on each other and when to use which.
---

# Ogre, Qt and RViz2 on QAROS

Three libraries build on QAROS from C++ and run on Windows 11. **`qar_ogre_adapter`** streams an Ogre scene into the room, **`qar_qt_streaming`** turns Qt widgets into GUI panels, and **`qar_rviz_plugin`** brings RViz2 and the ROS 2 graph into AR. Each one is C++17 code in a repository of its own on GitHub, licensed under MIT. A repository releases when a pull request that changes its behavior is merged, and its `VERSION` file holds the version of the last release.

The first two are **independent** of each other: you can use either without the other. The Qt package needs no ROS 2 at all, and the Ogre library uses the ROS 2 Humble distribution only as the source of Ogre. The RViz2 plugin builds on both. This page tells you what each library does, when to use which, what you need installed and where the detail lives. The installation steps, the API and the troubleshooting stay in the repositories; this site links to them and does not copy them.

:::info Which QAROS runtime
The three repositories take the QAROS C and C++ headers from the git submodule `external/qaros`, pinned to the QAROS commit `0f2cb2e` (the 0.10.0-dev line of the C API); clone them with `--recurse-submodules`. The live checks that these pages report were made with the installed QAROS runtime **0.9.0-dev.3008+65dc9aeb**, a development build that precedes the 0.9.0 release, with the libraries built before they moved onto that pin. The pinned header lists functions that the dynamic loader requires, and its own rule is that a runtime without a required function fails the load of the module; the export table of the 0.9.0-dev.3008 runtime, read from the file without loading it, lacks 12 of them (and one optional function). The libraries built on the pinned header have not been run against any runtime in a live session. The C headers carry no version macro, so a library cannot check the runtime version. The libraries that join QAROS themselves do it the way [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) describes: a program on the PC that runs the Hub joins as a **local app** (one **Allow** in the Hub, nothing stored), and that is the only join they offer. To work with a Hub on another PC, make the Hub of your own PC a member of it ([Remote app: through a member hub](/docs/developer-guide/onboarding-and-sessions#remote-app-through-a-member-hub)). The observations about earlier SDK drops that these pages report are marked with their drop. Linux is not verified: the libraries have not been built there.
:::

## When to use which

| Library | Use it when | What it does |
|---|---|---|
| **`qar_ogre_adapter`** | Your application renders with Ogre and should appear in the room as a stereo stream, or you want the C++ adapter of the C API that the RViz2 plugin builds on. | Renders an Ogre scene for each eye into OpenGL textures and streams them to QAROS, one stream for every target. Holds the adapter of the C API: the session owner that joins the Hub, and the adapters for render senders, URL panels and gestures. |
| **`qar_qt_streaming`** | You have a Qt 5.15 or Qt 6.5 (or newer) widget interface and want it to work as a GUI panel in the room, and your program uses no Ogre, ROS 2 or RViz2. | Draws a `QWidget` off screen, streams the picture as the content of a GUI panel and turns the gestures that QAROS reports on the panel into Qt events. Its backend does that over the C++ SDK `qar-streaming-cpp`. |
| **`qar_rviz_plugin`** | You work with ROS 2 Humble on Windows and want RViz2, its panels or the topics of the ROS 2 graph in AR. | An RViz2 plugin that renders the live scene of RViz2 for each target, streams RViz2 panels into AR as GUI panels, generates web panels for ROS 2 topics, services and actions, and turns the pointer of the headset into ROS 2 messages. |

If your application uses none of Ogre, Qt and RViz2, call the C API directly: [Language Bindings](/docs/developer-guide/language-bindings#c) lists the idioms for C++, [Rendering Streams](/docs/developer-guide/rendering-streams) describes the render sender, and [Custom GUI Panels](/docs/developer-guide/gui-panels) the panels.

## How the libraries depend on each other

| Library (repository) | Builds on | Also needs |
|---|---|---|
| `qar_ogre_adapter` ([qaros-ogre](https://github.com/Quaternar/qaros-ogre)) | nothing | the ROS 2 Humble distribution, for Ogre only |
| `qar_qt_streaming` ([qaros-qt](https://github.com/Quaternar/qaros-qt)) | nothing | Qt 5.15 or Qt 6.5 and newer, and the headers of the C++ SDK with the C header |
| `qar_rviz_plugin` ([qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz)) | `qar_ogre_adapter` and `qar_qt_streaming`, at the versions that its `CMakeLists.txt` requests | the ROS 2 Humble distribution with its repairs, Qt 5.15.2 and colcon |

`qar_ogre_adapter` and `qar_qt_streaming` never depend on each other. What only the RViz2 plugin needs lives in the RViz2 plugin's repository: the scripts that repair the ROS 2 distribution, the Python requirements for colcon (`requirements-ros.txt`) and the glue `qar_capi_panels`, which lets the Qt panels use the QAROS session of the plugin. `qar_ogre_adapter` and the RViz2 plugin each carry a script that downloads the ROS 2 distribution (`fetch_dependencies.ps1`); the Qt package has none. `qar_ogre_adapter`, `qar_qt_streaming` and the RViz2 plugin compile against the headers of the submodule `external/qaros`; the Qt package takes the C++ SDK headers from targets that your project defines, else from the folder named by `QAR_STREAMING_CPP_INCLUDE_DIR`, else from the submodule, and the plugin stops its configure when an installed upstream package carries other QAROS headers (`QAR_QAROS_HEADER_CHECK`).

## Requirements

All three libraries need:

- Windows 11, 64-bit; Linux is not built or tested,
- the Microsoft C++ toolset (v142, Visual Studio 2019 Build Tools, for Ogre and the RViz2 plugin; the Qt package also has a preset for Visual Studio 2022),
- CMake 3.22 or newer, Git and PowerShell 7,
- Python 3.10 for the tooling: the tests include a layering guard that checks which library may include what.

Beyond that, each library needs what the table above names. Ogre 1.12.1 comes from the binary distribution of ROS 2 Humble for Windows, and the RViz2 plugin needs the same distribution with the repairs that the scripts of its repository apply. The Qt package needs only Qt (Core, Gui and Widgets) and, for its QAROS backend, the headers of the C++ SDK and the QAROS C header, which come from the submodule unless your project or `QAR_STREAMING_CPP_INCLUDE_DIR` supplies them.

The libraries do not contain the QAROS runtime. They load `qar-streaming-c.dll` at run time with the vendor's runtime lookup (`qar_library_load(NULL)`), which finds a copy beside the program, the runtime of the instance that `QAR_INSTANCE` names or the installed QAROS (`QAROS_RUNTIME_DIR` is not read), so building and testing need no QAROS; joining a Hub does. The installed QAROS is also the source of the C header that the libraries compile against.

All three libraries follow one rule for the threads: no QAROS call that may wait runs on the Qt thread or on the render thread of your program. In the Ogre library and the RViz2 plugin a thread of the session owner runs those calls in order; in the Qt package the worker of its backend does (or an executor of your own that runs them in order off the Qt thread). The calls of the frame path (reserve a frame, read the poses, show the frame) run on a frame worker, a thread of its own for each 3D sender and for each panel sender of the plugin, so the Qt thread only hands work over and never waits for QAROS; in the Qt package without the plugin's glue the Qt thread still makes the three calls of a frame. The Qt thread collects the results. A query of the installed runtime costs about two seconds, so this rule is what keeps RViz2 and a Qt program responsive; [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#how-long-a-query-takes) lists the measurements.

## What each library gives you

### Ogre: `qar_ogre_adapter`

- **A stereo stream for every target.** It renders an Ogre scene into a color and a depth render texture for each eye, takes the pose and the field of view of each eye from QAROS and renders the targets that are due in one batch under a render budget.
- **The adapter of the C API.** Five components (`capi`, `session`, `render`, `url` and `gestures`) hold the loader of the runtime, the session owner that joins the Hub and lists its targets, the senders, the URL panels and the gestures. The RViz2 plugin builds on them.
- **Geo anchor.** A WGS84 fix or an ECEF frame can georeference the app volume.
- **A headless sample and a live example.** The sample renders the scene for synthetic peers, needs no SDK and writes stereo image pairs. The live example joins the Hub and streams the scene to the first target that comes up, a mode that the authors of `qar_ogre_adapter` have not run with a target.

The page [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) describes how a frame travels, how to integrate the library and the conventions your renderer follows.

### Qt: `qar_qt_streaming`

- **Any `QWidget` as a GUI panel.** The library draws the widget with `QWidget::render()` off screen and sends a frame only when the picture changed. A panel has a stable name, so the next start finds the same panel in the Hub again.
- **Three layers.** A seam of plain C++ types and interfaces, the Qt core over the seam, and the `qaros` backend over the C++ SDK. You can use the core with a backend of your own.
- **Panels sized for reading in AR.** By default a panel hangs in the room at 1.2 mm for each logical pixel of the widget, whatever the pixel density of the display.
- **AR input as Qt events.** A tap, a drag or a hover on a panel reaches the widget as touch events, or as mouse events for a widget that does not take touch.
- **Popups, dialogs and video.** The popups of a panel are drawn into its picture, a dialog that a panel opens becomes a GUI panel of its own, and a video widget paints the newest frame that a decoding thread delivers.
- **An example program.** `qar_qt_panel_example` joins the Hub and streams two widgets.

The page [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) describes how a GUI panel displays a video, what the package consists of and what it does with a widget.

### RViz2: `qar_rviz_plugin`

- **A panel in RViz2.** The plugin `qar_rviz_streaming` adds a panel to RViz2 that joins the QAROS Hub of its PC as a local app. For every target that the Hub lists it renders the live 3D scene of RViz2 from the head pose that QAROS reports and sends it as stereo video.
- **RViz2 panels as GUI panels.** Any dock of RViz2 can be streamed into AR as a GUI panel; a click or a hover in AR reaches the real widget. The Qt package does the streaming, and the glue `qar_capi_panels` in the RViz2 plugin's repository lets it use the QAROS session of the plugin.
- **ROS 2 graph tools.** The panel lists the topics, services and actions of the ROS 2 graph and generates an AR tool for any of them: a web page that a server in RViz2 serves and that QAROS opens as a URL panel.
- **Web libraries you can use without RViz2.** `qar_ui_model` describes a panel as data and renders it as a page, `qar_web_panels` serves the page and its WebSocket, and `qar_ros_ui` reads and writes the ROS 2 graph. The two examples of the repository use them in plain programs.
- **Picking.** The pointer of the headset publishes a point, a goal pose, an initial pose or an area on the topics of the RViz2 tools.

The page [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) tells an operator how to start the plugin, what to expect in AR and how to read the logs. The behavior of the plugin does not depend on how the repositories are divided. The subsections below name the parts of the design that matter to a developer who builds on the libraries or reads their logs.

### The failure model

RViz2 must never wait for QAROS, so the plugin is built around four parts. All of it is checked by unit tests against a table of fake C functions; the live checks of 2026-11-09 to 2026-11-19 are listed on the operator page.

1. **The outage is detected and followed.** A supervisor thread of the session owner (`qar_ogre_adapter`) watches the owner thread and two hints from the PC of the Hub, the endpoint file and the Hub processes. The Hub counts as silent after 3 seconds without a runtime peer, a call of 10 seconds or a task of 30 seconds; after 20 seconds of silence (5 when a hint says the Hub is gone) the session is lost. The panel then releases everything it built on the session, hands the session back with `release_lost_session()`, remembers which AR panels streamed, and once the owner has joined again by itself (after 2, 4, 8, 16, then every 30 seconds, without a limit) it makes its factory, its streams and those panels again.
2. **Nothing is released or waited for on the Qt thread.** Handles go to the release lane of the owner, `IReleaseLane::post_release()`, which queues the release for the thread that owns the session and refuses when that thread has ended or was left behind (the handles then stay unreleased, the safe outcome). The exit is bounded: the destructor of the owner waits for its thread and the frame workers for a few seconds and then leaves them running with the plugin pinned in memory, and an exit guard ends the process itself 5 seconds after the application was asked to quit.
3. **Work that can wait runs on workers.** One frame worker per sender makes the frame calls; the 3D frame ends its copy with a GL fence that the worker waits for; a GPU progress gate closes the frames of RViz2 and the 3D batch while the GPU makes no progress; a stall watchdog names the step of the Qt thread when the render tick stops for 3 seconds.
4. **The plugin keeps its own state honest.** A size change never closes a QAROS panel (the default `new-panel` mode keeps a ring of 8 hidden panels), a 3D sender that the runtime asks for no frame is made again only as a last resort after 45 seconds, and the panel says in words which stream is in trouble.

The layering is unchanged: the owner, the frame worker, the gate and the lane live in `qar_ogre_adapter`, the resize modes and the ring in `qar_qt_streaming`, and neither depends on the other. The glue `qar_capi_panels` of the plugin's repository gives the panel senders the same frame worker and release lane.

### The 3D sender across a restart of the Hub

After a restart of the Hub the runtime replays the id of the render stream that the Visualizer asked for before, so a sender made for the old Hub is never asked for frames again, and a new sender is asked for them only when the old one is gone. The factory therefore releases the 3D senders when the Hub process ends and makes them again when the new Hub runs: the owner counts the Hub processes it saw end (`QarSessionOwner::hub_process()` reports `known`, `pid`, `gone_count` and `last_gone_pid`), the producer compares the count on its tick, and when it rose the factory lists the peers of its senders as closed, the producer destroys them, and no new sender is handed out until the Hub has another pid and the owner runs. The step makes no QAROS call. A restart of the Visualizer alone changes nothing. The recycle rule stays as the last resort for what the Hub process does not show: a sender whose requests fail for 3 seconds and that has begun no frame for 45 seconds (`QAR_RENDER_RECYCLE_S`; counted from the latest of its creation, its last frame and the end of its last reconnecting run, doubled each time and capped at 240 seconds) is closed and made again. `log_render_requests()` prints once per pair of target and stream id which stream the receiver asks for, with the verdict `match`, `MISMATCH` or `unknown`; it makes no call into QAROS. With the installed runtime 0.9.0-dev.3008 the floor came back in about 30 seconds after a Hub restart in the live check of 2026-11-18.

### Screen-space objects are left out of the AR image

The scene of RViz2 holds objects that are placed on the screen and not in the room, such as the rectangle of the selection tool. A renderable with an identity projection would land at the same screen fraction in both eyes of the AR image, because the eye views have a camera of their own. A listener on the render queue therefore refuses such a renderable around the color pass and around the depth pass (`ar_render_keeps`), and the first frame of a stream that left objects out prints one line that says how many.

### Copies of Qt panels, the builder and the binder

An AR copy of a Qt panel is a second instance of the panel, shown off-screen only and streamed under the key of the dock. A table in the pure unit `ar_copy_panels.hpp` names the classes that have one (Displays, Views, Tool Properties, Selection, Help) with the width of a copy; the factory of the copies is the one of the hidden Displays panel, made general. The copy of an RViz2 panel binds to the same models as the dock. The plugin's own panel has no model to share, so its copy is built another way: `build_qar_panel_view` is the construction part of the panel's window, called once for the panel and once more for the copy under the enlarged style, and `QarPanelArCopy` binds the controls of the copy to those of the panel with a `WidgetTwinBinder`. The binder, a class of `qar_qt_streaming`, keeps a second widget tree (the twin) in step with a first (the original): `sync()` polls the original, writes only where a value differs, and an action on the twin (a click, a choice, a step, a chosen row, an edit) is carried out on the original, so every slot connected to the original runs as for a user of the desktop. The style `EnlargedControlsStyle`, also in `qar_qt_streaming`, draws the controls of a widget tree larger by a factor and leaves the text alone; the host decides the factor, the height limit of 1.8 m and when to switch. A host can install a dialog scale provider and reporter on the manager of the panels, so that a dialog's panel gets a scale of its own.

## Environment variables of the libraries

The libraries read these variables once for each process unless the row says otherwise. The plugin's own variables are on the page [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin#environment-variables).

| Variable | Library | Effect |
|---|---|---|
| `QAR_FRAME_CALLS` | `qar_ogre_adapter` | `worker` (the default) or `gui`: where the QAROS frame calls of a sender run. |
| `QAR_FRAME_FENCE` | `qar_ogre_adapter` | `worker` (the default) or `gui`: where the wait for the GPU copy of a 3D frame runs. |
| `QAR_GPU_GATE`, `QAR_GPU_GATE_TEST` | `qar_ogre_adapter` | `on` (the default) or `off` for the GPU progress gate; a number of seconds closes the gate once on purpose, for a test. |
| `QAR_RENDER_RECYCLE_S` | `qar_ogre_adapter` | Seconds, 45 by default: how long a 3D sender may get no frame request before it is made again; `0` switches the rule off. |
| `QAR_ADAPTER_STREAM_RELEASE` | `qar_ogre_adapter` | `owner` (the default) or `gui`: which thread destroys the stream handles of the 3D senders. |
| `QAR_POSE_WORLD`, `QAR_DEPTH_ENCODING` | `qar_ogre_adapter` | `zup` and `ndc`; see [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming#conventions-your-renderer-must-follow). |
| `QAR_PANEL_RESIZE` | `qar_qt_streaming` | `swap` (the library's default), `new-panel` or `defer`, read by a program with `parse_panel_resize_mode`; the plugin runs `new-panel`. |

## Where the detail lives

Each repository documents its installation from a clean Windows 11 machine, getting started, architecture, API, configuration, testing, troubleshooting and SDK compatibility:

- `qar_ogre_adapter`: [github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)
- `qar_qt_streaming`: [github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)
- `qar_rviz_plugin`: [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz)

## Related pages in this guide

- [Language Bindings](/docs/developer-guide/language-bindings#c) — there is no separate C++ binding; the C++ section lists the idioms for the C API.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender, the frame loop and the layouts that a stereo stream uses.
- [Custom GUI Panels](/docs/developer-guide/gui-panels) — the GUI panels that the Qt package and the RViz2 plugin create.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the local app the libraries use to join the Hub, and the member Hub that brings a program on another PC in.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — the video route that the Qt package uses, and what it does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, the gesture rules of an app volume, and what the C++ SDK headers need.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how the Ogre library streams a scene, from the frame loop to the conventions your renderer follows.
- [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) — starting RViz2 with the plugin, what you see and do in AR, and the logs.
