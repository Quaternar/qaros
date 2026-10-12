---
sidebar_position: 11
title: Unity
description: Install the QAROS Unity packages, stream a Unity scene to one or more end users, and use MRTK 3 with QAROS hands.
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
- **Direct3D 11.** In *Project Settings > Player > Other Settings*, turn off **Auto Graphics API
  for Windows** and put Direct3D11 first in the list. Other graphics APIs are not supported yet.
- The built-in render pipeline or URP.

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

Open `QarSampleScene` from the `QarStreaming` package's `Samples` folder and press **Connect**.
The button calls `QarMultiUserStreamingBehaviour.InitAndConnectAll`. The scene has one
`QarStreaming` prefab with one endpoint whose `IpAddress` is `visualizer`, so it streams to the
Visualizer on the Hub machine.

## Prefabs

| Prefab | Role | Contains |
|---|---|---|
| `QarStreaming` | The streaming coordinator, one per scene | `QarMultiUserStreamingBehaviour`: the endpoint list, per-endpoint render and pose settings, the device prefab, and runtime-wide settings such as the native runtime folder (`DllFolder`) and `EnableMixer` |
| `QarDevice` | One end user's view | `QarDeviceBehaviour`, a left-eye and a right-eye camera, and optional hand and head visualizers |

`QarMultiUserStreamingBehaviour` creates a child `QarDevice` for every endpoint that has none,
before it initializes or connects. Each generated child is named after its endpoint and takes
that endpoint's render and pose settings.

## Endpoints

An endpoint is one end user. Its `IpAddress` names the device to stream to:

| `IpAddress` | Streams to |
|---|---|
| `visualizer` | The Visualizer on the Hub machine |
| `android:<host>` | An Android device, such as P&C Solutions METALENSE 2 |
| `quest:<host>` | A Meta Quest |
| `hololens:<host>`, or a bare host name or IP address | A HoloLens 2 |

The device needs its player app installed, see
[Player Installation to Device](/docs/operator-guide/player-installation-to-device).

### One end user

Add one `QarStreaming` prefab to the scene and set its default endpoint.

### Multiple end users

1. Add one `QarStreaming` prefab to the scene.
2. In `QarMultiUserStreamingBehaviour`'s **Endpoints**, add one entry per end user.
3. Set each endpoint's **Name**, **Ip Address**, **Enable Gestures**, and its render and pose
   settings (**Near**, **Far**).
4. Change the runtime-wide settings on `QarMultiUserStreamingBehaviour` if the defaults do not
   suit you.
5. Call `QarMultiUserStreamingBehaviour.InitAndConnectAll`, or enable **Connect On Start**.

- The native runtime is initialized once and shared by all endpoints.
- Every enabled endpoint connects on its own. If one fails to connect, the others are still
  attempted.
- Several endpoints may all use `visualizer`: each gets its own Visualizer view.

### Gestures

Each endpoint's **Enable Gestures** turns QAROS [app volume gestures](/docs/developer-guide/app-volumes#gestures)
on or off for that end user's hands. Keep it on for normal QAROS gesture handling. Turn it off
when another input stack consumes that end user's hands.

## Connecting and disconnecting

`QarMultiUserStreamingBehaviour` has these methods for UI buttons and scripts:

| Method | Does |
|---|---|
| `InitAndConnectAll()` | Initializes the runtime, then connects every enabled endpoint |
| `ConnectAll()`, `ConnectEndpoint(int endpointIndex)` | Connects every enabled endpoint, or one |
| `DisconnectAll()`, `DisconnectEndpoint(int endpointIndex)` | Disconnects every endpoint, or one |
| `ReconnectAll()`, `ReconnectEndpoint(int endpointIndex)` | Disconnects and connects again |

- Disconnecting releases the endpoint's native objects but keeps its `QarDevice` GameObject, ready
  to reconnect.
- When `QarMultiUserStreamingBehaviour` is destroyed, it disconnects every endpoint and destroys
  only the `QarDevice` GameObjects it generated.

### Changing endpoints at runtime

Scripts can change the endpoint list with `SetEndpoints(Endpoint[] endpoints)`, `AddEndpoint(...)`,
`RemoveEndpointAt(int endpointIndex)` and `RefreshEndpointDevices()`.

- A removed endpoint is disconnected.
- A generated `QarDevice` that no endpoint uses any more is destroyed.
- An endpoint without a device gets a new one from the `QarDevice` prefab.

### Connection status

Each endpoint reports `CurrentConnectionState`, `LastResolvedConnectionString`, `LastError`,
`LastConnectSucceeded` and `LastDisconnectSucceeded`. `GetEndpointConnectionState`,
`GetEndpointLastError` and `GetEndpointLastResolvedConnectionString` look them up by endpoint
index.

## MRTK 3

`QarStreamingMRTK` puts QAROS behind MRTK 3: the QAROS head pose drives the MRTK rig and the QAROS
hands become MRTK hands.

- **Install MRTK first** with the [Mixed Reality Feature Tool](https://learn.microsoft.com/en-us/windows/mixed-reality/develop/unity/welcome-to-mr-feature-tool).
  The Unity Package Manager may not offer the MRTK versions the package needs.
- **It uses the first endpoint only** (`Endpoints[0]`). To keep a hand from being handled twice,
  by MRTK and by QAROS app volume gestures, it turns gestures off for that endpoint at runtime.
  The **Enable Gestures** value in the Inspector does not change. Further endpoints stream as
  usual, with gestures as configured.
- **The MRTK sample scene** `QarSampleSceneMrtk` has a debug **Connect** button built with MRTK UX
  Components and MRTK Extended Assets. The integration itself does not use those two packages;
  only that button does.

## Player builds

A player build copies the native runtime next to the executable, under `lib/windows-x64`. Ship
that folder with the player.

:::info
The package logs a warning if the project runs with a graphics API other than Direct3D 11. It
does not stop, but streaming is only supported with Direct3D 11.
:::
