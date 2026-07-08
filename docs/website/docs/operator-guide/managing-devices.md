---
title: Managing Devices
sidebar_position: 8
description: The operator's side of onboarding - minting and reading pairing codes, listing connected peers, and forgetting or blacklisting a device.
---

# Managing Devices

This page is the operator's side of onboarding. The device owner's side - the steps to bring a HoloLens, P&C Solutions METALENSE 2, Meta Quest, Android device, or visualizer into a session - is in [Onboarding Devices](/docs/user-guide/onboarding-devices). Read both: onboarding is a two-party act, and each page describes one party.

## How pairing codes are minted

The Hub continuously mints short-lived pairing codes:

- A **fresh code snapshot** is minted roughly every 8 seconds.
- Each code is **single-use** - it is consumed on success *and* on failure.
- Each code has a **short time-to-live of about 10 seconds**.
- **One failed attempt invalidates the code** - there is no retry against a spent or failed code.

Security comes from this handling - single-use, short TTL, one-failure-max - not from the length of the code. The code never crosses any transport; it is only ever read by a human and typed into a device.

## The two onboarding paths

**Push (operator-configured).** You add a device through the Hub UI, and the Hub sends it an onboarding invite automatically at startup. This is today's default and the standard path for fixed installations - the device does not need anyone to read a code to it.

**Pull (code entry).** A device's app asks its user for the code currently shown by the Hub and onboards itself. This is the path integrated applications use through the C API. It requires that someone can read the Hub's current code (see the gap note below).

## Listing connected peers

The runtime tracks which peers are connected. The visualizer surfaces the connected peers so you can see who is currently in the session.

## Forgetting and blacklisting a device

Two related operations remove a device:

- **Forget** wipes the device's persisted identity slot (certificate and session state) so its next connection requires a fresh pairing. A device cannot be forgotten while its session is still active - have it leave first, then forget.
- **Blacklist (revoke)** blocks a device at the Hub even if it still holds a valid certificate. A blacklisted device can only return through a **fresh, human-approved pairing** - and completing that fresh pairing clears the blacklist entry for it.

## A current gap: reading this Hub's code

Be aware of a product gap today. The visualizer has a screen for connecting *this* Hub to a **remote** Hub (the hub-connect screen), but a dedicated screen that simply **displays this Hub's own current pairing code** was not found in the current build. The code-minting backend runs as described above (a new code roughly every 8 seconds, about 10 second TTL), and the current code is written to the **Hub log**.

So, in practice today: for push onboarding you do not need the code at all; for pull onboarding, read the current pairing code from the Hub log and relay it to the device owner within its short lifetime. An in-visualizer "show this Hub's code" screen is the natural next step here.
