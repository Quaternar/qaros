---
title: "GuiPanelPoint struct"
sidebar_label: "GuiPanelPoint"
description: "A position on a GUI panel in panel-content-space pixels: origin top-left as the viewer sees it, +X right, +Y down."
---

# GuiPanelPoint struct

A position on a GUI panel in panel-content-space pixels: origin top-left as the viewer sees it, +X right, +Y down.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1886)

```csharp
public readonly struct GuiPanelPoint
```

## Remarks

Clamped to the image the renderer is displaying: x in [0, width - 1], y in
[0, height - 1]. For a stream panel that is the video at its sender's own resolution, and
on a video the renderer has scrolled it is the pixel under the finger, not one relative
to the visible part.

## Constructors

### GuiPanelPoint(int, int) {#ctor}

```csharp
public GuiPanelPoint(int xPixels, int yPixels)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `xPixels` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |
| `yPixels` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |

## Properties

### XPixels {#xpixels}

```csharp
public int XPixels { get; }
```

**Value:** [int](https://learn.microsoft.com/dotnet/api/system.int32)

### YPixels {#ypixels}

```csharp
public int YPixels { get; }
```

**Value:** [int](https://learn.microsoft.com/dotnet/api/system.int32)
