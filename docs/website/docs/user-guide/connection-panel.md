---
sidebar_position: 3
title: Connecting to the Hub
description: The Visualizer's Connection panel - the Hubs in your session, pairing another Hub, inviting a HoloLens by IP address and pairing Android headsets.
---
# Connection Panel

The **Session** button is the first on the QAROS Visualizer's sidebar. Its header shows this Hub,
how many Hubs of the session are online and how many devices are streaming. It has three tabs:

| Tab | What it is for |
|---|---|
| **Devices** (default) | Headsets this Hub streams to, and inviting a new one |
| **Rooms** | Which devices are in which room |
| **Hubs** | The session: every Hub in it, session actions, pairing another Hub |

Apps are not managed here: start, approve and revoke them in the
[Source Applications](/docs/operator-guide/source-applications) panel.

## Hubs

This Hub and every Hub paired with it share one **session**.

- One row per Hub, this Hub first (tagged *this hub*): online or offline, its address and how many
  apps and devices it serves, or when it was last seen.
- A new Hub is named after the PC it runs on (its host name). **Edit** on this Hub's row changes its name and room. With Hubs in more than one room, the list
  is grouped by room.
- The arrow on a row lists what runs on that Hub: **Apps** and **Devices**. Turn on **Show internal
  services** to also see the Hub's own services (mixers, renderers).

### Session actions

- **New session** retires the session and starts the next one: app volumes, GUI panels and rooms are
  deleted, while the Hub's name, remembered devices and approved apps are kept and paired devices
  move to the new session on their own. The confirmation's **(?)** lists what is discarded and what
  is kept. Use it when the session is stuck on stale state; step by step in
  [Reset the Session or the Whole Install](/docs/operator-guide/visualizer-tutorials/reset-session-and-install).
- **Leave session** is for a member Hub: it leaves the primary Hub's session.

### Pair another Hub

Opens only when you ask for it. Pairing takes both Hubs:

1. On the Hub that shares: **Let another hub join us** shows a pairing code (digits, it changes
   every few seconds), a countdown and the addresses this Hub is reachable at, each with a copy
   button. **Stop sharing** closes it; no code is offered while it is closed.
2. On the joining Hub: **Join another hub**. Turn on **Discover** to list Hubs on this LAN and press
   **Use** on one, or type its **Address**. Type the **Code** from step 1 and press **Pair with hub**.

## Devices

**Remembered devices** lists every headset this Hub knows, with its status (*Streaming from
&lt;hub&gt;*, *Waiting for player*, *Offline*) and address.

- **Auto**: this Hub invites the device again whenever it starts.
- **Invite** / **Stop** starts or stops streaming to it. An Android headset row has no **Invite**:
  the headset rejoins by itself. The trash icon forgets it.
- A device streaming without being remembered (invited by an app) is listed too, without Auto.
- **Revoked peers (N)**, folded under the list, holds the revoked peers no remembered device names:
  each with its name, device kind, the reason, the PC it ran on and when it was last seen, and an
  unlock button that allows it again. A peer the session no longer knows is shown by its id.

### Onboard a device

1. Install and start the QAROS player on the device
   ([Player installation](/docs/operator-guide/player-installation-to-device)).
2. **Android headsets** (Meta Quest, METALENSE 2, Android XR) are not invited: pair them once with
   **Pair headset** (a one-time invite over the USB cable) or **Show QR code** in the device's last
   setup step on the Devices tab
   ([Pair a Headset](/docs/operator-guide/visualizer-tutorials/pair-a-headset)). The headset then
   joins by itself and reports its exact model. Stop here for an Android headset.
   For a **HoloLens** (*legacy*: streams over holographic remoting), the **Invite device** card is
   open on the Devices tab; **HoloLens** is its only kind.

An address typed with `headset:`, `android:` or `quest:` is refused: *Android headsets are not
invited: pair them with Pair headset on the Devices tab (adb push or Show QR code).*

3. Enter the device's **Address** and an optional **Label**.
4. Check **Invite automatically** to have the Hub invite the device on every start; leave it
   unchecked to invite once.
5. Press **Invite**. The row shows *Waiting for player*, then *Streaming*. A failure is shown as a
   notification. The card stays open for the next device; **Clear** empties it, and **Invite
   device** in the title folds it away.

An app or a device that pairs itself with a code (for example a [Unity app](/docs/developer-guide/unity))
uses the code shown by **Let another hub join us** above. In Debug mode, **By QR code invite** in
the **Invite device** card opens the large QR code for players that scan it.
