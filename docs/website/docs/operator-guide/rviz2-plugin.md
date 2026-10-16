---
sidebar_position: 4.6
title: Running the RViz2 Plugin on Windows
description: Start RViz2 with the QAROS plugin on a Windows 11 workstation, connect it to a Visualizer, and bring the live 3D scene, RViz2 panels, ROS 2 graph tools, video and picks into AR.
---

# Running the RViz2 Plugin on Windows

The **RViz2 plugin** `qar_rviz_streaming` brings a ROS 2 system into AR from a Windows 11 workstation. It adds one panel, **Qar Streaming**, to RViz2. For every admitted peer the plugin renders the live 3D scene of RViz2 from the head pose of that peer and streams it as stereo video. It also shows RViz2 panels as GUI panels in AR, generates AR tools from the topics, services and actions of the ROS 2 graph, shows ROS 2 image topics as video panels, and publishes the picks of the headset on the topics of the RViz2 tools.

The installation, the configuration reference and the log reference stay in the repository [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz); this page names the documents and does not copy them. [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) describes the libraries that the plugin builds on.

:::info Which SDK generation
This page describes the plugin as it was built and verified against the QAROS SDK drop of 2026-09-28. With that drop the plugin creates a session and invites a Visualizer. Release 0.9.0, which the rest of this guide documents, is a later generation of the C API and replaces that session start-up with onboarding to the Hub. The plugin has not been rebuilt against release 0.9.0. The sections that depend on the session start-up say so. Read them for what the plugin does today, and read [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) and [Running the Hub](/docs/operator-guide/running-the-hub) for release 0.9.0. The document `docs/SDK_COMPATIBILITY.md` of the repository lists the supported SDK drops, how to identify the drop you have and what differs in the later generation.
:::

## What you need before you start

The document `docs/INSTALL.md` of the repository installs everything below from a clean Windows 11 machine, step by step. You need:

- **Windows 11, 64-bit, at the console of the machine.** RViz2 and the Visualizer open OpenGL windows, and the documented runs were made from the local console; Remote Desktop and services are not covered. Linux is not built or tested.
- **An NVIDIA GPU with a current OpenGL driver.** The C API currently supports NVIDIA adapters only, as its header states in the drop of 2026-09-28 and in release 0.9.0 alike, and no minimum GPU or driver version is stated. On a laptop with an integrated and a discrete GPU, RViz2 should render on the adapter that QAROS uses, or every picture passes through a copy between the adapters, so the live launcher sets the graphics preference of `rviz2.exe` to high performance.
- **PowerShell 7.** The scripts and every command on this page run in `pwsh`.
- **The ROS 2 Humble binary distribution for Windows, repaired.** The plugin runs inside the `rviz2.exe` of the official binary release. That release has gaps: some runtime libraries are missing, the C++ runtime beside `rviz2.exe` is too old for the QAROS library, and `tinyxml.dll` is missing. The repository ships the scripts that repair the distribution in place, and the plugin needs all nine repairs.
- **The plugin and the libraries it builds on.** `colcon` builds the plugin, and the installation puts it into one prefix together with `qar_ogre_adapter` ([github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)) and `qar_qt_streaming` ([github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)), on Qt 5.15.2.
- **The QAROS SDK drop of 2026-09-28.** It includes the Visualizer. You get a drop from Quaternar; the repository does not contain it. Keep the `bin` folder of the SDK whole and unmodified, because QAROS starts its own runtime launcher (`qar-runtime-launcher.exe`) and Visualizer (`qar-streaming-viz.exe`) from that folder.
- **One local network.** The documented runs put the machine and its devices on one local network, and the discovery of QAROS with this drop is for local networks only. QAROS has its own ports, and they differ between releases, so this page names none of them; [Networking & Federation](/docs/operator-guide/networking-and-federation) lists those of release 0.9.0.
- **UDP port `7446` free (session start-up).** With this drop QAROS uses it for discovery and cannot create a session while another program holds it. ROS 2 programs on domain 0, the default when `ROS_DOMAIN_ID` is not set, can take it by accident: each ROS 2 participant on that domain takes a pair of UDP ports in steps of two upward from `7410`, so after about nineteen participants, for example test publishers that were never stopped, one of them holds `7446`.

This command prints the programs that hold the port, or says that it is free. It changes nothing:

