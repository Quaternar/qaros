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

The installer registers these firewall rules ([Installation](/docs/operator-guide/installation#what-the-installer-does)).

## Deployment models

**LAN (default).** The Hub and every device sit on the same local network. Discovery multicast reaches the devices, session streams stay local, and latency and bandwidth are whatever your LAN provides. This is the intended and simplest model.

**Cloud relay (off-LAN).** When a device cannot reach the Hub on the same LAN - a remote participant, a different site - a relay is required to carry traffic between the two networks. Plan a relay deployment with Quaternar; it is not something you get for free from the LAN discovery mechanism.

## Discovery is unauthenticated and LAN-scoped

Discovery (the multicast beacon and framed TCP discovery on the onboarding port) is **deliberately unauthenticated** and carries **no secrets** - only coordination metadata such as the Hub's display name, a nonce, its server ID, and where to reach the pairing service. A forged beacon can at worst redirect a *connection attempt*; it cannot onboard a device, because the pairing code is the trust boundary and never travels over any transport.

Because it is unauthenticated, **do not expose discovery to the internet.** Keep multicast `239.77.77.77:7445` and the Hub port range on the LAN. Off-LAN reach is the job of a relay, not of opening discovery to the world.

## Joining Hubs: primary and members

Several PCs share one session by making one Hub the **primary** and joining the others to it as **members**. Joining always makes this Hub a member.

1. On the Hub that should become a member, choose *Join hub…* (Visualizer Connection panel, **Hubs** area, or tray menu).
2. Pick the primary from the nearby list, or enter its address as **`host:port`** (there is no default port). Hubs run by your own user account on this PC are listed first and marked *(yours)*.
3. Either wait for the primary's operator to click **Allow** on the pending request in the primary's *Members* list (up to 10 minutes), or type the primary's 8-digit **join code** (shown on the primary; valid briefly, no click on the primary).
4. The member restarts into the primary's session; the tray shows "Member of *primary*".

Apps that link several PCs themselves (for example a simulation) can bring a Hub in without any click: approve the app on the primary with **may bring in member hubs** and a quota.

### What a member is - read this

- **One CA per session.** The member has no CA; every certificate in the session is signed by the primary. There is no merge of trust between Hubs.
- **The primary decides.** App approvals, revocation, new sessions and member admission happen on the primary. The primary's policy says whether a member may pair headsets and approve local apps on its own PC.
- **Members follow the primary.** When the primary starts a new session, members follow within seconds by restarting. While the primary is unreachable nothing new is admitted and the member says so.
- **Leaving and removing.** *Leave and start own session* on the member starts a fresh, standalone session (it revokes nothing; its devices pair again). *Remove* on the primary is final and revokes the member and everything it enrolled.
- **No chains.** A member cannot accept members, and a Hub with members cannot join another.
- **After this update:** Hubs that were connected with the old *Connect to other Hub* start on their own and show a one-time notice offering to join the former Hub as a member.

For the pairing and enrollment protocol behind all of this, see [Security Model](/docs/operator-guide/security-model).
