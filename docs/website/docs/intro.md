---
sidebar_position: 1
title: Quaternar Platform Overview
description: High level summary of the Quaternar AR/VR middleware and guiding principles.
---

Quaternar delivers middleware, runtimes, and APIs that let software teams add AR/VR capabilities to existing 3D applications without rebuilding their codebases. The solution targets professional developers who need a fast, reliable path to multi-device immersive experiences.

## Value Proposition

- Remove the steep learning curve of raw OpenXR by providing a layered API focused on developer productivity.
- Support both tethered headsets (Valve Index, Varjo, HTC Vive…) and remotely connected devices (Meta Quest, HoloLens, tablets, phones, Apple Vision Pro).
- Package remote rendering, multi-user collaboration, and rich 3D UI components as first-class features.
- Keep simple use cases simple: a new developer should be able to add basic AR/VR support in a single day.

## Product Objectives

- Ship an approachable AR/VR API and runtime stack that covers a wide device spectrum.
- Offer optional advanced features—remote rendering, multi-device orchestration, multi-user sessions—without burdening basic scenarios.
- Deliver comprehensive documentation, samples, and integrations with popular open-source tools (Blender, ParaView, CloudCompare, …).
- Maintain compatibility with OpenXR while extending the ecosystem with Quaternar-specific UI, networking, and device services.
- Provide high quality visuals for demos and marketing, including full-HD remote streams with alpha and depth channels for superior capture.

## Core Capabilities

- **Remote Rendering:** Stream high fidelity frames (RGB/A/D) from powerful source machines to lightweight clients.
- **3D UI & Debug Tooling:** Inject interactive panels, adjust transforms live, and iterate without recompiling.
- **Mixer Pipeline:** Combine multiple 2D/3D applications into tailored output streams per user.
- **Device & Session Management:** Discover devices, connect/disconnect, and sync head pose, controllers, and sensor data.
- **Extensible Runtime:** Blend Quaternar APIs with native OpenXR when specific functionality is required.

## Key Terminology

- **Device:** Any AR/VR headset or companion screen (tethered or standalone).
- **Source Machine:** Windows/Linux workstation that runs the customer’s 3D application.
- **Source App:** Customer application that renders AR/VR frames; can be monolithic or split via plugins.
- **Source App Plugin:** Integration layer providing Quaternar APIs and debug tooling inside the source app.
- **Player App:** Quaternar runtime deployed on the headset/tablet/phone to receive streams and report user input.
- **Mixer App:** Service that merges multiple inputs into unique outputs for each participant.
- **State App:** Stores shared scene state and supports backup/restore via API.
- **Session:** Connected group of cooperating source, mixer, player, and UI components.

Use this overview as the entry point for the rest of the knowledge base.
