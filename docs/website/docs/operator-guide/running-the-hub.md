---
sidebar_position: 4
title: Running the Hub
description: Running the QAROS Hub - tray service, runtime launcher, the visualizer, reconnects, logs and crash dumps.
---

# Using the Hub

The Hub is not a single program but three cooperating processes that ship together:

| Process | Role |
|---|---|
| `qaros-hub-service.exe` | Tray companion. Always-on supervisor: starts the launcher, restarts it if it crashes, and gives you the tray menu. |
| `qar-runtime-launcher.exe` | The actual Hub runtime: hosts the session and message router, runs the private CA and onboarding services, broadcasts discovery, and orchestrates the streaming pipeline (source apps, mixers, targets). |
| `qar-streaming-viz.exe` | The visualizer - the interactive desktop window where you see and manipulate the shared room. |

## Starting the Hub

1. Start **`qaros-hub-service.exe`** (`bin/` of the runtime ZIP, or **QAROS Hub** in the Start menu after the installer), or let it start with Windows if the startup task is enabled. Do not start the launcher or the visualizer directly. It launches the runtime launcher automatically and keeps it running - crashes are restarted with backoff.
2. **Left-click the tray icon** to open the visualizer window.
3. The tray menu offers: *Open visualizer*, *Restart launcher*, *Connect to other Hub*, and *Exit*.

On startup the launcher restores its previous session (or creates a fresh one), registers the configured source app volumes, starts the streaming services, begins broadcasting the discovery beacon (default display name "QAROS Hub"), and starts the onboarding services. By default it also invites a local visualizer into the session automatically.

Hub configuration is managed through the Hub UI and persists across restarts. Startup settings (paths, ports, onboarding, source and target apps) come from the launcher configuration file - see the [Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference).

## Running several Hubs on one PC

For development and testing, one PC can run several independent Hubs from the same release, demo or development build. Give each its own data root:

```powershell
$env:QAR_APP_DATA_ROOT = "D:\qaros\hub-b"; .\bin\qaros-hub-service.exe
```

