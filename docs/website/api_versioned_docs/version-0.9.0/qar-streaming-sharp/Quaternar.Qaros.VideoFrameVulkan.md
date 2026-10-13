---
title: "VideoFrameVulkan class"
sidebar_label: "VideoFrameVulkan"
description: "Access to a sender-owned Vulkan frame. The library owns the native frame; this object is valid until the next GetVulkanFrame call on the same sender or the sender's disposal. Before showing a frame, w"
---

# VideoFrameVulkan class

Access to a sender-owned Vulkan frame. The library owns the native frame;
this object is valid until the next GetVulkanFrame call on the same sender
or the sender's disposal. Before showing a frame, write the layouts and
queue families the images were left in and the synchronization objects
with [`VideoFrameVulkan.SetTextureState`](#settexturestate) and [`VideoFrameVulkan.SetSynchronization`](#setsynchronization).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2215)

```csharp
public sealed class VideoFrameVulkan
```

## Properties

| Member | Description |
|---|---|
| [TextureViews](#textureviews) | Snapshot of the views when the frame was fetched. |
| [Textures](#textures) | Snapshot of the textures when the frame was fetched. |
| [Semaphore](#semaphore) | VkSemaphore when the frame was fetched. |
| [Fence](#fence) | VkFence when the frame was fetched. |

### TextureViews {#textureviews}

Snapshot of the views when the frame was fetched.

```csharp
public VideoFrameView[] TextureViews { get; }
```

**Value:** [VideoFrameView](Quaternar.Qaros.VideoFrameView.md)\[\]

### Textures {#textures}

Snapshot of the textures when the frame was fetched.

```csharp
public VideoTextureVulkan[] Textures { get; }
```

**Value:** [VideoTextureVulkan](Quaternar.Qaros.VideoTextureVulkan.md)\[\]

### Semaphore {#semaphore}

VkSemaphore when the frame was fetched.

```csharp
public ulong Semaphore { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### Fence {#fence}

VkFence when the frame was fetched.

```csharp
public ulong Fence { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

## Methods

### SetTextureState {#settexturestate}

Records the VkImageLayout and queue family texture `textureIndex` was left in.

```csharp
public void SetTextureState(int textureIndex, int layout, uint queueFamilyIndex)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `textureIndex` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |
| `layout` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |
| `queueFamilyIndex` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

### SetSynchronization {#setsynchronization}

Records the VkSemaphore and VkFence the frame's rendering signals.

```csharp
public void SetSynchronization(ulong semaphore, ulong fence)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `semaphore` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `fence` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
