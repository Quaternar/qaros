---
sidebar_position: 5
title: Launcher Configuration Reference
description: Every key of the QAROS runtime launcher configuration - type, default, effect, and an example.
---

# Launcher Configuration Reference

The runtime launcher (`qar-runtime-launcher.exe`) is configured by one JSON object. This page lists every key it reads.

## Where the configuration comes from

| Started by | Configuration used |
|---|---|
| Launcher alone, empty input | `qar-runtime-launcher.jsonc` next to the launcher executable (comments allowed). |
| Tray (`qaros-hub-service.exe <config-path>`) | The file given as the tray's first argument. Must be **plain JSON** (no comments); a file that fails to parse is replaced by `{"logSeverity": "debug"}`. |
| Tray, no argument | A minimal built-in config (`logSeverity: debug`), so all other keys take their defaults. |

The tray always adds `startupTarget` and `startupTargetPath` (see [Hub interconnect](#hub-interconnect-startuptarget)) and sets `prewarmVisualizer: true` unless the file sets it.

- Unknown keys are ignored.
- "Default" below is the value used when the key is **omitted**. The shipped `qar-runtime-launcher.jsonc` sets some keys differently; those are noted.

## Schema version

| Key | Type | Default | Effect |
|---|---|---|---|
| `schema_version` | integer | - | Config schema version. Current: `3`. Older versions are migrated on load; a newer version is rejected. `_sv` is accepted as an alias; `schema_version` wins if both are set. |

Migrations applied on load:

| From | Change |
|---|---|
| 1 -> 2 | `onboardingInviteIntervalSec` renamed to `codeRotationSec`. `maxAutoPipelines` and `handleSourceAppLaunchRequests` removed. |
| 2 -> 3 | `targetApps` entries with `deviceType: "ZedCamera"` dropped (a ZED is a source app, not a target). |

## Identity and logging

| Key | Type | Default | Effect |
|---|---|---|---|
| `hubCommonName` | string | `"hub.qaros.local"` | Common name of the Hub's mTLS identity. Give federated Hubs distinct names. |
| `logSeverity` | string or integer | `"debug"` | Log level for the launcher and the processes it spawns. `trace`, `debug`, `info`, `warn` (or `warning`), `error`, `fatal`, or `0`-`5` in the same order. |
| `zenohRustLogLevel` | string | `"info"` | Log level of the message-router (Zenoh) crates; other Rust crates stay at `warn`. One of `trace`, `debug`, `info`, `warn`, `error`, `off`. Any other value fails startup. |
| `peerId` | string (UUID) | - | Overrides the Hub's peer id. Omit it: the Hub keeps a durable id in its own store. |

```json
{ "hubCommonName": "hub.lab2.qaros.local", "logSeverity": "info", "zenohRustLogLevel": "warn" }
```

## Paths

All paths default to subfolders of the per-user application-data root (see [Installation](/docs/operator-guide/installation)).

| Key | Type | Default | Effect |
|---|---|---|---|
| `caDir` | path | `<app-data>/hub-ca` | Hub certificate authority directory. |
| `runtimeStorageRoot` | path | `<app-data>` | Root of identity store, trust store and launcher session state. |
| `logFolder` | path | `<app-data>/qar-launcher-default/<session-id>` | Log folder. The default folder is emptied on every start; a configured one is not. |
| `startupTargetPath` | path | - | File the launcher writes the Hub-interconnect target to after connecting to another Hub. Without it the interconnect is not restored on the next start. Set by the tray to `startup-target.json` next to its config file. |

```json
{ "caDir": "C:/ProgramData/Quaternar/hub-ca", "logFolder": "C:/ProgramData/Quaternar/logs/runtime-launcher" }
```

## Networking

| Key | Type | Default | Effect |
|---|---|---|---|
| `routerPort` | integer | - | Fixed message-router port. Omit it: the Hub takes the first port in the range below free on both TCP and UDP, so several local Hubs coexist. Pin it only for a firewalled deployment. |
| `routerPortRangeStart` | integer | `19120` | First port searched when `routerPort` is unset. |
| `routerPortRangeEnd` | integer | `19199` | Last port searched when `routerPort` is unset. |
| `sessionEndpoints` | array of locator strings or address objects | - | Endpoints put into invites sent to devices. Omit (or leave empty) to derive them from the router's listening address. Each entry is either a locator string `"<protocol>/<host>:<port>"` (IPv6 host in brackets) or `{"Protocol": "tcp", "Hostname": "<ip-or-host>", "Port": <port>}` with all three fields. A malformed entry fails the config load. |
| `discoveryHost` | string | `""` | Host name or IP advertised in the discovery beacon as the discovery server address. |
| `discoveryPort` | integer | `7445` | Discovery port. |
| `discoveryDisplayName` | string | - | Overrides the Hub name shown in discovery. Omit it: the Hub keeps its persisted name (initially "QAROS Hub"). |
| `hubRoomName` | string | - | Overrides the Hub's room name. Omit it: the Hub keeps its persisted room. |

```json
{
  "routerPort": 19120,
  "sessionEndpoints": [ { "Protocol": "tcp", "Hostname": "192.168.1.20", "Port": 19120 } ]
}
```

See [Networking & Federation](/docs/operator-guide/networking-and-federation) for firewall rules.

## Onboarding

| Key | Type | Default | Effect |
|---|---|---|---|
| `onboardingPakePort` | integer | `0` | Port of the pairing-code (PAKE) onboarding listener. `0` picks a free port. |
| `codeRotationSec` | integer | `8` | Seconds between freshly minted pairing codes. A code lives at least 10 s, or longer if this is larger. |
| `pakeCodeLength` | integer | `8` | Characters in the human-entered pairing code. |
| `certLifetimeHours` | integer | CA default | Lifetime of certificates issued to onboarded devices. |

```json
{ "codeRotationSec": 8, "pakeCodeLength": 8, "certLifetimeHours": 168 }
```

See [Security Model](/docs/operator-guide/security-model) and [Managing Devices](/docs/operator-guide/managing-devices).

## Visualizer

| Key | Type | Default | Effect |
|---|---|---|---|
| `autoLaunchVisualizer` | boolean | `false` (shipped file: `true`) | Open the single control visualizer when the Hub starts. |
| `prewarmVisualizer` | boolean | `false` (tray: `true`) | When `autoLaunchVisualizer` is off, start the visualizer hidden so opening it later is instant. |
| `openVisualizerOnStartup` | boolean | `true` | Read by the tray, not the launcher: show the visualizer once the Hub has started. |

## Pipeline behaviour

| Key | Type | Default | Effect |
|---|---|---|---|
| `enableMixer` | boolean | `true` | Insert the mixer between source apps and targets. See [The Mixer](/docs/operator-guide/the-mixer). |
| `unifyTargetAppWithMixer` | boolean | `false` | Run the mixer inside the target's process where the target supports it, instead of as its own process. Only with `enableMixer`. |
| `applyDeterministicIds` | boolean | `true` | Keep launcher-assigned peer ids stable across restarts, so child processes reuse their onboarding identities instead of re-enrolling. |
| `targetAppOverride` | string | `"None"` | Launch this target app for every target request instead of the one matching the device. `None`, `StreamVisualizer`, `HoloLens`, `MetaQuest`, `StereoKit`, `AndroidStreamer`. |
| `handleInvites` | boolean | `true` | Answer session-invite requests. |
| `detectConnectedPeers` | boolean | `true` | Track peers connecting to the session. |
| `replyToServiceExists` | boolean | `true` | Answer service-existence probes. |

## Source apps (`sourceApps[]`)

Each enabled entry is a source application added to every new target pipeline, with its app volume registered at startup. A missing or empty array means no source apps.

| Key | Type | Default | Effect |
|---|---|---|---|
| `enabled` | boolean | `true` | `false` skips the entry. |
| `type` | string | `"StereoKit"` | Source app kind: `StereoKit`, `CpuRenderer`, `Ogre`, `ZedCamera`, `VRGB`, `OpenXr`. |
| `appVolumeCommonName` | string | **required** | Stable name of the app volume (get-or-create key, not a UUID). Use a reverse-domain name unique across Hubs, e.g. `slot0.hub1.example.com`. An empty or missing name fails startup. Alias: `appVolumeId`. |
| `appVolumeDisplayName` | string | `"AppVolume_<index>"` | Name shown in the UI. Alias: `displayName`. |
| `appVolumePose` | object | placed on a diagonal by index | `position` `[x,y,z]` and `orientation` `[x,y,z,w]` (or `{x,y,z[,w]}` objects). Each field optional. |
| `appVolumeSize` | `[x,y,z]` or object | `[1.5,1.5,1.5]` | Volume size in metres. Alias: `size`. |

Entries must be objects; bare type strings are rejected.

```json
"sourceApps": [
  {
    "enabled": true,
    "type": "StereoKit",
    "appVolumeCommonName": "slot0.hub1.example.com",
    "appVolumeDisplayName": "StereoKit Source",
    "appVolumePose": { "position": [0.2, 0.0, 0.2], "orientation": [0, 0, 0, 1] },
    "appVolumeSize": [1.5, 1.5, 1.5]
  }
]
```

## Target apps (`targetApps[]`)

Each enabled entry is a device the Hub invites into the session at startup. Startup waits for each invite to be accepted.

| Key | Type | Default | Effect |
|---|---|---|---|
| `enabled` | boolean | `true` | `false` skips the entry. |
| `hostname` | string | `""` | Host name or IP of the device. |
| `deviceType` | string | `"Visualizer"` | `HoloLensARHeadset`, `MetaQuestHeadset` or `AndroidStreamer`. `Visualizer` and `CameraVisualizer` are ignored with a warning: use `autoLaunchVisualizer` instead. |
| `configOverrides` | object | `{}` | Passed into the launched app's config (e.g. `roomTag`). Aliases: `specialKeys`, `config`. |

```json
"targetApps": [
  { "enabled": true, "hostname": "192.168.1.42", "deviceType": "MetaQuestHeadset", "configOverrides": { "roomTag": "default" } }
]
```

## Hub interconnect (`startupTarget`)

Selects whether the launcher hosts a local Hub or rejoins another Hub at startup. Normally written by the tray from `startup-target.json`; you rarely set it by hand. See [Networking & Federation](/docs/operator-guide/networking-and-federation).

| Key | Type | Default | Effect |
|---|---|---|---|
| `type` | string | `"auto"` | `"local"` or `"auto"`. Any other value means `"auto"`. |
| `hubConnectionId` | string | - | Id of a saved connection to another Hub. When set, the launcher rejoins that Hub instead of hosting a local one. |
| `trustedHubCaBundles` | array | `[]` | CA bundles of other Hubs to trust. Each entry: `HubId`, `CaBundlePem`, `CertificateFingerprints[]`. A malformed array is ignored with a warning. |

```json
"startupTarget": { "type": "auto" }
```
