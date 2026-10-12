---
title: "AppVolumeGestureConfiguration class"
sidebar_label: "AppVolumeGestureConfiguration"
description: "A gesture mode's mapping rules, in descending priority order, plus the peers whose gestures may move or scale the app."
---

# AppVolumeGestureConfiguration class

A gesture mode's mapping rules, in descending priority order, plus the
peers whose gestures may move or scale the app.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1715)

```csharp
public sealed class AppVolumeGestureConfiguration
```

## Remarks

The parameterless constructor mirrors qar_app_volume_gesture_configuration_default(),
which is what a volume created without a configuration gets: two-hand
distance scales the app, single-pointer 6DoF moves it (both with rotation
off), plus hover and click rules.

## Constructors

### AppVolumeGestureConfiguration() {#ctor}

```csharp
public AppVolumeGestureConfiguration()
```

## Properties

### MappingRules {#mappingrules}

Rules in descending priority order; element 0 has the highest priority. At most 16 are used.

```csharp
public AppVolumeGestureMappingRule[] MappingRules { get; set; }
```

**Value:** [AppVolumeGestureMappingRule](Quaternar.Qaros.AppVolumeGestureMappingRule.md)\[\]

### AllowedPeers {#allowedpeers}

The peers whose gestures may move or scale the app; empty lets every
peer's. Sent as QarAppVolumeGestureAllowedPeersExt when non-empty. A
configuration is replaced whole, so set it every time. Belongs to the
volume, not to a mode: [`AppVolumeControl.ChangeGestureMode`](Quaternar.Qaros.AppVolumeControl.md#changegesturemode)
rejects a non-empty list with [`StatusCode.ArgumentNotSupported`](Quaternar.Qaros.StatusCode.md#argumentnotsupported).

```csharp
public PeerId[] AllowedPeers { get; set; }
```

**Value:** [PeerId](Quaternar.Qaros.PeerId.md)\[\]
