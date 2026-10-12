---
title: "GestureKind enum"
sidebar_label: "GestureKind"
description: "What a hand interaction with an app volume was recognised as."
---

# GestureKind enum

What a hand interaction with an app volume was recognised as.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L185)

```csharp
public enum GestureKind
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="click"></a>`Click` | `0` | Short tap or click, emitted as an [`GesturePhase.Instant`](Quaternar.Qaros.GesturePhase.md#instant) event. |
| <a id="hover"></a>`Hover` | `1` | Single-pointer hover over a targetable surface or volume. |
| <a id="singlepointer6dof"></a>`SinglePointer6Dof` | `2` | Single-pointer 6DoF manipulation. The translation delta is the accumulated hand-position offset and the rotation delta the accumulated hand-orientation offset, both in the target volume's app volume space and shaped by its mapping rule. |
| <a id="dualpointertranslatedistance"></a>`DualPointerTranslateDistance` | `3` | Two-hand distance change, recognised in the user's head-yaw frame. The translation delta is the accumulated change of the distance between the two hands as a vector along their separation: its length is the distance change and all components share one sign. Measured between the hands, not between the points their rays strike. |
| <a id="dualpointerrotate"></a>`DualPointerRotate` | `4` | Two-hand rotation of the inter-hand vector, recognised about the user's head-yaw axes and reported in the target volume's app volume space. |
