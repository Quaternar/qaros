---
title: Getting Started
sidebar_position: 2
description: "Join your first QAROS session in about five minutes: the Hub operator pairs your device once, over USB, with a QR code or by the code your headset shows."
---

# Getting Started

You can be inside a QAROS Shared Space in about five minutes. All you need is the QAROS app on your device and the person running the Hub (the operator), who pairs your device with the Hub once.

## Before you start

- **Get on the right network.** Devices find the Hub by listening for its beacon on the local network, so the simplest path is to be on the same Wi-Fi or LAN as the Hub. If you are off-site, your operator can set up a **relay** that lets you reach the Hub from elsewhere — ask them whether one is available for you.
- **Install the QAROS app for your device.** A HoloLens, P&C Solutions METALENSE 2, Meta Quest, Android phone or tablet, desktop visualizer, and ZED camera each have their own app. See [Onboarding Devices](/docs/user-guide/onboarding-devices) for the per-device specifics.
- **Tell the operator which device you have.** How it pairs depends on the device.

## How your device pairs

The Hub never shows a code for you to type into your device. The operator pairs it from the Visualizer, in one of these ways:

| Device | How it pairs |
|---|---|
| Meta Quest, METALENSE 2, Android XR headset | **Over USB:** the operator plugs it into the Hub's PC and clicks **Pair headset**; nothing for you to do. |
| | **QR code:** the operator shows a QR code (**Pair Headset** dialog); look at it with the QAROS app running. |
| | **The code your headset shows:** while it is connected to no Hub, the QAROS app shows its address and a 6-digit code. Read both to the operator, who types them into the **Pair Headset** dialog. |
| HoloLens 2 | The operator invites it by its address; nothing for you to type. |
| Any other device | Ask the operator; see [Onboarding Devices](/docs/user-guide/onboarding-devices). |

Two things are worth knowing about the codes:

- They are **single-use**: once your device joins with one, it is spent.
- They are **short-lived**: a QR code lasts 60 seconds and the operator's screen replaces it before it runs out; the code your headset shows changes every 30 seconds. Use the one shown now.

## Join the Shared Space

1. Open the QAROS app on your device.
2. Pair it in the way the operator chose: scan the QR code, read your headset's code and address to the operator, or wait while they pair it over USB or invite it.
3. That's it. Your device pairs with the Hub and enters the Shared Space.

You only do this once per device. After the first join, your device **rejoins automatically** every time it reconnects — no code, no typing — until someone deliberately removes it. More on that in [Working in a Session](/docs/user-guide/working-in-a-session).

## What to expect on your first join

The Shared Space may look empty at first, and that is normal. You will only see application content once an application starts drawing into an **app volume** — a box in the Shared Space that an app renders into. Until a source application is running and streaming, there is simply nothing to show yet.

You may, however, already see **GUI panels** (flat screens floating in the Shared Space) and other **people** who are present. If the Shared Space stays blank, it usually means no application has started streaming — check with the operator rather than assuming something is broken.

For a plain-language tour of rooms, volumes, and panels, read [The Shared Space](/docs/user-guide/the-shared-space).

## Next steps

- [Working in a Session](/docs/user-guide/working-in-a-session) — what to do once you're in.
- [Onboarding Devices](/docs/user-guide/onboarding-devices) — device-by-device setup.
- [Troubleshooting](/docs/user-guide/troubleshooting) — if pairing fails or the Shared Space won't appear.
