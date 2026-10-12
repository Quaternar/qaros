---
title: "PeerControl class"
sidebar_label: "PeerControl"
description: "The peers of a session: every participant, devices and applications alike, this process included. Reached through Session.Peers."
---

# PeerControl class

The peers of a session: every participant, devices and applications alike, this process
included. Reached through [`Session.Peers`](Quaternar.Qaros.Session.md#peers).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [PeerControl.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/PeerControl.cs#L20)

```csharp
public sealed class PeerControl
```

## Remarks

Peers join, change and leave asynchronously, so [`PeerControl.GetAll`](#getall) is a snapshot of what
this session knows at that moment. To react to changes, subscribe with
[`PeerControl.SubscribeUpdates`](#subscribeupdates-eventhandler-peerspecupdatedeventargs).

## Methods

| Member | Description |
|---|---|
| [GetMySpec()](#getmyspec) | This process' own peer, for example to learn its peer id. |
| [GetAll()](#getall) | Every peer this session knows now, this process' own included (see [`PeerControl.GetMySpec`](#getmyspec)). |
| [UpdateDisplayName(string)](#updatedisplayname) | Changes the display name other peers see for this process; any UTF-8. |
| [SubscribeUpdates()](#subscribeupdates) | Starts delivering peer updates to [`PeerControl.Updated`](#updated) until the returned subscription is disposed. |
| [SubscribeUpdates(CancellationToken)](#subscribeupdates-cancellationtoken) | Starts delivering peer updates to [`PeerControl.Updated`](#updated) until the returned subscription is disposed or `cancellationToken` is canceled. |
| [SubscribeUpdates(EventHandler&lt;PeerSpecUpdatedEventArgs&gt;)](#subscribeupdates-eventhandler-peerspecupdatedeventargs) | Calls `handler` for every peer that joins, leaves or changes its presentation or app state, until the returned subscription is disposed. The way to learn that an invited target app joined and reached [`AppState.Running`](Quaternar.Qaros.AppState.md#running). |
| [SubscribeUpdates(EventHandler&lt;PeerSpecUpdatedEventArgs&gt;, CancellationToken)](#subscribeupdates-eventhandler-peerspecupdatedeventargs-cancellationtoken) | [`PeerControl.SubscribeUpdates`](#subscribeupdates-eventhandler-peerspecupdatedeventargs) that also ends when `cancellationToken` is canceled. Still dispose the subscription. |
| [WatchUpdatesAsync(CancellationToken)](#watchupdatesasync) | Peer updates as an async stream, buffered in arrival order, until `cancellationToken` is canceled or the enumeration is abandoned. |

### GetMySpec {#getmyspec}

This process' own peer, for example to learn its peer id.

```csharp
public Result<PeerSpec> GetMySpec()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PeerSpec](Quaternar.Qaros.PeerSpec.md)&gt;

### GetAll {#getall}

Every peer this session knows now, this process' own included (see
[`PeerControl.GetMySpec`](#getmyspec)).

```csharp
public Result<PeerSpec[]> GetAll()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PeerSpec](Quaternar.Qaros.PeerSpec.md)\[\]&gt;

### UpdateDisplayName {#updatedisplayname}

Changes the display name other peers see for this process; any UTF-8.

```csharp
public Result<Unit> UpdateDisplayName(string name)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `name` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### SubscribeUpdates() {#subscribeupdates}

Starts delivering peer updates to [`PeerControl.Updated`](#updated) until the returned subscription is
disposed.

```csharp
public Result<IDisposable> SubscribeUpdates()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

### SubscribeUpdates(CancellationToken) {#subscribeupdates-cancellationtoken}

Starts delivering peer updates to [`PeerControl.Updated`](#updated) until the returned subscription is
disposed or `cancellationToken` is canceled.

```csharp
public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

### SubscribeUpdates(EventHandler&lt;PeerSpecUpdatedEventArgs&gt;) {#subscribeupdates-eventhandler-peerspecupdatedeventargs}

Calls `handler` for every peer that joins, leaves or changes its
presentation or app state, until the returned subscription is disposed. The way to learn
that an invited target app joined and reached [`AppState.Running`](Quaternar.Qaros.AppState.md#running).

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[PeerSpecUpdatedEventArgs](Quaternar.Qaros.PeerSpecUpdatedEventArgs.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

The handler runs on a library thread: marshal to your own thread before touching
single-threaded state, and do not make blocking QAROS calls from it. An exception it
throws is swallowed.

When `Dispose` of the subscription returns, a handler call running on another thread
has finished and no further one starts. Disposing from inside the handler returns at once.
Never dispose while holding a lock the handler takes: that deadlocks.

### SubscribeUpdates(EventHandler&lt;PeerSpecUpdatedEventArgs&gt;, CancellationToken) {#subscribeupdates-eventhandler-peerspecupdatedeventargs-cancellationtoken}

[`PeerControl.SubscribeUpdates`](#subscribeupdates-eventhandler-peerspecupdatedeventargs) that also ends
when `cancellationToken` is canceled. Still dispose the subscription.

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[PeerSpecUpdatedEventArgs](Quaternar.Qaros.PeerSpecUpdatedEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

### WatchUpdatesAsync {#watchupdatesasync}

Peer updates as an async stream, buffered in arrival order, until
`cancellationToken` is canceled or the enumeration is abandoned.

```csharp
public IAsyncEnumerable<PeerSpec> WatchUpdatesAsync(CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [IAsyncEnumerable](https://learn.microsoft.com/dotnet/api/system.collections.generic.iasyncenumerable-1)&lt;[PeerSpec](Quaternar.Qaros.PeerSpec.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | The subscription could not be started. |

## Events

### Updated {#updated}

Raised for every peer update, but only while a subscription from
[`PeerControl.SubscribeUpdates`](#subscribeupdates) or [`PeerControl.SubscribeUpdates`](#subscribeupdates-cancellationtoken) is
alive: attaching a handler alone delivers nothing.

```csharp
public event EventHandler<PeerSpecUpdatedEventArgs>? Updated
```

**Event type:** [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[PeerSpecUpdatedEventArgs](Quaternar.Qaros.PeerSpecUpdatedEventArgs.md)&gt;

**Remarks**

Raised on a library thread, like every subscription handler.
