---
title: "AppVolumeAxisFlags enum"
sidebar_label: "AppVolumeAxisFlags"
description: "Bitmask of the axes a mapping rule lets a gesture translate along or rotate about; see AppVolumeGestureMappingRule.TranslationAxes and AppVolumeGestureMappingRule.RotationAxes."
---

# AppVolumeAxisFlags enum

Bitmask of the axes a mapping rule lets a gesture translate along or rotate about; see
[`AppVolumeGestureMappingRule.TranslationAxes`](Quaternar.Qaros.AppVolumeGestureMappingRule.md#translationaxes) and
[`AppVolumeGestureMappingRule.RotationAxes`](Quaternar.Qaros.AppVolumeGestureMappingRule.md#rotationaxes).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L226)

```csharp
[Flags]
public enum AppVolumeAxisFlags
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="none"></a>`None` | `0` |  |
| <a id="x"></a>`X` | `1` |  |
| <a id="y"></a>`Y` | `2` |  |
| <a id="z"></a>`Z` | `4` |  |
| <a id="all"></a>`All` | `X | Y | Z` |  |
