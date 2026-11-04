---
title: HoloLens 2 - Install, Invite, Start and Stop
sidebar_position: 2
description: Install the QAROS player on a HoloLens 2, invite it from the Visualizer, and start or stop streaming to it.
---

# HoloLens 2: Install, Invite, Start and Stop

HoloLens 2 is a **legacy** device in QAROS: it streams over holographic remoting and is invited by
IP address only. The player is an `.msix` app package, not an APK. The Visualizer installs it over
your network: it shows an install code, and scanning it on the HoloLens opens App Installer on the
player. No Developer Mode, no Device Portal, no cable.

## 1. Install the player

1. **Session** › **Devices**, scroll to **Set up or update a device**.
2. Set **Device type** to **HoloLens · Install code**. Four steps are listed: **Show the install
   code**, **Trust the QAROS certificate**, **Install QAROS Player**, **Invite the HoloLens**.
3. Under **Show the install code**, click **Show install code**. The **Install QAROS Player on
   HoloLens** dialog shows a QR code and, beside it:

   | Item | What it is |
   |---|---|
   | **Code opens** | What scanning does: **App Installer** (default), **Certificate (once per HoloLens)**, or **Install page in Edge** |
   | **Page** | The install page, `http://<PC address>:<port>/`, to type into Edge. The copy button copies it |
   | **Serve on** | The PC's network the code points at. Pick the one the HoloLens is on |
   | Countdown | Serving stops after 15 minutes; **Serve again** restarts it |
   | `QAROS Player <version> · …` | What is served. Hover it for the folder it comes from |
   | Progress | *Waiting for a HoloLens to scan the code…*, then what the HoloLens fetched |

   If the HoloLens camera over-exposes the code, lower **Brightness** under it.
4. **Once per HoloLens, trust the certificate.** Set **Code opens** to **Certificate** and scan the
   code with the HoloLens: it downloads `QAROS-Player-HoloLens.cer`, and the code switches back to
   **App Installer**. On the HoloLens **Settings › Update & Security › Certificates › Install a
   certificate**, **Import File** (`QAROS-Player-HoloLens.cer` in **Downloads**), Store Location
   **Local Machine**, Certificate Store **Trusted People**, **Install**. The Visualizer cannot see
   this: click **Mark as done** under **Trust the QAROS certificate**.
5. Scan the code again (**App Installer**). App Installer shows the player: select **Install**. The
   dialog reads *Scanned on 1 device*, and once the player has been downloaded, **Install QAROS
   Player** is marked done.
6. If a policy disables App Installer links on the HoloLens, set **Code opens** to **Install page
   in Edge**, scan, select **Install QAROS Player** on the page and open the downloaded file.
7. Close the dialog: the Visualizer stops serving.
8. Start the player on the HoloLens and allow **camera** and **eye tracking** when it asks. Note the
   IP address it shows.

A managed HoloLens (MDM, provisioning package or Entra ID sign-in) has App Installer turned off:
install through the Device Portal instead, see
[HoloLens 2 installation](/docs/operator-guide/player-installation-to-device/hololens-2#install-through-the-device-portal).

A HoloLens with an older player build (package version `1.0.0.0`) must uninstall it first.

## 2. Invite the HoloLens

1. **Session** › **Devices**. The **Invite device** card is open at the top.
2. Pick **HoloLens** (it carries the *LEGACY* tag).
3. Type the HoloLens IP address in **Address**. The address and the device kind are all the Hub
   needs; the device is listed under its address.
4. Tick **Invite automatically** to have the Hub invite it again on every start. Left unticked, it
   is invited once (the device is still listed and can be invited again by hand).

   ![Invite device card with HoloLens selected](../../img/visualizer/invite-hololens.png)

5. Click **Invite HoloLens**. The device row shows *Waiting for player*, then *Streaming from
   &lt;hub&gt;* in green.

On the first run the connection can take a while. If the player does not show *Receiving...* within
about 60 seconds, reboot the HoloLens and invite it again.

## 3. Start and stop streaming

Each remembered device has a row in **Session** › **Devices**. While it streams, the row shows
*Streaming from &lt;hub&gt;* and a **Stop** button; **⋯** opens its menu:

![Streaming device row with its menu open](../../img/visualizer/device-row.png)

After **Stop** the row turns *Offline* and **Invite** replaces **Stop**:

![Stopped device row with Invite](../../img/visualizer/device-row-stopped.png)

| Action | How |
|---|---|
| Stop streaming | **Stop** on the row. The status line reads *Stopped the target app for &lt;address&gt;.* |
| Start streaming again | **Invite** on the row |
| Invite it on every Hub start, or stop doing so | **⋯** › **Invite automatically** |
| Remove the device | **⋯** › **Forget...**, then **Forget** |
| Go to its setup steps | **⋯** › **Set up or install...** |
| Block the device | The crossed-circle icon revokes it; see [Managing Devices](/docs/operator-guide/managing-devices#forgetting-and-blacklisting-a-device) |

Hover the device name to see its presence, Hub and room.
