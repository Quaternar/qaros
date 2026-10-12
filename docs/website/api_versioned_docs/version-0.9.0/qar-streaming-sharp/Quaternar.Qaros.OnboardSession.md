---
title: "OnboardSession struct"
sidebar_label: "OnboardSession"
description: "What Runtime.Onboard produces. Persist OnboardSession.OnboardingId before anything else; the caller owns and disposes OnboardSession.Session."
---

# OnboardSession struct

What [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) produces. Persist [`OnboardSession.OnboardingId`](#onboardingid) before
anything else; the caller owns and disposes [`OnboardSession.Session`](#session).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L307)

```csharp
public readonly struct OnboardSession
```

## Constructors

### OnboardSession(OnboardingId, Session) {#ctor}

```csharp
public OnboardSession(OnboardingId onboardingId, Session session)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `onboardingId` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |
| `session` | [Session](Quaternar.Qaros.Session.md) |  |

## Properties

### OnboardingId {#onboardingid}

```csharp
public OnboardingId OnboardingId { get; }
```

**Value:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)

### Session {#session}

```csharp
public Session Session { get; }
```

**Value:** [Session](Quaternar.Qaros.Session.md)
