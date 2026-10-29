---
sidebar_position: 11
title: Unity
description: Install the QAROS Unity packages, onboard a Unity app with the Hub, stream a scene to one or more end users, and use MRTK 3 with QAROS hands.
---

# Unity

QAROS ships two Unity packages that stream a Unity scene to AR/VR devices. They carry the
[C# binding](/docs/developer-guide/language-bindings) and the native runtime, so a Unity project
needs nothing else installed.

| Package | Folder | Install when | Depends on |
|---|---|---|---|
| `com.quaternar.streaming` | `QarStreaming` | Always | Unity Input System `1.7.0`, Unity UI `1.0.0` |
| `com.quaternar.streaming.mrtk` | `QarStreamingMRTK` | Only in MRTK 3 projects | `com.quaternar.streaming`, Unity Input System `1.7.0`, XR Core Utils `2.2.3`, MRTK Core `4.0.0-pre.1`, MRTK Input `4.0.0-pre.1`, MRTK UX Components `4.0.0-pre.1`, MRTK Extended Assets `3.0.3` |

- **`QarStreaming`** is the integration: the `QarStreaming` and `QarDevice` prefabs, stereo
  camera capture for the built-in render pipeline and URP, a sample scene, the C# binding and the
  Windows x64 native runtime in `lib/windows-x64`.
- **`QarStreamingMRTK`** extends it, never replaces it: an MRTK XR rig prefab, MRTK
  profile and configuration assets, QAROS hands as an MRTK hand subsystem, the QAROS head pose
  driving the `XROrigin`, and an MRTK sample scene.

## Requirements

- Unity **2022.3** or newer. Tested with 2022.3 and 6000.4.
- **Windows x64.** The native runtime is built for Windows x64 only.
- **Direct3D 11 or Vulkan.** In *Project Settings > Player > Other Settings*, turn off **Auto
  Graphics API for Windows** and put Direct3D11 or Vulkan first in the list. Direct3D 12 and
  OpenGL are not supported. See [Graphics APIs](#graphics-apis) for how the two differ.
- The built-in render pipeline or URP.
- A running **QAROS Hub** (see [Installation](/docs/operator-guide/installation)). The app
  joins it, and streams to the target apps in its session.

## Installation

The Unity ZIP (`QAROS Unity <version>.zip`, see [Installation](/docs/operator-guide/installation))
holds both packages, ready to install, plus a short `INSTALL.md` and the `CHANGELOG.md`.

1. Unzip it to a location you keep. A package added from disk is referenced, not copied:
   moving or deleting the folder breaks the project.
2. MRTK 3 projects only: install the MRTK packages first (see [MRTK 3](#mrtk-3)).
3. Add the packages in this order. Unity cannot resolve `QarStreamingMRTK`'s dependency on a
   local package by itself, so the base package must already be there.
   1. `QarStreaming/package.json`
   2. `QarStreamingMRTK/package.json`, only if the project uses MRTK 3

### With the Package Manager

1. Open your Unity project.
2. Open *Window > Package Manager*.
3. Press **+** and choose **Add package from disk...**.
4. Select the package's `package.json`.

### With the manifest file

Alternatively, add the packages to the project's `Packages/manifest.json`. The paths are relative
to that file. With the unzipped folder next to the Unity project:

```json
{
  "dependencies": {
    "com.quaternar.streaming": "file:../../QAROS-Unity-<version>/QarStreaming",
    "com.quaternar.streaming.mrtk": "file:../../QAROS-Unity-<version>/QarStreamingMRTK"
  }
}
```

### Release and demo

`QAROS Unity <version>.zip` and `QAROS Unity Demo <version>.zip` carry the same package names, so
a project has one of them. The demo packages' display names end in "(Demo)", which shows in the
Package Manager which runtime is installed. To switch, remove the package in the Package Manager
and add the other one.

## The sample scene

Open `QarSampleScene` from the `QarStreaming` package's `Samples` folder. It has one `QarStreaming`
prefab and a **Connect** button that calls `QarMultiUserStreamingBehaviour.Join`.

1. Start QAROS on this machine, and a target: a headset player, or a Visualizer receiver.
2. Enter Play mode. The app joins the Hub on this PC as a local app; the first time, click
   **Allow** in the QAROS approval prompt.
3. The scene streams to every reachable target. The target list shows in the game view and in the
   `QarStreaming` Inspector.

## Prefabs

| Prefab | Role | Contains |
|---|---|---|
| `QarStreaming` | The streaming coordinator, one per scene | `QarMultiUserStreamingBehaviour` (onboarding, which targets to stream to, render and gesture settings, the device prefab, runtime settings such as `DllFolder` and `EnableMixer`) and `QarTargetPickerOverlay` (the in-game target list) |
| `QarDevice` | One target's view | `QarDeviceBehaviour`, a left-eye and a right-eye camera, and optional hand and head visualizers |

`QarMultiUserStreamingBehaviour` creates a child `QarDevice` for each target it streams to, named
after the target, and destroys it when streaming to that target stops.

## Onboarding

The app joins the Hub's session once; every target is streamed to through that session. The
general flow is in [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions); in
Unity it is driven by **Onboarding** on `QarMultiUserStreamingBehaviour`.

- **LocalApp** (the default) joins the Hub on this PC with no code. The first time, QAROS asks the
  user to allow the app (the Unity Editor once for every project opened in it); later starts join
  silently. Nothing is saved.
- **Code** onboards with a Hub's pairing code, for a Hub on another machine. The onboarding ID is
  saved in `PlayerPrefs` and rejoins silently later; `ForgetOnboarding()` erases it.

| Setting | Use |
|---|---|
| **Mode** | `LocalApp` or `Code`. A **Hub Host** always uses `Code`. |
| **Code** | The pairing code (Code mode). Used only when the saved onboarding ID cannot rejoin. |
| **Hub Host**, **Hub Port** | Onboard with a Hub on another machine. A remote Hub needs its port too: Hubs have no fixed onboarding port. |
| **Approval Timeout Seconds** | LocalApp: how long to wait for the user to allow the app. 0 uses the Hub's default (120 s). |
| **Use Unity Graphics Adapter** | Creates the session's GPU devices on the adapter Unity renders with (NVIDIA only), so frames need no copy between GPUs. `QAR_GPU_ADAPTER_ID` in the environment overrides it (see [Choosing the GPU](/docs/developer-guide/onboarding-and-sessions#choosing-the-gpu)). |
| **Onboarding Id Key**, **Planned Peer Id**, **Timeout Seconds** | Code mode details: where the ID is saved, joining under a planned peer ID, how long to wait. |

`Join()` blocks until the Hub answers. When it fails, `LastJoinError` says why and the target list
offers to join again.

## Targets

A target is a target app already in the Hub's session: a HoloLens 2, Meta Quest or Metalens player,
the Android streamer, or a Visualizer receiver. **The app never invites or starts a device.** A
target appears in the list once it joined the Hub on its own (see
[Player Installation to Device](/docs/operator-guide/player-installation-to-device)), and leaves it
when it is gone.

- **Stream To All Targets** (on by default) streams to every reachable target, including ones that
  join later.
- Turn it off to pick: only the targets checked in the list are streamed to.
- Each row shows the target's display name, its device type and its state; hover it for the peer
  ID.
- The list is in the game view (`QarTargetPickerOverlay`, immediate-mode GUI, no Canvas needed) and
  in the `QarStreaming` Inspector in Play mode. Turn the overlay's **Visible** off, or remove it, to
  build your own UI on the API below.
- A target that fails to connect is tried again every few seconds while it stays picked.

| Member | Does |
|---|---|
| `Join()`, `IsJoined`, `LastJoinError` | Joins the Hub and starts listing targets; whether it did; why not |
| `Targets` | The target list (`QarStreamTarget`: `Id`, `DisplayName`, `Category`, `DeviceId`, `DeviceFamily`, `DeviceName`, `DeviceLabel`, `Presence`, `IsOnline`, `IsReachable`, `Selected`, `Device`, `ConnectionState`, `LastError`) |
| `TargetsChanged` | Raised on the main thread when a target joins, leaves, changes or is picked |
| `StreamToAllTargets` | Stream to every reachable target, or only the picked ones |
| `SetTargetSelected(PeerId, bool)`, `IsStreamedTo(target)` | Picks a target; whether it is streamed to |
| `Leave()` | Stops streaming to every target and stops listing them |
| `ForgetOnboarding()` | Leaves, ends the session and erases the saved onboarding ID (Code mode) |

### App volume

Every target shows the scene in one [app volume](/docs/developer-guide/app-volumes), identified by
**App Volume Common Name** (the product name when empty), so all users see the same scene placed
once. Joining again gets the same volume back with its placement.

### Gestures

**Enable Gestures** turns QAROS [app volume gestures](/docs/developer-guide/app-volumes#gestures) on
or off for the targets' hands. Keep it on for normal QAROS gesture handling. Turn it off when
another input stack consumes the hands.

## Coordinate spaces

The views and hands a target's device receives are in its app volume's
[app content space](/docs/developer-guide/coordinate-systems#app-content-space--your-applications-world-inside-the-box),
in OpenXR convention: right-handed, +Y up, -Z forward, in app units. The Unity world is the app
content root.

- `QarDeviceBehaviour.ToUnityPosition`, `ToUnityRotation`, `ToUnityLinearVelocity` and
  `ToUnityAngularVelocity` convert to Unity's left-handed space. They only flip handedness.
- `LeftHandJoints` and `RightHandJoints` stay in app content space: convert them with those methods.
- Never apply the app volume's pose or scale on top: the Unity world already is the app content
  space.

## Transparency

`QarDeviceBehaviour` declares what its color's alpha means with the stream, through **Alpha Mode**
in the Inspector. The mixer composites the stream by it
([Transparency](/docs/developer-guide/rendering-streams#transparency)).

| Alpha Mode | Use when |
|---|---|
| `Premultiplied` (default) | The cameras clear to transparent black, so translucent materials, soft edges and glow are composited over what is behind the app volume |
| `Straight` | Your color is not multiplied by its alpha (for example a custom pass writes straight alpha). QAROS premultiplies it before sending |
| `Opaque` | Post-processing or a custom pass overwrites the alpha channel. With `Premultiplied` such content would turn transparent |
| `Unspecified` | Declares nothing: the mixer draws the stream opaque unless the operator enables alpha for undeclared streams |

- Unity's transparent materials write no depth. They show at the middle of the app volume along
  each view ray, so they occlude there and move without parallax.
- Opaque materials must leave alpha at 1. A shader that writes a lower alpha on an opaque surface
  makes it translucent in the mixer.

## MRTK 3

`QarStreamingMRTK` puts QAROS behind MRTK 3: the QAROS head pose drives the MRTK rig and the QAROS
hands become MRTK hands.

- **Install MRTK first** with the [Mixed Reality Feature Tool](https://learn.microsoft.com/en-us/windows/mixed-reality/develop/unity/welcome-to-mr-feature-tool).
  The Unity Package Manager may not offer the MRTK versions the package needs.
- **It uses the first streamed target only.** To keep a hand from being handled twice, by MRTK and
  by QAROS app volume gestures, it turns gestures off for that target's device at runtime. The
  **Enable Gestures** value in the Inspector does not change. Further targets stream as usual.
- **The MRTK sample scene** `QarSampleSceneMrtk` has a debug **Connect** button built with MRTK UX
  Components and MRTK Extended Assets. The integration itself does not use those two packages;
  only that button does.

## Player builds

A player build copies the native runtime next to the executable, under `lib/windows-x64`. Ship
that folder with the player.

## Graphics APIs

The package streams under Direct3D 11 and under Vulkan, in the Editor and in player builds.

- **Any other graphics API is refused.** Under Direct3D 12 or OpenGL the runtime does not start:
  it throws a `NotSupportedException` that names the API, and nothing streams.
- **Vulkan needs `QarUnityGraphics.dll`**, which the package ships in `lib/windows-x64`. It is the
  one native plugin Unity loads itself, at startup, so that it can prepare Unity's Vulkan device
  for sharing frames. Keep **Load on startup** set in its import settings. Without it a Vulkan
  player cannot stream, and the runtime refuses to start with a `NotSupportedException` that says
  why.
- **Under Vulkan, cap the frame rate instead of using VSync.** A Vulkan frame is shown only after
  Unity's render thread has handed it over. **Show Vulkan Frames On Worker**, on `QarDevice` and
  on by default, waits for that hand-off on a worker thread instead of the next frame.
  - In our measurements a Vulkan player capped at 60 fps with `Application.targetFrameRate` streamed
    at the Direct3D 11 rate, with frames within 2 ms as fresh.
  - Under VSync it streamed a little over half the Direct3D 11 rate (46 against 79 per second on a
    165 Hz display), so set `QualitySettings.vSyncCount` to 0 and cap with `targetFrameRate`.
  - Turning the option off shows every frame from the main thread, which streams about half as many
    frames at a cap. Leave it on unless you need that fallback.
  - With the option on, the package can move the eye cameras, hands and headset model just before
    the first eye camera renders. A script reading them in `LateUpdate` may see the previous frame's
    pose.
