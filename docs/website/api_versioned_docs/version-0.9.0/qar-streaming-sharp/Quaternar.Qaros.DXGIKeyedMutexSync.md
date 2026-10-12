---
title: "DXGIKeyedMutexSync struct"
sidebar_label: "DXGIKeyedMutexSync"
description: "The keyed mutex of one shared D3D11 texture and the keys for its AcquireSync and ReleaseSync. Needed only when StreamParamsD3D11.AcquireKeyedMutexSync is false and the application drives the key chain"
---

# DXGIKeyedMutexSync struct

The keyed mutex of one shared D3D11 texture and the keys for its AcquireSync and
ReleaseSync. Needed only when [`StreamParamsD3D11.AcquireKeyedMutexSync`](Quaternar.Qaros.StreamParamsD3D11.md#acquirekeyedmutexsync) is
false and the application drives the key chain itself.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L2087)

```csharp
public readonly struct DXGIKeyedMutexSync
```

## Constructors

### DXGIKeyedMutexSync(nint, uint, uint) {#ctor}

```csharp
public DXGIKeyedMutexSync(nint mutex, uint acquire, uint release)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `mutex` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `acquire` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `release` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |

## Properties

| Member | Description |
|---|---|
| [Mutex](#mutex) | IDXGIKeyedMutex*. |
| [Acquire](#acquire) |  |
| [Release](#release) |  |

### Mutex {#mutex}

IDXGIKeyedMutex*.

```csharp
public nint Mutex { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### Acquire {#acquire}

```csharp
public uint Acquire { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Release {#release}

```csharp
public uint Release { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)
