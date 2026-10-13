---
title: "TargetAppConnectionString class"
sidebar_label: "TargetAppConnectionString"
description: "Builds the connection strings Session.InviteTargetApp accepts. A host is a hostname or an IP address; it must not be empty."
---

# TargetAppConnectionString class

Builds the connection strings [`Session.InviteTargetApp`](Quaternar.Qaros.Session.md#invitetargetapp) accepts.
A host is a hostname or an IP address; it must not be empty.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [Session.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Session.cs#L323)

```csharp
public static class TargetAppConnectionString
```

## Methods

| Member | Description |
|---|---|
| [HoloLens(string)](#hololens) |  |
| [Quest(string)](#quest) |  |
| [Android(string)](#android) |  |

### HoloLens {#hololens}

```csharp
public static Result<string> HoloLens(string host)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `host` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[string](https://learn.microsoft.com/dotnet/api/system.string)&gt;

### Quest {#quest}

```csharp
public static Result<string> Quest(string host)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `host` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[string](https://learn.microsoft.com/dotnet/api/system.string)&gt;

### Android {#android}

```csharp
public static Result<string> Android(string host)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `host` | [string](https://learn.microsoft.com/dotnet/api/system.string) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[string](https://learn.microsoft.com/dotnet/api/system.string)&gt;
