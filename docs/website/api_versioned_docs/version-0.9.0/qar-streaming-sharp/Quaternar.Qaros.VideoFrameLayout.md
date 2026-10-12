---
title: "VideoFrameLayout class"
sidebar_label: "VideoFrameLayout"
description: "The views of a video frame and the textures backing them."
---

# VideoFrameLayout class

The views of a video frame and the textures backing them.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1410)

```csharp
public sealed class VideoFrameLayout
```

## Constructors

### VideoFrameLayout() {#ctor}

```csharp
public VideoFrameLayout()
```

## Properties

### Views {#views}

At most 8 are sent; the rest are silently dropped.

```csharp
public VideoFrameView[] Views { get; set; }
```

**Value:** [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\]

### Textures {#textures}

At most 4 are sent; the rest are silently dropped.

```csharp
public TextureSize[] Textures { get; set; }
```

**Value:** [TextureSize](Quaternar.Qaros.TextureSize.md)\[\]
