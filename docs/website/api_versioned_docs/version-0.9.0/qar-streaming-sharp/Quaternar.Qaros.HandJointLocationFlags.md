---
title: "HandJointLocationFlags enum"
sidebar_label: "HandJointLocationFlags"
description: "Bits of HandJointLocation.LocationFlags and HandJointVelocity.Flags."
---

# HandJointLocationFlags enum

Bits of [`HandJointLocation.LocationFlags`](Quaternar.Qaros.HandJointLocation.md#locationflags) and [`HandJointVelocity.Flags`](Quaternar.Qaros.HandJointVelocity.md#flags).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L90)

```csharp
[Flags]
public enum HandJointLocationFlags : ulong
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="none"></a>`None` | `0` |  |
| <a id="orientationvalid"></a>`OrientationValid` | `1` |  |
| <a id="positionvalid"></a>`PositionValid` | `2` |  |
| <a id="orientationtracked"></a>`OrientationTracked` | `4` |  |
| <a id="positiontracked"></a>`PositionTracked` | `8` |  |
