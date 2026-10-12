---
title: "VideoTextureD3D11 struct"
sidebar_label: "VideoTextureD3D11"
---

# VideoTextureD3D11 struct

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2068)

```csharp
public readonly struct VideoTextureD3D11
```

## Constructors

### VideoTextureD3D11(TextureSize, nint) {#ctor}

```csharp
public VideoTextureD3D11(TextureSize size, nint texture)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `size` | [TextureSize](Quaternar.Qaros.TextureSize.md) |  |
| `texture` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |

## Properties

### Size {#size}

```csharp
public TextureSize Size { get; }
```

**Value:** [TextureSize](Quaternar.Qaros.TextureSize.md)

### Texture {#texture}

ID3D11Texture2D*, a shared texture created on the session's adapter.

```csharp
public nint Texture { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)