- A Hub is identified by its data root. Starting `qaros-hub-service.exe` again with the same root replaces the running one; a different root starts a second Hub next to it, with its own tray icon, launcher and visualizer.
- Each Hub has its own identity, CA, session, Hub configuration and source-app presets. Devices onboarded to one are not onboarded to the other.
- Ports do not collide: each Hub takes the next free router port (19120, 19121, ...) and its own onboarding (pairing) port from the same range, bound exclusively, so a second Hub never shares the first one's. Pin them with `routerPort` / `routerPortRangeStart` / `discoveryPort` only for firewalled setups ([Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference#networking)).
- To pair with a specific Hub by address, use `host:port`: the port is shown with that Hub's onboarding code. An app given only a code finds the Hub of the same Windows user on its own.
- With `QAR_APP_DATA_ROOT` set, logs and crash dumps stay under that root, so runs never mix.
- A Hub started without `QAR_APP_DATA_ROOT` uses the default root of its install ([Installation: Where data and logs live](/docs/operator-guide/installation#where-data-and-logs-live)).

## The visualizer

The visualizer is the Hub's window into the shared room: it renders the same mixed scene a headset user would see, and hosts the desktop-side UI panels.

- **Source Applications** starts, configures and stops the apps streaming into the room: see [Source Applications](/docs/operator-guide/source-applications).
- **UI mode** (Settings): **Basic**, **Developer** or **Debug**. Debug needs the debug password and shows the debug-only apps and tools.
- **Logs** shows the log lines of the processes in the session (see [Logs](#logs)).

A screen-by-screen tour with screenshots is not yet captured.

## When a process drops out or restarts

QAROS recovers on its own; nothing needs restarting by hand.

| Event | What happens | Typical time |
|---|---|---|
| A process dies or its network drops | The session drops it once it has been silent for about 2 s. | ~2 s |
| A stream stops delivering frames | The receiver probes the sender and reconnects the stream once the sender is gone. | ~5 s from kill to reconnected stream |
| A source app is killed and relaunched | The relaunched app takes over its streams from the dead instance. A second copy is refused only while the first is provably still live. | ~2-3 s |
| **Stop** on a running app | The Hub asks the app to stop and kills it after about 1.3 s. | ~1.3 s |
| A QAROS API app exits within 10 s of starting | The Hub restarts it with a fresh one-time launch invite (the onboarding screen's code is not affected), backing off from 2 s up to 60 s, and reports the first failure. | 2 s, doubling |
| The Hub's router is unreachable | Processes keep retrying with backoff up to 10 s and reconnect once it is back, without a restart. | up to 10 s after it returns |

### When an app restarts

Restarting a source app (crash, kill, or **Stop** then **Launch**) is safe: the new instance keeps the same streams and app volume, and viewers see it again within a few seconds.

## Logs

- Every process writes its log files into the Hub's log folder (default `<log root>/qar-launcher-default/<session-id>`, `%TEMP%\quaternar\logs\...` for an installed Hub; see [Installation: Where data and logs live](/docs/operator-guide/installation#where-data-and-logs-live)). Set it with `logFolder` ([Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference#paths)). The default folder is emptied when the Hub starts: copy it **before** restarting if you need it.
- The tray writes its own log and the launcher's console output to `<log root>/qaros-hub-service`.
- Processes started by the Hub also write `<name>_stdout.log` and `<name>_stderr.log` there.
- Log files roll over so a Hub that runs for months never fills the disk. The current file keeps its name. Once it is a day old or 256 MB, it is renamed to `<name>.<UTC time>.log`, for example `launcher_stdout.20261009T120000Z.log`, and a new one starts. The oldest of these are deleted so each log file and its rolled-over copies stay under 3 GB together.
- The visualizer's **Logs** panel streams log lines live. Delivery is best-effort: a gap shows as a warning line `[viz] N log messages lost`, and a restarted process as `[viz] peer log stream restarted`. The log files stay complete. Log-level changes from the panel are always delivered.

## Crash dumps

A crashing process writes a pair of files: `<time>_<process>_<pid>.crash.txt` (a short report) and `<time>_<process>_<pid>.dmp` (the dump).

| Process | Where |
|---|---|
| Launcher (`qar-runtime-launcher`) started by the tray | `<log root>/qaros-hub-service/crashes` |
| Processes started by the Hub | Next to that process's logs, in the Hub's log folder |
| Your application on the C API | `crashes/` under the `log_folder_path` it passes to `qar_library_init` |
| Anything else | `%TEMP%\quaternar\crashes` |

When reporting a crash, send both files together with the log files next to them. The crash report below collects all of them for you.

## When something goes wrong

The visualizer opens only once it works: it has joined the Hub's session. If the Hub cannot get there, the tray shows a **QAROS Hub** window instead of the splash just closing. It also shows this window when a part of the Hub keeps crashing.

| You see | What happened |
|---|---|
| QAROS Hub could not start. | The visualizer could not join the Hub's session, or did not within 60 s. The details say why. |
| QAROS Hub did not finish starting. | Nothing was ready after 120 s. The details name the last step. |
| \<part\> keeps crashing. | One process (the visualizer, a mixer, a source app…) was restarted 10 times within 10 minutes. |
| QAROS Hub keeps stopping. | The launcher itself stopped 10 times within 10 minutes. |
| A tray notification "A QAROS process crashed" | A process wrote a crash dump. Click the notification for the window. |

In the window:

- **Copy details** copies the text, ready to paste into an email or a ticket. You can also select part of it.
- **Create crash report** saves `QAROS-crash-report-<date>-<time>.zip` in your Downloads folder and selects it in Explorer. It holds the Hub's logs and crash dumps from the last 7 days, plus `report.txt`, which says what happened and lists every file. It holds no keys or certificates. Send it to Quaternar support.
- **Clean data and start fresh** deletes the Hub's data on this PC and starts it again: paired devices, certificates, sessions, receivers and settings. Logs and crash dumps are kept. Every device has to be paired again. Use it when the Hub stays stuck, after creating a crash report.

You can create a crash report at any time from the tray menu: **Create crash report…**.

## Connecting two Hubs

Connecting this Hub to another so devices trust both is covered in [Networking & Federation](/docs/operator-guide/networking-and-federation).
