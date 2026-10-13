---
title: "LibraryOptions class"
sidebar_label: "LibraryOptions"
description: "Arguments of Library.Init, which must run once per process before any other call. The defaults match qar_library_init_default()."
---

# LibraryOptions class

Arguments of [`Library.Init`](Quaternar.Qaros.Library.md#init), which must run once per process before any other
call. The defaults match `qar_library_init_default()`.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L961)

```csharp
public sealed class LibraryOptions
```

## Constructors

### LibraryOptions() {#ctor}

```csharp
public LibraryOptions()
```

## Properties

| Member | Description |
|---|---|
| [EnableConsoleLogging](#enableconsolelogging) | Defaults to true. |
| [LogFolderPath](#logfolderpath) | Folder for `qar_streaming.log`; null (the default) writes no log file. The file starts empty on each init and rolls over once it is a day old or 256 MiB; the oldest rolled files are deleted so the log takes at most 3 GiB. |
| [LogSeverity](#logseverity) | Minimum severity logged. Defaults to [`LogSeverity.Info`](Quaternar.Qaros.LogSeverity.md#info). |

### EnableConsoleLogging {#enableconsolelogging}

Defaults to true.

```csharp
public bool EnableConsoleLogging { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### LogFolderPath {#logfolderpath}

Folder for `qar_streaming.log`; null (the default) writes no log file. The file
starts empty on each init and rolls over once it is a day old or 256 MiB; the oldest
rolled files are deleted so the log takes at most 3 GiB.

```csharp
public string? LogFolderPath { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### LogSeverity {#logseverity}

Minimum severity logged. Defaults to [`LogSeverity.Info`](Quaternar.Qaros.LogSeverity.md#info).

```csharp
public LogSeverity LogSeverity { get; set; }
```

**Value:** [LogSeverity](Quaternar.Qaros.LogSeverity.md)

## Methods

### WithExtension {#withextension}

```csharp
public LibraryOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [LibraryOptions](Quaternar.Qaros.LibraryOptions.md)
