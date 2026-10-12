---
title: "Structure class"
sidebar_label: "Structure"
description: "Base of the extension structures chained onto an options object with its WithExtension method. The options are copied into native memory for the duration of the call that takes them."
---

# Structure class

Base of the extension structures chained onto an options object with its
`WithExtension` method. The options are copied into native memory for the duration
of the call that takes them.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L671)

```csharp
public abstract class Structure
```

**Derived:** [GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md), [OnboardCode](Quaternar.Qaros.OnboardCode.md), [OnboardHost](Quaternar.Qaros.OnboardHost.md), [OnboardInvite](Quaternar.Qaros.OnboardInvite.md), [OnboardPeerId](Quaternar.Qaros.OnboardPeerId.md), [RenderFrameShowViewOverrides](Quaternar.Qaros.RenderFrameShowViewOverrides.md), [StreamParamsD3D11](Quaternar.Qaros.StreamParamsD3D11.md), [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md), [StreamParamsVulkan](Quaternar.Qaros.StreamParamsVulkan.md)

## Remarks

An instance belongs to one chain at a time: appending it to a second options object cuts
off whatever followed it in the first, so create a new instance per options object.

## Constructors

### Structure(StructureType) {#ctor}

```csharp
protected Structure(StructureType type)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `type` | [StructureType](Quaternar.Qaros.StructureType.md) |  |

## Properties

### Type {#type}

```csharp
public StructureType Type { get; }
```

**Value:** [StructureType](Quaternar.Qaros.StructureType.md)
