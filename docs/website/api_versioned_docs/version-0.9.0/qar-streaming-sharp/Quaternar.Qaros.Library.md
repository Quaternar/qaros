---
title: "Library class"
sidebar_label: "Library"
description: "Process-wide library state. Library.Init comes first, before any other call of this API; then Runtime.Create, then Runtime.Onboard or Runtime.Rejoin for a Session."
---

# Library class

Process-wide library state. [`Library.Init`](#init) comes first, before any other call of this
API; then [`Runtime.Create`](Quaternar.Qaros.Runtime.md#create), then [`Runtime.Onboard`](Quaternar.Qaros.Runtime.md#onboard) or
[`Runtime.Rejoin`](Quaternar.Qaros.Runtime.md#rejoin) for a [`Session`](Quaternar.Qaros.Session.md).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarLibrary.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarLibrary.cs#L16)

```csharp
public static class Library
```

## Methods

### Init {#init}

Initializes the library once per process, from a single thread. Calling it again without
a [`Library.Destroy`](#destroy) in between may fail.

```csharp
public static Result<Unit> Init(LibraryOptions? options = null)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `options` | [LibraryOptions](Quaternar.Qaros.LibraryOptions.md) |  |

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

### Destroy {#destroy}

Frees process-wide state. No other call of this API is allowed afterwards until the next
[`Library.Init`](#init).

```csharp
public static Result<Unit> Destroy()
```

**Returns:** [Result](Quaternar.Qaros.Result-1.md)&lt;[Unit](Quaternar.Qaros.Unit.md)&gt;

**Remarks**

It ends every session and subscription still alive, waiting for a subscription handler
running on another thread. Dispose your sessions and the runtime first, and never call it
from a QAROS callback, event handler or progress report.
