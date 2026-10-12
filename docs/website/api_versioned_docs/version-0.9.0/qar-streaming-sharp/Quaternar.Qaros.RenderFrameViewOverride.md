---
title: "RenderFrameViewOverride struct"
sidebar_label: "RenderFrameViewOverride"
description: "The pose and/or field of view one view was really rendered with, when it deviates from what RenderSender.BeginFrame handed out. Only the parts whose flag is set are used."
---

# RenderFrameViewOverride struct

The pose and/or field of view one view was really rendered with, when it deviates from
what [`RenderSender.BeginFrame`](Quaternar.Qaros.RenderSender.md#beginframe) handed out. Only the parts whose flag is set are
used.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1970)

```csharp
public readonly struct RenderFrameViewOverride
```

## Constructors

### RenderFrameViewOverride(nuint, bool, bool, Pose, Fov) {#ctor}

```csharp
public RenderFrameViewOverride(nuint viewIndex, bool overridePose, bool overrideFov, Pose pose, Fov fov)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `viewIndex` | [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr) |  |
| `overridePose` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `overrideFov` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `pose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |
| `fov` | [Fov](Quaternar.Qaros.Fov.md) |  |

## Properties

| Member | Description |
|---|---|
| [ViewIndex](#viewindex) | Index of the view in the sender's layout order. |
| [OverridePose](#overridepose) |  |
| [OverrideFov](#overridefov) |  |
| [Pose](#pose) | The pose the view was really rendered with, in the bound app volume's app content space - the space [`RenderFrameInfo.GetViewPose`](Quaternar.Qaros.RenderFrameInfo.md#getviewpose) hands it out in. |
| [Fov](#fov) | In view space, radians. |

### ViewIndex {#viewindex}

Index of the view in the sender's layout order.

```csharp
public nuint ViewIndex { get; }
```

**Value:** [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr)

### OverridePose {#overridepose}

```csharp
public bool OverridePose { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### OverrideFov {#overridefov}

```csharp
public bool OverrideFov { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### Pose {#pose}

The pose the view was really rendered with, in the bound app volume's
app content space - the space [`RenderFrameInfo.GetViewPose`](Quaternar.Qaros.RenderFrameInfo.md#getviewpose)
hands it out in.

```csharp
public Pose Pose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Fov {#fov}

In view space, radians.

```csharp
public Fov Fov { get; }
```

**Value:** [Fov](Quaternar.Qaros.Fov.md)
