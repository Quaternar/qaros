---
title: "Handedness enum"
sidebar_label: "Handedness"
description: "Left or right: which hand made a gesture, and the handedness of the axes of a GeoAnchorFrame."
---

# Handedness enum

Left or right: which hand made a gesture, and the handedness of the axes of a
[`GeoAnchorFrame`](Quaternar.Qaros.GeoAnchorFrame.md).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L46)

```csharp
public enum Handedness
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="unspecified"></a>`Unspecified` | `0` | None given: a gesture no single hand made (two-hand gestures), or a default-initialized value. A [`GeoAnchorFrame`](Quaternar.Qaros.GeoAnchorFrame.md) with it is rejected with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported). |
| <a id="left"></a>`Left` | `1` |  |
| <a id="right"></a>`Right` | `2` |  |
