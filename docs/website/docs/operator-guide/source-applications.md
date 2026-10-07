---
sidebar_position: 4.5
title: Source Applications
description: Start, configure and stop the applications that stream content into the room, from the visualizer's Source Applications panel.
---

# Source Applications

A **source application** renders content into an app volume and streams it to the target applications (headsets, visualizers). You start and manage them from the visualizer's **Source Applications** panel.

## The panel

| Section | Shows |
|---|---|
| **Apps** | A tile grid: built-in apps, then detected apps, then your presets, then **+ Add source app**. The refresh icon looks for installed apps again. |
| **Running** | One row per running app, with its status, **Streams to**, **Stop** and, where supported, **Live settings**. |

Tile tags:

| Tag | Meaning |
|---|---|
| **Built-in** | Ships with QAROS: StereoKit, CPU Renderer, Test OpenXR. |
| **Detected** | Found on this PC (see [Detected apps](#detected-apps)). |
| *(none)* | A preset you saved. |
| **Debug** | Visible only in the Debug UI mode. |
| **N on** | N copies of this app are running. |

Built-in and detected apps cannot be edited or removed. To change one, open it and **Save as new preset**.

## Launching an app

1. Click a tile. A dialog opens (**Built-in app**, **Detected app** or **Edit preset**).
2. Adjust the settings and choose **Streams to**.
3. Press **Launch**.

Each **Launch** starts a new, independent copy with its own copy of the settings. Launch failures appear as error notifications (`<app name>: <error>`).

Preset dialogs also offer **Save**, **Save as new preset** and **Remove**. Removing a preset does not stop the apps started from it, and stopping an app does not remove its preset.

## Adding your own app

1. Click **+ Add source app**.
2. In the Debug UI mode, pick the type first: **OpenXR app** or **QAROS API app**. In the other modes only OpenXR apps can be added.
3. Set **Executable** (type a path or **Browse**). **Name** defaults to the executable name.
4. Press **Add app**. This saves a preset; it does not launch. Click the new tile and press **Launch**.

| App type | Settings | Processes |
|---|---|---|
| OpenXR app | Executable, Arguments (one per line) | One process per target app |
| QAROS API app | Executable, Arguments (one per line) | One process for all targets |
| StereoKit | Model, Scale, Orientation, Show gesture points | One process per target app |
| CPU Renderer | Points, Sphere radius | One process per target app |

A **QAROS API app** is an application built on the QAROS C or C# API (for example the Vulkan cube example). QAROS starts it and tells it its GPU; the app joins as a local app, so it needs the same one-time approval as when a user starts it ([Approved local apps](#approved-local-apps)). QAROS restarts it with backoff if it exits within 10 s. What the app has to do is in [Developer Guide: Launched by QAROS](/docs/developer-guide/onboarding-and-sessions#launched-by-qaros-qaros-api-apps).

## Approved local apps

A source app running on this PC joins QAROS without a code, after someone allowed it once. Whoever starts it, a user or QAROS, the first start asks:

1. **"Allow &lt;app&gt; to join QAROS?"** opens in the Visualizer, or as a window of the QAROS tray when no Visualizer is open.
2. Check what Windows reports about the app: publisher, product, executable, path and signature. Text the app supplies about itself is shown separately as "App says:"; do not rely on it.
3. Click **Allow** to admit it, or **Deny**. Deny is the default; Enter and Space never approve. Closing the window denies. Unanswered, the prompt expires after 120 s (the app can ask for longer) and the app is told nobody answered.

The first answer, in the Visualizer or the tray, wins and closes the other. Deny remembers nothing: the next start of the app asks again.

| After you allow an app that is | Later starts |
|---|---|
| code-signed | join silently, also after updates in place. Moved or re-signed: refused until you revoke it. |
| not signed | join silently while the file is unchanged. Rebuilt or updated in place: asks again. A copy elsewhere: refused until you revoke it. |

### Listing and revoking

The Visualizer's **Connection** panel has an **Approved apps** section: one row per approved app with its name and path, publisher, rule (signed publisher or exact file), when it was approved and when it was last seen.

- To revoke an app, click its trash icon and confirm **Revoke**. Its next start asks again.
- A running copy is not renewed and peers that have read the Hub's revocation list refuse it at once; others accept it until its certificate expires, at most 30 minutes.
- An app reported as "no longer matches its approval" (moved, copied, re-signed) joins again only after you revoke its old approval and allow it anew.

Approvals belong to the signed-in Windows user and survive QAROS upgrades.

## OpenXR apps

An OpenXR app runs unchanged on the QAROS OpenXR runtime, which the Hub starts once per target app. The runtime renders on the target's GPU (otherwise the first discrete GPU).

- The app's `LOCAL` reference space has its origin at the **centre of the app volume**. Place content relative to `LOCAL` and it appears inside the volume.
- **Test OpenXR** (built-in, Debug mode) draws a test scene filling its app volume, with a blue box at the centre. Use it to check the OpenXR path end to end.

## Streams to

Selects which target applications receive the app's stream:

- **All target applications** (default), or
- one checkbox per connected target application.

Changing it re-routes a running app without a restart. A QAROS API app always streams to all target applications that request it.

## Live settings

**Live settings** (CPU Renderer and StereoKit only) change a running app without a restart:

| App | Live |
|---|---|
| CPU Renderer | Points, Sphere radius |
| StereoKit | Model, Scale, Orientation, Draw a cube on each click |

Everything else (Show gesture points, executable, arguments) applies at launch: stop the app and launch it again. Live changes affect only the running app and are not written back to its preset.

## Detected apps

On start, and when you press refresh, the visualizer looks in its own folder for known applications:

| File | Tile |
|---|---|
| `qar-vulkan-source.exe` | **Vulkan cube** (QAROS API app, Debug mode only) |

## UI modes and debug-only apps

The visualizer has three UI modes: **Basic**, **Developer** and **Debug** (Debug needs the debug password in Settings). CPU Renderer, Test OpenXR and every QAROS API app are debug-only: outside Debug mode their tiles and running rows are hidden, but apps already running keep running.

## Persistence

Presets and the list of running apps are kept by the Hub on this PC, not by the visualizer. They survive visualizer and Hub restarts; when the Hub starts, it relaunches the apps that were running. Model and executable paths must stay reachable.

If a source app is killed and relaunched with the same identity, the new instance takes over its streams within about 2-3 s; see [Running the Hub](/docs/operator-guide/running-the-hub#when-an-app-restarts).
