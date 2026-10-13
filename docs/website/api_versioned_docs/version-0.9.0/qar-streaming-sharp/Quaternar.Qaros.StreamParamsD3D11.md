---
title: "StreamParamsD3D11 class"
sidebar_label: "StreamParamsD3D11"
description: "Extension: adopt the application's D3D11.1 device, so the sender's frame textures are shared D3D11 textures it renders into directly, with no CPU copy. Windows only."
---

# StreamParamsD3D11 class

Extension: adopt the application's D3D11.1 device, so the sender's frame textures are
shared D3D11 textures it renders into directly, with no CPU copy. Windows only.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L916)

```csharp
public sealed class StreamParamsD3D11 : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → StreamParamsD3D11

## Constructors

### StreamParamsD3D11() {#ctor}

```csharp
public StreamParamsD3D11()
```

## Properties

| Member | Description |
|---|---|
| [D3D11Context](#d3d11context) | ID3D11DeviceContext1*. |
| [D3D11Device](#d3d11device) | ID3D11Device1*. |
| [ColorBindFlags](#colorbindflags) | D3D11_BIND_FLAG bits of the colour textures, for example RENDER_TARGET \| SHADER_RESOURCE. |
| [DepthBindFlags](#depthbindflags) | D3D11_BIND_FLAG bits of the depth textures, for example DEPTH_STENCIL. |
| [AcquireKeyedMutexSync](#acquirekeyedmutexsync) | True (the default) lets the library acquire and release the textures' keyed mutexes. Set false only to drive the acquire/release key chain yourself, for example when copying through a chain of shared textures; the keys are in [`SyncFrameD3D11`](Quaternar.Qaros.SyncFrameD3D11.md). |

### D3D11Context {#d3d11context}

ID3D11DeviceContext1*.

```csharp
public nint D3D11Context { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### D3D11Device {#d3d11device}

ID3D11Device1*.

```csharp
public nint D3D11Device { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### ColorBindFlags {#colorbindflags}

D3D11_BIND_FLAG bits of the colour textures, for example RENDER_TARGET | SHADER_RESOURCE.

```csharp
public uint ColorBindFlags { get; set; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### DepthBindFlags {#depthbindflags}

D3D11_BIND_FLAG bits of the depth textures, for example DEPTH_STENCIL.

```csharp
public uint DepthBindFlags { get; set; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### AcquireKeyedMutexSync {#acquirekeyedmutexsync}

True (the default) lets the library acquire and release the textures' keyed mutexes.
Set false only to drive the acquire/release key chain yourself, for example when copying
through a chain of shared textures; the keys are in [`SyncFrameD3D11`](Quaternar.Qaros.SyncFrameD3D11.md).

```csharp
public bool AcquireKeyedMutexSync { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
