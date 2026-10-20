---
title: "AppVolumeOptions class"
sidebar_label: "AppVolumeOptions"
description: "Arguments of AppVolumeControl.GetOrCreate. The defaults match qar_app_volume_init_default(): identity poses, a 1 m cube, app scale 1, no world anchor and the default gesture configuration."
---

# AppVolumeOptions class

Arguments of [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate). The defaults match
`qar_app_volume_init_default()`: identity poses, a 1 m cube, app scale 1, no world
anchor and the default gesture configuration.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1181)

```csharp
public sealed class AppVolumeOptions
```

## Constructors

### AppVolumeOptions(string) {#ctor}

```csharp
public AppVolumeOptions(string commonName)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `commonName` | [string](https://learn.microsoft.com/dotnet/api/system.string) | Required stable identity key the volume's id is derived from: [`AppVolumeControl.GetOrCreate`](Quaternar.Qaros.AppVolumeControl.md#getorcreate) returns the existing volume with this name, or creates one. |

## Properties

| Member | Description |
|---|---|
| [CommonName](#commonname) | Required stable identity key the volume's id is derived from. Must be ASCII and non-empty; put any other text in [`AppVolumeOptions.DisplayName`](#displayname). |
| [DisplayName](#displayname) | Required non-empty label when creating or reopening this object; Unicode and emoji are allowed. The constructor leaves it null: set it before calling GetOrCreate. CommonName supplies identity only and is not a fallback display name. |
| [Pose](#pose) | Placement of the cuboid's centre in room space, and the rotation taking app volume space into room space. |
| [Size](#size) | Cuboid extents in metres along app-volume-space X (width), Z (length) and Y (height). |
| [InitialPeers](#initialpeers) |  |
| [AppPose](#apppose) | Origin of the rendering application inside app volume space, in volume metres, relative to the cuboid centre. Not a room-space pose, and [`AppVolumeOptions.AppScale`](#appscale) does not scale it. |
| [AppScale](#appscale) | Room-metres per app-metre: the factor between app content space and app volume space. 1.0 means 1 m in app coordinates renders as 1 m in the app volume; 0.5 makes the app appear smaller, 10.0 bigger. [`AppVolumeOptions.AppPose`](#apppose) remains in app-volume metres. |
| [AppWorldAnchor](#appworldanchor) | Pins the volume to an Earth-fixed frame; null, the default, sets no anchor. |
| [GestureConfiguration](#gestureconfiguration) | Rules of the default gesture mode and the allowed peers. Null, the default, gives the configuration a parameterless [`AppVolumeGestureConfiguration`](Quaternar.Qaros.AppVolumeGestureConfiguration.md) describes. |

### CommonName {#commonname}

Required stable identity key the volume's id is derived from. Must be ASCII and
non-empty; put any other text in [`AppVolumeOptions.DisplayName`](#displayname).

```csharp
public string CommonName { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### DisplayName {#displayname}

Required non-empty label when creating or reopening this object; Unicode and emoji are allowed. The constructor leaves it null: set it before calling GetOrCreate. CommonName supplies identity only and is not a fallback display name.

```csharp
public string? DisplayName { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Pose {#pose}

Placement of the cuboid's centre in room space, and the rotation taking
app volume space into room space.

```csharp
public Pose Pose { get; set; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Size {#size}

Cuboid extents in metres along app-volume-space X (width), Z (length) and Y (height).

```csharp
public AppVolumeSize Size { get; set; }
```

**Value:** [AppVolumeSize](Quaternar.Qaros.AppVolumeSize.md)

### InitialPeers {#initialpeers}

```csharp
public PeerId[] InitialPeers { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]

### AppPose {#apppose}

Origin of the rendering application inside app volume space, in volume
metres, relative to the cuboid centre. Not a room-space pose, and
[`AppVolumeOptions.AppScale`](#appscale) does not scale it.

```csharp
public Pose AppPose { get; set; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### AppScale {#appscale}

Room-metres per app-metre: the factor between app content space and
app volume space. 1.0 means 1 m in app coordinates renders as 1 m in
the app volume; 0.5 makes the app appear smaller, 10.0 bigger.
[`AppVolumeOptions.AppPose`](#apppose) remains in app-volume metres.

```csharp
public float AppScale { get; set; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AppWorldAnchor {#appworldanchor}

Pins the volume to an Earth-fixed frame; null, the default, sets no anchor.

```csharp
public GeoAnchorFrame? AppWorldAnchor { get; set; }
```

**Value:** [GeoAnchorFrame](Quaternar.Qaros.GeoAnchorFrame.md)?

### GestureConfiguration {#gestureconfiguration}

Rules of the default gesture mode and the allowed peers. Null, the default, gives the
configuration a parameterless [`AppVolumeGestureConfiguration`](Quaternar.Qaros.AppVolumeGestureConfiguration.md) describes.

```csharp
public AppVolumeGestureConfiguration? GestureConfiguration { get; set; }
```

**Value:** [AppVolumeGestureConfiguration](Quaternar.Qaros.AppVolumeGestureConfiguration.md)

## Methods

### WithExtension {#withextension}

```csharp
public AppVolumeOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [AppVolumeOptions](Quaternar.Qaros.AppVolumeOptions.md)
