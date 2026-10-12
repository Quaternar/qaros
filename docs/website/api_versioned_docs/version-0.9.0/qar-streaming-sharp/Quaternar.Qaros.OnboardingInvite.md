---
title: "OnboardingInvite class"
sidebar_label: "OnboardingInvite"
description: "A one-time onboarding invite minted by the hub: everything another instance needs to onboard with OnboardOptions.UseInvite, or a code to hand out by OnboardingInvite.GetCode."
---

# OnboardingInvite class

A one-time onboarding invite minted by the hub: everything another instance needs to
onboard with [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite), or a code to hand out by
[`OnboardingInvite.GetCode`](#getcode).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingInvite.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingInvite.cs#L16)

```csharp
public sealed class OnboardingInvite : IDisposable
```

**Implements:** [IDisposable](https://learn.microsoft.com/dotnet/api/system.idisposable)

## Remarks

The caller owns every instance and disposes it; an onboard only borrows it. Invites expire
([`OnboardingInvite.GetExpiresUnix`](#getexpiresunix)) and work once: mint them when needed and do not stockpile
them. They carry a secret, so never log their bytes or code.

## Methods

| Member | Description |
|---|---|
| [IsValid()](#isvalid) |  |
| [GetMethod()](#getmethod) |  |
| [GetExpiresUnix()](#getexpiresunix) | Expiry in Unix seconds, so a UI can warn before handing out a stale invite. |
| [GetCode()](#getcode) | The code text to onboard with (OnboardCode). It names its own invitation, so it works for any code the hub minted. It is a secret: never log it. |
| [Serialize()](#serialize) | The invite as bytes for your IPC boundary (pipe, shared memory, config field); the receiving instance turns them back with [`OnboardingInvite.Deserialize`](#deserialize). |
| [Deserialize(byte\[\])](#deserialize) | Parses invite bytes from [`OnboardingInvite.Serialize`](#serialize), or the launch invite QAROS writes as the first stdin line of an API app it starts: pass that JSON line's UTF-8 bytes unchanged. Bytes that are not an invite fail with [`StatusCode.PakeError`](Quaternar.Qaros.StatusCode.md#pakeerror). |
| [Dispose()](#dispose) | Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources. |

### IsValid {#isvalid}

```csharp
public bool IsValid()
```

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### GetMethod {#getmethod}

```csharp
public Result<OnboardingMethod> GetMethod()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardingMethod](Quaternar.Qaros.OnboardingMethod.md)&gt;

### GetExpiresUnix {#getexpiresunix}

Expiry in Unix seconds, so a UI can warn before handing out a stale invite.

```csharp
public Result<long> GetExpiresUnix()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[long](https://learn.microsoft.com/dotnet/api/system.int64)&gt;

### GetCode {#getcode}

The code text to onboard with (OnboardCode). It names its own invitation, so it works for any
code the hub minted. It is a secret: never log it.

```csharp
public Result<string> GetCode()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[string](https://learn.microsoft.com/dotnet/api/system.string)&gt;

**Remarks**

Only a [`OnboardingMethod.Pake`](Quaternar.Qaros.OnboardingMethod.md#pake) invite has one; any other fails with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported).

### Serialize {#serialize}

The invite as bytes for your IPC boundary (pipe, shared memory, config field); the
receiving instance turns them back with [`OnboardingInvite.Deserialize`](#deserialize).

```csharp
public Result<byte[]> Serialize()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[byte](https://learn.microsoft.com/dotnet/api/system.byte)\[\]&gt;

### Deserialize {#deserialize}

Parses invite bytes from [`OnboardingInvite.Serialize`](#serialize), or the launch invite QAROS writes as
the first stdin line of an API app it starts: pass that JSON line's UTF-8 bytes unchanged.
Bytes that are not an invite fail with [`StatusCode.PakeError`](Quaternar.Qaros.StatusCode.md#pakeerror).

```csharp
public static Result<OnboardingInvite> Deserialize(byte[] inviteData)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `inviteData` | [byte](https://learn.microsoft.com/dotnet/api/system.byte)\[\] |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md)&gt;

### Dispose {#dispose}

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

```csharp
public void Dispose()
```
