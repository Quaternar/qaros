---
sidebar_position: 8
title: Glossary
description: Short definitions of every QAROS term used in this documentation.
---

# Glossary

**App content space** — an application's own coordinate system as rendered inside its app volume, positioned by `app_pose` and scaled by `app_scale`.

**App scale** — room-meters per app-meter; `0.5` shows content at half size, `10` at ten times life size.

**App volume** — a named 3D box in room space where one application renders; the unit of streaming, masking, and mixing.

**CA (Certificate Authority)** — the Hub's private certificate issuer; every onboarded peer holds a certificate signed by its Hub's CA.

**Common name** — the stable string identity of an app volume or GUI panel (e.g. `cad-review.hub1.example.com`); asking for the same name always returns the same object.

**Discovery** — the Hub's unauthenticated LAN beacon (UDP multicast) that lets devices find it without configuration.

**EC-JPAKE** — the password-authenticated key exchange used for pairing; proves both sides know the pairing code without transmitting it.

**Forget** — the operation that wipes a device's persisted identity slot, requiring a fresh pairing to return.

**GUI panel** — a shared 2D interface quad (currently web content) placed directly in room space.

**Hub** — the workstation that runs the QAROS runtime, source applications, mixer, CA, and session; the trust root of a deployment.

**Mixer** — the per-user compositor that warps, masks, depth-sorts, and blends all app-volume streams into one stereo color+depth frame per user.

**Onboarding** — the one-time pairing of a device/application with the Hub, producing a certificate and session membership.

**Onboarding ID** — the opaque identifier of a persisted identity slot; the only thing an application must store to rejoin later.

**Pairing code** — the short-lived (10 s), single-use, 8-character code shown by the Hub and entered on the device.

**Peer** — a session participant (a device or an application) with a certificate-backed identity and presentation metadata.

**Rejoin** — silently re-entering the session using a persisted identity; the normal path on every start after the first.

**Room space** — the shared coordinate frame of the physical room (meters, OpenXR conventions); where volumes and panels are placed.

**Session** — the collaborative space all peers of a Hub share; entered by onboarding, re-entered by rejoin.

**Stream** — the flow of rendered frames (color, alpha, depth + projection metadata) from a source application toward a target.

**Warping** — late-stage reprojection of a received frame to the viewer's freshest head pose, using the frame's depth.

**World anchor** — an optional Earth-fixed (ECEF WGS84) frame attached to an app volume, pinning it to a geographic pose.
