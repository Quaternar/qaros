---
title: "TextureSize struct"
sidebar_label: "TextureSize"
description: "One texture of a frame layout; width and height in pixels."
---

# TextureSize struct

One texture of a frame layout; width and height in pixels.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1321)

```csharp
public readonly struct TextureSize
```

## Constructors

### TextureSize(PixelFormat, uint, uint, uint) {#ctor}

```csharp
public TextureSize(PixelFormat format, uint width, uint height, uint arrayLayers)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `format` | [PixelFormat](Quaternar.Qaros.PixelFormat.md) |  |
| `width` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `height` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `arrayLayers` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

| Member | Description |
|---|---|
| [Format](#format) |  |
| [Width](#width) |  |
| [Height](#height) |  |
| [ArrayLayers](#arraylayers) |  |

### Format {#format}

```csharp
public PixelFormat Format { get; }
```

**Value:** [PixelFormat](Quaternar.Qaros.PixelFormat.md)

### Width {#width}

```csharp
public uint Width { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Height {#height}

```csharp
public uint Height { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### ArrayLayers {#arraylayers}

```csharp
public uint ArrayLayers { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
