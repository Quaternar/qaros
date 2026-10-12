---
title: "GuiPanelUpdatedEventArgs class"
sidebar_label: "GuiPanelUpdatedEventArgs"
description: "One GUI panel update; GuiPanelUpdatedEventArgs.Panel is a snapshot, safe to keep."
---

# GuiPanelUpdatedEventArgs class

One GUI panel update; [`GuiPanelUpdatedEventArgs.Panel`](#panel) is a snapshot, safe to keep.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarGuiPanelUpdatedEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/QarGuiPanelUpdatedEventArgs.cs#L12)

```csharp
public sealed class GuiPanelUpdatedEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → GuiPanelUpdatedEventArgs

## Constructors

### GuiPanelUpdatedEventArgs(GuiPanel) {#ctor}

```csharp
public GuiPanelUpdatedEventArgs(GuiPanel panel)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `panel` | [GuiPanel](Quaternar.Qaros.GuiPanel.md) |  |

## Properties

### Panel {#panel}

```csharp
public GuiPanel Panel { get; }
```

**Value:** [GuiPanel](Quaternar.Qaros.GuiPanel.md)
