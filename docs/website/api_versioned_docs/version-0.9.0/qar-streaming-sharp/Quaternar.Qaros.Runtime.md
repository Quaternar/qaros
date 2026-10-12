---
title: "Runtime class"
sidebar_label: "Runtime"
description: "Hosts the identity slots a process onboards with and produces its sessions. Create it after Library.Init; every Session comes from Runtime.Onboard or Runtime.Rejoin on it."
---

# Runtime class

Hosts the identity slots a process onboards with and produces its sessions. Create it after
[`Library.Init`](Quaternar.Qaros.Library.md#init); every [`Session`](Quaternar.Qaros.Session.md) comes from [`Runtime.Onboard`](#onboard) or
[`Runtime.Rejoin`](#rejoin) on it.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Runtime.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Runtime.cs#L19)

```csharp
public sealed class Runtime : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

The canonical startup: try [`Runtime.Rejoin`](#rejoin) with the persisted
[`OnboardingId`](Quaternar.Qaros.OnboardingId.md); on [`StatusCode.OnboardingSessionNotFound`](Quaternar.Qaros.StatusCode.md#onboardingsessionnotfound) run
[`Runtime.Onboard`](#onboard) and persist the id it returns. Dispose sessions before the runtime,
and the runtime before [`Library.Destroy`](Quaternar.Qaros.Library.md#destroy).

## Methods

| Member | Description |
|---|---|
| [Create(RuntimeOptions?)](#create) | Creates the runtime. Set [`RuntimeOptions.RuntimeBinariesFolderPath`](Quaternar.Qaros.RuntimeOptions.md#runtimebinariesfolderpath) to the SDK `bin` folder the native library was loaded from: left empty, sessions still work but [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp) never completes. |
| [Rejoin(RejoinOptions, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#rejoin) | Restores the session last persisted for [`RejoinOptions.OnboardingId`](Quaternar.Qaros.RejoinOptions.md#onboardingid). Pops no UI, so call it optimistically on every start. Blocks until the restore validated or failed. |
| [RejoinAsync(RejoinOptions, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#rejoinasync) | Async [`Runtime.Rejoin`](#rejoin): returns at once, and the task completes on a library thread with the same results. |
| [Onboard(OnboardOptions, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#onboard) | Runs full onboarding with the mode chosen by [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode) or [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite), and blocks until it succeeded or failed. Persist the returned [`OnboardSession.OnboardingId`](Quaternar.Qaros.OnboardSession.md#onboardingid): it is the ticket for every later [`Runtime.Rejoin`](#rejoin) and [`Runtime.Forget`](#forget). |
| [OnboardAsync(OnboardOptions, IProgress&lt;ActionProgress&gt;?, CancellationToken)](#onboardasync) | Async [`Runtime.Onboard`](#onboard): returns at once and keeps the UI responsive; the task completes with the same results. Canceling `cancellationToken` cancels the task. |
| [Forget(ForgetOptions)](#forget) | Erases everything stored for an identity slot: certificate, persisted peer id and session state. Afterwards only a full [`Runtime.Onboard`](#onboard) gets back in. Needs no session, so it also serves a "forget this hub" action while disconnected. |
| [Dispose()](#dispose) | Releases the runtime. Dispose its sessions first. |

### Create {#create}

Creates the runtime. Set [`RuntimeOptions.RuntimeBinariesFolderPath`](Quaternar.Qaros.RuntimeOptions.md#runtimebinariesfolderpath) to the
SDK `bin` folder the native library was loaded from: left empty, sessions still
work but [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp) never completes.

```csharp
public static Result<Runtime> Create(RuntimeOptions? options = null)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RuntimeOptions](Quaternar.Qaros.RuntimeOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Runtime](Quaternar.Qaros.Runtime.md)&gt;

### Rejoin {#rejoin}

Restores the session last persisted for [`RejoinOptions.OnboardingId`](Quaternar.Qaros.RejoinOptions.md#onboardingid). Pops no
UI, so call it optimistically on every start. Blocks until the restore validated or
failed.

```csharp
public Result<Session> Rejoin(RejoinOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RejoinOptions](Quaternar.Qaros.RejoinOptions.md) | The onboarding id to restore, and optional presentation and GPU. |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; | Optional progress reports. |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) | Cancels the restore; the call then returns a failure. |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Session](Quaternar.Qaros.Session.md)&gt;

**Remarks**

The session keeps the peer id the onboard persisted. Presentation fields left null keep
their persisted values.

Expected failures: [`StatusCode.OnboardingSessionNotFound`](Quaternar.Qaros.StatusCode.md#onboardingsessionnotfound) means this slot has
nothing persisted here, fall back to [`Runtime.Onboard`](#onboard).
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported) means the onboarding id is
[`OnboardingId.Default`](Quaternar.Qaros.OnboardingId.md#default).

`progress` is reported from a library thread while this call blocks.
A [`Progress<T>`](https://learn.microsoft.com/dotnet/api/system.progress-1) created on a UI thread posts there, so do not block that
thread on this call if you want to see the reports.

### RejoinAsync {#rejoinasync}

Async [`Runtime.Rejoin`](#rejoin): returns at once, and the task completes on a library thread
with the same results.

```csharp
public Task<Result<Session>> RejoinAsync(RejoinOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [RejoinOptions](Quaternar.Qaros.RejoinOptions.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[Session](Quaternar.Qaros.Session.md)&gt;&gt;

### Onboard {#onboard}

Runs full onboarding with the mode chosen by [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode) or
[`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite), and blocks until it succeeded or failed. Persist
the returned [`OnboardSession.OnboardingId`](Quaternar.Qaros.OnboardSession.md#onboardingid): it is the ticket for every later
[`Runtime.Rejoin`](#rejoin) and [`Runtime.Forget`](#forget).

```csharp
public Result<OnboardSession> Onboard(OnboardOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [OnboardOptions](Quaternar.Qaros.OnboardOptions.md) | The onboarding mode and its optional extensions. |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; | Optional progress reports, for an onboarding UI. |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) | Cancels the onboarding; the call then returns a failure. |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardSession](Quaternar.Qaros.OnboardSession.md)&gt;

**Remarks**

With [`OnboardOptions.OnboardingId`](Quaternar.Qaros.OnboardOptions.md#onboardingid) unset a new identity slot is created; set
to an id an earlier onboard returned, that slot is re-enrolled.

Expected failures: [`StatusCode.PakeError`](Quaternar.Qaros.StatusCode.md#pakeerror) means the code was wrong or
expired, or the invite was rejected or expired: re-prompt for a code or mint a fresh
invite. [`StatusCode.OnboardingHubUnreachable`](Quaternar.Qaros.StatusCode.md#onboardinghubunreachable) means no hub answered: check
that it runs and pass its host and port with [`OnboardOptions.WithHost`](Quaternar.Qaros.OnboardOptions.md#withhost).
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported) means a zero
[`OnboardOptions.WithPeerId`](Quaternar.Qaros.OnboardOptions.md#withpeerid) id, a host port of 0, or a GPU the session
cannot create its devices on.

`progress` is reported from a library thread while this call blocks.

### OnboardAsync {#onboardasync}

Async [`Runtime.Onboard`](#onboard): returns at once and keeps the UI responsive; the task
completes with the same results. Canceling `cancellationToken` cancels
the task.

```csharp
public Task<Result<OnboardSession>> OnboardAsync(OnboardOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [OnboardOptions](Quaternar.Qaros.OnboardOptions.md) |  |
| `progress` | [IProgress](https://learn.microsoft.com/dotnet/api/system.iprogress-1)&lt;[ActionProgress](Quaternar.Qaros.ActionProgress.md)&gt; |  |
| `cancellationToken` | [CancellationToken](https://learn.microsoft.com/dotnet/api/system.threading.cancellationtoken) |  |

**Returns:** [Task](https://learn.microsoft.com/dotnet/api/system.threading.tasks.task-1)&lt;[Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardSession](Quaternar.Qaros.OnboardSession.md)&gt;&gt;

### Forget {#forget}

Erases everything stored for an identity slot: certificate, persisted peer id and session
state. Afterwards only a full [`Runtime.Onboard`](#onboard) gets back in. Needs no session, so it
also serves a "forget this hub" action while disconnected.

```csharp
public Result<Unit> Forget(ForgetOptions options)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [ForgetOptions](Quaternar.Qaros.ForgetOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

Fails with [`StatusCode.OnboardingSessionStillActive`](Quaternar.Qaros.StatusCode.md#onboardingsessionstillactive) while a session of the
slot is alive: dispose it first. A slot with nothing persisted fails with
[`StatusCode.OnboardingSessionNotFound`](Quaternar.Qaros.StatusCode.md#onboardingsessionnotfound).

### Dispose {#dispose}

Releases the runtime. Dispose its sessions first.

```csharp
public void Dispose()
```
