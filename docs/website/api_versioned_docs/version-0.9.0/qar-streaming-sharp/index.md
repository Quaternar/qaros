---
title: "QAR Streaming C# API"
sidebar_label: "Overview"
slug: "/qar-streaming-sharp/"
description: "Reference of the QAROS .NET binding: the public types of the Quaternar.Qaros namespaces, grouped by area."
---


# QAR Streaming C# API

`Quaternar.Qaros` is the .NET binding of the QAROS streaming C API. It targets
netstandard2.0, netstandard2.1 and net8.0, loads the same native `qar-streaming-c`
library, and is the C# surface Unity uses too.

- **`Quaternar.Qaros`** (this reference) is the managed API: `IDisposable` handles, `Result<T>`
  return values, C# events and `...Async` variants. Every public type is listed below.
- **`Quaternar.Qaros.Mathematics`** holds `Vector3`, `Vector3d`, `Quaternion` and `Pose`, apart
  from the main namespace so they do not clash with the `UnityEngine` or `System.Numerics` types
  of the same names. Add `using Quaternar.Qaros.Mathematics;` where you use them.
- **`Quaternar.Qaros.Interop`** is the internal P/Invoke layer the managed API is built on. It is not
  part of the public surface and is not documented here.

Concepts, threading rules and usage snippets live in the
[Developer Guide](/docs/developer-guide/language-bindings), next to the C examples.

## Entry points

1. [`Library.Init`](Quaternar.Qaros.Library.md) loads the native library, once per process.
2. [`Runtime.Create`](Quaternar.Qaros.Runtime.md) creates the runtime. Its `Onboard` joins a session
   for the first time and returns an [`OnboardSession`](Quaternar.Qaros.OnboardSession.md); `Rejoin`
   returns to a session joined before.
3. A [`Session`](Quaternar.Qaros.Session.md) exposes one control per area:
   [`Peers`](Quaternar.Qaros.PeerControl.md), [`GuiPanels`](Quaternar.Qaros.GuiPanelControl.md),
   [`AppVolumes`](Quaternar.Qaros.AppVolumeControl.md), [`RenderSenders`](Quaternar.Qaros.RenderSenderControl.md)
   and [`PrimarylessVideoSenders`](Quaternar.Qaros.PrimarylessVideoSenderControl.md).
4. Calls return a [`Result<T>`](Quaternar.Qaros.Result-1.md): check `IsSuccess`, or use `ValueOrThrow()`.
   Failures carry a [`StatusCode`](Quaternar.Qaros.StatusCode.md).

## Library and runtime

Load the native library and create the runtime that onboards and rejoins sessions.

