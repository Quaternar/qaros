---
title: Meta Quest and METALENSE 2 - Install and Invite
sidebar_position: 3
description: Install the QAROS player APK on a Meta Quest or P&C Solutions METALENSE 2 over USB from the Visualizer, then invite it with the Headset device kind.
---

# Meta Quest and METALENSE 2: Install and Invite

Meta Quest and P&C Solutions METALENSE 2 run the same kind of player, an Android APK. The
Visualizer installs it over a USB-C cable and invites the device with the **Headset** device kind.
No Android tools or terminal are needed.

| | Meta Quest | METALENSE 2 |
|---|---|---|
| **Device type** in the Visualizer | **Meta Quest · USB** | **Metalens · USB** |
| APK in the QAROS ZIP (`devices/android/`) | `QAROS Player Quest <version>.apk` | `QAROS Player MetaLens <version>.apk` |
| One-time preparation | Developer Mode on the headset (Meta Horizon phone app) | None |
| Wi-Fi | Set on the headset itself | Can be set from the Visualizer ([Put METALENSE 2 on Wi-Fi](./metalense-2-wi-fi.md)) |
| Invite as | **Headset** | **Headset** |

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

## 3. Invite the device with the Headset button

The device must be on the same network as the Hub, with the player running.

A player that joins the session by itself is not invited here: pair it once instead
([Pair a Headset That Joins by Itself](./pair-a-headset.md)).

1. **Session** › **Devices**. The **Invite device** card is open at the top. (From the setup steps,
   **Invite the headset** › **Invite device...** jumps there too.)
2. Keep the device kind on **Headset**. It is the default and covers Meta Quest, METALENSE 2 and
   other Android headsets; the player reports its exact model once it runs.
3. Type the IP address the player shows in **Address**, and optionally a **Label** (`Quest Lab`).
4. Tick **Invite automatically** to have the Hub invite the device again on every start.

   ![Invite device card with Headset selected](../../img/visualizer/invite-headset.png)

5. Click **Invite Headset**. The row goes from *Waiting for player* to *Streaming from &lt;hub&gt;*,
   and the Session header counts *1 device streaming*.

Start, stop and forget work as for every device: **Stop** / **Invite** on the row, **⋯** for
**Invite automatically** and **Forget...** (see
[HoloLens 2, step 3](./hololens-2.md#3-start-and-stop-streaming)).

- An empty **Address** gives *Enter the device's hostname or IP address before inviting it.*
- A wrong address gives a notification *Invite failed: …*.
- A device remembered under an older kind (*Meta Quest*, *Metalens*) is not invited any more:
  **Forget** it and invite it again as **Headset**.
