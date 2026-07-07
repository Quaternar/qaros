---
sidebar_position: 5
title: Onboarding Devices
description: How to bring a HoloLens, Android device, VR headset, or desktop visualizer into the Hub's session.
---

# Onboarding Devices

"Onboarding" is the one-time act of introducing a device to the Hub: the device proves it is allowed in (via a pairing code or an operator-issued invite), receives a certificate, and joins the session. After onboarding, the device **rejoins automatically** on every reconnect — no code, no user interaction — until you explicitly remove (*forget*) it.

There are two ways a device gets in:

1. **Push (operator-configured):** you list the device in the Hub's launcher configuration (`targetApps[]`), and the Hub invites it automatically at startup. This is the standard path for fixed installations today.
2. **Pull (code entry):** the device's app asks the user for the pairing code currently shown by the Hub, and onboards itself. This is the path integrated applications use via the C API (`qar_runtime_onboard_with_code`).

## HoloLens

The HoloLens is driven by the Hub-side `qar-hololens-desktop` service using Holographic Remoting:

1. Install and start the **Holographic Remoting Player** on the HoloLens; note the IP address it displays.
2. Add a HoloLens entry to the Hub's `targetApps[]` with that hostname/IP (or use the C API connection-string helper, format `hololens:<hostname>` — the hostname is required).
3. Start the Hub. The launcher invites the HoloLens target; the service onboards with the invite and starts streaming the mixed view to the device and reading back head pose and hands.

## Meta Quest

The Quest runs the QarOS player app (an OpenXR native app):

1. Install the QarOS Quest APK on the device.
2. Add a Meta Quest entry to `targetApps[]` (connection-string format `quest:` — no hostname needed; the device connects out to the Hub).
3. Start the player on the device; it discovers the Hub and joins with the configured invite.

:::note TODO
Document the on-device first-run experience for Quest: how the player selects a Hub when several are discovered, and the code-entry / QR-scan UI. The C API supports code-based onboarding, but the current player builds are provisioned through the launcher-invite path; an on-device code-entry UI and QR room-tag reading are planned.
:::

## Android phones and tablets

Android devices use the QarOS Android player (built on the same OpenXR app + Android streamer components):

1. Install the QarOS Android APK.
2. Add an Android streamer entry to `targetApps[]` with the device hostname (connection-string format `android:<hostname>`).
3. Start the app; it joins the session and displays the mixed stream.

:::note TODO
Same gap as Quest: document the on-device onboarding UI (code entry / QR scan) once it ships; today provisioning is operator-configured via the launcher.
:::

## Desktop visualizer

The desktop visualizer onboards automatically: by default the Hub invites a local visualizer into the session at startup (`sendDefaultInvite`). Additional visualizer instances can be invited with the `visualizer:` connection string. For connecting a visualizer to a *remote* Hub, use the hub-connect screen with the remote Hub's address and pairing code.

## ZED camera

A ZED stereo camera participates as a camera-visualizer peer (connection-string format `zed:`), contributing a camera feed as a source into the room.

:::note TODO
Document the ZED setup end-to-end: camera prerequisites/drivers, which machine runs `qar-zed-source`, and how the camera's volume is placed in the room.
:::

## Removing a device

Removing a device from the deployment is the *forget* operation: it wipes the device's persisted identity slot (certificate and session state) so the next connection requires a fresh pairing. A device cannot be forgotten while its session is still active — leave first, then forget. Operators can additionally blacklist a device at the Hub, which blocks even a valid certificate; a blacklisted device can only return through a fresh, human-approved pairing.
