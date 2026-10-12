---
title: "HandJointLocation struct"
sidebar_label: "HandJointLocation"
description: "One XR_EXT_hand_tracking joint."
---

# HandJointLocation struct

One XR_EXT_hand_tracking joint.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2269)

```csharp
public readonly struct HandJointLocation
```

## Constructors

### HandJointLocation(uint, ulong, Pose, float) {#ctor}

```csharp
public HandJointLocation(uint jointId, ulong locationFlags, Pose pose, float radius)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `jointId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `locationFlags` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `pose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |
| `radius` | [float](https://learn.microsoft.com/dotnet/api/system.single) |  |

## Properties

| Member | Description |
|---|---|
| [JointId](#jointid) | The joint index; see [`HandJointLocation.Joint`](#joint). |
| [Joint](#joint) |  |
| [LocationFlags](#locationflags) | Bits of [`HandJointLocationFlags`](Quaternar.Qaros.HandJointLocationFlags.md). |
| [Pose](#pose) | In the space of the [`HandJoints`](Quaternar.Qaros.HandJoints.md) holding it: the bound app volume's app content space when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands). |
| [Radius](#radius) | Joint radius in the same space and units as [`HandJointLocation.Pose`](#pose): app content units when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands). |

### JointId {#jointid}

The joint index; see [`HandJointLocation.Joint`](#joint).

```csharp
public uint JointId { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Joint {#joint}

```csharp
public HandJoint Joint { get; }
```

**Value:** [HandJoint](Quaternar.Qaros.HandJoint.md)

### LocationFlags {#locationflags}

Bits of [`HandJointLocationFlags`](Quaternar.Qaros.HandJointLocationFlags.md).

```csharp
public ulong LocationFlags { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### Pose {#pose}

In the space of the [`HandJoints`](Quaternar.Qaros.HandJoints.md) holding it: the bound app
volume's app content space when it comes from
[`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands).

```csharp
public Pose Pose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Radius {#radius}

Joint radius in the same space and units as [`HandJointLocation.Pose`](#pose): app
content units when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands).

```csharp
public float Radius { get; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)
