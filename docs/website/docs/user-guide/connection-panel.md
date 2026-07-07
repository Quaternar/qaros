---
sidebar_position: 5
title: Connecting to the Hub
description: Connecting to the QarOS Hub — conenction via QR code, floating code or direct IP address invite
---
# Connection Panel

The Connection panel is the first panel in the left dock of the QAROS Hub Visualizer. It shows the current session, helps onboard devices into the hub, manages saved session invites, and lists peers currently visible in the session.

## Current Session

The top of the panel shows the active session ID, this visualizer's peer ID, and its device type. If no session is active, invite and peer controls stay visible but cannot send requests.

## Onboard a Device

Use this section when a headset or another device needs to join this hub.

1. Press the refresh button to request a fresh onboarding invite from the runtime.
2. Press the QR button to open the onboarding QR modal.
3. Show the QR code to the device that is joining the hub.
4. If the device cannot scan the QR code, use the hostname and code shown below the QR code for manual entry.
5. Adjust QR Brightness in the modal if the device camera has trouble sensing the white QR modules. The default brightness is 50%.

The Hostname and Code fields in the panel remember the most recent invite details so the operator can read them without reopening the modal.

## Session Invites

Use Session Invites for peers that should be launched or reconnected by the runtime.

Each row stores:

- Auto: when enabled, the visualizer automatically sends this invite after restart if the saved peer is not already active in the current session.
- Device: target device type for the invite.
- Hostname: host name used for the session invite.
- IP: enables the optional IP Address field and uses that value instead of Hostname.
- Send: manually sends or resends the invite.
- Status: latest invite result.

Rows are saved automatically. After an invite succeeds, the returned peer ID is stored. On the next visualizer or runtime restart, the panel checks whether that peer ID is already active in the current session. If it is missing and Auto is enabled, the invite is submitted again.

## Active Peers

The Active Peers section lists peers currently visible in the session with status and device icons. Saved invite rows whose peer IDs are currently offline are shown as offline entries with a Reinvite button, so the operator can resend the matching invite without rebuilding the row.
