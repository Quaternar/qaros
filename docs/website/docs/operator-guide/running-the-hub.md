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

1. Start **`qaros-hub-service`** (or let it start with Windows, if the startup task is enabled). It launches the runtime launcher automatically and keeps it running - crashes are restarted with backoff.
2. **Left-click the tray icon** to open the visualizer window.
3. The tray menu offers: *Open visualizer*, *Restart launcher*, *Connect to other Hub*, and *Exit*.

On startup the launcher restores its previous session (or creates a fresh one), registers the configured source app volumes, starts the streaming services, begins broadcasting the discovery beacon (default display name "QAROS Hub"), and starts the onboarding services. By default it also invites a local visualizer into the session automatically.

Hub configuration is managed through the Hub UI and persists across restarts. Startup settings (paths, ports, onboarding, source and target apps) come from the launcher configuration file - see the [Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference).

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
| A QAROS API app exits within 10 s of starting | The Hub restarts it with a fresh one-time launch code (the onboarding screen's code is not affected), backing off from 2 s up to 60 s, and reports the first failure. | 2 s, doubling |
| The Hub's router is unreachable | Processes keep retrying with backoff up to 10 s and reconnect once it is back, without a restart. | up to 10 s after it returns |

### When an app restarts

Restarting a source app (crash, kill, or **Stop** then **Launch**) is safe: the new instance keeps the same streams and app volume, and viewers see it again within a few seconds.

## Logs

- Every process writes its log files into the Hub's log folder (default `<data root>/qar-launcher-default/<session-id>`, set with `logFolder`, see the [Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference#paths)). The default folder is emptied when the Hub starts: copy it **before** restarting if you need it.
- Processes started by the Hub also write `<name>_stdout.log` and `<name>_stderr.log` there.
- The visualizer's **Logs** panel streams log lines live. Delivery is best-effort: a gap shows as a warning line `[viz] N log messages lost`, and a restarted process as `[viz] peer log stream restarted`. The log files stay complete. Log-level changes from the panel are always delivered.

## Crash dumps

A crashing process writes a pair of files: `<time>_<process>_<pid>.crash.txt` (a short report) and `<time>_<process>_<pid>.dmp` (the dump).

| Process | Where |
|---|---|
| Processes started by the Hub | Next to that process's logs, in the Hub's log folder |
| Your application on the C API | `crashes/` under the `log_folder_path` it passes to `qar_library_init` |
| Anything else | `%TEMP%\quaternar\crashes` |

When reporting a crash, send both files together with the log files next to them.

## Connecting two Hubs

Connecting this Hub to another so devices trust both is covered in [Networking & Federation](/docs/operator-guide/networking-and-federation).
