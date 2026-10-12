---
title: "GraphicsAPI enum"
sidebar_label: "GraphicsAPI"
description: "The graphics backend a sender's frame textures live on. Every GPU value requires exactly the matching stream-parameter extension on the options; GraphicsAPI.Cpu takes none."
---

# GraphicsAPI enum

The graphics backend a sender's frame textures live on. Every GPU value requires exactly
the matching stream-parameter extension on the options; [`GraphicsAPI.Cpu`](#cpu) takes none.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L30)

```csharp
public enum GraphicsAPI
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="cpu"></a>`Cpu` | `0` | Library-owned CPU memory; portable and always available. |
| <a id="d3d11"></a>`D3D11` | `4` | Windows only. Requires [`StreamParamsD3D11`](Quaternar.Qaros.StreamParamsD3D11.md). |
| <a id="vulkan"></a>`Vulkan` | `6` | Requires a library built with QAR_ENABLE_VULKAN and [`StreamParamsVulkan`](Quaternar.Qaros.StreamParamsVulkan.md). |
| <a id="opengl"></a>`OpenGL` | `7` | Requires a library built with QAR_ENABLE_OPENGL and exactly one [`StreamParamsOpenGl`](Quaternar.Qaros.StreamParamsOpenGl.md) extension. |
