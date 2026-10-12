---
title: "GeoAnchorFrame struct"
sidebar_label: "GeoAnchorFrame"
description: "A local frame fully defined in world coordinates: its origin and unit axes in GeoAnchorFrame.WorldReferenceSystem (ECEF WGS84, metres, double precision), plus the handedness of those axes. Pins an app"
---

# GeoAnchorFrame struct

A local frame fully defined in world coordinates: its origin and unit axes in
[`GeoAnchorFrame.WorldReferenceSystem`](#worldreferencesystem) (ECEF WGS84, metres, double precision), plus the
handedness of those axes. Pins an app volume to the Earth.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1483)

```csharp
public readonly struct GeoAnchorFrame
```

## Constructors

### GeoAnchorFrame(WorldReferenceSystem, Vector3d, Vector3d, Vector3d, Vector3d, Handedness) {#ctor}

```csharp
public GeoAnchorFrame(WorldReferenceSystem worldReferenceSystem, Vector3d originWorld, Vector3d axisXWorld, Vector3d axisYWorld, Vector3d axisZWorld, Handedness handedness)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `worldReferenceSystem` | [WorldReferenceSystem](Quaternar.Qaros.WorldReferenceSystem.md) |  |
| `originWorld` | [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md) |  |
| `axisXWorld` | [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md) |  |
| `axisYWorld` | [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md) |  |
| `axisZWorld` | [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md) |  |
| `handedness` | [Handedness](Quaternar.Qaros.Handedness.md) |  |

## Properties

| Member | Description |
|---|---|
| [WorldReferenceSystem](#worldreferencesystem) |  |
| [OriginWorld](#originworld) | Origin of the local frame, in ECEF metres. |
| [AxisXWorld](#axisxworld) | The local +X axis as a unit vector in world coordinates. |
| [AxisYWorld](#axisyworld) | The local +Y axis as a unit vector in world coordinates. |
| [AxisZWorld](#axiszworld) | The local +Z axis as a unit vector in world coordinates. |
| [Handedness](#handedness) |  |

### WorldReferenceSystem {#worldreferencesystem}

```csharp
public WorldReferenceSystem WorldReferenceSystem { get; }
```

**Value:** [WorldReferenceSystem](Quaternar.Qaros.WorldReferenceSystem.md)

### OriginWorld {#originworld}

Origin of the local frame, in ECEF metres.

```csharp
public Vector3d OriginWorld { get; }
```

**Value:** [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md)

### AxisXWorld {#axisxworld}

The local +X axis as a unit vector in world coordinates.

```csharp
public Vector3d AxisXWorld { get; }
```

**Value:** [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md)

### AxisYWorld {#axisyworld}

The local +Y axis as a unit vector in world coordinates.

```csharp
public Vector3d AxisYWorld { get; }
```

**Value:** [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md)

### AxisZWorld {#axiszworld}

The local +Z axis as a unit vector in world coordinates.

```csharp
public Vector3d AxisZWorld { get; }
```

**Value:** [Vector3d](Quaternar.Qaros.Mathematics.Vector3d.md)

### Handedness {#handedness}

```csharp
public Handedness Handedness { get; }
```

**Value:** [Handedness](Quaternar.Qaros.Handedness.md)
