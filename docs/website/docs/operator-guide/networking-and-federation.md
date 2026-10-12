---
title: Networking & Federation
sidebar_position: 6
description: Ports and firewall rules, LAN vs cloud-relay deployment, discovery scope, and connecting two QAROS Hubs into a federated trust domain.
---

# Networking & Federation

A QAROS Hub and its devices are built to share a LAN. This page covers the ports you must open, the two deployment models, and how to connect two Hubs so devices onboarded to either are accepted by both.

## Ports and firewall

| Traffic | Protocol / port | Notes |
|---|---|---|
| QAROS service range | TCP + UDP **19120–19200** | The general range the Hub's services use. Open it once for the Hub machine. |
| Discovery beacon | UDP multicast **`239.77.77.77:7445`** | The Hub broadcasts a beacon every 5 seconds so devices find it with zero configuration. The beacon names the Hub's onboarding port. |
| Discovery requests, pairing and onboarding | TCP, one port from **19120–19199** (framed) | One Hub listener dispatches discovery requests, EC-JPAKE pairing, and shared-password SecureChannel onboarding by first frame. The Hub takes the first free port of its range (not the router port); `discoveryPort` pins it. There is no fixed onboarding port. |
| Session traffic | TCP **19120-19200** (Zenoh mTLS) | All in-session communication, mutually authenticated with Hub-issued certificates. Hub, launched apps, and devices run as Zenoh peers: they connect to known endpoints and listen for full-mesh peer links in this bounded range. |

The system installer registers these firewall rules automatically. For runtime ZIP installs, create them once or accept the Windows firewall prompt when the Hub first starts.

## Deployment models

**LAN (default).** The Hub and every device sit on the same local network. Discovery multicast reaches the devices, session streams stay local, and latency and bandwidth are whatever your LAN provides. This is the intended and simplest model.

**Cloud relay (off-LAN).** When a device cannot reach the Hub on the same LAN - a remote participant, a different site - a relay is required to carry traffic between the two networks. Plan a relay deployment with Quaternar; it is not something you get for free from the LAN discovery mechanism.

## Discovery is unauthenticated and LAN-scoped

Discovery (the multicast beacon and framed TCP discovery on the onboarding port) is **deliberately unauthenticated** and carries **no secrets** - only coordination metadata such as the Hub's display name, a nonce, its server ID, and where to reach the pairing service. A forged beacon can at worst redirect a *connection attempt*; it cannot onboard a device, because the pairing code is the trust boundary and never travels over any transport.

Because it is unauthenticated, **do not expose discovery to the internet.** Keep multicast `239.77.77.77:7445` and the Hub port range on the LAN. Off-LAN reach is the job of a relay, not of opening discovery to the world.

## Connecting two Hubs (federation)

Two Hubs can be joined so that a device onboarded to one is trusted by the other:

1. On the visualizer, open the **hub-connect** screen (tray menu -> *Connect to other Hub*).
2. Pick the remote Hub from the nearby list, or enter its address as **`host:port`** (the port is shown with its pairing code; there is no default port), and its **current pairing code**. Hubs run by your own user account on this PC are listed first and marked *(yours)*.
3. On a successful connection, the two Hubs **merge their CA trust bundles** through a trust-store service. From then on, devices onboarded to either Hub are accepted by both.

### What federation does and does not do - read this

Federation shares **trust**, not signing authority. Be precise about the boundaries:

- **Signing stays one-CA-per-Hub.** Each Hub only ever issues certificates for its own devices. Connecting Hubs does not create a shared CA; it makes each Hub *trust the other's root*.
- **Merges are additive-only.** Connecting adds the remote Hub's trust to the local store. There is no "disconnect and untrust" that cleanly withdraws it - **removing trust requires an out-of-band rotation** (re-issuing/rotating the affected CA material), not a menu action. Treat a federation as a commitment.

For the pairing and enrollment protocol behind all of this, see [Security Model](/docs/operator-guide/security-model).
