---
title: "PeerSpec class"
sidebar_label: "PeerSpec"
description: "An immutable snapshot of one peer, taken when it was read or delivered; safe to keep. It does not change: read a new one, or watch PeerControl.SubscribeUpdates."
---

# PeerSpec class

An immutable snapshot of one peer, taken when it was read or delivered; safe to keep. It does
not change: read a new one, or watch [`PeerControl.SubscribeUpdates`](Quaternar.Qaros.PeerControl.md#subscribeupdates).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [PeerSpec.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/PeerSpec.cs#L15)

```csharp
public sealed class PeerSpec
```

## Properties

| Member | Description |
|---|---|
| [Id](#id) |  |
| [DisplayName](#displayname) |  |
| [AppVersion](#appversion) |  |
| [AppCustomPeerInfo](#appcustompeerinfo) | Application-defined metadata the peer presented itself with. |
| [AppState](#appstate) | Whether the peer is still starting, live or gone. A target app is ready to be rendered for once this is [`AppState.Running`](Quaternar.Qaros.AppState.md#running). |
| [VersionId](#versionid) |  |
| [RoomTag](#roomtag) | The physical room the peer is in, to group peers in multi-room deployments. |
| [IsTargetApp](#istargetapp) | Whether this peer shows rendered content (a headset, the Android streamer, a visualizer receiver). A source application decides itself which target apps it renders for. |

### Id {#id}

```csharp
public PeerId Id { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)

### DisplayName {#displayname}

```csharp
public string DisplayName { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### AppVersion {#appversion}

```csharp
public string AppVersion { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### AppCustomPeerInfo {#appcustompeerinfo}

Application-defined metadata the peer presented itself with.

```csharp
public string AppCustomPeerInfo { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### AppState {#appstate}

Whether the peer is still starting, live or gone. A target app is ready to be rendered
for once this is [`AppState.Running`](Quaternar.Qaros.AppState.md#running).

```csharp
public AppState AppState { get; }
```

**Value:** [AppState](Quaternar.Qaros.AppState.md)

### VersionId {#versionid}

```csharp
public string VersionId { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### RoomTag {#roomtag}

The physical room the peer is in, to group peers in multi-room deployments.

```csharp
public string RoomTag { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### IsTargetApp {#istargetapp}

Whether this peer shows rendered content (a headset, the Android streamer, a visualizer
receiver). A source application decides itself which target apps it renders for.

```csharp
public bool IsTargetApp { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)
