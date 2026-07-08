---
sidebar_position: 1
title: Welcome to QarOS
description: What QarOS is, who this documentation is for, and where to start.
---

# Welcome to QarOS

QarOS is Quaternar's AR/VR middleware: it lets you add multi-device, multi-user AR/VR capabilities to an existing 3D application without rebuilding it. Your application keeps rendering the way it always has; QarOS streams its output to headsets, tablets, and phones, composites content from several applications into one coherent shared space, and handles discovery, security, input, and session management for you.

**Don't rewrite. Extend.**

## How this documentation is organized

The documentation is split into three self-contained guides by audience, plus a generated API reference. Each guide has its own navigation, so you stay in one guide to accomplish your goals.

- **[User Guide](/docs/user-guide/qaros-world)** - for people who join a QarOS Hub from a device, and for anyone wanting a non-technical picture. It explains the QarOS world without code: joining your first Shared Space, connecting to the Hub, onboarding your device, understanding the shared space, and working in it.
- **[Developer Guide](/docs/developer-guide/getting-started)** - for engineers integrating the QarOS API (C or C#/.NET) into their application: SDK setup, API conventions, coordinate systems, onboarding and sessions, streaming rendered frames, GUI panels, and app volumes. Code samples are selectable per language, and tutorials are backed by example programs compiled as part of this repository, so every referenced snippet is real, valid code.
- **[Administrator & Operator Guide](/docs/operator-guide/overview)** - for people who install, secure, configure, run, and maintain a Hub: system requirements, installation, Hub operation, networking and Hub federation, the security model, device management, the mixer, and maintenance.
- **[C API Reference](/api/qar-streaming-c/)** - the complete API reference, generated from the C headers with Doxygen.

A shared **[Glossary](/docs/glossary)** defines the cross-cutting terms.

## The elevator pitch, in one diagram

A **Hub** (a powerful workstation) runs your 3D applications and the QarOS runtime. Devices - HoloLens, Android phones and tablets, VR headsets, desktop viewers - **onboard** to the Hub with a short pairing code and join a shared **Shared Space**. In the developer/runtime model, that Shared Space maps to a **session**. Each application renders into an **app volume** (a 3D region inside the space, potentially spanning multiple rooms); the **mixer** composites all volumes into one personalized, latency-compensated view per user, and **GUI panels** provide shared 2D interfaces floating in 3D space.

![QAROS world overview](/img/diagrams/qaros-world-overview.png)

## Where to start

| I want to... | Start here |
|---|---|
| Understand what QarOS is made of | [The QAROS World](/docs/user-guide/qaros-world) |
| Join a Shared Space from my device | [User Guide: Getting Started](/docs/user-guide/getting-started) |
| Connect a HoloLens / Android device | [Onboarding Devices](/docs/user-guide/onboarding-devices) |
| Install and run a Hub | [Installation](/docs/operator-guide/installation) |
| Run and manage a Hub | [Operator Guide: Overview](/docs/operator-guide/overview) |
| Understand the security model | [Security Model](/docs/operator-guide/security-model) |
| Integrate the API into my app | [Developer Guide: Getting Started](/docs/developer-guide/getting-started) |
| Stream my rendered frames | [Rendering Streams](/docs/developer-guide/rendering-streams) |
| Browse the full C API | [C API Reference](/api/qar-streaming-c/) |
