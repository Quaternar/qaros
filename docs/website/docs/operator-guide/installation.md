---
sidebar_position: 3
title: Installation
description: How QAROS is distributed and installed - ZIP runtime, NuGet SDK, MSIX installer - and where its data lives.
---

# Deployment and Installation

QAROS is distributed in three artifact types, each aimed at a different consumer:

| Artifact | For whom | Contents |
|---|---|---|
| **ZIP** (`QAROS Hub <version>.zip`) | Operators installing a Hub; integrators who want a self-contained runtime | `bin/` with the Hub processes (`qaros-hub-service`, `qar-runtime-launcher`, `qar-streaming-viz`), the CA tooling, runtime DLLs, and assets |
| **NuGet** | Developers integrating the C API into their build | The `qar-streaming-c` library, headers, and launcher runtime for development |
| **MSIX** (`QAROS Hub <version>.msixbundle`) | End users on Windows - one-click Hub install | The same Hub processes packaged as a signed, full-trust desktop app |
| **APK** (`QAROS Hub Quest/Spaces <version>.apk`) | Meta Quest, P&C Solutions METALENSE 2, and Android target devices | The device player app |

## Installing a Hub from the ZIP (recommended today)

1. Unzip the archive to a writable location.
2. Start `bin/qaros-hub-service.exe`. It starts the launcher and puts the QAROS icon in the tray.
3. Open the Hub UI and configure your source apps and target devices there. Hub configuration is persistent across restarts.
4. Make sure the firewall permits the QAROS port range **19120-19200 (TCP+UDP)**; discovery additionally uses UDP multicast `239.77.77.77:7445` and session traffic uses mTLS on `7447`. (The MSIX package registers these rules automatically; for ZIP installs create them once or accept the Windows prompt.)

Data written at runtime (identity slots, CA state, session state, logs, and persisted Hub configuration) is kept out of the install directory, under the per-user application-data root (`%LOCALAPPDATA%\Quaternar\...`), so upgrading is "replace the folder".

## Installing via MSIX

The MSIX bundle installs the Hub as a regular Windows app (with distinct release and demo package identities) and registers the firewall rules and an optional autostart task.

The MSIX pipeline (packaging, signing, firewall declarations) is built and verified, but the packaged install path is still not the recommended deployment path. Until that changes, install a Hub from the ZIP. See [Maintenance & Updates](/docs/operator-guide/maintenance-and-updates) for the current MSIX status.

## Installing player apps on devices

For **HoloLens 2**, **P&C Solutions METALENSE 2**, and **Meta Quest**, use the dedicated step-by-step guide in [Player Installation to Device](/docs/operator-guide/player-installation-to-device). That section links to the device-specific install pages for HoloLens Device Portal deployment and Android ADB-based installation.

## The SDK package for integrators (this repository)

This repository (`qaros`) is the public SDK: it contains the C API headers (including the single-file `qar_streaming.h`), compiled examples, the C# bindings, and this documentation. The proprietary runtime binaries are delivered separately:

1. Request the QAROS binary package from [quaternar.com](https://www.quaternar.com/).
2. Unzip it into the repository's `package/` directory, giving:

```text
package/
  bin/       # runtime DLLs + services your app loads/spawns
  lib/       # import libraries for static linking
  include/   # headers matching the binary version
  shared/    # shared assets
```

3. Build the examples against it (see [Developer Guide: Getting Started](/docs/developer-guide/getting-started)).

## System requirements

Hub hardware, OS, network, and per-device requirements have their own page: see [System Requirements](/docs/operator-guide/system-requirements).

## Versioning, compatibility, and upgrades

The C API compatibility promise and how to upgrade across the artifact matrix are covered in [Maintenance & Updates](/docs/operator-guide/maintenance-and-updates).
