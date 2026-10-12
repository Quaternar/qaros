---
title: "GesturePhase enum"
sidebar_label: "GesturePhase"
---

# GesturePhase enum

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L211)

```csharp
public enum GesturePhase
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="started"></a>`Started` | `0` |  |
| <a id="updated"></a>`Updated` | `1` |  |
| <a id="ended"></a>`Ended` | `2` |  |
| <a id="instant"></a>`Instant` | `3` | A single event with no lifecycle: no started, updated or ended phase follows. |
| <a id="canceled"></a>`Canceled` | `4` |  |
