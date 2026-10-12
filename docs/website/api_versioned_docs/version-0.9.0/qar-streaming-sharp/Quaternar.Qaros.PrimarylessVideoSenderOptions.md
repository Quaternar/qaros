---
title: "PrimarylessVideoSenderOptions class"
sidebar_label: "PrimarylessVideoSenderOptions"
description: "Arguments of PrimarylessVideoSender: application-owned video, such as GUI panel content, under a library-generated StreamId. Unlike a render sender it answers no receiver request and may exist before "
---

# PrimarylessVideoSenderOptions class

Arguments of [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md): application-owned video, such as GUI
panel content, under a library-generated [`StreamId`](Quaternar.Qaros.StreamId.md). Unlike a render sender it
answers no receiver request and may exist before any receiver connects.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1089)

```csharp
public sealed class PrimarylessVideoSenderOptions
```

## Remarks

The stream-parameter property matching [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) must be set and the other
two left null; creating the sender throws [`ArgumentException`](https://learn.microsoft.com/dotnet/api/system.argumentexception) otherwise.

## Constructors

### PrimarylessVideoSenderOptions(VideoFrameLayout) {#ctor}

```csharp
public PrimarylessVideoSenderOptions(VideoFrameLayout layout)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `layout` | [VideoFrameLayout](Quaternar.Qaros.VideoFrameLayout.md) |  |

## Properties

| Member | Description |
|---|---|
| [Layout](#layout) | Required. The views and textures the application writes; change it later with [`PrimarylessVideoSender.ChangeLayout`](Quaternar.Qaros.PrimarylessVideoSender.md#changelayout). |
| [EnableReconnects](#enablereconnects) | Defaults to true. |
| [ReconnectTimeout](#reconnecttimeout) | Defaults to 5 minutes. Must be positive and is sent in whole milliseconds; creating the sender throws [`ArgumentOutOfRangeException`](https://learn.microsoft.com/dotnet/api/system.argumentoutofrangeexception) otherwise, even with [`PrimarylessVideoSenderOptions.EnableReconnects`](#enablereconnects) false. |
| [GraphicsAPI](#graphicsapi) | Defaults to [`GraphicsAPI.Cpu`](Quaternar.Qaros.GraphicsAPI.md#cpu). |
| [StreamParamsD3D11](#streamparamsd3d11) | Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.D3D11`](Quaternar.Qaros.GraphicsAPI.md#d3d11); must be null otherwise. |
| [StreamParamsVulkan](#streamparamsvulkan) | Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.Vulkan`](Quaternar.Qaros.GraphicsAPI.md#vulkan); must be null otherwise. |
| [StreamParamsOpenGl](#streamparamsopengl) | Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.OpenGL`](Quaternar.Qaros.GraphicsAPI.md#opengl); must be null otherwise. |

### Layout {#layout}

Required. The views and textures the application writes; change it later with [`PrimarylessVideoSender.ChangeLayout`](Quaternar.Qaros.PrimarylessVideoSender.md#changelayout).

```csharp
public VideoFrameLayout Layout { get; set; }
```

**Value:** [VideoFrameLayout](Quaternar.Qaros.VideoFrameLayout.md)

### EnableReconnects {#enablereconnects}

Defaults to true.

```csharp
public bool EnableReconnects { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ReconnectTimeout {#reconnecttimeout}

Defaults to 5 minutes. Must be positive and is sent in whole milliseconds; creating
the sender throws [`ArgumentOutOfRangeException`](https://learn.microsoft.com/dotnet/api/system.argumentoutofrangeexception) otherwise, even with
[`PrimarylessVideoSenderOptions.EnableReconnects`](#enablereconnects) false.

```csharp
public TimeSpan ReconnectTimeout { get; set; }
```

**Value:** [TimeSpan](https://learn.microsoft.com/dotnet/api/system.timespan)

### GraphicsAPI {#graphicsapi}

Defaults to [`GraphicsAPI.Cpu`](Quaternar.Qaros.GraphicsAPI.md#cpu).

```csharp
public GraphicsAPI GraphicsAPI { get; set; }
```

**Value:** [GraphicsAPI](Quaternar.Qaros.GraphicsAPI.md)

### StreamParamsD3D11 {#streamparamsd3d11}

Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.D3D11`](Quaternar.Qaros.GraphicsAPI.md#d3d11); must be null otherwise.

```csharp
public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
```

**Value:** [StreamParamsD3D11](Quaternar.Qaros.StreamParamsD3D11.md)

### StreamParamsVulkan {#streamparamsvulkan}

Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.Vulkan`](Quaternar.Qaros.GraphicsAPI.md#vulkan); must be null otherwise.

```csharp
public StreamParamsVulkan? StreamParamsVulkan { get; set; }
```

**Value:** [StreamParamsVulkan](Quaternar.Qaros.StreamParamsVulkan.md)

### StreamParamsOpenGl {#streamparamsopengl}

Required when [`PrimarylessVideoSenderOptions.GraphicsAPI`](#graphicsapi) is [`GraphicsAPI.OpenGL`](Quaternar.Qaros.GraphicsAPI.md#opengl); must be null otherwise.

```csharp
public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
```

**Value:** [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md)
