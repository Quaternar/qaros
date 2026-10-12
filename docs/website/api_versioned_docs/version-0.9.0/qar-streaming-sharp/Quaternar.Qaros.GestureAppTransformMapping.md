---
title: "GestureAppTransformMapping enum"
sidebar_label: "GestureAppTransformMapping"
description: "What the library applies a mapped gesture to by itself. Either way the gesture still reaches the volume's gesture subscribers; AppVolumeGestureEvent.WasMappedToAppTransform tells whether it was alread"
---

# GestureAppTransformMapping enum

What the library applies a mapped gesture to by itself. Either way the gesture still
reaches the volume's gesture subscribers; [`AppVolumeGestureEvent.WasMappedToAppTransform`](Quaternar.Qaros.AppVolumeGestureEvent.md#wasmappedtoapptransform)
tells whether it was already applied.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L251)

```csharp
public enum GestureAppTransformMapping
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="none"></a>`None` | `0` | Nothing: the gesture is left to the application. |
| <a id="apppose"></a>`AppPose` | `1` | The deltas move the volume's app pose (built-in move). |
| <a id="appscale"></a>`AppScale` | `2` | The deltas change the volume's app scale (built-in zoom). |
