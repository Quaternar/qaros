---
title: "AppVolumeGestureMappingRule class"
sidebar_label: "AppVolumeGestureMappingRule"
description: "How one gesture kind on an app volume is shaped and what it is applied to. Its priority is its position in AppVolumeGestureConfiguration.MappingRules, not a field."
---

# AppVolumeGestureMappingRule class

How one gesture kind on an app volume is shaped and what it is applied to. Its priority is
its position in [`AppVolumeGestureConfiguration.MappingRules`](Quaternar.Qaros.AppVolumeGestureConfiguration.md#mappingrules), not a field.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L1636)

```csharp
public sealed class AppVolumeGestureMappingRule
```

## Remarks

The parameterless constructor mirrors `qar_app_volume_gesture_mapping_rule_default()`:
an enabled [`GestureKind.Click`](Quaternar.Qaros.GestureKind.md#click) rule, precision 1, constant sensitivity, all
axes, gestures from outside the volume accepted, the default ray, no app transform.
Hover and click reach the application even without a rule of their own.

## Constructors

### AppVolumeGestureMappingRule() {#ctor}

```csharp
public AppVolumeGestureMappingRule()
```

## Properties

| Member | Description |
|---|---|
| [GestureKind](#gesturekind) |  |
| [Enabled](#enabled) | A disabled hover or click rule does not stop delivery: those two always reach the application, through a built-in rule ranked after every configured one. |
| [Precision](#precision) | Hand centimetres per app-volume centimetre. With constant sensitivity 1.0 maps 1 cm of hand movement to 1 cm, 2.0 to 0.5 cm (finer control) and 0.5 to 2 cm. [`AppVolumeGestureMappingRule.AppScaleSensitivityMode`](#appscalesensitivitymode) scales the result after precision is applied. |
| [AppScaleSensitivityMode](#appscalesensitivitymode) |  |
| [TranslationAxes](#translationaxes) |  |
| [RotationAxes](#rotationaxes) |  |
| [EnableGesturesFromOutsideOfVolume](#enablegesturesfromoutsideofvolume) | Whether a gesture made with the hand outside the volume's cuboid is accepted; tested at the hand's position. |
| [ControllerRayStyle](#controllerraystyle) |  |
| [AppTransformMapping](#apptransformmapping) |  |

### GestureKind {#gesturekind}

```csharp
public GestureKind GestureKind { get; set; }
```

**Value:** [GestureKind](Quaternar.Qaros.GestureKind.md)

### Enabled {#enabled}

A disabled hover or click rule does not stop delivery: those two always reach the
application, through a built-in rule ranked after every configured one.

```csharp
public bool Enabled { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### Precision {#precision}

Hand centimetres per app-volume centimetre. With constant sensitivity 1.0 maps 1 cm of
hand movement to 1 cm, 2.0 to 0.5 cm (finer control) and 0.5 to 2 cm.
[`AppVolumeGestureMappingRule.AppScaleSensitivityMode`](#appscalesensitivitymode) scales the result after precision is applied.

```csharp
public float Precision { get; set; }
```

**Value:** [float](https://learn.microsoft.com/dotnet/api/system.single)

### AppScaleSensitivityMode {#appscalesensitivitymode}

```csharp
public AppScaleSensitivityMode AppScaleSensitivityMode { get; set; }
```

**Value:** [AppScaleSensitivityMode](Quaternar.Qaros.AppScaleSensitivityMode.md)

### TranslationAxes {#translationaxes}

```csharp
public AppVolumeAxisFlags TranslationAxes { get; set; }
```

**Value:** [AppVolumeAxisFlags](Quaternar.Qaros.AppVolumeAxisFlags.md)

### RotationAxes {#rotationaxes}

```csharp
public AppVolumeAxisFlags RotationAxes { get; set; }
```

**Value:** [AppVolumeAxisFlags](Quaternar.Qaros.AppVolumeAxisFlags.md)

### EnableGesturesFromOutsideOfVolume {#enablegesturesfromoutsideofvolume}

Whether a gesture made with the hand outside the volume's cuboid is accepted; tested at the hand's position.

```csharp
public bool EnableGesturesFromOutsideOfVolume { get; set; }
```

**Value:** [bool](https://learn.microsoft.com/dotnet/api/system.boolean)

### ControllerRayStyle {#controllerraystyle}

```csharp
public AppVolumeControllerRayStyle ControllerRayStyle { get; set; }
```

**Value:** [AppVolumeControllerRayStyle](Quaternar.Qaros.AppVolumeControllerRayStyle.md)

### AppTransformMapping {#apptransformmapping}

```csharp
public GestureAppTransformMapping AppTransformMapping { get; set; }
```

**Value:** [GestureAppTransformMapping](Quaternar.Qaros.GestureAppTransformMapping.md)
