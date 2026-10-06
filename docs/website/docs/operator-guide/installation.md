---
sidebar_position: 3
title: Installation
description: How QAROS is distributed and installed - runtime ZIP, system installer, Unity ZIP, C# NuGet, device apps - and where its data lives.
---

# Deployment and Installation

QAROS ships each release in two flavors (release and demo) as these artifacts:

| Artifact | For whom | Contents |
|---|---|---|
| **Runtime ZIP** (`QAROS <version>.zip`, `QAROS Demo <version>.zip`) | Running QAROS with no install and no admin rights: a lab PC, a CI box, a shared folder | `bin/` with the Hub processes (`qaros-hub-service`, `qar-runtime-launcher`, `qar-streaming-viz`), the C API library and every runtime DLL, the CA tooling, assets and the `qar-vulkan-source` example app; `include/` with the C API header; `devices/` with the device player apps |
| **System installer** (`QAROS Hub <version>.msixbundle`, `QAROS Hub Demo <version>.msixbundle`) | Machines that should have one runtime installed and shared by all their source apps | The same Hub processes and runtime, installed once per system |
| **Unity ZIP** (`QAROS Unity <version>.zip`) | Unity developers | The Unity packages (runtime included) and `devices/` |
| **C# NuGet** (`Quaternar.Qaros.Streaming`, `Quaternar.Qaros.Streaming.Demo`) | .NET developers | The C# binding; copies the native runtime next to the application on build and publish |

Every ZIP has one top-level folder and carries `CHANGELOG.md`, `third_party_licenses.md` and `README-Licensing.md`. Installer and runtime ZIP are both fully supported; pick the ZIP when you cannot or do not want to install.

The device player apps are in `devices/` of every ZIP:

| File | Device |
|---|---|
| `android/QAROS Player Quest <version>.apk` | Meta Quest |
| `android/QAROS Player MetaLens <version>.apk` | P&C Solutions METALENSE 2 |
| `hololens/QAROS Player HoloLens <version>.msix` (+ `.cer`, `Dependencies/`) | HoloLens 2 |
| `README.md` | Install steps for each |

## Installing a Hub from the runtime ZIP

1. Unzip the archive to a writable location.
2. Start `bin/qaros-hub-service.exe`. It starts the launcher and puts the QAROS icon in the tray.
3. Open the visualizer and set up your source apps there (see [Source Applications](/docs/operator-guide/source-applications)). Hub configuration is persistent across restarts. File-based launcher settings are listed in the [Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference).
4. Make sure the firewall permits the QAROS port range **19120-19200 (TCP+UDP)**; discovery additionally uses UDP multicast `239.77.77.77:7445` and session traffic uses mTLS on `7447`. (The installer registers these rules automatically; for ZIP installs create them once or accept the Windows prompt.)

Data written at runtime (identity slots, CA state, session state, logs, and persisted Hub configuration) is kept out of the install directory, so upgrading is "replace the folder":

| Install | Data root |
|---|---|
| Runtime ZIP | `%LOCALAPPDATA%\Quaternar\Qaros` |
| System installer | `%LOCALAPPDATA%\Packages\<package family name>\LocalState\Quaternar` |
| Either, overridden | The folder in the `QAR_APP_DATA_ROOT` environment variable |

## Installing with the system installer

The installer puts the Hub on the machine as a regular Windows app, registers the firewall rules and an optional autostart task (`qaros-hub-service`). Release and demo install side by side under distinct identities.

The installer is being reworked into a classic installer that source applications can share the runtime from. Until then, applications built on the Unity package or the C# NuGet still carry their own runtime copy.

## Installing player apps on devices

For **HoloLens 2**, **P&C Solutions METALENSE 2**, and **Meta Quest**, use the dedicated step-by-step guide in [Player Installation to Device](/docs/operator-guide/player-installation-to-device). That section links to the device-specific install pages for HoloLens Device Portal deployment and Android ADB-based installation.

## The SDK package for integrators (this repository)

This repository (`qaros`) is the public SDK: it contains the C API header (the single-file `qar_streaming.h`), compiled examples, and this documentation. The runtime binaries are delivered separately:

1. Request the QAROS runtime ZIP from [quaternar.com](https://www.quaternar.com/).
2. Unzip its contents into the repository's `package/` directory, giving:

```text
package/
  bin/       # qar-streaming-c.dll, qar-runtime-launcher and every runtime DLL
  include/   # the header matching the binary version
  devices/   # device player apps
```

The runtime ZIP carries no import library: applications load the C API dynamically from `bin/` (see [Developer Guide: Getting Started](/docs/developer-guide/getting-started)). Deploy `bin/` whole; do not copy single DLLs out of it.

3. Build the examples against it (see [Developer Guide: Getting Started](/docs/developer-guide/getting-started)).

## System requirements

Hub hardware, OS, network, and per-device requirements have their own page: see [System Requirements](/docs/operator-guide/system-requirements).

## Versioning, compatibility, and upgrades

The C API compatibility promise and how to upgrade across the artifact matrix are covered in [Maintenance & Updates](/docs/operator-guide/maintenance-and-updates).
