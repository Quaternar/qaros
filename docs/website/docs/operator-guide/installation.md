---
sidebar_position: 3
title: Installation
description: Install QAROS on a Windows PC with the per-machine installer - what it installs, firewall, upgrade, uninstall - and where its data lives.
---

# Installation

QAROS is installed on Windows by one signed installer, **per machine**: it needs an administrator once and serves every user of the PC. Every source app on the PC loads the QAROS runtime from this installation; none ships its own copy.

## What ships

| Artifact | For whom | Contents |
|---|---|---|
| **Installer** (`QAROS-Setup-<version>.exe`) | every QAROS PC | the Hub (tray, launcher, Visualizer), the runtime and C API library, the `qar-vulkan-source` example app, the C API header, the device player apps |
| **Unity ZIP** (`QAROS Unity <version>.zip`) | Unity developers | the Unity packages and `devices/` |
| **C# NuGet** (`Quaternar.Qaros.Streaming`, `Quaternar.Qaros.Streaming.Demo`) | .NET developers | the C# binding |

Each release comes in two flavors, release and demo. One PC holds one QAROS installation in one flavor.

## Install

1. Run `QAROS-Setup-<version>.exe` and accept the Windows administrator (UAC) prompt.
   :::note Preview builds
   Preview builds are signed with a temporary Quaternar certificate that Windows does not know yet. SmartScreen shows "Windows protected your PC": choose **More info**, then **Run anyway**. The UAC prompt shows an unverified publisher. Releases signed with the final certificate show neither.
   :::
2. When the setup finishes, the QAROS icon appears in the notification area. If it does not, start **QAROS** from the Start menu.
3. Left-click the tray icon to open the Visualizer. Continue with [Running the Hub](/docs/operator-guide/running-the-hub).

Silent install, from an elevated PowerShell:

```powershell
& ".\QAROS-Setup-<version>.exe" /S
```

| Exit code | Meaning |
|---|---|
| 0 | installed |
| 1 | cancelled |
| 2 | aborted with an error |
| 3 | an older per-user QAROS is installed; uninstall it first |
| 4 | Windows version not supported |
| 5 | administrator prompt declined |

## What the installer does

| What | Where |
|---|---|
| Program files: `bin\` (Hub processes, runtime, C API library, `qar-vulkan-source.exe`), `include\` (C API header), `devices\` (player apps) | `C:\Program Files\Quaternar\QAROS` (read-only for users) |
| Windows service `QAROS.SystemService` (`qaros-system-service.exe`) | runs as a service |
| Firewall rules for every QAROS executable, TCP and UDP, in and out | Windows Defender Firewall, all network profiles |
| Runtime location for apps: value `InstallFolder` | `HKLM\SOFTWARE\Quaternar\QAROS` |
| QAROS tray starting at sign-in, for every user (`/NoAutostart` skips it) | `HKLM\...\Run\QAROS` |
| Start menu shortcut, Add/Remove Programs entry, signed uninstaller | |

The Hub processes (tray, launcher, Visualizer) run in the signed-in user's session, not as the service.

### Ports

| Ports | Use |
|---|---|
| 7445 TCP + UDP multicast `239.77.77.77` | discovery and code pairing |
| 7447 TCP | session traffic, mutual TLS |
| 7440-7460, 19120-19199 TCP + UDP | QAROS port ranges (session router and peers) |

Details: [Networking and Federation](/docs/operator-guide/networking-and-federation). If the firewall step fails, the setup warns that LAN devices cannot connect; local source apps still work.

## Data

Program files and data are kept apart. Data survives upgrades and uninstall.

| Folder | Holds |
|---|---|
| `%LOCALAPPDATA%\Quaternar\Qaros` | per user: the Hub's CA, Hub and session state, device identities, approved local apps, logs, settings, the editable launcher configuration `qar-runtime-launcher.jsonc` |
| `%ProgramData%\Quaternar\Qaros` | per machine: service state |
| `QAR_APP_DATA_ROOT` environment variable | overrides the per-user folder |

## Upgrade

Run the newer `QAROS-Setup-<version>.exe`. It asks the running Hub to exit and replaces the program files. Start QAROS from the Start menu if it is not running afterwards.

- Approved local apps, the session and paired devices are kept: approved apps join without a new prompt.
- Installing the **demo** over a **release** installation deletes the QAROS data first (a demo never runs on release data). Demo to release keeps it.
- Source apps need no change: they load the runtime from the installation.

## Uninstall

**Settings > Apps > Installed apps > QAROS > Uninstall**, or silently from an elevated PowerShell:

```powershell
& "C:\Program Files\Quaternar\QAROS\uninstall.exe" /S
```

It removes the program files, the service, the firewall rules, the registry key, the autostart entry and the shortcuts. The data folders above are kept unless you choose to delete them; delete them by hand for a clean machine.

## Player apps on devices

The device player apps are in `C:\Program Files\Quaternar\QAROS\devices\`:

| File | Device |
|---|---|
| `android/QAROS Player Quest <version>.apk` | Meta Quest |
| `android/QAROS Player MetaLens <version>.apk` | P&C Solutions METALENSE 2 |
| `hololens/QAROS Player HoloLens <version>.msix` (+ `.cer`, `Dependencies/`) | HoloLens 2 |

Install them with [Player Installation to Device](/docs/operator-guide/player-installation-to-device).

## For developers: the SDK repository

This repository (`qaros`) holds the C API header (`qar-streaming-c/include/qar_streaming.h`), compiled examples and this documentation. It needs no runtime download: examples load the installed QAROS with `qar_library_load(NULL)`. Start with [Developer Guide: Getting Started](/docs/developer-guide/getting-started).

## See also

- [System Requirements](/docs/operator-guide/system-requirements)
- [Maintenance and Updates](/docs/operator-guide/maintenance-and-updates): C API compatibility, backups
