---
title: "RenderStreamRequest struct"
sidebar_label: "RenderStreamRequest"
---

# RenderStreamRequest struct

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L319)

```csharp
public readonly struct RenderStreamRequest
```

## Constructors

### RenderStreamRequest(PeerId, StreamId) {#ctor}

```csharp
public RenderStreamRequest(PeerId targetPeerId, StreamId streamId)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `targetPeerId` | [PeerId](Quaternar.Qaros.PeerId.md) |  |
| `streamId` | [StreamId](Quaternar.Qaros.StreamId.md) |  |

## Properties

### TargetPeerId {#targetpeerid}

```csharp
public PeerId TargetPeerId { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### StreamId {#streamid}

```csharp
public StreamId StreamId { get; }
```

**Value:** [StreamId](Quaternar.Qaros.StreamId.md)
