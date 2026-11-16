---
sidebar_position: 4.5
title: Source Applications
description: Start, configure and stop the applications that stream content into the room, from the visualizer's Source Applications panel.
---

# Source Applications

A **source application** renders content into an app volume and streams it to the target applications (headsets, visualizers). You start and manage them from the visualizer's **Source Applications** panel.

To start with Rerun, RViz (ROS 2), Blender or 3D Slicer, follow its page in [Supported Apps](../user-guide/supported-apps/index.md).

## The panel

| Section | Shows |
|---|---|
| **Apps** | One row per app: built-in apps, detected apps and your presets. Above the list: search, filter by type, sort and **+**. The refresh icon in the heading looks for installed apps again. |
| **Running** | One row per running app, with its status, **Streams to**, **Stop** and, where supported, **Live settings**. Web apps have their own buttons: see [Web apps](#web-apps). |

Row tags:

| Tag | Meaning |
|---|---|
| **Built-in** | Ships with QAROS: StereoKit, CPU Renderer, Test OpenXR, Web app. |
| **Detected** | Found on this PC (see [Detected apps](#detected-apps)). |
| *(none)* | A preset you saved. |
| **Developer** | Visible from the Developer UI mode up. |
| **Debug** | Visible only in the Debug UI mode. |
| **N on** | N copies of this app are running. |

Built-in and detected apps are system defaults: they cannot be edited or removed, by the panel or by any other request to the Hub. To change one, duplicate it as a preset of your own (see [Presets](#presets)).

A known app that is not found on this PC (Blender, ROS 2 Rviz) stays in the list, greyed out, with **Locate...** (see [Locating an app QAROS did not find](#locating-an-app-qaros-did-not-find)).

## Presets

Each row's **...** menu:

| Item | Shown for | Does |
|---|---|---|
| **Duplicate as new preset** | every app | Saves a copy named `<name> copy` and opens it, so you can rename it and set another **Executable**. Use it to keep several versions side by side, such as Blender 4.5 and Blender 5.2. |
| **Locate...** / **Locate another folder...** | Blender, ROS 2 Rviz | Picks a folder for QAROS to search (see below). |
| **Remove preset...** | your presets only | Asks to confirm under the row, then removes the preset. Apps already started from it keep running. |

Saving a preset again updates it in place; nothing in the list is ever added twice by itself.

## Launching an app

1. Click a tile. A dialog opens (**Built-in app**, **Detected app** or **Edit preset**).
2. Adjust the settings and choose **Streams to**.
3. Press **Launch**.

Each **Launch** starts a new, independent copy with its own copy of the settings. Launch failures appear as error notifications (`<app name>: <error>`).

Preset dialogs also offer **Save**, **Save as new preset** and **Remove**. Removing a preset does not stop the apps started from it, and stopping an app does not remove its preset.

## Adding your own app

1. Click **+**.
2. Pick a card, the same four in every UI mode: **Add OpenXR app**, **Web app**, **Import app package** (imports a package instead, see [Importing a web app package](#importing-a-web-app-package)) or **Open QAROS supported app** (an app built on the QAROS API, such as Blender, Rviz or the ANARI demo).
3. Set **Executable** (type a path or **Browse**), or for a web app its **Address**. **Name** defaults to the executable name or the address's host.
4. Press **Add app**. This saves a preset; it does not launch. Click the new tile and press **Launch**.

| App type | Settings | Processes |
|---|---|---|
| OpenXR app | Executable, Arguments (one per line) | One process per target app |
| QAROS supported app | Executable; Arguments (one per line) in the Developer and Debug UI modes | One process for all targets |
| Web app | Address, adaptation (see [Web apps](#web-apps)) | Runs inside this visualizer, one page for all targets |
| StereoKit | Model, Scale, Orientation, Show gesture points | One process per target app |
| CPU Renderer | Points, Sphere radius | One process per target app |

To start your own app built on the QAROS API: add it as above with **Open QAROS supported app**, then launch its row. Check **Running** for its status and the Hub's log folder for its stdout/stderr. Its launch arguments start it with QAROS (for Blender, `--addons qaros_anari`), so only the Developer and Debug UI modes show and change them; every mode sets its **Executable**.

A **QAROS supported app** (QAROS API app) is an application built on the QAROS C or C# API, such as Blender with the QAROS add-on, ROS 2 Rviz with the QAROS plugin, or the ANARI demo. QAROS starts it; the app joins as a local app, so it needs the same one-time approval as when a user starts it ([Approved local apps](#approved-local-apps)). QAROS restarts it with backoff if it exits within 10 s. What the app has to do is in [Developer Guide: Started by QAROS](/docs/developer-guide/onboarding-and-sessions#started-by-qaros).

## Approved local apps

A source app running on this PC joins QAROS without a code, after someone allowed it once. Whoever starts it, a user or QAROS, the first start asks:

1. **"Allow &lt;app&gt; to join QAROS?"** opens as a window of the QAROS tray, in front of every other window, whether or not the Visualizer is open. An open Visualizer shows the same question, and its **Source Applications** panel lists it under **Waiting for approval** with **Deny** / **Approve**.
2. Check what Windows reports about the app: publisher, product, executable, path and signature. Text the app supplies about itself is shown separately as "App says:"; do not rely on it.
3. Click **Allow** to admit it, or **Deny**. Deny is the default, and **Allow** stays greyed out for one second after the window opens. Then **Allow** takes a mouse click or an accessibility Invoke (Narrator, NVDA, UI Automation); no key ever presses it. Any program running as the same Windows user can press **Allow** that way too, even while the window is covered. Closing the window denies. Unanswered, the prompt expires after 120 s (the app can ask for longer) and the app is told nobody answered.

The first answer, in the Visualizer or the tray, wins and closes the others. Deny remembers nothing: the next start of the app asks again.

| After you allow an app that is | Later starts |
|---|---|
| code-signed | join silently, also after updates in place. Moved or re-signed: refused until you revoke it. |
| not signed | join silently while the file is unchanged. Rebuilt or updated in place: asks again. A copy elsewhere: refused until you revoke it. |

### Listing and revoking

All in the Visualizer's **Source Applications** panel:

- Each detected app's tile shows **Approved** (green) or **Not approved** (amber). Built-in apps show neither.
- **Approved apps**, at the bottom, has one card per approved app: name and path, publisher, rule (signed publisher or exact file), when it was approved and when it was last seen.

- To revoke an app, press **Revoke** on its card and confirm. Its next start asks again.
- A running copy is not renewed and peers that have read the Hub's revocation list refuse it at once; others accept it until its certificate expires, at most 30 minutes.
- An app reported as "no longer matches its approval" (moved, copied, re-signed) joins again only after you revoke its old approval and allow it anew.

Approvals belong to the signed-in Windows user and survive QAROS upgrades.

## OpenXR apps

An OpenXR app runs unchanged on the QAROS OpenXR runtime, which the Hub starts once per target app. The runtime renders on the target's GPU (otherwise the first discrete GPU).

- The app's `LOCAL` reference space has its origin at the **centre of the app volume**. Place content relative to `LOCAL` and it appears inside the volume.
- **Test OpenXR** (built-in, Debug mode) draws a test scene filling its app volume, with a blue box at the centre. Use it to check the OpenXR path end to end.

## Web apps

A **web app** streams an existing website (for example a three.js viewer or configurator) without changing it. The visualizer that launches it loads the page in a built-in browser and renders each target's view from it. Unlike the other types, it runs in that visualizer, not on the Hub's launcher: close the visualizer and the web app stops.

### Launching

1. Click the built-in **Web app** tile or a web app preset.
2. Set **Address**, and optionally the adaptation:

   | Field | Meaning |
   |---|---|
   | **Stored profile** | An adaptation profile by name. Empty: the profile that best matches the address. |
   | **Page region** | The top-left rectangle, in pixels, the page keeps for its own view and interface. |
   | **App scale** | Room metres per unit of the page's scene. Off: the profile's. |
   | **Scripts** | Not edited here. A web app preset's scripts come from registering the web app through the visualizer's MCP (developer tooling); otherwise the stored profile's scripts are used. |

   Leave all of it empty to use the stored profile that matches the address.
3. Press **Launch**. The page loads; the app starts streaming by itself as soon as the visualizer finds the page's renderer.

### Web apps QAROS provides

QAROS comes with ready-made adaptations for some public web apps, each a tile in **Built-in**:

| Tile | Website | Shown from |
|---|---|---|
| **Foxglove** | `https://app.foxglove.dev/`: the largest 3D panel of any project and layout, after you sign in | Basic |
| **three.js examples** | any official three.js example (`https://threejs.org/examples/`): pick it in the gallery, for instance Littlest Tokyo (`#webgl_animation_keyframes`) | Basic |
| **three.js Shinjuku Station** | the Shinjuku Station indoor map demo, as a table model | Basic |
| WebGL Aquarium, WebGPU Deferred Rendering | developer samples, only in development builds | Developer |

These adaptations are published in the public repository
[Quaternar/qaros-webapps](https://github.com/Quaternar/qaros-webapps), each with its own version.
Every QAROS setup includes the newest release at the time it was built, and the visualizer keeps
them up to date by itself:

- At start and every hour while the PC is online, the visualizer checks the newest release. It
  installs a newer version of an adaptation you already have when that version runs with your
  QAROS version, after checking the package's SHA-256. The update is installed for the Windows user
  only and keeps the tile's settings.
- A web app that is running keeps the version it started with. Its tile shows **Update ready**; the
  new version applies the next time you start it.
- An adaptation the release adds shows as **Installable**: press **Install** on its tile to add it.
- Each tile shows its state: **Installed**, **Needs update** (press **Update**), **Update ready**,
  **Installable**, or **Not available** with the reason in its tooltip (for example, a version that
  needs a newer QAROS).
- **Settings** > **General** > **Web app updates**: turn **Update web apps automatically** off to
  download nothing by yourself, and **Check now** to look for a new release at once. The line below
  shows the release last read.
- Downloads use the PC's proxy settings. Offline, nothing changes; the next check tries again.
- The releases are not signed yet; the visualizer installs them without asking because they come
  only from this repository over HTTPS. Once the releases are signed, only packages signed by
  Quaternar install by themselves.

To change one of these adaptations, contribute to the repository, or duplicate it in the
visualizer under a new id: an update never changes your own copy.

- An adaptation a newer release no longer lists stays installed but is never picked by address,
  so it cannot override the adaptation that replaced it.
- An adaptation you installed (an import, or such a leftover) is removed with the row's **...**
  menu, **Uninstall web app adaptation...**. One that no tile opens gets a tile of its own for
  this. Adaptations that ship with QAROS cannot be uninstalled.
- To write your own adaptation, see [Web App Adaptations](../developer-guide/web-app-adaptations/index.md).

### Logging in

When the address opens on a login page, there is no renderer yet and the row shows **Waiting for a renderer - log in on Page, then Probe**:

1. Press **Page**. The **Web App Source Page** panel opens with the website.
2. Log in there with mouse and keyboard as in a normal browser.
3. Press **Probe** on the row. Streaming starts once the renderer is found.

The login is kept: all web apps in a visualizer share one browser profile, cookies included.

### Running row

| Button | Shown | Does |
|---|---|---|
| **Page** | always | Shows the page in the **Web App Source Page** panel. |
| **Details** | always | Opens the details below. |
| **Probe** | waiting for a renderer | Looks for the page's renderer again. |
| **Stop** | streaming or starting | Stops streaming. The page stays loaded. |
| **Start** | stopped | Streams again. After a failure or a visualizer restart, reloads the page first. |
| **Remove** | always | Closes the page and removes the app from the list. |

In a narrow panel the buttons move to a line under the app's name, and become icons (their name on hover) when even that line is too narrow. A name too long for two lines is cut with "..."; hover it to read it whole.

**Details** shows the address, what the probe found (engine, canvases, depth), the adaptation profile in use, active targets, frame counters, the last error and the event log (**Clear**, **Copy**). **Probe again** probes now; **Reload** loads the page again without streaming. In the Developer and Debug UI modes, **Profile** switches the live app to another stored profile and reloads the page.

A red warning above the Running list means the web apps together are close to the GPU's memory limit: remove one before starting another.

### Web App Source Page panel

Shows the page of the app last selected with **Page**, and passes mouse, wheel and keyboard input to it.

| Control | Does |
|---|---|
| **Page Size** | **Page Width** and height, then **Apply Size**, resize the page itself. Resizing the panel only rescales the picture. |

### After a restart

Web apps the visualizer was running come back after a restart as **Stopped**, with their address and adaptation. Press **Start** to load and stream them again.

## Streams to

Selects which target applications receive the app's stream:

- **All target applications** (default), or
- one checkbox per connected target application.

Changing it re-routes a running app without a restart. For a QAROS API app, **Streams to** shows *Chosen by the app* instead: the app decides which target applications it renders for, one, some or all of them (see [Rendering Streams](/docs/developer-guide/rendering-streams#finding-rendering-targets)).

## Live settings

**Live settings** (CPU Renderer and StereoKit only) change a running app without a restart:

| App | Live |
|---|---|
| CPU Renderer | Points, Sphere radius |
| StereoKit | Model, Scale, Orientation, Draw a cube on each click |

Everything else (Show gesture points, executable, arguments) applies at launch: stop the app and launch it again. Live changes affect only the running app and are not written back to its preset.

## Locating an app QAROS did not find

QAROS looks for Blender (4.5 or newer) and ROS 2 distributions with rviz2 in their usual places. When it finds none, the app's row stays greyed out with **Locate...**:

1. Press **Locate...** on the row.
2. Pick the folder: for Blender the folder with `blender.exe` (an unzipped blender.org download works), for ROS 2 Rviz the distribution folder with its setup script and rviz2.
3. The row becomes usable. Every **Refresh** searches that folder from then on; picking the same folder again adds nothing.

To add another Blender version while one is already found, use **Locate another folder...** in the row's **...** menu, or duplicate the row and set its **Executable**.

## Importing a web app package

A web app adaptation exported on another PC is a `.qarapp` package. Click **+**, then the **Import app package** card, and pick the file. The package is checked as every package is (signature, publisher trust, version) before it installs; an unsigned package or one from an unknown publisher asks first. Importing the same package again updates its row instead of adding a second one.

## Detected apps

On start, and when you press refresh, the visualizer looks in its own folder for known applications:

| File | Tile |
|---|---|
| `qar-vulkan-source.exe` | **Vulkan cube** (QAROS supported app, Developer mode and up) |

## UI modes and mode-gated apps

The visualizer has three UI modes: **Basic**, **Developer** and **Debug** (Debug needs the debug password in Settings).

| Apps | Shown from |
|---|---|
| StereoKit, your OpenXR presets, the web apps QAROS provides, QAROS supported apps (Blender, Rviz, your own) | Basic |
| The QAROS examples (Vulkan cube, ANARI cube), the web app developer samples | Developer |
| CPU Renderer, Test OpenXR | Debug |

In a lower mode their tiles and running rows are hidden, but apps already running keep running.

## Persistence

Presets and the list of running apps are kept by the Hub on this PC, not by the visualizer. They survive visualizer and Hub restarts; when the Hub starts, it relaunches the apps that were running. Model and executable paths must stay reachable. Web apps are the exception: the Hub keeps their presets, but the visualizer that launched them keeps the running ones and restores them stopped (see [After a restart](#after-a-restart)).

If a source app is killed and relaunched with the same identity, the new instance takes over its streams within about 2-3 s; see [Running the Hub](/docs/operator-guide/running-the-hub#when-an-app-restarts).
