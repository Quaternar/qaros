---
sidebar_position: 10.5
title: Ogre, Qt and RViz2 on QAROS
description: Three C++ libraries that build on QAROS — an Ogre integration layer, a standalone Qt panel package that turns widgets into GUI panels and an RViz2 plugin — how they depend on each other and when to use which.
---

# Ogre, Qt and RViz2 on QAROS

Three libraries build on QAROS from C++ and run on Windows 11. **`qar_ogre_adapter`** streams an Ogre scene into the room, **`qar_qt_streaming`** turns Qt widgets into GUI panels, and **`qar_rviz_plugin`** brings RViz2 and the ROS 2 graph into AR. Each one is C++17 code in a repository of its own on GitHub, licensed under MIT, at version 0.2.0.

The first two are **independent** of each other: you can use either without the other. The Qt package needs no ROS 2 at all, and the Ogre library uses the ROS 2 Humble distribution only as the source of Ogre. The RViz2 plugin builds on both. This page tells you what each library does, when to use which, what you need installed and where the detail lives. The installation steps, the API and the troubleshooting stay in the repositories; this site links to them and does not copy them.

:::info Which QAROS runtime
The libraries were built and checked against the installed QAROS runtime **0.9.0-dev.3008+65dc9aeb**, a development build of the setup that precedes the 0.9.0 release, and against its header. The Qt package asks for at least QAROS 0.9 with the onboarding C API; the C headers carry no version macro, so the package records that number in its CMake configuration and cannot check it. The libraries that join QAROS themselves do it the way [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) describes: a program on the PC that runs the Hub joins as a **local app** (one **Allow** in the Hub, nothing stored), and a program that talks to a Hub on another PC pairs with the code the Hub shows for a new device or app. They have not been checked against another build, and the observations about earlier SDK drops that these pages report are marked with their drop. The document `docs/SDK_COMPATIBILITY.md` of each repository lists the runtime that the library supports and how to identify yours.
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
| `qar_rviz_plugin` ([qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz)) | `qar_ogre_adapter` and `qar_qt_streaming`, both 0.2.0 | the ROS 2 Humble distribution with its repairs, Qt 5.15.2 and colcon |

`qar_ogre_adapter` and `qar_qt_streaming` never depend on each other. What only the RViz2 plugin needs lives in the RViz2 plugin's repository: the scripts that repair the ROS 2 distribution, the Python requirements for colcon (`requirements-ros.txt`) and the glue `qar_capi_panels`, which lets the Qt panels use the QAROS session of the plugin. `qar_ogre_adapter` and the RViz2 plugin each carry a script that downloads the ROS 2 distribution (`fetch_dependencies.ps1`); the Qt package has none. `qar_ogre_adapter` and `qar_qt_streaming` each carry a copy of the C header that they compile against, and the Qt package also carries a copy of the C++ SDK headers.

## Requirements

All three libraries need:

- Windows 11, 64-bit; Linux is not built or tested,
- the Microsoft C++ toolset (v142, Visual Studio 2019 Build Tools, for Ogre and the RViz2 plugin; the Qt package also has a preset for Visual Studio 2022),
- CMake 3.22 or newer, Git and PowerShell 7,
- Python 3.10 for the tooling: the tests include a layering guard that checks which library may include what.

Beyond that, each library needs what the table above names. Ogre 1.12.1 comes from the binary distribution of ROS 2 Humble for Windows, and the RViz2 plugin needs the same distribution with the repairs that the scripts of its repository apply. The Qt package needs only Qt (Core, Gui and Widgets) and, for its QAROS backend, the headers of the C++ SDK and the QAROS C header; a copy of both is part of the repository, and the build uses the headers of an SDK installation instead when you name one.

The libraries do not contain the QAROS runtime. They load `qar-streaming-c.dll` at run time with the vendor's runtime lookup (`qar_library_load(NULL)`), which finds a copy beside the program, the runtime of the instance that `QAR_INSTANCE` names or the installed QAROS (the libraries' own loader also reads `QAROS_RUNTIME_DIR`, which the current header no longer does), so building and testing need no QAROS; joining a Hub does. The installed QAROS is also the source of the C header that the libraries compile against.

All three libraries follow one rule for the threads: no QAROS call that may wait runs on the Qt thread or on the render thread of your program. In the Ogre library and the RViz2 plugin a thread of the session owner runs those calls in order; in the Qt package the worker of its backend does (or an executor of your own that runs them in order off the Qt thread), and the Qt thread makes only the three calls of a frame. The Qt thread collects the results. A query of the installed runtime costs about two seconds, so this rule is what keeps RViz2 and a Qt program responsive; [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#how-long-a-query-takes) lists the measurements.

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

- **A panel in RViz2.** The plugin `qar_rviz_streaming` adds a panel to RViz2 that joins the QAROS Hub, as a local app on the Hub's PC or by pairing with a Hub on another PC. For every target that the Hub lists it renders the live 3D scene of RViz2 from the head pose that QAROS reports and sends it as stereo video.
- **RViz2 panels as GUI panels.** Any dock of RViz2 can be streamed into AR as a GUI panel; a click or a hover in AR reaches the real widget. The Qt package does the streaming, and the glue `qar_capi_panels` in the RViz2 plugin's repository lets it use the QAROS session of the plugin.
- **ROS 2 graph tools.** The panel lists the topics, services and actions of the ROS 2 graph and generates an AR tool for any of them: a web page that a server in RViz2 serves and that QAROS opens as a URL panel.
- **Web libraries you can use without RViz2.** `qar_ui_model` describes a panel as data and renders it as a page, `qar_web_panels` serves the page and its WebSocket, and `qar_ros_ui` reads and writes the ROS 2 graph. The two examples of the repository use them in plain programs.
- **Picking.** The pointer of the headset publishes a point, a goal pose, an initial pose or an area on the topics of the RViz2 tools.

The page [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) tells an operator how to start the plugin, what to expect in AR and how to read the logs. The behavior of the plugin does not depend on how the repositories are divided.

## Where the detail lives

Each repository documents its installation from a clean Windows 11 machine, getting started, architecture, API, configuration, testing, troubleshooting and SDK compatibility:

- `qar_ogre_adapter`: [github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)
- `qar_qt_streaming`: [github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)
- `qar_rviz_plugin`: [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz)

## Related pages in this guide

- [Language Bindings](/docs/developer-guide/language-bindings#c) — there is no separate C++ binding; the C++ section lists the idioms for the C API.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender, the frame loop and the layouts that a stereo stream uses.
- [Custom GUI Panels](/docs/developer-guide/gui-panels) — the GUI panels that the Qt package and the RViz2 plugin create.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the local app and the pairing that the libraries use to join the Hub.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — the video route that the Qt package uses, and what it does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, the gesture rules of an app volume, and what the C++ SDK headers need.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how the Ogre library streams a scene, from the frame loop to the conventions your renderer follows.
- [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) — starting RViz2 with the plugin, what you see and do in AR, and the logs.
