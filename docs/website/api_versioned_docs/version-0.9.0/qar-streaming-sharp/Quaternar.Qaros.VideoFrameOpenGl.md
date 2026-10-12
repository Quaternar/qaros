---
title: "VideoFrameOpenGl struct"
sidebar_label: "VideoFrameOpenGl"
description: "A snapshot of a sender's OpenGL frame: its views and textures. The textures stay the sender's; render into them only inside a frame cycle. A render sender's frame is readable from creation, before the"
---

# VideoFrameOpenGl struct

A snapshot of a sender's OpenGL frame: its views and textures. The textures stay the
sender's; render into them only inside a frame cycle. A render sender's frame is readable
from creation, before the first begin-frame.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2163)

```csharp
public readonly struct VideoFrameOpenGl
```

## Constructors

### VideoFrameOpenGl(VideoFrameView\[\], VideoTextureOpenGl\[\]) {#ctor}

```csharp
public VideoFrameOpenGl(VideoFrameView[] textureViews, VideoTextureOpenGl[] textures)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `textureViews` | [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\] |  |
| `textures` | [VideoTextureOpenGl](Quaternar.Qaros.VideoTextureOpenGl.md)\[\] |  |

## Properties

### TextureViews {#textureviews}

```csharp
public VideoFrameView[] TextureViews { get; }
```

**Value:** [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\]

### Textures {#textures}

```csharp
public VideoTextureOpenGl[] Textures { get; }
```

**Value:** [VideoTextureOpenGl](Quaternar.Qaros.VideoTextureOpenGl.md)\[\]
