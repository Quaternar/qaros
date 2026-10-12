---
title: "HandJoints struct"
sidebar_label: "HandJoints"
description: "One hand's tracking state: 26 joint locations and velocities indexed by HandJoint."
---

# HandJoints struct

One hand's tracking state: 26 joint locations and velocities indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2326)

```csharp
public readonly struct HandJoints
```

## Constructors

### HandJoints(bool, bool, Pose, HandJointLocation\[\], bool, HandJointVelocity\[\]) {#ctor}

```csharp
public HandJoints(bool isTracked, bool isActive, Pose pose, HandJointLocation[] jointLocations, bool hasVelocity, HandJointVelocity[] jointVelocities)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `isTracked` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `isActive` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `pose` | [Pose](Quaternar.Qaros.Mathematics.Pose.md) |  |
| `jointLocations` | [HandJointLocation](Quaternar.Qaros.HandJointLocation.md)\[\] |  |
| `hasVelocity` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `jointVelocities` | [HandJointVelocity](Quaternar.Qaros.HandJointVelocity.md)\[\] |  |

## Properties

| Member | Description |
|---|---|
| [IsTracked](#istracked) | False when the hand is not tracked; also false for both hands once no sample arrived for a second, or while the render sender's app volume does not resolve. |
| [IsActive](#isactive) |  |
| [Pose](#pose) | The hand's aim pose, the OpenXR /input/aim/pose: its -Z is the hand ray. The bound app volume's app content space when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands). |
| [JointLocations](#jointlocations) | 26 entries, indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md). |
| [HasVelocity](#hasvelocity) | Whether [`HandJoints.JointVelocities`](#jointvelocities) carries data. |
| [JointVelocities](#jointvelocities) | 26 entries, indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md); meaningful only when [`HandJoints.HasVelocity`](#hasvelocity) is true. |

### IsTracked {#istracked}

False when the hand is not tracked; also false for both hands once no sample arrived for a second, or while the render sender's app volume does not resolve.

```csharp
public bool IsTracked { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### IsActive {#isactive}

```csharp
public bool IsActive { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### Pose {#pose}

The hand's aim pose, the OpenXR /input/aim/pose: its -Z is the hand ray.
The bound app volume's app content space when it comes from
[`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands).

```csharp
public Pose Pose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### JointLocations {#jointlocations}

26 entries, indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md).

```csharp
public HandJointLocation[] JointLocations { get; }
```

**Value:** [HandJointLocation](Quaternar.Qaros.HandJointLocation.md)\[\]

### HasVelocity {#hasvelocity}

Whether [`HandJoints.JointVelocities`](#jointvelocities) carries data.

```csharp
public bool HasVelocity { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### JointVelocities {#jointvelocities}

26 entries, indexed by [`HandJoint`](Quaternar.Qaros.HandJoint.md); meaningful only when [`HandJoints.HasVelocity`](#hasvelocity) is true.

```csharp
public HandJointVelocity[] JointVelocities { get; }
```

**Value:** [HandJointVelocity](Quaternar.Qaros.HandJointVelocity.md)\[\]
