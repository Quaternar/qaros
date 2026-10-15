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
- **Direct3D 11.** In *Project Settings > Player > Other Settings*, turn off **Auto Graphics API
  for Windows** and put Direct3D11 first in the list. Other graphics APIs are not supported yet.
- The built-in render pipeline or URP.
- A running **QAROS Hub** (see [Installation](/docs/operator-guide/installation)). The app
  onboards with it, and every connection goes through it.

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
prefab with one endpoint whose **Connection Target** is `visualizer`, and a **Connect** button
that calls `QarMultiUserStreamingBehaviour.InitAndConnectAll`.

1. Start the Hub on this machine.
2. The first time only: put the Hub's current pairing code into
   **Onboarding > Code** on the `QarStreaming` object (see [Onboarding](#onboarding)).
3. Enter Play mode and press **Connect**. The scene streams to the Visualizer.

## Prefabs

| Prefab | Role | Contains |
|---|---|---|
| `QarStreaming` | The streaming coordinator, one per scene | `QarMultiUserStreamingBehaviour`: the onboarding settings, the endpoint list with per-endpoint render and pose settings, the device prefab, and runtime-wide settings such as the native runtime folder (`DllFolder`) and `EnableMixer` |
| `QarDevice` | One end user's view | `QarDeviceBehaviour`, a left-eye and a right-eye camera, and optional hand and head visualizers |

`QarMultiUserStreamingBehaviour` creates a child `QarDevice` for every endpoint that has none,
before it initializes or connects. Each generated child is named after its endpoint and takes
that endpoint's render and pose settings.

## Onboarding

Every connection goes through the QAROS Hub. The app joins the Hub's session once, and all its
endpoints share that session. The general flow is in
[Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions); in Unity it is driven by
**Onboarding** on `QarMultiUserStreamingBehaviour`.

1. **First connect:** read the Hub's current pairing code from the Visualizer's
   [Connection panel](/docs/user-guide/connection-panel#onboard-a-device), put it into
   **Onboarding > Code**, and connect within a few seconds. Codes are single-use and short-lived.
2. **Every later connect** rejoins silently with the onboarding ID saved in `PlayerPrefs` under
   **Onboarding Id Key**. The code is only used again when the Hub no longer knows that ID.
3. **To unpair**, call `ForgetOnboarding()`. It disconnects, ends the session and erases the saved
   ID; the next connect needs a fresh code.

| Setting | Use |
|---|---|
| **Code** | The pairing code. Used only when the saved onboarding ID cannot rejoin. |
| **Hub Host**, **Hub Port** | Onboard with a Hub on another machine. Leave **Hub Host** empty to use the Hub on this machine. A remote Hub needs its port too: Hubs have no fixed onboarding port. |
| **Onboarding Id Key** | The `PlayerPrefs` key the onboarding ID is saved under. |
| **Use Unity Graphics Adapter** | Creates the session's GPU devices on the adapter Unity renders with (NVIDIA only), so frames need no copy between GPUs. On by default; `QAR_GPU_ADAPTER_ID` in the environment overrides it (see [Choosing the GPU](/docs/developer-guide/onboarding-and-sessions#choosing-the-gpu)). |
| **Planned Peer Id** | Joins under a given peer ID. Leave it empty unless another component planned this app's peer ID (see [Joining under a planned peer id](/docs/developer-guide/onboarding-and-sessions#joining-under-a-planned-peer-id)). |
| **Timeout Seconds** | How long to wait for rejoin or onboarding before giving up. |

If the session cannot be obtained, every enabled endpoint reports that error in `LastError`.

## Endpoints

An endpoint is one end user. Its **Connection Target** says where its frames go:

| Connection Target | What happens |
|---|---|
| `visualizer`, `visualizer:N` | Streams to a Visualizer already running on this machine's Hub: the N-th one, `0` by default. Nothing is launched. |
| `hololens:<host>`, or a bare host name or IP address | This machine's Hub starts the HoloLens 2 player at `<host>`. |
| `quest:<host>` | The same for a Meta Quest. |
| `android:<host>` | The same for an Android device, such as P&C Solutions METALENSE 2. |

- A player the endpoint started is stopped again when the endpoint disconnects.
- Connecting returns at once with the endpoint in `Connecting`. Finding or starting the target and
  creating the stream run in the background.
- Endpoints that all use `visualizer` stream to the same Visualizer; use `visualizer:N` to reach
  another one.
- The device needs its player installed, see
  [Player Installation to Device](/docs/operator-guide/player-installation-to-device).

### App volumes

Each endpoint streams into its own [app volume](/docs/developer-guide/app-volumes), identified by
its **App Volume Common Name**. Left empty, the name is `<product name>/<endpoint name>`.
Reconnecting gets the same volume back with its placement, and disconnecting keeps it. Give
endpoints distinct names when they must not share a volume.

### One end user

Add one `QarStreaming` prefab to the scene and set its default endpoint.

### Multiple end users

1. Add one `QarStreaming` prefab to the scene.
2. In `QarMultiUserStreamingBehaviour`'s **Endpoints**, add one entry per end user.
3. Set each endpoint's **Name**, **Connection Target**, **App Volume Common Name**,
   **Enable Gestures**, and its render settings (**Near**, **Far**).
4. Change the runtime-wide settings on `QarMultiUserStreamingBehaviour` if the defaults do not
   suit you.
5. Call `QarMultiUserStreamingBehaviour.InitAndConnectAll`, or enable **Connect On Start**.

- The native runtime is initialized once, and the session is rejoined or onboarded once, for all
  endpoints.
- Every enabled endpoint connects on its own. If one fails to connect, the others are still
  attempted.

### Gestures

Each endpoint's **Enable Gestures** turns QAROS [app volume gestures](/docs/developer-guide/app-volumes#gestures)
on or off for that end user's hands. Keep it on for normal QAROS gesture handling. Turn it off
when another input stack consumes that end user's hands.

## Connecting and disconnecting

`QarMultiUserStreamingBehaviour` has these methods for UI buttons and scripts:

| Method | Does |
|---|---|
| `InitAndConnectAll()` | Initializes the runtime, obtains the session, then connects every enabled endpoint |
| `ConnectAll()`, `ConnectEndpoint(int endpointIndex)` | Connects every enabled endpoint, or one |
| `DisconnectAll()`, `DisconnectEndpoint(int endpointIndex)` | Disconnects every endpoint, or one |
| `ReconnectAll()`, `ReconnectEndpoint(int endpointIndex)` | Disconnects and connects again |
| `ForgetOnboarding()` | Disconnects, ends the session and erases the saved onboarding ID |

- Disconnecting releases the endpoint's native objects but keeps its `QarDevice` GameObject, ready
  to reconnect.
- The shared session ends when the last endpoint and the coordinator have released the runtime.
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

## Coordinate spaces

The views and hands an endpoint receives are in its app volume's
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
