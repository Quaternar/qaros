---
sidebar_position: 7
title: Security Model
description: The QarOS trust model — pairing codes, password-authenticated key exchange, per-Hub certificate authority, and mTLS sessions.
---

# Security and Onboarding

QarOS deployments often run on factory floors, in hospitals, and on corporate networks — places where "trust the LAN" is not acceptable. The security model is therefore built on a simple promise: **a short pairing code, typed once, bootstraps a full cryptographic identity — and the code itself never travels over the network.**

## The four layers

![Security layers](/img/diagrams/security-layers.png)

### Layer 0 — Discovery (open by design)

Devices find the Hub with zero configuration: the Hub broadcasts a beacon on the local network (UDP multicast `239.77.77.77:7445`) and answers discovery requests on TCP `7445`. This layer is deliberately unauthenticated and carries **no secrets** — only coordination metadata (device name, invitation nonce, server ID, where to reach the pairing service). A forged beacon can at worst redirect a *connection attempt*; pairing still fails without the correct code.

### Layer 1 — Pairing (EC-JPAKE)

The Hub issues short-lived pairing **codes** (8 characters from a no-look-alike alphabet). A user transfers the code out-of-band — reads it off the Hub's screen, scans a QR, or pastes it. Device and Hub then run **EC-JPAKE** (a password-authenticated key exchange on P-256): each side proves it knows the code *without ever transmitting it*. What actually protects the deployment is not code length but code handling:

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

All session communication runs over the messaging bus (port `7447`) with **mutual TLS**: both sides present their Hub-issued certificates. From this point on, pairing codes play no role — identity is the certificate, and rejoining after a restart is silent.

## The per-Hub Certificate Authority

Each Hub operates its own private CA (based on `step-ca`). This means:

- **No external dependency.** A deployment is self-contained; no internet, cloud, or corporate PKI is required.
- **The Hub is the trust root.** A certificate from Hub A means "Hub A vouched for this device". The Hub machine itself is assumed trusted — protect it as you would a domain controller.
- **Renewal** is online and CSR-based: a device with a valid certificate can refresh it without re-pairing. A certificate that expires while the device is offline requires onboarding again.
- **Revocation** is two-layered: the CA can revoke, and the gateway keeps an encrypted blacklist. A blacklisted device can only return through a fresh, human-approved pairing.

### Hub-to-Hub federation

When two Hubs connect, they exchange and merge **CA trust bundles**. Devices onboarded to Hub A are then accepted by Hub B and vice versa — federated *trust*. Signing stays local: each Hub only ever issues certificates for its own devices.

## What this means for each audience

**For users:** you type a short code once per device. Everything after that is automatic — reconnecting later needs no code.

**For operators:** the security perimeter is (1) the Hub machine, and (2) whoever can see the pairing screen for the 10 seconds a code lives. Network eavesdroppers learn nothing useful from discovery, and cannot join without a code.

**For application developers:** the entire stack above is hidden behind three C API calls — *onboard*, *rejoin*, and *forget*. Your app persists one opaque onboarding ID and never touches keys or certificates. See [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions).