---
title: "AppVolumeUpdatedEventArgs class"
sidebar_label: "AppVolumeUpdatedEventArgs"
description: "One app volume update; AppVolumeUpdatedEventArgs.Volume is a snapshot, safe to keep."
---

# AppVolumeUpdatedEventArgs class

One app volume update; [`AppVolumeUpdatedEventArgs.Volume`](#volume) is a snapshot, safe to keep.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarAppVolumeUpdatedEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/QarAppVolumeUpdatedEventArgs.cs#L12)

```csharp
public sealed class AppVolumeUpdatedEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → AppVolumeUpdatedEventArgs

## Constructors

### AppVolumeUpdatedEventArgs(AppVolume) {#ctor}

```csharp
public AppVolumeUpdatedEventArgs(AppVolume volume)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `volume` | [AppVolume](Quaternar.Qaros.AppVolume.md) |  |

## Properties

### Volume {#volume}

```csharp
public AppVolume Volume { get; }
```

**Value:** [AppVolume](Quaternar.Qaros.AppVolume.md)
