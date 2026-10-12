---
title: "OnboardCode class"
sidebar_label: "OnboardCode"
description: "Onboarding mode extension behind OnboardOptions.UseCode; prefer that method."
---

# OnboardCode class

Onboarding mode extension behind [`OnboardOptions.UseCode`](Quaternar.Qaros.OnboardOptions.md#usecode); prefer that method.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L200)

```csharp
public sealed class OnboardCode : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → OnboardCode

## Constructors

### OnboardCode(string) {#ctor}

```csharp
public OnboardCode(string code)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `code` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

## Properties

### Code {#code}

```csharp
public string Code { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)
