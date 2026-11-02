---
title: Pair a Headset That Joins by Itself
sidebar_position: 4.5
description: Install and pair a METALENSE 2, Quest or Android XR player that joins the session on its own, from the Visualizer over USB, with a QR code or by the code the headset shows, and set the name it shows.
---

# Pair a Headset That Joins by Itself

A player that joins the session by itself is paired **once** with a one-time code. After that it
keeps its own identity and rejoins on its own after an app restart, a reboot or a Hub restart. The
Hub then starts its mixer and video relay for it. Android headsets (Meta Quest, METALENSE 2,
Android XR) are never invited by address: pairing is the only way in, in one of three ways:

| Way | Needs |
|---|---|
| [Pair over USB](#pair-over-usb) | The headset on a USB cable |
| [Pair with a QR code](#pair-with-a-qr-code-no-usb-needed) | The headset's camera sees the Visualizer's screen |
| [Pair by the code the headset shows](#pair-by-the-code-the-headset-shows) | Neither: you read the headset's code and type it into the Visualizer |

## Before you start

- The player is installed and running on the headset
  ([Install and Pair](./quest-and-metalense-2.md)).
- The headset can reach the Hub over a 5 GHz network to join it
  ([Put METALENSE 2 on Wi-Fi](./metalense-2-wi-fi.md),
  [Wi-Fi on a Meta Quest or Android XR headset](#wi-fi-on-a-meta-quest-or-android-xr-headset)).
- **Session** › **Devices** › **Set up or update a device** › **Device type** = **Metalens · USB**
  (or **Meta Quest · USB**, or **Android XR · USB**: see
  [Android XR headsets](#android-xr-headsets)).

## Pair over USB

**Pair headset** needs the headset connected by USB, with USB debugging allowed (adb). The code
travels over the cable, never over the network.

1. Plug the headset in with a USB-C data cable; it is listed under **Connected over USB**.
2. Open the step **Pair the glasses** (**Pair the headset** for Quest and Android XR) and click
   **Pair headset**.
3. The step shows *Asking the hub for a one-time pairing code*, then *Sending the pairing code to
   the headset*, then *Invite delivered — the headset is joining…*.
4. When the headset appears in the session the step shows *Paired: the headset joined the
   session.*

## Pair with a QR code (no USB needed)

1. Click **Show QR code**. A dialog shows the code at once and how long it stays valid (60 s).
2. Put the headset on and look at the code with the player running. The dialog closes once the
   code is used.
3. While the dialog stays open, a new code replaces the shown one a few seconds before it
   expires, so the code on screen always works. Closing the dialog stops the codes.

## Pair by the code the headset shows

Use this when the headset can neither be plugged in nor scan the QR code. The headset and the PC
are on the same network.

1. Start QAROS Player on the headset. While it is connected to no hub (not paired yet, or its hub
   is away), it shows the connection prompt: *Connect: scan the connection QR code in the QAROS
   Visualizer, or enter this address and code there:* with its address, an 8-digit code
   (`1234-5678`) and a countdown.
2. In the Visualizer open **Session** › **Devices** › **Invite device** and pick **By code**. The
   headsets showing a code on this network are listed by name and model with their address
   (*Looking for headsets on this network...* until one answers).
3. Click the headset in the list, or type its **Address**.
4. Type the **Pairing code** the headset shows now and click **Pair**. The card shows *Giving the
   code to the hub*, *Calling the headset at &lt;address&gt;*, then *The headset is pairing*; the
   prompt on the headset hides.
5. When the headset joins, the card shows *Headset paired* and the headset is listed as a device.

- The code changes every 60 s and after every pairing attempt: type the one shown now.
- A code of the wrong shape (a hub join code, a typo) is refused before anything is sent.
- The code never crosses the network: the Visualizer hands it to the Hub and only asks the headset
  to call back.
- A headset paired with another Hub can be paired this way too; it moves to this Hub.
- A headset connected to its Hub shows no code, is not listed and refuses the call (*connected to
  its hub*). Scan a QR code with it to move it to this Hub.
- The list comes from the headsets announcing themselves on the local network. Where the network
  blocks that, type the address.

## Android XR headsets

One **Android XR** device type covers every Android XR headset (for example Samsung Galaxy XR) and
the Android XR emulator: installing apps is the same on all of them. Its steps:

1. **Developer options and USB debugging**: on the headset open **Settings** › **About** and select
   **Build number** seven times, then **Developer options** › **USB debugging** on. Connect the
   headset with a USB-C data cable. The step is done once the headset is listed under **Connected
   over USB**.
2. **Allow USB debugging**: put the headset on and choose **Allow** in the *Allow USB debugging?*
   prompt (tick *Always allow from this computer*). Until then the row says *Allow USB debugging on
   the device*.
3. **Install QAROS Player**: click **Install**; the Visualizer offers the `QAROS Player AndroidXR`
   package that comes with QAROS, or pick one with **Browse...**.
   - Without a PC: download the `.apk` in the headset's browser and allow the browser to install
     unknown apps in **Settings** › **Security & privacy**.
4. **Wi-Fi on the headset**: the networks the headset sees, strongest first. Pick one, type its
   password and click **Join**; the step is done once the headset reports the network and its
   address. Use a 5 GHz network only. See
   [Wi-Fi on a Meta Quest or Android XR headset](#wi-fi-on-a-meta-quest-or-android-xr-headset).
5. **Pair the headset**: start QAROS Player, then **Pair headset** over the cable, or **Show QR
   code** for the player to scan ([Pair over USB](#pair-over-usb),
   [Pair with a QR code](#pair-with-a-qr-code-no-usb-needed)), or type the code it shows
   ([Pair by the code the headset shows](#pair-by-the-code-the-headset-shows)).
6. **Anchor: look at the anchoring QR code**: click **Show anchoring QR code** and look at it with
   the headset until *Anchor your headset: look at the anchoring QR code* goes. The step completes
   by itself and shows the room
   ([Scene Origin](./scene-origin-and-rooms.md#4-anchor-each-device)).

- **Pair headset** works only with a headset that allowed USB debugging from this PC. To stop this
  PC from reaching it, revoke USB debugging authorisations in **Developer options**.

### Wi-Fi on a Meta Quest or Android XR headset

Meta Quest setup has the same **Wi-Fi on the headset** step, between **Install QAROS Player** and
**Pair the headset**.

- **Use a 5 GHz network only:** 2.4 GHz is too slow and unstable for streaming, and the step says
  so. A network the headset sees only on 2.4 GHz starts with an amber **2.4 GHz**; picking it shows
  *Only seen on 2.4 GHz: pick a 5 GHz network for streaming.* (joining is still allowed). On a
  2.4 GHz network the step warns *This is a 2.4 GHz network: streaming will stutter. Join a 5 GHz
  network.*
- The headset remembers a network joined here: it shows among the headset's saved networks and
  rejoins on its own when in range. A network marked **Saved** joins without a password (the
  Visualizer turns Wi-Fi off and on, and the headset picks it).
- No prompt appears on the headset.
- The headset does not leave a network it is on for the one you join here. If it stays on, or
  moves to, another network, the join fails: click **Disconnect** (or leave that network in the
  headset's settings) and join again.
- **Disconnect** forgets the network on the headset; joining it again needs the password.
  **Turn Wi-Fi off** is beside it.
- The password is never stored, logged or shown by the Visualizer. While the headset applies it,
  it is briefly on the command the Visualizer runs on the headset over USB.
- A headset that refuses this shows *Set Wi-Fi on the headset*: join in the headset's own settings,
  then click **Check again**.

## How pairing behaves

| Situation | What happens |
|---|---|
| Headset not paired yet | Shows the connection prompt with its code; takes a code from **Pair headset**, a QR code or **By code** |
| Headset already paired (or its stored identity cannot be read right now) | Refuses **Pair headset** and keeps rejoining with its identity; a **QR code** scanned on the headset still pairs it again |
| Headset joined | Shows no code and refuses new codes from the network; a QR code scanned on the headset still moves it |
| Its stored identity no longer works (data restored from another install, refused by the Hub) | Waits for a new code: pair it again |
| Uninstalled | Its identity is gone with the app: pair it again after reinstalling |

- Every code is single use and lives 60 s, for **Pair headset** and for a QR code alike.
  **Pair again** always makes a new one. A headset that took a pushed code has 90 s to join.
- **Pair headset** sends the code over the USB cable, never over the network. If the Visualizer
  cannot reach the player over USB it reports the failure and sends nothing; use a QR code then.
- The player also listens for codes on the local network, without encryption, for other tools.
  That is why only a one-time code is ever sent, and why a headset that already has an identity
  ignores it.
- **Pair headset** pairs one headset at a time.

## Troubleshooting

| Message | Meaning |
|---|---|
| *Not paired* with *closed the connection without answering* or *did not answer* | The player is not running on the headset |
| *adb could not open a tunnel…* | The headset is not connected by USB, not authorised for USB debugging, or offline; reconnect it, or pair with a QR code |
| *the headset already has a session identity…* | It is paired already; scan a QR code on the headset to pair it to this Hub again |
| *Not paired: the headset did not join (code expired or onboarding failed) — pair again* | The headset took the code but never appeared: check it can reach the Hub over the network, then click **Pair again** |
| **Pair headset** disabled | No headset on USB; **Show QR code** and **By code** still work |
| *this is no code a headset shows; check it for a typo.* (**By code**) | The typed code has the wrong shape; type the 8 digits the headset shows |
| *Not paired: ...* after **Pair** in **By code** | The code was wrong or expired. The headset shows a new code at once: type that one |

## Set the name the headset shows

The headset joins under the first name it finds:

1. the first line of `qar-display-name.txt` in the player's Documents folder;
2. the device's host name, unless it is empty or generated by Android (`localhost`, `android-…`);
3. the device name from Android settings;
4. manufacturer and model (for example *P&C Solutions METALENSE 2*).

To set your own name, write the file over USB and restart the player:

```powershell
"Lab glasses 1" | Out-File -Encoding utf8 -NoNewline name.txt
adb push name.txt /sdcard/Android/data/<player package>/files/Documents/qar-display-name.txt
```

- `<player package>` is the player's application ID (`adb shell pm list packages | findstr -i qar`).
- No storage permission is needed: the folder belongs to the player.
- Only the first line counts; it is trimmed and cut to 64 bytes. Any language and emoji work.
- Delete the file to go back to the automatic name.
