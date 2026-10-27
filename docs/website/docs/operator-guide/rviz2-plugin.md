---
sidebar_position: 4.6
title: Running the RViz2 Plugin on Windows
description: Start RViz2 with the QAROS plugin on a Windows 11 workstation, join the QAROS Hub of this PC or of another PC, and bring the live 3D scene, RViz2 panels, ROS 2 graph tools, video and picks into AR.
---

# Running the RViz2 Plugin on Windows

The **RViz2 plugin** `qar_rviz_streaming` brings a ROS 2 system into AR from a Windows 11 workstation. It adds one panel, **Qar Streaming**, to RViz2. The panel joins the QAROS Hub, and for every target that the Hub lists (a headset, a receiver of a Visualizer) the plugin renders the live 3D scene of RViz2 from the head pose of that target and streams it as stereo video. It also shows RViz2 panels as GUI panels in AR, generates AR tools from the topics, services and actions of the ROS 2 graph, shows ROS 2 image topics as video panels, and publishes the picks of the headset on the topics of the RViz2 tools.

The installation, the configuration reference and the log reference stay in the repository [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz); this page names the documents and does not copy them. [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) describes the libraries that the plugin builds on.

:::info Which QAROS runtime
This page describes the plugin as it was built and checked against the QAROS runtime **0.9.0-dev.3008+65dc9aeb** (release flavor), the version of the QAROS setup installed on the machine of the authors on 2026-10-22. It is a development build that precedes the 0.9.0 release, and the plugin has not been checked against another build. The plugin joins QAROS the way [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) describes: as a **local app** on the PC that runs the Hub, or by **code pairing** with a Hub on another PC. It does not start a Visualizer; you open one from QAROS yourself. The sections that describe a path that the authors have not run in a live session say that this path is designed and not tested.
:::

## What you need before you start

The document `docs/INSTALL.md` of the repository installs everything below from a clean Windows 11 machine, step by step. You need:

