---
sidebar_position: 7.5
title: Security Deep Dive
description: Auditor-level reference for QAROS onboarding and certificates - layer protocols, EC-JPAKE transcript binding, the secure-channel nonce scheme, CA gateway policy, certificate SANs, storage, renewal, revocation, federation, and known limitations.
---

# Security Deep Dive

This page is for security reviewers and auditors. It states what each layer does, what it assumes, and where today's implementation falls short of the design. For the operator-level summary read [Security Model](./security-model.md) first.

:::caution Read the limitations
The cryptographic core (EC-JPAKE, the AES-256-GCM secure channel, CSR-based enrolment) is sound. The open gaps are in **authorization, trust distribution and secrets at rest** - see [Limitations](#limitations). Do not assume any guarantee this page does not state.
:::

## Layers at a glance

| Layer | Transport | Purpose | Authentication | Confidential |
|---|---|---|---|---|
| 0a Beacon | UDP multicast `239.77.77.77:7445`, every 5 s for a Hub | Announce a Hub on the LAN | None | No |
| 0b Discovery | Framed TCP, the Hub's onboarding port (claimed from 19120-19199) | Return the invitation metadata (no code) | None | No |
| 1 Pairing | Framed TCP, same onboarding port (same listener) | EC-JPAKE - prove knowledge of the code | Pairing code | Handshake only |
| 2 Enrolment | Secure channel over the pairing connection | Issue the device certificate | Key from layer 1 (or a provisioned key) | Yes (AES-256-GCM) |
| 3 Session | Zenoh over TLS, TCP `19120-19200` | All session traffic | Mutual TLS, Hub-CA certificates | Yes |

The Hub's TCP listener reads the first frame of each connection and dispatches it to discovery, pairing or provisioned-key onboarding.

## Layer 0 - Discovery

- **Beacon** carries device id, display name, device type and the discovery endpoint.
- **Discovery reply** carries invitation id, Hub server id, pairing host/port and invitation expiry. Never the code.
- Both are plaintext and unauthenticated by design. Discovery is *untrusted input*; the code is the trust boundary.

| Attacker | Outcome |
|---|---|
| Forged beacon / forged discovery reply | Device is pointed at a rogue host; pairing fails without the code |
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
| Codes an onboarded app requests (1-64 per request, optionally bound to a planned peer id) | 2 min default, 10 min max | No |
| Launch code of a QAROS API app, one per start | 2 min | No, written to the app's stdin only |

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
- The Hub **assigns** the common name: a fresh `<uuid>.device.qaros.local` per enrolment.

### CA gateway policy

The CA itself is reachable only on the Hub's loopback interface. Every signing request passes the gateway first:

| Check | Rejects with |
|---|---|
| Request id or nonce seen before (replay cache) | `replay_rejected` |
| Channel binding does not match the channel | rejected before authorization |
| Channel not authenticated | `unauthenticated` |
| Claimed peer id differs from the channel's peer | `peer_mismatch` |
| Requested session differs from the channel's session | `session_mismatch` |
| CSR empty / larger than 32 KiB | `missing_csr` / `csr_too_large` |
| Device blacklisted and channel is not a fresh pairing | `device_revoked_manual_pake_required` |
| Peer not in the gateway inventory | `unknown_peer` |
| Profile, DNS SAN or IP SAN not assigned | `profile_not_allowed` / `dns_san_not_allowed` / `ip_san_not_allowed` |
| Common name differs from the assigned one | `common_name_not_allowed` |

Every decision is written to the gateway audit log.

### Signing

- Backend: a private Smallstep `step-ca` on loopback, reached over HTTPS with the root fingerprint pinned.
- Each request carries a one-time token (ES256 JWT) signed with the provisioner key. The token lives 5 minutes and is never stored. The provisioner key is decrypted in memory only.
- Certificate lifetime: the Hub requests the configured lifetime (`certLifetimeHours`); the CA's provisioner policy caps it at **24 h**.

### Certificate URI SANs

| SAN | Meaning | Source |
|---|---|---|
| `qar://peer/<peer-id>` | The QAROS peer the certificate is issued to | Derived by the gateway from the channel's peer, not copied from the CSR |
| `qar://session/<session-id>` | The session the certificate is bound to | The channel's session; omitted when there is none |

A stored certificate is reused only while both the peer and the session still match; otherwise the device generates a new key and enrols again. **The SANs are minted but not enforced at connect** - see L4.

## Layer 3 - Session (mutual TLS)

- Each peer builds its TLS configuration from the received CA bundle plus its **own** key and certificate. Material is passed to Zenoh in memory; no temporary files.
- With TLS on, multicast scouting is off and peers dial known endpoints in `19120-19200`.
- Trust anchor: the merged CA bundle. `verify_name_on_connect` is off - hostnames are not checked, by design (L4).
- TLS material is redacted from logs.

## Hub self-enrolment

The Hub signs its own certificate through an in-process gateway (no network hop) under the same policy checks, against an inventory containing only itself. Its peer id comes from local configuration, not from the wire.

## What is stored where

Paths are relative to the Hub's configured CA directory (`caDir`) and runtime storage root (`runtimeStorageRoot`) - see [Launcher Configuration Reference](./launcher-config-reference.md).

| Location | Contents | Protection today |
|---|---|---|
| `caDir/certs/` | Root and intermediate CA certificates | Public trust material |
| `caDir/secrets/` | CA private keys (step-ca encrypted), blacklist key | OS file permissions only |
| `caDir/config/ca.json` | CA configuration incl. the encrypted provisioner key | OS file permissions only |
| `caDir/ca-password.txt`, `provisioner-password.txt` | Passwords that unlock the CA keys and the provisioner key | **Plaintext**, OS file permissions only |
| `caDir/ca-db.bolt` | CA database (issued records, used tokens) | Not relied on for trust |
| `caDir/policy/device-blacklist.enc` | Device blacklist, AES-256-GCM, tamper-evident | Key stored on the same host |
| `caDir/security-service/` | Gateway audit log, issued-certificate inventory | OS file permissions only |
| `<storage>/identities/<process>/` | Per-process `private_key.pem`, certificate chain, CA bundle, last CSR, binding metadata | **Plaintext**, OS file permissions only |
| `<storage>/identities/<process>/trusted-hub-ca-bundles.json` | Merged federation trust store | OS file permissions only |
| `<storage>/zenoh-storage.key` | Key for the encrypted session store | **Plaintext hex** next to the data |

No file is written with explicit permission hardening; each inherits its parent directory's ACL. Protect `caDir` and the storage root like a domain controller's secrets.

## Renewal

- The renewal policy marks a certificate due at **75 %** of its lifetime or within **1 h** of expiry.
- There is **no online renewal path in production today**. The gateway's renewal check (current-certificate fingerprint bound to a certificate-authenticated channel) exists but no production channel supplies that binding.
- Effect: once a stored certificate is due, silent rejoin is refused and the device **onboards again** (new code, or a new provisioned key for Hub-launched processes). With the default 24 h cap, that is roughly once a day for a device that is not re-launched by the Hub.

## Revocation and blacklist

| Step | Effect |
|---|---|
| 1. CA revocation | The certificate serial is revoked at the local CA |
| 2. Gateway blacklist | Device added to the encrypted blacklist; blocks enrolment and renewal |
| 3. Peer revocation list | The Hub publishes the blacklist (peer id, serial) on a stored topic; peers that read it refuse the device |
| Re-entry | Only through a fresh pairing; a successful enrolment on that pairing clears the entry |

- A peer that has read the list refuses the device's peer id: the launcher releases its pipeline at once and never provisions it again. The zenoh link is not closed; zenoh decides TLS acceptance itself and offers no per-certificate hook.
- A peer that has not read the list (storage unreachable, older build) accepts the certificate until it expires, at most 24 h. A missing list is "unknown", never "everyone is revoked".
- Revoke / allow / list is restricted to configured admin peers, but it is an in-process interface; there is no authenticated remote control plane yet.

## Hub federation via trust bundles

```text
Hub A pairs into Hub B (B's code)  ->  A receives B's CA bundle during enrolment
A merges B's roots into its trust store (dedup by certificate fingerprint, versioned, persisted)
A publishes the merged bundle to its session devices  ->  each device merges, persists, acknowledges
```

- Federated **trust**, not federated signing: each Hub signs only its own devices.
- The merged store survives restarts without the remote Hub online.
- Bundle updates are ordered per connection id (stale versions dropped).
- Treat a federation as a commitment: there is no operator action to withdraw a remote root. See [Networking & Federation](./networking-and-federation.md).

## Assumptions

| # | Assumption | If it breaks |
|---|---|---|
| A1 | The Hub machine is trusted and operator-controlled | Full break: CA keys are on it |
| A2 | The pairing code is shown only to the intended person, for its 10 s | Whoever has the code and network reach can onboard |
| A3 | A provisioned key reaches only the process it was minted for | Its holder can onboard within 5 minutes |
| A3b | A requested or launch code reaches only the device or app it was minted for | Its holder can onboard within its lifetime (at most 10 minutes) |
| A4 | OS file permissions on the Hub and devices are enforced | Local admin / same-user processes read keys |
| A5 | Deployment is a LAN or private network | Discovery and pairing ports are not designed for internet exposure |
| A6 | One signing CA per Hub; federation shares roots only | - |
| A7 | Discovery is untrusted input | - |

## Limitations

Ranked by severity. These are current, known engineering items.

| # | Severity | Limitation | Consequence |
|---|---|---|---|
| L1 | High | **Trust-bundle updates are not signed or bound to the Hub.** Devices accept a bundle update from any authenticated session peer. | An onboarded peer can inject a CA root that persists across restarts. Combined with L4, that allows peer impersonation inside the session. |
| L2 | High | **Peer identity on the pairing path is self-asserted.** The gateway takes the peer id from the client's request; pairing proves knowledge of the code, not an identity. | A paired device can obtain a certificate with any `qar://peer/<id>`. The common name is Hub-assigned and not affected. |
| L3 | High | **Blacklist is keyed on that self-asserted id.** | A revoked device can re-pair under a new id (it still needs a valid code). |
| L4 | Medium | **No identity check at connect.** Hostnames are not verified and `qar://peer` / `qar://session` SANs are not enforced; any certificate chaining to a trusted root is accepted as any peer. | Amplifies L1 and L2. |
| L5 | High | **Secrets at rest are plaintext** (device private keys, CA and provisioner passwords, session-store key) and rely on OS file permissions; the blacklist key sits beside the blacklist. | File-system read access means key theft or CA takeover. |
| L6 | Medium | **No online renewal**; certificates are short-lived and devices re-onboard. | Availability cost, not a confidentiality gap. |
| L7 | Medium | **Revocation reaches only peers that read the Hub's list, and is enforced by peer id, not at TLS.** | A peer that has not read it accepts a revoked certificate until expiry (≤ 24 h); one that has keeps the link open but refuses to serve the device. |
| L8 | Medium | **No rate limit** on concurrent pairing handshakes or CA signing. | CPU / CA exhaustion by connection flood. |
| L9 | Medium | **Admin surface is in-process only**; the admin actor is caller-asserted. | No authenticated remote revocation yet. |
| L10 | Medium | **Certificate lifetime is capped only by CA configuration**, not also by the gateway. | A mis-edited CA config would lift the cap. |
| L11 | Low | The default (Smallstep) CA bootstrap is Windows-only. | No supported Linux Hub CA yet. |
| L12 | Low | Discovery is plaintext; some discovery inputs are loosely validated. | Denial of service only. |
| L13 | Low | Confirmation tags are compared in non-constant time. | Not practically exploitable over a network. |

## Roadmap

Planned, in priority order. Nothing here is shipped.

1. Authenticate trust-bundle updates (bind to the Hub authority and/or sign them) - closes L1.
2. Hub-assigned peer identity bound to the pairing, blacklist keyed on it, and SAN enforcement at connect - closes L2, L3, L4.
3. Encrypted storage for keys and secrets (DPAPI on Windows, Android Keystore, TPM where available), with rollback detection for policy stores - closes most of L5.
4. Authenticated online renewal over mutual TLS - closes L6.
5. Gateway rate limits, gateway-side lifetime clamp, authenticated remote admin endpoint - closes L8, L9, L10.
6. Discovery input hardening and constant-time comparisons - L12, L13.
