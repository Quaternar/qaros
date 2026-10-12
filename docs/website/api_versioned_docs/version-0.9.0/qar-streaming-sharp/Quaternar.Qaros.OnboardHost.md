---
title: "OnboardHost class"
sidebar_label: "OnboardHost"
description: "The hub to code-onboard against, behind OnboardOptions.WithHost; prefer that method."
---

# OnboardHost class

The hub to code-onboard against, behind [`OnboardOptions.WithHost`](Quaternar.Qaros.OnboardOptions.md#withhost); prefer that
method.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L215)

```csharp
public sealed class OnboardHost : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → OnboardHost

## Constructors

### OnboardHost(string, ushort) {#ctor}

```csharp
public OnboardHost(string hostname, ushort port)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `hostname` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `port` | [ushort](https://learn.microsoft.com/dotnet/api/system.uint16) |  |

## Properties

### Hostname {#hostname}

A plain host name or IP address, not a URL.

```csharp
public string Hostname { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Port {#port}

The hub's onboarding port, shown with its code. Required: 0 is rejected.

```csharp
public ushort Port { get; set; }
```

**Value:** [ushort](https://learn.microsoft.com/dotnet/api/system.uint16)
