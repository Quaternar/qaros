---
title: "HandJointVelocity struct"
sidebar_label: "HandJointVelocity"
description: "Velocity of one joint."
---

# HandJointVelocity struct

Velocity of one joint.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2298)

```csharp
public readonly struct HandJointVelocity
```

## Constructors

### HandJointVelocity(uint, ulong, Vector3, Vector3) {#ctor}

```csharp
public HandJointVelocity(uint jointId, ulong flags, Vector3 linearVelocity, Vector3 angularVelocity)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `jointId` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `flags` | [ulong](https://learn.microsoft.com/dotnet/api/system.uint64) |  |
| `linearVelocity` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |
| `angularVelocity` | [Vector3](Quaternar.Qaros.Mathematics.Vector3.md) |  |

## Properties

| Member | Description |
|---|---|
| [JointId](#jointid) |  |
| [Joint](#joint) |  |
| [Flags](#flags) | Bits of [`HandJointLocationFlags`](Quaternar.Qaros.HandJointLocationFlags.md). |
| [LinearVelocity](#linearvelocity) | Units per second, in the axes and units of the joint poses: the bound app volume's app content space when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands). |
| [AngularVelocity](#angularvelocity) | Radians per second, about the axes of the joint poses: app content space axes when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands). |

### JointId {#jointid}

```csharp
public uint JointId { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Joint {#joint}

```csharp
public HandJoint Joint { get; }
```

**Value:** [HandJoint](Quaternar.Qaros.HandJoint.md)

### Flags {#flags}

Bits of [`HandJointLocationFlags`](Quaternar.Qaros.HandJointLocationFlags.md).

```csharp
public ulong Flags { get; }
```

**Value:** [ulong](https://learn.microsoft.com/dotnet/api/system.uint64)

### LinearVelocity {#linearvelocity}

Units per second, in the axes and units of the joint poses: the bound
app volume's app content space when it comes from
[`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands).

```csharp
public Vector3 LinearVelocity { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)

### AngularVelocity {#angularvelocity}

Radians per second, about the axes of the joint poses: app content
space axes when it comes from [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands).

```csharp
public Vector3 AngularVelocity { get; }
```

**Value:** [Vector3](Quaternar.Qaros.Mathematics.Vector3.md)
