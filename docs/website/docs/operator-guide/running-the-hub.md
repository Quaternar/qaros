---
sidebar_position: 4
title: Running the Hub
description: Running the QAROS Hub - tray service, runtime launcher, and the visualizer.
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

Hub configuration is managed through the Hub UI and persists across restarts.

## The visualizer

The visualizer is the Hub's window into the shared room: it renders the same mixed scene a headset user would see, and hosts the desktop-side UI panels.

A full operator walkthrough of the visualizer's UI surface - the app-volumes panel (moving and resizing volumes), the warping/timing monitor, the hub-connect screen, and device onboarding - with screenshots is not yet captured. The underlying features run as described elsewhere in this guide; this section will grow into a screen-by-screen tour once those screens are captured.

## Connecting two Hubs

Connecting this Hub to another so devices trust both is covered in [Networking & Federation](/docs/operator-guide/networking-and-federation).
