---
title: HoloLens 2
description: Sideload the QarOS HoloLens Player app onto HoloLens 2 through the Device Portal.
---

# HoloLens 2

HoloLens 2 runs the **QarOS HoloLens Player** app, shipped as *QuaternAR Player* or *Skyline Player*. You sideload this app onto the device, then the Hub-side `qar-hololens-desktop` service connects to it over the network, streams the mixed view to it, and reads back head pose and hands.

We support only the sideloaded Quaternar player app on HoloLens. The Microsoft Holographic Remoting Player is not part of this setup.

## Install the HoloLens Player

Obtain the HoloLens Player app bundle from Quaternar, then sideload it through the HoloLens Device Portal.

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
5. On the HoloLens, open the app menu and launch **QuaternAR Player** or **Skyline Player**.

### 4. First launch

- Allow access to both the **camera** and **eye tracking** when prompted.
- On the first run, the connection from the PC to the device can take a while. If it takes more than **60 seconds** before you see `Receiving...`, reboot the device and try again.
