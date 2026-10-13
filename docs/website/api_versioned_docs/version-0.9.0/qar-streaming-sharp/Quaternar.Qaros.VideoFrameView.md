---
title: "VideoFrameView struct"
sidebar_label: "VideoFrameView"
description: "Where one view of a video frame lives: a pixel rectangle in one texture (and array layer). The end coordinates are exclusive and may be smaller than the start ones, which flips the view."
---

# VideoFrameView struct

Where one view of a video frame lives: a pixel rectangle in one texture (and array layer).
The end coordinates are exclusive and may be smaller than the start ones, which flips the
view.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1352)

```csharp
public readonly struct VideoFrameView
```

## Constructors

### VideoFrameView(uint, uint, uint, uint, uint, uint, PixelFormat, VideoFrameViewType, VideoFrameViewEye) {#ctor}

```csharp
public VideoFrameView(uint startX, uint startY, uint endX, uint endY, uint textureIndex, uint arrayLayerIndex, PixelFormat textureFormat, VideoFrameViewType dataType, VideoFrameViewEye eye)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `startX` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `startY` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `endX` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `endY` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `textureIndex` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `arrayLayerIndex` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `textureFormat` | [PixelFormat](Quaternar.Qaros.PixelFormat.md) |  |
| `dataType` | [VideoFrameViewType](Quaternar.Qaros.VideoFrameViewType.md) |  |
| `eye` | [VideoFrameViewEye](Quaternar.Qaros.VideoFrameViewEye.md) |  |

## Properties

| Member | Description |
|---|---|
| [StartX](#startx) |  |
| [StartY](#starty) |  |
| [EndX](#endx) |  |
| [EndY](#endy) |  |
| [TextureIndex](#textureindex) | Index into the frame's or layout's textures. |
| [ArrayLayerIndex](#arraylayerindex) |  |
| [TextureFormat](#textureformat) |  |
| [DataType](#datatype) |  |
| [Eye](#eye) |  |

### StartX {#startx}

```csharp
public uint StartX { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### StartY {#starty}

```csharp
public uint StartY { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### EndX {#endx}

```csharp
public uint EndX { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### EndY {#endy}

```csharp
public uint EndY { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### TextureIndex {#textureindex}

Index into the frame's or layout's textures.

```csharp
public uint TextureIndex { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### ArrayLayerIndex {#arraylayerindex}

```csharp
public uint ArrayLayerIndex { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### TextureFormat {#textureformat}

```csharp
public PixelFormat TextureFormat { get; }
```

**Value:** [PixelFormat](Quaternar.Qaros.PixelFormat.md)

### DataType {#datatype}

```csharp
public VideoFrameViewType DataType { get; }
```

**Value:** [VideoFrameViewType](Quaternar.Qaros.VideoFrameViewType.md)

### Eye {#eye}

```csharp
public VideoFrameViewEye Eye { get; }
```

**Value:** [VideoFrameViewEye](Quaternar.Qaros.VideoFrameViewEye.md)
