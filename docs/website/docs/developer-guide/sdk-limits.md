---
sidebar_position: 11.5
title: Limits and behaviours to design around
description: Where the C API of release 0.9.0 offers no call or leaves a question open — retiring a video stream, resizing one, locating the operator — and the gesture rules to know before you build on it.
---

# Limits and behaviours to design around

This page collects points where the C API offers no call or leaves a question open, so that you design around them from the start. Each item rests on the header `qar_streaming.h` of release 0.9.0 and, where it says so, on the documentation comments of the C# binding, and it says what these state and what they do not. The last section reports observations that were made with earlier SDK drops.

:::info Which SDK generation
All sections except the last describe the header and the C# binding of release 0.9.0. The last section reports what the authors of the [Ogre, Qt and RViz2 libraries](/docs/developer-guide/ogre-qt-rviz) observed in live sessions with earlier SDK drops, which they have not repeated on every drop. The C API of 0.9.0 is a later generation, and this page does not claim that the observations hold with it. The document `docs/SDK_COMPATIBILITY.md` of each of those repositories lists what differs between the generations.
:::

## Ending a video stream

A **video sender** (`qar_video_sender_create`) owns a logical video stream with a library-generated stream id, and a stream panel shows that stream. The C header declares no call that stops, retires or withdraws a stream, apart from destroying the sender:

- `qar_video_sender_handle_destroy` destroys the sender handle and returns `void`. There is no status to check, and the C header does not say what a receiver of the stream sees. The C# binding documents `PrimarylessVideoSender.Dispose` as closing the stream to every receiver after the frames that are still being sent have drained.
- `qar_gui_panels_close_panel` closes and removes the panel. The header does not say that the stream behind the panel ends.

Plan for this:

- Treat the panel and the sender as two lifetimes. Do not assume that closing a panel ends the stream behind it; the header describes only the panel. Destroy the sender when you no longer need the stream.
- Do not build on a receiver learning that its stream has ended: destroying the sender returns nothing that you can check, and the header describes no notification (see the last section of this page).
- To stop showing a video, close its panel. To show a different video in an existing stream panel, give it the other stream with `qar_gui_panels_set_stream`.

## Changing the size of a video

The resolution of a stream panel comes from the layout of its sender: the header says that the panel receiver is allocated from the layout of the sender that the stream id names, and that `qar_gui_panels_set_stream` takes the resolution from the current layout of the sender it is given. `qar_video_sender_change_layout` has the one-line comment "Retrieve or change the CPU ingress layout" and nothing more. Unlike `qar_render_sender_change_layout`, which is documented as a request for a layout change for subsequent frames, it does not say what happens to a receiver that is already attached. The C# binding documents `PrimarylessVideoSender.ChangeLayout` for the sender's own frame only: the new frame resources are created on the calling thread, the previous layout stays usable if that fails, and texture handles cached from an earlier frame do not survive a successful change.

Plan for this:

- Fix the layout of a video sender before you create the panel for it, and treat it as fixed.
- When the size must change, create a new sender with the new layout and give the panel the new stream with `qar_gui_panels_set_stream`, or create a new panel for it. The header says that the layout of the new sender determines the receiver resolution; it does not describe how the swap looks to a viewer.
- `qar_gui_panels_change_size` changes the size of the panel in metres and its content scale. The header takes the receiver resolution from the sender, not from the panel size.

## Where the operator is

No call asks where the operator stands or looks. The header returns the pose of a viewer only with a render sender:

- `qar_render_frame_info_get_view_pose` gives the pose of a view, and for a view of eye `QAR_VIDEO_FRAME_VIEW_EYE_NONE` the head pose, in the app content space of the app volume that the sender is bound to.
- `qar_render_sender_last_hands` returns the newest hand sample of the same sender, also in app content space. Both hands are untracked unless the target peer is focused on the volume and not editing it, and the volume resolves.

The other pose getters of the header return the pose of a GUI panel or of an app volume, and a peer spec carries no pose. An application that only shows GUI panels has no render sender, so it cannot know where the operator is. Place its panels at positions that do not depend on the operator, or take the viewer pose from a render sender that the application already runs.

## What a stream panel ignores or refuses

A GUI panel shows either web content or a video stream, chosen at creation with `QarGuiPanelUrlExt` or `QarGuiPanelStreamIdExt` (see [Custom GUI Panels](/docs/developer-guide/gui-panels)). A stream panel differs from a web panel in three ways:

- **It ignores the content scale.** For a web panel, `QarGuiPanelSize::content_scale` is the zoom of the page. A stream panel always draws its video at the full panel width, and the renderer scrolls the video vertically when it is taller than the panel.
- **It refuses navigation.** `qar_gui_panels_navigate_to_uri` on a stream panel returns `QAR_STATUS_GUI_PANEL_CHANGE_NOT_PERMITTED`, because the content type of a panel is fixed at creation. A web panel returns the same status from `qar_gui_panels_set_stream`.
- **It reports positions in video pixels.** A position that a gesture reports on a panel (`QarGuiPanelPoint`) is in panel content space. For a stream panel that is the pixel raster of the decoded video image: the origin is the top-left corner as the viewer sees it, +X runs right and +Y runs down.

