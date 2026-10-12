---
title: "AppVolumeGestureState class"
sidebar_label: "AppVolumeGestureState"
description: "An app volume's gesture modes and who is using them, read with the volume snapshot."
---

# AppVolumeGestureState class

An app volume's gesture modes and who is using them, read with the
volume snapshot.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [AppVolume.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/AppVolume.cs#L185)

```csharp
public sealed class AppVolumeGestureState
```

## Fields

### DefaultMode {#defaultmode}

The mode every volume has and cannot remove, and the one every peer is in until switched.

```csharp
public const uint DefaultMode = 0
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

## Properties

| Member | Description |
|---|---|
| [AllowedPeers](#allowedpeers) | The peers whose gestures may move or scale the app; empty means every peer's. |
| [Holder](#holder) | The one peer currently manipulating the app with a gesture, or null when none holds it. |
| [Modes](#modes) | Every mode the volume defines, by id, with its rules; always contains [`AppVolumeGestureState.DefaultMode`](#defaultmode). |
| [SelectedDefaultModeId](#selecteddefaultmodeid) | The mode every peer without its own mode follows. |
| [ActiveModeByPeer](#activemodebypeer) | The mode each peer using the volume maps its next gesture through. |

### AllowedPeers {#allowedpeers}

The peers whose gestures may move or scale the app; empty means every peer's.

```csharp
public PeerId[] AllowedPeers { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]

### Holder {#holder}

The one peer currently manipulating the app with a gesture, or null when none holds it.

```csharp
public PeerId? Holder { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)?

### Modes {#modes}

Every mode the volume defines, by id, with its rules; always contains [`AppVolumeGestureState.DefaultMode`](#defaultmode).

```csharp
public IReadOnlyDictionary<uint, AppVolumeGestureConfiguration> Modes { get; }
```

**Value:** [IReadOnlyDictionary](https://learn.microsoft.com/dotnet/api/system.collections.generic.ireadonlydictionary-2)&lt;[uint](https://learn.microsoft.com/dotnet/api/system.uint32), [AppVolumeGestureConfiguration](Quaternar.Qaros.AppVolumeGestureConfiguration.md)&gt;

### SelectedDefaultModeId {#selecteddefaultmodeid}

The mode every peer without its own mode follows.

```csharp
public uint SelectedDefaultModeId { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### ActiveModeByPeer {#activemodebypeer}

The mode each peer using the volume maps its next gesture through.

```csharp
public IReadOnlyDictionary<PeerId, uint> ActiveModeByPeer { get; }
```

**Value:** [IReadOnlyDictionary](https://learn.microsoft.com/dotnet/api/system.collections.generic.ireadonlydictionary-2)&lt;[PeerId](Quaternar.Qaros.PeerId.md), [uint](https://learn.microsoft.com/dotnet/api/system.uint32)&gt;
