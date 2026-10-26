---
sidebar_position: 7.5
title: Security Deep Dive
description: Auditor-level reference for QAROS onboarding and certificates - layer protocols, EC-JPAKE transcript binding, the secure-channel nonce scheme, CA gateway policy, certificate SANs, storage, renewal, revocation, member Hubs, the audit log, and known limitations.
---

# Security Deep Dive

This page is for security reviewers and auditors. It states what each layer does, what it assumes, and where today's implementation falls short of the design. For the operator-level summary read [Security Model](./security-model.md) first.

:::caution Read the limitations
The cryptographic core (EC-JPAKE, the AES-256-GCM secure channel, CSR-based enrolment) is sound. Secrets at rest are encrypted under a key of the signed-in user (TPM-backed where the PC has one), which stops other users and disk copies but not malware running as that user; this and the other gaps are listed in [Limitations](#limitations). Do not assume any guarantee this page does not state.
:::

## Layers at a glance

| Layer | Transport | Purpose | Authentication | Confidential |
|---|---|---|---|---|
| 0a Beacon | UDP multicast `239.77.77.77:7445`, every 5 s for a Hub | Announce a Hub on the LAN | None | No |
| 0b Discovery | Framed TCP, the Hub's onboarding port (claimed from 19120-19199) | Return the invitation metadata (no code) | Hub signature for a device that knows the Hub, none on first contact | No |
| 1 Pairing | Framed TCP, same onboarding port (same listener) | EC-JPAKE - prove knowledge of the code | Pairing code | Handshake only |
| 2 Enrolment | Secure channel over the pairing connection | Issue the device certificate | Key from layer 1 (or a provisioned key) | Yes (AES-256-GCM) |
| 3 Session | Zenoh over TLS, TCP `19120-19200` | All session traffic | Mutual TLS, Hub-CA certificates | Yes |

The Hub's TCP listener reads the first frame of each connection and dispatches it to discovery, pairing or provisioned-key onboarding.

## Layer 0 - Discovery

- **Beacon** carries device id, display name, device type and the discovery endpoint.
- **Discovery reply** carries invitation id, Hub server id, pairing host/port and invitation expiry. Never the code.
- Both are plaintext. The beacon is never authenticated. The Hub signs each discovery reply over a fresh nonce the device sent, with the Hub key the device pinned when it enrolled: a device that already knows the Hub refuses a reply that key did not sign (`QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC`).
- First contact is unchecked by design. Discovery is *untrusted input*; the code is the trust boundary.
- Discovery input is strictly validated: printable codes and invitation ids, unknown or oversized fields refused, expired invitations refused before any handshake, bracketed IPv6.
- The listener caps handshakes in flight, overall and per address, and backs off an address after repeated failed authentications.

| Attacker | Outcome |
|---|---|
| Forged beacon / forged discovery reply | First contact: device is pointed at a rogue host; pairing fails without the code. Known Hub: the forged reply is refused |
| Passive LAN eavesdropper | Learns metadata only |
| On-path observer of the invitation id | Can burn the single-use invitation (denial of service), cannot pair |
| Off-LAN attacker | Multicast does not cross routers; TCP ports are not meant to be internet-exposed |

## Layer 1 - EC-JPAKE pairing

| Property | Value |
|---|---|
| Primitive | EC-JPAKE (RFC 8236) on P-256 / SHA-256, Mbed TLS |
| Code | 8 digits, uniform (rejection sampling), CSPRNG-generated |
| Code lifetime | 10 s; a fresh invitation is minted every few seconds |
| Use | Single-use - consumed on success **and** on failure |
| Failures allowed | 1 per invitation |
| Handshake read deadline | 30 s per frame |

Wire sequence (length-prefixed frames on one TCP connection):

1. Client hello (invitation id) → server public invitation (id, server id, host, port, expiry - no code).
2. EC-JPAKE round 1 and round 2, each direction.
3. Server confirmation, then client confirmation: `HMAC-SHA256(confirmKey, "server-confirm" | "client-confirm" ‖ transcriptHash)`.

**Transcript binding.**

- The transcript is the SHA-256 over every type-tagged, length-prefixed frame: client hello, public invitation (including the **server id**), and both rounds in both directions.
- Keys: `HKDF-SHA256(salt = transcriptHash, ikm = EC-JPAKE secret, info = "qar-pake-v1 strong key")` → 32-byte strong key + 32-byte confirmation key. Intermediate key material is zeroized.
- The client rejects a public invitation whose id or server id differs from the one it was given, so a code for one Hub cannot be used against another.
- A wrong code fails key confirmation; the invitation is then discarded.

**Code entropy.** 8 digits is about 2^26.6. That is acceptable only because EC-JPAKE gives no offline-guessing foothold: each guess costs a live handshake against one invitation that allows one failure and lives 10 s. The TTL and the one-failure rule are load-bearing.

## Layer 2 - Secure channel

The Hub upgrades the pairing connection in place (one port). The same channel type also serves provisioned-key onboarding.

| Aspect | Scheme |
|---|---|
| Cipher | AES-256-GCM, separate key per direction |
| Key derivation (on the pairing connection) | `HKDF-SHA256(salt = pairing transcriptHash, ikm = strong key, info = "qar-sec-v1 aes-256-gcm keys")` → client-write key, server-write key, confirmation key |
| Key derivation (provisioned key, new connection) | Both sides first exchange fresh 32-byte random nonces; the nonces join the transcript; same HKDF; mutual HMAC confirmation (`sc-server-confirm` / `sc-client-confirm`) |
| Per-frame nonce (96-bit) | 1-byte direction tag (`S` / `C`) ‖ zero padding ‖ 64-bit big-endian frame counter |
| Nonce uniqueness | Counter is strictly monotonic per direction; the direction tag and per-direction keys separate the two streams; fresh keys per connection |
| Frame limit | 16 KiB, checked before allocation |

### Provisioned-key onboarding (no code)

Processes the Hub launches itself are onboarded without a human code:

- The Hub mints a 256-bit random key, registers it single-use with a 5-minute lifetime, and passes `{port, key id, key}` in the launched process's start-up configuration.
- The process opens the secure channel with that key directly - no PAKE, because the key is already high-entropy.
- From here it is identical to the pairing path.

### Many one-time codes

The Hub holds many pairing codes at once, each its own single-use PAKE invitation, burned by one failed attempt:

| Code | Lifetime | Shown / announced |
|---|---|---|
| Onboarding screen code | ~10 s, rotates | Yes, the only one |
| Codes minted for peers configured as the Hub's operators (1-64 per request, optionally bound to a planned peer id); the Visualizer offers no screen for them and applications cannot request them | 2 min default, 10 min max | No |
| Launch code of a QAROS API app, one per start, only where local enrollment is not available (not Windows) | 2 min | No, written to the app's stdin only |

- A request never invalidates codes others hold or the screen code.
- A code that is not announced carries its invitation id and expiry with it (`qar-code:<id>:<expiry>:<code>`). Neither is secret; both cross the network in the handshake anyway.
- A code bound to a planned peer id enrols that peer id and no other.
- Codes are never command-line arguments and never logged.

## Layer 2 - Certificate enrolment

```text
Hub    -> device : EnrollmentOffer { assigned common name, CA bundle, session id, requested lifetime }
device           : generate P-256 key pair locally; build CSR (CN, qar://peer/<id>, qar://session/<id>)
device -> Hub    : CsrRequest { request id, nonce, claimed peer id, requested CN/SANs, session id, CSR }
Hub gateway      : replay + channel-binding checks, policy checks, sign via the local CA
Hub    -> device : CsrReply { certificate chain, CA bundle }
Hub    -> device : session invite { Zenoh endpoints, CA bundle }
```

- The private key **never leaves the device**. Only the CSR travels.
- The Hub **assigns** the common name and the peer id. Every device of a session gets the session's device name `<session-id>.session.qaros.local`; the primary Hub gets `hub.<session-id>.session.qaros.local`, a member Hub `member.<session-id>.session.qaros.local`.
- A pairing credential is bound to one peer id: planned when the code is minted, or assigned by the Hub during the handshake. A request for any other peer id, or for the Hub name on a device credential, is refused.

### CA gateway policy

The CA runs inside the primary Hub's process and has no network endpoint. Every signing request passes the gateway first:

| Check | Rejects with |
|---|---|
| Request id or nonce seen before (replay cache) | `replay_rejected` |
| Channel binding does not match the channel | rejected before authorization |
| Channel not authenticated | `unauthenticated` |
| Claimed peer id differs from the channel's peer | `peer_mismatch` |
| Claimed peer id is not the one the Hub assigned to this credential | `peer_not_assigned` |
| Hub name requested on a device credential | `role_not_assigned` |
| Requested session differs from the channel's session | `session_mismatch` |
| CSR empty / larger than 32 KiB | `missing_csr` / `csr_too_large` |
| Device revoked (its assigned peer id, or the key it enrolled with), on every channel including a new pairing | `device_revoked_operator_allow_required` |
| Too many signing requests from this peer, or too many at once | `ca_gateway_overloaded` |
| Peer not in the gateway inventory | `unknown_peer` |
| Profile, DNS SAN or IP SAN not assigned | `profile_not_allowed` / `dns_san_not_allowed` / `ip_san_not_allowed` |
| Common name differs from the assigned one | `common_name_not_allowed` |

Every decision is written to the gateway audit log.

### Signing

- Backend: the in-process CA (root and intermediate, P-256, under `caDir/ca/`), or an administrator's EST server that the primary reaches as a registration authority. A member Hub has no CA; it relays to its primary.
- The CA takes the certificate's names from the gateway's decision, never from the CSR; the CSR contributes only the public key.
- Bootstrap is crash-safe (staged, then renamed into place); a damaged CA is reported, never silently replaced.
- Certificate lifetime: the Hub requests the configured lifetime (`certLifetimeHours`). The gateway caps it at **24 h** for devices and **1 h** for local apps, and the CA caps it again.

### Certificate URI SANs

| SAN | Meaning | Source |
|---|---|---|
| `qar://peer/<peer-id>` | The QAROS peer the certificate is issued to | Derived by the gateway from the channel's peer, not copied from the CSR |
| `qar://session/<session-id>` | The session the certificate is bound to | The channel's session; omitted when there is none |
| `qar://member/<peer-id>` | The member Hub the request came through | Stamped by the primary on every forwarded request, never copied from the CSR |

A stored certificate is reused only while the peer, the session and the session name still match; otherwise the device generates a new key and enrols again. A process opens a session only with a certificate naming that session and its own peer id.

## Layer 3 - Session (mutual TLS)

- Each peer builds its TLS configuration from the received CA bundle plus its **own** key and certificate. Material is passed to Zenoh in memory; no temporary files.
- With TLS on, multicast scouting is off and peers dial known endpoints in `19120-19200`.
- Trust anchor: the merged CA bundle. `verify_name_on_connect` is off - hostnames are not checked, by design.
- **Session access control.** Every peer admits only the session's three names (device, member, Hub) and refuses everything else by default. Only the Hub name may publish on the primary's authority topics (trust bundle, revocation list, approvals, member admission, device admin, rotation); the member name may also publish on the per-Hub topics (launcher, targets, pairing, role, audit). A peer with another certificate keeps its link but nothing it sends is delivered.
- TLS material is redacted from logs.

## Hub self-enrolment

The Hub signs its own certificate through an in-process gateway (no network hop) under the same policy checks, against an inventory containing only itself. Its peer id comes from local configuration, not from the wire.

## What is stored where

Every secret is in an encrypted store; the rest are plain files. An **installed** Hub keeps its own secrets in the store of the QAROS System Service under `%ProgramData%`; a Hub run from a development build, and every device and app, keep theirs in `secure-store` under the runtime storage root (`runtimeStorageRoot`, default the per-user data folder). Paths are relative to the Hub's configured CA directory (`caDir`) and runtime storage root - see [Launcher Configuration Reference](./launcher-config-reference.md).

| Location | Contents | Protection |
|---|---|---|
| `%ProgramData%\Quaternar\Qaros\secure\store\hub\` (installed Hub) | Hub and member identity, CA private keys (primary only; a member deletes them on join), trust-authority key, session-store key, device blacklist, issued-certificate inventory, approved local apps, launcher state; one folder per Windows user and Hub data folder. The licence is kept once for the PC (`hub\machine`): every Windows user's Hub reads it, any of them may activate or replace it, none can delete it or put an older one back | **Encrypted** (AES-256-GCM per value, bound to its name) under the machine key below. The folder and the key are usable only by SYSTEM and the QAROS service; the service hands values only to the installed Hub (`qar-runtime-launcher.exe`) and `qaros-license.exe` from the installation folder, checked by Windows, not by what they claim |
| `<storage>/secure-store/` | Each device and app's identity (private key, certificate chain, CA bundle) and last session; trusted Hub CAs; the Hub's own secrets (as above) when the Hub runs from a development build | **Encrypted** (AES-256-GCM per value, bound to its name) under a key of the signed-in user: the TPM through the Microsoft Platform Crypto Provider, or DPAPI where no TPM works |
| `caDir/ca/certs/` | Root and intermediate CA certificates (primary only) | Public trust material |
| `caDir/ca/db/revoked-serials.json` | Revoked serials, an audit artifact | Not relied on for trust |
| `caDir/hub-state.json` | Hub name, room, remembered devices, membership (member) or members and their pinned keys (primary) | OS file permissions only |
| `caDir/audit/` | Hash-chained audit log; on a primary also `members/<peer>/` | Checkpoints signed with the Hub key; OS file permissions |
| `caDir/security-service/` | Gateway audit log | OS file permissions only |
| `%ProgramData%\Quaternar\Qaros\secure\` | Service storage: a machine key `QAROS-storage-kek-v1-system` on the TPM (or software) usable only by SYSTEM and the QAROS service | Protects the installed Hub's secrets; the service does not start when it holds secrets but cannot use the key |

- The store's key never leaves the TPM and needs no PIN; it is not bound to the boot state. Secrets are bound to **this PC** (and, in `secure-store`, to this Windows user): a copy of the folder, a backup restored elsewhere, or another user cannot read them.
- Malware running as the signed-in user cannot read an installed Hub's secrets from disk or with the user's key. It could still read what the running Hub holds in memory by injecting into it; keep antivirus or EDR on the Hub PC.
- A store the PC can no longer read (TPM cleared, Windows reinstalled, profile moved) is reported, never silently replaced. A development Hub does not start until the `secure-store` folder is deleted; an installed Hub recovers with a reset to a new install. Either creates a new CA: every device pairs again.
- An installed Hub does not start while the QAROS System Service is stopped; it never falls back to the user's store.
- Values that must not go backwards (blacklist, approvals, licence) carry a revision; an older one is refused while the Hub runs, not across a restart.
- Updating from a build before this storage destroys the old plaintext key files at the first start; nothing is imported (see [Maintenance & Updates](./maintenance-and-updates.md#updating-to-encrypted-storage)).

## Renewal

- A device renews online, with a new key, over a channel on the Hub's onboarding port where both ends prove the certificates they hold. No code is needed.
- It renews during the session before expiry, and on joining after a long time offline.
- After a session rotation, a certificate of one of the two previous sessions renews into the current one; it is admitted for nothing else.
- Every renewal proves possession: a signature by the certificate's current key over the new request, verified by every hub path before it signs. A renewal without it (an SDK older than the hub) is refused with `renewal_possession_unproven` and the device cannot renew until it is updated ([Maintenance & Updates](./maintenance-and-updates.md#certificate-renewal-needs-the-current-sdk)).
- An expired, revoked or older certificate is refused: the device reports that it must be paired again and the operator pairs it with a new code.

## Revocation and blacklist

| Step | Effect |
|---|---|
| 1. CA revocation | The certificate serial is revoked at the local CA |
| 2. Gateway blacklist | Device added to the encrypted blacklist; blocks enrolment and renewal |
| 3. Peer revocation list | The Hub publishes the blacklist (peer id, serial) on a stored topic; peers that read it refuse the device |
| Re-entry | Only after an operator clicks **Allow**; a new pairing code alone is refused |

- A peer that has read the list refuses the device's peer id: the launcher releases its pipeline at once and never provisions it again. The zenoh link is not closed; zenoh decides TLS acceptance itself and offers no per-certificate hook.
- A peer that has not read the list (storage unreachable, older build) accepts the certificate until it expires, at most 24 h. A missing list is "unknown", never "everyone is revoked".
- Removing a member Hub revokes the member and every certificate it forwarded. Revoking an app approval does not revoke certificates already issued through members; they expire within 30 minutes.
- Revoke / allow / list run over the session for the Hub, its own Visualizer and configured operator peers. The Hub takes the sender from a signature made with the sender's session certificate, never from a field of the request. Revoke and allow also need the Hub's machine. Every action is audited.

## Member Hubs

```text
Hub M -> primary P : signed join request (M's Hub key, CSR, names, nonce) on P's onboarding port
P                  : pending until an operator allows it (or a join code / a member-Hub invitation)
P -> M             : reply signed with P's Hub key: member certificate, CA bundle, session invite
M                  : verifies the reply, pins P's key, deletes its own CA, restarts as a member
```

- **One CA per session.** A member trusts only the primary's CA; there is no merge of roots between Hubs.
- **Forwarding.** A member authorizes its own headsets and local apps, then forwards each request over a certificate-authenticated channel to the primary, which authorizes it again (member admitted and not removed, device name only, app approval applies to that member, quota of 200 per 10 minutes, peer id not taken) and stamps the member's SAN.
- **Invitations.** An app approved on the primary to bring in member Hubs creates an invitation signed with P's Hub key (24 h by default, at most 7 days, bounded uses, revocable in the Visualizer); each Hub admitted spends one unit of that app's quota. The joining Hub proves it holds the invitation's secret without sending it, and accepts P only if P's key is in the administrator's allow-list or, without one, if P is the first primary it joined by invitation (pinned). A writable policy refuses every invitation.
- **Pinning.** The primary pins the member's Hub key; the member pins the primary's on the first verified reply, or checks it against the invitation's key. A pinned member rejoins without a click and spends no quota.
- **Devices move along.** A Hub that was a primary of its own can take its paired devices, launcher children and approved apps into P's session: it cross-certifies P's CA with its old CA root before it restarts, and its devices renew through it without a code (old certificates for 30 days, known device keys for 180 days).
- **No chains.** A member redirects joiners to its primary; a Hub with members cannot join another.
- **Reset.** A member detects that its primary started a new session within about 10 s and restarts into it; when the primary is unreachable it backs off and reports it.
- **EST.** An EST-backed primary admits a Hub joining by invitation in one round. A Hub whose own certificates came from EST cannot take its devices along; they pair again.

## Audit log

- Every Hub writes enrolments, issued and refused certificates, operator actions and trust changes to a daily JSON Lines file. Each record is SHA-256 chained to the previous one; every 100 records or 10 minutes a checkpoint is signed with the Hub key.
- Kept 90 days, at most 256 MB. Secrets are never logged.
- Members forward their records to the primary, which verifies them against the member's pinned key and marks any gap.
- Read by operators only (signed request), in the Visualizer's *Audit log* panel.
- Limit: the Hub's own Windows user can read the key and rewrite the whole log; there is no off-PC export yet.

## Assumptions

| # | Assumption | If it breaks |
|---|---|---|
| A1 | The Hub machine is trusted and operator-controlled | Full break: CA keys are on it |
| A2 | The pairing code is shown only to the intended person, for its 10 s | Whoever has the code and network reach can onboard |
| A3 | A provisioned key reaches only the process it was minted for | Its holder can onboard within 5 minutes |
| A3b | A requested or launch code reaches only the device or app it was minted for | Its holder can onboard within its lifetime (at most 10 minutes) |
| A4 | OS file permissions on the Hub and devices are enforced | Local admin / same-user processes read keys |
| A5 | Deployment is a LAN or private network | Discovery and pairing ports are not designed for internet exposure |
| A6 | One signing CA per session, the primary's; members have none | - |
| A8 | A member Hub's account of what it verified on its PC is believed | A compromised member gets device certificates for its own PC within its quota |
| A7 | Discovery is untrusted input | - |

## Limitations

Ranked by severity. These are current, known engineering items.

| # | Severity | Limitation | Consequence |
|---|---|---|---|
| L5 | High | **Secrets at rest are protected by the signed-in user's key.** They are encrypted under a TPM-backed (or DPAPI) key of that user; a program running as the same user can open them like the Hub does. | Malware in the Hub user's account can read the CA keys and approve itself; other users, disk copies and backups cannot. |
| L20 | Low | **A rollback between two Hub runs is not detected.** Someone who can write the store's folder while the Hub is stopped can put back an older blacklist, approval list or licence. | A revoked device could be unrevoked by restoring an older copy. |
| L7 | Medium | **Revocation reaches only peers that read the Hub's list, and is enforced by peer id, not at TLS.** | A peer that has not read it accepts a revoked certificate until expiry (≤ 24 h); one that has keeps the link open but refuses to serve the device. |
| L14 | Medium | **The peer id is not checked per connection.** A device certificate is the same for every device of a session, so a device can claim another device's peer id in session messages. | It cannot act as the Hub: the Hub's topics accept only the Hub's certificate. |
| L15 | Medium | **Member Hubs share one name.** One member can publish under another member's identity on the topics members may use. | It cannot publish the primary's authority topics. |
| L18 | Medium | **First join trusts the operator or the first key.** *Allow* relies on recognising the Hub; a join by invitation without an administrator's allow-list trusts the primary of the first invitation the Hub accepts. | A wrong *Allow* admits a rogue member with member rights. |
| L19 | Low | **A member follows a new session by restarting.** | Its local apps and headsets are interrupted briefly. |
| L16 | Low | Same-machine checks (revoke, allow, rotation) trust the machine name a peer states about itself. | A remote operator peer could pass for local. |
| L17 | Low | Zenoh keeps the TLS key in its own memory unscrubbed while the session runs. | Memory read access to a running process reveals its session key. |

## Roadmap

Planned, in priority order. Nothing here is shipped.

1. Secrets held by the QAROS system service instead of the user (its TPM machine key is already provisioned), and rollback detection across restarts - closes L5 and L20.
2. Scrubbing Zenoh's copy of the TLS key - closes L17.
