---
title: "RenderFrameInfo class"
sidebar_label: "RenderFrameInfo"
description: "Per-frame information from a successful begin-frame. Everything it hands out is in the bound app volume's app content space."
---

# RenderFrameInfo class

Per-frame information from a successful begin-frame. Everything it hands out is in the bound app volume's app content space.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarRenderFrameInfo.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarRenderFrameInfo.cs#L22)

```csharp
public sealed class RenderFrameInfo : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

Valid only for the frame it was begun with: read the poses and FOVs before
[`RenderSender.ShowFrame`](Quaternar.Qaros.RenderSender.md#showframe), which ends that frame, then dispose it. Read after
that, the getters fail with [`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror), or return the next
frame's values once that one is begun; disposing the sender invalidates it. A view index
is a position in the sender's layout ([`RenderSender.GetLayout`](Quaternar.Qaros.RenderSender.md#getlayout)), in the order
of [`RenderSenderOptions.FrameViews`](Quaternar.Qaros.RenderSenderOptions.md#frameviews); one out of range fails with
[`StatusCode.LogicError`](Quaternar.Qaros.StatusCode.md#logicerror).

## Methods

| Member | Description |
|---|---|
| [GetViewPose(int)](#getviewpose) | The pose to render one view with, in the bound app volume's app content space (right-handed, +Y up, -Z forward, app units). The sender has already taken the viewer's room-space pose through the volume's pose, app pose and app scale, so the application renders its scene in its own coordinates. A view of eye [`VideoFrameViewEye.None`](Quaternar.Qaros.VideoFrameViewEye.md#none) gets the head pose. |
| [GetViewFov(int)](#getviewfov) | The field of view of one view, in view space, in radians. A view of eye [`VideoFrameViewEye.None`](Quaternar.Qaros.VideoFrameViewEye.md#none) gets the left eye's. |
| [Dispose()](#dispose) | Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources. |

### GetViewPose {#getviewpose}

The pose to render one view with, in the bound app volume's app content
space (right-handed, +Y up, -Z forward, app units). The sender has
already taken the viewer's room-space pose through the volume's pose, app
pose and app scale, so the application renders its scene in its own
coordinates. A view of eye [`VideoFrameViewEye.None`](Quaternar.Qaros.VideoFrameViewEye.md#none) gets the
head pose.

```csharp
public Result<Pose> GetViewPose(int viewIndex)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `viewIndex` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Pose](Quaternar.Qaros.Mathematics.Pose.md)&gt;

### GetViewFov {#getviewfov}

The field of view of one view, in view space, in radians. A view of eye [`VideoFrameViewEye.None`](Quaternar.Qaros.VideoFrameViewEye.md#none) gets the left eye's.

```csharp
public Result<Fov> GetViewFov(int viewIndex)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `viewIndex` | [int](https://learn.microsoft.com/dotnet/api/system.int32) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Fov](Quaternar.Qaros.Fov.md)&gt;

### Dispose {#dispose}

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

```csharp
public void Dispose()
```
