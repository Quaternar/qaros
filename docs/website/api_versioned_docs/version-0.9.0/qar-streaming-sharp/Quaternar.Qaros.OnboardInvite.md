---
title: "OnboardInvite class"
sidebar_label: "OnboardInvite"
description: "Onboarding mode extension behind OnboardOptions.UseInvite; prefer that method. The invite is borrowed, not owned."
---

# OnboardInvite class

Onboarding mode extension behind [`OnboardOptions.UseInvite`](Quaternar.Qaros.OnboardOptions.md#useinvite); prefer that method.
The invite is borrowed, not owned.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L234)

```csharp
public sealed class OnboardInvite : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → OnboardInvite

## Constructors

### OnboardInvite(OnboardingInvite) {#ctor}

```csharp
public OnboardInvite(OnboardingInvite invite)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `invite` | [OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md) |  |

## Properties

### Invite {#invite}

```csharp
public OnboardingInvite Invite { get; }
```

**Value:** [OnboardingInvite](Quaternar.Qaros.OnboardingInvite.md)
