---
title: "GuiPanelGestureEventArgs class"
sidebar_label: "GuiPanelGestureEventArgs"
---

# GuiPanelGestureEventArgs class

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarGuiPanelUpdatedEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/QarGuiPanelUpdatedEventArgs.cs#L19)

```csharp
public sealed class GuiPanelGestureEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → GuiPanelGestureEventArgs

## Constructors

### GuiPanelGestureEventArgs(GuiPanelGestureEvent) {#ctor}

```csharp
public GuiPanelGestureEventArgs(GuiPanelGestureEvent gestureEvent)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `gestureEvent` | [GuiPanelGestureEvent](Quaternar.Qaros.GuiPanelGestureEvent.md) |  |

## Properties

### GestureEvent {#gestureevent}

```csharp
public GuiPanelGestureEvent GestureEvent { get; }
```

**Value:** [GuiPanelGestureEvent](Quaternar.Qaros.GuiPanelGestureEvent.md)
