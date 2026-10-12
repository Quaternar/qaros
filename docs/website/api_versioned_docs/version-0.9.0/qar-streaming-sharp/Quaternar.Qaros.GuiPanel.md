---
title: "GuiPanel class"
sidebar_label: "GuiPanel"
description: "An immutable snapshot of one GUI panel, safe to keep. Get a newer one from GuiPanelControl.SubscribeUpdates."
---

# GuiPanel class

An immutable snapshot of one GUI panel, safe to keep. Get a newer one from
[`GuiPanelControl.SubscribeUpdates`](Quaternar.Qaros.GuiPanelControl.md#subscribeupdates-eventhandler-guipanelupdatedeventargs-cancellationtoken).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [GuiPanel.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/GuiPanel.cs#L16)

```csharp
public sealed class GuiPanel
```

## Properties

| Member | Description |
|---|---|
| [Id](#id) |  |
| [DisplayName](#displayname) |  |
| [Pose](#pose) | The panel's pose in the publishing peer's room space: the midpoint of its top edge, front normal along -Z. |
| [Size](#size) |  |
| [ContentType](#contenttype) | Which of [`GuiPanel.ContentUri`](#contenturi) and [`GuiPanel.ContentStreamId`](#contentstreamid) is set; the other is null. Fixed when the panel is created. |
| [ContentUri](#contenturi) | The web page shown, for [`GuiPanelContentType.WebsiteUri`](Quaternar.Qaros.GuiPanelContentType.md#websiteuri); null otherwise. |
| [ContentStreamId](#contentstreamid) | The video stream shown, for [`GuiPanelContentType.StreamId`](Quaternar.Qaros.GuiPanelContentType.md#streamid); null otherwise. |
| [State](#state) | Whether peers see the panel. A [`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed) panel is reopened by [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate) with the same common name. |
| [VisibleToPeers](#visibletopeers) | The peers the panel is narrowed to. Empty means visible to every peer, not to none. |

### Id {#id}

```csharp
public GuiPanelId Id { get; }
```

**Value:** [GuiPanelId](Quaternar.Qaros.GuiPanelId.md)

### DisplayName {#displayname}

```csharp
public string DisplayName { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Pose {#pose}

The panel's pose in the publishing peer's room space: the midpoint of its top edge, front normal along -Z.

```csharp
public Pose Pose { get; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Size {#size}

```csharp
public GuiPanelSize Size { get; }
```

**Value:** [GuiPanelSize](Quaternar.Qaros.GuiPanelSize.md)

### ContentType {#contenttype}

Which of [`GuiPanel.ContentUri`](#contenturi) and [`GuiPanel.ContentStreamId`](#contentstreamid) is set; the other
is null. Fixed when the panel is created.

```csharp
public GuiPanelContentType ContentType { get; }
```

**Value:** [GuiPanelContentType](Quaternar.Qaros.GuiPanelContentType.md)

### ContentUri {#contenturi}

The web page shown, for [`GuiPanelContentType.WebsiteUri`](Quaternar.Qaros.GuiPanelContentType.md#websiteuri); null otherwise.

```csharp
public string? ContentUri { get; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### ContentStreamId {#contentstreamid}

The video stream shown, for [`GuiPanelContentType.StreamId`](Quaternar.Qaros.GuiPanelContentType.md#streamid); null otherwise.

```csharp
public StreamId? ContentStreamId { get; }
```

**Value:** [StreamId](Quaternar.Qaros.StreamId.md)?

### State {#state}

Whether peers see the panel. A [`GuiPanelState.Closed`](Quaternar.Qaros.GuiPanelState.md#closed) panel is reopened by
[`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate) with the same common name.

```csharp
public GuiPanelState State { get; }
```

**Value:** [GuiPanelState](Quaternar.Qaros.GuiPanelState.md)

### VisibleToPeers {#visibletopeers}

The peers the panel is narrowed to. Empty means visible to every peer, not to none.

```csharp
public PeerId[] VisibleToPeers { get; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]
