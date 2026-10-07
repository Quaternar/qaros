---
sidebar_position: 7
title: Security Model
description: The QAROS trust model — pairing codes, password-authenticated key exchange, per-Hub certificate authority, and mTLS sessions.
---

# Security and Onboarding

QAROS deployments often run on factory floors, in hospitals, and on corporate networks — places where "trust the LAN" is not acceptable. The security model is therefore built on a simple promise: **a short pairing code, typed once, bootstraps a full cryptographic identity — and the code itself never travels over the network.**

## The four layers

![Security layers](/img/diagrams/security-layers.png)

### Layer 0 — Discovery (open by design)

Devices find the Hub with zero configuration: the Hub broadcasts a beacon on the local network (UDP multicast `239.77.77.77:7445`) and answers discovery requests on TCP `7445`. This layer is deliberately unauthenticated and carries **no secrets** — only coordination metadata (device name, invitation nonce, server ID, where to reach the pairing service). A forged beacon can at worst redirect a *connection attempt*; pairing still fails without the correct code.

### Layer 1 — Pairing (EC-JPAKE)

The Hub issues short-lived pairing **codes** (8 digits). A user transfers the code out-of-band — reads it off the Hub's screen, scans a QR, or pastes it. Device and Hub then run **EC-JPAKE** (a password-authenticated key exchange on P-256): each side proves it knows the code *without ever transmitting it*. What actually protects the deployment is not code length but code handling:

- every code is **single-use** — consumed on success *and* on failure,
- codes expire after **10 seconds** (the Hub mints a fresh one every few seconds),
- one failed attempt invalidates the invitation,
- the Hub's server ID is bound into the exchange, so a code for one Hub can never be replayed against another.

A successful exchange yields a strong shared key.

### Layer 2 — Secure channel and enrollment (AES-256-GCM)

The shared key encrypts a private channel (AES-256-GCM, separate keys per direction, fresh nonces). Over this channel — and only now — the real credentialing happens:

1. The Hub sends an **enrollment offer**: the device's assigned name and the Hub's CA trust bundle.
2. The device generates a **key pair locally** and sends a certificate signing request (CSR). *The private key never leaves the device.*
3. The Hub's **CA gateway** validates the request and has the Hub's private CA sign a certificate. Crucially, the certificate's identity content (names, peer and session bindings) is derived from the authenticated channel — not from whatever the CSR claims.

### Layer 3 — Session traffic (mutual TLS)

All session communication runs over the Zenoh messaging bus on TCP `19120-19200` with **mutual TLS**: both sides present their Hub-issued certificates. Hub, launched apps, and devices run as Zenoh peers, connecting to known endpoints and listening for full-mesh peer links in that bounded range. From this point on, pairing codes play no role - identity is the certificate, and rejoining after a restart is silent.

## The per-Hub Certificate Authority

Each Hub operates its own private CA (based on `step-ca`). This means:

- **No external dependency.** A deployment is self-contained; no internet, cloud, or corporate PKI is required.
- **The Hub is the trust root.** A certificate from Hub A means "Hub A vouched for this device". The Hub machine itself is assumed trusted — protect it as you would a domain controller.
- **Renewal** is online and CSR-based: a device with a valid certificate can refresh it without re-pairing. A certificate that expires while the device is offline requires onboarding again.
- **Revocation** is two-layered: the CA can revoke, and the gateway keeps an encrypted blacklist. A blacklisted device can only return through a fresh, human-approved pairing. Peers that have received the Hub's revocation list refuse the device; peers that have not keep accepting it until its certificate expires (at most 24 h) - see [Managing Devices](/docs/operator-guide/managing-devices).

### Hub-to-Hub federation

When two Hubs connect, they exchange and merge **CA trust bundles**. Devices onboarded to Hub A are then accepted by Hub B and vice versa — federated *trust*. Signing stays local: each Hub only ever issues certificates for its own devices.

## Local source apps (no code)

A source app on the QAROS PC joins without a code. Windows, not the app, says who it is:

- **Local only.** The app talks to the Hub over a named pipe that refuses remote clients and other Windows users.
- **Identity from Windows.** The Hub reads the app's executable path, file hash, Authenticode signature and user from the operating system; what the app says about itself is shown but never trusted.
- **Asked once.** An unknown app waits until someone clicks **Allow** in QAROS. The approval is remembered as a trust record: publisher and location for a signed app, path and file hash for an unsigned one. A moved, copied or re-signed app is refused.
- **The app checks the Hub.** Before it talks, the runtime verifies that the pipe is served by the installed, Quaternar-signed QAROS (otherwise `QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC`).
- **Short-lived, memory-only credentials.** The app gets a 30-minute certificate for a key that never leaves its memory; the runtime renews it. Nothing is written to disk on the app side.
- **Revocable.** Revoking an app in the Visualizer stops its renewal and blacklists its certificates.

## Known gaps in this release

What this release does **not** protect against. The full, maintained threat model is [Quaternar issue #220](https://github.com/Quaternar/Quaternar/issues/220).

- **Malware running as the same Windows user.** Hub keys, Hub state and the approved-apps list are plain files in the user's profile. Such malware can read them, approve itself, or act as the Hub's user.
- **Code inside an approved app.** Plugins, scripts or injected code in an approved process act as that app. Approving an editor or engine admits everything it loads.
- **Session members are trusted.** Any admitted peer, including an approved local app, can see the session's streams. Operator-only actions are not yet bound to the Visualizer's authenticated identity, so a malicious admitted peer may perform them.
- **Revocation reaches peers gradually.** A peer that has not read the Hub's revocation list accepts a revoked device until its certificate expires (local apps 30 minutes, devices up to 24 hours).
- **Offline signature checks.** Revocation of a code-signing certificate is checked against the local cache only.
- **Firewall on every network profile.** The installer opens the QAROS ports on public networks too. Restrict the rules yourself if the PC joins untrusted networks.

Treat the QAROS PC and the signed-in user's account as the trust boundary.

## What this means for each audience

**For users:** you type a short code once per device. Everything after that is automatic — reconnecting later needs no code.

**For operators:** the security perimeter is (1) the Hub machine and its signed-in user, (2) whoever can see the pairing screen for the 10 seconds a code lives, and (3) whoever answers the approval prompt for local apps. Network eavesdroppers learn nothing useful from discovery, and cannot join without a code.

**For application developers:** the entire stack above is hidden behind the onboarding calls. A source app on the QAROS PC calls *onboard* as a local app and persists nothing; an app on another PC persists one opaque onboarding ID for *rejoin*. Neither touches keys or certificates. See [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions).

## Going deeper

For the protocol details, certificate contents, storage layout, assumptions and the current known limitations, see the auditor-level [Security Deep Dive](./security-deep-dive.md).
