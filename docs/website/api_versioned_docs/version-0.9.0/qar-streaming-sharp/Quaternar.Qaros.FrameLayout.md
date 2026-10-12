---
title: "FrameLayout enum"
sidebar_label: "FrameLayout"
description: "How the views of one video frame are packed into textures."
---

# FrameLayout enum

How the views of one video frame are packed into textures.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L132)

```csharp
public enum FrameLayout
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="sidebyside"></a>`SideBySide` | `0` | All views of one data type share one texture, side by side (both eyes in one texture). |
| <a id="separatedtextures"></a>`SeparatedTextures` | `1` | One texture per view. |
| <a id="layered"></a>`Layered` | `2` | One texture array per data type, the views in its layers. |
