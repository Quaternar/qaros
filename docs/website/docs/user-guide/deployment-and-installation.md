---
sidebar_position: 7
title: Deployment and Installation
description: How QarOS is distributed and installed — ZIP runtime, NuGet SDK, MSIX installer — and where its data lives.
---

# Deployment and Installation

QarOS is distributed in three artifact types, each aimed at a different consumer:

| Artifact | For whom | Contents |
|---|---|---|
| **ZIP** (`QAROS Hub <version>.zip`) | Operators installing a Hub; integrators who want a self-contained runtime | `bin/` with the Hub processes (`qaros-hub-service`, `qar-runtime-launcher`, `qar-streaming-viz`), the CA tooling, runtime DLLs, assets, and the default launcher configuration |
| **NuGet** | Developers integrating the C API into their build | The `qar-streaming-c` library, headers, and launcher runtime for development |
| **MSIX** (`QAROS Hub <version>.msixbundle`) | End users on Windows — one-click Hub install | The same Hub processes packaged as a signed, full-trust desktop app |
| **APK** (`QAROS Hub Quest/Spaces <version>.apk`) | Quest / Android target devices | The device player app |

## Installing a Hub from the ZIP (recommended today)

1. Unzip the archive to a writable location.
2. Review `bin/qar-runtime-launcher.jsonc` — configure your source apps and target devices (see [Using the Hub](/docs/user-guide/using-the-hub#the-launcher-configuration)).
3. Start `bin/qaros-hub-service.exe`. It starts the launcher and puts the QarOS icon in the tray.
4. Make sure the firewall permits the QarOS port range **7440–7460 (TCP+UDP)**; discovery additionally uses UDP multicast `239.77.77.77:7445` and session traffic uses mTLS on `7447`. (The MSIX package registers these rules automatically; for ZIP installs create them once or accept the Windows prompt.)

Data written at runtime (identity slots, CA state, session state, logs) is kept out of the install directory, under the per-user application-data root (`%LOCALAPPDATA%\Quaternar\...`), so upgrading is "replace the folder".

## Installing via MSIX

The MSIX bundle installs the Hub as a regular Windows app (with distinct release and demo package identities) and registers the firewall rules and an optional autostart task.

:::note TODO
The MSIX pipeline (packaging, signing, firewall declarations) is built and verified, but two integration items are still open before MSIX is the recommended path: seeding the launcher configuration into the package's `LocalState` (the packaged install root is read-only), and the tray toggle for the startup task. Update this section and promote MSIX to the primary instructions once those land.
:::

## The SDK package for integrators (this repository)

This repository (`qaros`) is the public SDK: it contains the C API headers (including the single-file `qar_streaming.h`), compiled examples, the C# bindings, and this documentation. The proprietary runtime binaries are delivered separately:

1. Request the QarOS binary package from [quaternar.com](https://www.quaternar.com/).
2. Unzip it into the repository's `package/` directory, giving:

```
package/
  bin/       # runtime DLLs + services your app loads/spawns
  lib/       # import libraries for static linking
  include/   # headers matching the binary version
  shared/    # shared assets
```

3. Build the examples against it (see [Developer Guide: Getting Started](/docs/developer-guide/getting-started)).

## Versioning and compatibility

- The C API is versioned as **v0**: symbols are exported as plain functions, and the single header is regenerated from the authoritative sources on every build, so headers and binaries in one package always match.
- Dynamically-loading applications are resilient by design: functions are loaded by name, missing symbols are detectable at load time, and deprecated functions keep exporting (returning a deprecation error) rather than disappearing.

:::note TODO
Write the formal compatibility policy page: what ABI stability v0 guarantees, the deprecation timeline, and the runtime↔SDK version compatibility matrix.
:::

## System requirements

:::note TODO
Document minimum and recommended Hub hardware (GPU with Vulkan support, CPU, RAM, NIC), supported Windows versions, per-device OS requirements (HoloLens 2 OS build, Quest OS, Android version), and network requirements (LAN latency/bandwidth guidance per stream).
:::
