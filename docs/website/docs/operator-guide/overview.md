---
title: Overview
sidebar_position: 1
description: The operator's mental model of a QAROS Hub - its processes, data, and responsibilities - and an index to the rest of this guide.
---

# Overview

This guide is for the people who install, secure, configure, run, and maintain a QAROS **Hub** - the machine that brings applications and devices together into one shared, streamed room. This page gives you the mental model; the rest of the guide fills in the detail.

![QAROS world overview](/img/diagrams/qaros-world-overview.png)

## The Hub is three processes

A Hub is not one program. Three processes ship together and start in a chain:

| Process | Role |
|---|---|
| `qaros-hub-service.exe` | The tray supervisor. It is the always-on parent: it starts the runtime launcher, restarts it with backoff if it crashes, and gives you the tray menu. This is what you launch (or what autostart launches). |
| `qar-runtime-launcher.exe` | The runtime. It hosts the session and message router, runs the private CA and onboarding services, broadcasts the discovery beacon, and orchestrates the streaming pipeline (source apps, mixers, targets). Everything operationally interesting happens here. |
| `qar-streaming-viz.exe` | The visualizer - the interactive desktop window where you see and manipulate the shared room, and where the hub-connect UI lives. |

The startup chain is: you start the tray service -> it starts the launcher -> the launcher restores or creates a session, registers source-app volumes, starts streaming and onboarding services, begins broadcasting discovery, and invites a local visualizer. If the launcher dies, the tray service brings it back.

## The mixer, one per user

Inside the runtime's streaming pipeline sits the **mixer**: the component that composites several independent application streams into one coherent, latency-compensated stereo view. There is one mixer instance per connected user, so the Hub's GPU load scales with the number of users. You never manage the mixer directly - the pipeline planner inserts and places it for you. See [The Mixer](/docs/operator-guide/the-mixer).

## The per-Hub Certificate Authority

Each Hub operates its own private certificate authority (based on `step-ca`), localhost-only. It is the trust root for the deployment: a device onboarded to this Hub carries a certificate this Hub signed. No internet, cloud, or corporate PKI is required. A short pairing code - typed once, never sent over the network - bootstraps each device's full cryptographic identity. See [Security Model](/docs/operator-guide/security-model) for the trust model and onboarding flow.

## Where data lives

Everything the Hub writes at runtime - identity slots, CA state, session state, and logs - is kept out of the install directory, under the per-user application-data root (`%LOCALAPPDATA%\Quaternar\...`). That is what makes upgrading a ZIP install "replace the folder", and it is what you back up. See [Maintenance & Updates](/docs/operator-guide/maintenance-and-updates).

## What the operator owns

Four things are yours to manage:

- **Configuration** - source apps, target devices, and other Hub behavior, managed through the Hub UI and persisted across restarts.
- **Network and ports** - firewall rules for the QAROS port range, discovery scope, and (for off-LAN) relay or Hub federation.
- **Device trust** - minting and reading pairing codes, connecting peers, and forgetting or blacklisting devices.
- **Updates** - moving between versions across the ZIP / MSIX / APK artifact matrix, and backing up Hub identity.

## What's in this guide

| Page | What it covers |
|---|---|
| [System Requirements](/docs/operator-guide/system-requirements) | Hardware, OS, network, and per-device requirements for a Hub. |
| [Installation](/docs/operator-guide/installation) | The artifact matrix (ZIP, NuGet, MSIX, APK) and how to install a Hub. |
| [Player Installation to Device](/docs/operator-guide/player-installation-to-device) | Operator-side player installation section with separate pages for HoloLens 2, P&C Solutions METALENSE 2, and Meta Quest. |
| [Running the Hub](/docs/operator-guide/running-the-hub) | Starting the tray service, the startup sequence, and the visualizer. |
| [Networking & Federation](/docs/operator-guide/networking-and-federation) | Ports and firewall, LAN vs relay, and connecting two Hubs. |
| [Security Model](/docs/operator-guide/security-model) | The four-layer trust model, the per-Hub CA, and onboarding. |
| [Managing Devices](/docs/operator-guide/managing-devices) | Pairing codes, connecting peers, and forgetting/blacklisting devices. |
| [The Mixer](/docs/operator-guide/the-mixer) | How multiple streams are composited into one per-user view. |
| [Maintenance & Updates](/docs/operator-guide/maintenance-and-updates) | Upgrades, the compatibility promise, backups, and logs. |

For device-owner instructions (bringing a headset or phone into a session), see the [User Guide](/docs/user-guide/onboarding-devices). For integrating the C API, see the [Developer Guide](/docs/developer-guide/getting-started). Unfamiliar terms are defined in the [Glossary](/docs/glossary).