## Gesture rules of an app volume

A peer's gestures against an app volume are mapped by the `QarAppVolumeGestureConfiguration` of the volume. The kinds are `QAR_GESTURE_CLICK`, `QAR_GESTURE_HOVER`, `QAR_GESTURE_SINGLE_POINTER_6DOF`, `QAR_GESTURE_DUAL_POINTER_TRANSLATE_DISTANCE` and `QAR_GESTURE_DUAL_POINTER_ROTATE`. [App Volumes and Gestures](/docs/developer-guide/app-volumes#gestures) shows how to configure and consume them; these are the rules to know on top of that.

- **The order of the list is the priority.** `mapping_rules` is in descending priority: element 0 has the highest. When several gestures are active for the same volume and the same source peer, the runtime applies only the active rule of the highest priority and ignores the rules of lower priority until that gesture ends, is cancelled or becomes inactive. A configuration holds at most 16 rules (`QAR_MAX_APP_VOLUME_GESTURE_MAPPING_RULES`).
- **One peer manipulates at a time.** Whatever the allowed peers, the first peer whose manipulation binds holds the volume until that manipulation ends. The manipulations of other peers are refused meanwhile, while their hover and click still arrive. `qar_app_volume_get_gesture_holder` tells you which peer holds the volume.
- **You can restrict who may manipulate.** Chain `QarAppVolumeGestureAllowedPeersExt` from the configuration. Without it, or with an empty list, every peer's gestures may move and scale the app. A gesture from a peer that is not in the list still reaches your application, unmapped. A configuration is replaced whole, so chain the extension every time you change the configuration.
- **A rule that could never work is refused.** `QAR_STATUS_APP_VOLUME_GESTURE_CONFIGURATION_INVALID` comes back for an enabled rule for `QAR_GESTURE_DUAL_POINTER_ROTATE` that moves the app while `rotation_axes` is `QAR_APP_VOLUME_AXIS_NONE`: it would take two-hand gestures over and turn nothing. The volume keeps its previous configuration, and adding a volume with such a configuration fails the same way. Removing the default gesture mode returns the same status.
- **Modes switch whole rule sets.** A gesture mode is one set of mapping rules. `QAR_APP_VOLUME_GESTURE_MODE_DEFAULT` always exists, cannot be removed, and every peer is in it until it is switched with `qar_app_volumes_select_gesture_mode`. A gesture that is already running keeps the mode it began in.

### The default configuration

The header says that `qar_app_volume_gesture_configuration_default` mirrors the configuration that a volume created without a gesture configuration gets, so `gesture_configuration = NULL` gives you these four rules, in this order of priority:

| Priority | Gesture kind | Mapping | Ray |
|---|---|---|---|
| 1 | `QAR_GESTURE_DUAL_POINTER_TRANSLATE_DISTANCE` | the scale of the app, sensitivity based on the app scale, no rotation axes | green |
| 2 | `QAR_GESTURE_SINGLE_POINTER_6DOF` | the pose of the app, no rotation axes | green |
| 3 | `QAR_GESTURE_HOVER` | none | dotted, white |
| 4 | `QAR_GESTURE_CLICK` | none | solid, light green |

Two hands change the scale of the app, one hand moves it, and hover and click are only reported, each with its own ray.

## Observed in live sessions with earlier SDK drops

These two observations come from live sessions with earlier SDK drops and have not been repeated on every drop. The header of release 0.9.0 describes neither of them, and this page does not claim that they hold with 0.9.0.

- **Receivers keep retrying a stream whose sender was destroyed.** A receiver that was attached to such a stream was not told that the stream was gone. It entered a reconnecting state and retried about every ten seconds, each retry failed with the status 507, and this went on until the peer of the receiver went away.
- **The viewport of a URL panel is fixed.** The page of a URL panel saw a viewport of 500 by 500 CSS pixels at a device pixel ratio of 2.0, whatever the size of the panel. The header documents the page zoom (`content_scale`) but no way to configure a viewport: `QarGuiPanelUrlExt` carries only the URL. Measure the viewport in your page instead of assuming it from the size of the panel.

## Related pages

- [Custom GUI Panels](/docs/developer-guide/gui-panels) — creating, changing and observing panels.
- [Rendering Streams](/docs/developer-guide/rendering-streams) — the render sender, its frame loop and layouts.
- [App Volumes and Gestures](/docs/developer-guide/app-volumes#gestures) — configuring and consuming gestures.
- [Troubleshooting](/docs/developer-guide/troubleshooting) — status codes and frequent integration mistakes.
- [Video in GUI panels, and Qt widgets as AR panels](/docs/developer-guide/qt-ar-panels) — showing a video in a panel, and the Qt library built on it.
