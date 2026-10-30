---
title: Managing Devices
sidebar_position: 8
description: The operator's side of onboarding - minting and reading pairing codes, listing connected peers, and forgetting or blacklisting a device.
---

# Managing Devices

This page is the operator's side of onboarding. The device owner's side - the steps to bring a HoloLens, P&C Solutions METALENSE 2, Meta Quest, Android device, or visualizer into a session - is in [Onboarding Devices](/docs/user-guide/onboarding-devices). Read both: onboarding is a two-party act, and each page describes one party.

## How pairing codes are minted

The Hub continuously mints short-lived pairing codes:

- A **fresh code** is minted every 8 seconds by default (`codeRotationSec` in the [Launcher Configuration Reference](/docs/operator-guide/launcher-config-reference)). This is the code the Visualizer shows.
- Each code is **single-use** - it is consumed on success *and* on failure.
- Each code has a **short time-to-live of about 10 seconds**.
- **One failed attempt invalidates the code** - there is no retry against a spent or failed code.

Security comes from this handling - single-use, short TTL, one-failure-max - not from the length of the code. The code never crosses any transport; it is only ever read by a human and typed into a device.

**One code at a time.** The Visualizer shows only the current rotating code; it offers no list of further codes to hand out. Applications cannot request codes either: a source app on the Hub's PC joins without a code, after you allow it once ([Source Applications](/docs/operator-guide/source-applications#approved-local-apps)).

## The two onboarding paths

**Push (operator-configured).** You add a HoloLens in the Visualizer's [Session panel](/docs/user-guide/connection-panel#onboard-a-device) (**Devices** tab, **Invite device** card), and the Hub sends it an onboarding invite, again at every startup when **Auto** is on. This is today's default and the standard path for fixed installations - the device does not need anyone to read a code to it.

**Pair (Android headsets).** Meta Quest, METALENSE 2 and Android XR headsets are never invited by IP address. Pair each one once from the Visualizer's **Devices** tab: **Pair headset** pushes a one-time invite over the USB cable, **Show QR code** shows one the player scans. The headset then joins the session by itself and the Hub starts its mixer and video relay ([Pair a Headset](/docs/operator-guide/visualizer-tutorials/pair-a-headset)).

**Pull (code entry).** A device's app asks its user for the code currently shown by the Hub and onboards itself. Integrated applications on a PC or device without a Hub use this path through the C API; on the Hub's PC they join as local apps instead. Someone reads the Hub's current code from the Visualizer (see [Reading this Hub's code](#reading-this-hubs-code)).

## Listing connected peers

The runtime tracks which peers are connected. In the Visualizer's Session panel, the **Devices** tab (shown first) lists the headsets with their streaming status; **Hubs** lists every Hub of the session, expand one to see the apps and devices it serves.

## Forgetting and blacklisting a device

Two related operations remove a device:

- **Forget** wipes the device's persisted identity slot (certificate and session state) so its next connection requires a fresh pairing. A device cannot be forgotten while its session is still active - have it leave first, then forget.
- **Blacklist (revoke)** stops the Hub from renewing or re-issuing the device's certificate and tells the rest of the session about it. A blacklisted device is refused on every channel, a new pairing code included, until you **Allow** it again (Visualizer, Session > *Devices*: the device's row, or *Revoked peers* for a peer no remembered device names). Revoke and Allow work only from the Hub's PC.

### What a blacklisted device can still do

A revoked certificate is not invalid everywhere at once. What happens depends on whether a peer has heard of the revocation:

| Who | Has heard of the revocation | What happens to the device |
|---|---|---|
| The Hub's certificate authority | Always | Refuses to renew or re-issue its certificate |
| A peer that has received the Hub's revocation list | Yes | Drops the device and refuses it. A launcher stops streaming to it at once and does not start streaming to it again |
| A peer that has not received the list | No | Keeps accepting the device until its certificate expires |

- The Hub publishes the list the moment you blacklist a device, and keeps it stored, so a peer that joins later still receives it. Peers connected to the Hub normally hear of it within about a second.
- A peer that cannot reach the Hub's stored data keeps accepting the device. It never refuses everyone just because the list is missing.
- **Worst case: up to 24 hours.** That is the longest a certificate lives. When it expires, every peer drops the connection and the device cannot renew.
- A peer that refuses the device stops serving it (no pipeline, no video), but the network connection itself is not closed.

## Reading this Hub's code

For push onboarding you do not need the code. For pull onboarding, open the Connection panel's **Hubs** > **Pair another Hub** > **Let another hub join us**: it shows the current code as text with its countdown and this Hub's addresses. Relay it to the device owner within its short lifetime. A code is offered only while that card is open.

Device players that scan a QR code are still in development. In Debug mode, **Devices** > **Invite device** > **By QR code invite** shows the large QR code for them.

- **Several devices at once:** pair them one after another, each with the code shown at that moment; a code is spent by the device that uses it. A HoloLens you can reach by address is easier with push onboarding (**Devices** > **Invite device**), and an Android headset with **Pair headset**; neither needs a code read to it.
