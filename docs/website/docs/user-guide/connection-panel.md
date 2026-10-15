---
sidebar_position: 3
title: Connecting to the Hub
description: The Visualizer's Connection panel - the Hubs in your session, pairing another Hub, and inviting devices by IP address.
---
# Connection Panel

The Connection panel is the first panel in the left dock of the QAROS Visualizer. Its header shows
this Hub, how many Hubs of the session are online and how many devices are streaming. Below it, two
areas:

| Area | What it is for |
|---|---|
| **Hubs** (default) | The session: every Hub in it, session actions, pairing another Hub |
| **Devices** | Headsets this Hub streams to, and inviting a new one |

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

- **Reset session** deletes the stored data of every Hub in the session and restarts them empty. The
  confirmation lists what is discarded and what is kept. Use it when the session is stuck on stale
  state.
- **Leave session** is not available yet; it will unpair this Hub from the others in a later
  version. Until then, a pairing cannot be undone from the Visualizer (see
  [Networking & Federation](/docs/operator-guide/networking-and-federation#what-federation-does-and-does-not-do---read-this)).

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
- **Invite** / **Stop** starts or stops streaming to it. The trash icon forgets it.
- A device streaming without being remembered (invited by an app) is listed too, without Auto.

### Onboard a device

1. Install and start the QAROS player on the device
   ([Player installation](/docs/operator-guide/player-installation-to-device)).
2. Press **Add device** and pick its kind:

| Kind | Notes |
|---|---|
| HoloLens | Invited by IP address |
| Meta Quest | *Legacy*: invited by IP address until its player pairs by QR code |
| Metalens | *Legacy*: invited by IP address until its player pairs by QR code |

3. Enter the device's **Address** and an optional **Label**.
4. Leave **Invite automatically** checked to remember the device and invite it on every start;
   uncheck it to invite once.
5. Press **Invite**. The row shows *Waiting for player*, then *Streaming*. A failure is shown as a
   notification.

An app or a device that pairs itself with a code (for example a [Unity app](/docs/developer-guide/unity))
uses the code shown by **Let another hub join us** above. In Debug mode, **Other devices** in
**Add device** opens the large QR code for players that scan it.
