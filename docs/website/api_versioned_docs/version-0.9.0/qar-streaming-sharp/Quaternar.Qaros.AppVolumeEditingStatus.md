---
title: "AppVolumeEditingStatus struct"
sidebar_label: "AppVolumeEditingStatus"
description: "Whether a peer holds the volume's editing lock (AppVolumeControl.StartEditing) to move or resize the box itself. A peer manipulating the app with a gesture is not editing it."
---

# AppVolumeEditingStatus struct

Whether a peer holds the volume's editing lock ([`AppVolumeControl.StartEditing`](Quaternar.Qaros.AppVolumeControl.md#startediting))
to move or resize the box itself. A peer manipulating the app with a gesture is not
editing it.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1569)

```csharp
public readonly struct AppVolumeEditingStatus
```

## Constructors

### AppVolumeEditingStatus(bool, PeerId) {#ctor}

```csharp
public AppVolumeEditingStatus(bool isBeingEdited, PeerId editorPeer)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `isBeingEdited` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `editorPeer` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

## Properties

### IsBeingEdited {#isbeingedited}

```csharp
public bool IsBeingEdited { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### EditorPeer {#editorpeer}

Meaningful only while [`AppVolumeEditingStatus.IsBeingEdited`](#isbeingedited) is true.

```csharp
public PeerId EditorPeer { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)
