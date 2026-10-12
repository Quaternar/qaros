---
title: "PrimarylessVideoSender class"
sidebar_label: "PrimarylessVideoSender"
description: "Application-owned video stream, such as GUI panel content, that exists before any receiver connects and is delivered to every receiver that requests its stream id."
---

# PrimarylessVideoSender class

Application-owned video stream, such as GUI panel content, that exists before any receiver
connects and is delivered to every receiver that requests its stream id.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [PrimarylessVideoSender.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/PrimarylessVideoSender.cs#L26)

```csharp
public sealed class PrimarylessVideoSender : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

Unlike [`RenderSender`](Quaternar.Qaros.RenderSender.md), which answers one target's render request and hands
out poses and FOVs to render with, this sender carries an arbitrary layout and no render
metadata. Use [`RenderSender`](Quaternar.Qaros.RenderSender.md) for remote-rendered content shown in an app
volume, this one for video the application produces on its own.

Frame cycle: [`PrimarylessVideoSender.TryBeginFrame`](#trybeginframe) until it returns true, write the frame
([`PrimarylessVideoSender.GetCpuFrame`](#getcpuframe), [`PrimarylessVideoSender.GetD3D11Frame`](#getd3d11frame), [`PrimarylessVideoSender.GetOpenGlFrame`](#getopenglframe) or
[`PrimarylessVideoSender.GetVulkanFrame`](#getvulkanframe), matching the sender's graphics API), then
[`PrimarylessVideoSender.ShowFrame`](#showframe). The frame exists from creation and may be inspected outside a
cycle; its contents and synchronization may be written only inside one. Its texture
handles stay stable until a layout change, so they can be cached between cycles.

## Methods

| Member | Description |
|---|---|
| [GetStreamId()](#getstreamid) |  |
| [TryBeginFrame()](#trybeginframe) | Non-blocking: true when a frame cycle was begun for the receivers ready at this moment; false when no cycle exists yet, so poll again later. |
| [GetLayout()](#getlayout) |  |
| [ChangeLayout(VideoFrameLayout)](#changelayout) | Replaces the frame layout. The new frame resources are created synchronously on the calling thread; if that fails, the previous layout and frame stay usable. |
| [GetCpuFrame()](#getcpuframe) |  |
| [GetD3D11Frame()](#getd3d11frame) |  |
| [GetOpenGlFrame()](#getopenglframe) | Requires a library built with QAR_ENABLE_OPENGL (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)). |
| [GetVulkanFrame()](#getvulkanframe) | Sender-owned Vulkan frame; requires a library built with QAR_ENABLE_VULKAN (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)). Valid until the next call or this sender's disposal. |
| [ShowFrame()](#showframe) | Sends the written frame to the receivers reserved by [`PrimarylessVideoSender.TryBeginFrame`](#trybeginframe) and ends the cycle. |
| [Dispose()](#dispose) | Closes the stream to every receiver, draining frames still being sent first. |

### GetStreamId {#getstreamid}

```csharp
public Result<StreamId> GetStreamId()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[StreamId](Quaternar.Qaros.StreamId.md)&gt;

### TryBeginFrame {#trybeginframe}

Non-blocking: true when a frame cycle was begun for the receivers ready at this
moment; false when no cycle exists yet, so poll again later.

```csharp
public Result<bool> TryBeginFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[bool](https://learn.microsoft.com/dotnet/api/system.boolean)&gt;

**Remarks**

Receivers that become ready afterwards get the next cycle. Recoverable failures, to
keep polling through: [`StatusCode.MediaStreamingNoActiveTransfers`](Quaternar.Qaros.StatusCode.md#mediastreamingnoactivetransfers) while
no receiver is connected (the normal state before the first one requests the
stream), [`StatusCode.MediaStreamingStreamIsPaused`](Quaternar.Qaros.StatusCode.md#mediastreamingstreamispaused) while every receiver
has paused it, and [`StatusCode.MediaStreamingStreamIsReconnecting`](Quaternar.Qaros.StatusCode.md#mediastreamingstreamisreconnecting) while
every receiver is reconnecting. Calling it again before [`PrimarylessVideoSender.ShowFrame`](#showframe) fails
with [`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror): only one cycle may be open.

### GetLayout {#getlayout}

```csharp
public Result<VideoFrameLayout> GetLayout()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameLayout](Quaternar.Qaros.VideoFrameLayout.md)&gt;

### ChangeLayout {#changelayout}

Replaces the frame layout. The new frame resources are created synchronously on the
calling thread; if that fails, the previous layout and frame stay usable.

```csharp
public Result<Unit> ChangeLayout(VideoFrameLayout layout)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `layout` | [VideoFrameLayout](Quaternar.Qaros.VideoFrameLayout.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `layout` is null. |

**Remarks**

Texture handles cached from an earlier frame do not survive a successful change: fetch
the frame again.

### GetCpuFrame {#getcpuframe}

```csharp
public Result<VideoFrameCpu> GetCpuFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameCpu](Quaternar.Qaros.VideoFrameCpu.md)&gt;

### GetD3D11Frame {#getd3d11frame}

```csharp
public Result<VideoFrameD3D11> GetD3D11Frame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameD3D11](Quaternar.Qaros.VideoFrameD3D11.md)&gt;

### GetOpenGlFrame {#getopenglframe}

Requires a library built with QAR_ENABLE_OPENGL (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)).

```csharp
public Result<VideoFrameOpenGl> GetOpenGlFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameOpenGl](Quaternar.Qaros.VideoFrameOpenGl.md)&gt;

### GetVulkanFrame {#getvulkanframe}

Sender-owned Vulkan frame; requires a library built with QAR_ENABLE_VULKAN
(otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)). Valid until the next
call or this sender's disposal.

```csharp
public Result<VideoFrameVulkan> GetVulkanFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameVulkan](Quaternar.Qaros.VideoFrameVulkan.md)&gt;

**Remarks**

Inside a frame cycle, record on it the layout and queue family each image was left in
([`VideoFrameVulkan.SetTextureState`](Quaternar.Qaros.VideoFrameVulkan.md#settexturestate)) and the synchronization the rendering
signals ([`VideoFrameVulkan.SetSynchronization`](Quaternar.Qaros.VideoFrameVulkan.md#setsynchronization)) before
[`PrimarylessVideoSender.ShowFrame`](#showframe). [`PrimarylessVideoSender.ShowFrame`](#showframe) fails with
[`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror) when this was not called in the current cycle.

### ShowFrame {#showframe}

Sends the written frame to the receivers reserved by [`PrimarylessVideoSender.TryBeginFrame`](#trybeginframe) and
ends the cycle.

```csharp
public Result<Unit> ShowFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Succeeds when at least one reserved receiver got the frame; fails with
[`StatusCode.MediaStreamingNoActiveTransfers`](Quaternar.Qaros.StatusCode.md#mediastreamingnoactivetransfers) when all of them were lost
before delivery.

### Dispose {#dispose}

Closes the stream to every receiver, draining frames still being sent first.

```csharp
public void Dispose()
```
