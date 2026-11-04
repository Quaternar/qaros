---
sidebar_position: 2
title: Concepts
description: The QAROS world mapped to API objects - how Hubs, sessions, peers, panels, volumes, and streams appear in the API, with a C to C# name map.
---

# Concepts

This guide is self-contained: you do not need the User Guide to integrate QAROS.
This page maps the [QAROS world](/docs/user-guide/qaros-world) onto the API
objects you actually call. The prose is language-neutral; only the name table
at the end is paired C <-> C#. How the Rust binding maps the C API is in
[Language Bindings](/docs/developer-guide/language-bindings#rust).

## The objects you work with

- **Library** - process-wide initialization and logging. You initialize it once
  at startup and tear it down last. It owns no session state; it is the entry
  point that makes every other call valid.
- **Runtime** - your process's QAROS context: its identity store, its connection
  to a Hub, and the factory for sessions. One runtime per logical QAROS instance;
  local-app instances in one process with the same app tag share one session (see
  [several API instances in one process](/docs/developer-guide/onboarding-and-sessions#several-api-instances-in-one-process)).
- **Session** - the developer/runtime representation of a Hub's Shared Space, always obtained
  through [onboarding](/docs/developer-guide/onboarding-and-sessions), never
  created directly. Everything you do in the room hangs off the session: peers,
  panels, volumes, and render streams.
- **Peer** - any participant in the session (a device or an application),
  described by an immutable snapshot you read or subscribe to. See
  [Working with Peers](/docs/developer-guide/peers).
- **App volume** - your application's named 3-D box in the room; the unit of
  streaming and the anchor for grab/move/scale gestures. See
  [App Volumes and Gestures](/docs/developer-guide/app-volumes).
- **GUI panel** - a shared 2-D quad floating in room space, identified by a
  stable common name and idempotently created. See
  [Custom GUI Panels](/docs/developer-guide/gui-panels).
- **Render sender** - the object that carries your rendered frames into a
  volume, driven by a *begin frame -> render -> show frame* loop. See
  [Rendering Streams](/docs/developer-guide/rendering-streams).
- **Result** - the value every fallible call returns; some of its status codes
  are [expected control flow](/docs/developer-guide/api-conventions#2-results-and-status)
  rather than errors.

How these relate in space (room -> volume -> content -> panels) is the subject of
[Coordinate Systems](/docs/developer-guide/coordinate-systems).

## Identity, naming, and persistence

The User Guide keeps these details high-level. For integration work, the
important concrete rules are:

- **Peers** have stable IDs. A local app on the QAROS PC gets its peer ID back
  on every start: the hub remembers it per approval and app tag, and the app
  stores nothing. An app that paired with a code keeps the onboarding ID its
  first onboard returned and rejoins with it. See
  [Choosing an Onboarding Mode](/docs/developer-guide/onboarding).
- **App volumes** and **GUI panels** are identified by stable common names and
  are created idempotently by those names. See
  [App Volumes](/docs/developer-guide/app-volumes) and
  [GUI Panels](/docs/developer-guide/gui-panels).
- **Streams** are runtime objects recreated per run; they are not the same kind
  of persisted session object as peers, panels, or volumes. See
  [Rendering Streams](/docs/developer-guide/rendering-streams).
- **Target app** is the broad runtime term for a stream consumer. A **player**
  is the device-side companion app variant of a target app; the desktop
  visualizer is a target app too, but not a player. In the API a target app is
  a peer whose role is `TARGET`; your application is always a `SOURCE`. See
  [what a peer is](/docs/developer-guide/peers#what-a-peer-is-role-category-and-device).

## C <-> C# object map

The C# binding lives in the `Quaternar.Qaros` namespace and drops the `Qar` prefix from type
names. Operations on the things inside a session (peers, panels, volumes,
senders) are reached through properties on the session object rather than free
functions.

| Concept | C | C# (`Quaternar.Qaros`) | Covered in |
|---|---|---|---|
| Library init/logging | `qar_library_*` | `Library` (static) | [Getting Started](/docs/developer-guide/getting-started) |
| Process context | `QarRuntime*` | `Runtime : IDisposable` | [Onboarding & Sessions](/docs/developer-guide/onboarding-and-sessions) |
| Room membership | `QarSession*` | `Session : IDisposable` | [Onboarding & Sessions](/docs/developer-guide/onboarding-and-sessions) |
| Onboarding invite | `QarOnboardingInvite*` | `OnboardingInvite : IDisposable` | [Onboarding & Sessions](/docs/developer-guide/onboarding-and-sessions#co-process-a-process-your-app-starts) |
| Participant snapshot | `QarPeerSpec*` | `PeerSpec` (DTO); ops via `session.Peers` | [Peers](/docs/developer-guide/peers) |
| Shared 2-D panel | `QarGuiPanel*` | `GuiPanel` (DTO); ops via `session.GuiPanels` | [GUI Panels](/docs/developer-guide/gui-panels) |
| Application's 3-D box | `QarAppVolume*` | `AppVolume` (DTO); ops via `session.AppVolumes` | [App Volumes](/docs/developer-guide/app-volumes) |
| Frame stream | render sender | `RenderSender`; ops via `session.RenderSenders` | [Rendering Streams](/docs/developer-guide/rendering-streams) |
| Frame handle | `QarRenderFrameInfo*` | `RenderFrameInfo : IDisposable` | [Rendering Streams](/docs/developer-guide/rendering-streams) |
| Call result | `QarResult` (by value) | `Result<T>` / `Result<Unit>` / `ResultInfo` | [API Conventions](/docs/developer-guide/api-conventions#2-results-and-status) |
| Cancellation | `QarCancelToken*` | `System.Threading.CancellationToken` | [API Conventions](/docs/developer-guide/api-conventions#4-blocking--async-pairs) |
| Value IDs | `QarPeerId`, `QarSessionId`, `QarGuiPanelId`, `QarAppVolumeId`, `QarStreamId` | `PeerId`, `SessionId`, `GuiPanelId`, `AppVolumeId`, `StreamId` (readonly structs) | [API Conventions](/docs/developer-guide/api-conventions#1-opaque-handles-and-value-ids) |
| Extensible init | `QarStructureHeader{type,next}` chain | `Structure` / `StructureType` | [API Conventions](/docs/developer-guide/api-conventions#3-extensible-init-structs-qarstructureheader) |

The precise idioms behind these mappings - how handles, results, extension
chains, async pairs, enumeration, and ownership differ between languages - are the
subject of [API Conventions](/docs/developer-guide/api-conventions).
