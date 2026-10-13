---
title: "Fov struct"
sidebar_label: "Fov"
description: "Field of view of one render view, in view space: angles in radians from the view's -Z axis, with OpenXR signs - left and down negative, right and up positive."
---

# Fov struct

Field of view of one render view, in view space: angles in radians from the view's -Z
axis, with OpenXR signs - left and down negative, right and up positive.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L354)

```csharp
public readonly struct Fov
```

## Constructors

### Fov(float, float, float, float) {#ctor}

```csharp
public Fov(float angleLeft, float angleRight, float angleUp, float angleDown)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `angleLeft` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `angleRight` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `angleUp` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `angleDown` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [AngleLeft](#angleleft) |  |
| [AngleRight](#angleright) |  |
| [AngleUp](#angleup) |  |
| [AngleDown](#angledown) |  |

### AngleLeft {#angleleft}

```csharp
public float AngleLeft { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AngleRight {#angleright}

```csharp
public float AngleRight { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AngleUp {#angleup}

```csharp
public float AngleUp { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AngleDown {#angledown}

```csharp
public float AngleDown { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)
