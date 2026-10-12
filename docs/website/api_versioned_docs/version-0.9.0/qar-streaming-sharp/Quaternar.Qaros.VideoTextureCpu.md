---
title: "VideoTextureCpu struct"
sidebar_label: "VideoTextureCpu"
description: "One sender-owned CPU texture. The pixel memory is not copied: VideoTextureCpu.TextureData points into the sender."
---

# VideoTextureCpu struct

One sender-owned CPU texture. The pixel memory is not copied: [`VideoTextureCpu.TextureData`](#texturedata) points into the sender.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2000)

```csharp
public readonly struct VideoTextureCpu
```

## Constructors

### VideoTextureCpu(TextureSize, uint, nint, nuint) {#ctor}

```csharp
public VideoTextureCpu(TextureSize size, uint pitch, nint textureData, nuint textureDataSize)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `size` | [TextureSize](Quaternar.Qaros.TextureSize.md) |  |
| `pitch` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `textureData` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `textureDataSize` | [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr) |  |

## Properties

| Member | Description |
|---|---|
| [Size](#size) |  |
| [Pitch](#pitch) | Row stride in bytes. |
| [TextureData](#texturedata) | The sender's pixel buffer, [`VideoTextureCpu.TextureDataSize`](#texturedatasize) bytes. Write into it only inside a frame cycle, and never after the sender is disposed. |
| [TextureDataSize](#texturedatasize) | Size of [`VideoTextureCpu.TextureData`](#texturedata) in bytes. |
| [TextureDataSpan](#texturedataspan) | Read-only view of [`VideoTextureCpu.TextureData`](#texturedata); empty when there is none. Write pixels through [`VideoTextureCpu.TextureData`](#texturedata). |

### Size {#size}

```csharp
public TextureSize Size { get; }
```

**Value:** [TextureSize](Quaternar.Qaros.TextureSize.md)

### Pitch {#pitch}

Row stride in bytes.

```csharp
public uint Pitch { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### TextureData {#texturedata}

The sender's pixel buffer, [`VideoTextureCpu.TextureDataSize`](#texturedatasize) bytes. Write into it only
inside a frame cycle, and never after the sender is disposed.

```csharp
public nint TextureData { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### TextureDataSize {#texturedatasize}

Size of [`VideoTextureCpu.TextureData`](#texturedata) in bytes.

```csharp
public nuint TextureDataSize { get; }
```

**Value:** [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr)

### TextureDataSpan {#texturedataspan}

Read-only view of [`VideoTextureCpu.TextureData`](#texturedata); empty when there is none. Write pixels
through [`VideoTextureCpu.TextureData`](#texturedata).

```csharp
public ReadOnlySpan<byte> TextureDataSpan { get; }
```

**Value:** [ReadOnlySpan](https://learn.microsoft.com/dotnet/api/system.readonlyspan-1)&lt;[byte](https://learn.microsoft.com/dotnet/api/system.byte)&gt;
