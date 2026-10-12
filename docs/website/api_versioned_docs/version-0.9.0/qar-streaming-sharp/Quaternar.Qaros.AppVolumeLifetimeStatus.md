---
title: "AppVolumeLifetimeStatus enum"
sidebar_label: "AppVolumeLifetimeStatus"
---

# AppVolumeLifetimeStatus enum

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L174)

```csharp
public enum AppVolumeLifetimeStatus
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="active"></a>`Active` | `0` |  |
| <a id="closed"></a>`Closed` | `1` | Retired with [`AppVolumeControl.Close`](Quaternar.Qaros.AppVolumeControl.md#close). A render sender bound to it hands out no more frames; recreate the volume to stream into it again. |