```powershell
$holders = Get-NetUDPEndpoint -LocalPort 7446 -ErrorAction SilentlyContinue
if ($holders) {
    $holders | ForEach-Object { 'UDP 7446 is held by process {0} ({1})' -f $_.OwningProcess, (Get-Process -Id $_.OwningProcess -ErrorAction SilentlyContinue).ProcessName }
} else {
    'UDP 7446 is free'
}
```

## Starting RViz2 with the plugin

### The launcher scripts

The `scripts` folder of the repository holds two launchers. Run them in PowerShell 7, from the folder of your clone of the repository.

| Script | What it does |
|---|---|
| `scripts\run_live_rviz_stream_real.ps1` | Starts RViz2 with the panel in **live mode**, so that the panel can connect to QAROS. Without `-Confirm` it only prints what it would do and starts nothing. |
| `scripts\run_rviz_with_plugin.ps1` | Starts RViz2 with the panel in the **offline preview**: simulated peers replace QAROS, and nothing is sent to it. It has no dry run and starts RViz2 at once. |
| `scripts\env_setup.ps1` | Starts nothing. Dot-source it, with a leading dot and a space, to prepare a PowerShell window with the environment of the ROS 2 distribution and Qt, for the ROS 2 commands that you run next to RViz2. |

### Check what the launcher will do, then start it

Run the live launcher without `-Confirm` first. It takes the locations from the parameters `-SdkRoot`, `-Prefix`, `-QtRoot` and `-Ros2Root` or, for a parameter that you omit, from the environment variables `QAR_SDK_ROOT`, `QAR_PREFIX`, `QAR_QT_ROOT` and `ROS2_INSTALL_ROOT`. It prints the scene, the SDK programs that it found, the settings that it would use and the logging mode. It starts nothing, changes nothing and, when it finds all four locations, ends with exit code `0`:

```powershell
.\scripts\run_live_rviz_stream_real.ps1
```

Then start RViz2:

```powershell
.\scripts\run_live_rviz_stream_real.ps1 -Confirm
```

The command starts RViz2 and returns; it does not wait for RViz2 to end. With `-Confirm` the launcher does these things:

- It stops, before it changes or starts anything, when the C++ runtime beside `rviz2.exe` is older than version `14.40` or `tinyxml.dll` is missing there, and names the repair script to run.
- It writes the Windows graphics preference of `rviz2.exe` (high performance) for your user, in the registry under `HKCU`, unless it is set to high performance already.
- It renames each QAROS log file over 256 MB by adding `.prev` to its name.
- It ends the test publishers of earlier runs: the Python processes whose command line names `publish_test_markers.py`, `publish_test_tf.py`, `publish_test_robot.py` or `publish_test_pointcloud.py`, also when you started them yourself in another window.
- It cleans up the Fast-DDS shared memory of programs that have ended.
- It starts `scripts\publish_test_markers.py` in a hidden window, so that the scene has markers to show, and prints its process id. The publisher keeps running after RViz2 closes: stop it with `Stop-Process` and that id, or let the next run end it.
- It warns when UDP port `7446` is held, and it starts `rviz2.exe` with the default scene `scene3_grid_marker.rviz`, which holds a grid and the markers of the test publisher.

