---
title: "OnboardingId struct"
sidebar_label: "OnboardingId"
description: "The key of an identity slot, minted by the first Runtime.Onboard. It is the one thing an application persists: hand it back to Runtime.Rejoin on every start and to Runtime.Forget to unpair."
---

# OnboardingId struct

The key of an identity slot, minted by the first [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard). It is the one
thing an application persists: hand it back to [`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) on every start
and to [`Runtime.Forget`](Quaternar.Qaros.Runtime.md#forget) to unpair.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L15)

```csharp
public readonly struct OnboardingId : IEquatable<OnboardingId>
```

**Implements:** [IEquatable](https://learn.microsoft.com/dotnet/api/system.iequatable-1)&lt;[OnboardingId](Quaternar.Qaros.OnboardingId.md)&gt;

## Remarks

Store it as text: [`OnboardingId.ToString`](#tostring) writes the UUID string [`OnboardingId.Parse`](#parse) reads.
Equality is by byte content.

## Properties

### Default {#default}

The all-zero id, meaning "no id". Rejoin and forget reject it.

```csharp
public static OnboardingId Default { get; }
```

**Value:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)

## Methods

| Member | Description |
|---|---|
| [Parse(string)](#parse) |  |
| [TryParse(string, out OnboardingId)](#tryparse) |  |
| [ToByteArray()](#tobytearray) |  |
| [AsSpan()](#asspan) |  |
| [TryCopyTo(Span&lt;byte&gt;)](#trycopyto) |  |
| [ToString()](#tostring) | Returns the fully qualified type name of this instance. |
| [Equals(OnboardingId)](#equals-onboardingid) | Indicates whether the current object is equal to another object of the same type. |
| [Equals(object?)](#equals-object) | Indicates whether this instance and a specified object are equal. |
| [GetHashCode()](#gethashcode) | Returns the hash code for this instance. |

### Parse {#parse}

```csharp
public static OnboardingId Parse(string text)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [OnboardingId](Quaternar.Qaros.OnboardingId.md)

### TryParse {#tryparse}

```csharp
public static bool TryParse(string text, out OnboardingId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `id` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ToByteArray {#tobytearray}

```csharp
public byte[] ToByteArray()
```

**Returns:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)\[\]

### AsSpan {#asspan}

```csharp
public ReadOnlySpan<byte> AsSpan()
```

**Returns:** [ReadOnlySpan](https://learn.microsoft.com/dotnet/api/system.readonlyspan-1)&lt;[byte](https://learn.microsoft.com/dotnet/api/system.byte)&gt;

### TryCopyTo {#trycopyto}

```csharp
public bool TryCopyTo(Span<byte> destination)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `destination` | [Span](https://learn.microsoft.com/dotnet/api/system.span-1)&lt;[byte](https://learn.microsoft.com/dotnet/api/system.byte)&gt; |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ToString {#tostring}

Returns the fully qualified type name of this instance.

```csharp
public override string ToString()
```

**Returns:** [string](https://learn.microsoft.com/dotnet/api/system.string) — The fully qualified type name.

### Equals(OnboardingId) {#equals-onboardingid}

Indicates whether the current object is equal to another object of the same type.

```csharp
public bool Equals(OnboardingId other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) | An object to compare with this object. |

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
public static bool operator ==(OnboardingId left, OnboardingId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |
| `right` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator != {#op-inequality}

```csharp
public static bool operator !=(OnboardingId left, OnboardingId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |
| `right` | [OnboardingId](Quaternar.Qaros.OnboardingId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
