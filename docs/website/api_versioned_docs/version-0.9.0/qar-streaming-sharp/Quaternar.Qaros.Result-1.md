---
title: "Result<T> struct"
sidebar_label: "Result<T>"
description: "A value or a failure. A native failure is returned, never thrown: check Result<T>.IsSuccess, or opt into an exception with Result<T>.ValueOrThrow or Result<T>.EnsureSuccess."
---

# Result&lt;T&gt; struct

A value or a failure. A native failure is returned, never thrown: check
[`Result<T>.IsSuccess`](#issuccess), or opt into an exception with [`Result<T>.ValueOrThrow`](#valueorthrow) or
[`Result<T>.EnsureSuccess`](#ensuresuccess).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarResult.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarResult.cs#L167)

```csharp
public readonly struct Result<T>
```

**Type parameters**

| Name | Description |
|---|---|
| `T` |  |

## Remarks

The binding throws only for managed misuse: a null argument, or an
[`ObjectDisposedException`](https://learn.microsoft.com/dotnet/api/system.objectdisposedexception) for an object already disposed. An async method
completes its task with a failed result on a native failure; only managed cancellation
cancels the task.

## Properties

| Member | Description |
|---|---|
| [Info](#info) |  |
| [Value](#value) |  |
| [IsSuccess](#issuccess) |  |

### Info {#info}

```csharp
public ResultInfo Info { get; }
```

**Value:** [ResultInfo](Quaternar.Qaros.ResultInfo.md)

### Value {#value}

```csharp
public T Value { get; }
```

**Value:** T

**Exceptions**

| Exception | Condition |
|---|---|
| [InvalidOperationException](https://learn.microsoft.com/dotnet/api/system.invalidoperationexception) | The result is a failure. |

### IsSuccess {#issuccess}

```csharp
public bool IsSuccess { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

## Methods

| Member | Description |
|---|---|
| [Success(ResultInfo, T)](#success) |  |
| [Failure(ResultInfo)](#failure) |  |
| [EnsureSuccess()](#ensuresuccess) |  |
| [ValueOrThrow()](#valueorthrow) |  |

### Success {#success}

```csharp
public static Result<T> Success(ResultInfo info, T value)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `info` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |
| `value` | T |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;T&gt;

### Failure {#failure}

```csharp
public static Result<T> Failure(ResultInfo info)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `info` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;T&gt;

### EnsureSuccess {#ensuresuccess}

```csharp
public void EnsureSuccess()
```

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | The result is a failure. |

### ValueOrThrow {#valueorthrow}

```csharp
public T ValueOrThrow()
```

**Returns:** T

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | The result is a failure. |
