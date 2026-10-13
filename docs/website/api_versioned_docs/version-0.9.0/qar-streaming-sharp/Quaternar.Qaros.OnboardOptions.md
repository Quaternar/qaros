---
title: "OnboardOptions class"
sidebar_label: "OnboardOptions"
description: "Options of Runtime.Onboard. Choose exactly one mode: OnboardOptions.UseCode or OnboardOptions.UseInvite; the last call wins."
---

# OnboardOptions class

Options of [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard). Choose exactly one mode: [`OnboardOptions.UseCode`](#usecode) or
[`OnboardOptions.UseInvite`](#useinvite); the last call wins.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L109)

```csharp
public sealed class OnboardOptions
```

## Properties

### OnboardingId {#onboardingid}

Unset to create a new identity slot; an id an earlier onboard returned re-enrolls that
slot.

```csharp
public OnboardingId? OnboardingId { get; set; }
```

**Value:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)?

### Presentation {#presentation}

```csharp
public PeerPresentationOptions? Presentation { get; set; }
```

**Value:** [PeerPresentationOptions](Quaternar.Qaros.PeerPresentationOptions.md)

## Methods

| Member | Description |
|---|---|
| [WithPresentation(PeerPresentationOptions)](#withpresentation) |  |
| [UseCode(string)](#usecode) | Onboards with a pairing code: the short code on the hub's onboarding screen, or the text of any code the hub minted ([`OnboardingInvite.GetCode`](Quaternar.Qaros.OnboardingInvite.md#getcode)). Without [`OnboardOptions.WithHost`](#withhost) it finds this user's hub on this PC by itself. |
| [UseInvite(OnboardingInvite)](#useinvite) | Onboards from a full invite: one a sibling instance obtained with [`Session.RequestOnboardingInvite`](Quaternar.Qaros.Session.md#requestonboardinginvite), or the launch invite QAROS writes as the first stdin line of an API app it starts ([`OnboardingInvite.Deserialize`](Quaternar.Qaros.OnboardingInvite.md#deserialize)). No discovery, no code. |
| [WithHost(string, ushort)](#withhost) | Code onboarding against the hub at `hostname`:`port`. The port is required: hubs claim their onboarding port from a range, so there is no default. Without WithHost, code onboarding finds this user's hub on its own. |
| [WithPeerId(PeerId)](#withpeerid) | Join the session under `peerId` instead of a fresh id; it is persisted with the slot and kept by every later rejoin. For a process whose peer id was planned by someone else, such as the runtime launcher. |
| [WithGraphicsDevice(GraphicsDeviceId)](#withgraphicsdevice) | Create the session's GPU devices on this adapter. |
| [WithExtension(Structure)](#withextension) |  |

### WithPresentation {#withpresentation}

```csharp
public OnboardOptions WithPresentation(PeerPresentationOptions presentation)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `presentation` | [PeerPresentationOptions](Quaternar.Qaros.PeerPresentationOptions.md) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

### UseCode {#usecode}

Onboards with a pairing code: the short code on the hub's onboarding screen, or the text
of any code the hub minted ([`OnboardingInvite.GetCode`](Quaternar.Qaros.OnboardingInvite.md#getcode)). Without
[`OnboardOptions.WithHost`](#withhost) it finds this user's hub on this PC by itself.

```csharp
public OnboardOptions UseCode(string code)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `code` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

**Remarks**

The code is a secret: do not log it or pass it on a command line. Replaces an earlier
mode, including a [`OnboardOptions.WithHost`](#withhost) made before this call.

### UseInvite {#useinvite}

Onboards from a full invite: one a sibling instance obtained with
[`Session.RequestOnboardingInvite`](Quaternar.Qaros.Session.md#requestonboardinginvite), or the launch invite QAROS writes as the
first stdin line of an API app it starts ([`OnboardingInvite.Deserialize`](Quaternar.Qaros.OnboardingInvite.md#deserialize)).
No discovery, no code.

```csharp
public OnboardOptions UseInvite(OnboardingInvite invite)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `invite` | [OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

**Remarks**

The invite is borrowed: the caller keeps ownership and must not dispose it before the
onboard call has completed. Replaces an earlier mode.

### WithHost {#withhost}

Code onboarding against the hub at `hostname`:`port`.
The port is required: hubs claim their onboarding port from a range, so there is no
default. Without WithHost, code onboarding finds this user's hub on its own.

```csharp
public OnboardOptions WithHost(string hostname, ushort port)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `hostname` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `port` | [ushort](https://learn.microsoft.com/dotnet/api/system.uint16) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

**Exceptions**

| Exception | Condition |
|---|---|
| [InvalidOperationException](https://learn.microsoft.com/dotnet/api/system.invalidoperationexception) | The mode is an invite ([`OnboardOptions.UseInvite`](#useinvite)). |

### WithPeerId {#withpeerid}

Join the session under `peerId` instead of a fresh id;
it is persisted with the slot and kept by every later rejoin. For a
process whose peer id was planned by someone else, such as the runtime
launcher.

```csharp
public OnboardOptions WithPeerId(PeerId peerId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `peerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

### WithGraphicsDevice {#withgraphicsdevice}

Create the session's GPU devices on this adapter.

```csharp
public OnboardOptions WithGraphicsDevice(GraphicsDeviceId graphicsDevice)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `graphicsDevice` | [GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)

### WithExtension {#withextension}

```csharp
public OnboardOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [OnboardOptions](Quaternar.Qaros.OnboardOptions.md)
