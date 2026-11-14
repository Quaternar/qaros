---
title: Managing Devices
sidebar_position: 8
description: The operator's side of onboarding - how pairing codes are minted and handed to a device, listing connected peers, and forgetting or blacklisting a device.
---

# Managing Devices

This page is the operator's side of onboarding. The device owner's side - the steps to bring a HoloLens, P&C Solutions METALENSE 2, Meta Quest, Android device, or visualizer into a session - is in [Onboarding Devices](/docs/user-guide/onboarding-devices). Read both: onboarding is a two-party act, and each page describes one party.

## How pairing codes are minted

The Hub mints a pairing code only when the Visualizer shows one, never on a timer:

- A code is **created when you open** a dialog that shows one (**Show QR code**, or the **Pair Headset** dialog) or click **Pair headset**, and it appears at once. **Let another hub join us** mints a code too, but that one is a join code for another Hub only: a device or an app that types it fails.
- Each code **lives 60 seconds**. While the card or dialog stays open, a new code replaces the shown one a few seconds before it expires; closing it stops the codes. While no code is on screen, the Hub offers none.
- Each code is **single-use** - it is consumed on success *and* on failure.
- **One failed attempt invalidates the code** - there is no retry against a spent or failed code.

Security comes from this handling - single-use, short TTL, one-failure-max - not from the length of the code. A device code reaches the headset only over the USB cable or as a QR code it scans; the code a headset shows goes the other way, typed by you into the Visualizer, and never crosses the network.

**One code at a time.** The Visualizer shows only the current QR code, never a device code as text to read out or type into a device; it offers no list of further codes to hand out. Applications cannot request codes either: a source app on the Hub's PC joins without a code, after you allow it once ([Source Applications](/docs/operator-guide/source-applications#approved-local-apps)).

## The onboarding paths

**Push (operator-configured).** You add a HoloLens in the Visualizer's [Session panel](/docs/user-guide/connection-panel#onboard-a-device) (**Devices** tab, **Invite device** card, **HoloLens (legacy)** tab), and the Hub sends it an onboarding invite, again at every startup when **Auto** is on. This is the standard path for fixed HoloLens installations - the device does not need anyone to read a code to it.

**Pair (Android headsets).** Meta Quest, METALENSE 2 and Android XR headsets are never invited by IP address. Pair each one once from the Visualizer's **Devices** tab: **Pair headset** pushes a one-time invite over the USB cable, **Show QR code** shows one the player scans, and the **Pair Headset** dialog, opened by **Pair headset** on the **All devices** tab of the **Invite device** card, shows a QR code and takes the address and code a headset shows while it is connected to no Hub. The headset then joins the session by itself and the Hub starts its mixer and video relay ([Pair a Headset](/docs/operator-guide/visualizer-tutorials/pair-a-headset)).

**Applications.** An integrated application on the Hub's PC joins as a local app, without a code, after you allow it once. An application on another PC joins through that PC's own Hub, made a member of this one ([Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions#remote-app-through-a-member-hub)). The C API's code onboarding is deprecated, and the Visualizer shows no code for it.

## Listing connected peers

The runtime tracks which peers are connected. In the Visualizer's Session panel, the **Devices** tab (shown first) lists the headsets with their streaming status; **Hubs** lists every Hub of the session, expand one to see the apps and devices it serves.

## How long a device stays paired

| Device | Certificate valid | Rejoins without pairing again after being off for up to |
|---|---|---|
| Android headset (Meta Quest, METALENSE 2, Android XR) paired with **Pair headset**, a QR code or the code it shows | 30 days | 37 days |
| Every other device | 24 hours | 8 days |

- A device renews its certificate during the session, or at its next join, before it expires; an expired one still renews within a 7-day grace. Off for longer, it must be paired again.
- A headset paired before this release gets 30 days at its next pairing.
- A lost or stolen headset keeps a valid certificate for up to 30 days: **Revoke** it in **Session** > **Devices** (see below).
- With your own certificate authority (EST), its policy must allow 30-day certificates and issue the `qar://device-class/headset` URI name a headset's request carries. A shorter certificate is accepted; the headset then renews sooner.

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
- **Worst case: until its certificate expires** - up to 24 hours for most devices, up to **30 days for a paired headset**. When it expires, every peer drops the connection and the device cannot renew. Revoke a lost headset at once; do not wait for its certificate to expire.
- A peer that refuses the device stops serving it (no pipeline, no video), but the network connection itself is not closed.

## Handing a code to a device

The Visualizer shows no device pairing code as text. **Let another hub join us** (**Hubs** > **Pair another Hub**) shows *Our join code*, which only another Hub can use under **Join another hub**; a device or an app that types it fails pairing.

A device gets its one-time code in one of these ways, all on the **Devices** tab ([Pair a Headset](/docs/operator-guide/visualizer-tutorials/pair-a-headset)):

| Way | Where | The code |
|---|---|---|
| Over USB | **Pair headset** in the pairing step of the setup steps | pushed over the USB cable |
| QR code | **Show QR code** in the pairing step, or **Invite device** > **All devices** > **Pair headset**, which opens the **Pair Headset** dialog in every UI mode | the player scans it; the one QR invite, 60 s, replaced while the dialog stays open |
| The code the headset shows | the **Pair Headset** dialog, *Or type the code the headset shows* | the headset shows its address and a 6-digit code while it is connected to no Hub; you type both |
| By address (HoloLens) | **Invite device** > **HoloLens (legacy)** | none: the Hub invites the device |

- **Several devices at once:** pair them one after another, each with the code shown at that moment; a code is spent by the device that uses it. A HoloLens you can reach by address is easier with push onboarding (**Devices** > **Invite device** > **HoloLens (legacy)**), and an Android headset with **Pair headset**; neither needs a code read to it.
