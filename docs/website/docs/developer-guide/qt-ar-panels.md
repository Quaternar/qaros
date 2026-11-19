---
sidebar_position: 7.5
title: Video in GUI Panels, and Qt Widgets as AR Panels
description: How a GUI panel shows a video stream, what a stream panel accepts, and what the standalone Qt package qar_qt_streaming does to show a Qt widget in AR.
---

# Video in GUI Panels, and Qt Widgets as AR Panels

A GUI panel can show a **video stream**, so any picture that you can draw becomes a panel: a camera image, a rendered view, a Qt widget. This page has two parts. The first describes the video route of the C API. The second describes how the package `qar_qt_streaming` uses that route to turn a `QWidget` into a panel.

:::info Which QAROS runtime
The first part describes the header and the C# binding of the C API that this guide documents. The second part describes the package `qar_qt_streaming`. Its live checks were made through the RViz2 plugin and its example with the installed QAROS runtime **0.9.0-dev.3008+65dc9aeb**, a development build that precedes the 0.9.0 release, before the package moved onto the C and C++ headers of the git submodule `external/qaros` (QAROS commit `0f2cb2e`, the 0.10.0-dev line). The header lists functions that the dynamic loader requires, and its own rule is that a runtime without a required function fails the load of the module; the export table of the 0.9.0-dev.3008 runtime, read from the file without loading it, lacks 12 of them (and one optional function). The package built on the pinned header has not been run against any runtime in a live session, and Linux is not verified. The package does not join the Hub itself: your program does, as a local app on the PC of the Hub, or as a local app of a member Hub on another PC ([Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions)), and hands the joined session to the package. The document `docs/SDK_COMPATIBILITY.md` of the repository lists the runtime that the package supports.
:::

## Show a video in a GUI panel

A GUI panel shows a page at a URI, a page your session serves or the video of a video sender, and the choice is made when the panel is created: chain `QarGuiPanelUrlExt`, `QarGuiPanelServedContentExt` or `QarGuiPanelStreamIdExt` into `QarGuiPanelInit` (see [Custom GUI Panels](/docs/developer-guide/gui-panels)). To show a video:

