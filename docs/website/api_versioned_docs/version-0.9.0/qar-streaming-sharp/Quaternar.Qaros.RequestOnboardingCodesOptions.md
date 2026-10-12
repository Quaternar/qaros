---
title: "RequestOnboardingCodesOptions class"
sidebar_label: "RequestOnboardingCodesOptions"
description: "A request for a new set of one-time onboarding codes. Every request gets codes of its own; codes other callers hold and the hub's onboarding screen code stay valid."
---

# RequestOnboardingCodesOptions class

A request for a new set of one-time onboarding codes. Every request gets codes of its own;
codes other callers hold and the hub's onboarding screen code stay valid.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L260)

```csharp
public sealed class RequestOnboardingCodesOptions
```

## Properties

| Member | Description |
|---|---|
| [Count](#count) | How many codes, 1 to 64. |
| [Lifetime](#lifetime) | How long each code stays valid; zero takes the hub default (120 s). At most 600 s. |
| [PlannedPeerId](#plannedpeerid) | When set, a device pairing with one of these codes can only join under this peer id. |

### Count {#count}

How many codes, 1 to 64.

```csharp
public uint Count { get; set; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Lifetime {#lifetime}

How long each code stays valid; zero takes the hub default (120 s). At most 600 s.

```csharp
public TimeSpan Lifetime { get; set; }
```

**Value:** [TimeSpan](https://learn.microsoft.com/dotnet/api/system.timespan)

### PlannedPeerId {#plannedpeerid}

When set, a device pairing with one of these codes can only join under this peer id.

```csharp
public PeerId? PlannedPeerId { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)?
