---
title: "RuntimeOptions class"
sidebar_label: "RuntimeOptions"
description: "Arguments of Runtime.Create."
---

# RuntimeOptions class

Arguments of [`Runtime.Create`](Quaternar.Qaros.Runtime.md#create).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L989)

```csharp
public sealed class RuntimeOptions
```

## Properties

### RuntimeBinariesFolderPath {#runtimebinariesfolderpath}

The SDK `bin` folder: the one the native library was loaded from, not the
application folder. The runtime starts `qar-runtime-launcher` from it; left null
or empty, no launcher starts and peer invites never complete.
A relative path resolves against the current working directory.

```csharp
public string? RuntimeBinariesFolderPath { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### StorageFolderPath {#storagefolderpath}

Root of the device certificates and session state. Null or empty uses the platform's
application data folder, on Windows `%LOCALAPPDATA%\Quaternar\QarOS`.

```csharp
public string? StorageFolderPath { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

## Methods

### WithExtension {#withextension}

```csharp
public RuntimeOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [RuntimeOptions](Quaternar.Qaros.RuntimeOptions.md)
