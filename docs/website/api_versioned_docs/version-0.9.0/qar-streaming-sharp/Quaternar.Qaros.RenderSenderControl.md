---
title: "RenderSenderControl class"
sidebar_label: "RenderSenderControl"
description: "Creates render senders and delivers render stream requests for a session; reached through Session.RenderSenders."
---

# RenderSenderControl class

Creates render senders and delivers render stream requests for a session; reached through
[`Session.RenderSenders`](Quaternar.Qaros.Session.md#rendersenders).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [RenderSenderControl.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/RenderSenderControl.cs#L12)

```csharp
public sealed class RenderSenderControl
```

## Methods

| Member | Description |
|---|---|
| [Create(RenderSenderOptions, CancellationToken)](#create) | Creates a sender that renders for [`RenderSenderOptions.PeerId`](Quaternar.Qaros.RenderSenderOptions.md#peerid) into the app volume [`RenderSenderOptions.AppVolumeId`](Quaternar.Qaros.RenderSenderOptions.md#appvolumeid), which must already exist. |
| [CreateAsync(RenderSenderOptions, CancellationToken)](#createasync) | Async [`RenderSenderControl.Create`](#create); completes once the target has requested the stream. |
| [SubscribeRequests(EventHandler&lt;RenderStreamRequestEventArgs&gt;, CancellationToken)](#subscriberequests) | Raises `handler` for each render stream request for this application's content. |

### Create {#create}

Creates a sender that renders for [`RenderSenderOptions.PeerId`](Quaternar.Qaros.RenderSenderOptions.md#peerid) into the app
volume [`RenderSenderOptions.AppVolumeId`](Quaternar.Qaros.RenderSenderOptions.md#appvolumeid), which must already exist.

```csharp
public Result<RenderSender> Create(RenderSenderOptions options, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RenderSenderOptions](Quaternar.Qaros.RenderSenderOptions.md) |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[RenderSender](Quaternar.Qaros.RenderSender.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` is null. |

**Remarks**

The sender attaches its target to the app volume, which makes the target request the
content, and the call waits for that request: it does not return until the target
asks for the stream. Bound the wait with `cancellationToken`, or use
[`RenderSenderControl.CreateAsync`](#createasync).

Create one sender per target served; which targets to serve is the application's
decision. A [`PeerId.Unique`](Quaternar.Qaros.PeerId.md#unique) target belongs to no peer, so the stream never
negotiates. A missing app volume id fails with
[`StatusCode.AppVolumeInvalidId`](Quaternar.Qaros.StatusCode.md#appvolumeinvalidid).

### CreateAsync {#createasync}

Async [`RenderSenderControl.Create`](#create); completes once the target has requested the stream.

```csharp
public Task<Result<RenderSender>> CreateAsync(RenderSenderOptions options, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RenderSenderOptions](Quaternar.Qaros.RenderSenderOptions.md) |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[RenderSender](Quaternar.Qaros.RenderSender.md)&gt;&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` is null. |

### SubscribeRequests {#subscriberequests}

Raises `handler` for each render stream request for this
application's content.

```csharp
public Result<IDisposable> SubscribeRequests(EventHandler<RenderStreamRequestEventArgs> handler, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[RenderStreamRequestEventArgs](Quaternar.Qaros.RenderStreamRequestEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

A target requests the content only once it is attached to the application's app
volume, so this does not discover targets for a new application: find those in the
peer list and create a sender for each one served, which attaches it. It sees
requests for a volume something else attached a target to. Repeated requests for a
target already served need no further sender.

The handler runs on a native thread: hand the target id to the render thread with
proper synchronization. Exceptions thrown by the handler are swallowed. Disposing the
returned subscription, or cancelling `cancellationToken`, ends it;
once `Dispose` returns no handler call is running on another thread and none
starts.
