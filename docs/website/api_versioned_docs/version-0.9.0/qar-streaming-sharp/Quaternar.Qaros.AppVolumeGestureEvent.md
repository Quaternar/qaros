---
title: "AppVolumeGestureEvent struct"
sidebar_label: "AppVolumeGestureEvent"
description: "One mapped gesture on an app volume, copied out of the native callback."
---

# AppVolumeGestureEvent struct

One mapped gesture on an app volume, copied out of the native callback.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1792)

```csharp
public readonly struct AppVolumeGestureEvent
```

## Constructors

### AppVolumeGestureEvent(PeerId, AppVolumeId, GestureKind, GesturePhase, TimePoint, bool, Vector3, bool, Vector3, Vector3, Quaternion, bool, Handedness, ulong, uint) {#ctor}

```csharp
public AppVolumeGestureEvent(PeerId sourcePeerId, AppVolumeId targetAppVolumeId, GestureKind gestureKind, GesturePhase state, TimePoint timestamp, bool hasStartPoint, Vector3 startPoint, bool hasActionPoint, Vector3 actionPoint, Vector3 translationDelta, Quaternion rotationDelta, bool wasMappedToAppTransform, Handedness handedness, ulong gestureId, uint modeId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `sourcePeerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `targetAppVolumeId` | [AppVolumeId](Quaternar.Qaros.AppVolumeId.md) |  |
| `gestureKind` | [GestureKind](Quaternar.Qaros.GestureKind.md) |  |
| `state` | [GesturePhase](Quaternar.Qaros.GesturePhase.md) |  |
| `timestamp` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `hasStartPoint` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `startPoint` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |
| `hasActionPoint` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `actionPoint` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |
| `translationDelta` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |
| `rotationDelta` | [Quaternion](Quaternar.Qaros.Mathematics.Quaternion.md) |  |
| `wasMappedToAppTransform` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `handedness` | [Handedness](Quaternar.Qaros.Handedness.md) |  |
| `gestureId` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `modeId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

| Member | Description |
|---|---|
| [SourcePeerId](#sourcepeerid) | The peer whose hand or controller produced the gesture, not the peer that recognised it. |
| [TargetAppVolumeId](#targetappvolumeid) |  |
| [GestureKind](#gesturekind) |  |
| [State](#state) | Where in its lifecycle the gesture sits. |
| [Timestamp](#timestamp) | When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer. |
| [HasStartPoint](#hasstartpoint) |  |
| [StartPoint](#startpoint) | Where the hand was when the gesture began, in app content space, in app units. Zero when [`AppVolumeGestureEvent.HasStartPoint`](#hasstartpoint) is false. |
| [HasActionPoint](#hasactionpoint) |  |
| [ActionPoint](#actionpoint) | The intersection the gesture acts on (or the start point), in app content space, in app units. Zero when [`AppVolumeGestureEvent.HasActionPoint`](#hasactionpoint) is false. |
| [TranslationDelta](#translationdelta) | Accumulated translation since the gesture started, in the target volume's app volume space (not app content space), shaped by the mapping rule. |
| [RotationDelta](#rotationdelta) | Accumulated rotation since the gesture started, in the target volume's app volume space. |
| [WasMappedToAppTransform](#wasmappedtoapptransform) | True when the library already folded the delta into the volume's app pose or app scale; applying it again moves the content twice. |
| [Handedness](#handedness) | The hand that made the gesture; [`Handedness.Unspecified`](Quaternar.Qaros.Handedness.md#unspecified) for two-hand gestures, which have no single hand. |
| [GestureId](#gestureid) | Lifecycle id, unique within the source peer; 0 for an instant gesture. |
| [ModeId](#modeid) | The gesture mode whose rule mapped the gesture, fixed for the gesture's whole lifecycle. |

### SourcePeerId {#sourcepeerid}

The peer whose hand or controller produced the gesture, not the peer that recognised it.

```csharp
public PeerId SourcePeerId { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### TargetAppVolumeId {#targetappvolumeid}

```csharp
public AppVolumeId TargetAppVolumeId { get; }
```

**Value:** [AppVolumeId](Quaternar.Qaros.AppVolumeId.md)

### GestureKind {#gesturekind}

```csharp
public GestureKind GestureKind { get; }
```

**Value:** [GestureKind](Quaternar.Qaros.GestureKind.md)

### State {#state}

Where in its lifecycle the gesture sits.

```csharp
public GesturePhase State { get; }
```

**Value:** [GesturePhase](Quaternar.Qaros.GesturePhase.md)

### Timestamp {#timestamp}

When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer.

```csharp
public TimePoint Timestamp { get; }
```

**Value:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### HasStartPoint {#hasstartpoint}

```csharp
public bool HasStartPoint { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### StartPoint {#startpoint}

Where the hand was when the gesture began, in app content space, in app units. Zero when [`AppVolumeGestureEvent.HasStartPoint`](#hasstartpoint) is false.

```csharp
public Vector3 StartPoint { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)

### HasActionPoint {#hasactionpoint}

```csharp
public bool HasActionPoint { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ActionPoint {#actionpoint}

The intersection the gesture acts on (or the start point), in app content space, in app units. Zero when [`AppVolumeGestureEvent.HasActionPoint`](#hasactionpoint) is false.

```csharp
public Vector3 ActionPoint { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)

### TranslationDelta {#translationdelta}

Accumulated translation since the gesture started, in the target volume's app volume space (not app content space), shaped by the mapping rule.

```csharp
public Vector3 TranslationDelta { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)

### RotationDelta {#rotationdelta}

Accumulated rotation since the gesture started, in the target volume's app volume space.

```csharp
public Quaternion RotationDelta { get; }
```

**Value:** [Quaternion](Quaternar.Qaros.Mathematics.Quaternion.md)

### WasMappedToAppTransform {#wasmappedtoapptransform}

True when the library already folded the delta into the volume's app pose or app scale; applying it again moves the content twice.

```csharp
public bool WasMappedToAppTransform { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### Handedness {#handedness}

The hand that made the gesture; [`Handedness.Unspecified`](Quaternar.Qaros.Handedness.md#unspecified) for two-hand gestures, which have no single hand.

```csharp
public Handedness Handedness { get; }
```

**Value:** [Handedness](Quaternar.Qaros.Handedness.md)

### GestureId {#gestureid}

Lifecycle id, unique within the source peer; 0 for an instant gesture.

```csharp
public ulong GestureId { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### ModeId {#modeid}

The gesture mode whose rule mapped the gesture, fixed for the gesture's whole lifecycle.

```csharp
public uint ModeId { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
