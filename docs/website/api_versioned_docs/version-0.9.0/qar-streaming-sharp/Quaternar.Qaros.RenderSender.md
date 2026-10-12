---
title: "RenderSender class"
sidebar_label: "RenderSender"
description: "Streams the application's rendered frames to one target peer, displayed in the bound app volume. Create one per target with RenderSenderControl.Create."
---

# RenderSender class

Streams the application's rendered frames to one target peer, displayed in the bound app
volume. Create one per target with [`RenderSenderControl.Create`](Quaternar.Qaros.RenderSenderControl.md#create).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [RenderSender.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/RenderSender.cs#L37)

```csharp
public sealed class RenderSender : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

Frame loop: [`RenderSender.BeginFrame`](#beginframe) (or [`RenderSender.BeginFrameAsync`](#beginframeasync) /
[`RenderSender.TryBeginFrame`](#trybeginframe)) hands out the per-view poses and FOVs; render with exactly
those into the frame from [`RenderSender.GetCpuFrame`](#getcpuframe), [`RenderSender.GetD3D11Frame`](#getd3d11frame),
[`RenderSender.GetOpenGlFrame`](#getopenglframe) or [`RenderSender.GetVulkanFrame`](#getvulkanframe) (the one matching the
sender's graphics API); then [`RenderSender.ShowFrame`](#showframe) with the near/far actually used. The
mixer re-projects the frame with that metadata and the depth, so wrong metadata shows as
swimming content, not as an error. Only one frame cycle is open at a time; calling
[`RenderSender.ShowFrame`](#showframe) without a begun frame fails with
[`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror).

