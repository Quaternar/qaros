---
title: "ResultInfo struct"
sidebar_label: "ResultInfo"
description: "The outcome of a native call: a ResultInfo.StatusCode and a diagnostic message."
---

# ResultInfo struct

The outcome of a native call: a [`ResultInfo.StatusCode`](#statuscode) and a diagnostic message.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarResult.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarResult.cs#L76)

```csharp
public readonly struct ResultInfo : IEquatable<ResultInfo>
```

**Implements:** [IEquatable](https://learn.microsoft.com/dotnet/api/system.iequatable-1)&lt;[ResultInfo](Quaternar.Qaros.ResultInfo.md)&gt;

## Remarks

Some failures are expected control flow rather than errors, for example
[`StatusCode.OnboardingSessionNotFound`](Quaternar.Qaros.StatusCode.md#onboardingsessionnotfound) from a rejoin. Equality compares
[`ResultInfo.StatusCode`](#statuscode) only; the message and [`ResultInfo.ErrorHandle`](#errorhandle) are ignored.

## Constructors

### ResultInfo(StatusCode, uint, string) {#ctor}

```csharp
public ResultInfo(StatusCode statusCode, uint errorHandle, string message)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `statusCode` | [StatusCode](Quaternar.Qaros.StatusCode.md) |  |
| `errorHandle` | [uint](https://learn.microsoft.com/dotnet/api/system.uint32) |  |
| `message` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

## Properties

| Member | Description |
|---|---|
| [StatusCode](#statuscode) |  |
| [ErrorHandle](#errorhandle) | The native diagnostic object behind a failure; 0 on success. |
| [Message](#message) |  |
| [IsSuccess](#issuccess) |  |

### StatusCode {#statuscode}

```csharp
public StatusCode StatusCode { get; }
```

**Value:** [StatusCode](Quaternar.Qaros.StatusCode.md)

### ErrorHandle {#errorhandle}

The native diagnostic object behind a failure; 0 on success.

```csharp
public uint ErrorHandle { get; }
```

**Value:** [uint](https://learn.microsoft.com/dotnet/api/system.uint32)

### Message {#message}

```csharp
public string Message { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### IsSuccess {#issuccess}

```csharp
public bool IsSuccess { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

## Methods

| Member | Description |
|---|---|
| [EnsureSuccess()](#ensuresuccess) |  |
| [ToString()](#tostring) | Returns the fully qualified type name of this instance. |
| [Equals(ResultInfo)](#equals-resultinfo) | Indicates whether the current object is equal to another object of the same type. |
| [Equals(object?)](#equals-object) | Indicates whether this instance and a specified object are equal. |
| [GetHashCode()](#gethashcode) | Returns the hash code for this instance. |

### EnsureSuccess {#ensuresuccess}

```csharp
public void EnsureSuccess()
```

**Exceptions**

| Exception | Condition |
|---|---|
| [NativeCallException](Quaternar.Qaros.NativeCallException.md) | The result is a failure. |

### ToString {#tostring}

Returns the fully qualified type name of this instance.

```csharp
public override string ToString()
```

**Returns:** [string](https://learn.microsoft.com/dotnet/api/system.string) — The fully qualified type name.

### Equals(ResultInfo) {#equals-resultinfo}

Indicates whether the current object is equal to another object of the same type.

```csharp
public bool Equals(ResultInfo other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) | An object to compare with this object. |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean) — `true` if the current object is equal to the `other` parameter; otherwise, `false`.

### Equals(object?) {#equals-object}

Indicates whether this instance and a specified object are equal.

```csharp
public override bool Equals(object? obj)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `obj` | [object](https://learn.microsoft.com/dotnet/api/system.object) | The object to compare with the current instance. |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean) — `true` if `obj` and this instance are the same type and represent the same value; otherwise, `false`.

### GetHashCode {#gethashcode}

Returns the hash code for this instance.

```csharp
public override int GetHashCode()
```

**Returns:** [int](https://learn.microsoft.com/dotnet/api/system.int32) — A 32-bit signed integer that is the hash code for this instance.

## Operators

### operator == {#op-equality}

```csharp
public static bool operator ==(ResultInfo left, ResultInfo right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |
| `right` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator != {#op-inequality}

```csharp
public static bool operator !=(ResultInfo left, ResultInfo right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |
| `right` | [ResultInfo](Quaternar.Qaros.ResultInfo.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
