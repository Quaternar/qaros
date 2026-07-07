---
sidebar_position: 1
title: Welcome to QarOS
description: What QarOS is, who this documentation is for, and where to start.
---

# Welcome to QarOS

QarOS is Quaternar's AR/VR middleware: it lets you add multi-device, multi-user AR/VR capabilities to an existing 3D application without rebuilding it. Your application keeps rendering the way it always has; QarOS streams its output to headsets, tablets, and phones, composites content from several applications into one coherent shared space, and handles discovery, security, input, and session management for you.

**Don't rewrite. Extend.**

## How this documentation is organized

The documentation is split by audience:

- **[User Guide](/docs/category/user-guide)** — for operators, project managers, and anyone evaluating or running QarOS. It explains the QAROS world without code: what entities exist (Hub, devices, sessions, app volumes, GUI panels, streams), how they behave, how coordinate spaces relate, how security and onboarding work, what the mixer does, and how to install and run the system.
- **[Developer Guide](/docs/category/developer-guide)** — for engineers integrating the QarOS C API into their application. It covers SDK setup, API conventions, onboarding and sessions, streaming rendered frames, creating GUI panels, and working with app volumes. Tutorials are backed by example programs that are compiled as part of this repository, so every referenced snippet is real, valid code.
- **[C API Reference](/api/qar-streaming-c/)** — the complete API reference, generated from the C headers with Doxygen.

## The elevator pitch, in one diagram

A **Hub** (a powerful workstation) runs your 3D applications and the QarOS runtime. Devices — HoloLens, Android phones and tablets, VR headsets, desktop viewers — **onboard** to the Hub with a short pairing code and join a shared **session**. Each application renders into an **app volume** (a 3D region of the shared room); the **mixer** composites all volumes into one personalized, latency-compensated view per user, and **GUI panels** provide shared 2D interfaces floating in 3D space.

![QAROS world overview](/img/diagrams/qaros-world-overview.svg)

## Where to start

| I want to… | Start here |
|---|---|
| Understand what QarOS is made of | [The QAROS World](/docs/user-guide/qaros-world) |
| Install and run a Hub | [Deployment and Installation](/docs/user-guide/deployment-and-installation) |
| Connect a HoloLens / Android device | [Onboarding Devices](/docs/user-guide/onboarding-devices) |
| Understand the security model | [Security and Onboarding](/docs/user-guide/security-and-onboarding) |
| Integrate the C API into my app | [Developer Guide: Getting Started](/docs/developer-guide/getting-started) |
| Stream my rendered frames | [Rendering Streams](/docs/developer-guide/rendering-streams) |
| Browse the full C API | [C API Reference](/api/qar-streaming-c/) |
