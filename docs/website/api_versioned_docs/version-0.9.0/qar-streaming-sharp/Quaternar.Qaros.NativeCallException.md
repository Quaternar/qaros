---
title: "NativeCallException class"
sidebar_label: "NativeCallException"
description: "A native failure turned into an exception on request, by Result<T>.ValueOrThrow or EnsureSuccess. NativeCallException.Result carries the status code."
---

# NativeCallException class

A native failure turned into an exception on request, by [`Result<T>.ValueOrThrow`](Quaternar.Qaros.Result-1.md#valueorthrow)
or `EnsureSuccess`. [`NativeCallException.Result`](#result) carries the status code.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarResult.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarResult.cs#L221)

```csharp
public sealed class NativeCallException : Exception, ISerializable
```

**Inherits:** [Exception](https://learn.microsoft.com/dotnet/api/system.exception) → NativeCallException  
**Implements:** [ISerializable](https://learn.microsoft.com/dotnet/api/system.runtime.serialization.iserializable)

## Constructors

### NativeCallException(ResultInfo) {#ctor}

```csharp
public NativeCallException(ResultInfo result)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `result` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |

## Properties

### Result {#result}

```csharp
public ResultInfo Result { get; }
```

**Value:** [ResultInfo](Quaternar.Qaros.ResultInfo.md)
