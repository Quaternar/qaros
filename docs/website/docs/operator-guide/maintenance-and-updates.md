---
title: Maintenance & Updates
sidebar_position: 10
description: Upgrading a Hub with the installer and the device apps, the C API compatibility promise, and backing up Hub identity, storage, and logs.
---

# Maintenance & Updates

This page covers keeping a Hub running over time: how to upgrade, what compatibility you can rely on across versions, and what to back up.

## Upgrading across the artifact matrix

QAROS ships as several artifacts (see [Installation](/docs/operator-guide/installation)); each upgrades differently:

- **QAROS.** Run the newer installer over the installed version ([Installation: Upgrade](/docs/operator-guide/installation#upgrade)). All runtime state lives outside the install folder, so identity, CA state, session state, approved local apps and Hub configuration are kept.
- **Device players.** Install the new player from `devices\` of the installation on each device: `adb install -r` the APK on Meta Quest and METALENSE 2, redeploy the package on HoloLens 2. Keep Hub and players on the same release.
- **Applications on the C API.** A runtime update reaches every application that loads it, which is why the compatibility promise below matters.

## Compatibility promise

Integrators loading the C API can rely on a deliberate stability policy. It follows directly from how the C API and its dynamic-loading path are built:

- The C API is versioned **v0**.
- **v0 exports are never renamed.** A symbol that exists keeps its name.
- **Deprecated entries do not disappear** - they keep exporting, and return a deprecation error rather than vanishing, so a binary that loads them by name still links and can detect the deprecation at runtime.
- **Loaded structs are application-owned**, so the application controls their lifetime across the boundary.
- The **single header is regenerated from the authoritative source on every build**, so the header and the binary shipped in one package always match - there is no drift between what you compile against and what you load.

In practice this means a dynamically-loading application built against one package keeps working against a compatible newer runtime: missing symbols are detectable at load time, and deprecated ones degrade gracefully instead of breaking the load. For the developer-facing detail, see [Language Bindings](/docs/developer-guide/language-bindings) and [API Conventions](/docs/developer-guide/api-conventions).

## What to back up

Three categories matter under the per-user application-data root:

- **Hub identity and CA state** - the Hub's trust root and issued device trust. Losing it means every device must re-onboard and any federation must be re-established. Back it up, and protect the backup as you would the Hub machine itself.
- **Runtime storage and session state** - preserves Shared Space continuity and peer continuity across a machine move.
- **Logs** - collect logs from a run *before* restarting if you need them for diagnosis. Logs are also where the current pairing code appears today (see [Managing Devices](/docs/operator-guide/managing-devices)).

## Version numbers

Every artifact carries the product version `X.Y.Z`, with `-rc.N` for release candidates. The release notes list what changed. An application reads a peer's product version through the peer API (`qar_peer_spec_get_version_id`).