The launcher does not start the QAROS runtime launcher or the Visualizer. With this drop the QAROS library starts them from the folder in `QAR_RUNTIME_BINARIES_DIR`, which the launcher sets to the `bin` folder of the SDK: the runtime launcher when you connect and a Visualizer when you invite one, as [Connecting (session start-up)](#connecting-session-start-up) says. These parameters matter most:

| Parameter | Effect |
|---|---|
| `-Scene` | The RViz2 configuration to load, instead of the default test scene. |
| `-LogDir` | The folder of the RViz2 logs. The default is `qar_rviz_logs` in your `TEMP` folder. |
| `-TraceLogs` | Collects the full QAROS and adapter logs that a bug report needs. They can grow to gigabytes, so do not judge stalls in this mode. |
| `-TimingCsv` | Records the timing of every video stage in CSV files. |
| `-Visualizers`, `-InvitePeers`, `-RealPeers` | Set which peers the plugin invites when it connects and which peers it admits (session start-up). |

The document `docs/CONFIGURATION.md` lists every parameter and every environment variable.

### Open the panel

The scene of the launcher loads the panel. RViz2 opens with the panels Displays, Views and **Qar Streaming**, and the panel starts in the state `Qar runtime: STOPPED`. It connects to nothing until you press a button. An RViz2 configuration of your own loads the panel with this entry in its `Panels` list:

```yaml
Panels:
  - Class: qar_rviz_streaming/QarStreamingPanel
    Name: Qar Streaming
```

If the panel is missing, either the shell that started RViz2 did not source the overlay of the install prefix (`local_setup.ps1`) or the plugin is not installed into that prefix. A launcher sources the overlay itself, and `docs/TROUBLESHOOTING.md` has the checks. If you start `rviz2.exe` without a launcher, you have to set the variables that the launcher sets. `QAR_PANEL_LIVE_STREAM` comes first, because without it the panel starts in the offline preview. The section "Running RViz without the launcher" of `docs/CONFIGURATION.md` lists them.

### The Qar Streaming panel

The panel is a vertical stack of six groups: **Qar runtime**, **Stream settings**, **Peers**, **AR GUI panels**, **Spatial AR interaction** and **Diagnostics**. The sections below name the controls that you use, and the document `docs/OPERATOR_GUIDE.md` lists every control.

Three buttons change their labels with the mode. In live mode they read **Connect to QarOS**, **Disconnect (pause)** and **Invite peer**. In the offline preview the same buttons read **Start Qar**, **Stop Qar** and **Add simulated peer**.

## Connecting (session start-up)

The plugin builds its QAROS session when you press **Connect to QarOS**, never when the panel loads. This section describes the session start-up of the SDK drop of 2026-09-28. Release 0.9.0 replaces it with onboarding to the Hub: see [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) and [Running the Hub](/docs/operator-guide/running-the-hub).

1. **Connect.** Press **Connect to QarOS**. The plugin builds the session on a worker thread, so that RViz2 stays responsive, and this takes a few seconds (about 2 seconds in the logged sessions). While the session builds, the state label reads `Qar runtime: CONNECTING...`; when it is open, the label reads `Qar runtime: RUNNING (live)`. If the attempt fails, the label can keep saying `CONNECTING...`, and `Last error:` shows the reason; press the button again.
2. **Invite.** The connection invites nothing by default, so no Visualizer opens until you ask for one. Choose a kind in the drop-down, type a host name if the kind needs one, and press **Invite peer**. Press it only when the label reads `Qar runtime: RUNNING (live)`: before the connection, and after **Disconnect (pause)**, the same button adds a simulated peer instead of inviting a device.
3. **See the scene.** The Visualizer asks for a stream, and the plugin creates the sender for it in the background, so that RViz2 stays responsive. The log line `LIVE: opened stream for peer` says that the stream is open; in the logged sessions it came about 5 seconds after the invitation.

The drop-down offers the four kinds of device that QAROS can invite with this drop: **Visualizer**, a QAROS Visualizer that QAROS starts on this machine; **HoloLens 2**, which needs its host name, because it is the only kind that QAROS can address by name; **Meta Quest**; and **ZED camera**.

The invitation is a call into QAROS on the GUI thread of RViz2, so RViz2 does not repaint while it runs; press the button once and wait. Each invitation of a Visualizer carries a fresh identifier by default, so every invitation opens a Visualizer of its own, and one Visualizer is one head.

**Which peers get a stream.** A **peer** is a device or an application that receives a stream, and each admitted peer gets its own stereo stream of the scene. QAROS also reports the internal peers of its own runtime, so the plugin admits by default only the peers that it invited itself (`QAR_PANEL_PEER_ADMISSION=invited`). A Visualizer that you started by hand, or a device that joins by itself through onboarding, is not admitted by default. For a demo with such devices, start the launcher with `-RealPeers`, and the plugin also admits any peer that first appears more than 5 seconds after the session is up. That rule depends on timing, so inviting is more reliable.

By default the plugin serves at most 8 peers at once and refuses a further peer with a logged warning, because every peer costs a color and a depth render of the scene for each eye in every frame.

**Invitations at connect.** The launcher parameters `-Visualizers` and `-InvitePeers` make the plugin invite Visualizers and devices as soon as it connects, and `docs/CONFIGURATION.md` describes them.

**Pause and end.** **Disconnect (pause)** stops the streaming of the 3D scene and keeps everything else: the session stays open, and the AR panels and AR tools stay open and keep running. It pauses and does not disconnect. **Connect to QarOS** resumes the streaming on the open session. The session ends when RViz2 closes.

## What you see and do in AR

### The 3D scene

The plugin renders the scene once for each peer, from the head pose that QAROS reports for that peer, and sends a color picture and a depth picture for each eye. The view in the Visualizer follows the pose of the Visualizer and not the camera of RViz2: orbiting the camera of RViz2 changes only the picture on your desktop. To make the scene readable in AR, the plugin adds an opaque gray plane 1 mm under every Grid display, so that the depth picture shows a solid floor instead of gaps between the grid lines. While it streams, the plugin lights the scene with a rig that is fixed in the world and suppresses the headlight of RViz2, so that the shading does not swing when you orbit the view.

**Resolution:** and **FPS:** start at 1280 x 720 and 60 frames per second. They apply to the streams that are created from then on and do not resize a stream that exists. The plugin renders all streams that are due in one batch, with a budget of about 16 ms, and sheds the frames of the peers beyond it, except that a peer that has not been rendered for 1/30 s is always served. Choose a lower resolution when you serve several headsets.

The launcher sets `QAR_POSE_WORLD` to `zup` and `QAR_DEPTH_ENCODING` to `ndc`. Without the first the scene arrives on its side, and without the second the Visualizer showed the depth in one color with an SDK drop earlier than that of 2026-09-28. If you start RViz2 without the launcher, set both.

The plugin also reads the fixes of a `sensor_msgs/msg/NavSatFix` topic, `/gps/fix` unless the environment names another, and tells QAROS from them where the origin of the scene lies on Earth. This path does nothing while the topic is silent.

### RViz2 panels in AR

The group **AR GUI panels** streams a window of RViz2 as a **GUI panel**. The drop-down lists **This QAR panel**, every dock of RViz2 by its title, one entry `Video: <topic>` for each video panel that an AR tool opened, and the hidden Displays copy. Press **Refresh** after you add a dock. Choose an entry and press **Stream panel to AR**; **Stop streaming** ends the stream.

- **Placement.** The first panel hangs 1.5 m high and 1 m in front of the origin of the room, and each further panel hangs 0.5 m further along the x axis. No call of QAROS asks where the operator is, so the plugin cannot place a panel in front of you ([Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#where-the-operator-is) explains why).
- **Input.** A click and a hover on the panel reach the real widget of RViz2, including the lists of drop-down boxes and the dialogs that the widget opens. A dialog becomes a panel of its own, beside the panel that opened it. A panel is captured at up to 15 frames per second, and at up to 30 when its dock holds an Image or Camera display.
- **Closing.** Closing the dock of a streamed window closes its AR panel and, by default, destroys its video sender. Showing the dock again does not restart the stream: press **Stream panel to AR** again. If you close an AR panel in AR, the plugin stops capturing it and marks it `CLOSED IN AR` in the readout `AR panels:`.
- **Pauses.** The press on **Stream panel to AR** makes no call into QAROS, but the QAROS panel is created on the next render tick with synchronous calls, so RViz2 can stand still for a moment then. The same holds when a streamed panel closes.

When the connection is up, the plugin also makes a second instance of the Displays panel, shows it off-screen only, so that it never appears on your desktop, and streams it into AR as `Displays (AR)`, 480 by 720 pixels, at x = -1.2 m, y = 1.5 m, z = -1.0 m. It views the same display tree as the real Displays dock, so a check box that you toggle in AR, a property that you edit and a display that you add, remove or rename in the headset change the displays of RViz2. `QAR_PANEL_HIDDEN_DISPLAYS` set to `off` stops the automatic start on connect, and the last entry of the drop-down still starts the copy by hand.

### AR tools from the ROS 2 graph

An **AR tool** is a panel that the plugin generates from a topic, a service or an action of the live ROS 2 graph. It is a web page that a server in RViz2 serves and that QAROS opens as a URL panel. The plugin needs no generated message code: it reads the layout of a type from the introspection typesupport of its interface package at run time.

1. Press **Add from ROS graph...**. The dialog `Add from the ROS graph` lists the topics, services and actions of the graph as trees by namespace, with their types. **Filter:** narrows the list by a part of a name or of a type, and **Refresh** asks the graph again.
2. Select an entry. The lines under the tree say what **Add** will do before you press it. For a topic, **Add** creates the RViz2 display that accepts the type and pins the topic to the list of watched topics.
3. Tick **Expose as a QAR AR tool** to generate the AR panel. It is always on for a service and an action, which RViz2 has no display for. An exposed image topic gets no RViz2 display at all.
4. Tick **Allow publishing from AR** only when the tool has to publish. The box exists for topics only and is off by default, because a publication to a topic is a command to a robot. The buttons that command a robot, `Publish` and `Send goal`, ask for a second press within 4 seconds and read `Press again to confirm`. `Call service`, `Cancel goal` and the `STOP` button of a velocity command, which publishes a zero command, act on the first press.

The first tool hangs 1.5 m high and 1.2 m in front of the origin, and each further tool hangs 0.7 m further to the left. A row of the list of tools shows the kind, the name and the type, followed by `[in AR]` when its QAROS panel exists, `[pending]` when it does not yet, and `[writable]` for a tool that may publish. The buttons **Remove tool**, **Re-open in AR** and **Copy URL** act on the selected row. A tool that you expose before you connect is served at once but stays `[pending]` until the connection is up.

The plugin picks one of 17 layouts for the type of a tool, for common types such as velocity commands (`Twist`), poses, battery state, odometry, maps, laser scans, images and GPS fixes (`NavSatFix`), and a type that no layout claims gets a generic form with one widget for each field. The page of a topic shows the newest message and refreshes at 5 Hz, however fast the topic is.

:::caution The pages are served over plain HTTP
The server of the tools listens on all network interfaces, so that a headset on the network can reach it, on a port that the operating system picks. A page is reached at `http://<host>:<port>/p/<token>/`. The traffic is not encrypted, and the token of 128 random bits in the address is the only protection; a wrong or missing token gets a 404. `QAR_PANEL_WEB_PORT` pins the port, for a firewall rule, and `QAR_PANEL_WEB_HOST` sets the host name that the addresses contain.
:::

To try the tools without a robot, run `python scripts\publish_template_topics.py` in a PowerShell window that you prepared with `. .\scripts\env_setup.ps1` and that uses the same `ROS_DOMAIN_ID` as RViz2. It publishes ten topics and one `Trigger` service under `/qar_demo`, which you find by typing `qar_demo` in **Filter:**, and it ends by itself after 1500 seconds.

### Video panels

A ROS 2 image topic reaches AR in two ways. The first is the button `Open video in AR` on the page of an exposed `sensor_msgs/msg/Image` or `sensor_msgs/msg/CompressedImage` topic: it opens a new AR panel, titled with the topic, which appears in the drop-down as `Video: <topic>`. The second is a dock of RViz2 that holds an Image or Camera display: press **Stream panel to AR** on that dock, and the plugin subscribes to the topic of the display and streams the video into the AR panel directly.

The plugin sends the video at up to 30 frames per second. It decodes the usual raw encodings of `sensor_msgs/msg/Image` and the PNG or JPEG data of `sensor_msgs/msg/CompressedImage`, and it scales a larger frame down to fit 1280 by 1280 pixels. Large video topics need the Fast-DDS profile `scripts\fastdds_large_data.xml` at both ends of the connection. The launcher exports it unless you pass `-NoDdsProfile` or set `FASTRTPS_DEFAULT_PROFILES_FILE` yourself, and `scripts\publish_video_file.py` applies it to its own process.

### Picks

The group **Spatial AR interaction** turns the pointer of the headset into ROS 2 messages. **Mode** chooses what a pointer means in the 3D scene, and it starts at `Off`, so that a gesture can never move a robot because a mode was left on. **Target** chooses what the pointer is resolved against, and **Frame** names the frame in the header of every published pick (`map` at the start).

| Mode | Gesture | Topic | Message type |
|---|---|---|---|
| `Pick a point -> /clicked_point` | A click. | `/clicked_point` | `geometry_msgs/msg/PointStamped` |
| `Set a navigation goal -> /goal_pose` | Press to fix the position, drag to set the heading, release. | `/goal_pose` | `geometry_msgs/msg/PoseStamped` |
| `Set the initial pose -> /initialpose` | The same. | `/initialpose` | `geometry_msgs/msg/PoseWithCovarianceStamped` |
| `Select a rectangle -> polygon` | Press at the first corner, drag to the opposite corner, release. | `/qar/selected_area` | `geometry_msgs/msg/PolygonStamped` |
| `Select a circle -> polygon` | Press at the center, drag to the rim, release. | `/qar/selected_area` | `geometry_msgs/msg/PolygonStamped` |
| `Select a polygon (tap vertices)` | Tap each vertex, then press **Close polygon**. | `/qar/selected_area` | `geometry_msgs/msg/PolygonStamped` |

The topics and types are those of the tools Publish Point, 2D Goal Pose and 2D Pose Estimate of RViz2, so subscribers of those tools work unchanged. A pick waits for your confirmation: the status says `waiting for confirmation`, **Confirm pick** publishes it and **Cancel** drops it. **Publish without confirming** removes that step. The default **Target**, `Action Point (QarOS scene intersect)`, publishes the intersection point that QAROS itself computes for the pointing ray; the SDK does not document which geometry QAROS intersects. A click that hits nothing is refused and is never published as the origin of the map. The plugin stamps every pick with time zero, which tf resolves as the latest available transform, so a subscriber that needs the time of a pick has to stamp the message itself.

### What has been seen, and what has not

The authors of `qar_rviz_plugin` ran two live sessions of the plugin in RViz2 against a QAROS Visualizer, with the SDK drop of 2026-09-28. The builds of both sessions were made before the last changes of the code, so no live session has run the current code. The automated tests of the repository and the dry runs of the launcher pass for the current code, but they need no Visualizer and do not exercise the calls into QAROS.

- **The logs of the sessions show** the connection of the session, the invitation of a Visualizer and its open stream, and a second Visualizer; a 3D stream at a median of 60 frames per second in every launch that streamed, with about 52 frames per second for each of two streams; the pause and the resume; 15 AR tools exposed from the ROS 2 graph, with 12 of the 17 layouts and the generic form; one velocity command published from AR; hover and click events with the ray of the runtime and 21 points published on `/clicked_point` in one launch; and a video at a median of 28 frames per second by both routes, from a publisher that sent 30.
- **The authors confirmed in earlier live sessions**, held with older builds before the SDK drop of 2026-09-28, that a click and a hover reach the real widget of RViz2, with the lists of drop-down boxes and the dialogs; that two Visualizers stream at the same time; and that the page of a velocity command opens in AR, can be paged with the pager buttons and publishes.
- **In the two sessions nobody judged** how the pictures looked in the Visualizer (depth, shading, the color of the floor), whether the pages of the other AR tools were readable, or how near the picks landed to the points that were aimed at. No accuracy of a pick has been measured; the authors accept a pick that lands about 0.1 m to 0.2 m from the aimed point at a few meters.
- **Not checked in a live session:** the geographic anchor against a live GPS source, which only unit tests and a synthetic fix have exercised; real multi-user devices; the drag of a slider and the use of two hands, which only unit tests cover; and the tap count and the long press, which neither a unit test nor a live session covers.
- **Not confirmed:** the result of a service call and of an action goal from AR, because the plugin logs neither, and the layouts `diagnostics`, `imu`, `robot-service` and `navigation-goal`, which appear in no log of a live session.
- **Devices.** No device was available for the checks of the current code. The authors state that an earlier build streamed to a HoloLens 2 with this SDK drop; no record of that run exists.

## Reading the logs

### Where the logs are

- **The log of RViz2** is the file `rviz_live_<stamp>.err.log`, with the file `rviz_live_<stamp>.out.log` next to it, in the folder `qar_rviz_logs` in your `TEMP` folder, or in the folder of `-LogDir`. The launcher prints the paths. The panel writes every line that it logs to its **Diagnostics** view, which keeps the last 2000 lines and does not survive RViz2, and to the standard error of RViz2 after the prefix `[qar-panel]`, so the file is the record.
- **The QAROS logs** are in the folder `quaternar` in your `TEMP` folder, or in the folder that `QAR_PANEL_LOG_DIR` names. With this drop every QAROS process that the library starts writes its own files there, whose names carry the role of the process: `streaming_viz_<id>_stdout.log` for a Visualizer, `mixer_<id>_stdout.log` for a mixer and `qar_launcher_stdout.log` for the runtime launcher. Crash reports are the files whose names end in `crash.txt`.
- **The logging mode** is quiet by default: QAROS logs at the severity `warn`. `-TraceLogs` selects the full logs that a bug report needs.

### Lines to look at

The heartbeat is the line that tells you most about a run, so read it first when something stalls. These commands print the lines that the table names from the newest log:

```powershell
$log = Get-ChildItem -Path "$env:TEMP\qar_rviz_logs" -Filter 'rviz_live_*.err.log' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Select-String -Path $log.FullName -Pattern 'Connected to QarOS|LIVE: invited|LIVE: opened stream|AR PANEL: streaming|AR TOOL: exposed|AR TOOL: panel server' | ForEach-Object { $_.Line }
Select-String -Path $log.FullName -Pattern 'HB dt=' | Select-Object -Last 3 | ForEach-Object { $_.Line }
```

| The line contains | It tells you |
|---|---|
| `Connected to QarOS: session open, streaming active` | The session is open. A failed attempt writes `QarOS connect failed:` or `QarOS connect threw:` instead, and `Last error:` shows the same text. |
| `LIVE: invited` and `LIVE: opened stream for peer` | An invitation went out, and the stream of that peer is open (session start-up). If the second line never appears, the peer was not admitted or the device has not connected. |
| `AR PANEL: streaming` | A window of RViz2 is streamed as a GUI panel. |
| `AR TOOL: exposed` | An AR tool was generated. `AR TOOL: panel server listening on` gives the address of the page server. |
| `HB dt=` | The heartbeat, written every 2 seconds. `streams=` is the number of open 3D streams, and each stream has a field with the first eight digits of its peer identifier and its achieved frame rate `fps=`. A `tick_fps=` far below the frame rate of the streams, or a `max_gap=` of hundreds of milliseconds, means that the GUI thread of RViz2 was blocked. `rx=0` after `AR gestures:` on every heartbeat means that the runtime sends no gestures. |

The document `docs/LOG_REFERENCE.md` explains every line and every field of the heartbeat, and the timing files that `-TimingCsv` records.

### Symptoms and what to check

The document `docs/TROUBLESHOOTING.md` of the repository has the full list.

| What you see | What to check |
|---|---|
| The buttons read **Start Qar** and **Add simulated peer**, and no Visualizer opens. | `QAR_PANEL_LIVE_STREAM` is not set in the environment of RViz2, so the panel runs the offline preview. Start RViz2 with `run_live_rviz_stream_real.ps1`, which sets it. Any value that is not empty turns the live mode on, `0` and `off` included. |
| After **Connect to QarOS** the label stays on `Qar runtime: CONNECTING...` and `Last error:` turns red with `QarOS connect failed:`. | Read the text after the colon, correct the cause and press the button again; a failed connect leaves no session behind. On a new machine the first failure is `qar_library_load failed for` and a path: `qar-streaming-c.dll` is not in the folder that `QAR_RUNTIME_BINARIES_DIR` names, or one of its dependencies did not resolve. A session that cannot be created because UDP port `7446` is held (session start-up) fails with a message that mentions `MsgInternalError`. |
| RViz2 ends when you press **Connect to QarOS**. | The C++ runtime beside `rviz2.exe` is older than `14.40`, and the log of RViz2 holds a line about a `SEH FAULT` in `qar_library_init` that names `MSVCP140.dll`. Close RViz2, run `scripts\fix_old_vc_runtime.ps1` and start RViz2 again. |
| A Visualizer is open, but the heartbeat says `streams=0`. | By default the plugin streams only to the peers that it invited itself (session start-up). Connect first, then press **Invite peer** while the label reads `Qar runtime: RUNNING (live)`. A Visualizer that you started by hand, or a device that joins by itself, needs `-RealPeers`. A peer above the default ceiling of 8 is refused with a warning. |
| A stream is open, and the room shows only the grid or nothing. | RViz2 has nothing to show: the default scene draws markers only while `scripts\publish_test_markers.py` runs, and the launcher starts it only with `-Confirm`. Start a publisher or play a bag. Also check that `QAR_PANEL_APP_VOLUME` is not `off`: QAROS places a stream in the scene by its app volume. |
| You click a panel in AR and the control does not react. | Read the first number of the readout `AR panels:`, in the form `input: in N, missed N, dropped N`. With `in 0` the runtime sends nothing to the plugin, so look at the session and the SDK drop and not at the panel. A growing `missed` means that the pointer aims at nothing, and a growing `dropped` that the sequence of events was wrong. |
| An AR tool opens in the headset, but its page does not load. | The headset has to reach the host and the port in the address. Read the log line `AR TOOL: panel server listening on`. The port changes at every start unless you pin it with `QAR_PANEL_WEB_PORT` and open it in the firewall, and on a machine with several networks the host can be the wrong one: set `QAR_PANEL_WEB_HOST` to an address of the machine in the network of the headset. |
| A click in the scene publishes nothing on `/clicked_point`, `/goal_pose` or `/initialpose`. | **Mode** starts at `Off`. A pick waits for **Confirm pick** unless **Publish without confirming** is ticked. A click that hit nothing is refused and counted as `clicks-on-nothing=` in the status, and a hover only previews. |

## Known issues

### The Visualizer crashes when a panel is created again

With the SDK drop of 2026-09-28, the window of the QAROS Visualizer disappeared within a second of the creation of the hidden Displays panel in each of the 25 cases that the authors of `qar_rviz_plugin` observed in which the plugin created that panel again after it had been closed. The windows of all other running Visualizers disappeared with it, QAROS started a Visualizer again after several seconds, and until then the 3D streams stood still. A Visualizer that is started again can show an empty or black scene. The crash happened with the title of the panel unchanged, with a numbered title, with a shifted position of the panel and with a pause of 8 seconds before the start.

In one longer session the Visualizer crashed five more times, each within a second of the close or the creation of a panel: twice at the close of the panel of a dialog, once at the close and the creation again of a video panel, and twice at the opening of the page of an AR tool. It did not crash at the other closes and creations of panels in that session.

Every crash report of these crashes names the same access violation, which reads address `0x4` at the same place inside `WebCore.dll`, the browser engine library of the Visualizer. The crash reports are the files in the QAROS log folder whose names end in `crash.txt`, and they are named after the program `qar-stereokit-gui.exe`, which the authors call the Visualizer GUI process. The SDK `bin` folder holds this program next to `qar-streaming-viz.exe`; how the two programs relate is not documented in the repository. The authors attribute the crash to the Visualizer and not to RViz2 or the plugin, and no title that the plugin chooses prevents it. The plugin gives every QAROS panel that it starts, except the panels of dialogs and the pages of the AR tools, a title that no earlier panel of the RViz2 process used, so a restart of the hidden Displays panel is titled `Displays (AR) (2)`, then `Displays (AR) (3)`. That is a precaution of unproven value and did not prevent the crash. The plugin has not been run against release 0.9.0, so it is not known whether the crash occurs there.

What you can do:

- **Avoid the trigger.** Do not stop the hidden Displays panel (**Stop streaming** on its entry) and start it again (**Stream panel to AR**). **Disconnect (pause)** does not stop it. If you do not need the copy, `QAR_PANEL_HIDDEN_DISPLAYS=off` stops its automatic start on connect; the other closes and creations of panels named above remain.
- **Close and start panels at a calm pace.** After a start or a close, wait several seconds for the Visualizer to come back before you press the next button, and press **Invite peer** for a new Visualizer if none comes back (session start-up). A quick burst of closes and starts once gave a black scene in the trials of the authors.
- **Set the close mode `notice`.** `QAR_PANEL_CLOSE_MODE=notice` keeps the AR panel of a closed window as a notice and keeps its video sender until the session ends; a restart then brings the notice panel back to life and creates no QAROS panel. No crash occurred in the 4 restarts that the authors tried in this mode. The price is one notice panel in AR for each closed window, which you close in AR. Set the variable in the PowerShell window before you start the launcher.
- **Report it.** Send the crash reports with the logs of the run to the authors of `qar_rviz_plugin`, and say whether a log line with `gets the QarOS panel name` stood before the crash. The document `docs/TROUBLESHOOTING.md` lists what a report needs.

### RViz2 can crash when it closes

When you close RViz2, Windows can report that `rviz2.exe` stopped working. The authors know the crash and tolerate it, and most of the runs that they recorded ended with a crash report. Judge a run by its heartbeat lines and not by a clean exit. When the last Visualizer has closed, or while RViz2 closes, the log can also hold the lines `submit_phase: sender rejected the frame` and `wglMakeCurrent FAILED`. Neither line is a failure by itself.

## Related pages

- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries, how they stack and what to install.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how `qar_ogre_adapter` streams an Ogre scene, which the plugin does for the scene of RViz2.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — how a GUI panel shows a video, and what the Qt library does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the session start-up of release 0.9.0.
- [Running the Hub](/docs/operator-guide/running-the-hub) — the Hub that release 0.9.0 onboards to.
- [Source Applications](/docs/operator-guide/source-applications) — the applications that stream content into the room, and how to start them.
