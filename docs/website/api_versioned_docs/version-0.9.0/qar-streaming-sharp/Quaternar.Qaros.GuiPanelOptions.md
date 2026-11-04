---
title: "GuiPanelOptions class"
sidebar_label: "GuiPanelOptions"
description: "Arguments of GuiPanelControl.GetOrCreate."
---

# GuiPanelOptions class

Arguments of [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1120)

```csharp
public sealed class GuiPanelOptions
```

## Constructors

### GuiPanelOptions(string) {#ctor}

```csharp
public GuiPanelOptions(string commonName)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `commonName` | [string](https://learn.microsoft.com/dotnet/api/system.string) | Required stable identity key the panel's id is derived from: [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate) returns the existing panel with this name, or creates one. |

## Properties

| Member | Description |
|---|---|
| [CommonName](#commonname) | Required stable identity key the panel's id is derived from. Must be ASCII and non-empty; put any other text in [`GuiPanelOptions.DisplayName`](#displayname). |
| [DisplayName](#displayname) | Required non-empty label when creating or reopening this object; Unicode and emoji are allowed. The constructor leaves it null: set it before calling GetOrCreate. CommonName supplies identity only and is not a fallback display name. |
| [Pose](#pose) | The panel's pose in room space: the midpoint of its top edge, front normal along -Z of the pose, the surface hanging below it. |
| [Size](#size) | Defaults to [`GuiPanelSize.Default`](Quaternar.Qaros.GuiPanelSize.md#default). |
| [ContentUri](#contenturi) | Website to show. Set at most one of this and [`GuiPanelOptions.ContentStreamId`](#contentstreamid), or [`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate) throws [`ArgumentException`](https://learn.microsoft.com/dotnet/api/system.argumentexception); with neither, a website panel is created with the default URL. The content type is fixed at creation. |
| [ContentStreamId](#contentstreamid) | Video to show: the [`PrimarylessVideoSender.GetStreamId`](Quaternar.Qaros.PrimarylessVideoSender.md#getstreamid) of a video sender owned by the same session, whose layout sizes the panel's receiver. Excludes [`GuiPanelOptions.ContentUri`](#contenturi). |
| [VisibleToPeers](#visibletopeers) | Peers the panel is shown to. Empty, the default, means visible to every peer. A panel meant only for some peers is created with them here rather than narrowed afterwards. |

### CommonName {#commonname}

Required stable identity key the panel's id is derived from. Must be ASCII and
non-empty; put any other text in [`GuiPanelOptions.DisplayName`](#displayname).

```csharp
public string CommonName { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### DisplayName {#displayname}

Required non-empty label when creating or reopening this object; Unicode and emoji are allowed. The constructor leaves it null: set it before calling GetOrCreate. CommonName supplies identity only and is not a fallback display name.

```csharp
public string? DisplayName { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### Pose {#pose}

The panel's pose in room space: the midpoint of its top edge, front
normal along -Z of the pose, the surface hanging below it.

```csharp
public Pose Pose { get; set; }
```

**Value:** [Pose](Quaternar.Qaros.Mathematics.Pose.md)

### Size {#size}

Defaults to [`GuiPanelSize.Default`](Quaternar.Qaros.GuiPanelSize.md#default).

```csharp
public GuiPanelSize Size { get; set; }
```

**Value:** [GuiPanelSize](Quaternar.Qaros.GuiPanelSize.md)

### ContentUri {#contenturi}

Website to show. Set at most one of this and [`GuiPanelOptions.ContentStreamId`](#contentstreamid), or
[`GuiPanelControl.GetOrCreate`](Quaternar.Qaros.GuiPanelControl.md#getorcreate) throws [`ArgumentException`](https://learn.microsoft.com/dotnet/api/system.argumentexception); with
neither, a website panel is created with the default URL. The content type is fixed at
creation.

```csharp
public string? ContentUri { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### ContentStreamId {#contentstreamid}

Video to show: the [`PrimarylessVideoSender.GetStreamId`](Quaternar.Qaros.PrimarylessVideoSender.md#getstreamid) of a video sender owned
by the same session, whose layout sizes the panel's receiver. Excludes
[`GuiPanelOptions.ContentUri`](#contenturi).

```csharp
public StreamId? ContentStreamId { get; set; }
```

**Value:** [StreamId](Quaternar.Qaros.StreamId.md)?

### VisibleToPeers {#visibletopeers}

Peers the panel is shown to. Empty, the default, means visible to every peer. A panel
meant only for some peers is created with them here rather than narrowed afterwards.

```csharp
public PeerId[] VisibleToPeers { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]

## Methods

### WithExtension {#withextension}

```csharp
public GuiPanelOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [GuiPanelOptions](Quaternar.Qaros.GuiPanelOptions.md)
