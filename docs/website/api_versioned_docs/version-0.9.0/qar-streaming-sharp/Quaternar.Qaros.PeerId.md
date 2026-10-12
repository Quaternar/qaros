---
title: "PeerId struct"
sidebar_label: "PeerId"
description: "A peer's 16-byte UUID. Compared by content; default(PeerId) equals PeerId.Default."
---

# PeerId struct

A peer's 16-byte UUID. Compared by content; `default(PeerId)` equals
[`PeerId.Default`](#default).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L484)

```csharp
public readonly struct PeerId : IEquatable<PeerId>
```

**Implements:** [IEquatable](https://learn.microsoft.com/dotnet/api/system.iequatable-1)&lt;[PeerId](Quaternar.Qaros.PeerId.md)&gt;

## Properties

### Default {#default}

The all-zero id, meaning "no peer".

```csharp
public static PeerId Default { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

## Methods

| Member | Description |
|---|---|
| [Unique()](#unique) | A fresh id that identifies no peer in any session. Never use it as a render target ([`RenderSenderOptions.PeerId`](Quaternar.Qaros.RenderSenderOptions.md#peerid)): a stream created with one never produces a frame. |
| [Parse(string)](#parse) | Parses the UUID text [`PeerId.ToString`](#tostring) produces. |
| [TryParse(string, out PeerId)](#tryparse) | Parses the UUID text [`PeerId.ToString`](#tostring) produces; false, with a default id, when it is not one. |
| [ToByteArray()](#tobytearray) | A copy of the 16 bytes. |
| [AsSpan()](#asspan) |  |
| [TryCopyTo(Span&lt;byte&gt;)](#trycopyto) | Copies the 16 bytes; false, copying nothing, when `destination` is shorter. |
| [ToString()](#tostring) | The 36-character UUID text, which [`PeerId.Parse`](#parse) accepts back. |
| [Equals(PeerId)](#equals-peerid) | Indicates whether the current object is equal to another object of the same type. |
| [Equals(object?)](#equals-object) | Indicates whether this instance and a specified object are equal. |
| [GetHashCode()](#gethashcode) | Returns the hash code for this instance. |

### Unique {#unique}

A fresh id that identifies no peer in any session. Never use it as a render target
([`RenderSenderOptions.PeerId`](Quaternar.Qaros.RenderSenderOptions.md#peerid)): a stream created with one never produces
a frame.

```csharp
public static PeerId Unique()
```

**Returns:** [PeerId](Quaternar.Qaros.PeerId.md)

### Parse {#parse}

Parses the UUID text [`PeerId.ToString`](#tostring) produces.

```csharp
public static PeerId Parse(string text)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [PeerId](Quaternar.Qaros.PeerId.md)

**Exceptions**

| Exception | Condition |
|---|---|
| [FormatException](https://learn.microsoft.com/dotnet/api/system.formatexception) | `text` is not a UUID string. |

### TryParse {#tryparse}

Parses the UUID text [`PeerId.ToString`](#tostring) produces; false, with a default id, when it is not one.

```csharp
public static bool TryParse(string text, out PeerId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `id` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ToByteArray {#tobytearray}

A copy of the 16 bytes.

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

Copies the 16 bytes; false, copying nothing, when `destination` is shorter.

```csharp
public bool TryCopyTo(Span<byte> destination)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `destination` | [Span](https://learn.microsoft.com/dotnet/api/system.span-1)&lt;[byte](https://learn.microsoft.com/dotnet/api/system.byte)&gt; |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ToString {#tostring}

The 36-character UUID text, which [`PeerId.Parse`](#parse) accepts back.

```csharp
public override string ToString()
```

**Returns:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Equals(PeerId) {#equals-peerid}

Indicates whether the current object is equal to another object of the same type.

```csharp
public bool Equals(PeerId other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [PeerId](Quaternar.Qaros.PeerId.md) | An object to compare with this object. |

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
public static bool operator ==(PeerId left, PeerId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `right` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator != {#op-inequality}

```csharp
public static bool operator !=(PeerId left, PeerId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `right` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
