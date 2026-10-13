---
title: "Session class"
sidebar_label: "Session"
description: "An active session, the entry point to peers, GUI panels, app volumes and senders. Only Runtime.Onboard and Runtime.Rejoin produce one, and there is no getter for a \"current\" session: keep the instance"
---

# Session class

An active session, the entry point to peers, GUI panels, app volumes and senders. Only
[`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) and [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) produce one, and there is no
getter for a "current" session: keep the instance.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Session.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Session.cs#L21)

```csharp
public sealed class Session : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

The caller owns the session. [`Session.Dispose`](#dispose) tears the session down but keeps the
persisted identity, so the next start can [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) silently; erasing the
identity is [`Runtime.Forget`](Quaternar.Qaros.Runtime.md#forget). Disposing ends every subscription of the session
and waits for a handler running on another thread, so never dispose it from a QAROS
callback or event handler, nor while holding a lock a handler takes.

## Properties

| Member | Description |
|---|---|
| [Peers](#peers) |  |
| [GuiPanels](#guipanels) |  |
| [AppVolumes](#appvolumes) |  |
| [RenderSenders](#rendersenders) |  |
| [PrimarylessVideoSenders](#primarylessvideosenders) |  |

### Peers {#peers}

```csharp
public PeerControl Peers { get; }
```

**Value:** [PeerControl](Quaternar.Qaros.PeerControl.md)

### GuiPanels {#guipanels}

```csharp
public GuiPanelControl GuiPanels { get; }
```

**Value:** [GuiPanelControl](Quaternar.Qaros.GuiPanelControl.md)

### AppVolumes {#appvolumes}

```csharp
public AppVolumeControl AppVolumes { get; }
```

**Value:** [AppVolumeControl](Quaternar.Qaros.AppVolumeControl.md)

### RenderSenders {#rendersenders}

```csharp
public RenderSenderControl RenderSenders { get; }
```

**Value:** [RenderSenderControl](Quaternar.Qaros.RenderSenderControl.md)

### PrimarylessVideoSenders {#primarylessvideosenders}

```csharp
public PrimarylessVideoSenderControl PrimarylessVideoSenders { get; }
```

**Value:** [PrimarylessVideoSenderControl](Quaternar.Qaros.PrimarylessVideoSenderControl.md)

## Methods

| Member | Description |
|---|---|
| [GetId()](#getid) |  |
| [GetGraphicsDeviceId()](#getgraphicsdeviceid) | The GPU adapter the session's graphics devices were created on, or null when it is not known. It is the one [`OnboardOptions.WithGraphicsDevice`](Quaternar.Qaros.OnboardOptions.md#withgraphicsdevice) named, else the one QAROS selected for an app it launched, else the one the library picked. Create your own graphics device on it so the frames you render need no cross-GPU copy. |
| [RequestOnboardingInvite(RequestOnboardingInviteOptions?, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#requestonboardinginvite) | Asks the hub this session onboarded through for a fresh invite, to onboard another API instance (a helper process, a plugin sandbox) without a code. Blocks until the hub answers. |
| [RequestOnboardingCodes(RequestOnboardingCodesOptions?, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#requestonboardingcodes) | Ask the hub for a new set of one-time onboarding codes, for several devices to onboard at once. Codes other callers hold and the hub's onboarding screen code stay valid. The caller owns and disposes every returned invite. |
| [RequestOnboardingInviteAsync(RequestOnboardingInviteOptions?, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#requestonboardinginviteasync) | Async [`Session.RequestOnboardingInvite`](#requestonboardinginvite). Canceling `cancellationToken` cancels the task. |
| [FindRenderTarget(string)](#findrendertarget) | Finds an already-connected render target from its connection string. |
| [InviteTargetApp(string, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#invitetargetapp) | Asks this machine's hub to start the target app for one AR device and returns the target app's peer id once its peer joined the session - the id a render sender renders for. Build `connectionString` with [`TargetAppConnectionString`](Quaternar.Qaros.TargetAppConnectionString.md); a malformed one or a device kind that cannot be invited fails with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported). |
| [InviteTargetAppAsync(string, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#invitetargetappasync) | Async [`Session.InviteTargetApp`](#invitetargetapp). |
| [KillTargetApp(PeerId, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#killtargetapp) | Asks this machine's hub to stop a target app it started, with the whole pipeline feeding it. A target app this hub did not start ends in [`StatusCode.Timeout`](Quaternar.Qaros.StatusCode.md#timeout). |
| [KillTargetAppAsync(PeerId, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#killtargetappasync) | Async [`Session.KillTargetApp`](#killtargetapp). |
| [Dispose()](#dispose) | Tears the session down but keeps its persisted identity for a later [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin). Waits for subscription handlers running on other threads; do not call it from one. |

### GetId {#getid}

```csharp
public Result<SessionId> GetId()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[SessionId](Quaternar.Qaros.SessionId.md)&gt;

### GetGraphicsDeviceId {#getgraphicsdeviceid}

The GPU adapter the session's graphics devices were created on, or null when it is not
known. It is the one [`OnboardOptions.WithGraphicsDevice`](Quaternar.Qaros.OnboardOptions.md#withgraphicsdevice) named, else the one
QAROS selected for an app it launched, else the one the library picked. Create your own
graphics device on it so the frames you render need no cross-GPU copy.

```csharp
public Result<GraphicsDeviceId?> GetGraphicsDeviceId()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md)&gt;

### RequestOnboardingInvite {#requestonboardinginvite}

Asks the hub this session onboarded through for a fresh invite, to onboard another API
instance (a helper process, a plugin sandbox) without a code. Blocks until the hub
answers.

```csharp
public Result<OnboardingInvite> RequestOnboardingInvite(RequestOnboardingInviteOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RequestOnboardingInviteOptions](Quaternar.Qaros.RequestOnboardingInviteOptions.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md)&gt;

**Remarks**

The caller owns and disposes the invite. Hand it over with
[`OnboardingInvite.Serialize`](Quaternar.Qaros.OnboardingInvite.md#serialize); the other instance, with a runtime and storage
folder of its own, onboards with [`OnboardingInvite.Deserialize`](Quaternar.Qaros.OnboardingInvite.md#deserialize) and
[`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite). Invites expire: mint one when it is needed.
Fails with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported) when the hub cannot be resolved
from this session, for example while it is not connected.

### RequestOnboardingCodes {#requestonboardingcodes}

Ask the hub for a new set of one-time onboarding codes, for several devices to onboard at once.
Codes other callers hold and the hub's onboarding screen code stay valid. The caller owns and
disposes every returned invite.

```csharp
public Result<IReadOnlyList<OnboardingInvite>> RequestOnboardingCodes(RequestOnboardingCodesOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RequestOnboardingCodesOptions](Quaternar.Qaros.RequestOnboardingCodesOptions.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;IReadOnlyList&lt;[OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md)&gt;&gt;

**Remarks**

Blocks until the hub answers. Each code works once (a failed attempt also uses it up) and
is never shown on the hub's screen. Hand a device either its text
([`OnboardingInvite.GetCode`](Quaternar.Qaros.OnboardingInvite.md#getcode), for [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode)) or the
serialized invite (for [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite)). Fails with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported) for a count or lifetime out of range, or
when the hub cannot be resolved from this session.

### RequestOnboardingInviteAsync {#requestonboardinginviteasync}

Async [`Session.RequestOnboardingInvite`](#requestonboardinginvite). Canceling
`cancellationToken` cancels the task.

```csharp
public Task<Result<OnboardingInvite>> RequestOnboardingInviteAsync(RequestOnboardingInviteOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RequestOnboardingInviteOptions](Quaternar.Qaros.RequestOnboardingInviteOptions.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md)&gt;&gt;

### FindRenderTarget {#findrendertarget}

Finds an already-connected render target from its connection string.

```csharp
public Result<PeerId> FindRenderTarget(string connection)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `connection` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PeerId](Quaternar.Qaros.PeerId.md)&gt;

**Remarks**

Takes the strings [`TargetAppConnectionString`](Quaternar.Qaros.TargetAppConnectionString.md) builds. It only looks at the
peers this session knows now: it neither invites nor waits, and fails with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported) when no peer matches. To start a target
app, use [`Session.InviteTargetApp`](#invitetargetapp).

### InviteTargetApp {#invitetargetapp}

Asks this machine's hub to start the target app for one AR device and
returns the target app's peer id once its peer joined the session - the
id a render sender renders for. Build `connectionString`
with [`TargetAppConnectionString`](Quaternar.Qaros.TargetAppConnectionString.md); a malformed one or a device
kind that cannot be invited fails with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported).

```csharp
public Result<PeerId> InviteTargetApp(string connectionString, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `connectionString` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[PeerId](Quaternar.Qaros.PeerId.md)&gt;

### InviteTargetAppAsync {#invitetargetappasync}

Async [`Session.InviteTargetApp`](#invitetargetapp).

```csharp
public Task<Result<PeerId>> InviteTargetAppAsync(string connectionString, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `connectionString` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[PeerId](Quaternar.Qaros.PeerId.md)&gt;&gt;

### KillTargetApp {#killtargetapp}

Asks this machine's hub to stop a target app it started, with the whole
pipeline feeding it. A target app this hub did not start ends in
[`StatusCode.Timeout`](Quaternar.Qaros.StatusCode.md#timeout).

```csharp
public Result<Unit> KillTargetApp(PeerId targetApp, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `targetApp` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### KillTargetAppAsync {#killtargetappasync}

Async [`Session.KillTargetApp`](#killtargetapp).

```csharp
public Task<Result<Unit>> KillTargetAppAsync(PeerId targetApp, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `targetApp` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;&gt;

### Dispose {#dispose}

Tears the session down but keeps its persisted identity for a later
[`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin). Waits for subscription handlers running on other threads;
do not call it from one.

```csharp
public void Dispose()
```
