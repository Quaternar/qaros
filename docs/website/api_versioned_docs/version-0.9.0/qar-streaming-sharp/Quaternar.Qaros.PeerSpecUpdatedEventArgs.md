---
title: "PeerSpecUpdatedEventArgs class"
sidebar_label: "PeerSpecUpdatedEventArgs"
description: "One peer update; PeerSpecUpdatedEventArgs.Peer is a snapshot, safe to keep."
---

# PeerSpecUpdatedEventArgs class

One peer update; [`PeerSpecUpdatedEventArgs.Peer`](#peer) is a snapshot, safe to keep.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [PeerSpecUpdatedEventArgs.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/Events/PeerSpecUpdatedEventArgs.cs#L12)

```csharp
public sealed class PeerSpecUpdatedEventArgs : EventArgs
```

**Inherits:** [EventArgs](https://learn.microsoft.com/dotnet/api/system.eventargs) → PeerSpecUpdatedEventArgs

## Constructors

### PeerSpecUpdatedEventArgs(PeerSpec) {#ctor}

```csharp
public PeerSpecUpdatedEventArgs(PeerSpec peer)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `peer` | [PeerSpec](Quaternar.Qaros.PeerSpec.md) |  |

## Properties

### Peer {#peer}

```csharp
public PeerSpec Peer { get; }
```

**Value:** [PeerSpec](Quaternar.Qaros.PeerSpec.md)
