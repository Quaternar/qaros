---
title: "GraphicsDeviceId class"
sidebar_label: "GraphicsDeviceId"
description: "Extension: selects the graphics adapter the session's Vulkan and D3D11 devices, and with them every render sender's frame textures, are created on. Chain on OnboardOptions or RejoinOptions; absent, th"
---

# GraphicsDeviceId class

Extension: selects the graphics adapter the session's Vulkan and D3D11
devices, and with them every render sender's frame textures, are created
on. Chain on [`OnboardOptions`](Quaternar.Qaros.OnboardOptions.md) or [`RejoinOptions`](Quaternar.Qaros.RejoinOptions.md);
absent, the library picks one. Windows accepts a LUID, other platforms a
UUID. The C API currently supports NVIDIA adapters only.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L724)

```csharp
public sealed class GraphicsDeviceId : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → GraphicsDeviceId

## Constructors

### GraphicsDeviceId() {#ctor}

```csharp
public GraphicsDeviceId()
```

## Properties

| Member | Description |
|---|---|
| [IdType](#idtype) |  |
| [Luid](#luid) | The adapter LUID as one 64-bit value: HighPart in the upper 32 bits, LowPart in the lower. |
| [Uuid](#uuid) | The adapter UUID, 16 bytes; used when [`GraphicsDeviceId.IdType`](#idtype) is [`GpuDeviceIdType.Uuid`](Quaternar.Qaros.GpuDeviceIdType.md#uuid). |

### IdType {#idtype}

```csharp
public GpuDeviceIdType IdType { get; set; }
```

**Value:** [GpuDeviceIdType](Quaternar.Qaros.GpuDeviceIdType.md)

### Luid {#luid}

The adapter LUID as one 64-bit value: HighPart in the upper 32 bits, LowPart in the lower.

```csharp
public long Luid { get; set; }
```

**Value:** [long](https://learn.microsoft.com/dotnet/api/system.int64)

### Uuid {#uuid}

The adapter UUID, 16 bytes; used when [`GraphicsDeviceId.IdType`](#idtype) is [`GpuDeviceIdType.Uuid`](Quaternar.Qaros.GpuDeviceIdType.md#uuid).

```csharp
public byte[] Uuid { get; set; }
```

**Value:** [byte](https://learn.microsoft.com/dotnet/api/system.byte)\[\]

## Methods

### FromLuid {#fromluid}

Selects the adapter by its LUID, as on Windows; see [`GraphicsDeviceId.Luid`](#luid) for the packing.

```csharp
public static GraphicsDeviceId FromLuid(long luid)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `luid` | [long](https://learn.microsoft.com/dotnet/api/system.int64) |  |

**Returns:** [GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md)

### FromUuid {#fromuuid}

Selects the adapter by its 16-byte UUID, as on non-Windows platforms. The array is copied.

```csharp
public static GraphicsDeviceId FromUuid(byte[] uuid)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `uuid` | [byte](https://learn.microsoft.com/dotnet/api/system.byte)\[\] |  |

**Returns:** [GraphicsDeviceId](Quaternar.Qaros.GraphicsDeviceId.md)

**Exceptions**

| Exception | Condition |
|---|---|
| [ArgumentNullException](https://learn.microsoft.com/dotnet/api/system.argumentnullexception) | `uuid` is null. |
| [ArgumentException](https://learn.microsoft.com/dotnet/api/system.argumentexception) | `uuid` is not 16 bytes long. |
