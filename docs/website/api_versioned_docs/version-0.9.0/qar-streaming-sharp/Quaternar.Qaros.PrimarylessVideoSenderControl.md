---
title: "PrimarylessVideoSenderControl class"
sidebar_label: "PrimarylessVideoSenderControl"
description: "Creates PrimarylessVideoSenders for a session; reached through Session.PrimarylessVideoSenders."
---

# PrimarylessVideoSenderControl class

Creates [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md)s for a session; reached through
[`Session.PrimarylessVideoSenders`](Quaternar.Qaros.Session.md#primarylessvideosenders).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [VideoSender.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/VideoSender.cs#L10)

```csharp
public sealed class PrimarylessVideoSenderControl
```

## Methods

### CreateCpu {#createcpu}

Creates a CPU video sender; [`PrimarylessVideoSenderOptions.GraphicsAPI`](Quaternar.Qaros.PrimarylessVideoSenderOptions.md#graphicsapi) must be [`GraphicsAPI.Cpu`](Quaternar.Qaros.GraphicsAPI.md#cpu).

```csharp
public Result<PrimarylessVideoSender> CreateCpu(PrimarylessVideoSenderOptions options)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [PrimarylessVideoSenderOptions](Quaternar.Qaros.PrimarylessVideoSenderOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PrimarylessVideoSender](Quaternar.Qaros.PrimarylessVideoSender.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` or its layout is null. |
| [ArgumentException](https://learn.microsoft.com/dotnet/api/system.argumentexception) | The graphics API is not [`GraphicsAPI.Cpu`](Quaternar.Qaros.GraphicsAPI.md#cpu). |
| [ArgumentOutOfRangeException](https://learn.microsoft.com/dotnet/api/system.argumentoutofrangeexception) | The reconnect timeout is not positive. |

### Create {#create}

Creates a producer-owned logical video sender for
[`PrimarylessVideoSenderOptions.GraphicsAPI`](Quaternar.Qaros.PrimarylessVideoSenderOptions.md#graphicsapi), with the matching
stream-parameter extension for a GPU API.

```csharp
public Result<PrimarylessVideoSender> Create(PrimarylessVideoSenderOptions options)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [PrimarylessVideoSenderOptions](Quaternar.Qaros.PrimarylessVideoSenderOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PrimarylessVideoSender](Quaternar.Qaros.PrimarylessVideoSender.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` or its layout is null. |
| [ArgumentOutOfRangeException](https://learn.microsoft.com/dotnet/api/system.argumentoutofrangeexception) | The reconnect timeout is not positive (checked even with reconnects disabled) or does not fit in milliseconds. |

**Remarks**

Synchronous: it negotiates nothing and needs no receiver, and creates the frame
resources on the calling thread. The library generates the stream id; publish
[`PrimarylessVideoSender.GetStreamId`](Quaternar.Qaros.PrimarylessVideoSender.md#getstreamid) (for example as
[`GuiPanelOptions.ContentStreamId`](Quaternar.Qaros.GuiPanelOptions.md#contentstreamid)) so receivers can request the stream.
