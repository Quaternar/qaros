---
title: "NearFar struct"
sidebar_label: "NearFar"
description: "Near and far clip distances along a view's -Z, in the units of the poses the view was rendered with: app units for a render sender. A near plane greater than the far plane declares reverse-Z."
---

# NearFar struct

Near and far clip distances along a view's -Z, in the units of the poses the view was
rendered with: app units for a render sender. A near plane greater than the far plane
declares reverse-Z.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L385)

```csharp
public readonly struct NearFar
```

## Constructors

### NearFar(float, float) {#ctor}

```csharp
public NearFar(float nearPlane, float farPlane)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `nearPlane` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `farPlane` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

### NearPlane {#nearplane}

```csharp
public float NearPlane { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### FarPlane {#farplane}

```csharp
public float FarPlane { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)
