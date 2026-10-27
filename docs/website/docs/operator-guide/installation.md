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
| **Installer** (`QAROS-Setup-<version>.exe`, demo: `QAROS-Demo-Setup-<version>.exe`) | every QAROS PC | the Hub (tray, launcher, Visualizer), the runtime and C API library, the `qar-vulkan-source` example app, the C API header, the device player apps |
| **Unity ZIP** (`QAROS Unity <version>.zip`) | Unity developers | the [Unity packages](/docs/developer-guide/unity) and `devices/` |
| **C# NuGet** (`Quaternar.Qaros`) | .NET developers | the C# binding; it loads the installed runtime, release or demo |

Each release comes in two flavors, release and demo. One PC holds one QAROS installation in one flavor.

## Install

1. Run `QAROS-Setup-<version>.exe` (demo: `QAROS-Demo-Setup-<version>.exe`) and accept the Windows administrator (UAC) prompt.
   :::note Preview builds
   Preview builds are signed with a temporary Quaternar certificate that Windows does not know yet. SmartScreen shows "Windows protected your PC": choose **More info**, then **Run anyway**. The UAC prompt shows an unverified publisher. Releases signed with the final certificate show neither.
   :::
2. When the setup finishes, QAROS is running for every signed-in user: its icon is in the notification area. If it is not, start **QAROS** from the Start menu.
3. Left-click the tray icon to open the Visualizer. Continue with [Running the Hub](/docs/operator-guide/running-the-hub).

Silent install, from an elevated PowerShell:

```powershell
& ".\QAROS-Setup-<version>.exe" /S
```

| Option | Effect |
|---|---|
| `/S` | silent; accepts the QAROS licence |
| `/NoFirewall` | adds no firewall rules: devices on the LAN cannot connect, local source apps still work |
| `/NoAutostart` | QAROS does not start at sign-in for every user (it still starts once when the setup finishes); each user can still turn it on in the tray menu |
| `/AllowDowngrade` | lets an older setup replace a newer installation (silent setups refuse otherwise) |
| `/ANARI`, `/NoANARI` | adds or removes the optional ANARI device; without either an upgrade keeps the previous choice |

| Exit code | Meaning |
|---|---|
| 0 | installed |
| 1 | cancelled |
| 2 | aborted with an error |
| 3 | an older per-user QAROS is installed; uninstall it first |
| 4 | Windows version not supported |
| 5 | administrator prompt declined |
| 6 | a newer QAROS is installed; nothing changed (`/AllowDowngrade` to install anyway) |
| 8 | demo setup over a licensed QAROS: uninstall QAROS first |
| 9 | a running program holds QAROS files open (silent), or you cancelled the list of such programs; nothing changed |

## What the installer does

