---
title: "PeerPresentationOptions class"
sidebar_label: "PeerPresentationOptions"
description: "How this peer presents itself to the others. Never a storage key. A property left null keeps its persisted value, or gets a generated default on the first onboard."
---

# PeerPresentationOptions class

How this peer presents itself to the others. Never a storage key. A property left null keeps
its persisted value, or gets a generated default on the first onboard.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [OnboardingTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/OnboardingTypes.cs#L48)

```csharp
public sealed class PeerPresentationOptions
```

## Properties

| Member | Description |
|---|---|
| [DisplayName](#displayname) | Human label shown to other peers; any UTF-8. It also feeds a sanitized debug label in the device certificate and the storage slot folder name. |
| [AppVersion](#appversion) |  |
| [AppCustomPeerInfo](#appcustompeerinfo) | Application-defined metadata, for example a JSON blob to coordinate instances of your own application across the session. Read back as [`PeerSpec.AppCustomPeerInfo`](Quaternar.Qaros.PeerSpec.md#appcustompeerinfo). |

### DisplayName {#displayname}

Human label shown to other peers; any UTF-8. It also feeds a sanitized debug label in the
device certificate and the storage slot folder name.

```csharp
public string? DisplayName { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### AppVersion {#appversion}

```csharp
public string? AppVersion { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

### AppCustomPeerInfo {#appcustompeerinfo}

Application-defined metadata, for example a JSON blob to coordinate instances of your
own application across the session. Read back as [`PeerSpec.AppCustomPeerInfo`](Quaternar.Qaros.PeerSpec.md#appcustompeerinfo).

```csharp
public string? AppCustomPeerInfo { get; set; }
```

**Value:** [string](https://learn.microsoft.com/dotnet/api/system.string)

## Methods

### WithExtension {#withextension}

```csharp
public PeerPresentationOptions WithExtension(Structure extension)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `extension` | [Structure](Quaternar.Qaros.Structure.md) |  |

**Returns:** [PeerPresentationOptions](Quaternar.Qaros.PeerPresentationOptions.md)
