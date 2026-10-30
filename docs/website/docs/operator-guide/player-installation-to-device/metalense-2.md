---
title: P&C Solutions METALENSE 2
description: Install the QAROS player APK onto a P&C Solutions METALENSE 2 device with ADB.
---

# P&C Solutions METALENSE 2

## Install from the Visualizer

The QAROS Visualizer installs the player over USB from its **Devices** tab, and can put the glasses on Wi-Fi over the same cable.

1. Connect the glasses to the PC with a USB-C data cable and power them on.
2. In the Visualizer, open the **Devices** tab (plug icon, next to **Connection**) and select **Metalens**.
3. If the Visualizer does not find `adb`, click **Download platform tools**, read Google's licence and accept it. The tools are saved for your Windows user under `%LOCALAPPDATA%\QAROS\platform-tools`; no administrator rights are needed. If you already have them, click **Locate adb.exe…** instead.
4. Allow USB debugging on the glasses when they ask. They then show as **Ready** under **Connected over USB**.
5. Under **Install QAROS Player**, check the package (the Visualizer finds the one shipped in `devices/android/`; **Browse…** picks another) and click **Install** or **Update to** *version*.

### Wi-Fi on the glasses

Under **Wi-Fi on the device**, type or pick the network name (names saved on this PC are offered, the network this PC is on first), choose the security type, type the password and click **Join**. The password is typed every time; the Visualizer never stores or logs it, and does not read saved Wi-Fi keys from Windows.

When the glasses report the network, the card shows **On** *network* · *address*.

### Pair the glasses

Start the player, then in the last step, **Pair the glasses**, click **Pair headset** to send it a one-time invite over the USB cable, or **Show QR code** for the player to scan. The glasses then join the session by themselves ([Pair a Headset](/docs/operator-guide/visualizer-tutorials/pair-a-headset)). Android headsets are not invited by IP address.

If the card says **Set Wi-Fi on the glasses**, the glasses do not accept a network over USB (this needs Android 11 or newer); set Wi-Fi on the glasses themselves. Enterprise (802.1X) networks are not supported.

## Install manually with ADB

### Prerequisites

Make sure **ADB** is installed on your PC. ADB is included in the Android SDK Platform Tools.

Download Android Platform Tools here:

https://developer.android.com/tools/releases/platform-tools

After installation, make sure `adb.exe` is available from your terminal or command prompt.

### Device setup

Connect the device to your PC using a USB cable, then power on the device.

### Verify the device connection

Open a terminal or command prompt and run:

```powershell
adb devices
```

You should see the connected device listed.

If the device is shown as `unauthorized`, accept the USB debugging authorization prompt on the device.

If no device is shown, check that:

- the USB cable supports data transfer
- the device is powered on
- the USB debugging prompt was accepted
- ADB is installed correctly

### Install the APK

The APK ships in the QAROS ZIP under `devices/android/` as `QAROS Player MetaLens <version>.apk`. Run the command from that folder, or provide the full path to the APK.

```powershell
adb install "QAROS Player MetaLens <version>.apk"
```

### Reinstall or update the app

If the app is already installed and you want to replace it, use the `-r` flag.

```powershell
adb install -r "QAROS Player MetaLens <version>.apk"
```

The player is **QAROS Player** in the app list, package `com.metalense.com.quaternar.qaros.player` (METALENSE requires its `com.metalense` prefix). An earlier player (*QAR Player Vulkan*, `com.metalense.quaternar.vulkan`) is a separate app: the Visualizer's **Devices** tab removes it after installing, or remove it yourself and scan the scene origin code again.

```powershell
adb uninstall com.metalense.quaternar.vulkan
```
