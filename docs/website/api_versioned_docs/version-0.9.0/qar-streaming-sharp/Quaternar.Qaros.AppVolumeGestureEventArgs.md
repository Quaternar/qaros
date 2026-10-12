---
title: "AppVolumeGestureEventArgs class"
sidebar_label: "AppVolumeGestureEventArgs"
---

# AppVolumeGestureEventArgs class

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarAppVolumeUpdatedEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/QarAppVolumeUpdatedEventArgs.cs#L19)

```csharp
public sealed class AppVolumeGestureEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → AppVolumeGestureEventArgs

## Constructors

### AppVolumeGestureEventArgs(AppVolumeGestureEvent) {#ctor}

```csharp
public AppVolumeGestureEventArgs(AppVolumeGestureEvent gestureEvent)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `gestureEvent` | [AppVolumeGestureEvent](Quaternar.Qaros.AppVolumeGestureEvent.md) |  |

## Properties

### GestureEvent {#gestureevent}

```csharp
public AppVolumeGestureEvent GestureEvent { get; }
```

**Value:** [AppVolumeGestureEvent](Quaternar.Qaros.AppVolumeGestureEvent.md)
