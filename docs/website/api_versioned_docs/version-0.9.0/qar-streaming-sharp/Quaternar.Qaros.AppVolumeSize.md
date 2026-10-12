---
title: "AppVolumeSize struct"
sidebar_label: "AppVolumeSize"
description: "An app volume cuboid's extents in metres, centred on the volume's pose: it spans plus and minus half of each extent along app volume space X, Y and Z. Note the constructor order: width (X), length (Z)"
---

# AppVolumeSize struct

An app volume cuboid's extents in metres, centred on the volume's pose: it spans plus and
minus half of each extent along app volume space X, Y and Z. Note the constructor order:
width (X), length (Z), height (Y).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1543)

```csharp
public readonly struct AppVolumeSize
```

## Constructors

### AppVolumeSize(float, float, float) {#ctor}

```csharp
public AppVolumeSize(float widthMeters, float lengthMeters, float heightMeters)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `widthMeters` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `lengthMeters` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |
| `heightMeters` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [WidthMeters](#widthmeters) | Along app volume space X. |
| [LengthMeters](#lengthmeters) | Along app volume space Z, the horizontal depth. |
| [HeightMeters](#heightmeters) | Along app volume space Y, the vertical rise. |
| [Default](#default) | A 1 m cube. |

### WidthMeters {#widthmeters}

Along app volume space X.

```csharp
public float WidthMeters { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### LengthMeters {#lengthmeters}

Along app volume space Z, the horizontal depth.

```csharp
public float LengthMeters { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### HeightMeters {#heightmeters}

Along app volume space Y, the vertical rise.

```csharp
public float HeightMeters { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### Default {#default}

A 1 m cube.

```csharp
public static AppVolumeSize Default { get; }
```

**Value:** [AppVolumeSize](Quaternar.Qaros.AppVolumeSize.md)
