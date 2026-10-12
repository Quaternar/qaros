---
title: "SyncFrameD3D11 struct"
sidebar_label: "SyncFrameD3D11"
---

# SyncFrameD3D11 struct

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2103)

```csharp
public readonly struct SyncFrameD3D11
```

## Constructors

### SyncFrameD3D11(DXGIKeyedMutexSync\[\]) {#ctor}

```csharp
public SyncFrameD3D11(DXGIKeyedMutexSync[] textureSync)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `textureSync` | [DXGIKeyedMutexSync](Quaternar.Qaros.DXGIKeyedMutexSync.md)\[\] |  |

## Properties

### TextureSync {#texturesync}

One entry per frame texture; empty when no external synchronization is needed.

```csharp
public DXGIKeyedMutexSync[] TextureSync { get; }
```

**Value:** [DXGIKeyedMutexSync](Quaternar.Qaros.DXGIKeyedMutexSync.md)\[\]
