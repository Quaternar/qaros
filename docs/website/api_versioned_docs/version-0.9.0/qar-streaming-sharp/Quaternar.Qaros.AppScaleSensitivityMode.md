---
title: "AppScaleSensitivityMode enum"
sidebar_label: "AppScaleSensitivityMode"
---

# AppScaleSensitivityMode enum

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L236)

```csharp
public enum AppScaleSensitivityMode
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="constant"></a>`Constant` | `0` | Ignore AppScale. With Precision = 1.0, 1 cm hand movement maps to 1 cm of gesture delta. |
| <a id="inversetoappscale"></a>`InverseToAppScale` | `1` | Divide by AppScale. With AppScale = 4.0, 1 cm maps to 0.25 cm, so gestures slow down as the rendered app gets bigger. |
| <a id="basedonappscale"></a>`BasedOnAppScale` | `2` | Multiply by AppScale. With AppScale = 4.0, 1 cm maps to 4 cm, so gestures speed up as the rendered app gets bigger. |
