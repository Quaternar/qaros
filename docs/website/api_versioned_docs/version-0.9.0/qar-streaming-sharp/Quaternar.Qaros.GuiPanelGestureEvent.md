---
title: "GuiPanelGestureEvent struct"
sidebar_label: "GuiPanelGestureEvent"
description: "One recognised gesture on a GUI panel, copied out of the native callback."
---

# GuiPanelGestureEvent struct

One recognised gesture on a GUI panel, copied out of the native callback.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1905)

```csharp
public readonly struct GuiPanelGestureEvent
```

## Remarks

Several producers publish on one panel, one per user. Key lifecycles on
([`GuiPanelGestureEvent.SourcePeerId`](#sourcepeerid), [`GuiPanelGestureEvent.GestureId`](#gestureid)): the id is unique only
within a source peer.

## Constructors

### GuiPanelGestureEvent(PeerId, ulong, GuiPanelId, GuiPanelGestureKind, GesturePhase, GuiPanelPoint, GuiPanelPoint, TimePoint, Handedness, byte) {#ctor}

```csharp
public GuiPanelGestureEvent(PeerId sourcePeerId, ulong gestureId, GuiPanelId panelId, GuiPanelGestureKind kind, GesturePhase phase, GuiPanelPoint position, GuiPanelPoint startPosition, TimePoint timestamp, Handedness handedness, byte tapCount)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `sourcePeerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `gestureId` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `panelId` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `kind` | [GuiPanelGestureKind](Quaternar.Qaros.GuiPanelGestureKind.md) |  |
| `phase` | [GesturePhase](Quaternar.Qaros.GesturePhase.md) |  |
| `position` | [GuiPanelPoint](Quaternar.Qaros.GuiPanelPoint.md) |  |
| `startPosition` | [GuiPanelPoint](Quaternar.Qaros.GuiPanelPoint.md) |  |
| `timestamp` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `handedness` | [Handedness](Quaternar.Qaros.Handedness.md) |  |
| `tapCount` | [byte](https://learn.microsoft.com/dotnet/api/system.byte) |  |

## Properties

| Member | Description |
|---|---|
| [SourcePeerId](#sourcepeerid) | The peer whose hand made the contact, not the peer that recognised it. |
| [GestureId](#gestureid) | Non-zero and stable for one hover or drag lifecycle; 0 for tap and long press. |
| [PanelId](#panelid) |  |
| [Kind](#kind) |  |
| [Phase](#phase) |  |
| [Position](#position) | Where the contact is now, in panel-content-space pixels. |
| [StartPosition](#startposition) | Where this lifecycle began, in panel-content-space pixels; equals [`GuiPanelGestureEvent.Position`](#position) for instant kinds. |
| [Timestamp](#timestamp) | When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer. |
| [Handedness](#handedness) | The hand that made the contact; [`Handedness.Unspecified`](Quaternar.Qaros.Handedness.md#unspecified) when the recogniser did not tell. |
| [TapCount](#tapcount) | Taps in this run for [`GuiPanelGestureKind.Tap`](Quaternar.Qaros.GuiPanelGestureKind.md#tap); 0 for every other kind. |

### SourcePeerId {#sourcepeerid}

The peer whose hand made the contact, not the peer that recognised it.

```csharp
public PeerId SourcePeerId { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### GestureId {#gestureid}

Non-zero and stable for one hover or drag lifecycle; 0 for tap and long press.

```csharp
public ulong GestureId { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### PanelId {#panelid}

```csharp
public GuiPanelId PanelId { get; }
```

**Value:** [GuiPanelId](Quaternar.Qaros.GuiPanelId.md)

### Kind {#kind}

```csharp
public GuiPanelGestureKind Kind { get; }
```

**Value:** [GuiPanelGestureKind](Quaternar.Qaros.GuiPanelGestureKind.md)

### Phase {#phase}

```csharp
public GesturePhase Phase { get; }
```

**Value:** [GesturePhase](Quaternar.Qaros.GesturePhase.md)

### Position {#position}

Where the contact is now, in panel-content-space pixels.

```csharp
public GuiPanelPoint Position { get; }
```

**Value:** [GuiPanelPoint](Quaternar.Qaros.GuiPanelPoint.md)

### StartPosition {#startposition}

Where this lifecycle began, in panel-content-space pixels; equals [`GuiPanelGestureEvent.Position`](#position) for instant kinds.

```csharp
public GuiPanelPoint StartPosition { get; }
```

**Value:** [GuiPanelPoint](Quaternar.Qaros.GuiPanelPoint.md)

### Timestamp {#timestamp}

When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer.

```csharp
public TimePoint Timestamp { get; }
```

**Value:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### Handedness {#handedness}

The hand that made the contact; [`Handedness.Unspecified`](Quaternar.Qaros.Handedness.md#unspecified) when the recogniser did not tell.

```csharp
public Handedness Handedness { get; }
```

**Value:** [Handedness](Quaternar.Qaros.Handedness.md)

### TapCount {#tapcount}

Taps in this run for [`GuiPanelGestureKind.Tap`](Quaternar.Qaros.GuiPanelGestureKind.md#tap); 0 for every other kind.

```csharp
public byte TapCount { get; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)