- **Windows 11, 64-bit, at the console of the machine.** RViz2 and the Visualizer open OpenGL windows, and the documented runs were made from the local console; Remote Desktop and services are not covered. Linux is not built or tested.
- **An NVIDIA GPU with a current OpenGL driver.** The C API currently supports NVIDIA adapters only, as its header states, and no minimum GPU or driver version is stated. On a laptop with an integrated and a discrete GPU, RViz2 should render on the adapter that QAROS uses, or every picture passes through a copy between the adapters, so the live launcher sets the graphics preference of `rviz2.exe` to high performance.
- **PowerShell 7.** The scripts and every command on this page run in `pwsh`.
- **The ROS 2 Humble binary distribution for Windows, repaired.** The plugin runs inside the `rviz2.exe` of the official binary release. That release has gaps: some runtime libraries are missing, the C++ runtime beside `rviz2.exe` is too old for the QAROS library, and `tinyxml.dll` is missing. The repository ships the scripts that repair the distribution in place, and the plugin needs all nine repairs.
- **The plugin and the libraries it builds on.** `colcon` builds the plugin, and the installation puts it into one prefix together with `qar_ogre_adapter` ([github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)) and `qar_qt_streaming` ([github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)), on Qt 5.15.2.
- **A QAROS Hub to join.** Either QAROS installed on this PC ([Installation](/docs/operator-guide/installation)), so that the plugin can join its Hub as a local app, or a Hub on another PC and the code that Hub shows ([Join with a Hub on another PC](#join-with-a-hub-on-another-pc)). The plugin loads the installed runtime through the vendor's runtime lookup (`qar_library_load(NULL)`), so QAROS needs no entry in `PATH`; a `qar-streaming-c.dll` beside `rviz2.exe` or a `QAROS_RUNTIME_DIR` would win over the installation, and [Troubleshooting](/docs/developer-guide/troubleshooting#joining-qaros-as-a-local-app) says what that causes.
- **One local network.** The documented runs put the machine and its devices on one local network. QAROS has its own ports; [Networking & Federation](/docs/operator-guide/networking-and-federation#ports-and-firewall) lists them.

## Starting RViz2 with the plugin

### The launcher scripts

The `scripts` folder of the repository holds two launchers. Run them in PowerShell 7, from the folder of your clone of the repository.

| Script | What it does |
|---|---|
| `scripts\run_live_rviz_stream_real.ps1` | Starts RViz2 with the panel in **live mode**, so that the panel can join the QAROS Hub. Without `-Confirm` it only prints what it would do and starts nothing. |
| `scripts\run_rviz_with_plugin.ps1` | Starts RViz2 with the panel in the **offline preview**: simulated peers replace QAROS, and nothing is sent to it. It has no dry run and starts RViz2 at once. |
| `scripts\env_setup.ps1` | Starts nothing. Dot-source it, with a leading dot and a space, to prepare a PowerShell window with the environment of the ROS 2 distribution and Qt, for the ROS 2 commands that you run next to RViz2. |

### Check what the launcher will do, then start it

Run the live launcher without `-Confirm` first. It takes the locations from the parameters `-SdkRoot`, `-Prefix`, `-QtRoot` and `-Ros2Root` or, for a parameter that you omit, from the environment variables `QAR_SDK_ROOT`, `QAR_PREFIX`, `QAR_QT_ROOT` and `ROS2_INSTALL_ROOT`. `-SdkRoot` is optional and only feeds the checks and the printed plan: it defaults to the install folder that the QAROS setup records in the registry. The dry run prints the scene, the installed QAROS version, whether the Hub processes run, the Hub mode that the panel will resolve, the settings and the logging mode. It reads the machine and nothing else: it starts nothing, changes nothing and, when it finds the locations, ends with exit code `0`:

```powershell
.\scripts\run_live_rviz_stream_real.ps1
```

Then start RViz2:

```powershell
.\scripts\run_live_rviz_stream_real.ps1 -Confirm
```

The launcher starts RViz2 and then stays running until RViz2 exits, so that it can save the logs of the run. With `-Confirm` it does these things:

- It stops, before it changes or starts anything, when the C++ runtime beside `rviz2.exe` is older than version `14.40` or `tinyxml.dll` is missing there, and names the repair script to run.
- It writes the Windows graphics preference of `rviz2.exe` (high performance) for your user, in the registry under `HKCU`, unless it is set to high performance already.
- It renames each QAROS log file over 256 MB by adding `.prev` to its name.
- It ends the test publishers of earlier runs: the Python processes whose command line names `publish_test_markers.py`, `publish_test_tf.py`, `publish_test_robot.py` or `publish_test_pointcloud.py`, also when you started them yourself in another window.
- It cleans up the Fast-DDS shared memory of programs that have ended.
- It starts `rviz2.exe` with the default scene `scene3_grid_marker.rviz`, which holds a grid and the markers of the test publisher. The publisher itself starts only with `-TestMarkers`; without it the scene shows only the grid, and the button **Start test markers** on the Diagnostics tab starts and stops the publisher at any time.
- When RViz2 has exited, it copies the log of the QAROS library of the run, the newest log files of the QAROS launcher and every crash report of the run next to the RViz2 logs.

The launcher does **not** start the QAROS Hub or a Visualizer. With a local join, the plugin starts QAROS itself when you press **Connect to QarOS** and no Hub runs ([Joining the Hub of this PC](#join-the-hub-of-this-pc)); a Visualizer you open from QAROS yourself. These parameters matter most:

| Parameter | Effect |
|---|---|
| `-HubMode`, `-HubHost`, `-HubPort` | Which Hub the panel joins (`auto`, `local` or `remote`) and the host and onboarding port that the panel starts from for a Hub on another PC. The pairing code is never a parameter: you type it in the panel. |
| `-InvitePeers` | Devices to invite as soon as the panel has joined the Hub of this PC, as `hololens:<host>` or `headset:<host>` entries. |
| `-RuntimeDir` | A folder with another QAROS runtime for the library lookup. It sets `QAROS_RUNTIME_DIR` and is not set unless you pass it. |
| `-Scene` | The RViz2 configuration to load, instead of the default test scene. |
| `-LogDir` | The folder of the RViz2 logs. The default is `qar_rviz_logs` in your `TEMP` folder. |
| `-TraceLogs` | Collects the full QAROS and adapter logs that a bug report needs. They can grow to gigabytes, so do not judge stalls in this mode. |
| `-TimingCsv` | Records the timing of every video stage in CSV files. |
| `-TestMarkers` | Starts the test publisher with the launch. |

The document `docs/CONFIGURATION.md` lists every parameter. [Environment variables](#environment-variables) names the variables that the launcher exports.

### Open the panel

The scene of the launcher loads the panel. RViz2 opens with the panels Displays, Views and **Qar Streaming**. The panel connects to nothing until you press **Connect to QarOS**. An RViz2 configuration of your own loads the panel with this entry in its `Panels` list:

```yaml
Panels:
  - Class: qar_rviz_streaming/QarStreamingPanel
    Name: Qar Streaming
```

If the panel is missing, either the shell that started RViz2 did not source the overlay of the install prefix (`local_setup.ps1`) or the plugin is not installed into that prefix. A launcher sources the overlay itself, and `docs/TROUBLESHOOTING.md` has the checks. If you start `rviz2.exe` without a launcher, you have to set the variables that the launcher sets. `QAR_PANEL_LIVE_STREAM` comes first, because without it the panel starts in the offline preview. The section "Running RViz without the launcher" of `docs/CONFIGURATION.md` lists them.

### The Qar Streaming panel

The panel is a status strip over six tabs: **Connection**, **AR panels**, **AR tools**, **Picking**, **Settings** and **Diagnostics**. The strip shows the state of the connection and the last error. The sections below name the controls that you use, and the document `docs/OPERATOR_GUIDE.md` lists every control.

- **Connection** holds the buttons **Connect to QarOS** and **Disconnect (pause)**, the group **QAROS Hub** (which Hub, the pairing, the targets), the group **Invite a device** and the group **AR control panel**.
- **AR panels** streams RViz2 windows into AR.
- **AR tools** generates AR tools from the ROS 2 graph.
- **Picking** turns the pointer of the headset into ROS 2 messages.
- **Settings** keeps the values that the panel saves in the RViz2 configuration.
- **Diagnostics** shows the state of the display context, takes test captures and, in the offline preview, adds simulated peers.

In live mode the two runtime buttons read **Connect to QarOS** and **Disconnect (pause)**. In the offline preview the same buttons read **Start Qar** and **Stop Qar**, the group **QAROS Hub** shows its settings only, and its state line reads `QarOS: preview mode - set QAR_PANEL_LIVE_STREAM to connect`.

## Joining the QAROS Hub

The plugin builds its QAROS session when you press **Connect to QarOS**, never when the panel loads. The session is built by a thread of its own, the **session owner**, so the button returns at once and RViz2 stays responsive; the render tick of RViz2 collects the progress and the result. Two joins exist, and the **Hub** drop-down in the group **QAROS Hub** chooses between them:

| Hub setting | Meaning |
|---|---|
| **Automatic** (the default) | The Hub of another PC when a pairing with it is stored; else the Hub of this PC when QAROS is installed here; else the Hub of another PC. |
| **This PC (approve once in the QAROS Hub)** | Join the Hub of this PC as a local app. |
| **Another PC (pairing code)** | Pair with the Hub of another PC. |

The setting is saved in the RViz2 configuration (`Hub Mode`, `Hub Host`, `Hub Port`); the variables `QAR_PANEL_HUB_MODE`, `QAR_PANEL_HUB_HOST` and `QAR_PANEL_HUB_PORT` override the saved values, and the control that a variable decides is disabled. The plugin decides the mode when you connect and writes it to the log as `QAROS: hub mode <local|remote> (<reason>)`. A change of the setting applies at the next connect.

The plugin identifies itself to the Hub as `RViz2 (QAROS ROS plugin) on <name of this PC>`; the approval prompt of the Hub shows that text separately, as the text the app says about itself.

### Join the Hub of this PC

This is the join for a Hub on the PC that runs RViz2. Everything the vendor documents about it applies unchanged: [Local app: on the QAROS PC, no code](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code) describes the call and the approval prompt, and [Approved local apps](/docs/operator-guide/source-applications#approved-local-apps) shows how an operator lists and revokes the approvals.

1. **Connect.** Press **Connect to QarOS**. The state line reads `QarOS: loading the QAROS runtime ...` and then `QarOS: waiting for approval in the QAROS Hub - click Allow in the QAROS window`, followed by a countdown of the time left. If no Hub runs on this PC, the join starts QAROS; this is how the vendor's local enrollment behaves.
2. **Allow.** The first time, the Hub opens the window `Allow <app> to join QAROS?` in front of every other window. Check what it reports about `rviz2.exe` and press **Allow**. Nothing is stored by the plugin; the Hub remembers the approval. The `rviz2.exe` of the ROS 2 distribution is not code-signed, so the approval is for one exact file and the Hub asks again after RViz2 is updated or replaced; the approval window of the authors' runs said so.
3. **Connected.** The state line reads `QarOS: connected (local app) - <n> target(s), <k> served`, and the identity line reads `This PC: approved local app (...)`. Later starts of an unchanged `rviz2.exe` join without a prompt.

The prompt waits 120 seconds by default. `QAR_PANEL_APPROVAL_TIMEOUT_S` sets another wait, from 1 to 600 seconds; a value that is not a number of seconds is reported in the log and the default applies. **Cancel** stops the wait. The plugin has no button that revokes an approval: the operator does that in the Visualizer, on the Source Applications panel. The group **Invite a device** works in this mode only ([Devices](#devices)).

What the authors ran: the join with **Allow**, a second start of RViz2 without a prompt, and the streaming that follows, in the live checks of 2026-10-24 and 2026-10-25. The paths for **Deny**, for no answer within the wait, and for a Hub that is stopped while the plugin is connected are **designed and not tested** in a live session. For them the plugin shows these messages in the state line and in `Last error:`:

| Situation | Message |
|---|---|
| **Deny**, a closed prompt or a revoked approval | `QAROS denied this app (Deny was clicked, the prompt was closed, or the approval was revoked). Connect again and click Allow.` |
| No answer within the wait | `Nobody answered the approval prompt in the QAROS Hub in time. Connect again and click Allow in the QAROS window.` |
| No Hub could be started | `The QAROS Hub is not running and could not be started. Start QAROS from the Start menu, then connect again.` |
| The approval no longer matches `rviz2.exe` | `This program does not match what was approved (moved, copied or changed). Revoke the old approval in the Visualizer (Source Applications) and connect again.` |

### Join with a Hub on another PC

A Hub on another PC cannot identify `rviz2.exe` from its own Windows, so the plugin pairs with the code that Hub shows, as [Onboarding code: a hub on another PC](/docs/developer-guide/onboarding-and-sessions#onboarding-code-a-hub-on-another-pc) describes. Choose **Another PC (pairing code)**; the fields **Host:** and **Port:** appear.

1. On the other PC, open the Visualizer's Connection panel and let the Hub share: [Pair another Hub](/docs/user-guide/connection-panel#pair-another-hub) shows the pairing code, its countdown and the addresses of the Hub. The onboarding port is shown with the code.
2. In the panel, type the host name or address and the port, press **Connect to QarOS**, and when the state line reads `QarOS: type the pairing code of the Hub at <host>:<port> (Connection tab)`, type the code in **Code:** and press Enter or **Pair**. The code lives about ten seconds and works once, so type it at once. The field is cleared when it is sent; the plugin neither stores nor logs the code.
3. The state line reads `QarOS: connected (paired) - ...` and the identity line reads `Paired with <host>:<port> (id <first digits>)`. The plugin stores the onboarding id, the host and the port in the identity folder (below), and every later **Connect to QarOS** rejoins silently without a code.

**Forget pairing** erases the stored pairing: press it, then press it again within five seconds (the button reads `Click again to forget`). The next connect needs a new code. In the mode **This PC** the button is disabled and reads `Approval: revoke in the Visualizer`, because a local approval lives in the Hub.

A wrong or expired code gives the message `The code was wrong or has expired (codes live about 10 s and work once). Type the code the Hub shows at that moment.`, and a pairing that has expired or was revoked asks for a new code. **This path was not run in a live session** against a Hub on another PC; it is designed from the behavior of the vendor's calls and covered by the unit tests of the plugin only. The group **Invite a device** is disabled in this mode, because the Hub serves device invites only to an app that joined it on its own PC.

### Where the plugin keeps its identity

The plugin keeps what it needs in the folder `%LOCALAPPDATA%\Quaternar\QarOS-ROS\rviz2-plugin`, one folder for each program and for each Windows user. `QAR_PANEL_INSTANCE=<name>` adds `.<name>` to the folder name and to the names of the plugin's panels, so that a second RViz2 on the same PC keeps its own state, and `QAR_PANEL_IDENTITY_DIR` replaces `%LOCALAPPDATA%\Quaternar\QarOS-ROS` as the parent. The folder holds `identity.txt` (the onboarding id, the host and the port of a code pairing; no secret and never the code) and the folder `runtime`, which is the storage folder that the plugin gives to QAROS. In the mode **This PC** the plugin writes no identity file. The plugin does not use the data folder of the Hub itself.

### Targets and streams

The Hub lists the peers of the session. The plugin renders for those whose role is `TARGET` and whose presence is online or unknown ([Finding rendering targets](/docs/developer-guide/rendering-streams#finding-rendering-targets) and [What a peer is](/docs/developer-guide/peers#what-a-peer-is-role-category-and-device) describe both). The list **Targets** in the group **QAROS Hub** shows each one as name, kind, presence and `served` or `not served` with the reason. A target that goes offline, leaves or retires loses its stream; one that was offline and comes back online gets a new stream. Each served target gets its own stereo stream of the scene, and the plugin serves at most 8 targets at once (`QAR_PANEL_MAX_PEERS` changes the ceiling; a target beyond it is listed as `not served: cap reached`), because every target costs a color and a depth render of the scene for each eye in every frame.

A Visualizer lists a receiver only after one exists in it. In the runs of the authors (2026-10-24) a freshly started Visualizer had no receiver, and the Hub listed a target for it only after **Add receiver** was pressed in the Visualizer; the plugin then started the stream by itself.

### Devices

The group **Invite a device** asks the Hub of this PC to start the app of a device: choose **HoloLens 2** or **Headset (Quest, Android)**, type the host name or address of the device, and press **Invite peer**. Both kinds need a host. The call is made on the session owner's thread, the button reads `Inviting...` until the Hub answers, and the result is logged. The Hub serves device invites only to an app that joined it as a local app, so the group is disabled in the mode **Another PC**, while streaming is paused, and while the session is being set up. The same invites can be started from the Visualizer's [Devices](/docs/user-guide/connection-panel#devices) area, which is the way for a Hub on another PC. `QAR_PANEL_INVITE_PEERS` (and `-InvitePeers` of the launcher) invites the listed devices once when the session first connects. **Invites were not run in a live session.**

### Pause, a lost Hub and the end

**Disconnect (pause)** stops the streaming of the 3D scene and keeps everything else: the session stays open, and the AR panels, the AR tools and the AR control panel stay open and keep running. It pauses and does not disconnect. **Connect to QarOS** resumes the streaming on the open session at once, with no new join. Everything the plugin built on the session is released only by **Forget pairing**, by a session that ends on its own and when RViz2 closes.

When RViz2 closes, the plugin, with hiding on (the default, see [RViz2 panels in AR](#rviz2-panels-in-ar)), hides its QAROS panels and lets go of their senders and subscriptions instead of closing them, destroys the render senders, and cancels a wait that is still running. The thread of the session owner is joined, so no thread of the plugin outlives its library; the calls that were queued before the end get 8 seconds to finish.

If every runtime peer that the plugin has seen in the session reports offline or leaving for more than 3 seconds, the state line reads `QarOS: Hub not reachable - waiting for it to come back`, and it returns to connected when a runtime peer is online again. The plugin does not end the session by itself. **This path is designed and was not run in a live session**, and the signal is a heuristic: the C API has no call that reports the state of the session.

### How the plugin keeps RViz2 responsive

No QAROS call that may wait runs on the Qt thread of RViz2. The library load, the onboarding, the invites and every call that reads or changes a GUI panel or an app volume are queued for the session owner's thread, which runs them in order; the Qt thread collects the results at the render tick. This matters because a query of the installed runtime is slow: [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#how-long-a-query-takes) lists the measured costs, about 2 seconds for each query. A plugin that made four queries on the Qt thread in every render tick froze RViz2 for 8 seconds at a time; the design above removed that.

The plugin watches itself. A QAROS call that takes more than 20 ms on the Qt thread writes `[qar-gui-call] ... WARNING <call> took <n> ms on the GUI thread` to the log of RViz2; `QAR_GUI_CALL_WARN_MS` changes the limit. A call that takes 500 ms or more on the session owner's thread writes `QAROS: call <name> took <n> ms`, and a queued task that takes a second or more `task '<name>' took <n> ms`. These lines are normal for the queries above and are not errors.

## What you see and do in AR

### The 3D scene

The plugin renders the scene once for each target, from the head pose that QAROS reports for that target, and sends a color picture and a depth picture for each eye. The view in the Visualizer follows the pose of the Visualizer and not the camera of RViz2: orbiting the camera of RViz2 changes only the picture on your desktop. To make the scene readable in AR, the plugin adds an opaque gray plane 1 mm under every Grid display, so that the depth picture shows a solid floor instead of gaps between the grid lines. While it streams, the plugin lights the scene with a rig that is fixed in the world and suppresses the headlight of RViz2, so that the shading does not swing when you orbit the view.

The scene is placed in the room by an **app volume**, a cube of 10 m that the plugin gets or creates under the stable name `scene.rviz2-plugin.<name of this PC>.qaros-ros` (shown as `RViz Scene`); the next start finds the same volume again. `QAR_PANEL_VOLUME_SIZE`, `QAR_PANEL_VOLUME_SCALE` and `QAR_PANEL_VOLUME_Y` change its edge, its scale and the height of its center, and the launcher sets the center to -1.6 m when you did not set one. `QAR_PANEL_APP_VOLUME=off` makes every stream refuse with a legible message, because QAROS rejects a stream without an app volume. Whether QAROS applies those values to a volume that already exists is not documented, and the plugin reads the placement back from QAROS instead of assuming it.

**Resolution:** and **FPS cap:** on the Settings tab start at 1280 x 720 and 60 frames per second. They apply to the streams that are created from then on and do not resize a stream that exists. The plugin renders all streams that are due in one batch, with a budget of about 16 ms, and sheds the frames of the targets beyond it, except that a target that has not been rendered for 1/30 s is always served. Choose a lower resolution when you serve several headsets.

The launcher sets `QAR_POSE_WORLD` to `zup` and `QAR_DEPTH_ENCODING` to `ndc`. Without the first the scene arrives on its side, and without the second the Visualizer showed the depth in one color with an SDK drop earlier than 2026-09-28; whether the installed runtime still needs it was not tested separately. If you start RViz2 without the launcher, set both.

The plugin also reads the fixes of a `sensor_msgs/msg/NavSatFix` topic, `/gps/fix` unless the environment names another, and tells QAROS from them where the origin of the scene lies on Earth. This path does nothing while the topic is silent.

### RViz2 panels in AR

The tab **AR panels** streams a window of RViz2 as a **GUI panel**. The drop-down lists **This QAR panel**, every dock of RViz2 by its title, one entry `Video: <topic>` for each video panel that an AR tool opened, and the hidden Displays copy. Press **Refresh** after you add a dock. Choose an entry and press **Stream panel to AR**; **Stop streaming** ends the stream.

- **Placement.** A panel hangs with its top edge 1.5 m above the room origin, or 1.5 m above the RViz2 ground when `QAR_PANEL_FIXED_POSES=ground` (the launcher sets that unless you did), and is kept at least 0.1 m above the ground. The first panel hangs 1 m in front of the origin, and each further panel 0.65 m further along the x axis. No call of QAROS asks where the operator is, so the plugin cannot place a panel in front of you ([Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#where-the-operator-is) explains why). The AR control panel is the exception, and is placed in front of each target once the view of that target is streamed.
- **Input.** A click and a hover on the panel reach the real widget of RViz2, including the lists of drop-down boxes and the dialogs that the widget opens. A dialog becomes a panel of its own, beside the panel that opened it. A panel is captured at up to 15 frames per second, and at up to 30 when its dock holds an Image or Camera display.
- **Hiding, not closing.** By default (`Hide panels in AR instead of closing them` on the Settings tab, `QAR_PANEL_AR_VISIBILITY=hide`) the plugin never closes a QAROS panel while a session runs. **Stop streaming**, a closed dock or dialog and a removed AR tool hide their panel in AR and keep it, with its video sender, until the session ends; **Stream panel to AR**, the dock shown again or the tool exposed again shows it again and creates no new QAROS panel. The setting `Closed AR panels` and `QAR_PANEL_AR_VISIBILITY=close` select the older behavior, in which a closed window closes its AR panel and, by default, destroys its video sender.
- **Found again by name.** Every QAROS panel and the app volume get a stable ASCII name made from the key of the window, the program and the name of this PC. At the next start the plugin asks QAROS for the panel of the same name. If the Hub still has it, hidden, the plugin gives it the new video, fits its size and pose, and shows it, and the log writes `[qar-panel-adapter] ... panel '<key>' reattached name=<name> ...` instead of `created`. A panel that shows web content where video is wanted is not reused (the plugin repeats the request with the suffix `-v2`), and a name whose panel is closed is retried with the suffixes `-r1` to `-r8`. The authors' live checks of 2026-10-24 and 2026-10-25 found the panels again after a restart; what QAROS applies to an existing panel is not documented (see [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#a-panel-or-volume-that-already-exists)).
- **Resizing.** A dock that changes its size gets a new video sender of the new size, attached to the same QAROS panel. `QAR_PANEL_RESIZE=new-panel` makes a second panel instead.
- **Pauses.** The press on **Stream panel to AR** and the creation of the QAROS panel make no wait on the Qt thread; the panel appears in AR when the session owner's thread has made it, which can take several seconds with the installed runtime (creating a panel that is found again took about 4 seconds in the authors' runs).

When the connection is up, the plugin also makes a second instance of the Displays panel, shows it off-screen only, so that it never appears on your desktop, and streams it into AR as `Displays (AR)`, 480 pixels wide, with its top edge 1.2 m to the left and 1 m in front of the origin. It views the same display tree as the real Displays dock, so a check box that you toggle in AR, a property that you edit and a display that you add, remove or rename in the headset change the displays of RViz2. The check box `Start the hidden Displays copy when connecting` and `QAR_PANEL_HIDDEN_DISPLAYS` set to `off` stop the automatic start on connect, and the last entry of the drop-down still starts the copy by hand. The copy is made once the first 3D stream is open.

### The AR control panel

The group **AR control panel** on the Connection tab puts the main controls of the panel into AR as a page of its own. By default (`Show the AR control panel when connecting` on the Settings tab, or `QAR_PANEL_AR_CONTROL`) the plugin serves the page when it connects and shows one panel in front of each target once the view of that target is streamed (`QAR_PANEL_AR_CONTROL_PER_PEER=off` shows one panel for everybody). The line in the group says whether the panel exists. **Show AR control panel** shows it again if it was closed in AR, and **Copy page URL** copies the address of the page, which opens in a desktop browser. The page is served over plain HTTP like the AR tools below.

### AR tools from the ROS 2 graph

An **AR tool** is a panel that the plugin generates from a topic, a service or an action of the live ROS 2 graph. It is a web page that a server in RViz2 serves and that QAROS opens as a URL panel. The plugin needs no generated message code: it reads the layout of a type from the introspection typesupport of its interface package at run time.

1. Press **Add from ROS graph...**. The dialog `Add from the ROS graph` lists the topics, services and actions of the graph as trees by namespace, with their types. **Filter:** narrows the list by a part of a name or of a type, and **Refresh** asks the graph again.
2. Select an entry. The lines under the tree say what **Add** will do before you press it. For a topic, **Add** creates the RViz2 display that accepts the type and pins the topic to the list of watched topics.
3. Tick **Expose as a QAR AR tool** to generate the AR panel. It is always on for a service and an action, which RViz2 has no display for. An exposed image topic gets no RViz2 display at all.
4. Tick **Allow publishing from AR** only when the tool has to publish. The box exists for topics only and is off by default, because a publication to a topic is a command to a robot. The buttons that command a robot, `Publish` and `Send goal`, ask for a second press within 4 seconds and read `Press again to confirm`. `Call service`, `Cancel goal` and the `STOP` button of a velocity command, which publishes a zero command, act on the first press.

The first tool hangs 1.2 m in front of the origin, and each further tool hangs 0.7 m further to the left. A row of the list of tools shows the kind, the name and the type, followed by `[in AR]` when its QAROS panel exists, `[pending]` when it does not yet, and `[writable]` for a tool that may publish. The buttons **Remove tool**, **Re-open in AR** and **Copy URL** act on the selected row, and the box `size x` multiplies the size of the AR panel at the next **Re-open in AR** (`QAR_PANEL_WEB_SCALE` sets its start value). A tool that you expose before you connect is served at once but stays `[pending]` until the connection is up.

The plugin picks one of 17 layouts for the type of a tool, for common types such as velocity commands (`Twist`), poses, battery state, odometry, maps, laser scans, images and GPS fixes (`NavSatFix`), and a type that no layout claims gets a generic form with one widget for each field. The page of a topic shows the newest message and refreshes at 5 Hz, however fast the topic is.

The size of the AR panel of a tool is not taken from the page that the panel shows. With the installed runtime the page of a URL panel sees a viewport that follows the size of the panel, so a size that was computed from the reported viewport fed back into itself. The plugin therefore sizes each URL panel from a fixed design viewport of 500 by 500 CSS pixels (the AR control page is 0.55 by 0.55 m), allows at most three resizes of a panel in 10 seconds, and refuses a width below a floor. `QAR_PANEL_WEB_SIZE=<w>x<h>` (meters) pins the size, and `QAR_PANEL_WEB_SCALE` multiplies it.

:::caution The pages are served over plain HTTP
The server of the tools listens on all network interfaces, so that a headset on the network can reach it, on a port that the operating system picks. A page is reached at `http://<host>:<port>/p/<token>/`. The traffic is not encrypted, and the token of 128 random bits in the address is the only protection; a wrong or missing token gets a 404. `QAR_PANEL_WEB_PORT` pins the port, for a firewall rule, and `QAR_PANEL_WEB_HOST` sets the host name that the addresses contain.
:::

To try the tools without a robot, run `python scripts\publish_template_topics.py` in a PowerShell window that you prepared with `. .\scripts\env_setup.ps1` and that uses the same `ROS_DOMAIN_ID` as RViz2. It publishes ten topics and one `Trigger` service under `/qar_demo`, which you find by typing `qar_demo` in **Filter:**, and it ends by itself after 1500 seconds.

### Video panels

A ROS 2 image topic reaches AR in two ways. The first is the button `Open video in AR` on the page of an exposed `sensor_msgs/msg/Image` or `sensor_msgs/msg/CompressedImage` topic: it opens a new AR panel, titled with the topic, which appears in the drop-down as `Video: <topic>`. The second is a dock of RViz2 that holds an Image or Camera display: press **Stream panel to AR** on that dock, and the plugin subscribes to the topic of the display and streams the video into the AR panel directly.

The plugin sends the video at up to 30 frames per second. It decodes the usual raw encodings of `sensor_msgs/msg/Image` and the PNG or JPEG data of `sensor_msgs/msg/CompressedImage`, and it scales a larger frame down to fit 1280 by 1280 pixels. Large video topics need the Fast-DDS profile `scripts\fastdds_large_data.xml` at both ends of the connection. The launcher exports it unless you pass `-NoDdsProfile` or set `FASTRTPS_DEFAULT_PROFILES_FILE` yourself, and `scripts\publish_video_file.py` applies it to its own process.

### Picks

The tab **Picking** turns the pointer of the headset into ROS 2 messages. **Mode** chooses what a pointer means in the 3D scene, and it starts at `Off`, so that a gesture can never move a robot because a mode was left on. **Target** chooses what the pointer is resolved against, and **Frame** names the frame in the header of every published pick (`map` at the start).

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

The authors of `qar_rviz_plugin` ran live sessions of the plugin in RViz2 against a QAROS Visualizer in two generations.

- **With the installed runtime 0.9.0-dev.3008+65dc9aeb (2026-10-24 and 2026-10-25),** the authors checked, by looking at RViz2 and the Visualizer, that the offline preview shows the group **QAROS Hub**; that the first connect shows the approval window and, after **Allow**, a connected state and a stream in the Visualizer once a receiver exists; that a restart joins without a prompt and finds the panels again; and that the tabs, the AR control panel for each user, switching a streamed window off and on, resizing a dock, opening and closing a dialog, picking with `/clicked_point` and the status strip work as before. No crash report of the Visualizer or RViz2 appeared in these runs. Those checks are a judgment of what the user saw and not a measurement. The paths that these runs did not exercise are named where they are described: **Deny**, no answer, a stopped Hub, a code pairing against another PC, forgetting and pairing again, and device invites.
- **With the SDK drop of 2026-09-28 (an earlier generation, with a session start-up that the installed runtime replaced),** the logs of two sessions show a 3D stream at a median of 60 frames per second in every launch that streamed, with about 52 frames per second for each of two streams; the pause and the resume; 15 AR tools exposed from the ROS 2 graph, with 12 of the 17 layouts and the generic form; one velocity command published from AR; hover and click events with the ray of the runtime and 21 points published on `/clicked_point` in one launch; and a video at a median of 28 frames per second by both routes, from a publisher that sent 30. In these sessions nobody judged how the pictures looked in the Visualizer (depth, shading, the color of the floor), whether the pages of the other AR tools were readable, or how near the picks landed to the points that were aimed at. No accuracy of a pick has been measured; the authors accept a pick that lands about 0.1 m to 0.2 m from the aimed point at a few meters.
- **Not checked in a live session:** the geographic anchor against a live GPS source, which only unit tests and a synthetic fix have exercised; real multi-user devices; the drag of a slider and the use of two hands, which only unit tests cover; the result of a service call and of an action goal from AR, because the plugin logs neither; and the layouts `diagnostics`, `imu`, `robot-service` and `navigation-goal`, which appear in no log of a live session.
- **Devices.** No device was available for the checks of the current code. The authors state that an earlier build streamed to a HoloLens 2 with the drop of 2026-09-28; no record of that run exists.

## Reading the logs

### Where the logs are

- **The log of RViz2** is the file `rviz_live_<stamp>.err.log`, with the file `rviz_live_<stamp>.out.log` next to it, in the folder `qar_rviz_logs` in your `TEMP` folder, or in the folder of `-LogDir`. The launcher prints the paths. The panel writes every line that it logs to its **Diagnostics** view, which keeps the last 2000 lines and does not survive RViz2, and to the standard error of RViz2 after the prefix `[qar-panel]`, so the file is the record. The lines of the session owner start with `QAROS:`.
- **The QAROS library inside RViz2** writes its log into the folder `quaternar` in your `TEMP` folder, or into the folder that `QAR_PANEL_LOG_DIR` names, and the launcher saves a copy of its file `qar_streaming.log` next to the RViz2 logs when RViz2 exits. The processes that the Hub runs write their own logs and crash reports where [Using the Hub](/docs/operator-guide/running-the-hub#logs) says.
- **The logging mode** is quiet in the launcher: the library logs at `info`, there is no transport logging and no per-frame trace. Without the launcher the plugin logs at `trace`. `-TraceLogs` selects the full logs that a bug report needs, and `QAR_PANEL_LOG_SEVERITY` and `QAR_PANEL_LOG_CONSOLE` set the severity and the echo to the console.

### Lines to look at

The heartbeat is the line that tells you most about a run, so read it first when something stalls. These commands print the lines that the table names from the newest log:

```powershell
$log = Get-ChildItem -Path "$env:TEMP\qar_rviz_logs" -Filter 'rviz_live_*.err.log' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Select-String -Path $log.FullName -Pattern 'QAROS: (runtime|hub mode|joined as peer|target)|Connected to QarOS|LIVE: opened stream|AR PANEL: streaming|AR TOOL: exposed|AR TOOL: panel server' | ForEach-Object { $_.Line }
Select-String -Path $log.FullName -Pattern 'HB dt=' | Select-Object -Last 3 | ForEach-Object { $_.Line }
```

| The line contains | It tells you |
|---|---|
| `QAROS: runtime <version> <flavor> <api> from <path>` | Which runtime the plugin loaded and from where. A path other than the installed QAROS means a stray `qar-streaming-c.dll` or `QAROS_RUNTIME_DIR` won the lookup. |
| `QAROS: hub mode <local\|remote> (<reason>)` | The mode that this connect uses, and why. |
| `QAROS: state <state>` and `QAROS: progress <text>` | The steps of the join, including `Waiting for approval in QAROS hub`. |
| `QAROS: joined as peer <id> (<local\|rejoin\|pairing>) in <n> s` | The session is open. |
| `QAROS: target <id> <kind> '<name>' <presence> -> served` | A target of the Hub and whether it gets a stream. A target that went away reads `-> stream stopped`. |
| `Connected to QarOS: session open, streaming active` | The panel finished setting up the session. A failed join writes the message of the state line instead, and `Last error:` shows the same text. |
| `LIVE: opened stream for peer` | The stream of a target is open. If it never appears for a target that is listed as served, the target has not asked for the stream yet. |
| `[qar-panel-adapter] ... panel '<key>' <created\|reattached> ...` | A QAROS panel was made or found again, with the state it had and whether its stream or size changed. |
| `AR PANEL: streaming` | A window of RViz2 is streamed as a GUI panel. |
| `AR TOOL: exposed` | An AR tool was generated. `AR TOOL: panel server listening on` gives the address of the page server. |
| `HB dt=` | The heartbeat, written every 2 seconds. `streams=` is the number of open 3D streams, and each stream has a field with the first eight digits of its peer identifier and its achieved frame rate `fps=`. A `tick_fps=` far below the frame rate of the streams, or a `max_gap=` of hundreds of milliseconds, means that the GUI thread of RViz2 was blocked. `rx=0` after `AR gestures:` on every heartbeat means that the runtime sends no gestures. |

The document `docs/LOG_REFERENCE.md` explains every line and every field of the heartbeat, and the timing files that `-TimingCsv` records.

### Symptoms and what to check

The document `docs/TROUBLESHOOTING.md` of the repository has the full list.

| What you see | What to check |
|---|---|
| The buttons read **Start Qar** and **Stop Qar**, and nothing connects. | `QAR_PANEL_LIVE_STREAM` is not set in the environment of RViz2, so the panel runs the offline preview. Start RViz2 with `run_live_rviz_stream_real.ps1`, which sets it. Any value that is not empty turns the live mode on, `0` and `off` included. |
| After **Connect to QarOS** the state line stays on `waiting for approval in the QAROS Hub`. | The approval window of the Hub is open somewhere: look for `Allow <app> to join QAROS?` in the QAROS tray window and on the Source Applications panel of the Visualizer. Without an answer it ends after the wait (120 seconds, or `QAR_PANEL_APPROVAL_TIMEOUT_S`). |
| The state line asks for approval again after RViz2 was updated. | The `rviz2.exe` of the ROS 2 distribution is not code-signed, so the Hub approves one exact file. Press **Allow** again. If the Hub reports that the program does not match its approval, revoke the old approval on the Source Applications panel first. |
| The state line names a runtime problem, or `QAROS: runtime` shows a path that is not the installed QAROS. | Remove a `qar-streaming-c.dll` that lies beside `rviz2.exe` or on `PATH`, and unset `QAROS_RUNTIME_DIR`, or install QAROS ([Troubleshooting](/docs/developer-guide/troubleshooting#joining-qaros-as-a-local-app)). |
| RViz2 ends when you press **Connect to QarOS**. | The C++ runtime beside `rviz2.exe` is older than `14.40`, and the log of RViz2 holds a line about a `SEH FAULT` in `qar_library_init` that names `MSVCP140.dll`. Close RViz2, run `scripts\fix_old_vc_runtime.ps1` and start RViz2 again. |
| The state line reads connected, but `streams=0` stays on the heartbeat and the list **Targets** is empty. | The Hub lists no target. Open a Visualizer from QAROS and, if it has no receiver, add one in it (see [Targets and streams](#targets-and-streams)), or bring a device in ([Devices](#devices)). |
| A target is listed as `not served: cap reached`. | The plugin serves at most 8 targets; raise `QAR_PANEL_MAX_PEERS` if your machine can render for more. |
| A stream is open, and the room shows only the grid or nothing. | RViz2 has nothing to show: the default scene draws markers only while `scripts\publish_test_markers.py` runs, which the launcher starts only with `-TestMarkers`. Start a publisher or play a bag. Also check that `QAR_PANEL_APP_VOLUME` is not `off`: QAROS places a stream in the scene by its app volume. |
| You click a panel in AR and the control does not react. | Read the first number of the readout `AR panels:`, in the form `input: in N, missed N, dropped N`. With `in 0` the runtime sends nothing to the plugin, so look at the session and the runtime and not at the panel. A growing `missed` means that the pointer aims at nothing, and a growing `dropped` that the sequence of events was wrong. |
| An AR tool opens in the headset, but its page does not load. | The headset has to reach the host and the port in the address. Read the log line `AR TOOL: panel server listening on`. The port changes at every start unless you pin it with `QAR_PANEL_WEB_PORT` and open it in the firewall, and on a machine with several networks the host can be the wrong one: set `QAR_PANEL_WEB_HOST` to an address of the machine in the network of the headset. |
| A click in the scene publishes nothing on `/clicked_point`, `/goal_pose` or `/initialpose`. | **Mode** starts at `Off`. A pick waits for **Confirm pick** unless **Publish without confirming** is ticked. A click that hit nothing is refused and counted as `clicks-on-nothing=` in the status, and a hover only previews. |

## Environment variables

Variables that you set in the PowerShell window before you start the launcher or RViz2. A saved setting of the panel is overridden by its variable, and the control is then disabled. The launcher exports the variables that its parameters name; a value from your shell wins over a default of the launcher.

| Variable | Effect |
|---|---|
| `QAR_PANEL_LIVE_STREAM` | Any value that is not empty starts the panel in live mode. The launcher sets it. |
| `QAR_PANEL_HUB_MODE` | `auto`, `local` or `remote`: the Hub to join. |
| `QAR_PANEL_HUB_HOST`, `QAR_PANEL_HUB_PORT` | The host and the onboarding port that the fields of a code pairing start from. The code is never an environment variable. |
| `QAR_PANEL_APPROVAL_TIMEOUT_S` | Seconds that the Hub waits for the **Allow** of a local join, 1 to 600. Without it the Hub's default of 120 seconds applies. |
| `QAR_PANEL_INSTANCE` | A name that is added to the identity folder and to the names of the plugin's QAROS panels, for a second RViz2 on the same PC. |
| `QAR_PANEL_IDENTITY_DIR` | Replaces `%LOCALAPPDATA%\Quaternar\QarOS-ROS` as the parent of the identity folder. |
| `QAROS_RUNTIME_DIR` | A folder with another QAROS runtime for the library lookup (the vendor's variable; `-RuntimeDir` of the launcher sets it). |
| `QAR_PANEL_INVITE_PEERS` | `hololens:<host>,headset:<host>`: devices to invite once when the session first connects; ignored in the mode **Another PC**. |
| `QAR_PANEL_MAX_PEERS` | The ceiling of served targets, 8 by default. |
| `QAR_PANEL_HIDDEN_DISPLAYS`, `QAR_PANEL_AR_CONTROL` | `off` stops the automatic start of the hidden Displays copy and of the AR control panel at connect. |
| `QAR_PANEL_AR_CONTROL_PER_PEER` | `off` shows one AR control panel for everybody instead of one in front of each target. |
| `QAR_PANEL_AR_VISIBILITY` | `hide` (the default) or `close`: hide and keep AR panels, or close them. |
| `QAR_PANEL_CLOSE_MODE` | `destroy` (the default) or `notice`: what happens to the AR copy of a window that closes, while hiding is off. `notice` keeps the AR panel as a notice and keeps its video sender until the session ends. |
| `QAR_PANEL_RESIZE` | `swap` (the default) or `new-panel`: how a streamed window follows a size change. |
| `QAR_PANEL_FIXED_POSES` | `room` (the default) or `ground`: hang the panels with a fixed place from a fixed height of the room or from the RViz2 ground. The launcher sets `ground`. |
| `QAR_PANEL_WEB_HOST`, `QAR_PANEL_WEB_PORT` | The host name in the addresses of the AR tools, and the port of their server. |
| `QAR_PANEL_WEB_SCALE`, `QAR_PANEL_WEB_SIZE` | A multiplier on the size of a URL panel (0.25 to 4), or a size in meters as `<w>x<h>`. |
| `QAR_PANEL_APP_VOLUME`, `QAR_PANEL_VOLUME_NAME`, `QAR_PANEL_VOLUME_SIZE`, `QAR_PANEL_VOLUME_SCALE`, `QAR_PANEL_VOLUME_Y` | The app volume of the scene: `off`, its display name, the edge of its cube in meters, its scale and the height of its center in meters. |
| `QAR_PANEL_NEAR_M`, `QAR_PANEL_FAR_M` | The clip distances of the scene, 0.01 m and 150 m by default. |
| `QAR_POSE_WORLD`, `QAR_DEPTH_ENCODING` | `zup` and `ndc`: the world axes and the depth encoding that the launcher sets. |
| `QAR_PANEL_LOG_DIR`, `QAR_PANEL_LOG_SEVERITY`, `QAR_PANEL_LOG_CONSOLE` | The folder, the severity and the console echo of the QAROS library inside RViz2. |
| `QAR_GUI_CALL_WARN_MS` | The limit, in milliseconds, above which a QAROS call on the Qt thread is reported as slow; 20 by default. |

A variable of the earlier session start-up (the invitation of a Visualizer, the admission of peers, the runtime folder) has no effect with the installed runtime; when one is set, the plugin writes `ignored: <name> has no effect with this QAROS` to its log. The saved settings of an earlier plugin are read and migrated: the setting for peer admission is dropped, and an invitation kind that the Hub cannot start is read as `hololens`.

## Known issues

### The Visualizer crashed when a panel was created again

With the SDK drop of 2026-09-28, the window of the QAROS Visualizer disappeared within a second of the creation of the hidden Displays panel in each of the 25 cases that the authors of `qar_rviz_plugin` observed in which the plugin created that panel again after it had been closed. The windows of all other running Visualizers disappeared with it, QAROS started a Visualizer again after several seconds, and until then the 3D streams stood still. In one longer session the Visualizer crashed five more times, each within a second of the close or the creation of a panel. Every crash report of these crashes named the same access violation, which reads address `0x4` at the same place inside `WebCore.dll`, the browser engine library of the Visualizer.

The plugin with the installed runtime hides its panels and finds them again by name instead of closing and creating them, which removes both triggers from the default path. No Visualizer crash appeared in the authors' runs of 2026-10-24 and 2026-10-25, but those runs closed no panel, so it is **not known** whether the crash happens on the installed runtime when a panel is closed. `QAR_PANEL_AR_VISIBILITY=close` selects the closing behavior; if you use it and the Visualizer disappears, send the crash reports (the files whose names end in `crash.txt`, in the QAROS log folders) with the logs of the run to the authors of `qar_rviz_plugin`, and say whether a log line with `gets the QarOS panel name` stood before the crash.

What you can do: leave `Hide panels in AR instead of closing them` on, close and start panels at a calm pace, and, if you need panels closed, set `QAR_PANEL_CLOSE_MODE=notice`, which keeps the AR panel of a closed window as a notice and keeps its video sender until the session ends.

### Lines at the end of a run

When the last Visualizer has closed, or while RViz2 closes, the log can hold the lines `submit_phase: sender rejected the frame` and `wglMakeCurrent FAILED`. Neither line is a failure by itself. RViz2 used to leave a Windows crash report when it closed. The authors removed the two causes, which were objects of the plugin that outlived its library, and two runs after the change ended without a new crash report; judge a run by its heartbeat lines.

## Related pages

- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries, how they stack and what to install.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how `qar_ogre_adapter` streams an Ogre scene, which the plugin does for the scene of RViz2.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — how a GUI panel shows a video, and what the Qt library does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, and what a QAROS query costs.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the local app, the approval and the pairing code that the plugin uses.
- [Source Applications](/docs/operator-guide/source-applications) — the approved local apps, and the applications that stream content into the room.
- [Connection Panel](/docs/user-guide/connection-panel) — pairing another Hub and inviting devices from the Visualizer.
- [Running the Hub](/docs/operator-guide/running-the-hub) — the Hub that the plugin joins.
