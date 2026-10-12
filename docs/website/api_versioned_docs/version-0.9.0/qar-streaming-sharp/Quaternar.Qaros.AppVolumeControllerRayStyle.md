---
title: "AppVolumeControllerRayStyle struct"
sidebar_label: "AppVolumeControllerRayStyle"
description: "How the hand ray is drawn for gestures a mapping rule matches. The colour is RGBA, 0 to 255 per channel; AppVolumeControllerRayStyle.Default is a solid opaque white line."
---

# AppVolumeControllerRayStyle struct

How the hand ray is drawn for gestures a mapping rule matches. The colour is RGBA, 0 to
255 per channel; [`AppVolumeControllerRayStyle.Default`](#default) is a solid opaque white line.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1587)

```csharp
public readonly struct AppVolumeControllerRayStyle
```

## Constructors

### AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle, byte, byte, byte, byte) {#ctor}

```csharp
public AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle lineStyle, byte colorR, byte colorG, byte colorB, byte colorA)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `lineStyle` | [AppVolumeControllerRayLineStyle](Quaternar.Qaros.AppVolumeControllerRayLineStyle.md) |  |
| `colorR` | [byte](https://learn.microsoft.com/dotnet/api/system.byte) |  |
| `colorG` | [byte](https://learn.microsoft.com/dotnet/api/system.byte) |  |
| `colorB` | [byte](https://learn.microsoft.com/dotnet/api/system.byte) |  |
| `colorA` | [byte](https://learn.microsoft.com/dotnet/api/system.byte) |  |

## Properties

| Member | Description |
|---|---|
| [LineStyle](#linestyle) |  |
| [ColorR](#colorr) |  |
| [ColorG](#colorg) |  |
| [ColorB](#colorb) |  |
| [ColorA](#colora) |  |
| [Default](#default) |  |

### LineStyle {#linestyle}

```csharp
public AppVolumeControllerRayLineStyle LineStyle { get; }
```

**Value:** [AppVolumeControllerRayLineStyle](Quaternar.Qaros.AppVolumeControllerRayLineStyle.md)

### ColorR {#colorr}

```csharp
public byte ColorR { get; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)

### ColorG {#colorg}

```csharp
public byte ColorG { get; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)

### ColorB {#colorb}

```csharp
public byte ColorB { get; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)

### ColorA {#colora}

```csharp
public byte ColorA { get; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)

### Default {#default}

```csharp
public static AppVolumeControllerRayStyle Default { get; }
```

**Value:** [AppVolumeControllerRayStyle](Quaternar.Qaros.AppVolumeControllerRayStyle.md)
