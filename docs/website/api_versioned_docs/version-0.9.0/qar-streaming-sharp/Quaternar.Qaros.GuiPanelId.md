---
title: "GuiPanelId struct"
sidebar_label: "GuiPanelId"
description: "A GUI panel's 16-byte UUID, derived from the panel's common name by GuiPanelControl.GetOrCreate. Compared by content; default(GuiPanelId) equals GuiPanelId.Default."
---

# GuiPanelId struct

A GUI panel's 16-byte UUID, derived from the panel's common name by
[`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate). Compared by content; `default(GuiPanelId)`
equals [`GuiPanelId.Default`](#default).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L564)

```csharp
public readonly struct GuiPanelId : IEquatable<GuiPanelId>
```

**Implements:** [IEquatable](https://learn.microsoft.com/dotnet/api/system.iequatable-1)&lt;[GuiPanelId](Quaternar.Qaros.GuiPanelId.md)&gt;

## Properties

### Default {#default}

The all-zero id, meaning "none".

```csharp
public static GuiPanelId Default { get; }
```

**Value:** [GuiPanelId](Quaternar.Qaros.GuiPanelId.md)

## Methods

| Member | Description |
|---|---|
| [Unique()](#unique) |  |
| [Parse(string)](#parse) | Parses the UUID text [`GuiPanelId.ToString`](#tostring) produces. |
| [TryParse(string, out GuiPanelId)](#tryparse) | Parses the UUID text [`GuiPanelId.ToString`](#tostring) produces; false, with a default id, when it is not one. |
| [ToByteArray()](#tobytearray) | A copy of the 16 bytes. |
| [AsSpan()](#asspan) |  |
| [TryCopyTo(Span&lt;byte&gt;)](#trycopyto) | Copies the 16 bytes; false, copying nothing, when `destination` is shorter. |
| [ToString()](#tostring) | The 36-character UUID text, which [`GuiPanelId.Parse`](#parse) accepts back. |
| [Equals(GuiPanelId)](#equals-guipanelid) | Indicates whether the current object is equal to another object of the same type. |
| [Equals(object?)](#equals-object) | Indicates whether this instance and a specified object are equal. |
| [GetHashCode()](#gethashcode) | Returns the hash code for this instance. |

### Unique {#unique}

```csharp
public static GuiPanelId Unique()
```

**Returns:** [GuiPanelId](Quaternar.Qaros.GuiPanelId.md)

### Parse {#parse}

Parses the UUID text [`GuiPanelId.ToString`](#tostring) produces.

```csharp
public static GuiPanelId Parse(string text)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [GuiPanelId](Quaternar.Qaros.GuiPanelId.md)

**Exceptions**

| Exception | Condition |
|---|---|
| [FormatException](https://learn.microsoft.com/dotnet/api/system.formatexception) | `text` is not a UUID string. |

### TryParse {#tryparse}

Parses the UUID text [`GuiPanelId.ToString`](#tostring) produces; false, with a default id, when it is not one.

```csharp
public static bool TryParse(string text, out GuiPanelId id)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `text` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |
| `id` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |

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

The 36-character UUID text, which [`GuiPanelId.Parse`](#parse) accepts back.

```csharp
public override string ToString()
```

**Returns:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Equals(GuiPanelId) {#equals-guipanelid}

Indicates whether the current object is equal to another object of the same type.

```csharp
public bool Equals(GuiPanelId other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) | An object to compare with this object. |

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
public static bool operator ==(GuiPanelId left, GuiPanelId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `right` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator != {#op-inequality}

```csharp
public static bool operator !=(GuiPanelId left, GuiPanelId right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |
| `right` | [GuiPanelId](Quaternar.Qaros.GuiPanelId.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
