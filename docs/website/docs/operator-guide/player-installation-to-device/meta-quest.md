---
title: Meta Quest
description: Install the QAROS player APK onto a Meta Quest device with ADB.
---

# Meta Quest

## Prerequisites

Make sure **ADB** is installed on your PC. ADB is included in the Android SDK Platform Tools.

Download Android Platform Tools here:

https://developer.android.com/tools/releases/platform-tools

After installation, make sure `adb.exe` is available from your terminal or command prompt.

## Enable Developer Mode on Meta Quest 3

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

## Device setup

Connect the device to your PC using a USB cable, then power on the device. Make sure Developer Mode is enabled before continuing.

## Verify the device connection

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

## Install the APK

Run the command from the folder containing the APK file, or provide the full path to the APK.

```powershell
adb install qar-openxr-app-OpenGLES-quest-release.apk
```

## Reinstall or update the app

If the app is already installed and you want to replace it, use the `-r` flag.

```powershell
adb install -r qar-openxr-app-OpenGLES-quest-release.apk
```
