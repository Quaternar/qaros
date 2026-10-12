---
title: "OnboardPeerId class"
sidebar_label: "OnboardPeerId"
description: "Extension: join the session under a caller-chosen peer id instead of a freshly minted one. Chain on OnboardOptions with OnboardOptions.WithPeerId. The id is persisted with the slot, so a later rejoin "
---

# OnboardPeerId class

Extension: join the session under a caller-chosen peer id instead of a
freshly minted one. Chain on [`OnboardOptions`](Quaternar.Qaros.OnboardOptions.md) with
[`OnboardOptions.WithPeerId`](Quaternar.Qaros.OnboardOptions.md#withpeerid). The id is persisted with
the slot, so a later rejoin keeps it. A zero id is rejected with
[`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L706)

```csharp
public sealed class OnboardPeerId : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → OnboardPeerId

## Constructors

### OnboardPeerId(PeerId) {#ctor}

```csharp
public OnboardPeerId(PeerId peerId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `peerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |

## Properties

### PeerId {#peerid}

```csharp
public PeerId PeerId { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)
