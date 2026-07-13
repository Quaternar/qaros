---
title: The Shared Space
sidebar_position: 5
description: A plain-language tour of the QAROS Shared Space — how everyone occupies the same space and sees the same layout from their own viewpoint.
---

# The Shared Space

When you join QAROS, you step into a **Shared Space**. Think of it like a shared collaborative environment: everyone is in the same space at the same time, standing in their own spot, looking in their own direction, but seeing the same content together.

## The room inside the Shared Space

The room is the physical part of the Shared Space. It has a fixed sense of where "here" is, so that when someone places something in the middle of the table, it is in the middle of the table for everyone in that room. You move through it by moving your own body or device; the room itself stays put.

Everyone sees the Shared Space from their own viewpoint. Just as two people across a real table see the same coffee cup from different angles, two QAROS participants see the same content from wherever they happen to be standing.

## App volumes

An **app volume** is a box in the Shared Space that an application draws into. Picture a glass display case: the application's content lives inside the case, and the case sits somewhere in the space.

- Each running application gets its own volume, so several applications can share the same Shared Space without stepping on each other.
- A volume can be **moved, rotated, and resized** — you can shrink a whole building down to fit on a tabletop, or push a model to the far wall.
- Whatever is inside the case is the same for everyone; if you move the case, it moves for everyone.

## GUI panels

A **GUI panel** is a flat, 2-D screen floating in the Shared Space — a settings page, a dashboard, a control surface, or a web page. If an app volume is a display case, a panel is a poster or a tablet hanging in the air.

Panels are shared too: everyone generally sees the same panel in the same place. Some panels can be shown only to certain people — for example, a control panel meant just for the operator.

## Why everyone agrees

The Hub keeps one authoritative picture of the Shared Space — where each volume and panel sits, and how big each one is. Your device receives that picture and renders it from your point of view. Because the layout comes from a single shared source, nobody drifts into a slightly different version of the same space.

## Why your view stays smooth

Your head moves constantly, even when you think you're holding still. To keep the picture glued to the world instead of lagging behind, QAROS gently re-aims each frame to match where you're actually looking at the last possible moment — a technique called asynchronous warping. The result is that content stays anchored in place and feels stable as you move, rather than swimming or juddering.

## Going deeper

This page is the plain-language version. If you want the exact model — how positions, orientations, and scales are defined precisely — see the Developer Guide's [Coordinate Systems](/docs/developer-guide/coordinate-systems). For how the shared view is actually assembled and delivered to each person, see the operator-side [The Mixer](/docs/operator-guide/the-mixer), which explains how it works under the hood.
