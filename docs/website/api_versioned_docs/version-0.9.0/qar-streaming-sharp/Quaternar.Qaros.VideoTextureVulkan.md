---
title: "VideoTextureVulkan struct"
sidebar_label: "VideoTextureVulkan"
description: "One sender-owned Vulkan image and the state it is in."
---

# VideoTextureVulkan struct

One sender-owned Vulkan image and the state it is in.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2182)

```csharp
public readonly struct VideoTextureVulkan
```

## Constructors

### VideoTextureVulkan(TextureSize, ulong, ulong, uint, int, uint) {#ctor}

```csharp
public VideoTextureVulkan(TextureSize size, ulong image, ulong imageMemory, uint formatFeatures, int layout, uint queueFamilyIndex)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `size` | [TextureSize](Quaternar.Qaros.TextureSize.md) |  |
| `image` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `imageMemory` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `formatFeatures` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `layout` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |
| `queueFamilyIndex` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

| Member | Description |
|---|---|
| [Size](#size) |  |
| [Image](#image) | VkImage handle. |
| [ImageMemory](#imagememory) | VkDeviceMemory handle. |
| [FormatFeatures](#formatfeatures) | VkFormatFeatureFlags. |
| [Layout](#layout) | VkImageLayout the image is in. |
| [QueueFamilyIndex](#queuefamilyindex) | Queue family that owns the image. |

### Size {#size}

```csharp
public TextureSize Size { get; }
```

**Value:** [TextureSize](Quaternar.Qaros.TextureSize.md)

### Image {#image}

VkImage handle.

```csharp
public ulong Image { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### ImageMemory {#imagememory}

VkDeviceMemory handle.

```csharp
public ulong ImageMemory { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### FormatFeatures {#formatfeatures}

VkFormatFeatureFlags.

```csharp
public uint FormatFeatures { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Layout {#layout}

VkImageLayout the image is in.

```csharp
public int Layout { get; }
```

**Value:** [int](https://learn.microsoft.com/dotnet/api/system.int32)

### QueueFamilyIndex {#queuefamilyindex}

Queue family that owns the image.

```csharp
public uint QueueFamilyIndex { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