| What | Where |
|---|---|
| Program files: `bin\` (Hub processes, runtime, C API library, `qar-vulkan-source.exe`), `include\` (C API header), `devices\` (player apps) | `C:\Program Files\Quaternar\QAROS` (read-only for users) |
| Windows service `QAROS.SystemService` (`qaros-system-service.exe`) | runs as a service |
| Firewall rules for every QAROS executable, TCP and UDP, in and out, on the QAROS ports below (`/NoFirewall` skips them) | Windows Defender Firewall, all network profiles |
| Runtime location for apps: value `InstallFolder` | `HKLM\SOFTWARE\Quaternar\QAROS` |
| QAROS tray (`qaros.exe`) starting at sign-in, for every user (`/NoAutostart` skips it). Each user turns it off for themselves with **Start QAROS at login** in the tray menu; it stays off across sign-ins, upgrades and reinstalls | `HKLM\...\Run\QAROS` |
| Start menu shortcut, Add/Remove Programs entry, signed uninstaller | |

The Hub processes (tray, launcher, Visualizer) run in the signed-in user's session, not as the service.

### Ports

| Ports | Use |
|---|---|
| UDP multicast `239.77.77.77:7445` | discovery beacon |
| one TCP port from 19120-19199 | discovery requests and code pairing: the Hub claims its onboarding port from this range, so a rule for TCP 7445 alone does not let a device pair |
| TCP 19120-19200 | session traffic, mutual TLS (router and peer links) |
| 7440-7460, 19120-19199 TCP + UDP | what the firewall rules open for each QAROS executable |

Details: [Networking and Federation](/docs/operator-guide/networking-and-federation). If the firewall step fails, the setup warns that LAN devices cannot connect; local source apps still work.

## Where data and logs live

Program files and data are kept apart. Data survives upgrades and uninstall.

| Folder | Holds |
|---|---|
| `%LOCALAPPDATA%\Quaternar\Qaros` | per user: the Hub's CA certificates, Hub and session state, the identities of apps the user runs, logs, settings, the editable launcher configuration `qar-runtime-launcher.jsonc` |
| `%ProgramData%\Quaternar\Qaros` | per machine: service state, and in `secure\` the Hub's secrets (CA keys, Hub identity, approved local apps), one partition per Windows user, and the licence once for the whole PC, encrypted under a machine key only the QAROS service can use ([Security Deep Dive](./security-deep-dive.md#what-is-stored-where)) |
| `QAR_APP_DATA_ROOT` environment variable | overrides the per-user folder |

Logs are kept apart from the data, under the **log root** `%TEMP%\quaternar\logs` (with `QAR_APP_DATA_ROOT` set: that folder).

| Logs of | Folder |
|---|---|
| Tray (`qaros`), including the launcher's console output | `<log root>\qaros-hub-service`; launcher crash dumps in its `crashes` subfolder |
| Launcher and every process it starts (Visualizer, source apps, mixer) | `<log root>\qar-launcher-default\<session-id>`, emptied when the Hub starts |

- Deleting the per-user data folder resets the Hub: devices must onboard again and local apps must be approved again.
- The Visualizer keeps its settings and window layout (`imgui.ini`) under `<data folder>\streaming-viz`.
- Copy the log folder **before** restarting the Hub if you need it.
- `logFolder` in the [launcher configuration](/docs/operator-guide/launcher-config-reference#paths) moves the launcher logs.

## Upgrade

Run the newer `QAROS-Setup-<version>.exe`. It asks the running Hub to exit and replaces the program files. Start QAROS from the Start menu if it is not running afterwards.

- Approved local apps, the session and paired devices are kept: approved apps join without a new prompt. Exception: the first update to encrypted storage starts the Hub with a new CA, so devices pair again, apps are approved again and the licence is activated again once ([Maintenance & Updates](./maintenance-and-updates.md#updating-to-encrypted-storage)).
- Installing the **demo** over a **release** installation deletes the QAROS data first (a demo never runs on release data). Demo to release keeps it.
- Source apps need no change: they load the runtime from the installation. A running one holds the runtime open: the setup lists such programs and waits until you close them (**Retry**), or leaves QAROS as it is (**Cancel**). The same applies to uninstalling.

## Uninstall

**Settings > Apps > Installed apps > QAROS > Uninstall**, or silently from an elevated PowerShell:

```powershell
& "C:\Program Files\Quaternar\QAROS\uninstall.exe" /S
```

It removes the program files, the service, the firewall rules, the registry key, the autostart entry and the shortcuts. First it asks whether to delete the data folders above (default: keep them; silently, add `/PurgeData` to delete them):

| Choice | What happens |
|---|---|
| Keep (default) | Both data folders stay, including the Hub's protected secrets and the machine key that encrypts them. Installing QAROS again finds approved apps, paired devices and the licence as before. |
| Delete | The QAROS service deletes its machine key first, then the protected secrets, and the uninstaller removes `%ProgramData%\Quaternar\Qaros` and your `%LOCALAPPDATA%\Quaternar\Qaros`. A copy of the old secrets can no longer be decrypted. Other Windows users' `%LOCALAPPDATA%` folders stay. |

```powershell
& "C:\Program Files\Quaternar\QAROS\uninstall.exe" /S /PurgeData
```

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
