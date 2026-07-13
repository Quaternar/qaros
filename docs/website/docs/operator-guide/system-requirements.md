---
title: System Requirements
sidebar_position: 2
description: Simple hardware, storage, OS, and network requirements for running a QAROS Hub.
---

# System Requirements

This page lists the practical minimum requirements for running a QAROS Hub.

## Hub machine

The Hub runs on a **Windows** desktop or server machine with:

- **NVIDIA GPU:** at least **RTX 2060**
- **CPU:** at least **8 CPU cores**
- **RAM:** at least **8 GB**
- **Storage:** at least **10 GB** of free disk space

These are the baseline requirements for the Hub itself. Your source applications may require a stronger machine than QAROS alone, so size the system for both **QAROS** and the **applications you plan to run on the Hub**.

## Network

The network is a critical part of the deployment.

- Use a **Wi-Fi 6 router or better**.
- Connect the **Hub to the router by LAN cable**.
- Keep only a **single wireless hop** between the Hub and the target device.
- QAROS supports standard Wi-Fi deployments and also supports **LiFi**.
- For **Android** and **HoloLens** deployments, **5G via dongle** is also supported.

Example router choices:

- **ASUS RT-AX55** or better
- **GL.iNet XE3000** or better for mobile workspaces and on-the-go setups

QAROS services expect the Hub and devices to share a LAN-like network. For ports and firewall details, see [Networking & Federation](/docs/operator-guide/networking-and-federation).

## Devices

Supported target devices include:

- **HoloLens 2**
- **P&C Solutions METALENSE 2**
- **Meta Quest**
- **Android phones and tablets**
- **Desktop visualizer**

For player installation steps, see [Player Installation to Device](/docs/operator-guide/player-installation-to-device). For device-owner setup, see [Onboarding Devices](/docs/user-guide/onboarding-devices).
