---
title: "ForgetOptions class"
sidebar_label: "ForgetOptions"
---

# ForgetOptions class

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L272)

```csharp
public sealed class ForgetOptions
```

## Constructors

### ForgetOptions(OnboardingId) {#ctor}

```csharp
public ForgetOptions(OnboardingId onboardingId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `onboardingId` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |

## Properties

### OnboardingId {#onboardingid}

```csharp
public OnboardingId OnboardingId { get; set; }
```

**Value:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)

## Methods

### WithExtension {#withextension}

```csharp
public ForgetOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [ForgetOptions](Quaternar.Qaros.ForgetOptions.md)
