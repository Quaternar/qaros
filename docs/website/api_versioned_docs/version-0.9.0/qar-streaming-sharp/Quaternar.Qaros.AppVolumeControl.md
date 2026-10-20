---
title: "AppVolumeControl class"
sidebar_label: "AppVolumeControl"
description: "The session's app volumes: the boxes in the room that remote 3D applications render into. Reached through Session.AppVolumes."
---

# AppVolumeControl class

The session's app volumes: the boxes in the room that remote 3D applications render into.
Reached through [`Session.AppVolumes`](Quaternar.Qaros.Session.md#appvolumes).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [AppVolumeControl.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/AppVolumeControl.cs#L38)

```csharp
public sealed class AppVolumeControl
```

## Remarks

App volume state is shared session state and eventually consistent. A change method
returns success once it has published the change; the change reaches the shared state,
and every peer's snapshots, later, and may still be refused there. A read right after a
write can return the old value: observe the result through
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken)
instead of reading back once.

A change to a volume this session does not know yet (its id was never seen, or its
creation has not arrived) fails rather than waiting for it.

Spaces: [`AppVolumeControl.ChangePose`](#changepose) places the cuboid centre in room space, in metres.
[`AppVolumeControl.ChangeAppPose`](#changeapppose) and [`AppVolumeControl.ChangeAppScale`](#changeappscale) place the application's own
world (app content space) inside the box. See [`AppVolume`](Quaternar.Qaros.AppVolume.md).

Every method throws [`ObjectDisposedException`](https://learn.microsoft.com/dotnet/api/system.objectdisposedexception) once the session is disposed.

## Methods

| Member | Description |
|---|---|
| [GetOrCreate(AppVolumeOptions)](#getorcreate) | Returns the volume identified by [`AppVolumeOptions.CommonName`](Quaternar.Qaros.AppVolumeOptions.md#commonname), creating it from `options` if it does not exist yet. |
| [ChangeDisplayName(AppVolumeId, string)](#changedisplayname) | Changes the label users see. Any UTF-8 text, emoji included; the common name, and with it the id, never changes. |
| [ChangeSize(AppVolumeId, AppVolumeSize)](#changesize) | Resizes the box around its centre, in metres. A fast-path change: [`AppVolumeControl.GetLatestSize`](#getlatestsize) returns it at once. |
| [Close(AppVolumeId)](#close) | Closes the volume for every peer: its [`AppVolume.LifetimeStatus`](Quaternar.Qaros.AppVolume.md#lifetimestatus) becomes [`AppVolumeLifetimeStatus.Closed`](Quaternar.Qaros.AppVolumeLifetimeStatus.md#closed). |
| [UpdateUsedByPeers(AppVolumeId, PeerId\[\], PeerId\[\])](#updateusedbypeers) | Adds and removes peers in [`AppVolume.UsedByPeers`](Quaternar.Qaros.AppVolume.md#usedbypeers). Null arrays mean none. |
| [StartEditing(AppVolumeId)](#startediting) | Marks the volume's box as being moved or resized by this session's own peer, which every peer reads in [`AppVolume.EditingStatus`](Quaternar.Qaros.AppVolume.md#editingstatus), for example to show who is moving it. |
| [StopEditing(AppVolumeId)](#stopediting) | Clears the editing status, whichever peer set it. |
| [ChangeAppPose(AppVolumeId, Pose)](#changeapppose) | Sets the app pose: the application's origin in app volume space, in volume metres. |
| [ChangeAppScale(AppVolumeId, float)](#changeappscale) | Sets the app scale: room metres per app metre, so 0.01 shows a 100 m model as 1 m. Must be greater than 0, otherwise the call fails with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported). |
| [ChangeAppWorldAnchor(AppVolumeId, GeoAnchorFrame)](#changeappworldanchor) | Pins the volume to an Earth-fixed frame (ECEF WGS84, metres). Cleared with [`AppVolumeControl.ClearAppWorldAnchor`](#clearappworldanchor). |
| [ClearAppWorldAnchor(AppVolumeId)](#clearappworldanchor) |  |
| [ChangePose(AppVolumeId, Pose)](#changepose) | Moves the volume. `pose` is the cuboid centre in room space. |
| [GetLatestAppPose(AppVolumeId)](#getlatestapppose) | Latest locally known app pose, in app volume space. |
| [GetLatestAppScale(AppVolumeId)](#getlatestappscale) | Latest locally known app scale; see [`AppVolumeControl.GetLatestAppPose`](#getlatestapppose). |
| [GetLatestPose(AppVolumeId)](#getlatestpose) | Latest locally known volume pose (the cuboid centre), in room space; see [`AppVolumeControl.GetLatestAppPose`](#getlatestapppose). |
| [GetLatestSize(AppVolumeId)](#getlatestsize) | Latest locally known size; see [`AppVolumeControl.GetLatestAppPose`](#getlatestapppose). |
| [ChangeGestureConfiguration(AppVolumeId, AppVolumeGestureConfiguration)](#changegestureconfiguration) | Replaces the rules of gesture mode [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode) and the allowed peers ([`AppVolumeGestureConfiguration.AllowedPeers`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#allowedpeers)). Other modes are kept. Fails with [`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid) for a rule that could never work; the volume then keeps its previous configuration. |
| [ChangeGestureMode(AppVolumeId, uint, AppVolumeGestureConfiguration)](#changegesturemode) | Defines gesture mode `modeId`, or replaces the mode with that id. Other modes and the allowed peers are kept; [`AppVolumeGestureConfiguration.AllowedPeers`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#allowedpeers) must be empty. |
| [RemoveGestureMode(AppVolumeId, uint)](#removegesturemode) | Removes a gesture mode; peers in it fall back to [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode). The default mode cannot be removed ([`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid)); an unknown one fails with [`StatusCode.AppVolumeGestureModeUnknown`](Quaternar.Qaros.StatusCode.md#appvolumegesturemodeunknown). |
| [SelectGestureMode(AppVolumeId, PeerId?, uint)](#selectgesturemode) | Switches `peer` to a gesture mode, or, with a null peer, makes it the default every peer without its own mode follows. A running gesture keeps the mode it began in. |
| [ClearPeerGestureMode(AppVolumeId, PeerId)](#clearpeergesturemode) | Drops one peer's own gesture mode, so it follows the default again. |
| [ClearAllPeerGestureModes(AppVolumeId)](#clearallpeergesturemodes) | Drops every peer's own gesture mode, so all follow the default. |
| [SubscribeAllGestureUpdates(AppVolumeId, EventHandler&lt;AppVolumeGestureEventArgs&gt;, CancellationToken)](#subscribeallgestureupdates) | Subscribe to every gesture kind on one app volume through one handler. Prefer this when taking more than one kind: everything arrives in the order the producer published it. Disposing the result stops delivery. |
| [GetAll()](#getall) | Snapshot of every app volume the session's shared storage holds. Blocks for a network round trip of about 2 s on every call. |
| [SubscribeUpdates()](#subscribeupdates) | Starts delivering app volume updates through the [`AppVolumeControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(CancellationToken)](#subscribeupdates-cancellationtoken) | Starts delivering app volume updates through the [`AppVolumeControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(EventHandler&lt;AppVolumeUpdatedEventArgs&gt;)](#subscribeupdates-eventhandler-appvolumeupdatedeventargs) | Calls `handler` once for every app volume already stored, then again for every change to any volume, until the result is disposed. |
| [SubscribeUpdates(EventHandler&lt;AppVolumeUpdatedEventArgs&gt;, CancellationToken)](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken) | Calls `handler` once for every app volume already stored, then again for every change to any volume, until the result is disposed. |
| [WatchUpdatesAsync(CancellationToken)](#watchupdatesasync) | The snapshots of [`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken) as an async stream: every stored volume first, then every change. |
| [SubscribeGestureUpdates(AppVolumeId, GestureKind, Action&lt;AppVolumeGestureEvent&gt;)](#subscribegestureupdates-appvolumeid-gesturekind-action-appvolumegestureevent) | Calls `handler` for every gesture of one kind that a peer performs against one app volume, until the result is disposed. |
| [SubscribeGestureUpdates(AppVolumeId, GestureKind, Action&lt;AppVolumeGestureEvent&gt;, CancellationToken)](#subscribegestureupdates-appvolumeid-gesturekind-action-appvolumegestureevent-cancellationtoken) | Calls `handler` for every gesture of one kind that a peer performs against one app volume, until the result is disposed. |
| [SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler&lt;AppVolumeGestureEventArgs&gt;)](#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs) | Calls `handler` for every gesture of one kind that a peer performs against one app volume, until the result is disposed. |
| [SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler&lt;AppVolumeGestureEventArgs&gt;, CancellationToken)](#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs-cancellationtoken) | Calls `handler` for every gesture of one kind that a peer performs against one app volume, until the result is disposed. |
| [WatchGestureUpdatesAsync(AppVolumeId, GestureKind, CancellationToken)](#watchgestureupdatesasync) | The gestures of [`AppVolumeControl.SubscribeGestureUpdates`](#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs-cancellationtoken) as an async stream. |

### GetOrCreate {#getorcreate}

Returns the volume identified by [`AppVolumeOptions.CommonName`](Quaternar.Qaros.AppVolumeOptions.md#commonname),
creating it from `options` if it does not exist yet.

```csharp
public Result<AppVolumeId> GetOrCreate(AppVolumeOptions options)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [AppVolumeOptions](Quaternar.Qaros.AppVolumeOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[AppVolumeId](Quaternar.Qaros.AppVolumeId.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` is null. |
| [ArgumentException](https://learn.microsoft.com/dotnet/api/system.argumentexception) | [`AppVolumeOptions.CommonName`](Quaternar.Qaros.AppVolumeOptions.md#commonname) is null. |

**Remarks**

The id is derived from the common name alone, so the same common name gives the
same volume in every session, from every peer and after a restart: call this on every
start instead of persisting the id. The common name is the identity key and must be
ASCII; put user-facing text, which may be any UTF-8, in
[`AppVolumeOptions.DisplayName`](Quaternar.Qaros.AppVolumeOptions.md#displayname). Set a non-empty DisplayName when creating or reopening a volume; CommonName does not supply a default label.

When an open volume of that name exists, the rest of `options` is
ignored and the existing volume keeps its pose, size, app transform and gesture
configuration; change them with the `Change*` methods. A closed volume
([`AppVolumeControl.Close`](#close)) is reopened with `options`.

A gesture configuration whose rule could never work fails with
[`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid).

### ChangeDisplayName {#changedisplayname}

Changes the label users see. Any UTF-8 text, emoji included; the common name, and with
it the id, never changes.

```csharp
public Result<Unit> ChangeDisplayName(AppVolumeId id, string displayName)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `displayName` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### ChangeSize {#changesize}

Resizes the box around its centre, in metres. A fast-path change:
[`AppVolumeControl.GetLatestSize`](#getlatestsize) returns it at once.

```csharp
public Result<Unit> ChangeSize(AppVolumeId id, AppVolumeSize size)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `size` | [AppVolumeSize](Quaternar.Qaros.AppVolumeSize.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### Close {#close}

Closes the volume for every peer: its [`AppVolume.LifetimeStatus`](Quaternar.Qaros.AppVolume.md#lifetimestatus) becomes
[`AppVolumeLifetimeStatus.Closed`](Quaternar.Qaros.AppVolumeLifetimeStatus.md#closed).

```csharp
public Result<Unit> Close(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

The volume is not forgotten: [`AppVolumeControl.GetOrCreate`](#getorcreate) with the same common name
reopens it with new options. A render sender bound to a closed volume fails to begin
frames with [`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved).

### UpdateUsedByPeers {#updateusedbypeers}

Adds and removes peers in [`AppVolume.UsedByPeers`](Quaternar.Qaros.AppVolume.md#usedbypeers). Null arrays mean none.

```csharp
public Result<Unit> UpdateUsedByPeers(AppVolumeId id, PeerId[] additions, PeerId[] removals)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `additions` | [PeerId](Quaternar.Qaros.PeerId.md)\[\] |  |
| `removals` | [PeerId](Quaternar.Qaros.PeerId.md)\[\] |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Each peer is a separate change, additions first. The first failure stops the call and
the changes before it stay published.

### StartEditing {#startediting}

Marks the volume's box as being moved or resized by this session's own peer, which
every peer reads in [`AppVolume.EditingStatus`](Quaternar.Qaros.AppVolume.md#editingstatus), for example to show who is
moving it.

```csharp
public Result<Unit> StartEditing(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

It replaces whatever editor was set before. A peer manipulating the app with a gesture
is not an editor; see [`AppVolumeGestureState.Holder`](Quaternar.Qaros.AppVolumeGestureState.md#holder).

### StopEditing {#stopediting}

Clears the editing status, whichever peer set it.

```csharp
public Result<Unit> StopEditing(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### ChangeAppPose {#changeapppose}

Sets the app pose: the application's origin in app volume space, in volume metres.

```csharp
public Result<Unit> ChangeAppPose(AppVolumeId id, Pose appPose)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `appPose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Pans the content inside the box without the renderer changing anything: the render
sender's view poses follow. Not scaled by the app scale. A fast-path change:
[`AppVolumeControl.GetLatestAppPose`](#getlatestapppose) returns it at once. Gestures mapped to the app
transform change it too.

### ChangeAppScale {#changeappscale}

Sets the app scale: room metres per app metre, so 0.01 shows a 100 m model as 1 m.
Must be greater than 0, otherwise the call fails with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported).

```csharp
public Result<Unit> ChangeAppScale(AppVolumeId id, float appScale)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `appScale` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Zooms the content inside the box without the renderer changing anything. A fast-path
change: [`AppVolumeControl.GetLatestAppScale`](#getlatestappscale) returns it at once. Gestures mapped to the app
transform change it too.

### ChangeAppWorldAnchor {#changeappworldanchor}

Pins the volume to an Earth-fixed frame (ECEF WGS84, metres). Cleared with
[`AppVolumeControl.ClearAppWorldAnchor`](#clearappworldanchor).

```csharp
public Result<Unit> ChangeAppWorldAnchor(AppVolumeId id, GeoAnchorFrame anchor)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `anchor` | [GeoAnchorFrame](Quaternar.Qaros.GeoAnchorFrame.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### ClearAppWorldAnchor {#clearappworldanchor}

```csharp
public Result<Unit> ClearAppWorldAnchor(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### ChangePose {#changepose}

Moves the volume. `pose` is the cuboid centre in room space.

```csharp
public Result<Unit> ChangePose(AppVolumeId id, Pose pose)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `pose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

The centre, not a corner or a point on the floor. A fast-path change:
[`AppVolumeControl.GetLatestPose`](#getlatestpose) returns it at once.

### GetLatestAppPose {#getlatestapppose}

Latest locally known app pose, in app volume space.

```csharp
public Result<Pose> GetLatestAppPose(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Pose](Quaternar.Qaros.Mathematics.Pose.md)&gt;

**Remarks**

The `GetLatest*` reads answer from this session's local view without a network
round trip: the latest fast-path value from this or any other peer, else the stored
state. They can be newer than the last [`AppVolume`](Quaternar.Qaros.AppVolume.md) snapshot delivered, and
fail while this session does not know the volume yet.

### GetLatestAppScale {#getlatestappscale}

Latest locally known app scale; see [`AppVolumeControl.GetLatestAppPose`](#getlatestapppose).

```csharp
public Result<float> GetLatestAppScale(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[float](https://learn.microsoft.com/dotnet/api/system.single)&gt;

### GetLatestPose {#getlatestpose}

Latest locally known volume pose (the cuboid centre), in room space; see
[`AppVolumeControl.GetLatestAppPose`](#getlatestapppose).

```csharp
public Result<Pose> GetLatestPose(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Pose](Quaternar.Qaros.Mathematics.Pose.md)&gt;

### GetLatestSize {#getlatestsize}

Latest locally known size; see [`AppVolumeControl.GetLatestAppPose`](#getlatestapppose).

```csharp
public Result<AppVolumeSize> GetLatestSize(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[AppVolumeSize](Quaternar.Qaros.AppVolumeSize.md)&gt;

### ChangeGestureConfiguration {#changegestureconfiguration}

Replaces the rules of gesture mode [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode)
and the allowed peers ([`AppVolumeGestureConfiguration.AllowedPeers`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#allowedpeers)).
Other modes are kept. Fails with
[`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid) for a rule
that could never work; the volume then keeps its previous configuration.

```csharp
public Result<Unit> ChangeGestureConfiguration(AppVolumeId id, AppVolumeGestureConfiguration configuration)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `configuration` | [AppVolumeGestureConfiguration](Quaternar.Qaros.AppVolumeGestureConfiguration.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `configuration` is null. |

**Remarks**

Rule order is priority: an earlier rule can suppress a later one when both are active
for the same volume and source peer. The call reads the volume back from the shared
storage before writing, so it takes about as long as [`AppVolumeControl.GetAll`](#getall).

### ChangeGestureMode {#changegesturemode}

Defines gesture mode `modeId`, or replaces the mode with
that id. Other modes and the allowed peers are kept;
[`AppVolumeGestureConfiguration.AllowedPeers`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#allowedpeers) must be empty.

```csharp
public Result<Unit> ChangeGestureMode(AppVolumeId id, uint modeId, AppVolumeGestureConfiguration configuration)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `modeId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `configuration` | [AppVolumeGestureConfiguration](Quaternar.Qaros.AppVolumeGestureConfiguration.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `configuration` is null. |

**Remarks**

A mode is a set of mapping rules peers are switched to with
[`AppVolumeControl.SelectGestureMode`](#selectgesturemode), so a user's gestures and rays change with one small
call. Any id other than [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode) is the
application's to choose. Non-empty allowed peers fail with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported), a rule that could never work with
[`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid).

### RemoveGestureMode {#removegesturemode}

Removes a gesture mode; peers in it fall back to
[`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode). The default mode cannot be
removed ([`StatusCode.AppVolumeGestureConfigurationInvalid`](Quaternar.Qaros.StatusCode.md#appvolumegestureconfigurationinvalid)); an
unknown one fails with [`StatusCode.AppVolumeGestureModeUnknown`](Quaternar.Qaros.StatusCode.md#appvolumegesturemodeunknown).

```csharp
public Result<Unit> RemoveGestureMode(AppVolumeId id, uint modeId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `modeId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

A gesture already running in the removed mode finishes in it.

### SelectGestureMode {#selectgesturemode}

Switches `peer` to a gesture mode, or, with a null peer,
makes it the default every peer without its own mode follows. A running
gesture keeps the mode it began in.

```csharp
public Result<Unit> SelectGestureMode(AppVolumeId id, PeerId? peer, uint modeId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `peer` | [PeerId](Quaternar.Qaros.PeerId.md)? |  |
| `modeId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Changing the default leaves peers switched individually in their own mode; clear them
with [`AppVolumeControl.ClearPeerGestureMode`](#clearpeergesturemode) or [`AppVolumeControl.ClearAllPeerGestureModes`](#clearallpeergesturemodes). A
mode the volume does not define fails with
[`StatusCode.AppVolumeGestureModeUnknown`](Quaternar.Qaros.StatusCode.md#appvolumegesturemodeunknown).

### ClearPeerGestureMode {#clearpeergesturemode}

Drops one peer's own gesture mode, so it follows the default again.

```csharp
public Result<Unit> ClearPeerGestureMode(AppVolumeId id, PeerId peer)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `peer` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### ClearAllPeerGestureModes {#clearallpeergesturemodes}

Drops every peer's own gesture mode, so all follow the default.

```csharp
public Result<Unit> ClearAllPeerGestureModes(AppVolumeId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### SubscribeAllGestureUpdates {#subscribeallgestureupdates}

Subscribe to every gesture kind on one app volume through one handler.
Prefer this when taking more than one kind: everything arrives in the
order the producer published it. Disposing the result stops delivery.

```csharp
public Result<IDisposable> SubscribeAllGestureUpdates(AppVolumeId id, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeGestureEventArgs](Quaternar.Qaros.AppVolumeGestureEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Dispatch on [`AppVolumeGestureEvent.GestureKind`](Quaternar.Qaros.AppVolumeGestureEvent.md#gesturekind). The volume's mapping
rules still decide what is published at all; only the kind filter is gone. Everything
else - mapping, several producers, threading and disposal - is as for
[`AppVolumeControl.SubscribeGestureUpdates`](#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs-cancellationtoken).

### GetAll {#getall}

Snapshot of every app volume the session's shared storage holds. Blocks for a network
round trip of about 2 s on every call.

```csharp
public Result<AppVolume[]> GetAll()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[AppVolume](Quaternar.Qaros.AppVolume.md)\[\]&gt;

**Remarks**

Use it for a one-off listing. To track volumes, subscribe with
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken),
which delivers the same snapshot first; for one volume's live transform, use the
`GetLatest*` reads.

### SubscribeUpdates() {#subscribeupdates}

Starts delivering app volume updates through the [`AppVolumeControl.Updated`](#updated) event until the
result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken).

### SubscribeUpdates(CancellationToken) {#subscribeupdates-cancellationtoken}

Starts delivering app volume updates through the [`AppVolumeControl.Updated`](#updated) event until the
result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken).

### SubscribeUpdates(EventHandler&lt;AppVolumeUpdatedEventArgs&gt;) {#subscribeupdates-eventhandler-appvolumeupdatedeventargs}

Calls `handler` once for every app volume already stored, then again
for every change to any volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<AppVolumeUpdatedEventArgs> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeUpdatedEventArgs](Quaternar.Qaros.AppVolumeUpdatedEventArgs.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each call carries an immutable [`AppVolume`](Quaternar.Qaros.AppVolume.md) snapshot that is safe to keep.
Its poses are in the publishing peer's room space.

The handler runs on a library thread: do not block in it, do not call blocking QAROS
methods from it, and marshal to your own thread before touching single-threaded
state. An exception it throws is caught and dropped.

Disposing the result ends delivery: when `Dispose` returns, a handler call
running on another thread has finished and no further one starts. Called from inside
the handler, `Dispose` returns at once and no call starts after the current one.
Never dispose while holding a lock the handler takes: it deadlocks. Cancelling
`cancellationToken` ends delivery the same way, and
[`CancellationTokenSource.Cancel`](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtokensource.cancel#system-threading-cancellationtokensource-cancel) waits the same way.

### SubscribeUpdates(EventHandler&lt;AppVolumeUpdatedEventArgs&gt;, CancellationToken) {#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken}

Calls `handler` once for every app volume already stored, then again
for every change to any volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<AppVolumeUpdatedEventArgs> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeUpdatedEventArgs](Quaternar.Qaros.AppVolumeUpdatedEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each call carries an immutable [`AppVolume`](Quaternar.Qaros.AppVolume.md) snapshot that is safe to keep.
Its poses are in the publishing peer's room space.

The handler runs on a library thread: do not block in it, do not call blocking QAROS
methods from it, and marshal to your own thread before touching single-threaded
state. An exception it throws is caught and dropped.

Disposing the result ends delivery: when `Dispose` returns, a handler call
running on another thread has finished and no further one starts. Called from inside
the handler, `Dispose` returns at once and no call starts after the current one.
Never dispose while holding a lock the handler takes: it deadlocks. Cancelling
`cancellationToken` ends delivery the same way, and
[`CancellationTokenSource.Cancel`](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtokensource.cancel#system-threading-cancellationtokensource-cancel) waits the same way.

### WatchUpdatesAsync {#watchupdatesasync}

The snapshots of
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken)
as an async stream: every stored volume first, then every change.

```csharp
public IAsyncEnumerable<AppVolume> WatchUpdatesAsync(CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [IAsyncEnumerable](https://learn.microsoft.com/dotnet/api/system.collections.generic.iasyncenumerable-1)&lt;[AppVolume](Quaternar.Qaros.AppVolume.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | Thrown on the first `MoveNextAsync` when the subscription cannot be started. |

**Remarks**

The subscription starts with the enumeration and ends when the enumerator is disposed.
Updates are queued without bound until read, so a consumer that stops reading without
disposing keeps accumulating them.

### SubscribeGestureUpdates(AppVolumeId, GestureKind, Action&lt;AppVolumeGestureEvent&gt;) {#subscribegestureupdates-appvolumeid-gesturekind-action-appvolumegestureevent}

Calls `handler` for every gesture of one kind that a peer performs
against one app volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `handler` | [Action](https://learn.microsoft.com/dotnet/api/system.action-1)&lt;[AppVolumeGestureEvent](Quaternar.Qaros.AppVolumeGestureEvent.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each gesture is already mapped through the volume's gesture configuration: filtered
by its rules, its
points in app content space, its deltas in app volume space. When
[`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform) is true the library
already applied the delta to the app pose or scale; applying it again moves the
content twice.

Several producers publish, one per user who can reach the volume. Order holds within
one producer, not across producers, and a source peer never seen before is a new user.
Keep per-user state per [`AppVolumeGestureEvent.SourcePeerId`](Quaternar.Qaros.AppVolumeGestureEvent.md#sourcepeerid). Separate
per-kind subscriptions have no order relative to each other; to take more than one
kind, prefer [`AppVolumeControl.SubscribeAllGestureUpdates`](#subscribeallgestureupdates).

Threading, exceptions and disposal follow
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken):
the handler runs on a library thread, and `Dispose` waits for a running call.

### SubscribeGestureUpdates(AppVolumeId, GestureKind, Action&lt;AppVolumeGestureEvent&gt;, CancellationToken) {#subscribegestureupdates-appvolumeid-gesturekind-action-appvolumegestureevent-cancellationtoken}

Calls `handler` for every gesture of one kind that a peer performs
against one app volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `handler` | [Action](https://learn.microsoft.com/dotnet/api/system.action-1)&lt;[AppVolumeGestureEvent](Quaternar.Qaros.AppVolumeGestureEvent.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each gesture is already mapped through the volume's gesture configuration: filtered
by its rules, its
points in app content space, its deltas in app volume space. When
[`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform) is true the library
already applied the delta to the app pose or scale; applying it again moves the
content twice.

Several producers publish, one per user who can reach the volume. Order holds within
one producer, not across producers, and a source peer never seen before is a new user.
Keep per-user state per [`AppVolumeGestureEvent.SourcePeerId`](Quaternar.Qaros.AppVolumeGestureEvent.md#sourcepeerid). Separate
per-kind subscriptions have no order relative to each other; to take more than one
kind, prefer [`AppVolumeControl.SubscribeAllGestureUpdates`](#subscribeallgestureupdates).

Threading, exceptions and disposal follow
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken):
the handler runs on a library thread, and `Dispose` waits for a running call.

### SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler&lt;AppVolumeGestureEventArgs&gt;) {#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs}

Calls `handler` for every gesture of one kind that a peer performs
against one app volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeGestureEventArgs](Quaternar.Qaros.AppVolumeGestureEventArgs.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each gesture is already mapped through the volume's gesture configuration: filtered
by its rules, its
points in app content space, its deltas in app volume space. When
[`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform) is true the library
already applied the delta to the app pose or scale; applying it again moves the
content twice.

Several producers publish, one per user who can reach the volume. Order holds within
one producer, not across producers, and a source peer never seen before is a new user.
Keep per-user state per [`AppVolumeGestureEvent.SourcePeerId`](Quaternar.Qaros.AppVolumeGestureEvent.md#sourcepeerid). Separate
per-kind subscriptions have no order relative to each other; to take more than one
kind, prefer [`AppVolumeControl.SubscribeAllGestureUpdates`](#subscribeallgestureupdates).

Threading, exceptions and disposal follow
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken):
the handler runs on a library thread, and `Dispose` waits for a running call.

### SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler&lt;AppVolumeGestureEventArgs&gt;, CancellationToken) {#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs-cancellationtoken}

Calls `handler` for every gesture of one kind that a peer performs
against one app volume, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeGestureEventArgs](Quaternar.Qaros.AppVolumeGestureEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each gesture is already mapped through the volume's gesture configuration: filtered
by its rules, its
points in app content space, its deltas in app volume space. When
[`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform) is true the library
already applied the delta to the app pose or scale; applying it again moves the
content twice.

Several producers publish, one per user who can reach the volume. Order holds within
one producer, not across producers, and a source peer never seen before is a new user.
Keep per-user state per [`AppVolumeGestureEvent.SourcePeerId`](Quaternar.Qaros.AppVolumeGestureEvent.md#sourcepeerid). Separate
per-kind subscriptions have no order relative to each other; to take more than one
kind, prefer [`AppVolumeControl.SubscribeAllGestureUpdates`](#subscribeallgestureupdates).

Threading, exceptions and disposal follow
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken):
the handler runs on a library thread, and `Dispose` waits for a running call.

### WatchGestureUpdatesAsync {#watchgestureupdatesasync}

The gestures of
[`AppVolumeControl.SubscribeGestureUpdates`](#subscribegestureupdates-appvolumeid-gesturekind-eventhandler-appvolumegestureeventargs-cancellationtoken)
as an async stream.

```csharp
public IAsyncEnumerable<AppVolumeGestureEvent> WatchGestureUpdatesAsync(AppVolumeId id, GestureKind gestureKind, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [IAsyncEnumerable](https://learn.microsoft.com/dotnet/api/system.collections.generic.iasyncenumerable-1)&lt;[AppVolumeGestureEvent](Quaternar.Qaros.AppVolumeGestureEvent.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | Thrown on the first `MoveNextAsync` when the subscription cannot be started. |

**Remarks**

The subscription starts with the enumeration and ends when the enumerator is disposed.
Gestures are queued without bound until read.

## Events

### Updated {#updated}

Raised for every app volume update, but only while a subscription started by
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates) or
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-cancellationtoken) is alive.

```csharp
public event EventHandler<AppVolumeUpdatedEventArgs>? Updated
```

**Event type:** [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[AppVolumeUpdatedEventArgs](Quaternar.Qaros.AppVolumeUpdatedEventArgs.md)&gt;

**Remarks**

Raised on a library thread with this control as the sender; see
[`AppVolumeControl.SubscribeUpdates`](#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken)
for what is delivered and the threading rules. Each such subscription raises the event
once per update, so two live subscriptions raise it twice.
