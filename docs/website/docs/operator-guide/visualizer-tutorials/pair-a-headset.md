---
title: Pair a Headset That Joins by Itself
sidebar_position: 4.5
description: Install and pair a METALENSE 2, Quest or Android XR player that joins the session on its own, from the Visualizer over USB or with a QR code, and set the name it shows.
---

# Pair a Headset That Joins by Itself

A player that joins the session by itself is paired **once** with a one-time code. After that it
keeps its own identity and rejoins on its own after an app restart, a reboot or a Hub restart. The
Hub then starts its mixer and video relay for it; you do not invite it from the **Invite device**
card.

## Before you start

- The player is installed and running on the headset
  ([Install and Invite](./quest-and-metalense-2.md)).
- The headset can reach the Hub over the network to join it
  ([Put METALENSE 2 on Wi-Fi](./metalense-2-wi-fi.md)).
- **Session** › **Devices** › **Set up or update a device** › **Device type** = **Metalens · USB**
  (or **Meta Quest · USB**, or **Android XR · USB**: see
  [Android XR headsets](#android-xr-headsets)).

## Pair over USB

**Pair headset** needs the headset connected by USB, with USB debugging allowed (adb). The code
travels over the cable, never over the network.

1. Plug the headset in with a USB-C data cable; it is listed under **Connected over USB**.
2. Open the step **Invite the glasses** (**Invite the headset** for Quest, **Pair the headset** for
   Android XR) and click **Pair headset**.
3. The step shows *Asking the hub for a one-time pairing code*, then *Sending the pairing code to
   the headset*, then *Invite delivered — the headset is joining…*.
4. When the headset appears in the session the step shows *Paired: the headset joined the
   session.*

## Pair with a QR code (no USB needed)

1. Click **Show QR code**. A dialog shows the code and how long it stays valid (two minutes).
2. Put the headset on and look at the code with the player running. The dialog closes once the
   code is used.
3. An expired code is replaced by a placeholder; click **Show QR code** again for a new one.

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
   address. See [Wi-Fi on an Android XR headset](#wi-fi-on-an-android-xr-headset).
5. **Pair the headset**: start QAROS Player, then **Pair headset** over the cable, or **Show QR
   code** for the player to scan ([Pair over USB](#pair-over-usb),
   [Pair with a QR code](#pair-with-a-qr-code-no-usb-needed)).

- **Pair headset** works only with a headset that allowed USB debugging from this PC. To stop this
  PC from reaching it, revoke USB debugging authorisations in **Developer options**.

### Wi-Fi on an Android XR headset

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
| Headset not paired yet | Takes a code from **Pair headset** or a QR code |
| Headset already paired (or its stored identity cannot be read right now) | Refuses **Pair headset** and keeps rejoining with its identity; a **QR code** scanned on the headset still pairs it again |
| Headset joined | Refuses new codes |
| Its stored identity no longer works (data restored from another install, refused by the Hub) | Waits for a new code: pair it again |
| Uninstalled | Its identity is gone with the app: pair it again after reinstalling |

- Every code is single use and short-lived (30 s for **Pair headset**, two minutes for a QR code).
  **Pair again** always makes a new one.
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
| **Pair headset** disabled | No headset on USB; **Show QR code** still works |

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
