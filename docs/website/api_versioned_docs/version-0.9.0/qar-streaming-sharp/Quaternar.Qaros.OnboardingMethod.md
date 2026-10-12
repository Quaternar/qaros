---
title: "OnboardingMethod enum"
sidebar_label: "OnboardingMethod"
description: "Which onboarding path an onboarding invite carries. More may be added, so do not treat \"not OnboardingMethod.Pake\" as OnboardingMethod.Password."
---

# OnboardingMethod enum

Which onboarding path an onboarding invite carries. More may be added, so do not treat
"not [`OnboardingMethod.Pake`](#pake)" as [`OnboardingMethod.Password`](#password).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L280)

```csharp
public enum OnboardingMethod
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="pake"></a>`Pake` | `0` | Code-based EC-JPAKE. |
| <a id="password"></a>`Password` | `1` | Shared-password channel. |
