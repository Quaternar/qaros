---
title: Meta Quest
description: Install the QAROS player APK onto a Meta Quest device with ADB.
---

# Meta Quest

## Install from the Visualizer

The QAROS Visualizer installs the player over USB from its **Devices** tab, so you do not need a terminal.

1. Enable Developer Mode on the headset once (see [Enable Developer Mode](#enable-developer-mode-on-meta-quest-3) below).
2. Connect the headset to the PC with a USB-C data cable and power it on.
3. In the Visualizer, open the **Devices** tab (plug icon, next to **Connection**) and select **Meta Quest**.
4. If the Visualizer does not find `adb`, click **Download platform tools**, read Google's licence and accept it. The tools are saved for your Windows user under `%LOCALAPPDATA%\QAROS\platform-tools`; no administrator rights are needed. If you already have them, click **Locate adb.exe…** instead.
5. Put on the headset and allow USB debugging when it asks. The headset then shows as **Ready** under **Connected over USB**.
6. Under **Install QAROS Player**, check the package (the Visualizer finds the one shipped in `devices/android/`; **Browse…** picks another) and click **Install** or **Update to** *version*. Leave **Launch after install** ticked to start the player right away.
7. Once the player runs, invite the headset in the **Connection** tab.

If the headset holds a player signed with another key, the Visualizer asks before reinstalling: reinstalling removes the player's settings and its saved scene origin. Install failures appear as a notification with `adb`'s message.

With several headsets plugged in, pick the target in the **Device** list, or click **Install on all ready devices**.

## Install manually with ADB

### Prerequisites

Make sure **ADB** is installed on your PC. ADB is included in the Android SDK Platform Tools.

Download Android Platform Tools here:

https://developer.android.com/tools/releases/platform-tools

After installation, make sure `adb.exe` is available from your terminal or command prompt.

### Enable Developer Mode on Meta Quest 3

Developer Mode must be enabled before installing APK files on a Meta Quest 3.

Official Meta instructions:

https://developers.meta.com/horizon/documentation/native/android/mobile-device-setup/

To enable Developer Mode:

1. Create or log in to a Meta developer account.
2. Create a developer organization in the Meta Developer Dashboard.
3. Verify your developer account if prompted.
4. Open the Meta Horizon mobile app on your phone.
5. Make sure your Meta Quest 3 is paired with the app.
6. Tap the headset icon in the app toolbar.
7. Select your paired headset.
8. Open **Headset Settings**.
9. Open **Developer Mode**.
10. Turn Developer Mode on.
11. Restart the headset.

After connecting the headset to the PC, you may need to put on the headset and accept the USB debugging prompt.

### Device setup

Connect the device to your PC using a USB cable, then power on the device. Make sure Developer Mode is enabled before continuing.

### Verify the device connection

Open a terminal or command prompt and run:

```powershell
adb devices
```

You should see the connected device listed.

If the device is shown as `unauthorized`, put on the headset and accept the USB debugging authorization prompt.

If no device is shown, check that:

- the USB cable supports data transfer
- the headset is powered on
- Developer Mode is enabled
- the USB debugging prompt was accepted
- ADB is installed correctly

### Install the APK

The APK ships in the QAROS ZIP under `devices/android/` as `QAROS Player Quest <version>.apk`. Run the command from that folder, or provide the full path to the APK.

```powershell
adb install "QAROS Player Quest <version>.apk"
```

### Reinstall or update the app

If the app is already installed and you want to replace it, use the `-r` flag.

```powershell
adb install -r "QAROS Player Quest <version>.apk"
```

The player is **QAROS Player** in the app library, package `com.quaternar.qaros.player`. An earlier player (*QAR Player Vulkan*, `com.metalense.quaternar.vulkan`) is a separate app: the Visualizer's **Devices** tab removes it after installing, or remove it yourself and scan the scene origin code again.

```powershell
adb uninstall com.metalense.quaternar.vulkan
```
