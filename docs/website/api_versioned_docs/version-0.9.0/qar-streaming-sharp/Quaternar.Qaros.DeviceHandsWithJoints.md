---
title: "DeviceHandsWithJoints struct"
sidebar_label: "DeviceHandsWithJoints"
description: "The newest hands sample a device sent. From RenderSender.GetLastHands every pose, radius and velocity in it is in the bound app volume's app content space (right-handed, +Y up, -Z forward, app units),"
---

# DeviceHandsWithJoints struct

The newest hands sample a device sent. From [`RenderSender.GetLastHands`](Quaternar.Qaros.RenderSender.md#getlasthands)
every pose, radius and velocity in it is in the bound app volume's app
content space (right-handed, +Y up, -Z forward, app units), like the view
poses.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2370)

```csharp
public readonly struct DeviceHandsWithJoints
```

## Constructors

### DeviceHandsWithJoints(PeerId, TimePoint, HandJoints, HandJoints) {#ctor}

```csharp
public DeviceHandsWithJoints(PeerId deviceId, TimePoint deviceTimestamp, HandJoints leftHand, HandJoints rightHand)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `deviceId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `deviceTimestamp` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `leftHand` | [HandJoints](Quaternar.Qaros.HandJoints.md) |  |
| `rightHand` | [HandJoints](Quaternar.Qaros.HandJoints.md) |  |

## Properties

| Member | Description |
|---|---|
| [DeviceId](#deviceid) | The peer whose device sent the sample. |
| [DeviceTimestamp](#devicetimestamp) | When the device sampled the hands, as UTC wall-clock time on the device's host. |
| [LeftHand](#lefthand) |  |
| [RightHand](#righthand) |  |

### DeviceId {#deviceid}

The peer whose device sent the sample.

```csharp
public PeerId DeviceId { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### DeviceTimestamp {#devicetimestamp}

When the device sampled the hands, as UTC wall-clock time on the device's host.

```csharp
public TimePoint DeviceTimestamp { get; }
```

**Value:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### LeftHand {#lefthand}

```csharp
public HandJoints LeftHand { get; }
```

**Value:** [HandJoints](Quaternar.Qaros.HandJoints.md)

### RightHand {#righthand}

```csharp
public HandJoints RightHand { get; }
```

**Value:** [HandJoints](Quaternar.Qaros.HandJoints.md)
