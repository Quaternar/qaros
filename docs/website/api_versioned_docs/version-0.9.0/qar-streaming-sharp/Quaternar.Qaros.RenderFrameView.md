---
title: "RenderFrameView struct"
sidebar_label: "RenderFrameView"
description: "One view declared in RenderSenderOptions.FrameViews."
---

# RenderFrameView struct

One view declared in [`RenderSenderOptions.FrameViews`](Quaternar.Qaros.RenderSenderOptions.md#frameviews).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1283)

```csharp
public readonly struct RenderFrameView
```

## Constructors

### RenderFrameView(VideoFrameViewType, VideoFrameViewEye, uint) {#ctor}

```csharp
public RenderFrameView(VideoFrameViewType dataType, VideoFrameViewEye eye, uint textureIndex)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `dataType` | [VideoFrameViewType](Quaternar.Qaros.VideoFrameViewType.md) |  |
| `eye` | [VideoFrameViewEye](Quaternar.Qaros.VideoFrameViewEye.md) |  |
| `textureIndex` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

| Member | Description |
|---|---|
| [DataType](#datatype) |  |
| [Eye](#eye) |  |
| [TextureIndex](#textureindex) |  |

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

### TextureIndex {#textureindex}

```csharp
public uint TextureIndex { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
