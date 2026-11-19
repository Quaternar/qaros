---
sidebar_position: 4.6
title: Running the RViz2 Plugin on Windows
description: Start RViz2 with the QAROS plugin on a Windows 11 workstation, join the QAROS Hub of this PC, and bring the live 3D scene, RViz2 panels, ROS 2 graph tools, video and picks into AR.
---

# Running the RViz2 Plugin on Windows

The **RViz2 plugin** `qar_rviz_streaming` brings a ROS 2 system into AR from a Windows 11 workstation. It adds one panel, **Qar Streaming**, to RViz2. The panel joins the QAROS Hub, and for every target that the Hub lists (a headset, a receiver of a Visualizer) the plugin renders the live 3D scene of RViz2 from the head pose of that target and streams it as stereo video. It also shows RViz2 panels as GUI panels in AR, generates AR tools from the topics, services and actions of the ROS 2 graph, shows ROS 2 image topics as video panels, and publishes the picks of the headset on the topics of the RViz2 tools.

The installation, the configuration reference and the log reference stay in the repository [github.com/Quaternar/qaros-ros-rviz](https://github.com/Quaternar/qaros-ros-rviz); this page names the documents and does not copy them. [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) describes the libraries that the plugin builds on.

:::info Which QAROS runtime
The live checks that this page reports were made with the installed QAROS runtime **0.9.0-dev.3008+65dc9aeb** (release flavor), a development build of the setup that precedes the 0.9.0 release, between 2026-10-24 and 2026-11-19. The plugin and its libraries are now built against the C API header that their repositories pin as the git submodule `external/qaros` (QAROS commit `0f2cb2e`, on the 0.10.0-dev line). That header lists functions that the dynamic loader requires, and its own rule is that a runtime without a required function fails the load of the module. The export table of the 0.9.0-dev.3008 runtime, read from the file without loading it, lacks 13 of those functions, so a plugin built from the pinned header is not expected to load that runtime. The page [RViz (ROS 2)](/docs/user-guide/supported-apps/rviz) names QAROS 0.9.0 or newer and the Visualizer's install of 0.10.0. The plugin built on the pinned header has not been run against any runtime in a live session; everything below that was observed was observed with the build before that move.

The plugin joins QAROS the way [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) describes: as a **local app** on the PC that runs the Hub, and on no other path. To work with a Hub on another PC, make the Hub of this PC a member of it ([A Hub on another PC](#a-hub-on-another-pc)). The plugin does not start a Visualizer; you open one from QAROS yourself. The sections that describe a path that the authors have not run in a live session say that this path is designed and not tested. Linux is not verified: the plugin has not been built there.
:::

## What you need before you start

The document `docs/INSTALL.md` of the repository installs everything below from a clean Windows 11 machine, step by step. You need:

- **Windows 11, 64-bit, at the console of the machine.** RViz2 and the Visualizer open OpenGL windows, and the documented runs were made from the local console; Remote Desktop and services are not covered. Linux is not built or tested.
- **An NVIDIA GPU with a current OpenGL driver.** The C API currently supports NVIDIA adapters only, as its header states, and no minimum GPU or driver version is stated. On a laptop with an integrated and a discrete GPU, RViz2 should render on the adapter that QAROS uses, or every picture passes through a copy between the adapters, so the live launcher sets the graphics preference of `rviz2.exe` to high performance.
- **PowerShell 7.** The scripts and every command on this page run in `pwsh`.
- **The ROS 2 Humble binary distribution for Windows, repaired.** The plugin runs inside the `rviz2.exe` of the official binary release. That release has gaps: some runtime libraries are missing, the C++ runtime beside `rviz2.exe` is too old for the QAROS library, and `tinyxml.dll` is missing. The repository ships the scripts that repair the distribution in place, and the plugin needs all nine repairs.
- **The plugin and the libraries it builds on.** `colcon` builds the plugin against the installed packages of `qar_ogre_adapter` ([github.com/Quaternar/qaros-ogre](https://github.com/Quaternar/qaros-ogre)) and `qar_qt_streaming` ([github.com/Quaternar/qaros-qt](https://github.com/Quaternar/qaros-qt)), at the versions that `CMakeLists.txt` of the plugin requests, on Qt 5.15.2. The two libraries are independent of each other, and what only the plugin needs (the repair scripts for the ROS 2 distribution, `requirements-ros.txt`, and the glue that lets the Qt panels use the plugin's QAROS session) is in the plugin's repository. The three repositories take the QAROS C and C++ headers from the git submodule `external/qaros`.
- **A QAROS Hub to join.** QAROS installed on this PC ([Installation](/docs/operator-guide/installation)), so that the plugin can join its Hub as a local app. For a Hub on another PC, make the Hub of this PC a member of it ([A Hub on another PC](#a-hub-on-another-pc)). The plugin loads the installed runtime through the vendor's runtime lookup (`qar_library_load(NULL)`), so QAROS needs no entry in `PATH`; a `qar-streaming-c.dll` beside `rviz2.exe` or a `QAROS_RUNTIME_DIR` would win over the installation, and [Troubleshooting](/docs/developer-guide/troubleshooting#joining-qaros-as-a-local-app) says what that causes.
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

Run the live launcher without `-Confirm` first. It takes the locations from the parameters `-SdkRoot`, `-Prefix`, `-QtRoot` and `-Ros2Root` or, for a parameter that you omit, from the environment variables `QAR_SDK_ROOT`, `QAR_PREFIX`, `QAR_QT_ROOT` and `ROS2_INSTALL_ROOT`. `-SdkRoot` is optional and only feeds the checks and the printed plan: it defaults to the install folder that the QAROS setup records in the registry. The dry run prints the scene, the installed QAROS version, whether the Hub processes run, which Hub the panel joins and whether it connects by itself, the settings and the logging mode. It reads the machine and nothing else: it starts nothing, changes nothing and, when it finds the locations, ends with exit code `0`:

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

The launcher does **not** start the QAROS Hub or a Visualizer. When the panel joins and no Hub runs, the QAROS runtime starts the Hub itself (the comment on `QarOnboardLocalAppExt` in `qar_streaming.h` says so; see [Join the Hub of this PC](#join-the-hub-of-this-pc)); a Visualizer you open from QAROS yourself. These parameters matter most:

| Parameter | Effect |
|---|---|
| `-HubMode` | `auto` or `local`, kept for older command lines: every value joins the Hub of this PC. |
| `-ArCopyScale` | The factor of the enlarged AR copies of Qt panels, 1.0 to 3.0 (default 2); it sets `QAR_PANEL_AR_COPY_SCALE`. |
| `-InvitePeers` | Devices to invite as soon as the panel has joined the Hub of this PC, as `hololens:<host>` entries (an Android headset is never invited; the operator pairs it in the Visualizer). |
| `-RuntimeDir` | A folder with another QAROS runtime for the library lookup. It sets `QAROS_RUNTIME_DIR` and is not set unless you pass it. |
| `-Scene` | The RViz2 configuration to load, instead of the default test scene. |
| `-LogDir` | The folder of the RViz2 logs. The default is `qar_rviz_logs` in your `TEMP` folder. |
| `-TraceLogs` | Collects the full QAROS and adapter logs that a bug report needs. They can grow to gigabytes, so do not judge stalls in this mode. |
| `-TimingCsv` | Records the timing of every video stage in CSV files. |
| `-TestMarkers` | Starts the test publisher with the launch. |

The launcher also sets `QAR_PANEL_AUTO_CONNECT=1` unless your shell has a value: the live panel then presses **Connect to QarOS** itself, once, shortly after it has loaded ([Join the Hub of this PC](#join-the-hub-of-this-pc)). The document `docs/CONFIGURATION.md` lists every parameter. [Environment variables](#environment-variables) names the variables that the launcher exports.

### Open the panel

The scene of the launcher loads the panel. RViz2 opens with the panels Displays, Views and **Qar Streaming**. Without auto-connect the panel connects to nothing until you press **Connect to QarOS**; with the launcher it connects by itself. An RViz2 configuration of your own loads the panel with this entry in its `Panels` list:

```yaml
Panels:
  - Class: qar_rviz_streaming/QarStreamingPanel
    Name: Qar Streaming
```

If the panel is missing, either the shell that started RViz2 did not source the overlay of the install prefix (`local_setup.ps1`) or the plugin is not installed into that prefix. A launcher sources the overlay itself, and `docs/TROUBLESHOOTING.md` has the checks. If you start `rviz2.exe` without a launcher, you have to set the variables that the launcher sets. `QAR_PANEL_LIVE_STREAM` comes first, because without it the panel starts in the offline preview. The section "Running RViz without the launcher" of `docs/CONFIGURATION.md` lists them.

### The Qar Streaming panel

The panel is a status strip over six tabs: **Connection**, **AR panels**, **AR tools**, **Picking**, **Settings** and **Diagnostics**. The strip shows the state of the connection and the last error. The sections below name the controls that you use, and the document `docs/OPERATOR_GUIDE.md` lists every control.

- **Connection** holds the buttons **Connect to QarOS** and **Disconnect (pause)**, the group **QAROS Hub** (who you are to the Hub, the state, the targets), the group **Invite a device** and the group **AR control panel**.
- **AR panels** lists every AR panel and streams RViz2 windows into AR.
- **AR tools** generates AR tools from the ROS 2 graph.
- **Picking** turns the pointer of the headset into ROS 2 messages.
- **Settings** keeps the values that the panel saves in the RViz2 configuration.
- **Diagnostics** shows the state of the display context, the readout of the AR panels (ten lines) and the log, takes test captures and, in the offline preview, adds simulated peers.

In live mode the two runtime buttons read **Connect to QarOS** and **Disconnect (pause)**. In the offline preview the same buttons read **Start Qar** and **Stop Qar**, the group **QAROS Hub** shows its settings only, and its state line reads `QarOS: preview mode - set QAR_PANEL_LIVE_STREAM to connect`.


## Joining the QAROS Hub

The plugin builds its QAROS session when you press **Connect to QarOS** or, with auto-connect on, once after the panel has loaded, and never earlier. The session is built by a thread of its own, the **session owner**, so the press returns at once and RViz2 stays responsive; the render tick of RViz2 collects the progress and the result. The panel joins the Hub of this PC and offers no other choice: the group **QAROS Hub** has no drop-down of Hubs, no host, no port and no field for a pairing code.

The plugin identifies itself to the Hub as `RViz2 (QAROS ROS plugin) on <name of this PC>`; the approval prompt of the Hub shows that text separately, as the text the app says about itself.

### Join the Hub of this PC

Everything the vendor documents about the local join applies unchanged: [Local app: on the QAROS PC, no code](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code) describes the call and the approval prompt, and [Approved local apps](/docs/operator-guide/source-applications#approved-local-apps) shows how an operator lists and revokes the approvals.

1. **Connect.** Press **Connect to QarOS**. The state line reads `QarOS: loading the QAROS runtime ...` and then `QarOS: waiting for approval in the QAROS Hub - click Allow in the QAROS window`, followed by a countdown of the time left. If no Hub runs on this PC, the join starts QAROS: the comment on `QarOnboardLocalAppExt` in the C API header `qar_streaming.h` says "Starts the QAROS hub when it does not run" and that a running Hub is left as it is. The vendor's [onboarding page](/docs/developer-guide/onboarding-and-sessions#local-app-on-the-qaros-pc-no-code) lists `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE` (1802) for a Hub that cannot be reached; the state line shows the message of the table below when the Hub could not be started.
2. **Allow.** The first time, the Hub opens the window `Allow <app> to join QAROS?` in front of every other window. Check what it reports about `rviz2.exe` and press **Allow**. Nothing is stored by the plugin; the Hub remembers the approval. The `rviz2.exe` of the ROS 2 distribution is not code-signed, so the approval is for one exact file and the Hub asks again after RViz2 is updated or replaced.
3. **Connected.** The state line reads `QarOS: connected (local app) - <n> target(s), <k> served`, and the identity line reads `This PC: approved local app (...)`. Later starts of an unchanged `rviz2.exe` join without a prompt.

The prompt waits 120 seconds by default. `QAR_PANEL_APPROVAL_TIMEOUT_S` sets another wait, from 0 to 600 seconds: 0 is the Hub's default of 120 seconds, and a larger value is lowered to 600 without a message; a value that is not a number of seconds, or is negative, is reported in the log and the default applies. **Cancel** stops the wait. The plugin has no button that revokes an approval: the operator does that in the Visualizer, on the Source Applications panel. The group **Invite a device** works with this join ([Devices](#devices)).

**Auto-connect.** With `QAR_PANEL_AUTO_CONNECT` set to `1`, `true`, `on` or `yes`, the live panel presses **Connect to QarOS** once, about half a second after it has loaded, when no join is under way; it is the same path as the button. A failed join shows its failure and waits for you (**Connect to QarOS** again, or **Cancel**), and **Disconnect (pause)** works as always. The variable is not a saved setting. Without `QAR_PANEL_LIVE_STREAM` (the offline preview) it has no effect, and the log says so. The launcher `run_live_rviz_stream_real.ps1` sets it unless your shell has a value, and QAROS sets it when it starts RViz2 from the **ROS 2 Rviz** tile of the Visualizer. The authors have not run auto-connect in a live session.

What the authors ran with the runtime 0.9.0-dev.3008: the join with **Allow** and a second start of RViz2 without a prompt (2026-10-24 and 2026-10-25), and, between 2026-11-09 and 2026-11-19, **Deny**, no answer within the wait followed by **Allow**, a Hub that was quit from its tray icon or whose process ended, a Visualizer that was stopped, and the closing of RViz2 after a Hub restart ([What has been seen](#what-has-been-seen-and-what-has-not)). A Hub that is suspended and the invitation of a device are **designed and not tested** in a live session. The plugin shows these messages in the state line and in `Last error:`:

| Situation | Message |
|---|---|
| **Deny**, a closed prompt or a revoked approval | `QAROS denied this app (Deny was clicked, the prompt was closed, or the approval was revoked). Connect again and click Allow.` |
| No answer within the wait | `Nobody answered the approval prompt in the QAROS Hub in time. Connect again and click Allow in the QAROS window.` |
| No Hub could be started | `The QAROS Hub is not running and could not be started. Start QAROS from the Start menu, then connect again.` |
| The approval no longer matches `rviz2.exe` | `This program does not match what was approved (moved, copied or changed). Revoke the old approval in the Visualizer (Source Applications) and connect again.` |

### A Hub on another PC

The plugin does not pair with a Hub on another PC: the Connection tab has no pairing code, no **Pair** and no **Forget pairing**, and the plugin stores no pairing and reads no identity file. Install QAROS on this PC, make its Hub a member of the other ([Pair another Hub](/docs/user-guide/connection-panel#pair-another-hub)) and join the Hub of this PC as a local app ([Remote app: through a member hub](/docs/developer-guide/onboarding-and-sessions#remote-app-through-a-member-hub)). A configuration saved by an earlier plugin that holds `Hub Mode` `remote` is read as `local`, and its `Hub Host` and `Hub Port` are dropped, with a sentence in the log. The variables `QAR_PANEL_HUB_HOST` and `QAR_PANEL_HUB_PORT` have no effect; when one is set, the panel logs `SETTINGS: ignored:` and its name once. `QAR_PANEL_HUB_MODE` accepts `auto` and `local`, which both join the Hub of this PC, and refuses `remote` with a warning.

### Targets and streams

The Hub lists the peers of the session. The plugin renders for those whose role is `TARGET` and whose presence is online or unknown ([Finding rendering targets](/docs/developer-guide/rendering-streams#finding-rendering-targets) and [What a peer is](/docs/developer-guide/peers#what-a-peer-is-role-category-and-device) describe both). The list **Targets** in the group **QAROS Hub** shows each one as name, kind, presence and `served` or `not served` with the reason; it shows eight rows and scrolls when the Hub lists more. A target that goes offline, leaves or retires loses its stream; one that was offline and comes back online gets a new stream. Each served target gets its own stereo stream of the scene, and the plugin serves at most 8 targets at once (`QAR_PANEL_MAX_PEERS` changes the ceiling; a target beyond it is listed as `not served: cap reached`), because every target costs a color and a depth render of the scene for each eye in every frame.

A Visualizer lists a receiver only after one exists in it. In the runs of the authors (2026-10-24) a freshly started Visualizer had no receiver, and the Hub listed a target for it only after **Add receiver** was pressed in the Visualizer; the plugin then started the stream by itself.

### Devices

The group **Invite a device** asks the Hub of this PC to start the app of a device: choose **HoloLens 2**, type the host name or address of the device, and press **Invite peer**. The choice **Headset (Quest, Android)** fails with `QAR_STATUS_HUB_TARGET_APP_DEVICE_NOT_INVITABLE`: an Android headset is never invited, the operator pairs it once in the Visualizer and it joins by itself. The call is made on the session owner's thread, the button reads `Inviting...` until the Hub answers, and the result is logged. The button is grey unless the panel has joined the Hub of this PC and streams, and its tool tip says what it needs. In the Visualizer's [Devices](/docs/user-guide/connection-panel#devices) tab, the **HoloLens (legacy)** tab of the **Invite device** card invites a HoloLens by address, and **All devices** › **Pair headset** opens the **Pair Headset** dialog that pairs an Android headset. `QAR_PANEL_INVITE_PEERS` (and `-InvitePeers` of the launcher) invites the listed devices once when the session first connects; an entry that fails is logged and skipped. **Invites were not run in a live session.**

### Pause and the end

**Disconnect (pause)** stops the streaming of the 3D scene and keeps everything else: the session stays open, and the AR panels, the AR tools and the AR control panel stay open and keep running. It pauses and does not disconnect. **Connect to QarOS** resumes the streaming on the open session at once, with no new join. Everything the plugin built on the session is released only when the session ends on its own and when RViz2 closes.

When RViz2 closes, the plugin, with hiding on (the default, see [RViz2 panels in AR](#rviz2-panels-in-ar)), hides its QAROS panels and lets go of their senders and subscriptions instead of closing them, destroys the render senders, and cancels a wait that is still running.

### When QAROS stops answering

RViz2 stays operable when QAROS stops answering, whether the Hub was quit, crashed or hangs, or the Visualizer stopped. Every QAROS call that can wait for the Hub runs on the thread of the session owner or on the frame worker of a 3D sender or of a panel, and the Qt thread of RViz2 only hands them work and never waits for them. The strip and the state line say where things stand:

| State | The strip says | What is going on | Buttons |
|---|---|---|---|
| The Hub is silent | `QarOS: Hub not reachable - waiting for it to come back`, then the seconds in parentheses | The session is kept and nothing is torn down. The Hub counts as silent when every runtime peer has been gone for 3 seconds, one call of the owner thread has run for 10 seconds or one task for 30 seconds. | **Join again** gives the session up at once |
| The session is given up | `the QAROS Hub is gone`, then that the plugin is releasing the session, then `joining it again in` with the seconds and the attempt, and that RViz2 keeps working | After 20 seconds of silence, or 5 seconds when the endpoint file or the process of the Hub is gone, everything built on the session is released and the owner joins again by itself: after 2 seconds, then 4, 8, 16 and 30, and then every 30 seconds, with no limit on the number of attempts. | **Stop rejoining** ends the automatic joining; **Connect to QarOS** joins again at once |
| An attempt runs | `QarOS: joining the QAROS Hub again` with the attempt | The Hub is asked to accept the program again; when it shows its approval prompt, the strip shows the prompt text with the countdown. | **Stop rejoining** (**Cancel** while the prompt is shown) |
| The joining ended | The sentence of the cause, for example that the program was denied or that nobody answered the prompt, and that **Connect to QarOS** joins again | The failure is not tried again by itself. | **Connect to QarOS** |
| The thread of QAROS was left behind | `QAROS stopped answering inside a call (`, the name of the call, and `RViz keeps working; restart RViz to stream again.` | A QAROS call did not return for 45 seconds, twice in this RViz2; RViz2 keeps working but streams no more. | none that helps; restart RViz2 |

When the joining succeeds, the panel makes its factory and its streams again, a stream opens for each target that is online, and the AR panels that streamed before the loss are made again one by one and found under their names. The AR control panel stays shown while the 3D stream of its target is made again. The panel also gives a session up by itself when, 45 seconds after the Hub answers again, no 3D stream has submitted a frame since and no target is online (`QAR_PANEL_REJOIN_AFTER_HUB_S` sets the seconds, `0` never gives it up), at most once in 60 seconds.

While a 3D stream is in trouble for 3 seconds or more, the line of the connection adds a note, for the stream that has been in trouble longest:

| The note says | Meaning |
|---|---|
| `the receiver is reconnecting` with the seconds | The receiver of the stream, the Visualizer or the device, is joining again; the picture comes back by itself. |
| `QAROS has not asked for 3D frames since it reconnected` with the seconds, `RViz makes the 3D stream again if this goes on` and `a known QAROS issue after a Hub restart, the 3D view can take minutes` | After a restart of the Hub the runtime may send no request for the frames of the stream that it replays; the plugin makes a new 3D stream after 45 seconds without a request, counted from the end of the reconnecting, and a Visualizer may keep asking for the old one, so the view can take minutes to return, or may not return ([Known issues](#known-issues)). |
| The 3D stream of the target `is being set up again` with the attempt | The creation of the sender did not answer. If it stays, the Visualizer may still wait for an earlier RViz2: close the Visualizer and start it again. |
| `is paused (a frame call blocked)` | Two frame calls were slow; the frame calls of that stream pause and are tried again. |
| `receiver` with the name, `left; waiting for it to come back` | The target went offline; the note stays for a minute. |

**Closing RViz2 while QAROS does not answer.** Closing RViz2 waits for QAROS only for a few seconds. The panel hands what it built on the session to the owner and the frame workers, which get a few seconds to finish; a QAROS call that does not return is left behind and the plugin stays loaded so that the thread can return into its own code. An exit guard, a thread that the panel makes before it loads the first QAROS library, ends RViz2 itself 5 seconds after the application was asked to quit if it has not ended by then, and writes a line that names the last step. It is armed only when the application is about to quit, never when a panel is merely removed. `QAR_EXIT_GUARD=off` leaves the exit to RViz2. In the authors' checks, five closes with QAROS gone or hung took about 4 to 5 seconds (2026-11-11); closing with a suspended Hub was not run.

**A size change of a streamed window.** By default a size change of a streamed window, an RViz2 dock that you resize, makes a second QAROS panel for the new size: the new panel is shown, the old one is hidden and kept, and no QAROS panel is closed while the session runs; a visible panel never has its video sender released and made again. A return to a size that is kept shows the kept panel again. A stream keeps at most 8 hidden panels as a ring; with the ring full, the next size is made on the oldest hidden panel. A size is adopted only after it has held for 1.5 seconds, while no mouse button is held, and at most once in 10 seconds for one panel, so a drag of a splitter does not rebuild anything on the way. `QAR_PANEL_RESIZE=swap` keeps the QAROS panel and swaps its video sender, which can stall the graphics driver while the panel is visible; `defer` swaps only while the panel is hidden in AR.

### How the plugin keeps RViz2 responsive

No QAROS call that may wait runs on the Qt thread of RViz2. The library load, the onboarding, the invites and every call that reads or changes a GUI panel or an app volume are queued for the session owner's thread, which runs them in order; the Qt thread collects the results at the render tick. This matters because a query of the installed runtime is slow: [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#how-long-a-query-takes) lists the measured costs, about 2 seconds for each query. A plugin that made four queries on the Qt thread in every render tick froze RViz2 for 8 seconds at a time; the design above removed that. On top of that:

- **Frame workers.** Each 3D sender and each streamed panel has a thread of its own that makes the QAROS calls of the frame path. A 3D frame ends its copy with a GPU fence, and the worker waits for the fence, so the Qt thread waits neither for QAROS nor for the GPU copy (`QAR_FRAME_CALLS=gui` and `QAR_FRAME_FENCE=gui` select the earlier paths, which stall for as long as a call waits).
- **A GPU progress gate.** A watch thread follows a fence of every RViz2 frame and every 3D render batch, and the frames pause while the oldest unfinished one is older than the larger of 50 ms and three times the median frame time, so a GPU fault does not pile up work (`QAR_GPU_GATE=off` never pauses).
- **A stall watchdog.** When the render tick stops for 3 seconds, a `[qar-stall]` line names the step of the Qt thread; `QAR_STALL_DUMP=on` also writes a minidump after 5 seconds.
- **Call timing.** A QAROS call that takes more than 20 ms on the Qt thread writes `[qar-gui-call] ... WARNING <call> took <n> ms on the GUI thread` to the log of RViz2; `QAR_GUI_CALL_WARN_MS` changes the limit. A call that takes 500 ms or more on the session owner's thread writes `QAROS: call <name> took <n> ms`, and a queued task that takes a second or more `task '<name>' took <n> ms`. These lines are normal for the queries above and are not errors.

**The render tick.** Everything the panel does repeatedly (the heartbeat, the AR panels, the AR tools, the picks, the floor plane and the streams) runs in one handler of one timer on the Qt thread. While nothing streams, the timer runs every 50 ms, the idle tier. While the panel is connected and at least one 3D stream or one streamed Qt panel exists, the timer runs at the configured period, 0.1 ms by default (`QAR_PANEL_TICK_MS`), the streaming tier: a timer without an interval, which fires whenever the event loop is idle, with the body of the tick running at most once in 100 microseconds. The pages of the AR tools and the AR control panel do not hold the streaming tier. The log says `Render tick: idle, every 50 ms (nothing to stream)` or `Render tick: streaming, 0.1 ms` at every switch, and the heartbeat carries `tick=<configured period> prime=<0|1> tier=<idle|stream>`. The idle tier answers the report that an idle panel ran its tick about 9,000 times a second. A headless test of the timer plan with Qt alone (no RViz2, no QAROS) measured about 20 callbacks a second and 2 % of one core for the idle plan, and about 164,000 callbacks a second (9,272 tick bodies) and 89 % of one core for the streaming plan. The panel inside RViz2 was not measured, so the idle figures for RViz2 are expected and not observed.


## What you see and do in AR

### The 3D scene

The plugin renders the scene once for each target, from the head pose that QAROS reports for that target, and sends a color picture and a depth picture for each eye. The view in the Visualizer follows the pose of the Visualizer and not the camera of RViz2: orbiting the camera of RViz2 changes only the picture on your desktop. To make the scene readable in AR, the plugin adds an opaque gray plane 1 mm under every Grid display, so that the depth picture shows a solid floor instead of gaps between the grid lines. While it streams, the plugin lights the scene with a rig that is fixed in the world and suppresses the headlight of RViz2, so that the shading does not swing when you orbit the view.

The scene is placed in the room by an **app volume**, a cube of 10 m that the plugin gets or creates under the stable name `scene.rviz2-plugin.<name of this PC>.qaros-ros` (shown as `RViz Scene`); the next start finds the same volume again. `QAR_PANEL_VOLUME_SIZE`, `QAR_PANEL_VOLUME_SCALE` and `QAR_PANEL_VOLUME_Y` change its edge, its scale and the height of its center, and the launcher sets the center to -1.6 m when you did not set one. `QAR_PANEL_APP_VOLUME=off` makes every stream refuse with a legible message, because QAROS rejects a stream without an app volume. For a volume that already exists and is not closed, QAROS ignores those values and the plugin takes the volume over; the plugin reads the placement back from QAROS instead of assuming it.

**Resolution:** and **FPS cap:** on the Settings tab start at 1280 x 720 and 60 frames per second. They apply to the streams that are created from then on and do not resize a stream that exists. The plugin renders all streams that are due in one batch, with a budget of about 16 ms, and sheds the frames of the targets beyond it, except that a target that has not been rendered for 1/30 s is always served. Choose a lower resolution when you serve several headsets.

The launcher sets `QAR_POSE_WORLD` to `zup` and `QAR_DEPTH_ENCODING` to `ndc`. Without the first the scene arrives on its side, and without the second the Visualizer showed the depth in one color with an SDK drop earlier than 2026-09-28; whether the installed runtime still needs it was not tested separately. If you start RViz2 without the launcher, set both.

The plugin also reads the fixes of a `sensor_msgs/msg/NavSatFix` topic, `/gps/fix` unless the environment names another, and tells QAROS from them where the origin of the scene lies on Earth. This path does nothing while the topic is silent.

### RViz2 panels in AR

The tab **AR panels** streams a window of RViz2 as a **GUI panel** and lists every AR panel in one table with the columns **Panel**, **Kind**, **Seen by**, **In AR** and **Size**. The rows are **This QAR panel**, every dock of RViz2 by its title, the dialogs that are streamed, one row `Video: <topic>` for each video panel that an AR tool opened, the hidden Displays copy, the AR control panel and each AR tool. The table shows eight rows; **Previous** and **Next** page through more. The list follows the docks of RViz2 by itself, a dock that you add or close appearing or leaving within about half a second, and **Refresh** scans again.

The column **In AR** says `off` for a panel that is not streamed, then `starting`, `shown`, `hidden` for a panel that is hidden in AR, `parked` for a panel whose window closed, `closed in AR` for a panel that the person closed in AR, `notice`, `waiting for a receiver`, or `error:` and the first characters of the error. The button under the table acts on the selected row: **Stream to AR** for a panel that is not streamed, **Hide in AR** for a panel that runs while hiding is on, **Stop streaming** for one that runs while hiding is off, and **Show in AR** for a panel that is hidden, parked or closed in AR. The size button reads **Native size in AR** or **Enlarged in AR**. The press of **Stream to AR** makes no call into QAROS on the Qt thread: the QAROS panel is created on the next render tick, once the widget is laid out, by the session owner's thread. The mark `closed in AR` was never seen in the authors' checks, because the Visualizer offered no way to close a panel.

- **Placement.** A panel hangs with its top edge 1.5 m above the room origin, or 1.5 m above the RViz2 ground when `QAR_PANEL_FIXED_POSES=ground` (the launcher sets that unless you did), and is kept at least 0.1 m above the ground. The first panel hangs 1 m in front of the origin, and each further panel 0.65 m further along the x axis (times the factor of an enlarged copy). No call of QAROS asks where the operator is, so the plugin cannot place a panel in front of you ([Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#where-the-operator-is) explains why). The AR control panel is the exception, and is placed in front of each target once the view of that target is streamed.
- **Input.** A click and a hover on the panel reach the real widget of RViz2, including the lists of drop-down boxes and the dialogs that the widget opens. The input never reaches a menu of RViz2 itself: a popup that belongs to RViz2 and not to the streamed panel takes no AR input, so a hover in AR does not move the highlight of the File menu on the desktop. A panel is captured at up to 15 frames per second, and at up to 30 when its dock holds an Image or Camera display.
- **Hiding, not closing.** By default (`Hide panels in AR instead of closing them` on the Settings tab, `QAR_PANEL_AR_VISIBILITY=hide`) the plugin never closes a QAROS panel while a session runs. **Stop streaming**, a closed dock or dialog and a removed AR tool hide their panel in AR and keep it, with its video sender, until the session ends; **Stream to AR**, the dock shown again or the tool exposed again shows it again and creates no new QAROS panel. The setting `Closed AR panels` and `QAR_PANEL_AR_VISIBILITY=close` select the older behavior, in which a closed window closes its AR panel and, by default, destroys its video sender.
- **Found again by name.** Every QAROS panel and the app volume get a stable ASCII name made from the key of the window, the program and the name of this PC. At the next start the plugin asks QAROS for the panel of the same name. If the Hub still has it, hidden, the plugin gives it the new video, fits its size and pose, and shows it, and the log writes `[qar-panel-adapter] ... panel '<key>' reattached name=<name> ...` instead of `created`. A panel that shows web content where video is wanted is not reused (the plugin repeats the request with the suffix `-v2`), and a name whose panel is closed is retried with the suffixes `-r1` to `-r8`. For an existing panel that is not closed, QAROS ignores the values passed and the plugin takes the panel over (see [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits#a-panel-or-volume-that-already-exists)).
- **Resizing.** See [A size change of a streamed window](#when-qaros-stops-answering): the default `new-panel` keeps a ring of hidden panels and never swaps the sender of a visible one.
- **Pauses.** The panel appears in AR when the session owner's thread has made it, which can take several seconds with the installed runtime (creating a panel that is found again took about 4 seconds in the authors' runs).

### Enlarged copies of Qt panels

The controls of a Qt panel, such as the arrows of a spin box and the check boxes of a tree, are small when the panel hangs in the room at its native size. An AR copy of a Qt panel is therefore enlarged by default, and the panel on the desktop does not change.

- **Copies with larger controls.** For the RViz2 panels Displays, Views, Tool Properties, Selection and Help, and for the Qar Streaming panel itself, the plugin makes a second instance, an AR copy, shows it off-screen only and streams it in place of the dock. The copy of an RViz2 panel binds to the same models as the dock, so every control of the copy acts on the same data; the copy of the Qar Streaming panel is a second view of its controls, and what is clicked, chosen, stepped or typed in AR is carried out on the control of the panel itself. The controls of a copy are drawn 2 times larger than the text, and the text is 1.25 times the desktop text with the hinting of the glyphs off, so it stays sharp. The whole copy then hangs in the room at 1.5 times (`QAR_PANEL_AR_COPY_OVERALL_SCALE`), so a control is about 3 times and the text about 1.9 times the native size.
- **Everything else.** The panels Time and Transformation and the panels of plugins have no copy, because a second instance would show a state of its own or run its timers twice; they are enlarged as a whole at the factor 2 (`QAR_PANEL_AR_COPY_SCALE`, `-ArCopyScale` of the launcher, 1.0 to 3.0). Video panels and the AR control panel keep their size. `QAR_PANEL_SELF_AR_COPY=off` streams the Qar Streaming panel itself, enlarged as a whole.
- **Native or enlarged.** The row **Qt panels in AR** of the Settings tab and the choice **Qt panels** of the AR control panel decide for every panel without a choice of its own; the size button of a row decides for that panel alone. The column **Size** shows what each copy has, for example `2x` or `native`.
- **A limit of 1.8 m.** An enlarged copy is at most 1.8 m tall; a tall copy such as **Displays (AR)** is enlarged less, never below its native size.

The person who ran the live checks between 2026-11-15 and 2026-11-19 with the installed runtime judged the copies: the controls were large enough to hit, "the scale looks excellent" at the overall factor 1.5, the spin buttons, the drop-downs and the colour picker worked from AR, and RViz2's selection box did not appear in the 3D picture. These are judgments of what the user saw and not measurements.

### Dialogs in AR

A dialog that RViz2 opens becomes a panel of its own in AR, placed beside the panel that opened it (0.5 m further along the x axis, times the larger factor of the two). Its scale depends on where it was opened. A dialog made below an AR copy, such as the colour picker of the Displays copy, is drawn with the style of that copy and is streamed at the overall factor of the copy. Any other dialog is enlarged as a whole at the factor of the setting, or at 1 for the size `native`. A limit remains: the dialogs that the desktop owns, **Add display**, which RViz2 parents to its main window, and **Add from the ROS graph**, which the dock of this panel opens, keep the proportions of the native controls in AR, because the style would change the dialog on the desktop as well. The authors accepted this limit.

When the connection is up, the plugin also makes a copy of the Displays panel, shows it off-screen only, so that it never appears on your desktop, and streams it into AR as **Displays (AR)** with its top edge 1.2 m to the left of the origin (times the factor) and 1 m in front of it. It views the same display tree as the real Displays dock, so a check box that you toggle in AR, a property that you edit and a display that you add, remove or rename in the headset change the displays of RViz2. The check box `Start the hidden Displays copy when connecting` and `QAR_PANEL_HIDDEN_DISPLAYS` set to `off` stop the automatic start on connect, and its row in the list of AR panels still starts the copy by hand. The copy is made once the first 3D stream is open.

### The AR control panel

The AR control panel is a web page with big buttons that hangs in front of each user in the headset, so that the user can switch panels and tools and pick in the scene without the desktop. From the top it offers: the choice **Off**, **Point** or **Nav goal** for picking, with **Confirm pick** and **Cancel pick**; up to four switches for the topic that a pick is published to; the state of the connection; a switch for each of up to 16 panels of the tab **AR panels**, and below them the choice **Qt panels** between **Enlarged** and **Native**; one switch for each of up to 8 AR tools; the number of peers; and the last error. A line that ends in `more: switch them on the desktop` says how many panels or tools have no switch. The switches follow the docks of RViz2 that are open, within about half a second. A switch hides or shows the AR panel of a window, or of a tool, and a pick command goes through the same code as the controls of the tab **Picking**. A command that the plugin refuses is answered on the page with the reason.

Each user gets a panel of their own, visible to that user only, one metre ahead of the head along the gaze, 0.35 m to the right and with its top edge 0.10 m below the eyes; it is placed once, when the first eye poses of the stream of that user are known, and never moved. By default (`Show the AR control panel when connecting` on the Settings tab, or `QAR_PANEL_AR_CONTROL`) the plugin serves the page when it connects. `QAR_PANEL_AR_CONTROL_PER_PEER=off` shows one panel for everybody. The group **AR control panel** on the Connection tab says whether the panel exists, **Show AR control panel** shows it again if it was closed in AR, and **Copy page URL** copies the address of the page, which opens in a desktop browser. The page is served over plain HTTP like the AR tools below.


### AR tools from the ROS 2 graph

An **AR tool** is a panel that the plugin generates from a topic, a service or an action of the live ROS 2 graph. It is a web page that a server in RViz2 serves and that QAROS opens as a URL panel. The plugin needs no generated message code: it reads the layout of a type from the introspection typesupport of its interface package at run time.

1. Press **Add from ROS graph...**. The dialog `Add from the ROS graph` lists the topics, services and actions of the graph as trees by namespace, with their types. **Filter:** narrows the list by a part of a name or of a type, and **Refresh** asks the graph again.
2. Select an entry. The lines under the tree say what **Add** will do before you press it. For a topic, **Add** creates the RViz2 display that accepts the type and pins the topic to the list of watched topics.
3. Press **Add**. It always generates the AR tool, so that no box has to be ticked, and for a topic it also creates the RViz2 display that accepts the type. **Add display only** does the second part alone and generates no tool; it exists for topics only, and it is grey for an image topic that no display handles. An image topic gets no RViz2 display at all.
4. Decide whether the tool may publish. The box **Allow publishing from AR** exists for topics only and starts in the state that the type asks for: a tool can publish by default only for `std_msgs/msg/Bool`, `geometry_msgs/msg/Twist` and `geometry_msgs/msg/TwistStamped`, and every other type starts read-only, because a publication to a topic is a command to a robot. The lines under the tree say which, and why, before you press **Add**. Later, **Allow publishing** and **Make read-only** on the tab **AR tools** change the selected tool (not for a service or an action). The buttons that command a robot, `Publish` and `Send goal`, ask for a second press within 4 seconds and read `Press again to confirm`. `Call service`, `Cancel goal` and the `STOP` button of a velocity command, which publishes a zero command, act on the first press.

The first tool hangs 1.2 m in front of the origin, and each further tool hangs 0.7 m further to the left. A row of the list of tools shows the kind, the name and the type, followed by `[in AR]` when its QAROS panel exists, `[hidden in AR]` when that panel is hidden, `[pending]` when it does not yet, and `[writable]` for a tool that may publish. The buttons **Remove tool**, **Re-open in AR**, **Copy URL** and **Allow publishing** / **Make read-only** act on the selected row, and the box `size x` multiplies the size of the AR panel at the next **Re-open in AR** (`QAR_PANEL_WEB_SCALE` sets its start value). A tool that you expose before you connect is served at once but stays `[pending]` until the connection is up.

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

The topics and types are those of the tools Publish Point, 2D Goal Pose and 2D Pose Estimate of RViz2, so subscribers of those tools work unchanged. A mode can publish to another topic: the row **Topic** of the tab **Picking** is an editable drop-down for the armed mode that lists the usual topic, the topics that the setting `Pick Topics` names for the mode, and every topic of the ROS 2 graph whose type the mode can publish (`PointStamped` or `Point` for a point; `PoseStamped`, `PoseWithCovarianceStamped` or `Pose` for a goal; `PoseWithCovarianceStamped` or `PoseStamped` for the initial pose; `PolygonStamped` or `Polygon` for an area). A name that you type is checked in the same way and refused with its reason in `Last error:`. The AR control panel offers the same choice with up to four switches. The plugin reads the topics of the graph when a mode is armed and then at most every 5 seconds while a mode is armed. A pick waits for your confirmation: the status says `waiting for confirmation`, **Confirm pick** publishes it and **Cancel** drops it. **Publish without confirming** removes that step. The default **Target**, `Action Point (QarOS scene intersect)`, publishes the intersection point that QAROS itself computes for the pointing ray; the SDK does not document which geometry QAROS intersects. A click that hits nothing is refused and is never published as the origin of the map. The plugin stamps every pick with time zero, which tf resolves as the latest available transform, so a subscriber that needs the time of a pick has to stamp the message itself.


### What has been seen, and what has not

The authors of `qar_rviz_plugin` ran live sessions of the plugin in RViz2 against a QAROS Visualizer in three generations. The judgments below are what the user saw and the logs and crash reports showed, not measurements.

- **With the installed runtime 0.9.0-dev.3008+65dc9aeb (2026-10-24 and 2026-10-25),** the authors checked that the offline preview shows the group **QAROS Hub**; that the first connect shows the approval window and, after **Allow**, a connected state and a stream in the Visualizer once a receiver exists; that a restart joins without a prompt and finds the panels again; and that the tabs, the AR control panel for each user, switching a streamed window off and on, resizing a dock, opening and closing a dialog, picking with `/clicked_point` and the status strip work as before.
- **Robustness checks with the same runtime (2026-11-09 to 2026-11-12).** In three checks of a Visualizer crash (the first of five planned rounds), RViz2 did not freeze; in two of them the Visualizer was back after 26 and 5 seconds. After the Hub was quit or crashed, RViz2 stayed operable and the AR panels were back within seconds, but the 3D floor came back only after minutes or not at all in that build ([Known issues](#known-issues)). Deny and the approval timeout showed their texts and left RViz2 operable; **Allow** after the timeout streamed, while **Allow** after **Deny** streamed nothing because the Visualizer kept requesting the stream of the RViz2 that had crashed before (a QAROS behavior). Five closes of RViz2 with QAROS gone or hung took about 4 to 5 seconds. Resizing docks with the default `new-panel` mode caused no freeze in four rounds; the forced `swap` mode froze RViz2 for about 20 seconds in the first build and for 3.7 seconds in a later one (a fault of the GPU driver). The File menu of RViz2 kept its highlight while panels streamed. The exit with a suspended Hub, most of the planned rounds of the Hub restart, and the crash checks that the unit left for the end of November were not run.
- **UX checks with the same runtime (2026-11-15 to 2026-11-19).** The enlarged copies, the list of AR panels, the panel switches, the choice of the pick topic, the shorter Connection tab, the AR tools without a check box and the AR state in the RViz2 configuration worked in the user's rounds after several fixes. After a restart of QAROS on 2026-11-18 the AR panels and the 3D floor came back by themselves in about 30 seconds, and after a crash of QAROS on 2026-11-19 everything came back, the floor included. The Visualizer crashed in the checks only in the kinds named in [Known issues](#known-issues).
- **With the SDK drop of 2026-09-28 (an earlier generation, with a session start-up that the installed runtime replaced),** the logs of two sessions show a 3D stream at a median of 60 frames per second in every launch that streamed, with about 52 frames per second for each of two streams; the pause and the resume; 15 AR tools exposed from the ROS 2 graph, with 12 of the 17 layouts and the generic form; one velocity command published from AR; hover and click events with the ray of the runtime and 21 points published on `/clicked_point` in one launch; and a video at a median of 28 frames per second by both routes, from a publisher that sent 30. In these sessions nobody judged how the pictures looked in the Visualizer, whether the pages of the other AR tools were readable, or how near the picks landed to the points that were aimed at. No accuracy of a pick has been measured; the authors accept a pick that lands about 0.1 m to 0.2 m from the aimed point at a few meters.
- **Not run in a live session:** the plugin built on the pinned header and the code of the move onto the local-only owner (the join, auto-connect, the idle tier of the render tick, the settings version 5) against any runtime; the geographic anchor against a live GPS source; real multi-user devices; the use of two hands on an AR panel; the result of a service call and of an action goal from AR, because the plugin logs neither; the layouts `diagnostics`, `imu`, `robot-service` and `navigation-goal`; a Hub that is suspended; and everything on Linux, where the plugin is not verified.
- **Devices.** No device was available for the checks of the current code. The authors state that an earlier build streamed to a HoloLens 2 with the drop of 2026-09-28; no record of that run exists.

## Settings and the saved state

The tab **Settings** holds the settings of the streams and of the session, and the titles of its groups say when a setting takes effect. **Resolution:** and **FPS cap:** apply to the streams that are created from then on. **Qt panels in AR:** applies at once to every AR copy of a Qt panel that has no size choice of its own. The group **Session (apply when you connect)** has the check boxes `Start the hidden Displays copy when connecting` and `Show the AR control panel when connecting`. The check box **Show advanced options** shows the group **Advanced** (`Prime next frame`, `Closed AR panels:`, `Hide panels in AR instead of closing them`) and, in a live session, the group **Simulated peers** of the Diagnostics tab; its state is not saved.

RViz2 keeps 16 settings of the panel in its configuration under the **settings version 5**, written by `File > Save Config` into the entry of the panel. For each setting the value in force is the first of three: the environment variable of the setting, when it is set and acceptable; the value saved in the RViz2 configuration, when it is acceptable; the default. A control whose variable wins shows that value and is grey, and its tool tip names the variable. The saved configuration never takes a value that only the variable supplied, so removing the variable brings back the choice of the operator. A variable that is not acceptable is logged once with the prefix `SETTINGS:` and ignored.

| Setting | Variable | Control | Default |
|---|---|---|---|
| `Resolution` | none | **Resolution:** | `1280x720` |
| `FPS Cap` | none | **FPS cap:** | `60` |
| `Hub Mode` | `QAR_PANEL_HUB_MODE` | none | `auto` (`auto` and `local` both join the Hub of this PC) |
| `AR Copy Size` | `QAR_PANEL_AR_COPY_SIZE` | **Qt panels in AR:** and the choice **Qt panels** of the AR control panel | `enlarged` |
| `AR Copy Scale` | `QAR_PANEL_AR_COPY_SCALE` | none; `-ArCopyScale` of the launcher sets the variable | `2.00` |
| `AR Copy Overall Scale` | `QAR_PANEL_AR_COPY_OVERALL_SCALE` | none | `1.50` |
| `Hidden Displays On Connect` | `QAR_PANEL_HIDDEN_DISPLAYS` | `Start the hidden Displays copy when connecting` | on |
| `AR Control Panel` | `QAR_PANEL_AR_CONTROL` | `Show the AR control panel when connecting` | on |
| `AR Panel Close Mode` | `QAR_PANEL_CLOSE_MODE` | **Closed AR panels:** | `destroy` |
| `AR Visibility` | `QAR_PANEL_AR_VISIBILITY` | `Hide panels in AR instead of closing them` | `hide` |
| `Web Panel Scale` | `QAR_PANEL_WEB_SCALE` | the spin box `size x` of the tab **AR tools** | `1.00` |
| `Pick Target` | none | **Target** | `action_point` |
| `Pick Frame` | none | **Frame** | `map` |
| `Pick Topics` | `QAR_PANEL_PICK_TOPICS` | the row **Topic** of the tab **Picking** | empty |
| `Invite Device` | none | the drop-down of the group **Invite a device** | `hololens` |
| `Invite Hostname` | none | the field for the host name | empty |

The pick mode, **Publish without confirming**, a pick that waits, `Prime next frame`, `Show advanced options`, the simulated peers, the selected tab and the pages of the lists are never saved, so a gesture cannot move a robot because a switch was left on. A configuration of version 3 or older loses its `Hub Host` and `Hub Port` with a sentence in the log, and a `Hub Mode` of `remote` is read as `local`. A file saved by an earlier plugin at version 4 (with or without the AR copy keys) loads with every value kept, and the next save writes version 5. The variables of earlier plugins that have no effect any more are ignored and logged as `ignored: <name> has no effect with this QAROS`; a saved peer admission is dropped, and a saved invitation kind `quest` is read as `headset`, and `visualizer` and `zed` as `hololens`.

**The AR state.** Besides the settings, `File > Save Config` writes the record `AR State` into the entry of the panel: the AR panels of the windows of RViz2 that are on or have a size choice of their own, the AR tools (kind, name, type, whether the tool may publish and whether it is shown) and the pinned topics. After a load, the pinned topics and the tools come back as soon as RViz2 has its display context, before any connect, one tool for each render tick; the panels come back after **Connect to QarOS**, once a receiver streams, one for each render tick, found again by their key. A tool comes back with the publish flag it was saved with. A record or a field that cannot be read is skipped with one sentence, logged with the prefix `SETTINGS:`.


## Reading the logs

### Where the logs are

- **The log of RViz2** is the file `rviz_live_<stamp>.err.log`, with the file `rviz_live_<stamp>.out.log` next to it, in the folder `qar_rviz_logs` in your `TEMP` folder, or in the folder of `-LogDir`. The launcher prints the paths. The panel writes every line that it logs to its **Diagnostics** view, which keeps the last 2000 lines and does not survive RViz2, and to the standard error of RViz2 after the prefix `[qar-panel]`, so the file is the record. The lines of the session owner start with `QAROS:`.
- **The QAROS library inside RViz2** writes its log into the folder `quaternar` in your `TEMP` folder, or into the folder that `QAR_PANEL_LOG_DIR` names, and the launcher saves a copy of its file `qar_streaming.log` next to the RViz2 logs when RViz2 exits. The processes that the Hub runs write their own logs and crash reports where [Using the Hub](/docs/operator-guide/running-the-hub#logs) says.
- **The logging mode** is quiet in the launcher: the library logs at `info`, there is no transport logging and no per-frame trace. Without the launcher the plugin logs at `trace`. `-TraceLogs` selects the full logs that a bug report needs, and `QAR_PANEL_LOG_SEVERITY` and `QAR_PANEL_LOG_CONSOLE` set the severity and the echo to the console.

### Lines to look at

The heartbeat is the line that tells you most about a run, so read it first when something stalls. These commands print the lines that the table names from the newest log:

```powershell
$log = Get-ChildItem -Path "$env:TEMP\qar_rviz_logs" -Filter 'rviz_live_*.err.log' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Select-String -Path $log.FullName -Pattern 'QAROS: (runtime|joined as peer|target)|Connected to QarOS|LIVE: opened stream|AR PANEL: streaming|AR TOOL: exposed|AR TOOL: panel server' | ForEach-Object { $_.Line }
Select-String -Path $log.FullName -Pattern 'HB dt=' | Select-Object -Last 3 | ForEach-Object { $_.Line }
```

| The line contains | It tells you |
|---|---|
| `QAROS: runtime <version> <flavor> <api> from <path>` | Which runtime the plugin loaded and from where. A path other than the installed QAROS means a stray `qar-streaming-c.dll` or `QAROS_RUNTIME_DIR` won the lookup. |
| `QAROS: state <state>` and `QAROS: progress <text>` | The steps of the join, including `Waiting for approval in QAROS hub`. |
| `QAROS: joined as peer <id> (local) in <n> s` | The session is open. |
| `QAROS: target <id> <kind> '<name>' <presence> -> served` | A target of the Hub and whether it gets a stream. A target that went away reads `-> stream stopped`. |
| `Connected to QarOS: session open, streaming active` | The panel finished setting up the session. A failed join writes the message of the state line instead, and `Last error:` shows the same text. |
| `LIVE: opened stream for peer` | The stream of a target is open. If it never appears for a target that is listed as served, the target has not asked for the stream yet. |
| `[qar-panel-adapter] ... panel '<key>' <created\|reattached> ...` | A QAROS panel was made or found again, with the state it had and whether its stream or size changed. |
| `AR PANEL: streaming` | A window of RViz2 is streamed as a GUI panel. |
| `AR TOOL: exposed` | An AR tool was generated. `AR TOOL: panel server listening on` gives the address of the page server. |
| `Render tick: idle, every 50 ms (nothing to stream)` and `Render tick: streaming, 0.1 ms` | The render tick switched its tier ([How the plugin keeps RViz2 responsive](#how-the-plugin-keeps-rviz2-responsive)). |
| `the QAROS Hub process ended (pid` and `the QAROS Hub restarted (pid` | The Hub went away and came back; the 3D sender is released now and made again when the target is online. |
| `[qar-stall]` | The Qt thread of RViz2 did not tick for 3 seconds; the line names the step. |
| `exit: owner finished in` and `exit: owner thread left running after` | The end of the session owner when RViz2 closes, in time or left behind. |
| `HB dt=` | The heartbeat, written every 2 seconds. `streams=` is the number of open 3D streams, and each stream has a field with the first eight digits of its peer identifier and its achieved frame rate `fps=`. A `tick_fps=` far below the frame rate of the streams, or a `max_gap=` of hundreds of milliseconds, means that the GUI thread of RViz2 was blocked; in the idle tier (`tier=idle` at the end of the tick field) about 20 ticks a second are normal. `rx=0` after `AR gestures:` on every heartbeat means that the runtime sends no gestures. |

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
| The strip says `Hub not reachable` or `the QAROS Hub is gone`. | QAROS stopped answering. RViz2 keeps working and joins again by itself ([When QAROS stops answering](#when-qaros-stops-answering)); **Join again** and **Stop rejoining** act at once. |
| After a Hub restart the AR panels are back but the 3D scene (the floor) is not. | A known QAROS issue: the runtime sends no render request for the replayed stream. The plugin makes a new 3D stream by itself; wait. If the strip says the 3D stream is being set up again and it stays, close the Visualizer and start it again ([Known issues](#known-issues)). |
| RViz2 takes a few seconds to close when QAROS is gone. | The exit guard ends RViz2 5 seconds after the application was asked to quit ([When QAROS stops answering](#when-qaros-stops-answering)). |
| An AR tool opens in the headset, but its page does not load. | The headset has to reach the host and the port in the address. Read the log line `AR TOOL: panel server listening on`. The port changes at every start unless you pin it with `QAR_PANEL_WEB_PORT` and open it in the firewall, and on a machine with several networks the host can be the wrong one: set `QAR_PANEL_WEB_HOST` to an address of the machine in the network of the headset. |
| A click in the scene publishes nothing on `/clicked_point`, `/goal_pose` or `/initialpose`. | **Mode** starts at `Off`. A pick waits for **Confirm pick** unless **Publish without confirming** is ticked. A click that hit nothing is refused and counted as `clicks-on-nothing=` in the status, and a hover only previews. |

## Environment variables

Variables that you set in the PowerShell window before you start the launcher or RViz2. A saved setting of the panel is overridden by its variable, and the control is then disabled. The launcher exports the variables that its parameters name; a value from your shell wins over a default of the launcher.

| Variable | Effect |
|---|---|
| `QAR_PANEL_LIVE_STREAM` | Any value that is not empty starts the panel in live mode. The launcher sets it. |
| `QAR_PANEL_AUTO_CONNECT` | `1`, `true`, `on` or `yes`: the live panel presses **Connect to QarOS** once, about half a second after it has loaded. Not a saved setting; no effect in the offline preview. The launcher sets it unless your shell has a value. |
| `QAR_PANEL_HUB_MODE` | `auto` or `local`, kept for older configurations: every value joins the Hub of this PC. `remote` is refused with a warning. |
| `QAR_PANEL_APPROVAL_TIMEOUT_S` | Seconds that the Hub waits for the **Allow** of a local join, 0 to 600. 0 and an unset variable give the Hub's default of 120 seconds; a larger value is lowered to 600. |
| `QAR_PANEL_INSTANCE` | A name that is added to the names of the plugin's QAROS panels and of the app volume, for a second RViz2 on the same PC. |
| `QAROS_RUNTIME_DIR` | A folder with another QAROS runtime for the library lookup (read by the plugin's own loader, not by the current C header; `-RuntimeDir` of the launcher sets it). |
| `QAR_PANEL_INVITE_PEERS` | `hololens:<host>,hololens:<host>`: HoloLens devices to invite once when the session first connects. |
| `QAR_PANEL_MAX_PEERS` | The ceiling of served targets, 8 by default. |
| `QAR_PANEL_HIDDEN_DISPLAYS`, `QAR_PANEL_AR_CONTROL` | `off` stops the automatic start of the hidden Displays copy and of the AR control panel at connect. |
| `QAR_PANEL_AR_CONTROL_PER_PEER` | `off` shows one AR control panel for everybody instead of one in front of each target. |
| `QAR_PANEL_AR_VISIBILITY` | `hide` (the default) or `close`: hide and keep AR panels, or close them. |
| `QAR_PANEL_CLOSE_MODE` | `destroy` (the default) or `notice`: what happens to the AR copy of a window that closes, while hiding is off. `notice` keeps the AR panel as a notice and keeps its video sender until the session ends. |
| `QAR_PANEL_RESIZE` | `new-panel` (the plugin's default), `swap` or `defer`: how a streamed window follows a size change. |
| `QAR_PANEL_AR_COPY_SIZE` | `enlarged` (the default) or `native`: the size of the AR copies of Qt panels. |
| `QAR_PANEL_AR_COPY_SCALE` | 1.00 to 3.00, 2.00 by default: the factor of the controls of an enlarged copy, and of a panel that is enlarged as a whole. |
| `QAR_PANEL_AR_COPY_OVERALL_SCALE` | 1.00 to 2.00, 1.50 by default: the factor of the whole copy that has larger controls. |
| `QAR_PANEL_AR_COPY_TEXT_SCALE`, `QAR_PANEL_AR_COPY_FONT_HINTING` | The text factor of such a copy, 1.0 to 2.0, 1.25 by default, and `default` for the hinting of the platform; neither is saved. |
| `QAR_PANEL_SELF_AR_COPY` | `off` streams the Qar Streaming panel itself, enlarged as a whole, instead of its AR copy. |
| `QAR_PANEL_PICK_TOPICS` | `point=/a,/b;nav_goal=/c;initial_pose=/d;area=/e`: topics that each pick mode may publish to besides its usual one. |
| `QAR_PANEL_FIXED_POSES` | `room` (the default) or `ground`: hang the panels with a fixed place from a fixed height of the room or from the RViz2 ground. The launcher sets `ground`. |
| `QAR_PANEL_WEB_HOST`, `QAR_PANEL_WEB_PORT` | The host name in the addresses of the AR tools, and the port of their server. |
| `QAR_PANEL_WEB_SCALE`, `QAR_PANEL_WEB_SIZE` | A multiplier on the size of a URL panel (0.25 to 4), or a size in meters as `<w>x<h>`. |
| `QAR_PANEL_APP_VOLUME`, `QAR_PANEL_VOLUME_NAME`, `QAR_PANEL_VOLUME_SIZE`, `QAR_PANEL_VOLUME_SCALE`, `QAR_PANEL_VOLUME_Y` | The app volume of the scene: `off`, its display name, the edge of its cube in meters, its scale and the height of its center in meters. |
| `QAR_PANEL_NEAR_M`, `QAR_PANEL_FAR_M` | The clip distances of the scene, 0.01 m and 150 m by default. |
| `QAR_POSE_WORLD`, `QAR_DEPTH_ENCODING` | `zup` and `ndc`: the world axes and the depth encoding that the launcher sets. |
| `QAR_PANEL_TICK_MS` | The period of the render tick while a stream exists, in milliseconds, 0.1 by default. The idle tick is 50 ms, or this value when it is larger. |
| `QAR_FRAME_CALLS` | `worker` (the default) or `gui`: where the QAROS frame calls of a sender run. |
| `QAR_FRAME_FENCE` | `worker` (the default) or `gui`: where the wait for the GPU copy of a 3D frame runs. |
| `QAR_GPU_GATE` | `on` (the default) or `off`: the GPU progress gate. |
| `QAR_GPU_GATE_TEST` | Seconds, above 0 and below 3600: closes the gate once on purpose, 30 seconds after the first 3D batch, so the pause and the resume can be seen. Not set by default. |
| `QAR_PANEL_REJOIN_AFTER_HUB_S` | Seconds, 45 by default: how long after the Hub answers again the panel waits for a 3D stream to submit a frame before it gives the session up. `0` never gives it up. |
| `QAR_RENDER_RECYCLE_S` | Seconds, 45 by default, 0 to 3600: how long the runtime may ask a 3D sender for no frame before the sender is made again. `0` switches the rule off. |
| `QAR_ADAPTER_STREAM_RELEASE` | `owner` (the default) or `gui`: which thread destroys the stream handles of the 3D senders. |
| `QAR_EXIT_GUARD` | `on` (the default) or `off`: the guard that ends RViz2 5 seconds after the application was asked to quit. |
| `QAR_STALL_DUMP` | `off` (the default) or `on`: a minidump 5 seconds after the Qt thread stopped ticking. |
| `QAR_PANEL_LOG_DIR`, `QAR_PANEL_LOG_SEVERITY`, `QAR_PANEL_LOG_CONSOLE` | The folder, the severity and the console echo of the QAROS library inside RViz2. |
| `QAR_GUI_CALL_WARN_MS` | The limit, in milliseconds, above which a QAROS call on the Qt thread is reported as slow; 20 by default. |

`QAR_PANEL_HUB_HOST` and `QAR_PANEL_HUB_PORT` have no effect: the plugin joins the Hub of this PC only. The variables of the earlier session start-up (the invitation of a Visualizer, the admission of peers) have no effect with the installed runtime; when one is set, the plugin writes `ignored: <name> has no effect with this QAROS` to its log.

## Known issues

### The Visualizer closes a stream panel 300 seconds after its sender is gone, and crashes

With the runtime 0.9.0-dev.3008+65dc9aeb the Visualizer closes a stream panel whose video sender has been gone for 300 seconds, and that close crashed it in `WebCore.dll` (the crash reports name the offsets `+0x1ACFD20` and `+0x2DA26E0`) in each of the six self-closes that the authors traced in their logs of 2026-11-11 and 2026-11-16; where the time of the sender's end was known, the close came 300 seconds after it. A panel that a session leaves hidden in the Hub loses its video sender at the end of the session, so it becomes such a panel unless the next session of RViz2 attaches the same name again within 300 seconds. With the SDK drop of 2026-09-28 the Visualizer disappeared within a second of the creation of a hidden panel that the plugin had closed before, in each of 25 observed cases; all other running Visualizers disappeared with it, QAROS started a Visualizer again after several seconds, and until then the 3D streams stood still. Every crash report named the same access violation inside the browser engine library.

The plugin with the installed runtime hides its panels and finds them again by name instead of closing and creating them, which removes the closes and creations that the plugin itself causes. It cannot prevent the Visualizer's own close of an orphan. What you can do: leave `Hide panels in AR instead of closing them` on, close and start panels at a calm pace, and, if you need panels closed, set `QAR_PANEL_CLOSE_MODE=notice`. If the Visualizer disappears, send the crash reports (the files whose names end in `crash.txt`, in the QAROS log folders) with the logs of the run to the authors of `qar_rviz_plugin`.

### After a Hub restart the 3D view can take minutes

After a restart of the Hub the runtime replays the 3D transfer but sends no render request for it (the statuses `RenderingProducerMissingRenderRequest`), and the Visualizer may keep requesting the stream id of the earlier RViz2. The C API has no call to steer this. The plugin releases the 3D sender when the Hub process ends and makes it again when the new Hub runs; with the installed runtime the floor then came back in about 30 seconds in the authors' check of 2026-11-18 and came back after a QAROS crash on 2026-11-19, while before that change it took minutes or did not return. The note in the strip names it as a known QAROS issue. Whether a newer runtime removes it was not checked.

### A GPU fault can still hold RViz2

A fault of the GPU driver that starts while RViz2 is inside its own frame can hold RViz2 for the length of the fault: 3.7 seconds were observed on 2026-11-11 with a forced swap of a visible panel's sender. GL contexts of one process do not overtake each other on the NVIDIA driver in use, so the plugin cannot prevent this in-process. The default `new-panel` mode avoided it in the authors' rounds.

### Lines at the end of a run

When the last Visualizer has closed, or while RViz2 closes, the log can hold the lines `submit_phase: sender rejected the frame` and `wglMakeCurrent FAILED`. Neither line is a failure by itself. Judge a run by its heartbeat lines.


## Related pages

- [Ogre, Qt and RViz2 on QAROS](/docs/developer-guide/ogre-qt-rviz) — the three libraries, how they depend on each other and what to install.
- [Streaming an Ogre Scene to QAROS](/docs/developer-guide/ogre-streaming) — how `qar_ogre_adapter` streams an Ogre scene, which the plugin does for the scene of RViz2.
- [Video in GUI Panels, and Qt Widgets as AR Panels](/docs/developer-guide/qt-ar-panels) — how a GUI panel shows a video, and what the Qt library does with a widget.
- [Limits and Behaviors to Design Around](/docs/developer-guide/sdk-limits) — what the header leaves open about streams, sizes and the position of the operator, and what a QAROS query costs.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions) — the local app and the approval that the plugin uses, and the member Hub that brings a program on another PC in.
- [Source Applications](/docs/operator-guide/source-applications) — the approved local apps, and the applications that stream content into the room.
- [Connection Panel](/docs/user-guide/connection-panel) — the Visualizer's Hubs and Devices areas, from which devices are invited.
- [Running the Hub](/docs/operator-guide/running-the-hub) — the Hub that the plugin joins.
