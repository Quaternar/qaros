---
title: "VideoFrameCpu struct"
sidebar_label: "VideoFrameCpu"
description: "A snapshot of a sender's CPU frame: its views and textures, in layout order. The pixel memory stays the sender's."
---

# VideoFrameCpu struct

A snapshot of a sender's CPU frame: its views and textures, in layout order. The pixel
memory stays the sender's.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2049)

```csharp
public readonly struct VideoFrameCpu
```

## Remarks

A render sender's frame exists from creation, so it can be read before the first
[`RenderSender.BeginFrame`](Quaternar.Qaros.RenderSender.md#beginframe) to size resources; its contents may be written only
between a successful begin-frame and [`RenderSender.ShowFrame`](Quaternar.Qaros.RenderSender.md#showframe). A
[`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md) hands it out during the send cycle
[`PrimarylessVideoSender.TryBeginFrame`](Quaternar.Qaros.PrimarylessVideoSender.md#trybeginframe) reserved.

## Constructors

### VideoFrameCpu(VideoFrameView\[\], VideoTextureCpu\[\]) {#ctor}

```csharp
public VideoFrameCpu(VideoFrameView[] textureViews, VideoTextureCpu[] textures)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `textureViews` | [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\] |  |
| `textures` | [VideoTextureCpu](Quaternar.Qaros.VideoTextureCpu.md)\[\] |  |

## Properties

### TextureViews {#textureviews}

The views in layout order; each view's texture index names its texture.

```csharp
public VideoFrameView[] TextureViews { get; }
```

**Value:** [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\]

### Textures {#textures}

```csharp
public VideoTextureCpu[] Textures { get; }
```

**Value:** [VideoTextureCpu](Quaternar.Qaros.VideoTextureCpu.md)\[\]