1. **Create a video sender** with `qar_video_sender_create`. Its `QarVideoSenderInit` carries the frame layout, the views and textures that you write, and the graphics API: CPU, D3D11, OpenGL or Vulkan, each GPU API with its matching stream-parameter extension. The sender owns a logical video stream whose id the library generates, and it may exist before any receiver connects. In C# the sender is a `PrimarylessVideoSender`, created with `session.PrimarylessVideoSenders.Create`.
2. **Read the stream id** with `qar_video_sender_stream_id` (C#: `GetStreamId()`).
3. **Create the panel with the stream.** Chain a `QarGuiPanelStreamIdExt` that holds the id into `QarGuiPanelInit::header.next` and call `qar_gui_panels_get_or_create` (C#: set `GuiPanelOptions.ContentStreamId`). The stream must be that of a video sender owned by the same session, and the layout of that sender sizes the receiver of the panel.
4. **Run the frame loop on the sender.** Call `qar_video_sender_try_begin_frame` until it reports ready, write the frame (`qar_video_sender_frame_cpu` for a CPU sender; the D3D11, OpenGL and Vulkan senders have their own frame calls), then call `qar_video_sender_show_frame`. A cycle begins only when at least one destination is ready.

```c
QarVideoSenderInit sender_init = qar_video_sender_init_default();
sender_init.layout = layout;            /* the views and textures you write */

QarVideoSender* sender = NULL;
QarResult r = qar_video_sender_create(session, &sender_init, &sender);
qar_result_log_if_error(r);

QarStreamId stream_id = qar_stream_id_default();
qar_result_log_if_error(qar_video_sender_stream_id(sender, &stream_id));

QarGuiPanelStreamIdExt stream_ext = qar_gui_panel_stream_id_ext_default();
stream_ext.stream_id = stream_id;

QarGuiPanelInit init = qar_gui_panel_init_default();
init.header.next = &stream_ext;         /* content type: video stream, fixed at creation */
init.common_name = "camera.myapp.example.com";
init.display_name = "Camera";
init.pose = qar_pose_default();
init.pose.position.y = 1.5f;            /* top-edge midpoint, room space, meters */
init.pose.position.z = -1.0f;

QarGuiPanelId panel_id;
r = qar_gui_panels_get_or_create(session, &init, &panel_id);
qar_result_log_if_error(r);
```

The video sender is not the render sender. The render sender answers the render request of one target and hands out per-frame poses and fields of view; the video sender carries any layout and no render metadata, and the header names GUI panel content as its use. For stereo content that is shown in an app volume, use the render sender (see [Rendering Streams](/docs/developer-guide/rendering-streams)).

While no receiver is connected, a frame cycle does not begin. The C# documentation of `TryBeginFrame` names three failures to keep polling through: no active transfer, the normal state before the first receiver requests the stream (`QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS`); a stream that every receiver has paused (`QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED`); and a stream whose receivers are all reconnecting (`QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING`). [Troubleshooting](/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow) says what to do about the last two.

## What a stream panel accepts

A stream panel differs from a web panel in these points. [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) gives the header lines and the consequences.

- **Its content type is fixed.** `qar_gui_panels_navigate_to_uri` on a stream panel returns `QAR_STATUS_GUI_PANEL_CHANGE_NOT_PERMITTED`. To show another video, give the panel another stream with `qar_gui_panels_set_stream` (C#: `SetStream`).
- **It ignores the content scale.** The video is always drawn at the full panel width and scrolled vertically when it is taller than the panel.
- **Its size comes from the sender.** The resolution of the receiver is taken from the layout of the video sender, and the header does not say what a change of that layout does to a receiver that is already attached: see [Changing the size of a video](/docs/developer-guide/sdk-limits#changing-the-size-of-a-video).
- **Ending the stream is not a panel call.** Closing a panel and destroying a sender are two separate calls: see [Ending a video stream](/docs/developer-guide/sdk-limits#ending-a-video-stream).
- **Gestures report video pixels.** A position on a stream panel is in the pixel raster of the decoded video image.

## Qt widgets as AR panels

`qar_qt_streaming` is the video route for Qt widgets. The library draws a `QWidget` into a pixel buffer with the paint path of Qt, streams the buffer as the video of a GUI panel and turns the gestures that QAROS reports on the panel into Qt events, so that a widget that works on the desktop works in AR. It is a **standalone Qt package**: it needs Qt 5.15 or Qt 6.5 and newer and the headers of the QAROS C++ SDK, and no Ogre, no ROS 2 and no RViz2 in its build or in its dependency chain. [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) says how the package relates to the other two libraries.

### The package

The package has three layers, each a library of its own, so that you take what you need:

| Layer | Target | What it holds | Needs |
|---|---|---|---|
| **seam** | `qar_qt_streaming::seam` | Plain C++ types and interfaces: the panel values, the sender, sender factory and input source interfaces (`IPanelSender`, `IPanelSenderFactory`, `IPanelInputSource`), the naming rule for panels, the timing log, the gesture mapper. | the C++ standard library |
| **Qt core** | `qar_qt_streaming::qar_qt_streaming` | The capture of a widget, the stream manager, the input injector, the popup and dialog handling, the video widget. It talks to QAROS only through the seam. | Qt (Core, Gui, Widgets) |
| **`qaros` backend** | `qar_qt_streaming::qaros` | The sender, the sender factory and the input source over the C++ SDK `qar-streaming-cpp`: a CPU video sender, the GUI panel that shows its stream, and the subscription for the gestures. | the SDK headers; no Qt |

The Qt core and the backend meet only at the seam, in your program. A program that streams to QAROS links the second and the third layer; the seam alone is enough to write a backend of your own, and the repository's tests run the Qt core against a recording test double of the seam, which is not part of the installed package.

How to get it into your build:

- **Installed package.** `find_package(qar_qt_streaming CONFIG REQUIRED COMPONENTS seam qar_qt_streaming qaros)` gives the targets above; without `COMPONENTS` you get all that the package installed. Qt is searched on `CMAKE_PREFIX_PATH` and in the folder named by the environment variable `QAR_QT_ROOT`. In the repository's own build, `-DQAR_QT_MAJOR=5` or `6` selects the Qt major version; without it the version of the kit in `QAR_QT_ROOT` decides, else Qt 6 when CMake finds one, else Qt 5.
- **Subproject.** `add_subdirectory` of the repository, or `FetchContent_Declare` with the source ZIP, builds only the libraries; the tests, the examples and the install rules are off unless you ask for them. The source ZIP is made with `cpack --config CPackSourceConfig.cmake` in a configured build folder.
- **The SDK headers.** The backend needs the target `qar-streaming-cpp` (the C++ wrapper, header only) and `qar-streaming-c-headers` (the C header `qar_streaming.h`). They come from targets of these names that your project defines, else from the folder `QAR_STREAMING_CPP_INCLUDE_DIR` that holds `qar_streaming_cpp/Config.hpp` and `qar_streaming.h`, else from the git submodule `external/qaros` (`QAR_QAROS_SOURCE_DIR`): clone with `--recurse-submodules`, or run `git submodule update --init external/qaros`. Build with `QAR_QT_STREAMING_WITH_QAROS=OFF` for the seam and the Qt core alone.
- **Compiler.** C++17 or newer; the headers of the C++ SDK refuse an older standard. The authors built the package and ran its tests in C++17, C++20 and with `/std:c++latest`. The package compiles its own targets with `/W4 /permissive- /we4996` and passes none of that on to your program; the C++ SDK wrapper needs no conforming mode ([Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#the-c-sdk-headers) says why). `/we4996` makes a call of a deprecated C API function a compile error in the package's own code.
- **Version.** The package releases when a pull request that changes its behavior is merged, and its `VERSION` file holds the version of the last release. It is written for the C API of the pinned header; the C headers carry no version macro, so it cannot check the runtime version.

The program, not the library, owns the C header's configuration. It expands the function tables of the header once, with `QAR_IMPLEMENT_DYNAMIC_LOADING()`, and compiles with the same macros as the backend (`QAR_ENABLE_DYNAMIC_LOADING` by default). The public headers of the backend record the macros that the backend was built with, and the MSVC linker refuses a program whose set differs.

```cmake
target_compile_definitions(qar_qt_panel_example PRIVATE ${QAR_QT_STREAMING_SDK_DEFINES})
target_link_libraries(qar_qt_panel_example PRIVATE
  qar_qt_streaming::qar_qt_streaming
  qar_qt_streaming::qaros
  Qt${QAR_QT_MAJOR}::Widgets
)
```

### Threads

The package reaches QAROS only through the seam, and the backend never lets the Qt thread wait. The calls that read or change a GUI panel can wait for the Hub, and a query of the installed runtime takes about two seconds ([How long a query takes](/docs/developer-guide/sdk-limits#how-long-a-query-takes)). The backend therefore runs everything that can wait on one worker, one task after the other in the order they were posted: making the video sender and the panel, finding a panel of an earlier session again, the subscriptions, the placement, the state and the teardown. You can give the factory an executor of your own instead; it must run the tasks in order and off the Qt thread. The Qt thread makes only the three calls of the frame cycle (begin a frame, take the frame buffer, show the frame), and only while the panel exists, and it collects the outcomes of the worker's tasks at each tick. A panel appears in AR a few seconds after the call that asked for it.

When the program ends, reset the stream manager first (it closes or hides the panels), then the factory, then release the program's own copy of the session, and never do that inside a callback of the SDK. The factory's destructor is the one place that blocks the calling thread: it waits for the worker to run the queued tasks for at most `FactoryOptions::shutdown_budget`, 8 seconds by default, and abandons a worker that is still busy. A worker that has not finished by then is left running and counted (`abandoned_workers()`); a host that sees a count above zero releases nothing of QAROS that such a worker may still use and ends its process without releasing the session, the runtime or the library, as the example does.

### Joining the Hub

The package does not join the Hub: your program does, and hands the joined session to the factory. [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) describes the modes. The example program `qar_qt_panel_example`, which streams a window with a button and a panel that is shown off screen and never reaches the desktop, joins the Hub of its own PC as a [local app](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code) (`OnboardBuilder::WithLocalApp()` of the C++ SDK). QAROS asks once to allow the program, and again after each rebuild of it; click **Allow** in the QAROS window. The join stores nothing, and the Hub recognises the program on its next start while it matches what was allowed. The example takes `--local` (the only mode, accepted so that existing commands keep working), `--connect-only`, `--invite CONNECTION`, `--seconds N` and `--version`. To work with a Hub on another PC, make the Hub of this PC a member of it ([Pair another Hub](/docs/user-guide/connection-panel#pair-another-hub)); the program still joins the Hub of its own PC ([Remote app: through a member hub](/docs/developer-guide/onboarding-and-sessions#remote-app-through-a-member-hub)).

The join takes as long as the person at the PC takes to answer, so the example runs it on a thread of its own and polls it from the Qt thread. Ctrl+C in the console ends the program the orderly way, also while the join still waits: the example asks the join to stop and waits up to 5 seconds for it. The SDK documents the effect of a cancel token for its asynchronous calls only, so a synchronous join may keep waiting; if it does, the example ends without it. If the backend's worker is still busy after the shutdown budget, the example ends the process at once and does not release QAROS or unload its library, because releasing the session under that worker could crash the program at its end.

```cpp
qqb::FactoryOptions factory_options;
factory_options.diagnostics = &sink;
factory_options.shutdown_budget = kShutdownBudget;
panel_factory = std::make_shared<qqb::QarosPanelSenderFactory>(joined.session, input, factory_options);

qqs::QtPanelStreamingConfig streaming;
streaming.panel_sender_factory = panel_factory;
streaming.input_source = input;
manager = std::make_unique<qqs::QtPanelStreamManager>(streaming);

qqs::PanelStreamOptions options;
options.placement.pose.position = seam::Vector3{x, 1.5f, -1.0f};   // top centre, room space
manager->add_panel("counter", &widget, options);
```

What the authors ran live, before the package moved onto the pinned header: the example with a local join, with Qt 5.15 and Qt 6.8 on the desktop Visualizer, with taps on the button, the list of the combo box, resizing the window, Ctrl+C, and finding the panels again at the next start. What they did not run live: a headset, and the exit path of the shutdown budget. The authors ran the unit tests of the package with Qt 5.15, 6.5, 6.8 and 6.10.

### Capture

- **Off-screen drawing.** The library draws the widget with `QWidget::render()` into an RGBA8888 buffer on the Qt GUI thread.
- **Change detection and keepalive.** It hashes the pixels and sends a frame only when the picture changed. By default an unchanged picture is sent again after one second, and a panel is captured at most 15 times a second; the host can change both.
- **Size in the room.** By default a panel hangs in the room at 1.2 mm for each logical pixel of the widget, whatever the pixel density of the display: a denser display makes the picture sharper and not larger. The library always sends the content scale 1.0.

### Input

- **Gestures become Qt events.** A tap, a drag or a hover on the panel reaches the widget as touch events first, and as mouse events for a widget that does not take touch, so that buttons, check boxes, sliders and combo boxes work.
- **Pointers.** The library keeps an implicit grab for each pointer, recognizes double clicks, sends the enter and leave events of a hover and treats the two hands of a user as two pointers.
- **Qt 5 and Qt 6.** A standard widget (a button, a slider, a line edit, a tab bar) does not accept touch events, so by default the library offers each pointer sequence as touch first and, when no widget in the chain accepts it, sends mouse events for the rest of that sequence. Under Qt 6 an injected touch point carries the position in the top-level widget and on the screen, and no position of its own in the receiving widget. A program whose widgets read the position of a touch point can set `PanelStreamOptions::input.delivery_mode` to `InputDeliveryMode::MouseOnly`, which sends mouse events only.
- **Status.** The live checks of the example covered taps on a button and on the list of a combo box, not drags. Drags were exercised live through the RViz2 plugin, which builds on the Qt core of this package: on streamed RViz2 dock panels in AR the authors expanded a tree item, dragged a slider and scrolled, and the check passed. The unit tests of the library cover drags and two hands; no live session has exercised two hands.

### Popups and dialogs

- **Popups.** The lists of combo boxes, menus and completers that a panel opens are drawn into its picture where Qt placed them. A popup that hangs over the edge of the panel is cut at the edge and not shifted. A tooltip window is drawn only when it has the panel as its parent. Qt draws nothing when it renders a top-level widget that has an enabled graphics effect, and the `windows11` style of Qt 6.8 gives every combo box list and menu a drop shadow effect; the library therefore switches such an effect off while it draws the widget and on again afterwards, so the list reaches the AR picture without its drop shadow.
- **Whose popup takes AR input.** The injector sends a pointer to a popup that the panel owns, and to no other: the active popup of Qt counts only when its parent chain reaches the root of the panel, or when it is an off-screen popup opened from it. A popup of the host application, such as the menus of RViz2, is never returned, so AR input on a streamed panel can neither move the highlight of a menu of the host nor close it.
- **Dialogs.** A dialog that a panel opens becomes a GUI panel of its own, placed beside the panel that opened it, 0.5 m away by default and a further 0.5 m for each dialog that panel already has (times the larger of the two scales), and it closes when the dialog hides. A scroll bar in an AR panel cannot be used, so a dialog is made to fit its content before its panel is made: a host that installs a dialog size adjuster (`set_dialog_size_adjuster`; the library has a census adjuster that grows the dialog by the overflow of its scroll areas) gets a fit by measurement, and the panel is created once, at the final size. The panel keeps the height of the dialog's video and fits its width to the frame of QAROS; the real window is never moved.
- **Who owns a dialog.** A dialog parented into the tree of a streamed panel belongs to that panel. An application that parents its dialogs to its main window, as RViz2 does, leaves that walk without an answer, so the panel that took the most recent activating AR action (a press, a release or a click, never a hover) within 1500 ms of the first sighting of the dialog owns it. A dialog that the local operator opens with the mouse has no owner and takes no AR input.
- **A modal dialog.** While an application-modal window that the panel owns is up, the pointers on that panel reach nothing: a hover becomes a leave, and a press, a drag and a release are dropped and counted. Delivering an AR click that opens a modal dialog spins a nested event loop inside `QDialog::exec()`; a host whose tick is a `QTimer` therefore sets `pump_input_in_tick` to false and calls `pump_input()` from a posted event, so that the tick goes on while the dialog is open.
- **A scale of its own.** A host that installs a dialog scale provider (`set_dialog_scale_provider`) gets a call on the Qt thread for each dialog that gets a panel, with the key of the opening panel and the dialog, and answers the metres per logical pixel and the supersampling for the dialog's panel; `set_dialog_scale_reporter` is called once the panel exists, with the scale that the stream really has. The provider must not block and the dialog window is never touched.

### Resizing

A widget of a new size needs a new video sender, because the layout of a sender is fixed when the sender is created and the receiver of the panel is allocated from it. A new size is adopted only when it is **significant** and **settled**, so a drag of a splitter does not rebuild anything on the way, and a widget that resizes itself (a status label whose text grows) cannot rebuild the sender for ever:

| Option of `PanelStreamOptions` | Default | Meaning |
|---|---|---|
| `resize_threshold_px` | 8 | a change smaller than this in both dimensions of the buffer is ignored and counted |
| `resize_settle_seconds`, `resize_settle_ticks` | 1.5 s, 3 | the new size must hold for this long and across this many consecutive ticks |
| `resize_wait_for_mouse_release` | true | and then only while no mouse button is held |
| `resize_min_swap_interval_seconds` | 10 | the least time between two rebuilds of the sender of one panel; the newest size wins; 0 means no limit |

The mode decides what a settled size does. By default (the mode `swap`, which `QAR_PANEL_RESIZE=swap` also selects in a program that reads the variable with `parse_panel_resize_mode`) the library releases the old sender and attaches a new sender of the new size to the same QAROS panel, which keeps its name: it gives the panel the new stream with `qar_gui_panels_set_stream` and the new size with `qar_gui_panels_change_size`. The header takes the resolution of the receiver from the sender at that call, but does not promise that a panel accepts a sender of another size; the installed runtime accepted it in the live checks of the authors with the RViz2 plugin (2026-10-25) and with the Qt example. With the plugin's live checks of 2026-11-09 and 2026-11-11, a forced `swap` of a **visible** panel froze RViz2 for about 20 seconds in the first build and 3.7 seconds in a later one, a fault of the GPU driver, so the plugin runs another mode:

- **`new-panel`** makes a second QAROS panel for each size (the name of every panel carries the buffer size as `@<width>x<height>`), hides the old one and keeps it, and shows the kept panel again on a return to its size. No QAROS panel is closed while the session runs, in every close mode, and a visible panel never has its video sender released and made again. A stream keeps at most `max_replaced_panels` hidden panels as a ring (32 for a manager, 8 in the plugin); with the ring full, the next size is made on the oldest hidden panel, whose old sender is detached first. The report of a size change names the fate of the replaced panel as `Revived`, or as `VisibleKept` when the hide was refused. A panel that is shown again takes AR input again, because the stream asks its sender for `ensure_input()`, which moves the subscription of the key to that panel when it names another one. A hide that QAROS refuses leaves the old panel visible and kept, with a warning, and is asked for again three more times, a second apart.
- **`defer`** swaps like the default, but only while the panel is hidden in AR: a visible panel keeps the size of its sender, and a show that is asked for while a size is pending adopts it first.

The header leaves open whether `qar_video_sender_change_layout` reaches a receiver that is already attached, and the library does not use it; see [Changing the size of a video](/docs/developer-guide/sdk-limits#changing-the-size-of-a-video).

### Enlarged controls and widget twins

A panel that is read and touched in AR needs larger targets but not larger text, because the metres that one logical pixel covers already size the text for the person in the room. Scaling the whole widget enlarges both, so the package offers two classes that a host combines.

- **`EnlargedControlsStyle`** is a `QProxyStyle` that draws the check and radio indicators, the buttons of a spin box, the arrows of a combo box, a slider, a scroll bar and a tree, and the arrows of headers and tool buttons `factor` times larger than the base style (the Qt style `fusion` unless the constructor names another), and leaves the text as it is. The factor is held between 1 and 4. `apply_style_to_tree(root, style)` gives the style to a widget, to every widget below it and to every widget made or re-parented below it later; `enlarged_controls_style_of(widget)` tells a window that opens below a styled tree from one that does not. The arrows of a tree, a header and a tool button are drawn as vector shapes, the arrows of the spin box, combo box and scroll bar are drawn at their own size and enlarged smoothly, the rows of a drop-down are at least as tall as the combo box, and the button under the pointer gets a light tint of the highlight colour.
- **`WidgetTwinBinder`** keeps a second widget tree, the twin, showing the state of a first, the original, and carries the input of the twin out on the original. A style belongs to the widgets, so a panel shown with enlarged controls needs a second tree built under the style: the host builds both trees with the same code and names the pairs with `bind(original, twin)`, or `bind_custom` for a class of its own. The original stays the only owner of the state; `sync()` polls it (so it also follows a change made with the signals blocked) and writes only where a value differs, and a click, a choice, a step, a chosen row or an edit on the twin is carried out on the original, so its slots run as for a user of the original. The twin of a `QPlainTextEdit` holds the last 200 blocks of the original. The binder is for the Qt thread and nothing in it waits.

A host scales a panel while it streams with `set_scale()` of a stream or `set_panel_scale()` of the manager: the metres per logical pixel and the supersampling of the capture. The call stores the numbers, and the next tick takes them up, so the Qt thread makes no call into QAROS there. A change of the metres alone leaves the buffer and keeps the sender; a change that gives the buffer another size goes through the resize rules above. The library has no policy for the factor; the host decides the factor, the height limit and when to switch. The RViz2 plugin enlarges the controls 2 times, the text 1.25 times and the whole copy 1.5 times, and limits a copy to 1.8 m ([Running the RViz2 Plugin on Windows](/docs/operator-guide/rviz2-plugin#enlarged-copies-of-qt-panels)).

### Names and the next start

Every panel gets a stable ASCII name, `<key>-<hash>.<program>.<host>.qaros-ros`, made from the key that the host gave the window, the program tag that your program gives the name scope (the example uses `qt-panel-example`) and the name of the PC, and the library asks QAROS for the panel with `qar_gui_panels_get_or_create`. The same name finds the same panel again in the next session. When QAROS returns a panel that an earlier session left behind, the library reads what it is and carries out what is missing: it gives it the new video (`set_stream`), fits its size and pose, and shows it if it was hidden. A panel that shows web content where video is wanted cannot be reused, because the content type is fixed at creation; the library asks again with the suffix `-v2`. A panel whose name is closed is not reused either: the library asks again with `-r1` up to `-r8`. Every attach writes one line, `[qar-panel-adapter] ... panel '<key>' <created|reattached> name=<name> id=<id> state_was=<state> stream_swapped=<y|n> size_changed=<y|n>`. For a panel that exists and is not closed, the call ignores the init and takes the panel over ([A panel or volume that already exists](/docs/developer-guide/sdk-limits#a-panel-or-volume-that-already-exists)); in the live checks of the authors a panel found again was shown in its place.

### Closing

When a Qt window closes, the close mode decides what happens to its AR panel. The default of the library is `destroy`; `parse_panel_close_mode` turns the text of `QAR_PANEL_CLOSE_MODE` into a mode for a program that reads the variable, and the example parks its panels unless the variable names a mode (`destroy`, `notice` or `park`):

| Mode | What happens |
|---|---|
| `destroy` (the default of the library) | The library releases the pointers that are pressed on the panel, cancels its subscriptions, closes the GUI panel with `qar_gui_panels_close_panel` and then destroys the video sender. Opening the window again makes a new panel. |
| `notice` | The AR panel stays and shows a notice that the window closed; its sender is kept until the session ends. This is the escape hatch for a runtime on which destroying a sender is not safe. |
| `park` | Nothing is closed while the session runs. The AR panel is hidden with `qar_gui_panels_set_state`, capture stops and input is dropped, and the panel, the sender and the subscriptions are kept. Adding the same key again shows the parked panel again and creates no new one. |

When the operator closes the panel in AR instead, the library stops capturing and keeps the sender until the Qt window closes or the session ends. [Ending a video stream](/docs/developer-guide/sdk-limits#ending-a-video-stream) says what the header leaves open about the end of a stream.

When the session ends, `destroy` closes the panels with their senders. `notice` and `park` hide every panel, release its sender and its input subscription and leave the panel in the Hub; the next session finds it by its name. If the Hub refuses the hide, the panel stays visible in AR and a warning says so.

### Hidden panels and video

- **Hidden panels.** `show_offscreen` shows a top-level widget to Qt, so that it lays out, paints and takes input, without a window ever reaching the desktop.
- **Video widgets.** `VideoPanelWidget` paints the newest frame of a `LatestVideoFrame`, the one-slot mailbox between a decoding thread and the GUI thread. `decode_raw_image` and `decode_compressed_image` turn raw and compressed camera images into the picture format of the capture, and a region source paints a video frame over a rectangle that `QWidget::render()` cannot paint, such as an embedded native window.

## Where the detail lives

The repository [github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt) documents the installation, the getting-started program, the architecture, the API, the configuration defaults and the troubleshooting of the package. This site does not copy them.

## Related pages

- [Custom GUI Panels](/docs/developer-guide/gui-panels) — creating and observing panels, with web content.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender and the layouts that the video sender shares.
- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries and how they depend on each other.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, and the gesture rules of an app volume.
