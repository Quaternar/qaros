---
title: "GuiPanelGestureKind enum"
sidebar_label: "GuiPanelGestureKind"
description: "What a contact on a GUI panel was recognised as."
---

# GuiPanelGestureKind enum

What a contact on a GUI panel was recognised as.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L101)

```csharp
public enum GuiPanelGestureKind
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="hover"></a>`Hover` | `0` | Fingertip inside the hover volume. A lifecycle: started on entry, updated as the content position changes, ended on exit. |
| <a id="tap"></a>`Tap` | `1` | Touched and released with little travel and before the long-press duration. Instant. |
| <a id="longpress"></a>`LongPress` | `2` | Touched and held in place for the long-press duration. Instant, once per contact. |
| <a id="drag"></a>`Drag` | `3` | A contact that travelled past the tap threshold. A lifecycle: started at the threshold crossing, ended on release, canceled when taken away. |
