---
title: "RenderStreamRequestEventArgs class"
sidebar_label: "RenderStreamRequestEventArgs"
---

# RenderStreamRequestEventArgs class

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [RenderStreamRequestEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/RenderStreamRequestEventArgs.cs#L5)

```csharp
public sealed class RenderStreamRequestEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → RenderStreamRequestEventArgs

## Constructors

### RenderStreamRequestEventArgs(RenderStreamRequest) {#ctor}

```csharp
public RenderStreamRequestEventArgs(RenderStreamRequest request)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `request` | [RenderStreamRequest](Quaternar.Qaros.RenderStreamRequest.md) |  |

## Properties

### Request {#request}

```csharp
public RenderStreamRequest Request { get; }
```

**Value:** [RenderStreamRequest](Quaternar.Qaros.RenderStreamRequest.md)
