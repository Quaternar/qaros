---
title: "GuiPanelSize struct"
sidebar_label: "GuiPanelSize"
description: "A GUI panel's surface size in metres and the zoom of the content drawn on it. The surface spans panel-space X from -Width/2 to +Width/2 and hangs down from the pose to -Height."
---

# GuiPanelSize struct

A GUI panel's surface size in metres and the zoom of the content drawn on it. The
surface spans panel-space X from -Width/2 to +Width/2 and hangs down from the pose to
-Height.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1448)

```csharp
public readonly struct GuiPanelSize
```

## Constructors

### GuiPanelSize(float, float, float) {#ctor}

```csharp
public GuiPanelSize(float widthMeters, float heightMeters, float contentScale)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `widthMeters` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `heightMeters` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `contentScale` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [WidthMeters](#widthmeters) |  |
| [HeightMeters](#heightmeters) |  |
| [ContentScale](#contentscale) | Zoom of the content raster on the surface: 1.0 fits the content across the full panel, 2.0 shows it at twice the size so only half fits, 0.5 at half size. It changes how contacts map to content pixels, not the panel's size. |
| [Default](#default) | 1 m by 1 m, content scale 1. |

### WidthMeters {#widthmeters}

```csharp
public float WidthMeters { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### HeightMeters {#heightmeters}

```csharp
public float HeightMeters { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### ContentScale {#contentscale}

Zoom of the content raster on the surface: 1.0 fits the content across the full panel,
2.0 shows it at twice the size so only half fits, 0.5 at half size. It changes how
contacts map to content pixels, not the panel's size.

```csharp
public float ContentScale { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

**Remarks**

A website panel takes it as the page zoom, like a desktop browser's Ctrl+ and Ctrl-:
the page is drawn larger and reflowed to the panel width. Only the application sets
it; the panel's own + and - buttons resize the panel instead. A stream panel ignores
it: its video is always drawn at full panel width and scrolled vertically when taller
than the panel.

### Default {#default}

1 m by 1 m, content scale 1.

```csharp
public static GuiPanelSize Default { get; }
```

**Value:** [GuiPanelSize](Quaternar.Qaros.GuiPanelSize.md)
