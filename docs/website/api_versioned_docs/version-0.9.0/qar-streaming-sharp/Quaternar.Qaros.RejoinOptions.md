---
title: "RejoinOptions class"
sidebar_label: "RejoinOptions"
description: "Options of Runtime.Rejoin."
---

# RejoinOptions class

Options of [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L71)

```csharp
public sealed class RejoinOptions
```

## Constructors

### RejoinOptions(OnboardingId) {#ctor}

```csharp
public RejoinOptions(OnboardingId onboardingId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `onboardingId` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |

## Properties

### OnboardingId {#onboardingid}

The id a previous onboard returned. [`OnboardingId.Default`](Quaternar.Qaros.OnboardingId.md#default) is rejected.

```csharp
public OnboardingId OnboardingId { get; set; }
```

**Value:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)

### Presentation {#presentation}

Presentation updates; null fields keep the persisted values.

```csharp
public PeerPresentationOptions? Presentation { get; set; }
```

**Value:** [PeerPresentationOptions](Quaternar.Qaros.PeerPresentationOptions.md)

## Methods

| Member | Description |
|---|---|
| [WithPresentation(PeerPresentationOptions)](#withpresentation) |  |
| [WithGraphicsDevice(GraphicsDeviceId)](#withgraphicsdevice) | Create the session's GPU devices on this adapter. |
| [WithExtension(Structure)](#withextension) |  |

### WithPresentation {#withpresentation}

```csharp
public RejoinOptions WithPresentation(PeerPresentationOptions presentation)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `presentation` | [PeerPresentationOptions](Quaternar.Qaros.PeerPresentationOptions.md) |  |

**Returns:** [RejoinOptions](Quaternar.Qaros.RejoinOptions.md)

### WithGraphicsDevice {#withgraphicsdevice}

Create the session's GPU devices on this adapter.

```csharp
public RejoinOptions WithGraphicsDevice(GraphicsDeviceId graphicsDevice)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `graphicsDevice` | [GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md) |  |

**Returns:** [RejoinOptions](Quaternar.Qaros.RejoinOptions.md)

### WithExtension {#withextension}

```csharp
public RejoinOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [RejoinOptions](Quaternar.Qaros.RejoinOptions.md)
