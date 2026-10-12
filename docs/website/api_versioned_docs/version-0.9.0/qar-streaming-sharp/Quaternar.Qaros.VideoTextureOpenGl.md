---
title: "VideoTextureOpenGl struct"
sidebar_label: "VideoTextureOpenGl"
---

# VideoTextureOpenGl struct

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2144)

```csharp
public readonly struct VideoTextureOpenGl
```

## Constructors

### VideoTextureOpenGl(TextureSize, uint) {#ctor}

```csharp
public VideoTextureOpenGl(TextureSize size, uint texture)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `size` | [TextureSize](Quaternar.Qaros.TextureSize.md) |  |
| `texture` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

### Size {#size}

```csharp
public TextureSize Size { get; }
```

**Value:** [TextureSize](Quaternar.Qaros.TextureSize.md)

### Texture {#texture}

The GL texture name.

```csharp
public uint Texture { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
