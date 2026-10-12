---
title: "Quaternion struct"
sidebar_label: "Quaternion"
description: "Rotation quaternion, Hamilton convention, stored and constructed in x, y, z, w order (w last). default(Quaternion) is all zeros, not a rotation; use Quaternion.Identity."
---

# Quaternion struct

Rotation quaternion, Hamilton convention, stored and constructed in x, y, z, w order (w
last). `default(Quaternion)` is all zeros, not a rotation; use [`Quaternion.Identity`](#identity).

**Namespace:** [Quaternar.Qaros.Mathematics](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Mathematics.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Mathematics.cs#L63)

```csharp
public readonly struct Quaternion
```

## Constructors

### Quaternion(float, float, float, float) {#ctor}

```csharp
public Quaternion(float x, float y, float z, float w)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `x` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `y` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `z` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `w` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [X](#x) |  |
| [Y](#y) |  |
| [Z](#z) |  |
| [W](#w) |  |
| [Identity](#identity) |  |

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

### W {#w}

```csharp
public float W { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### Identity {#identity}

```csharp
public static Quaternion Identity { get; }
```

**Value:** [Quaternion](Quaternar.Qaros.Mathematics.Quaternion.md)
