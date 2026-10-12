---
title: "RenderFrameShow class"
sidebar_label: "RenderFrameShow"
description: "Arguments of RenderSender.ShowFrame: the projection metadata the frame was rendered with. The defaults match qar_render_frame_show_default(): near 0.1, far 100, depth scale 1."
---

# RenderFrameShow class

Arguments of [`RenderSender.ShowFrame`](Quaternar.Qaros.RenderSender.md#showframe): the projection metadata the frame was
rendered with. The defaults match `qar_render_frame_show_default()`: near 0.1, far
100, depth scale 1.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1252)

```csharp
public sealed class RenderFrameShow
```

## Remarks

Report what was really used. The mixer re-projects the frame to the viewer's latest head
pose from this metadata and the depth buffer; wrong values show up as swimming or
misplaced content, not as an error.

## Constructors

### RenderFrameShow() {#ctor}

```csharp
public RenderFrameShow()
```

## Properties

### RenderedNearFar {#renderednearfar}

The near and far the frame was rendered with, in app units (app content space).

```csharp
public NearFar RenderedNearFar { get; set; }
```

**Value:** [NearFar](Quaternar.Qaros.NearFar.md)

### DepthScale {#depthscale}

Factor from the depth buffer's units to the view poses' units. The sender folds in the app scale itself; do not include it here.

```csharp
public float DepthScale { get; set; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

## Methods

### WithExtension {#withextension}

```csharp
public RenderFrameShow WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [RenderFrameShow](Quaternar.Qaros.RenderFrameShow.md)

### WithViewOverrides {#withviewoverrides}

Chains a [`RenderFrameShowViewOverrides`](Quaternar.Qaros.RenderFrameShowViewOverrides.md) carrying `viewOverrides`.

```csharp
public RenderFrameShow WithViewOverrides(params RenderFrameViewOverride[] viewOverrides)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `viewOverrides` | [RenderFrameViewOverride](Quaternar.Qaros.RenderFrameViewOverride.md)\[\] |  |

**Returns:** [RenderFrameShow](Quaternar.Qaros.RenderFrameShow.md)
