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

## Updating to encrypted storage

From this release every secret QAROS keeps (the Hub's CA keys, device and app identities, approved local apps, the licence) is encrypted in `secure-store` under the data root, protected by a key of the signed-in Windows user that lives in the PC's TPM (or in Windows DPAPI where no TPM is usable). See [Security Deep Dive: What is stored where](./security-deep-dive.md#what-is-stored-where).

The plaintext key files of earlier versions are **deleted** at the Hub's first start, not converted. After the update:

1. **Pair every device again** with a new code (headsets, desktop players, apps on other PCs). Apps the Hub starts itself re-enrol on their own.
2. **Approve local source apps again** when they first connect.
3. **Join member Hubs to their primary again.** The Hub has a new certificate authority and a new Hub key.
4. **Activate the QAROS licence again** with `qaros-license.exe` (release builds).

The first start may take a few seconds longer while Windows creates the TPM key.

## Certificate renewal needs the current SDK

An updated hub renews a device's or app's certificate only when the renewal proves it holds the certificate's key. Devices and apps built with an older SDK do not make that proof, so they **cannot renew** at an updated hub: they stop working when their certificate ends (within 24 hours, plus a 7-day grace). Pairing them again does not help.

1. Update the device players and every app that loads the QAROS runtime to the current release before, or together with, the hub ([Upgrading across the artifact matrix](#upgrading-across-the-artifact-matrix)).
2. A device that reports it could not renew its certificate (status `QAR_STATUS_ONBOARDING_RENEWAL_POSSESSION_UNPROVEN`) is updated first; if it still fails afterwards, pair it again with a new code ([Managing Devices](/docs/operator-guide/managing-devices)).
3. Developers: the minimum SDK and the status are described in [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions#minimum-sdk-for-certificate-renewal).

## Compatibility promise

Integrators loading the C API can rely on a deliberate stability policy. It follows directly from how the C API and its dynamic-loading path are built:

- The C API is versioned **v0**.
- **v0 exports are never renamed.** A symbol that exists keeps its name.
- **Deprecated entries do not disappear** - they keep exporting, and return a deprecation error rather than vanishing, so a binary that loads them by name still links and can detect the deprecation at runtime.
- **Loaded structs are application-owned**, so the application controls their lifetime across the boundary.
- The **single header is regenerated from the authoritative source on every build**, so the header and the binary shipped in one package always match - there is no drift between what you compile against and what you load.

In practice this means a dynamically-loading application built against one package keeps working against a compatible newer runtime: missing symbols are detectable at load time, and deprecated ones degrade gracefully instead of breaking the load. For the developer-facing detail, see [Language Bindings](/docs/developer-guide/language-bindings) and [API Conventions](/docs/developer-guide/api-conventions).

## What to back up

Two categories matter under the data root ([Installation: Where data and logs live](/docs/operator-guide/installation#where-data-and-logs-live)), plus the logs:

- **Hub identity and CA state** - the Hub's trust root and issued device trust. Losing it means every device must re-onboard and any federation must be re-established. Back it up, and protect the backup as you would the Hub machine itself.
  - The secrets in `secure-store` are bound to this PC's TPM (or DPAPI) and this Windows user. A backup restores them only on the same PC, for the same user, while the TPM has not been cleared. Restored anywhere else, the Hub cannot read them, does not start, and must begin with a new `secure-store` (a new CA: every device pairs again).
- **Runtime storage and session state** - preserves Shared Space continuity and peer continuity across a machine move.
- **Logs** - under the log root (`%TEMP%\quaternar\logs`), not the data root. Collect them from a run *before* restarting if you need them for diagnosis.

## Version numbers

Every artifact carries the product version `X.Y.Z`, with `-rc.N` for release candidates. The release notes list what changed. An application reads a peer's product version through the peer API (`qar_peer_spec_get_version_id`).
