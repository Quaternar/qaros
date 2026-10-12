---
title: "RenderSenderOptions class"
sidebar_label: "RenderSenderOptions"
description: "Arguments of a render sender: one stream rendered for one target peer, displayed in one app volume. The defaults match qar_render_sender_init_default()."
---

# RenderSenderOptions class

Arguments of a render sender: one stream rendered for one target peer, displayed in one
app volume. The defaults match `qar_render_sender_init_default()`.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1020)

```csharp
public sealed class RenderSenderOptions
```

## Remarks

The stream-parameter property matching [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) must be set and the other
two left null; creating the sender throws [`ArgumentException`](https://learn.microsoft.com/dotnet/api/system.argumentexception) otherwise.

## Constructors

### RenderSenderOptions(PeerId, GraphicsAPI) {#ctor}

```csharp
public RenderSenderOptions(PeerId peerId, GraphicsAPI graphicsAPI)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `peerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `graphicsAPI` | [GraphicsAPI](Quaternar.Qaros.GraphicsAPI.md) |  |

## Properties

| Member | Description |
|---|---|
| [PeerId](#peerid) | Required. The peer this stream renders for - the headset or viewer that receives the frames, never this process' own peer id. Take it from a render stream request ([`RenderSenderControl.SubscribeRequests`](Quaternar.Qaros.RenderSenderControl.md#subscriberequests)) or from [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp). [`PeerId.Unique`](Quaternar.Qaros.PeerId.md#unique) belongs to no peer, so a stream created with one never produces a frame. |
| [GraphicsAPI](#graphicsapi) |  |
| [EnableAutoReconnects](#enableautoreconnects) | True (the default) keeps the sender across network drops and target restarts: frame calls fail while it is disconnected, typically with [`StatusCode.MediaStreamingStreamIsReconnecting`](Quaternar.Qaros.StatusCode.md#mediastreamingstreamisreconnecting), and recover on their own. |
| [TextureLayout](#texturelayout) | Defaults to [`FrameLayout.SideBySide`](Quaternar.Qaros.FrameLayout.md#sidebyside). |
| [FrameViews](#frameviews) | The views of one video frame; at most 8 are sent. **The order is guaranteed**: the layout ([`RenderSender`](Quaternar.Qaros.RenderSender.md) layout) and every frame the sender hands back list their views in exactly this order, each with the eye and data type declared at its position. Textures come in the order the views first use them (one per view for separated textures, one per data type for side by side and layered), and each view's texture index names its texture. A view the frame layout provides but was not declared follows the declared ones. |
| [AppVolumeId](#appvolumeid) | Required. The app volume this stream is displayed in, which places it in the shared scene and defines the app content space every pose the sender hands out is in. Create it first with [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate). Null is rejected with [`StatusCode.AppVolumeInvalidId`](Quaternar.Qaros.StatusCode.md#appvolumeinvalidid). |
| [ColorFormat](#colorformat) | Defaults to [`PixelFormat.B8G8R8A8`](Quaternar.Qaros.PixelFormat.md#b8g8r8a8). |
| [DepthFormat](#depthformat) | Defaults to [`PixelFormat.D32Float`](Quaternar.Qaros.PixelFormat.md#d32float). |
| [StreamParamsD3D11](#streamparamsd3d11) | Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.D3D11`](Quaternar.Qaros.GraphicsAPI.md#d3d11); must be null otherwise. |
| [StreamParamsVulkan](#streamparamsvulkan) | Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.Vulkan`](Quaternar.Qaros.GraphicsAPI.md#vulkan); must be null otherwise. |
| [StreamParamsOpenGl](#streamparamsopengl) | Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.OpenGL`](Quaternar.Qaros.GraphicsAPI.md#opengl); must be null otherwise. |

### PeerId {#peerid}

Required. The peer this stream renders for - the headset or viewer that
receives the frames, never this process' own peer id. Take it from a
render stream request ([`RenderSenderControl.SubscribeRequests`](Quaternar.Qaros.RenderSenderControl.md#subscriberequests))
or from [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp). [`PeerId.Unique`](Quaternar.Qaros.PeerId.md#unique)
belongs to no peer, so a stream created with one never produces a frame.

```csharp
public PeerId PeerId { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### GraphicsAPI {#graphicsapi}

```csharp
public GraphicsAPI GraphicsAPI { get; set; }
```

**Value:** [GraphicsAPI](Quaternar.Qaros.GraphicsAPI.md)

### EnableAutoReconnects {#enableautoreconnects}

True (the default) keeps the sender across network drops and target restarts: frame
calls fail while it is disconnected, typically with
[`StatusCode.MediaStreamingStreamIsReconnecting`](Quaternar.Qaros.StatusCode.md#mediastreamingstreamisreconnecting), and recover on their own.

```csharp
public bool EnableAutoReconnects { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### TextureLayout {#texturelayout}

Defaults to [`FrameLayout.SideBySide`](Quaternar.Qaros.FrameLayout.md#sidebyside).

```csharp
public FrameLayout TextureLayout { get; set; }
```

**Value:** [FrameLayout](Quaternar.Qaros.FrameLayout.md)

### FrameViews {#frameviews}

The views of one video frame; at most 8 are sent. **The order is guaranteed**: the
layout ([`RenderSender`](Quaternar.Qaros.RenderSender.md) layout) and every frame the sender
hands back list their views in exactly this order, each with the eye
and data type declared at its position. Textures come in the order the
views first use them (one per view for separated textures, one per data
type for side by side and layered), and each view's texture index names
its texture. A view the frame layout provides but was not declared
follows the declared ones.

```csharp
public RenderFrameView[] FrameViews { get; set; }
```

**Value:** [RenderFrameView](Quaternar.Qaros.RenderFrameView.md)\[\]

### AppVolumeId {#appvolumeid}

Required. The app volume this stream is displayed in, which places it in
the shared scene and defines the app content space every pose the sender
hands out is in. Create it first with [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate).
Null is rejected with [`StatusCode.AppVolumeInvalidId`](Quaternar.Qaros.StatusCode.md#appvolumeinvalidid).

```csharp
public AppVolumeId? AppVolumeId { get; set; }
```

**Value:** [AppVolumeId](Quaternar.Qaros.AppVolumeId.md)?

### ColorFormat {#colorformat}

Defaults to [`PixelFormat.B8G8R8A8`](Quaternar.Qaros.PixelFormat.md#b8g8r8a8).

```csharp
public PixelFormat ColorFormat { get; set; }
```

**Value:** [PixelFormat](Quaternar.Qaros.PixelFormat.md)

### DepthFormat {#depthformat}

Defaults to [`PixelFormat.D32Float`](Quaternar.Qaros.PixelFormat.md#d32float).

```csharp
public PixelFormat DepthFormat { get; set; }
```

**Value:** [PixelFormat](Quaternar.Qaros.PixelFormat.md)

### StreamParamsD3D11 {#streamparamsd3d11}

Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.D3D11`](Quaternar.Qaros.GraphicsAPI.md#d3d11); must be null otherwise.

```csharp
public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
```

**Value:** [StreamParamsD3D11](Quaternar.Qaros.StreamParamsD3D11.md)

### StreamParamsVulkan {#streamparamsvulkan}

Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.Vulkan`](Quaternar.Qaros.GraphicsAPI.md#vulkan); must be null otherwise.

```csharp
public StreamParamsVulkan? StreamParamsVulkan { get; set; }
```

**Value:** [StreamParamsVulkan](Quaternar.Qaros.StreamParamsVulkan.md)

### StreamParamsOpenGl {#streamparamsopengl}

Required when [`RenderSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.OpenGL`](Quaternar.Qaros.GraphicsAPI.md#opengl); must be null otherwise.

```csharp
public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
```

**Value:** [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md)
