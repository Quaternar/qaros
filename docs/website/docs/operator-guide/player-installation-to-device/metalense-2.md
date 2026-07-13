---
title: P&C Solutions METALENSE 2
description: Install the QAROS player APK onto a P&C Solutions METALENSE 2 device with ADB.
---

# P&C Solutions METALENSE 2

## Prerequisites

Make sure **ADB** is installed on your PC. ADB is included in the Android SDK Platform Tools.

Download Android Platform Tools here:

https://developer.android.com/tools/releases/platform-tools

After installation, make sure `adb.exe` is available from your terminal or command prompt.

## Device setup

Connect the device to your PC using a USB cable, then power on the device.

## Verify the device connection

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

## Install the APK

Run the command from the folder containing the APK file, or provide the full path to the APK.

```powershell
adb install qar-openxr-app-OpenGLES-spaces-release.apk
```

## Reinstall or update the app

If the app is already installed and you want to replace it, use the `-r` flag.

```powershell
adb install -r qar-openxr-app-OpenGLES-spaces-release.apk
```
