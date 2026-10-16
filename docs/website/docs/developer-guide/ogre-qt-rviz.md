---
sidebar_position: 10.5
title: Ogre, Qt and RViz2 on QAROS
description: Three C++ libraries for Windows that build on the QAROS C API — an Ogre integration layer, Qt widgets as GUI panels and an RViz2 plugin — and when to use which.
---

# Ogre, Qt and RViz2 on QAROS

Three libraries build on the QAROS C API from C++ and run on Windows 11. **`qar_ogre_adapter`** streams an Ogre scene into the room, **`qar_qt_streaming`** turns Qt widgets into GUI panels, and **`qar_rviz_plugin`** brings RViz2 and the ROS 2 graph into AR. Each one is C++17 code in a repository of its own on GitHub, licensed under MIT, and together they form a **stack**: each library builds on the one before it.

This page tells you what each library does, when to use which, what you need installed and where the detail lives. The installation steps, the API and the troubleshooting stay in the repositories; this site links to them and does not copy them.

:::info Which SDK generation
The libraries were built, tested and verified against the QAROS SDK drop of 2026-09-28. The C API that this site documents, release 0.9.0, is a later generation: the session set-up, the way a gesture reports its hand, the time points and the init structures of volumes and panels are different, and the libraries have not been rebuilt against it. Read this page for what the libraries do, and use the rest of this guide for the calls of 0.9.0, starting with [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions). The document `docs/SDK_COMPATIBILITY.md` of each repository lists the SDK drops that the library supports, how to identify the drop you have and what differs in the later generation.
:::

## When to use which

| Library | Use it when | What it does |
|---|---|---|
| **`qar_ogre_adapter`** | Your application renders with Ogre and should appear in the room as a stereo stream, or you want the C++ adapter of the C API that the other two libraries build on. | Renders an Ogre scene for each eye into OpenGL textures and streams them to QAROS, one stream for every peer. Holds the adapter of the C API for peers, panels, URL panels and gestures. |
| **`qar_qt_streaming`** | You have a Qt 5.15 widget interface and want it to work as a GUI panel in the room. | Draws a `QWidget` off screen, streams the picture as the content of a GUI panel and turns the gestures that QAROS reports on the panel into Qt events. |
| **`qar_rviz_plugin`** | You work with ROS 2 Humble on Windows and want RViz2, its panels or the topics of the ROS 2 graph in AR. | An RViz2 plugin that renders the live scene of RViz2 for each peer, streams RViz2 panels into AR as GUI panels, generates web panels for ROS 2 topics, services and actions, and turns the pointer of the headset into ROS 2 messages. |

