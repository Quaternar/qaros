---
title: "VideoFrameD3D11 struct"
sidebar_label: "VideoFrameD3D11"
description: "A snapshot of a sender's D3D11 frame: its views, its shared textures and their keyed-mutex synchronization. The textures stay the sender's; render into them only inside a frame cycle. A render sender'"
---

# VideoFrameD3D11 struct

A snapshot of a sender's D3D11 frame: its views, its shared textures and their keyed-mutex
synchronization. The textures stay the sender's; render into them only inside a frame
cycle. A render sender's frame is readable from creation, before the first begin-frame.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2123)

```csharp
public readonly struct VideoFrameD3D11
```

## Constructors

### VideoFrameD3D11(VideoFrameView\[\], VideoTextureD3D11\[\], SyncFrameD3D11) {#ctor}

```csharp
public VideoFrameD3D11(VideoFrameView[] textureViews, VideoTextureD3D11[] textures, SyncFrameD3D11 synchronization)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `textureViews` | [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\] |  |
| `textures` | [VideoTextureD3D11](Quaternar.Qaros.VideoTextureD3D11.md)\[\] |  |
| `synchronization` | [SyncFrameD3D11](Quaternar.Qaros.SyncFrameD3D11.md) |  |

## Properties

| Member | Description |
|---|---|
| [TextureViews](#textureviews) |  |
| [Textures](#textures) |  |
| [Synchronization](#synchronization) |  |

### TextureViews {#textureviews}

```csharp
public VideoFrameView[] TextureViews { get; }
```

**Value:** [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\]

### Textures {#textures}

```csharp
public VideoTextureD3D11[] Textures { get; }
```

**Value:** [VideoTextureD3D11](Quaternar.Qaros.VideoTextureD3D11.md)\[\]

### Synchronization {#synchronization}

```csharp
public SyncFrameD3D11 Synchronization { get; }
```

**Value:** [SyncFrameD3D11](Quaternar.Qaros.SyncFrameD3D11.md)
