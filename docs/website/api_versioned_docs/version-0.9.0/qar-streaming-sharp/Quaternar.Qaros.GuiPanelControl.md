---
title: "GuiPanelControl class"
sidebar_label: "GuiPanelControl"
description: "The session's GUI panels: shared 2D surfaces in the room showing a web page or a video stream. Reached through Session.GuiPanels."
---

# GuiPanelControl class

The session's GUI panels: shared 2D surfaces in the room showing a web page or a video
stream. Reached through [`Session.GuiPanels`](Quaternar.Qaros.Session.md#guipanels).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [GuiPanelControl.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/GuiPanelControl.cs#L30)

```csharp
public sealed class GuiPanelControl
```

## Remarks

Panel state is shared session state and eventually consistent. A change method returns
success once it has published the change; the change reaches the shared state, and every
peer's snapshots, later, and may still be refused there. A read right after a write can
return the old value: observe the result through
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken)
instead of reading back once. A change to a panel this session does not know yet fails
rather than waiting for it.

Every method throws [`ObjectDisposedException`](https://learn.microsoft.com/dotnet/api/system.objectdisposedexception) once the session is disposed.

## Methods

| Member | Description |
|---|---|
| [GetOrCreate(GuiPanelOptions)](#getorcreate) | Returns the panel identified by [`GuiPanelOptions.CommonName`](Quaternar.Qaros.GuiPanelOptions.md#commonname), creating it from `options` if it does not exist yet. |
| [SubscribeGestureEvents(GuiPanelId, GuiPanelGestureKind, EventHandler&lt;GuiPanelGestureEventArgs&gt;, CancellationToken)](#subscribegestureevents) | Subscribe to one kind of gesture the rendering peers report for one panel. Several producers publish, one per user: key lifecycles on (source peer, gesture id). Disposing the result stops delivery. |
| [SubscribeAllGestureEvents(GuiPanelId, EventHandler&lt;GuiPanelGestureEventArgs&gt;, CancellationToken)](#subscribeallgestureevents) | Subscribe to every gesture kind on one panel through one handler. Prefer this when taking more than one kind: everything arrives in the order the producer published it. |
| [UpdatePose(GuiPanelId, Pose)](#updatepose) | Moves a panel. `pose` is in room space. |
| [ChangeSize(GuiPanelId, GuiPanelSize)](#changesize) | Changes the panel's size in metres and its content scale. A fast-path change that this session's [`GuiPanelControl.GetAll`](#getall) shows at once. |
| [SetState(GuiPanelId, GuiPanelState)](#setstate) | Shows, minimizes or hides the panel for every peer. This, not the visible-to list, is how a panel is hidden. [`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed) is the same as [`GuiPanelControl.Close`](#close). |
| [Close(GuiPanelId)](#close) | Closes the panel for every peer: its [`GuiPanel.State`](Quaternar.Qaros.GuiPanel.md#state) becomes [`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed). [`GuiPanelControl.GetOrCreate`](#getorcreate) with the same common name reopens it. |
| [NavigateToUri(GuiPanelId, string)](#navigatetouri) | Points a web panel at another URI. A stream panel fails with [`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted): content type is fixed at creation. |
| [SetStream(GuiPanelId, StreamId)](#setstream) | Switches a stream panel to another video stream, which must be a video sender owned by this session. A web panel fails with [`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted): content type is fixed at creation. |
| [UpdateVisibleTo(GuiPanelId, PeerId\[\], PeerId\[\])](#updatevisibleto) | Adds and removes peers from the panel's visible-to list. An empty list means the panel is visible to every peer: removing the last peer shows it to everyone again, it never hides it. Hide a panel through its state instead. |
| [GetAll()](#getall) | Snapshot of every panel the session's shared storage holds, with this session's own pending pose and size changes applied. Blocks for a network round trip of about 2 s on every call. |
| [SubscribeUpdates()](#subscribeupdates) | Starts delivering updates of every panel through the [`GuiPanelControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(CancellationToken)](#subscribeupdates-cancellationtoken) | Starts delivering updates of every panel through the [`GuiPanelControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(EventHandler&lt;GuiPanelUpdatedEventArgs&gt;)](#subscribeupdates-eventhandler-guipanelupdatedeventargs) | Calls `handler` once for every panel already stored, then again for every change to any panel, until the result is disposed. |
| [SubscribeUpdates(EventHandler&lt;GuiPanelUpdatedEventArgs&gt;, CancellationToken)](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken) | Calls `handler` once for every panel already stored, then again for every change to any panel, until the result is disposed. |
| [WatchUpdatesAsync(CancellationToken)](#watchupdatesasync-cancellationtoken) | The snapshots of [`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken) as an async stream: every stored panel first, then every change. |
| [SubscribeUpdates(GuiPanelId)](#subscribeupdates-guipanelid) | Starts delivering one panel's updates through the [`GuiPanelControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(GuiPanelId, CancellationToken)](#subscribeupdates-guipanelid-cancellationtoken) | Starts delivering one panel's updates through the [`GuiPanelControl.Updated`](#updated) event until the result is disposed. |
| [SubscribeUpdates(GuiPanelId, EventHandler&lt;GuiPanelUpdatedEventArgs&gt;)](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs) | Calls `handler` for one panel: first with its stored state, when it already exists, then for every change to it, until the result is disposed. |
| [SubscribeUpdates(GuiPanelId, EventHandler&lt;GuiPanelUpdatedEventArgs&gt;, CancellationToken)](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken) | Calls `handler` for one panel: first with its stored state, when it already exists, then for every change to it, until the result is disposed. |
| [WatchUpdatesAsync(GuiPanelId, CancellationToken)](#watchupdatesasync-guipanelid-cancellationtoken) | The snapshots of [`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken) as an async stream. |

### GetOrCreate {#getorcreate}

Returns the panel identified by [`GuiPanelOptions.CommonName`](Quaternar.Qaros.GuiPanelOptions.md#commonname),
creating it from `options` if it does not exist yet.

```csharp
public Result<GuiPanelId> GetOrCreate(GuiPanelOptions options)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [GuiPanelOptions](Quaternar.Qaros.GuiPanelOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[GuiPanelId](Quaternar.Qaros.GuiPanelId.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `options` is null. |
| [ArgumentException](https://learn.microsoft.com/dotnet/api/system.argumentexception) | [`GuiPanelOptions.CommonName`](Quaternar.Qaros.GuiPanelOptions.md#commonname) is null, or both a URI and a stream id are set. |

**Remarks**

The id is derived from the common name alone, so the same common name gives the
same panel in every session, from every peer and after a restart: call this on every
start instead of persisting the id. The common name is the identity key and must be
ASCII; put user-facing text, which may be any UTF-8, in
[`GuiPanelOptions.DisplayName`](Quaternar.Qaros.GuiPanelOptions.md#displayname).

When an open panel of that name exists, the rest of `options` is
ignored. A closed panel is reopened, visible, with `options`.

The content is a web page ([`GuiPanelOptions.ContentUri`](Quaternar.Qaros.GuiPanelOptions.md#contenturi)) or a video stream
([`GuiPanelOptions.ContentStreamId`](Quaternar.Qaros.GuiPanelOptions.md#contentstreamid), a video sender owned by this session)
and cannot change type later; with neither the panel shows the default web page. A
panel meant only for some peers is best created with them in
[`GuiPanelOptions.VisibleToPeers`](Quaternar.Qaros.GuiPanelOptions.md#visibletopeers) rather than narrowed afterwards.

### SubscribeGestureEvents {#subscribegestureevents}

Subscribe to one kind of gesture the rendering peers report for one
panel. Several producers publish, one per user: key lifecycles on
(source peer, gesture id). Disposing the result stops delivery.

```csharp
public Result<IDisposable> SubscribeGestureEvents(GuiPanelId id, GuiPanelGestureKind gestureKind, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `gestureKind` | [GuiPanelGestureKind](Quaternar.Qaros.GuiPanelGestureKind.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelGestureEventArgs](Quaternar.Qaros.GuiPanelGestureEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

The peer that draws the panel recognises hover, tap, long press and drag from its
user's hands; positions arrive in panel-content-space pixels. Every panel reports
gestures, web panels included.

The gesture id is unique only within a source peer: two users' first drags both carry
id 1. Order holds within one producer, not across producers, and a source peer never
seen before is a new user. A live lifecycle is republished every 250 ms even when
nothing changed; treat one silent for three intervals as cancelled, which is what
recovers from a crashed producer. A missed started phase is recoverable because
every update carries its start position.

Separate per-kind subscriptions have no order relative to each other; to take more
than one kind, prefer [`GuiPanelControl.SubscribeAllGestureEvents`](#subscribeallgestureevents). The handler runs on a
library thread and an exception it throws is dropped. Disposal waits for a running
handler call as described for
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).
The default all-zero id fails with [`StatusCode.GuiPanelInvalidId`](Quaternar.Qaros.StatusCode.md#guipanelinvalidid).

### SubscribeAllGestureEvents {#subscribeallgestureevents}

Subscribe to every gesture kind on one panel through one handler. Prefer
this when taking more than one kind: everything arrives in the order
the producer published it.

```csharp
public Result<IDisposable> SubscribeAllGestureEvents(GuiPanelId id, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelGestureEventArgs](Quaternar.Qaros.GuiPanelGestureEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

A hover and the drag that grows out of the same contact keep their order here. Dispatch
on [`GuiPanelGestureEvent.Kind`](Quaternar.Qaros.GuiPanelGestureEvent.md#kind). Everything else - several producers,
lifecycle keys, threading and disposal - is as for
[`GuiPanelControl.SubscribeGestureEvents`](#subscribegestureevents).

### UpdatePose {#updatepose}

Moves a panel. `pose` is in room space.

```csharp
public Result<Unit> UpdatePose(GuiPanelId id, Pose pose)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `pose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

The pose is the midpoint of the panel's top edge, not its centre, with the front
normal along -Z. A fast-path change that this session's [`GuiPanelControl.GetAll`](#getall) shows at
once.

### ChangeSize {#changesize}

Changes the panel's size in metres and its content scale. A fast-path change that
this session's [`GuiPanelControl.GetAll`](#getall) shows at once.

```csharp
public Result<Unit> ChangeSize(GuiPanelId id, GuiPanelSize size)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `size` | [GuiPanelSize](Quaternar.Qaros.GuiPanelSize.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### SetState {#setstate}

Shows, minimizes or hides the panel for every peer. This, not the visible-to list,
is how a panel is hidden. [`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed) is the same as
[`GuiPanelControl.Close`](#close).

```csharp
public Result<Unit> SetState(GuiPanelId id, GuiPanelState state)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `state` | [GuiPanelState](Quaternar.Qaros.GuiPanelState.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### Close {#close}

Closes the panel for every peer: its [`GuiPanel.State`](Quaternar.Qaros.GuiPanel.md#state) becomes
[`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed). [`GuiPanelControl.GetOrCreate`](#getorcreate) with the same common
name reopens it.

```csharp
public Result<Unit> Close(GuiPanelId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### NavigateToUri {#navigatetouri}

Points a web panel at another URI. A stream panel fails with
[`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted): content type is fixed at creation.

```csharp
public Result<Unit> NavigateToUri(GuiPanelId id, string uri)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `uri` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### SetStream {#setstream}

Switches a stream panel to another video stream, which must be a video sender owned
by this session. A web panel fails with
[`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted): content type is fixed at creation.

```csharp
public Result<Unit> SetStream(GuiPanelId id, StreamId streamId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `streamId` | [StreamId](Quaternar.Qaros.StreamId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### UpdateVisibleTo {#updatevisibleto}

Adds and removes peers from the panel's visible-to list. An empty list means the
panel is visible to every peer: removing the last peer shows it to everyone again,
it never hides it. Hide a panel through its state instead.

```csharp
public Result<Unit> UpdateVisibleTo(GuiPanelId id, PeerId[] additions, PeerId[] removals)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `additions` | [PeerId](Quaternar.Qaros.PeerId.md)\[\] |  |
| `removals` | [PeerId](Quaternar.Qaros.PeerId.md)\[\] |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### GetAll {#getall}

Snapshot of every panel the session's shared storage holds, with this session's own
pending pose and size changes applied. Blocks for a network round trip of about 2 s
on every call.

```csharp
public Result<GuiPanel[]> GetAll()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[GuiPanel](Quaternar.Qaros.GuiPanel.md)\[\]&gt;

**Remarks**

Use it for a one-off listing. To track panels, subscribe with
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken),
which delivers the same snapshot first.

### SubscribeUpdates() {#subscribeupdates}

Starts delivering updates of every panel through the [`GuiPanelControl.Updated`](#updated) event until
the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### SubscribeUpdates(CancellationToken) {#subscribeupdates-cancellationtoken}

Starts delivering updates of every panel through the [`GuiPanelControl.Updated`](#updated) event until
the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### SubscribeUpdates(EventHandler&lt;GuiPanelUpdatedEventArgs&gt;) {#subscribeupdates-eventhandler-guipanelupdatedeventargs}

Calls `handler` once for every panel already stored, then again for
every change to any panel, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<GuiPanelUpdatedEventArgs> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelUpdatedEventArgs](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each call carries an immutable [`GuiPanel`](Quaternar.Qaros.GuiPanel.md) snapshot that is safe to keep.
Its pose is in the publishing peer's room space. Changes come from any peer or app,
including a user grabbing the panel in the room.

The handler runs on a library thread: do not block in it, do not call blocking QAROS
methods from it, and marshal to your own thread before touching single-threaded
state. An exception it throws is caught and dropped.

Disposing the result ends delivery: when `Dispose` returns, a handler call
running on another thread has finished and no further one starts. Called from inside
the handler, `Dispose` returns at once and no call starts after the current one.
Never dispose while holding a lock the handler takes: it deadlocks. Cancelling
`cancellationToken` ends delivery the same way, and
[`CancellationTokenSource.Cancel`](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtokensource.cancel#system-threading-cancellationtokensource-cancel) waits the same way.

### SubscribeUpdates(EventHandler&lt;GuiPanelUpdatedEventArgs&gt;, CancellationToken) {#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken}

Calls `handler` once for every panel already stored, then again for
every change to any panel, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelUpdatedEventArgs](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Each call carries an immutable [`GuiPanel`](Quaternar.Qaros.GuiPanel.md) snapshot that is safe to keep.
Its pose is in the publishing peer's room space. Changes come from any peer or app,
including a user grabbing the panel in the room.

The handler runs on a library thread: do not block in it, do not call blocking QAROS
methods from it, and marshal to your own thread before touching single-threaded
state. An exception it throws is caught and dropped.

Disposing the result ends delivery: when `Dispose` returns, a handler call
running on another thread has finished and no further one starts. Called from inside
the handler, `Dispose` returns at once and no call starts after the current one.
Never dispose while holding a lock the handler takes: it deadlocks. Cancelling
`cancellationToken` ends delivery the same way, and
[`CancellationTokenSource.Cancel`](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtokensource.cancel#system-threading-cancellationtokensource-cancel) waits the same way.

### WatchUpdatesAsync(CancellationToken) {#watchupdatesasync-cancellationtoken}

The snapshots of
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken)
as an async stream: every stored panel first, then every change.

```csharp
public IAsyncEnumerable<GuiPanel> WatchUpdatesAsync(CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [IAsyncEnumerable](https://learn.microsoft.com/dotnet/api/system.collections.generic.iasyncenumerable-1)&lt;[GuiPanel](Quaternar.Qaros.GuiPanel.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | Thrown on the first `MoveNextAsync` when the subscription cannot be started. |

**Remarks**

The subscription starts with the enumeration and ends when the enumerator is disposed.
Updates are queued without bound until read.

### SubscribeUpdates(GuiPanelId) {#subscribeupdates-guipanelid}

Starts delivering one panel's updates through the [`GuiPanelControl.Updated`](#updated) event until
the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(GuiPanelId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### SubscribeUpdates(GuiPanelId, CancellationToken) {#subscribeupdates-guipanelid-cancellationtoken}

Starts delivering one panel's updates through the [`GuiPanelControl.Updated`](#updated) event until
the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(GuiPanelId id, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Remarks**

Same delivery and disposal rules as
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### SubscribeUpdates(GuiPanelId, EventHandler&lt;GuiPanelUpdatedEventArgs&gt;) {#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs}

Calls `handler` for one panel: first with its stored state, when it
already exists, then for every change to it, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelUpdatedEventArgs](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md)&gt; |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Snapshots, threading, exceptions and disposal are as for
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### SubscribeUpdates(GuiPanelId, EventHandler&lt;GuiPanelUpdatedEventArgs&gt;, CancellationToken) {#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken}

Calls `handler` for one panel: first with its stored state, when it
already exists, then for every change to it, until the result is disposed.

```csharp
public Result<IDisposable> SubscribeUpdates(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `handler` | [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelUpdatedEventArgs](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `handler` is null. |

**Remarks**

Snapshots, threading, exceptions and disposal are as for
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).

### WatchUpdatesAsync(GuiPanelId, CancellationToken) {#watchupdatesasync-guipanelid-cancellationtoken}

The snapshots of
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-guipanelid-eventhandler-guipanelupdatedeventargs-cancellationtoken)
as an async stream.

```csharp
public IAsyncEnumerable<GuiPanel> WatchUpdatesAsync(GuiPanelId id, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [IAsyncEnumerable](https://learn.microsoft.com/dotnet/api/system.collections.generic.iasyncenumerable-1)&lt;[GuiPanel](Quaternar.Qaros.GuiPanel.md)&gt;

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | Thrown on the first `MoveNextAsync` when the subscription cannot be started. |

**Remarks**

The subscription starts with the enumeration and ends when the enumerator is disposed.
Updates are queued without bound until read.

## Events

### Updated {#updated}

Raised for every panel update, but only while a subscription started by one of the
`SubscribeUpdates` overloads without a handler is alive.

```csharp
public event EventHandler<GuiPanelUpdatedEventArgs>? Updated
```

**Event type:** [EventHandler](https://learn.microsoft.com/dotnet/api/system.eventhandler-1)&lt;[GuiPanelUpdatedEventArgs](Quaternar.Qaros.GuiPanelUpdatedEventArgs.md)&gt;

**Remarks**

Raised on a library thread with this control as the sender; see
[`GuiPanelControl.SubscribeUpdates`](#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken)
for what is delivered and the threading rules. Each such subscription raises the event
once per update it delivers, so overlapping subscriptions raise it more than once.
