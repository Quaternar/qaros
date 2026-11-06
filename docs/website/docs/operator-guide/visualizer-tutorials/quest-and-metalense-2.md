---
title: Meta Quest and METALENSE 2 - Install and Pair
sidebar_position: 3
description: Install the QAROS player APK on a Meta Quest or P&C Solutions METALENSE 2 over USB from the Visualizer, then pair it so it joins the session by itself.
---

# Meta Quest and METALENSE 2: Install and Pair

Meta Quest and P&C Solutions METALENSE 2 run the same kind of player, an Android APK. The
Visualizer installs it over a USB-C cable and pairs the device once; the device then joins the
session by itself.
No Android tools or terminal are needed.

| | Meta Quest | METALENSE 2 |
|---|---|---|
| **Device type** in the Visualizer | **Meta Quest · USB** | **Metalens · USB** |
| APK in the QAROS ZIP (`devices/android/`) | `QAROS Player Quest <version>.apk` | `QAROS Player MetaLens <version>.apk` |
| One-time preparation | Developer Mode on the headset (Meta Horizon phone app) | None |
| Wi-Fi, 5 GHz only | Set from the Visualizer over USB: **Wi-Fi on the headset** | Set from the Visualizer over USB: **Wi-Fi on the glasses** ([Put METALENSE 2 on Wi-Fi](./metalense-2-wi-fi.md)) |
| Pairing step | **Pair the headset** | **Pair the glasses** |
| Last setup step | **Anchor: look at the anchoring QR code** | **Anchor: look at the anchoring QR code** |

## 1. Prepare the device

- **Meta Quest:** turn on Developer Mode once. In the Meta Horizon app on your phone: headset icon ›
  your headset › **Headset Settings** › **Developer Mode** on, then restart the headset. It needs a
  Meta developer account; see
  [Meta Quest installation](/docs/operator-guide/player-installation-to-device/meta-quest#enable-developer-mode-on-meta-quest-3).
- **METALENSE 2:** nothing to prepare.

Connect the device to the PC with a **USB-C data cable** (a charging-only cable is not seen) and
power it on.

## 2. Install the APK from the Visualizer

1. **Session** › **Devices**, scroll to **Set up or update a device**.
2. Set **Device type** to **Meta Quest · USB** or **Metalens · USB**.

   ![Device type list](../../img/visualizer/device-type-combo.png)

   The steps for that device are listed and ticked off as they are detected:

   | Meta Quest · USB | Metalens · USB |
   |---|---|
   | ![Meta Quest steps](../../img/visualizer/setup-quest.png) | ![METALENSE 2 steps](../../img/visualizer/setup-metalens.png) |

   Until a device is plugged in, **Connected over USB** says so and **Install** waits with *Plug
   the device in with a USB-C data cable*.

3. **If the card says *adb was not found on this PC*:** click **Download platform tools**, read
   Google's licence and click **Accept and download**. The tools are saved for your Windows user
   under `%LOCALAPPDATA%\QAROS\platform-tools`, no administrator rights needed. If you already have
   them, click **Locate adb.exe...** instead.
4. **Allow USB debugging** on the device when it asks (on a Quest, put the headset on; tick *Always
   allow*). Under **Connected over USB** the device turns ready.
5. Open **Install QAROS Player**. **Package** shows the APK found in `devices/android/` with its
   version; **Browse...** picks another file.
6. Leave **Launch after install** ticked and click **Install** (or **Update to** *version* when an
   older player is installed).
7. Wait for *QAROS Player X installed.* The row shows *Installed* and the player starts on the
   device.

- **Several devices plugged in:** pick one in the **Device** list, or click **Install on all ready
  devices**.
- **Player signed with another key:** the Visualizer asks before it reinstalls, because
  reinstalling removes the player's settings and its saved scene origin.
- **Install failed:** the row shows *Install failed* and a notification carries adb's message.

Prefer a terminal? The manual `adb install` steps are in
[Player Installation to Device](/docs/operator-guide/player-installation-to-device).

## 3. Join a 5 GHz Wi-Fi network

The step after **Install QAROS Player** is **Wi-Fi on the headset** (**Wi-Fi on the glasses** for
METALENSE 2): the device joins a network over the USB cable, so no password is typed on it.

1. Open the step. It lists the networks the device sees (on METALENSE 2, the networks this PC
   knows).
2. Pick a **5 GHz** network, set **Security**, type the password and click **Join**.
3. The step is done when it shows **On** *&lt;network&gt;* · *&lt;address&gt;* in green.

- **Use a 5 GHz network only:** 2.4 GHz is too slow and unstable for streaming. A network the
  device sees only on 2.4 GHz is marked **2.4 GHz**; picking it warns *Only seen on 2.4 GHz: pick a
  5 GHz network for streaming.*, and a device on one warns *This is a 2.4 GHz network: streaming
  will stutter. Join a 5 GHz network.*
- Details and messages: [Wi-Fi on a Meta Quest or Android XR headset](./pair-a-headset.md#wi-fi-on-a-meta-quest-or-android-xr-headset),
  [Put METALENSE 2 on Wi-Fi](./metalense-2-wi-fi.md).

## 4. Pair the device

Android headsets are not invited by IP address. Pair the device once from the pairing step
(**Pair the headset** for Quest, **Pair the glasses** for METALENSE 2):

1. Start QAROS Player on the device.
2. Click **Pair headset** to send it a one-time invite over the USB cable, or **Show QR code** for
   the player to scan. Without cable or camera, type the address and code the device shows into
   **Invite device** › **By code**.
3. The device joins the session by itself and the Hub starts its mixer and video relay. It reports
   its exact model, and rejoins on its own after a restart.

Details, messages and the name the device shows:
[Pair a Headset That Joins by Itself](./pair-a-headset.md).

## 5. Anchor the device

The last step, **Anchor: look at the anchoring QR code**, places the scene. This is the room's
origin marker, not the connection QR code used for pairing.

1. Click **Show anchoring QR code** (or use a printed code).
2. Put the device on. Until it is anchored it shows *Look at the anchoring QR code*. Look at the
   code until the message goes.
3. The step completes by itself and shows the room the device took (`anchored · room N`).

Several devices, rooms and printing the code:
[Scene Origin: Anchor Several Devices Together](./scene-origin-and-rooms.md#4-anchor-each-device).

- A remembered Android headset row has no **Invite**; the device rejoins by itself.
- An address typed with `headset:`, `android:` or `quest:` in **Invite device** is refused: *Android
  headsets are not invited: pair them with Pair headset on the Devices tab (adb push or Show QR
  code).*
