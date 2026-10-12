---
title: "TimePoint struct"
sidebar_label: "TimePoint"
description: "Wall-clock instant: nanoseconds since the Unix epoch (UTC). Instants stamped on different hosts are only as comparable as those hosts' clocks are synchronized. Converts to a UTC DateTime implicitly or"
---

# TimePoint struct

Wall-clock instant: nanoseconds since the Unix epoch (UTC). Instants stamped on
different hosts are only as comparable as those hosts' clocks are synchronized.
Converts to a UTC [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime) implicitly or with [`TimePoint.ToDateTime`](#todatetime),
truncated to its 100 ns tick.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L413)

```csharp
public readonly struct TimePoint : IEquatable<TimePoint>, IComparable<TimePoint>
```

**Implements:** [IEquatable](https://learn.microsoft.com/dotnet/api/system.iequatable-1)&lt;[TimePoint](Quaternar.Qaros.TimePoint.md)&gt;, [IComparable](https://learn.microsoft.com/dotnet/api/system.icomparable-1)&lt;[TimePoint](Quaternar.Qaros.TimePoint.md)&gt;

## Constructors

### TimePoint(long) {#ctor}

```csharp
public TimePoint(long unixNanoseconds)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `unixNanoseconds` | [long](https://learn.microsoft.com/dotnet/api/system.int64) |  |

## Properties

### UnixNanoseconds {#unixnanoseconds}

Nanoseconds since 1970-01-01T00:00:00Z.

```csharp
public long UnixNanoseconds { get; }
```

**Value:** [long](https://learn.microsoft.com/dotnet/api/system.int64)

## Methods

| Member | Description |
|---|---|
| [ToDateTime()](#todatetime) | The instant as a UTC [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime), truncated to its 100 ns tick. |
| [ToDateTimeOffset()](#todatetimeoffset) | The instant as a [`DateTimeOffset`](https://learn.microsoft.com/dotnet/api/system.datetimeoffset) at UTC, truncated to its 100 ns tick. |
| [FromDateTime(DateTime)](#fromdatetime) | From a [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime); a local or unspecified kind is converted to UTC first. |
| [FromDateTimeOffset(DateTimeOffset)](#fromdatetimeoffset) |  |
| [Equals(TimePoint)](#equals-timepoint) | Indicates whether the current object is equal to another object of the same type. |
| [Equals(object?)](#equals-object) | Indicates whether this instance and a specified object are equal. |
| [GetHashCode()](#gethashcode) | Returns the hash code for this instance. |
| [CompareTo(TimePoint)](#compareto) | Compares the current instance with another object of the same type and returns an integer that indicates whether the current instance precedes, follows, or occurs in the same position in the sort order as the other object. |
| [ToString()](#tostring) | Returns the fully qualified type name of this instance. |

### ToDateTime {#todatetime}

The instant as a UTC [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime), truncated to its 100 ns tick.

```csharp
public DateTime ToDateTime()
```

**Returns:** [DateTime](https://learn.microsoft.com/dotnet/api/system.datetime)

### ToDateTimeOffset {#todatetimeoffset}

The instant as a [`DateTimeOffset`](https://learn.microsoft.com/dotnet/api/system.datetimeoffset) at UTC, truncated to its 100 ns tick.

```csharp
public DateTimeOffset ToDateTimeOffset()
```

**Returns:** [DateTimeOffset](https://learn.microsoft.com/dotnet/api/system.datetimeoffset)

### FromDateTime {#fromdatetime}

From a [`DateTime`](https://learn.microsoft.com/dotnet/api/system.datetime); a local or unspecified kind is converted to UTC first.

```csharp
public static TimePoint FromDateTime(DateTime value)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `value` | [DateTime](https://learn.microsoft.com/dotnet/api/system.datetime) |  |

**Returns:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### FromDateTimeOffset {#fromdatetimeoffset}

```csharp
public static TimePoint FromDateTimeOffset(DateTimeOffset value)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `value` | [DateTimeOffset](https://learn.microsoft.com/dotnet/api/system.datetimeoffset) |  |

**Returns:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### Equals(TimePoint) {#equals-timepoint}

Indicates whether the current object is equal to another object of the same type.

```csharp
public bool Equals(TimePoint other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [TimePoint](Quaternar.Qaros.TimePoint.md) | An object to compare with this object. |

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

### CompareTo {#compareto}

Compares the current instance with another object of the same type and returns an integer that indicates whether the current instance precedes, follows, or occurs in the same position in the sort order as the other object.

```csharp
public int CompareTo(TimePoint other)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `other` | [TimePoint](Quaternar.Qaros.TimePoint.md) | An object to compare with this instance. |

**Returns:** [int](https://learn.microsoft.com/dotnet/api/system.int32) — A value that indicates the relative order of the objects being compared. The return value has these meanings:  

 <table><thead><tr><th class="term"> Value</th><th class="description"> Meaning</th></tr></thead><tbody><tr><td class="term"> Less than zero</td><td class="description"> This instance precedes `other` in the sort order.</td></tr><tr><td class="term"> Zero</td><td class="description"> This instance occurs in the same position in the sort order as `other`.</td></tr><tr><td class="term"> Greater than zero</td><td class="description"> This instance follows `other` in the sort order.</td></tr></tbody></table>

### ToString {#tostring}

Returns the fully qualified type name of this instance.

```csharp
public override string ToString()
```

**Returns:** [string](https://learn.microsoft.com/dotnet/api/system.string) — The fully qualified type name.

## Operators

| Member | Description |
|---|---|
| [implicit operator DateTime(TimePoint)](#op-implicit) |  |
| [operator -(TimePoint, TimePoint)](#op-subtraction) |  |
| [operator +(TimePoint, TimeSpan)](#op-addition) |  |
| [operator ==(TimePoint, TimePoint)](#op-equality) |  |
| [operator !=(TimePoint, TimePoint)](#op-inequality) |  |
| [operator &lt;(TimePoint, TimePoint)](#op-lessthan) |  |
| [operator &gt;(TimePoint, TimePoint)](#op-greaterthan) |  |
| [operator &lt;=(TimePoint, TimePoint)](#op-lessthanorequal) |  |
| [operator &gt;=(TimePoint, TimePoint)](#op-greaterthanorequal) |  |

### implicit operator DateTime {#op-implicit}

```csharp
public static implicit operator DateTime(TimePoint value)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `value` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [DateTime](https://learn.microsoft.com/dotnet/api/system.datetime)

### operator - {#op-subtraction}

```csharp
public static TimeSpan operator -(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [TimeSpan](https://learn.microsoft.com/dotnet/api/system.timespan)

### operator + {#op-addition}

```csharp
public static TimePoint operator +(TimePoint value, TimeSpan offset)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `value` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `offset` | [TimeSpan](https://learn.microsoft.com/dotnet/api/system.timespan) |  |

**Returns:** [TimePoint](Quaternar.Qaros.TimePoint.md)

### operator == {#op-equality}

```csharp
public static bool operator ==(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator != {#op-inequality}

```csharp
public static bool operator !=(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator &lt; {#op-lessthan}

```csharp
public static bool operator <(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator &gt; {#op-greaterthan}

```csharp
public static bool operator >(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator &lt;= {#op-lessthanorequal}

```csharp
public static bool operator <=(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### operator &gt;= {#op-greaterthanorequal}

```csharp
public static bool operator >=(TimePoint left, TimePoint right)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `left` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |
| `right` | [TimePoint](Quaternar.Qaros.TimePoint.md) |  |

**Returns:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
