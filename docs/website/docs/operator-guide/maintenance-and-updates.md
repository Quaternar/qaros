---
title: Maintenance & Updates
sidebar_position: 10
description: Upgrading a Hub across the ZIP / MSIX / APK artifacts, the C API compatibility promise, and backing up Hub identity, storage, and logs.
---

# Maintenance & Updates

This page covers keeping a Hub running over time: how to upgrade, what compatibility you can rely on across versions, and what to back up.

## Upgrading across the artifact matrix

QarOS ships as several artifacts (see [Installation](/docs/operator-guide/installation)); each upgrades differently:

- **ZIP (Hub runtime).** Upgrading is "replace the folder." All runtime state lives outside the install directory under the per-user application-data root, so unzipping a new version over a fresh location and pointing the tray service at it preserves identity, CA state, session state, and persisted Hub configuration.
- **MSIX (packaged Hub).** Install the new `.msixbundle`; Windows handles the in-place package upgrade and the firewall/autostart declarations. Release and demo package identities are distinct and upgrade independently.
- **APK (device players).** Update the QarOS player app on each P&C Solutions METALENSE 2, Meta Quest, or Android device by installing the new APK.

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

## MSIX status

The MSIX pipeline (packaging, signing, firewall declarations) is built and verified, but the ZIP remains the recommended path for Hub installs today; see [Installation](/docs/operator-guide/installation).
