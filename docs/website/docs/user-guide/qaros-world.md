---
sidebar_position: 1
title: The QAROS World
description: The entities that make up a QarOS deployment — Hub, devices, sessions, peers, app volumes, GUI panels, and streams — and how they behave.
---

# The QAROS World

Before touching any API, it helps to understand the cast of characters. A running QarOS deployment is a small world with a fixed set of entity kinds. Each entity has an identity, a lifecycle, and a clear set of responsibilities.

![QAROS world overview](/img/diagrams/qaros-world-overview.svg)

## The Hub

The **Hub** is the heart of a deployment: a powerful workstation (Windows or Linux) that runs your 3D applications, the QarOS runtime services, and the security infrastructure. One Hub serves one physical room of collaborating users.

The Hub is responsible for:

- **Discovery** — it broadcasts a beacon on the local network so devices can find it without any configuration.
- **Onboarding** — it displays a short **pairing code** and runs the secure pairing protocol when a new device wants to join.
- **Identity** — it operates its own private Certificate Authority (CA) and issues a certificate to every device it accepts. All later traffic is mutually authenticated with these certificates.
- **Session hosting** — it hosts the shared session that all peers participate in, and routes all session traffic over an encrypted (mTLS) message bus.
- **Compute** — it runs the source applications and the mixer, so lightweight devices only need to decode video.

Hubs can also connect to *each other*. When two Hubs federate, they merge their trust bundles, so a device onboarded to one Hub is accepted by the other. Each Hub still issues certificates only for its own devices.

## Devices and peers

A **device** is anything that joins the Hub's world: an AR headset (HoloLens), a VR headset, an Android phone or tablet, a desktop visualizer window, or a camera. Once a device has onboarded, it participates in the session as a **peer**.

A **peer** is the session-level identity of a participant — a user's device or an application. Every peer:

- has a stable **peer ID** and a certificate binding that identity cryptographically,
- advertises *presentation* metadata: a display name, application version, custom application info, and an application state (initializing / running / shutting down),
- can be enumerated and observed by every other peer — your application can list who is in the session and react when peers join, leave, or change state.

Two broad kinds of peers exist in practice:

- **Source peers** — applications that *produce* content: they render frames into an app volume.
- **Target peers** — devices that *consume* content: they display the mixed result to a user and send back input (head pose, hands, controllers).

A single machine can host several peers: for example, the Hub itself runs source applications, a mixer, and possibly a desktop visualizer, each participating as its own peer.

## Sessions

A **session** is the shared collaborative space — the group of peers that see the same room, the same app volumes, and the same GUI panels. In QarOS you do not "create" or "join" a session by hand: a session membership is the *result of onboarding*. When a device pairs with the Hub, it receives a certificate and enters the Hub's session; when it reconnects later, it *rejoins* the same session using its persisted identity, with no user interaction.

Session state is persistent. When a peer leaves, its identity slot (certificate, instance state, last-session state) stays on disk, so it can rejoin seamlessly. Only an explicit *forget* wipes that identity.

## App volumes

An **app volume** is a 3D region of the shared room where one application renders its content — think of it as a window, except it is a box in space rather than a rectangle on a screen.

Key behaviors:

- **Identity by name.** A volume is identified by a stable *common name* (for example `cad-review.hub1.example.com`). Asking for the same name always yields the same volume, so applications can restart and reattach without creating duplicates.
- **Placed in the room.** A volume has a pose (position + orientation) and a size (width × length × height in meters) in room space. Users or applications can move and resize it.
- **Contains app content at its own scale.** Inside the volume, the application's own coordinate system lives at an adjustable position and scale — a whole factory model can be shrunk into a tabletop box. See [Coordinate Systems](/docs/user-guide/coordinate-systems).
- **Interactive.** Volumes support a configurable gesture system: users can click, hover, grab and move, rotate, and scale the volume (or the app content inside it) with one or two hands, with per-axis constraints and precision settings.
- **Access-aware.** A volume tracks which peers are using it, and an editing lock reports which peer is currently manipulating it, so two users don't fight over the same object.
- **Streamed and mixed.** Each app volume is the unit of streaming: the mixer composites one stream per volume into each user's personal view. See [The Mixer](/docs/user-guide/the-mixer).

## GUI panels

A **GUI panel** is a 2D interface floating in 3D space — settings pages, dashboards, web content, control surfaces. Panels are shared session state: every participant sees the same panel in the same place (unless it is hidden from them).

- Panels use the same *common name* identity model as app volumes — recreate-by-name is idempotent.
- A panel has a pose in the room, a physical size in meters plus a content scale, and a display state (visible, minimized, hidden, closed).
- Panel content is currently a **website URI** — the panel renders a web page, and applications can navigate it remotely. This makes panels the easiest way to give your existing web UI a presence in the 3D room.
- Visibility is **per peer**: a panel can be shown to everyone or restricted to a chosen set of peers (for example, an operator-only control panel).

## Streams

A **stream** carries rendered frames from a source application to the room. The source application drives a simple frame loop — begin a frame, render into the provided textures (CPU buffers or GPU textures), and show the frame with its projection parameters. QarOS handles encoding, transport, decoding, warping, and compositing.

Streams carry more than color: frames can include **alpha** and **depth** channels, which is what allows the mixer to correctly occlude and blend multiple applications in one space, and to reproject frames to the viewer's latest head pose. In return, the source application can read back the viewer's **hand tracking** data (26 joints per hand) and pose information to make its content interactive.

## The mixer

The **mixer** is the per-user compositor. Every user gets their own mixer instance which takes all the app-volume streams, re-projects (warps) each one to that user's freshest head pose, masks each stream to its volume's box, depth-sorts and blends them, and outputs a single stereo color+depth frame for that user's device. This is what makes multiple independent applications appear as one coherent, low-latency world. Details in [The Mixer](/docs/user-guide/the-mixer).

## Identity and trust, in one paragraph

Every entity that participates in a session has a cryptographic identity issued by the Hub's private CA during onboarding. The pairing code a user types is never sent over the network — it feeds a password-authenticated key exchange, which bootstraps an encrypted channel, over which the device receives its certificate. From then on, all session traffic is mutual-TLS. The full story is in [Security and Onboarding](/docs/user-guide/security-and-onboarding).

## Lifecycle summary

| Entity | Created by | Identity | Persists across restarts? |
|---|---|---|---|
| Hub | Operator installs & starts it | Its CA root + server ID | Yes |
| Peer | Onboarding a device/app | Peer ID + certificate | Yes (until *forget*) |
| Session | The Hub | Session ID | Yes |
| App volume | An application, by common name | Common name → deterministic ID | Yes (until closed) |
| GUI panel | An application, by common name | Common name → deterministic ID | Yes (until closed) |
| Stream | A source application, per target | Stream ID | No — recreated per run |