Everything the sender hands out (view poses, hands) is in the bound app volume's app
content space, never room space. With auto-reconnects enabled, frame calls fail with
[`StatusCode.MediaStreamingStreamIsReconnecting`](Quaternar.Qaros.StatusCode.md#mediastreamingstreamisreconnecting) while the stream reconnects
and recover on their own; keep the sender.
[`StatusCode.RenderingProducerStreamIsClosed`](Quaternar.Qaros.StatusCode.md#renderingproducerstreamisclosed) from begin-frame means the
stream closed for good: dispose the sender and create a new one if the target is still
to be served.

## Methods

| Member | Description |
|---|---|
| [GetLayout()](#getlayout) | The negotiated frame layout. Available as soon as the sender exists, before the first begin-frame and while reconnecting, so size lasting resources from it. |
| [GetStreamId()](#getstreamid) |  |
| [ChangeLayout(VideoFrameLayout)](#changelayout) | Requests a layout for subsequent frames; their views and textures keep the order given in `layout`. |
| [GetCpuFrame()](#getcpuframe) | CPU-backed frame access. The frame exists from creation, so its textures, views and sizes are readable before the first begin-frame; its pixels may be written only inside a frame cycle. |
| [GetD3D11Frame()](#getd3d11frame) | D3D11-backed frame access: shared textures to render into directly. Readable before the first begin-frame, writable only inside a frame cycle. |
| [BeginFrame(CancellationToken)](#beginframe) | Blocks until a new frame may be produced and returns its per-view poses and FOVs. The poses are in the bound app volume's app content space. |
| [BeginFrameAsync(CancellationToken)](#beginframeasync) | Async [`RenderSender.BeginFrame`](#beginframe); fails the same way, including with [`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved). |
| [TryBeginFrame()](#trybeginframe) | Non-blocking [`RenderSender.BeginFrame`](#beginframe): starts the begin-frame wait on the first call and polls it on every call. The value is the frame info once the wait completed, or null while it is still pending - poll again later. A failure is a real failure, such as [`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved) (recoverable). |
| [GetOpenGlFrame()](#getopenglframe) | OpenGL-backed frame access; requires a library built with QAR_ENABLE_OPENGL (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)). Readable before the first begin-frame, writable only inside a frame cycle. |
| [GetVulkanFrame()](#getvulkanframe) | Sender-owned Vulkan frame and its current image state; requires a library built with QAR_ENABLE_VULKAN (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)). Valid until the next begin-frame or this sender's disposal. Inside a frame cycle, record final layouts, queue ownership and synchronization on it before [`RenderSender.ShowFrame`](#showframe). |
| [ShowFrame(RenderFrameShow?)](#showframe) | Submits the rendered frame. [`RenderFrameShow.RenderedNearFar`](Quaternar.Qaros.RenderFrameShow.md#renderednearfar) and any view-override pose are in app content space. |
| [GetLastHands()](#getlasthands) | The newest hands sample the target device sent since the previous call; poll once per frame. |
| [Dispose()](#dispose) | Ends the stream to the target. Every [`RenderFrameInfo`](Quaternar.Qaros.RenderFrameInfo.md) this sender handed out becomes invalid. |

### GetLayout {#getlayout}

The negotiated frame layout. Available as soon as the sender exists, before the first
begin-frame and while reconnecting, so size lasting resources from it.

```csharp
public Result<VideoFrameLayout> GetLayout()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameLayout](Quaternar.Qaros.VideoFrameLayout.md)&gt;

**Remarks**

Its views come in the order declared in [`RenderSenderOptions.FrameViews`](Quaternar.Qaros.RenderSenderOptions.md#frameviews),
or after [`RenderSender.ChangeLayout`](#changelayout) in the order of that layout, and every frame
matches it.

### GetStreamId {#getstreamid}

```csharp
public Result<StreamId> GetStreamId()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[StreamId](Quaternar.Qaros.StreamId.md)&gt;

### ChangeLayout {#changelayout}

Requests a layout for subsequent frames; their views and textures keep the order given
in `layout`.

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

Not callable inside a frame cycle: between a successful begin-frame and its
[`RenderSender.ShowFrame`](#showframe) it fails with [`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror) and changes
nothing.

### GetCpuFrame {#getcpuframe}

CPU-backed frame access. The frame exists from creation, so its textures, views and
sizes are readable before the first begin-frame; its pixels may be written only inside
a frame cycle.

```csharp
public Result<VideoFrameCpu> GetCpuFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameCpu](Quaternar.Qaros.VideoFrameCpu.md)&gt;

### GetD3D11Frame {#getd3d11frame}

D3D11-backed frame access: shared textures to render into directly. Readable before
the first begin-frame, writable only inside a frame cycle.

```csharp
public Result<VideoFrameD3D11> GetD3D11Frame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameD3D11](Quaternar.Qaros.VideoFrameD3D11.md)&gt;

**Remarks**

The sender acquires and releases the textures' keyed mutexes itself unless
[`StreamParamsD3D11.AcquireKeyedMutexSync`](Quaternar.Qaros.StreamParamsD3D11.md#acquirekeyedmutexsync) was false at creation; then the
application drives the keyed-mutex chain with the frame's synchronization data.

### BeginFrame {#beginframe}

Blocks until a new frame may be produced and returns its per-view poses
and FOVs. The poses are in the bound app volume's app content space.

```csharp
public Result<RenderFrameInfo> BeginFrame(CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[RenderFrameInfo](Quaternar.Qaros.RenderFrameInfo.md)&gt;

**Remarks**

Fails with [`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved)
while the sender's app volume does not resolve (not known to the session
yet, or removed): app content space is undefined, no frame is begun and
nothing is handed out in room space instead. That is recoverable - keep
calling, it succeeds once the volume resolves; a removed volume never
does, so recreate it. Meanwhile [`RenderSender.GetLastHands`](#getlasthands) reports both
hands untracked. The call is held about 16 ms before failing, so a retry
loop does not spin.

While the receiver has paused the stream (for example its display is not
visible) the call waits instead of failing, and the frame begins once the
receiver resumes. Cancel `cancellationToken` to stop
waiting; the call then fails. A frame still open from a previous
begin-frame without [`RenderSender.ShowFrame`](#showframe) is ended first.

Read the returned [`RenderFrameInfo`](Quaternar.Qaros.RenderFrameInfo.md) before [`RenderSender.ShowFrame`](#showframe)
and dispose it afterwards.

### BeginFrameAsync {#beginframeasync}

Async [`RenderSender.BeginFrame`](#beginframe); fails the same way, including with [`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved).

```csharp
public Task<Result<RenderFrameInfo>> BeginFrameAsync(CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[RenderFrameInfo](Quaternar.Qaros.RenderFrameInfo.md)&gt;&gt;

**Remarks**

Like [`RenderSender.BeginFrame`](#beginframe) it waits through a paused stream; cancel
`cancellationToken` to stop waiting.

### TryBeginFrame {#trybeginframe}

Non-blocking [`RenderSender.BeginFrame`](#beginframe): starts the begin-frame wait on the
first call and polls it on every call. The value is the frame info once
the wait completed, or null while it is still pending - poll again later.
A failure is a real failure, such as
[`StatusCode.RenderingProducerAppVolumeUnresolved`](Quaternar.Qaros.StatusCode.md#renderingproducerappvolumeunresolved) (recoverable).

```csharp
public Result<RenderFrameInfo?> TryBeginFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[RenderFrameInfo](Quaternar.Qaros.RenderFrameInfo.md)&gt;

**Remarks**

While the receiver has paused the stream the wait does not complete: the
call keeps succeeding with null until the receiver resumes.

### GetOpenGlFrame {#getopenglframe}

OpenGL-backed frame access; requires a library built with
QAR_ENABLE_OPENGL (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)).
Readable before the first begin-frame, writable only inside a frame cycle.

```csharp
public Result<VideoFrameOpenGl> GetOpenGlFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameOpenGl](Quaternar.Qaros.VideoFrameOpenGl.md)&gt;

### GetVulkanFrame {#getvulkanframe}

Sender-owned Vulkan frame and its current image state; requires a library
built with QAR_ENABLE_VULKAN (otherwise [`EntryPointNotFoundException`](https://learn.microsoft.com/dotnet/api/system.entrypointnotfoundexception)).
Valid until the next begin-frame or this sender's disposal. Inside a frame cycle,
record final layouts, queue ownership and synchronization on it before
[`RenderSender.ShowFrame`](#showframe).

```csharp
public Result<VideoFrameVulkan> GetVulkanFrame()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[VideoFrameVulkan](Quaternar.Qaros.VideoFrameVulkan.md)&gt;

**Remarks**

Readable from creation, so resources can be created against the real images during
initialization. Every begin-frame discards the previously returned frame: fetch it
again in each frame cycle, and do not write through an instance from an earlier
cycle. [`RenderSender.ShowFrame`](#showframe) of a Vulkan sender fails with
[`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror) when this was not called in the current cycle.

### ShowFrame {#showframe}

Submits the rendered frame. [`RenderFrameShow.RenderedNearFar`](Quaternar.Qaros.RenderFrameShow.md#renderednearfar) and any view-override pose are in app content space.

```csharp
public Result<Unit> ShowFrame(RenderFrameShow? frameShow = null)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `frameShow` | [RenderFrameShow](Quaternar.Qaros.RenderFrameShow.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Report the near/far actually rendered with; the defaults (0.1 to 100) are used when
`frameShow` is null. When a view was rendered with a pose or FOV
other than the one [`RenderFrameInfo`](Quaternar.Qaros.RenderFrameInfo.md) handed out, declare it with
[`RenderFrameShow.WithViewOverrides`](Quaternar.Qaros.RenderFrameShow.md#withviewoverrides). Fails with
[`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror) when no frame was begun.

### GetLastHands {#getlasthands}

The newest hands sample the target device sent since the previous call;
poll once per frame.

```csharp
public Result<DeviceHandsWithJoints> GetLastHands()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[DeviceHandsWithJoints](Quaternar.Qaros.DeviceHandsWithJoints.md)&gt;

**Remarks**

Every pose (the aim pose and the joint poses), joint radius and joint
linear/angular velocity is in the bound app volume's app content space,
like the view poses: right-handed, +Y up, -Z forward, app units. Samples
arrive only while the target peer is focused on the volume and not
editing it. With nothing new, the last sample is returned again while it
is younger than a second; after that, and while the app volume does not
resolve, both hands are reported untracked. Fails once the hand stream
has closed.

### Dispose {#dispose}

Ends the stream to the target. Every [`RenderFrameInfo`](Quaternar.Qaros.RenderFrameInfo.md) this sender handed
out becomes invalid.

```csharp
public void Dispose()
```