| Type | Kind | Description |
|---|---|---|
| [`Library`](Quaternar.Qaros.Library.md) | class | Process-wide library state. [`Library.Init`](Quaternar.Qaros.Library.md#init) comes first, before any other call of this API; then [`Runtime.Create`](Quaternar.Qaros.Runtime.md#create), then [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) or [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) for a [`Session`](Quaternar.Qaros.Session.md). |
| [`LibraryOptions`](Quaternar.Qaros.LibraryOptions.md) | class | Arguments of [`Library.Init`](Quaternar.Qaros.Library.md#init), which must run once per process before any other call. The defaults match `qar_library_init_default()`. |
| [`LogSeverity`](Quaternar.Qaros.LogSeverity.md) | enum | Minimum severity the library logs; see [`LibraryOptions.LogSeverity`](Quaternar.Qaros.LibraryOptions.md#logseverity). |
| [`Runtime`](Quaternar.Qaros.Runtime.md) | class | Hosts the identity slots a process onboards with and produces its sessions. Create it after [`Library.Init`](Quaternar.Qaros.Library.md#init); every [`Session`](Quaternar.Qaros.Session.md) comes from [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) or [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) on it. |
| [`RuntimeOptions`](Quaternar.Qaros.RuntimeOptions.md) | class | Arguments of [`Runtime.Create`](Quaternar.Qaros.Runtime.md#create). |
| [`Structure`](Quaternar.Qaros.Structure.md) | class | Base of the extension structures chained onto an options object with its `WithExtension` method. The options are copied into native memory for the duration of the call that takes them. |
| [`StructureType`](Quaternar.Qaros.StructureType.md) | enum | Native structure type tag of a [`Structure`](Quaternar.Qaros.Structure.md). Each extension class sets its own in its constructor. |

## Onboarding and sessions

Join a QAROS session for the first time, rejoin it later, invite devices and start target apps.

| Type | Kind | Description |
|---|---|---|
| [`ActionProgress`](Quaternar.Qaros.ActionProgress.md) | struct |  |
| [`ActionSeverity`](Quaternar.Qaros.ActionSeverity.md) | enum |  |
| [`ForgetOptions`](Quaternar.Qaros.ForgetOptions.md) | class |  |
| [`GpuDeviceIdType`](Quaternar.Qaros.GpuDeviceIdType.md) | enum | How [`GraphicsDeviceId`](Quaternar.Qaros.GraphicsDeviceId.md) names the adapter. |
| [`GraphicsDeviceId`](Quaternar.Qaros.GraphicsDeviceId.md) | class | Extension: selects the graphics adapter the session's Vulkan and D3D11 devices, and with them every render sender's frame textures, are created on. Chain on [`OnboardOptions`](Quaternar.Qaros.OnboardOptions.md) or [`RejoinOptions`](Quaternar.Qaros.RejoinOptions.md); absent, the library picks one. Windows accepts a LUID, other platforms a UUID. The C API currently supports NVIDIA adapters only. |
| [`OnboardCode`](Quaternar.Qaros.OnboardCode.md) | class | Onboarding mode extension behind [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode); prefer that method. |
| [`OnboardHost`](Quaternar.Qaros.OnboardHost.md) | class | The hub to code-onboard against, behind [`OnboardOptions.WithHost`](Quaternar.Qaros.OnboardOptions.md#withhost); prefer that method. |
| [`OnboardingId`](Quaternar.Qaros.OnboardingId.md) | struct | The key of an identity slot, minted by the first [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard). It is the one thing an application persists: hand it back to [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) on every start and to [`Runtime.Forget`](Quaternar.Qaros.Runtime.md#forget) to unpair. |
| [`OnboardingInvite`](Quaternar.Qaros.OnboardingInvite.md) | class | A one-time onboarding invite minted by the hub: everything another instance needs to onboard with [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite), or a code to hand out by [`OnboardingInvite.GetCode`](Quaternar.Qaros.OnboardingInvite.md#getcode). |
| [`OnboardingMethod`](Quaternar.Qaros.OnboardingMethod.md) | enum | Which onboarding path an onboarding invite carries. More may be added, so do not treat "not [`OnboardingMethod.Pake`](Quaternar.Qaros.OnboardingMethod.md#pake)" as [`OnboardingMethod.Password`](Quaternar.Qaros.OnboardingMethod.md#password). |
| [`OnboardInvite`](Quaternar.Qaros.OnboardInvite.md) | class | Onboarding mode extension behind [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite); prefer that method. The invite is borrowed, not owned. |
| [`OnboardOptions`](Quaternar.Qaros.OnboardOptions.md) | class | Options of [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard). Choose exactly one mode: [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode) or [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite); the last call wins. |
| [`OnboardPeerId`](Quaternar.Qaros.OnboardPeerId.md) | class | Extension: join the session under a caller-chosen peer id instead of a freshly minted one. Chain on [`OnboardOptions`](Quaternar.Qaros.OnboardOptions.md) with [`OnboardOptions.WithPeerId`](Quaternar.Qaros.OnboardOptions.md#withpeerid). The id is persisted with the slot, so a later rejoin keeps it. A zero id is rejected with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported). |
| [`OnboardSession`](Quaternar.Qaros.OnboardSession.md) | struct | What [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) produces. Persist [`OnboardSession.OnboardingId`](Quaternar.Qaros.OnboardSession.md#onboardingid) before anything else; the caller owns and disposes [`OnboardSession.Session`](Quaternar.Qaros.OnboardSession.md#session). |
| [`RejoinOptions`](Quaternar.Qaros.RejoinOptions.md) | class | Options of [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin). |
| [`RequestOnboardingCodesOptions`](Quaternar.Qaros.RequestOnboardingCodesOptions.md) | class | A request for a new set of one-time onboarding codes. Every request gets codes of its own; codes other callers hold and the hub's onboarding screen code stay valid. |
| [`RequestOnboardingInviteOptions`](Quaternar.Qaros.RequestOnboardingInviteOptions.md) | class |  |
| [`Session`](Quaternar.Qaros.Session.md) | class | An active session, the entry point to peers, GUI panels, app volumes and senders. Only [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) and [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) produce one, and there is no getter for a "current" session: keep the instance. |
| [`SessionId`](Quaternar.Qaros.SessionId.md) | struct | A session's 16-byte UUID. Compared by content; `default(SessionId)` equals [`SessionId.Default`](Quaternar.Qaros.SessionId.md#default). |
| [`TargetAppConnectionString`](Quaternar.Qaros.TargetAppConnectionString.md) | class | Builds the connection strings [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp) accepts. A host is a hostname or an IP address; it must not be empty. |

## Peers and hands

The peers in a session, their presentation and the hand-tracking data devices send.

| Type | Kind | Description |
|---|---|---|
| [`AppState`](Quaternar.Qaros.AppState.md) | enum | A peer's lifecycle state. A target app is ready to be rendered for only while it is [`AppState.Running`](Quaternar.Qaros.AppState.md#running). |
| [`DeviceHandsWithJoints`](Quaternar.Qaros.DeviceHandsWithJoints.md) | struct | The newest hands sample a device sent. From [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands) every pose, radius and velocity in it is in the bound app volume's app content space (right-handed, +Y up, -Z forward, app units), like the view poses. |
| [`Handedness`](Quaternar.Qaros.Handedness.md) | enum | Left or right: which hand made a gesture, and the handedness of the axes of a [`GeoAnchorFrame`](Quaternar.Qaros.GeoAnchorFrame.md). |
| [`HandJoint`](Quaternar.Qaros.HandJoint.md) | enum | XR_EXT_hand_tracking joint indices; [`HandJointLocation.JointId`](Quaternar.Qaros.HandJointLocation.md#jointid) carries one. |
| [`HandJointLocation`](Quaternar.Qaros.HandJointLocation.md) | struct | One XR_EXT_hand_tracking joint. |
| [`HandJointLocationFlags`](Quaternar.Qaros.HandJointLocationFlags.md) | enum | Bits of [`HandJointLocation.LocationFlags`](Quaternar.Qaros.HandJointLocation.md#locationflags) and [`HandJointVelocity.Flags`](Quaternar.Qaros.HandJointVelocity.md#flags). |
| [`HandJoints`](Quaternar.Qaros.HandJoints.md) | struct | One hand's tracking state: 26 joint locations and velocities indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md). |
| [`HandJointVelocity`](Quaternar.Qaros.HandJointVelocity.md) | struct | Velocity of one joint. |
| [`PeerControl`](Quaternar.Qaros.PeerControl.md) | class | The peers of a session: every participant, devices and applications alike, this process included. Reached through [`Session.Peers`](Quaternar.Qaros.Session.md#peers). |
| [`PeerId`](Quaternar.Qaros.PeerId.md) | struct | A peer's 16-byte UUID. Compared by content; `default(PeerId)` equals [`PeerId.Default`](Quaternar.Qaros.PeerId.md#default). |
| [`PeerPresentationOptions`](Quaternar.Qaros.PeerPresentationOptions.md) | class | How this peer presents itself to the others. Never a storage key. A property left null keeps its persisted value, or gets a generated default on the first onboard. |
| [`PeerSpec`](Quaternar.Qaros.PeerSpec.md) | class | An immutable snapshot of one peer, taken when it was read or delivered; safe to keep. It does not change: read a new one, or watch [`PeerControl.SubscribeUpdates`](Quaternar.Qaros.PeerControl.md#subscribeupdates). |
| [`PeerSpecUpdatedEventArgs`](Quaternar.Qaros.PeerSpecUpdatedEventArgs.md) | class | One peer update; [`PeerSpecUpdatedEventArgs.Peer`](Quaternar.Qaros.PeerSpecUpdatedEventArgs.md#peer) is a snapshot, safe to keep. |

## App volumes

The 3D volumes an application renders into, their gestures and world anchoring.

| Type | Kind | Description |
|---|---|---|
| [`AppScaleSensitivityMode`](Quaternar.Qaros.AppScaleSensitivityMode.md) | enum |  |
| [`AppVolume`](Quaternar.Qaros.AppVolume.md) | class | An immutable snapshot of one app volume: the box in the room a remote 3D application renders into, and how the application's content sits inside it. |
| [`AppVolumeAxisFlags`](Quaternar.Qaros.AppVolumeAxisFlags.md) | enum | Bitmask of the axes a mapping rule lets a gesture translate along or rotate about; see [`AppVolumeGestureMappingRule.TranslationAxes`](Quaternar.Qaros.AppVolumeGestureMappingRule.md#translationaxes) and [`AppVolumeGestureMappingRule.RotationAxes`](Quaternar.Qaros.AppVolumeGestureMappingRule.md#rotationaxes). |
| [`AppVolumeControl`](Quaternar.Qaros.AppVolumeControl.md) | class | The session's app volumes: the boxes in the room that remote 3D applications render into. Reached through [`Session.AppVolumes`](Quaternar.Qaros.Session.md#appvolumes). |
| [`AppVolumeControllerRayLineStyle`](Quaternar.Qaros.AppVolumeControllerRayLineStyle.md) | enum |  |
| [`AppVolumeControllerRayStyle`](Quaternar.Qaros.AppVolumeControllerRayStyle.md) | struct | How the hand ray is drawn for gestures a mapping rule matches. The colour is RGBA, 0 to 255 per channel; [`AppVolumeControllerRayStyle.Default`](Quaternar.Qaros.AppVolumeControllerRayStyle.md#default) is a solid opaque white line. |
| [`AppVolumeEditingStatus`](Quaternar.Qaros.AppVolumeEditingStatus.md) | struct | Whether a peer holds the volume's editing lock ([`AppVolumeControl.StartEditing`](Quaternar.Qaros.AppVolumeControl.md#startediting)) to move or resize the box itself. A peer manipulating the app with a gesture is not editing it. |
| [`AppVolumeGestureConfiguration`](Quaternar.Qaros.AppVolumeGestureConfiguration.md) | class | A gesture mode's mapping rules, in descending priority order, plus the peers whose gestures may move or scale the app. |
| [`AppVolumeGestureEvent`](Quaternar.Qaros.AppVolumeGestureEvent.md) | struct | One mapped gesture on an app volume, copied out of the native callback. |
| [`AppVolumeGestureEventArgs`](Quaternar.Qaros.AppVolumeGestureEventArgs.md) | class |  |
| [`AppVolumeGestureMappingRule`](Quaternar.Qaros.AppVolumeGestureMappingRule.md) | class | How one gesture kind on an app volume is shaped and what it is applied to. Its priority is its position in [`AppVolumeGestureConfiguration.MappingRules`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#mappingrules), not a field. |
| [`AppVolumeGestureState`](Quaternar.Qaros.AppVolumeGestureState.md) | class | An app volume's gesture modes and who is using them, read with the volume snapshot. |
| [`AppVolumeId`](Quaternar.Qaros.AppVolumeId.md) | struct | An app volume's 16-byte UUID, derived from the volume's common name by [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate). Compared by content; `default(AppVolumeId)` equals [`AppVolumeId.Default`](Quaternar.Qaros.AppVolumeId.md#default). |
| [`AppVolumeLifetimeStatus`](Quaternar.Qaros.AppVolumeLifetimeStatus.md) | enum |  |
| [`AppVolumeOptions`](Quaternar.Qaros.AppVolumeOptions.md) | class | Arguments of [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate). The defaults match `qar_app_volume_init_default()`: identity poses, a 1 m cube, app scale 1, no world anchor and the default gesture configuration. |
| [`AppVolumeSize`](Quaternar.Qaros.AppVolumeSize.md) | struct | An app volume cuboid's extents in metres, centred on the volume's pose: it spans plus and minus half of each extent along app volume space X, Y and Z. Note the constructor order: width (X), length (Z), height (Y). |
| [`AppVolumeUpdatedEventArgs`](Quaternar.Qaros.AppVolumeUpdatedEventArgs.md) | class | One app volume update; [`AppVolumeUpdatedEventArgs.Volume`](Quaternar.Qaros.AppVolumeUpdatedEventArgs.md#volume) is a snapshot, safe to keep. |
| [`AppWorldAnchor`](Quaternar.Qaros.AppWorldAnchor.md) | struct | An app volume's world anchor, if it has one. |
| [`GeoAnchorFrame`](Quaternar.Qaros.GeoAnchorFrame.md) | struct | A local frame fully defined in world coordinates: its origin and unit axes in [`GeoAnchorFrame.WorldReferenceSystem`](Quaternar.Qaros.GeoAnchorFrame.md#worldreferencesystem) (ECEF WGS84, metres, double precision), plus the handedness of those axes. Pins an app volume to the Earth. |
| [`GestureAppTransformMapping`](Quaternar.Qaros.GestureAppTransformMapping.md) | enum | What the library applies a mapped gesture to by itself. Either way the gesture still reaches the volume's gesture subscribers; [`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform) tells whether it was already applied. |
| [`GestureKind`](Quaternar.Qaros.GestureKind.md) | enum | What a hand interaction with an app volume was recognised as. |
| [`GesturePhase`](Quaternar.Qaros.GesturePhase.md) | enum |  |
| [`WorldReferenceSystem`](Quaternar.Qaros.WorldReferenceSystem.md) | enum |  |

## GUI panels

Flat 2D panels placed in the room and the gestures users make on them.

| Type | Kind | Description |
|---|---|---|
| [`GuiPanel`](Quaternar.Qaros.GuiPanel.md) | class | An immutable snapshot of one GUI panel, safe to keep. Get a newer one from [`GuiPanelControl.SubscribeUpdates`](Quaternar.Qaros.GuiPanelControl.md#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken). |
| [`GuiPanelContentType`](Quaternar.Qaros.GuiPanelContentType.md) | enum | What a GUI panel shows. Fixed at creation: [`GuiPanelControl.NavigateToUri`](Quaternar.Qaros.GuiPanelControl.md#navigatetouri) on a stream panel fails with [`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted). |
| [`GuiPanelControl`](Quaternar.Qaros.GuiPanelControl.md) | class | The session's GUI panels: shared 2D surfaces in the room showing a web page or a video stream. Reached through [`Session.GuiPanels`](Quaternar.Qaros.Session.md#guipanels). |
| [`GuiPanelGestureEvent`](Quaternar.Qaros.GuiPanelGestureEvent.md) | struct | One recognised gesture on a GUI panel, copied out of the native callback. |
| [`GuiPanelGestureEventArgs`](Quaternar.Qaros.GuiPanelGestureEventArgs.md) | class |  |
| [`GuiPanelGestureKind`](Quaternar.Qaros.GuiPanelGestureKind.md) | enum | What a contact on a GUI panel was recognised as. |
| [`GuiPanelId`](Quaternar.Qaros.GuiPanelId.md) | struct | A GUI panel's 16-byte UUID, derived from the panel's common name by [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate). Compared by content; `default(GuiPanelId)` equals [`GuiPanelId.Default`](Quaternar.Qaros.GuiPanelId.md#default). |
| [`GuiPanelOptions`](Quaternar.Qaros.GuiPanelOptions.md) | class | Arguments of [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate). |
| [`GuiPanelPoint`](Quaternar.Qaros.GuiPanelPoint.md) | struct | A position on a GUI panel in panel-content-space pixels: origin top-left as the viewer sees it, +X right, +Y down. |
| [`GuiPanelSize`](Quaternar.Qaros.GuiPanelSize.md) | struct | A GUI panel's surface size in metres and the zoom of the content drawn on it. The surface spans panel-space X from -Width/2 to +Width/2 and hangs down from the pose to -Height. |
| [`GuiPanelState`](Quaternar.Qaros.GuiPanelState.md) | enum |  |
| [`GuiPanelUpdatedEventArgs`](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md) | class | One GUI panel update; [`GuiPanelUpdatedEventArgs.Panel`](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md#panel) is a snapshot, safe to keep. |

## Rendering

Render senders stream frames for a target peer: frame begin/show, views, stream parameters per graphics API.

| Type | Kind | Description |
|---|---|---|
| [`DXGIKeyedMutexSync`](Quaternar.Qaros.DXGIKeyedMutexSync.md) | struct | The keyed mutex of one shared D3D11 texture and the keys for its AcquireSync and ReleaseSync. Needed only when [`StreamParamsD3D11.AcquireKeyedMutexSync`](Quaternar.Qaros.StreamParamsD3D11.md#acquirekeyedmutexsync) is false and the application drives the key chain itself. |
| [`Fov`](Quaternar.Qaros.Fov.md) | struct | Field of view of one render view, in view space: angles in radians from the view's -Z axis, with OpenXR signs - left and down negative, right and up positive. |
| [`FrameLayout`](Quaternar.Qaros.FrameLayout.md) | enum | How the views of one video frame are packed into textures. |
| [`GraphicsAPI`](Quaternar.Qaros.GraphicsAPI.md) | enum | The graphics backend a sender's frame textures live on. Every GPU value requires exactly the matching stream-parameter extension on the options; [`GraphicsAPI.Cpu`](Quaternar.Qaros.GraphicsAPI.md#cpu) takes none. |
| [`ImageSize`](Quaternar.Qaros.ImageSize.md) | struct | An image size in pixels. |
| [`NearFar`](Quaternar.Qaros.NearFar.md) | struct | Near and far clip distances along a view's -Z, in the units of the poses the view was rendered with: app units for a render sender. A near plane greater than the far plane declares reverse-Z. |
| [`OpenGlContextCallbacks`](Quaternar.Qaros.OpenGlContextCallbacks.md) | struct | Raw C function pointers overriding how the library binds an OpenGL context; see `QarOpenGlContextCallbacks` in types.h. Every one is optional and called on the library's internal graphics thread; zero uses the library's own platform implementation for that step. |
| [`PixelFormat`](Quaternar.Qaros.PixelFormat.md) | enum |  |
| [`RenderFrameInfo`](Quaternar.Qaros.RenderFrameInfo.md) | class | Per-frame information from a successful begin-frame. Everything it hands out is in the bound app volume's app content space. |
| [`RenderFrameShow`](Quaternar.Qaros.RenderFrameShow.md) | class | Arguments of [`RenderSender.ShowFrame`](Quaternar.Qaros.RenderSender.md#showframe): the projection metadata the frame was rendered with. The defaults match `qar_render_frame_show_default()`: near 0.1, far 100, depth scale 1. |
| [`RenderFrameShowViewOverrides`](Quaternar.Qaros.RenderFrameShowViewOverrides.md) | class | Extension of [`RenderFrameShow`](Quaternar.Qaros.RenderFrameShow.md): the pose or field of view a view was really rendered with, when it differs from what [`RenderSender.BeginFrame`](Quaternar.Qaros.RenderSender.md#beginframe) handed out. [`RenderFrameShow.WithViewOverrides`](Quaternar.Qaros.RenderFrameShow.md#withviewoverrides) chains one. |
| [`RenderFrameView`](Quaternar.Qaros.RenderFrameView.md) | struct | One view declared in [`RenderSenderOptions.FrameViews`](Quaternar.Qaros.RenderSenderOptions.md#frameviews). |
| [`RenderFrameViewOverride`](Quaternar.Qaros.RenderFrameViewOverride.md) | struct | The pose and/or field of view one view was really rendered with, when it deviates from what [`RenderSender.BeginFrame`](Quaternar.Qaros.RenderSender.md#beginframe) handed out. Only the parts whose flag is set are used. |
| [`RenderSender`](Quaternar.Qaros.RenderSender.md) | class | Streams the application's rendered frames to one target peer, displayed in the bound app volume. Create one per target with [`RenderSenderControl.Create`](Quaternar.Qaros.RenderSenderControl.md#create). |
| [`RenderSenderControl`](Quaternar.Qaros.RenderSenderControl.md) | class | Creates render senders and delivers render stream requests for a session; reached through [`Session.RenderSenders`](Quaternar.Qaros.Session.md#rendersenders). |
| [`RenderSenderOptions`](Quaternar.Qaros.RenderSenderOptions.md) | class | Arguments of a render sender: one stream rendered for one target peer, displayed in one app volume. The defaults match `qar_render_sender_init_default()`. |
| [`RenderStreamRequest`](Quaternar.Qaros.RenderStreamRequest.md) | struct |  |
| [`RenderStreamRequestEventArgs`](Quaternar.Qaros.RenderStreamRequestEventArgs.md) | class |  |
| [`StreamId`](Quaternar.Qaros.StreamId.md) | struct | A logical video stream's 16-byte UUID, generated by the library for each [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md) ([`PrimarylessVideoSender.GetStreamId`](Quaternar.Qaros.PrimarylessVideoSender.md#getstreamid)). Compared by content; `default(StreamId)` equals [`StreamId.Default`](Quaternar.Qaros.StreamId.md#default). |
| [`StreamParamsD3D11`](Quaternar.Qaros.StreamParamsD3D11.md) | class | Extension: adopt the application's D3D11.1 device, so the sender's frame textures are shared D3D11 textures it renders into directly, with no CPU copy. Windows only. |
| [`StreamParamsOpenGl`](Quaternar.Qaros.StreamParamsOpenGl.md) | class | Base of the three OpenGL stream-parameter extensions. Pick the one matching the context the application owns; requires a library built with QAR_ENABLE_OPENGL. |
| [`StreamParamsOpenGlEgl`](Quaternar.Qaros.StreamParamsOpenGlEgl.md) | class | Extension: adopt an existing EGL context. |
| [`StreamParamsOpenGlGlx`](Quaternar.Qaros.StreamParamsOpenGlGlx.md) | class | Extension: adopt an existing GLX context. |
| [`StreamParamsOpenGlWgl`](Quaternar.Qaros.StreamParamsOpenGlWgl.md) | class | Extension: adopt an existing WGL context. |
| [`StreamParamsVulkan`](Quaternar.Qaros.StreamParamsVulkan.md) | class | Extension: adopt an existing Vulkan device. Requires a library built with QAR_ENABLE_VULKAN. |
| [`SyncFrameD3D11`](Quaternar.Qaros.SyncFrameD3D11.md) | struct |  |
| [`TextureSize`](Quaternar.Qaros.TextureSize.md) | struct | One texture of a frame layout; width and height in pixels. |

## Video senders and frames

Video streams without a primary render target, and the frame and texture types every sender hands out.

| Type | Kind | Description |
|---|---|---|
| [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md) | class | Application-owned video stream, such as GUI panel content, that exists before any receiver connects and is delivered to every receiver that requests its stream id. |
| [`PrimarylessVideoSenderControl`](Quaternar.Qaros.PrimarylessVideoSenderControl.md) | class | Creates [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md)s for a session; reached through [`Session.PrimarylessVideoSenders`](Quaternar.Qaros.Session.md#primarylessvideosenders). |
| [`PrimarylessVideoSenderOptions`](Quaternar.Qaros.PrimarylessVideoSenderOptions.md) | class | Arguments of [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md): application-owned video, such as GUI panel content, under a library-generated [`StreamId`](Quaternar.Qaros.StreamId.md). Unlike a render sender it answers no receiver request and may exist before any receiver connects. |
| [`VideoFrameCpu`](Quaternar.Qaros.VideoFrameCpu.md) | struct | A snapshot of a sender's CPU frame: its views and textures, in layout order. The pixel memory stays the sender's. |
| [`VideoFrameD3D11`](Quaternar.Qaros.VideoFrameD3D11.md) | struct | A snapshot of a sender's D3D11 frame: its views, its shared textures and their keyed-mutex synchronization. The textures stay the sender's; render into them only inside a frame cycle. A render sender's frame is readable from creation, before the first begin-frame. |
| [`VideoFrameLayout`](Quaternar.Qaros.VideoFrameLayout.md) | class | The views of a video frame and the textures backing them. |
| [`VideoFrameOpenGl`](Quaternar.Qaros.VideoFrameOpenGl.md) | struct | A snapshot of a sender's OpenGL frame: its views and textures. The textures stay the sender's; render into them only inside a frame cycle. A render sender's frame is readable from creation, before the first begin-frame. |
| [`VideoFrameView`](Quaternar.Qaros.VideoFrameView.md) | struct | Where one view of a video frame lives: a pixel rectangle in one texture (and array layer). The end coordinates are exclusive and may be smaller than the start ones, which flips the view. |
| [`VideoFrameViewEye`](Quaternar.Qaros.VideoFrameViewEye.md) | enum |  |
| [`VideoFrameViewType`](Quaternar.Qaros.VideoFrameViewType.md) | enum |  |
| [`VideoFrameVulkan`](Quaternar.Qaros.VideoFrameVulkan.md) | class | Access to a sender-owned Vulkan frame. The library owns the native frame; this object is valid until the next GetVulkanFrame call on the same sender or the sender's disposal. Before showing a frame, write the layouts and queue families the images were left in and the synchronization objects with [`VideoFrameVulkan.SetTextureState`](Quaternar.Qaros.VideoFrameVulkan.md#settexturestate) and [`VideoFrameVulkan.SetSynchronization`](Quaternar.Qaros.VideoFrameVulkan.md#setsynchronization). |
| [`VideoTextureCpu`](Quaternar.Qaros.VideoTextureCpu.md) | struct | One sender-owned CPU texture. The pixel memory is not copied: [`VideoTextureCpu.TextureData`](Quaternar.Qaros.VideoTextureCpu.md#texturedata) points into the sender. |
| [`VideoTextureD3D11`](Quaternar.Qaros.VideoTextureD3D11.md) | struct |  |
| [`VideoTextureOpenGl`](Quaternar.Qaros.VideoTextureOpenGl.md) | struct |  |
| [`VideoTextureVulkan`](Quaternar.Qaros.VideoTextureVulkan.md) | struct | One sender-owned Vulkan image and the state it is in. |

## Results and errors

Every call returns a `Result<T>` instead of throwing; these types carry the outcome.

| Type | Kind | Description |
|---|---|---|
| [`NativeCallException`](Quaternar.Qaros.NativeCallException.md) | class | A native failure turned into an exception on request, by [`Result<T>.ValueOrThrow`](Quaternar.Qaros.Result-1.md#valueorthrow) or `EnsureSuccess`. [`NativeCallException.Result`](Quaternar.Qaros.NativeCallException.md#result) carries the status code. |
| [`Result<T>`](Quaternar.Qaros.Result-1.md) | struct | A value or a failure. A native failure is returned, never thrown: check [`Result<T>.IsSuccess`](Quaternar.Qaros.Result-1.md#issuccess), or opt into an exception with [`Result<T>.ValueOrThrow`](Quaternar.Qaros.Result-1.md#valueorthrow) or [`Result<T>.EnsureSuccess`](Quaternar.Qaros.Result-1.md#ensuresuccess). |
| [`ResultInfo`](Quaternar.Qaros.ResultInfo.md) | struct | The outcome of a native call: a [`ResultInfo.StatusCode`](Quaternar.Qaros.ResultInfo.md#statuscode) and a diagnostic message. |
| [`StatusCode`](Quaternar.Qaros.StatusCode.md) | enum | Status codes the C API exposes (QarStatusCode), with the same values. Any internal code outside this set reaches callers as [`StatusCode.Unclassified`](Quaternar.Qaros.StatusCode.md#unclassified). |
| [`Unit`](Quaternar.Qaros.Unit.md) | struct | The value of a [`Result<T>`](Quaternar.Qaros.Result-1.md) whose operation returns nothing. |

## Math and time

Value types shared by every area.

| Type | Kind | Description |
|---|---|---|
| [`Pose`](Quaternar.Qaros.Mathematics.Pose.md) | struct | Position plus orientation (x, y, z, w). A pose carries no space of its own: every member that holds one names the space it is in. The bindings convert no coordinates; each value is exactly what the native library produced. |
| [`Quaternion`](Quaternar.Qaros.Mathematics.Quaternion.md) | struct | Rotation quaternion, Hamilton convention, stored and constructed in x, y, z, w order (w last). `default(Quaternion)` is all zeros, not a rotation; use [`Quaternion.Identity`](Quaternar.Qaros.Mathematics.Quaternion.md#identity). |
| [`TimePoint`](Quaternar.Qaros.TimePoint.md) | struct | Wall-clock instant: nanoseconds since the Unix epoch (UTC). Instants stamped on different hosts are only as comparable as those hosts' clocks are synchronized. Converts to a UTC [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime) implicitly or with [`TimePoint.ToDateTime`](Quaternar.Qaros.TimePoint.md#todatetime), truncated to its 100 ns tick. |
| [`Vector3`](Quaternar.Qaros.Mathematics.Vector3.md) | struct | A 3D vector. It carries no space of its own; the member holding it names the space (see [`Pose`](Quaternar.Qaros.Mathematics.Pose.md)). |
| [`Vector3d`](Quaternar.Qaros.Mathematics.Vector3d.md) | struct | Double-precision 3D vector, used for Earth-fixed world coordinates in [`GeoAnchorFrame`](Quaternar.Qaros.GeoAnchorFrame.md). |
