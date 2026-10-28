---
title: HoloLens 2
description: Install the QAROS HoloLens Player app onto HoloLens 2 from the Visualizer by code, or through the Device Portal.
---

# HoloLens 2

HoloLens 2 runs the **QAROS Player** app (package `com.quaternar.qaros.player`). You sideload this app onto the device, then the Hub-side `qar-hololens-desktop` service connects to it over the network, streams the mixed view to it, and reads back head pose and hands.

We support only the sideloaded Quaternar player app on HoloLens. The Microsoft Holographic Remoting Player is not part of this setup.

The player ships with QAROS under `devices/hololens/`: `QAROS Player HoloLens <version>.msix`, its `.cer` certificate and a `Dependencies/` folder. Install it one of two ways:

| Way | Needs | Use it when |
|---|---|---|
| [From the Visualizer, by code](#install-from-the-visualizer) | HoloLens 2 on Windows Holographic 20H2 or newer, on the same network as the PC | Always, unless the HoloLens is managed |
| [Through the Device Portal](#install-through-the-device-portal) | Developer Mode and the Device Portal on the HoloLens | The HoloLens is managed (MDM, provisioning package or Entra ID sign-in): App Installer is turned off there |

An earlier player, listed as *QuaternAR Player* or *Skyline Player*, is a separate app: uninstall it (in Start, press and hold its tile and select **Uninstall**), then scan the scene origin code again with the new player.

## Install from the Visualizer

The Visualizer serves the player on your network while it shows an install code. Scanning the code on the HoloLens opens App Installer on the player. No Developer Mode, no Device Portal.

1. In the Visualizer, **Session** › **Devices** › **Set up or update a device**, **Device type** **HoloLens · Install code**, then **Show install code**. Step by step: [HoloLens 2 tutorial](/docs/operator-guide/visualizer-tutorials/hololens-2).
2. If the PC has several networks, pick the one the HoloLens is on in **Serve on**.
3. **Once per HoloLens, trust the certificate.** The player is signed with a QAROS certificate the HoloLens does not trust yet:
   1. In the Visualizer's dialog set **Code opens** to **Certificate**, and scan the code with the HoloLens. It downloads `QAROS-Player-HoloLens.cer`; the code then switches back to **App Installer**.
   2. On the HoloLens open **Settings › Update & Security › Certificates** and select **Install a certificate**.
   3. **Import File**: pick `QAROS-Player-HoloLens.cer` in **Downloads**.
   4. Store Location **Local Machine**, Certificate Store **Trusted People**, then **Install**. Only the HoloLens' device owner can install into Local Machine.
4. Scan the code (**Code opens** **App Installer**). App Installer shows the player: select **Install**. It also installs the framework package the player needs, from the PC.
5. Continue with [First launch](#first-launch).

If your organization disables App Installer links by policy, set **Code opens** to **Install page in Edge**: the page has the same steps, and its **Install QAROS Player** button downloads a file that App Installer opens.

The Visualizer stops serving when you close its dialog, or after 15 minutes (**Serve again** restarts it). It serves on a port between 7448 and 7460, which the QAROS setup allows through Windows Firewall.

If App Installer reports the package as untrusted, the certificate is missing from **Trusted People** (step 4).

## Install through the Device Portal

### 1. Enable Developer Mode on the HoloLens

1. Power on the HoloLens and open **Settings**.
2. Go to **Update & Security -> For Developers**.
3. Toggle **Developer Mode** on and confirm when prompted.

### 2. Connect the HoloLens to your PC

1. Make sure the HoloLens and PC are on the same network.
2. On the PC, open a web browser and enter the HoloLens' IP address to reach the **Device Portal**. The IP address is shown in the HoloLens network settings.
3. If you do not know the username and password, press **Submit** four times. The portal will then prompt you to set a new password.

### 3. Deploy the app to the HoloLens

1. In the Device Portal, go to **Views -> Apps**.
2. Find **Deploy apps** in the top-left corner.
3. Select **Local Storage**, click **Choose File**, and select the app bundle to upload it.
4. Click **Install**.
5. On the HoloLens, open the app menu and launch **QAROS Player**.

## First launch

- Allow access to both the **camera** and **eye tracking** when prompted.
- On the first run, the connection from the PC to the device can take a while. If it takes more than **60 seconds** before you see `Receiving...`, reboot the device and try again.
