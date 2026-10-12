---
title: "AppState enum"
sidebar_label: "AppState"
description: "A peer's lifecycle state. A target app is ready to be rendered for only while it is AppState.Running."
---

# AppState enum

A peer's lifecycle state. A target app is ready to be rendered for only while it is
[`AppState.Running`](#running).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L16)

```csharp
public enum AppState
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="unknown"></a>`Unknown` | `0` |  |
| <a id="initializing"></a>`Initializing` | `1` |  |
| <a id="running"></a>`Running` | `10` |  |
| <a id="shuttingdown"></a>`ShuttingDown` | `100` |  |
| <a id="retired"></a>`Retired` | `200` | The peer left for good and never comes back under its id. |
