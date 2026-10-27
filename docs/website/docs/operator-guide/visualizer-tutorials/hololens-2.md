---
title: HoloLens 2 - Install, Invite, Start and Stop
sidebar_position: 2
description: Install the QAROS player on a HoloLens 2, invite it from the Visualizer, and start or stop streaming to it.
---

# HoloLens 2: Install, Invite, Start and Stop

HoloLens 2 is a **legacy** device in QAROS: it streams over holographic remoting and is invited by
IP address only. The player is an `.msix` app package, not an APK, and it is installed through the
HoloLens **Device Portal** rather than over USB.

## 1. Install the player

The Visualizer lists the steps, but the installation itself happens in a web browser.

1. **Session** › **Devices**, scroll to **Set up or update a device**.
2. Set **Device type** to **HoloLens · Device Portal**. Three steps are listed:

   ![HoloLens steps in Set up or update a device](../../img/visualizer/setup-hololens.png)

   | Step | What to do |
   |---|---|
   | **Developer mode and Device Portal on** | On the HoloLens: **Settings › Update & Security › For developers**, turn on **Developer Mode**, then **Device discovery** and **Device Portal** |
   | **Install QAROS Player** | Open `https://<HoloLens IP>` in a browser on the PC, **Views › Apps › Deploy apps**, choose the `.msix` from `devices/hololens/` of the QAROS ZIP (with its `Dependencies/` folder) and click **Install**. Full steps: [HoloLens 2 installation](/docs/operator-guide/player-installation-to-device/hololens-2) |
   | **Invite the HoloLens** | Next section |

   The Visualizer cannot check the first two steps: click **Mark as done** under each once you
   have done it.

3. Start the player on the HoloLens and allow **camera** and **eye tracking** when it asks. Note the
   IP address it shows.

A HoloLens with an older player build (package version `1.0.0.0`) must uninstall it first.

## 2. Invite the HoloLens

1. **Session** › **Devices**. The **Invite device** card is open at the top.
2. Pick **HoloLens** (it carries the *LEGACY* tag).
3. Type the HoloLens IP address in **Address** and, optionally, a **Label** such as `Lab HoloLens`.
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
