---
title: "AppVolume class"
sidebar_label: "AppVolume"
description: "An immutable snapshot of one app volume: the box in the room a remote 3D application renders into, and how the application's content sits inside it."
---

# AppVolume class

An immutable snapshot of one app volume: the box in the room a remote 3D application
renders into, and how the application's content sits inside it.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [AppVolume.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/AppVolume.cs#L31)

```csharp
public sealed class AppVolume
```

## Remarks

Safe to keep; it never changes. Get a newer one from
[`AppVolumeControl.SubscribeUpdates`](Quaternar.Qaros.AppVolumeControl.md#subscribeupdates-eventhandler-appvolumeupdatedeventargs-cancellationtoken),
or read the live transform with the `AppVolumeControl.GetLatest*` methods, which can
be ahead of the last snapshot.

Three spaces: [`AppVolume.Pose`](#pose) places the box in room space. App volume space is the
box's own frame: origin at the cuboid centre, +X along width, +Y up along height, +Z
along length, in metres. App content space, the space the application renders in, is
app volume space taken into [`AppVolume.AppPose`](#apppose) with positions divided by
[`AppVolume.AppScale`](#appscale). Nothing clips the content to the box.

## Properties

| Member | Description |
|---|---|
| [Id](#id) |  |
| [DisplayName](#displayname) |  |
| [Pose](#pose) | The cuboid centre's pose in the publishing peer's room space. |
| [Size](#size) |  |
| [LifetimeStatus](#lifetimestatus) | [`AppVolumeLifetimeStatus.Closed`](Quaternar.Qaros.AppVolumeLifetimeStatus.md#closed) after [`AppVolumeControl.Close`](Quaternar.Qaros.AppVolumeControl.md#close); [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate) with the same common name reopens it. |
| [EditingStatus](#editingstatus) | Which peer, if any, is moving or resizing the box itself ([`AppVolumeControl.StartEditing`](Quaternar.Qaros.AppVolumeControl.md#startediting)). A peer manipulating the app with a gesture is in [`AppVolumeGestureState.Holder`](Quaternar.Qaros.AppVolumeGestureState.md#holder) instead. |
| [UsedByPeers](#usedbypeers) | The peers using the volume ([`AppVolumeControl.UpdateUsedByPeers`](Quaternar.Qaros.AppVolumeControl.md#updateusedbypeers)). [`AppVolumeGestureState.ActiveModeByPeer`](Quaternar.Qaros.AppVolumeGestureState.md#activemodebypeer) covers exactly these peers. |
| [AppPose](#apppose) | Origin of the rendering application in app volume space, in volume metres; not scaled by [`AppVolume.AppScale`](#appscale). |
| [AppScale](#appscale) | Room-metres per app-metre: the factor between app content space and app volume space. |
| [AppWorldAnchor](#appworldanchor) | The Earth-fixed frame (ECEF WGS84) the volume is pinned to, if one was set with [`AppVolumeControl.ChangeAppWorldAnchor`](Quaternar.Qaros.AppVolumeControl.md#changeappworldanchor). |
| [GestureConfiguration](#gestureconfiguration) | The rules of gesture mode [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode) and the allowed peers. |
| [GestureState](#gesturestate) | Gesture modes, the selected modes and the peer currently manipulating the app. |

### Id {#id}

```csharp
public AppVolumeId Id { get; }
```

**Value:** [AppVolumeId](Quaternar.Qaros.AppVolumeId.md)

### DisplayName {#displayname}

```csharp
public string DisplayName { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Pose {#pose}

The cuboid centre's pose in the publishing peer's room space.

```csharp
public Pose Pose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Size {#size}

```csharp
public AppVolumeSize Size { get; }
```

**Value:** [AppVolumeSize](Quaternar.Qaros.AppVolumeSize.md)

### LifetimeStatus {#lifetimestatus}

[`AppVolumeLifetimeStatus.Closed`](Quaternar.Qaros.AppVolumeLifetimeStatus.md#closed) after [`AppVolumeControl.Close`](Quaternar.Qaros.AppVolumeControl.md#close);
[`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate) with the same common name reopens it.

```csharp
public AppVolumeLifetimeStatus LifetimeStatus { get; }
```

**Value:** [AppVolumeLifetimeStatus](Quaternar.Qaros.AppVolumeLifetimeStatus.md)

### EditingStatus {#editingstatus}

Which peer, if any, is moving or resizing the box itself
([`AppVolumeControl.StartEditing`](Quaternar.Qaros.AppVolumeControl.md#startediting)). A peer manipulating the app with a
gesture is in [`AppVolumeGestureState.Holder`](Quaternar.Qaros.AppVolumeGestureState.md#holder) instead.

```csharp
public AppVolumeEditingStatus EditingStatus { get; }
```

**Value:** [AppVolumeEditingStatus](Quaternar.Qaros.AppVolumeEditingStatus.md)

### UsedByPeers {#usedbypeers}

The peers using the volume ([`AppVolumeControl.UpdateUsedByPeers`](Quaternar.Qaros.AppVolumeControl.md#updateusedbypeers)).
[`AppVolumeGestureState.ActiveModeByPeer`](Quaternar.Qaros.AppVolumeGestureState.md#activemodebypeer) covers exactly these peers.

```csharp
public PeerId[] UsedByPeers { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]

### AppPose {#apppose}

Origin of the rendering application in app volume space, in volume metres; not scaled by [`AppVolume.AppScale`](#appscale).

```csharp
public Pose AppPose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### AppScale {#appscale}

Room-metres per app-metre: the factor between app content space and app volume space.

```csharp
public float AppScale { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AppWorldAnchor {#appworldanchor}

The Earth-fixed frame (ECEF WGS84) the volume is pinned to, if one was set with
[`AppVolumeControl.ChangeAppWorldAnchor`](Quaternar.Qaros.AppVolumeControl.md#changeappworldanchor).

```csharp
public AppWorldAnchor AppWorldAnchor { get; }
```

**Value:** [AppWorldAnchor](Quaternar.Qaros.AppWorldAnchor.md)

### GestureConfiguration {#gestureconfiguration}

The rules of gesture mode [`AppVolumeGestureState.DefaultMode`](Quaternar.Qaros.AppVolumeGestureState.md#defaultmode) and the allowed peers.

```csharp
public AppVolumeGestureConfiguration GestureConfiguration { get; }
```

**Value:** [AppVolumeGestureConfiguration](Quaternar.Qaros.AppVolumeGestureConfiguration.md)

### GestureState {#gesturestate}

Gesture modes, the selected modes and the peer currently manipulating the app.

```csharp
public AppVolumeGestureState GestureState { get; }
```

**Value:** [AppVolumeGestureState](Quaternar.Qaros.AppVolumeGestureState.md)
