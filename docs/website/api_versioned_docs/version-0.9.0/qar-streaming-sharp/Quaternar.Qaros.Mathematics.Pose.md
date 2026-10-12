---
title: "Pose struct"
sidebar_label: "Pose"
description: "Position plus orientation (x, y, z, w). A pose carries no space of its own: every member that holds one names the space it is in. The bindings convert no coordinates; each value is exactly what the na"
---

# Pose struct

Position plus orientation (x, y, z, w). A pose carries no space of its
own: every member that holds one names the space it is in. The bindings
convert no coordinates; each value is exactly what the native library
produced.

**Namespace:** [Quaternar.Qaros.Mathematics](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Mathematics.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Mathematics.cs#L108)

```csharp
public readonly struct Pose
```

## Remarks

Every 3D space follows OpenXR: right-handed, +Y up, -Z forward, so a view
looks along -Z. The spaces, as in the C header (types.h, "Coordinate
systems") and https://docs.quaternar.com/docs/developer-guide/coordinate-systems/:

- **Room space**: the frame every peer in a session places things in, in metres. Whatever arrives through data streams and shared objects is in the publishing peer's room space: app volume poses, GUI panel poses, peer data.
- **App volume space**: one app volume's local frame, origin at the cuboid centre, +X along width, +Y up along height, +Z along length, in metres. The app pose and the deltas of a mapped app volume gesture are in it.
- **App content space**: the application's own world inside the volume, the space it renders in: app volume space taken into the app pose, positions divided by the app scale, so in app metres. Everything a render sender hands the source application is in the bound volume's app content space: view poses, the rendered pose of a view override, hand aim poses, joint poses, joint radii and joint velocities.
- **View space**: one render view's camera frame, looking along -Z. [`Fov`](Quaternar.Qaros.Fov.md) angles are in it.

## Constructors

### Pose(Quaternion, Vector3) {#ctor}

```csharp
public Pose(Quaternion orientation, Vector3 position)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `orientation` | [Quaternion](Quaternar.Qaros.Mathematics.Quaternion.md) |  |
| `position` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |

## Properties

| Member | Description |
|---|---|
| [Orientation](#orientation) |  |
| [Position](#position) |  |
| [Identity](#identity) |  |

### Orientation {#orientation}

```csharp
public Quaternion Orientation { get; }
```

**Value:** [Quaternion](Quaternar.Qaros.Mathematics.Quaternion.md)

### Position {#position}

```csharp
public Vector3 Position { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)

### Identity {#identity}

```csharp
public static Pose Identity { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)
