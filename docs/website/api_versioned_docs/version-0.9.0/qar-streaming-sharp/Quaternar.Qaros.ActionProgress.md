---
title: "ActionProgress struct"
sidebar_label: "ActionProgress"
---

# ActionProgress struct

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L289)

```csharp
public readonly struct ActionProgress
```

## Constructors

### ActionProgress(ActionSeverity, float, string) {#ctor}

```csharp
public ActionProgress(ActionSeverity severity, float percent, string message)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `severity` | [ActionSeverity](Quaternar.Qaros.ActionSeverity.md) |  |
| `percent` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `message` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

## Properties

| Member | Description |
|---|---|
| [Severity](#severity) |  |
| [Percent](#percent) |  |
| [Message](#message) |  |

### Severity {#severity}

```csharp
public ActionSeverity Severity { get; }
```

**Value:** [ActionSeverity](Quaternar.Qaros.ActionSeverity.md)

### Percent {#percent}

```csharp
public float Percent { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### Message {#message}

```csharp
public string Message { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)