If your application uses none of Ogre, Qt and RViz2, call the C API directly: [Language Bindings](/docs/developer-guide/language-bindings#c) lists the idioms for C++, [Rendering Streams](/docs/developer-guide/rendering-streams) describes the render sender, and [Custom GUI Panels](/docs/developer-guide/gui-panels) the panels.

## How the libraries stack

Install the libraries, and read their documentation, in this order: `qar_ogre_adapter` first, then `qar_qt_streaming`, then `qar_rviz_plugin`.

| Library | Builds on | Also needs |
|---|---|---|
| `qar_ogre_adapter` | nothing | the ROS 2 Humble distribution, for Ogre only |
| `qar_qt_streaming` | `qar_ogre_adapter` | Qt 5.15.2 |
| `qar_rviz_plugin` | `qar_ogre_adapter` and `qar_qt_streaming` | the ROS 2 Humble distribution with its repairs, Qt 5.15.2 and colcon |

`qar_ogre_adapter` vendors the C header of the SDK that all three libraries compile against, so a new SDK header is brought in there first and the libraries are rebuilt in the same order. Each repository is at version 0.1.0 and asks for that version of the repositories it builds on, so the three have to be at matching versions. The installation document of each repository installs the repositories it builds on, so you can start from any of them.

## Requirements

All three libraries need:

- Windows 11, 64-bit; Linux is not built or tested,
- the Microsoft C++ toolset v142 (Visual Studio 2019 Build Tools),
- CMake 3.22 or newer, Git and PowerShell 7,
- Python 3.10 for the tooling: the tests include a layering guard, and where a library needs the Qt installer or colcon, they run from a virtual environment.

Beyond that, each library needs what the table above names. Ogre 1.12.1 comes from the binary distribution of ROS 2 Humble for Windows, and the RViz2 plugin needs the same distribution with the repairs that the scripts of its repository apply.

The libraries do not contain the QAROS runtime. They load `qar-streaming-c.dll` at run time from the `bin` folder of the SDK that you point them at, so building and testing need no SDK; connecting to QAROS does.

## What each library gives you

### Ogre: `qar_ogre_adapter`

- **A stereo stream for every peer.** It renders an Ogre scene into a color and a depth render texture for each eye, takes the pose and the field of view of each eye from QAROS and renders the peers that are due in one batch under a render budget.
- **The adapter of the C API.** Five components (`capi`, `render`, `panels`, `url` and `gestures`) hold the loader of the SDK library, the senders, the panels, the URL panels and the gestures. The other two libraries build on them.
- **Geo anchor.** A WGS84 fix or an ECEF frame can georeference the app volume.
- **A headless sample and a live example.** The sample renders the scene for synthetic peers, needs no SDK and writes stereo image pairs. The live example streams the scene to a Visualizer, a mode that the authors of `qar_ogre_adapter` have not run.

The page [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) describes how a frame travels, how to integrate the library and the conventions your renderer follows.

### Qt: `qar_qt_streaming`

- **Any `QWidget` as a GUI panel.** The library draws the widget with `QWidget::render()` off screen and sends a frame only when the picture changed.
- **Panels sized for reading in AR.** By default a panel hangs in the room at 1.2 mm for each logical pixel of the widget, whatever the pixel density of the display.
- **AR input as Qt events.** A tap, a drag or a hover on a panel reaches the widget as touch events, or as mouse events for a widget that does not take touch.
- **Popups, dialogs and video.** The popups of a panel are drawn into its picture, a dialog that a panel opens becomes a GUI panel of its own, and a video widget paints the newest frame that a decoding thread delivers.

The page [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) describes how a GUI panel displays a video and what the library does with a widget.

### RViz2: `qar_rviz_plugin`

- **A panel in RViz2.** The plugin `qar_rviz_streaming` adds a panel to RViz2 that connects to QAROS. For every admitted peer it renders the live 3D scene of RViz2 from the head pose that QAROS reports and sends it as stereo video.
- **RViz2 panels as GUI panels.** Any dock of RViz2 can be streamed into AR as a GUI panel; a click or a hover in AR reaches the real widget.
- **ROS 2 graph tools.** The panel lists the topics, services and actions of the ROS 2 graph and generates an AR tool for any of them: a web page that a server in RViz2 serves and that QAROS opens as a URL panel.
- **Web libraries you can use without RViz2.** `qar_ui_model` describes a panel as data and renders it as a page, `qar_web_panels` serves the page and its WebSocket, and `qar_ros_ui` reads and writes the ROS 2 graph. The two examples of the repository use them in plain programs.
- **Picking.** The pointer of the headset publishes a point, a goal pose, an initial pose or an area on the topics of the RViz2 tools.

The page [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) tells an operator how to start the plugin, what to expect in AR and how to read the logs.

## Where the detail lives

Each repository documents its installation from a clean Windows 11 machine, getting started, architecture, API, configuration, testing, troubleshooting and SDK compatibility:

- `qar_ogre_adapter`: [github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)
- `qar_qt_streaming`: [github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)
- `qar_rviz_plugin`: [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz)

## Related pages in this guide

- [Language Bindings](/docs/developer-guide/language-bindings#c) — there is no separate C++ binding; the C++ section lists the idioms for the C API.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender, the frame loop and the layouts that a stereo stream uses.
- [Custom GUI Panels](/docs/developer-guide/gui-panels) — the GUI panels that the Qt library and the RViz2 plugin create.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the session set-up of release 0.9.0.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — the video route that the Qt library uses, and what it does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, and the gesture rules of an app volume.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how the Ogre library streams a scene, from the frame loop to the conventions your renderer follows.
- [Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin) — starting RViz2 with the plugin, what you see and do in AR, and the logs.
