---
title: "RenderFrameShowViewOverrides class"
sidebar_label: "RenderFrameShowViewOverrides"
description: "Extension of RenderFrameShow: the pose or field of view a view was really rendered with, when it differs from what RenderSender.BeginFrame handed out. RenderFrameShow.WithViewOverrides chains one."
---

# RenderFrameShowViewOverrides class

Extension of [`RenderFrameShow`](Quaternar.Qaros.RenderFrameShow.md): the pose or field of view a view was really
rendered with, when it differs from what [`RenderSender.BeginFrame`](Quaternar.Qaros.RenderSender.md#beginframe) handed out.
[`RenderFrameShow.WithViewOverrides`](Quaternar.Qaros.RenderFrameShow.md#withviewoverrides) chains one.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L945)

```csharp
public sealed class RenderFrameShowViewOverrides : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → RenderFrameShowViewOverrides

## Constructors

### RenderFrameShowViewOverrides() {#ctor}

```csharp
public RenderFrameShowViewOverrides()
```

## Properties

### ViewOverrides {#viewoverrides}

At most 8 are sent; the rest are silently dropped.

```csharp
public RenderFrameViewOverride[] ViewOverrides { get; set; }
```

**Value:** [RenderFrameViewOverride](Quaternar.Qaros.RenderFrameViewOverride.md)\[\]
