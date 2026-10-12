---
title: "AppWorldAnchor struct"
sidebar_label: "AppWorldAnchor"
description: "An app volume's world anchor, if it has one."
---

# AppWorldAnchor struct

An app volume's world anchor, if it has one.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1524)

```csharp
public readonly struct AppWorldAnchor
```

## Constructors

### AppWorldAnchor(bool, GeoAnchorFrame) {#ctor}

```csharp
public AppWorldAnchor(bool hasAnchor, GeoAnchorFrame anchor)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `hasAnchor` | [bool](https://learn.microsoft.com/dotnet/api/system.boolean) |  |
| `anchor` | [GeoAnchorFrame](Quaternar.Qaros.GeoAnchorFrame.md) |  |

## Properties

### HasAnchor {#hasanchor}

```csharp
public bool HasAnchor { get; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### Anchor {#anchor}

Valid only when [`AppWorldAnchor.HasAnchor`](#hasanchor) is true.

```csharp
public GeoAnchorFrame Anchor { get; }
```

**Value:** [GeoAnchorFrame](Quaternar.Qaros.GeoAnchorFrame.md)
