---
title: Troubleshooting
sidebar_position: 7
description: Common issues when joining or using a QarOS session, and how to fix them from the user's side.
---

# Troubleshooting

Most problems joining a session come down to one of a few things. Work through the section that matches what you're seeing. Anything that needs changes on the Hub belongs to your operator — see [Managing Devices](/docs/operator-guide/managing-devices) for the operator side.

## The pairing code was rejected

Pairing codes are short-lived and single-use, so a rejection is almost always one of these:

- **Mistyped.** The code uses an alphabet with no easily-confused characters, but it's still easy to fumble under time pressure. Re-read it carefully, or scan the QR code instead of typing.
- **Expired.** A code lapses after about ten seconds. If you were a moment too slow, it's already gone — ask for the current one.
- **Not the current code.** The Hub replaces the code frequently. An old code from earlier in the conversation won't work.

The fix is the same in every case: **ask the operator for a fresh code and enter it right away.** There's no limit on how many times you can try.

## Can't reach the Hub

If your device can't find the Hub at all:

- **Wrong network.** Devices find the Hub by listening on the local network. Make sure you're on the same Wi-Fi or LAN as the Hub.
- **Firewall.** Some networks block the local discovery traffic devices use to find the Hub. If you're on a managed or corporate network, this is worth raising with whoever runs it.
- **Off-site with no relay.** If you're not on the Hub's network, you can only reach it through a **relay** that the operator has set up. If none is configured, you won't be able to connect from outside — ask your operator whether off-network access is available.

## Connected but you see nothing

Joining successfully doesn't guarantee there's anything to look at yet:

- **No app volume yet.** The Shared Space can be legitimately empty until an application creates a volume and starts drawing into it.
- **The source app isn't streaming.** If the application that should be filling the Shared Space hasn't started sending its content, you'll see an empty space even though you're properly connected.

An empty Shared Space right after joining is usually normal. If it stays empty, check with the operator that the expected application is running and streaming — the problem is likely on the Hub side, not yours.

## My device dropped

Brief disconnects happen — a Wi-Fi hiccup, stepping out of range, closing and reopening the app. In almost all cases your device **reconnects and rejoins on its own**, as the same participant, with no code needed.

You only need to enter a pairing code again if your device's identity was **forgotten** — for example, the operator removed it, or it's a brand-new device. If a normal reconnect isn't happening, ask the operator whether your device is still onboarded.

## How do I get a new code

You don't generate codes yourself — the Hub does, and it shows a fresh one every few seconds. **Ask the operator for the code currently on screen**, and enter or scan it promptly before it lapses.

## Still stuck

If none of the above helps, the issue is likely something the operator needs to check on the Hub — device status, invites, blacklisting, or network configuration. Point them to [Managing Devices](/docs/operator-guide/managing-devices).
