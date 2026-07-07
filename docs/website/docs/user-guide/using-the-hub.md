---
sidebar_position: 4
title: Using the Hub
description: Running the QarOS Hub — tray service, runtime launcher, the visualizer, and the launcher configuration.
---

# Using the Hub

The Hub is not a single program but three cooperating processes that ship together:

| Process | Role |
|---|---|
| `qaros-hub-service.exe` | Tray companion. Always-on supervisor: starts the launcher, restarts it if it crashes, and gives you the tray menu. |
| `qar-runtime-launcher.exe` | The actual Hub runtime: hosts the session and message router, runs the private CA and onboarding services, broadcasts discovery, and orchestrates the streaming pipeline (source apps, mixers, targets). |
| `qar-streaming-viz.exe` | The visualizer — the interactive desktop window where you see and manipulate the shared room. |

## Starting the Hub

1. Start **`qaros-hub-service`** (or let it start with Windows, if the startup task is enabled). It launches the runtime launcher automatically and keeps it running — crashes are restarted with backoff.
2. **Left-click the tray icon** to open the visualizer window.
3. The tray menu offers: *Open visualizer*, *Restart launcher*, *Connect to other Hub*, and *Exit*.

On startup the launcher restores its previous session (or creates a fresh one), registers the configured source app volumes, starts the streaming services, begins broadcasting the discovery beacon (default display name "QarOS Hub"), and starts the onboarding services. By default it also invites a local visualizer into the session automatically.

You can also run `qar-runtime-launcher.exe` directly for headless or development scenarios — it reads its configuration from a JSON line on stdin, or falls back to the `qar-runtime-launcher.jsonc` file next to the executable.

## The visualizer

The visualizer is the Hub's window into the shared room: it renders the same mixed scene a headset user would see, and hosts the desktop-side UI panels.

:::note TODO
Document the visualizer's UI surface for operators, with screenshots: the app-volumes panel (moving/resizing volumes), the warping/timing monitor, the hub-connect screen, and the onboarding screen that displays the current pairing code / QR. The pairing-code backend (codes rotating every ~8 seconds with a 10-second TTL, published for UI consumption) is implemented; verify which visualizer screen surfaces the local Hub's code before documenting the exact click path.
:::

## Connecting two Hubs

From the tray menu choose **Connect to other Hub**. The visualizer shows a hub-connect screen where you enter the remote Hub's host address and its current pairing code. After a successful connection the two Hubs merge their trust bundles — devices onboarded to either Hub are accepted by both. See [Security and Onboarding](/docs/user-guide/security-and-onboarding#hub-to-hub-federation).

## The launcher configuration

The launcher is configured by a single JSON(C) document — `qar-runtime-launcher.jsonc`. The important groups of settings:

### Source apps (`sourceApps[]`)

The applications whose content appears in the room. Each entry declares:

- a stable **`appVolumeCommonName`** — the identity of the app's volume (keep it constant across restarts),
- the volume's initial **pose and size** in room space,
- the app **type** (StereoKit GUI, CPU renderer, Ogre, ZED camera, VRGB, …) and how to launch it.

### Target apps (`targetApps[]`)

The devices to bring into the session automatically at startup. Each entry gives a **device type** (HoloLens headset, Meta Quest, Android streamer, camera visualizer, desktop visualizer) plus, where needed, the device's **hostname/IP** and per-device configuration overrides. The launcher sends each configured target an onboarding invite when it starts.

### Networking

- `discoveryPort` / `discoveryHost` / `discoveryDisplayName` — the discovery beacon (default port 7445, name "QarOS Hub").
- `onboardingPakePort` — the pairing service port (0 = auto).
- `sessionEndpoints[]` — explicit session addresses to embed in invites (default: auto-resolved).
- `onboardingInviteIntervalSec` — how often a fresh pairing code is minted (default 8 s; codes live 10 s).

### Behavior flags

- `sendDefaultInvite` — auto-invite a local visualizer (default on),
- `enableMixer`, `unifyTargetAppWithMixer` — streaming-pipeline placement,
- `applyDeterministicIds` — keep stable peer IDs across restarts,
- `caDir`, `runtimeStorageRoot`, `logFolder` — where identity, CA, and log data live.

:::note TODO
Publish a complete, annotated reference of the launcher configuration schema (every key of `qar-runtime-launcher.jsonc` with type, default, and example), generated from or verified against `LauncherConfig.cpp`. The list above covers the operationally important keys only.
:::
