---
title: "Vector3 struct"
sidebar_label: "Vector3"
description: "A 3D vector. It carries no space of its own; the member holding it names the space (see Pose)."
---

# Vector3 struct

A 3D vector. It carries no space of its own; the member holding it names the space (see
[`Pose`](Quaternar.Qaros.Mathematics.Pose.md)).

**Namespace:** [Quaternar.Qaros.Mathematics](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Mathematics.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Mathematics.cs#L10)

```csharp
public readonly struct Vector3
```

## Constructors

### Vector3(float, float, float) {#ctor}

```csharp
public Vector3(float x, float y, float z)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `x` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `y` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `z` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [X](#x) |  |
| [Y](#y) |  |
| [Z](#z) |  |

### X {#x}

```csharp
public float X { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### Y {#y}

```csharp
public float Y { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### Z {#z}

```csharp
public float Z { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)
