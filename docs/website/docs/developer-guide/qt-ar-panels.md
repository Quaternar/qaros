---
sidebar_position: 7.5
title: Video in GUI Panels, and Qt Widgets as AR Panels
description: How a GUI panel shows a video stream, what a stream panel accepts, and what the qar_qt_streaming library does to show a Qt widget in AR.
---

# Video in GUI Panels, and Qt Widgets as AR Panels

A GUI panel can show a **video stream**, so any picture that you can draw becomes a panel: a camera image, a rendered view, a Qt widget. This page has two parts. The first describes the video route of the C API. The second describes how the library `qar_qt_streaming` uses that route to turn a `QWidget` into a panel.

:::info Which QAROS runtime
The first part describes the header and the C# binding of the C API that this guide documents. The second part describes the library `qar_qt_streaming`, which was built and checked against the installed QAROS runtime **0.9.0-dev.3008+65dc9aeb**, a development build of the setup that precedes the 0.9.0 release. The library does not join the Hub itself: the session owner of `qar_ogre_adapter` does, as a local app on the PC of the Hub or by code pairing with a Hub on another PC ([Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions)), and the library borrows its session. The document `docs/SDK_COMPATIBILITY.md` of the repository lists the runtime that the library supports.
:::

## Show a video in a GUI panel

A GUI panel shows either web content or the video of a video sender, and the choice is made when the panel is created: chain `QarGuiPanelUrlExt` or `QarGuiPanelStreamIdExt` into `QarGuiPanelInit` (see [Custom GUI Panels](/docs/developer-guide/gui-panels)). To show a video:

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

`qar_qt_streaming` is the video route for Qt widgets. The library draws a `QWidget` into a pixel buffer with the paint path of Qt, streams the buffer as the video of a GUI panel and turns the gestures that QAROS reports on the panel into Qt events, so that a widget that works on the desktop works in AR. It needs Qt 5.15.2 and builds on `qar_ogre_adapter`; [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) says how the libraries fit together and what to install.

The library never calls the C API itself. The `panels` component of `qar_ogre_adapter` implements the interfaces that it uses: it creates the video sender, the GUI panel and the input subscription, and the library supplies the frames and takes the pointer samples. The calls that read or change a GUI panel can wait for the Hub, and a query of the installed runtime takes about two seconds ([How long a query takes](/docs/developer-guide/sdk-limits#how-long-a-query-takes)). The component therefore queues them for the thread of the session owner, which runs them in order, and the Qt thread picks up the results at the next tick: no QAROS call that may wait runs on the Qt thread, and a panel appears in AR a few seconds after the call that asked for it.

### Capture

- **Off-screen drawing.** The library draws the widget with `QWidget::render()` into an RGBA8888 buffer on the Qt GUI thread.
- **Change detection and keepalive.** It hashes the pixels and sends a frame only when the picture changed. By default an unchanged picture is sent again after one second, and a panel is captured at most 15 times a second; the host can change both.
- **Size in the room.** By default a panel hangs in the room at 1.2 mm for each logical pixel of the widget, whatever the pixel density of the display: a denser display makes the picture sharper and not larger. The library always sends the content scale 1.0.

### Input

- **Gestures become Qt events.** A tap, a drag or a hover on the panel reaches the widget as touch events first, and as mouse events for a widget that does not take touch, so that buttons, check boxes, sliders and combo boxes work.
- **Pointers.** The library keeps an implicit grab for each pointer, recognizes double clicks, sends the enter and leave events of a hover and treats the two hands of a user as two pointers.
- **Status.** The unit tests of the library cover drags and two hands; no live session has exercised them yet.

### Popups and dialogs

- **Popups.** The lists of combo boxes, menus and completers that a panel opens are drawn into its picture where Qt placed them. A popup that hangs over the edge of the panel is cut at the edge and not shifted. A tooltip window is drawn only when it has the panel as its parent.
- **Dialogs.** A dialog that a panel opens becomes a GUI panel of its own, sized from the dialog and placed beside the panel that opened it, 0.5 m away by default and a further 0.5 m for each dialog that panel already has, and it closes when the dialog hides.

### Resizing

A widget of a new size needs a new video sender, because the layout of a sender is fixed when the sender is created and the receiver of the panel is allocated from it. By default (`QAR_PANEL_RESIZE=swap`) the library releases the old sender and attaches a new sender of the new size to the same QAROS panel, which keeps its name: it gives the panel the new stream with `qar_gui_panels_set_stream` and the new size with `qar_gui_panels_change_size`. The header takes the resolution of the receiver from the sender at that call, but does not promise that a panel accepts a sender of another size; the installed runtime accepted it in the live checks of the authors (2026-10-25), in which a resized dock was followed by its AR panel. With `QAR_PANEL_RESIZE=new-panel` the name of every panel carries the buffer size (`@<width>x<height>`), a size change makes a second QAROS panel, the old one is hidden and kept, and a return to a size that is kept shows the kept panel again. To avoid a rebuild for every pixel of a drag, the library adopts a new size by default only when the size differs by at least 8 pixels and has held for half a second and for three ticks. The header leaves open whether `qar_video_sender_change_layout` reaches a receiver that is already attached, and the library does not use it; see [Changing the size of a video](/docs/developer-guide/sdk-limits#changing-the-size-of-a-video).

### Names and the next start

Every panel gets a stable ASCII name, `<key>-<hash>.<program>.<host>.qaros-ros`, made from the key that the host gave the window, the program tag of the session owner and the name of the PC, and the library asks QAROS for the panel with `qar_gui_panels_get_or_create`. The same name finds the same panel again in the next session. When QAROS returns a panel that an earlier session left behind, the library reads what it is and carries out what is missing: it gives it the new video (`set_stream`), fits its size and pose, and shows it if it was hidden. A panel that shows web content where video is wanted cannot be reused, because the content type is fixed at creation; the library asks again with the suffix `-v2`. A panel whose name is closed is not reused either: the library asks again with `-r1` up to `-r8`. Every attach writes one line, `[qar-panel-adapter] ... panel '<key>' <created|reattached> name=<name> id=<id> state_was=<state> stream_swapped=<y|n> size_changed=<y|n>`. What QAROS applies to an existing panel is not documented ([A panel or volume that already exists](/docs/developer-guide/sdk-limits#a-panel-or-volume-that-already-exists)); in the live checks of the authors a panel found again was shown in its place.

### Closing

When a Qt window closes, `QAR_PANEL_CLOSE_MODE` decides what happens to its AR panel:

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

The repository [github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt) documents the installation, the getting-started program, the architecture, the API, the configuration defaults and the troubleshooting of the library. This site does not copy them.

## Related pages

- [Custom GUI Panels](/docs/developer-guide/gui-panels) — creating and observing panels, with web content.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender and the layouts that the video sender shares.
- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries and how they stack.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, and the gesture rules of an app volume.
