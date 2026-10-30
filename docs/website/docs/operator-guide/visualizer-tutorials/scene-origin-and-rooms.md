---
title: Scene Origin - Anchor Several Devices Together
sidebar_position: 6
description: Show or print a room's scene origin code, anchor HoloLens 2, Meta Quest and METALENSE 2 headsets to it so they share one scene, and line up several physical rooms with room offsets.
---

# Scene Origin: Anchor Several Devices Together

Every headset tracks its own space. To make several headsets in one physical room see the streamed
scene in **the same place**, each one anchors the scene at the same printed or on-screen **scene
origin code**: a 10 cm QR code made by the Visualizer. The code's position becomes the scene's
origin for every device that sees it.

- The scene is always **level** (up is gravity up), whatever surface the code is on. The code gives
  only the position and the facing direction.
- A **room** is one physical space. Devices in different physical places go in different rooms, and
  each room's **offset** says where it sits in the shared scene.

## Which devices anchor to the code

| Device | Scans the code | What you do | Feedback on the device |
|---|---|---|---|
| **HoloLens 2** | Yes | Look at the code with the player running | *Headset is not anchored! Look at QR code.* until it is anchored |
| **METALENSE 2** | Yes | Look at the code with the player running | A green outline on the code when it is picked up |
| **Meta Quest** | Yes, on a recent Horizon OS | Allow the player's **spatial data** permission on its first start, then look at the code with the player running | A green outline on the code when it is picked up |

- Every player anchors only to a QAROS scene origin code. Other QR codes in view (posters, badges,
  the Visualizer's pairing code) are ignored.
- **Meta Quest** reads QR codes through Meta's QR detection, which Meta still calls experimental.
  A headset on an older Horizon OS does not detect codes at all; update it. Denying the spatial data
  permission has the same effect: allow it again for the QAROS Player in the headset's app
  permission settings.

## 1. Create a room for each physical space

Skip this with a single physical space: every device starts in **Room 0 · Default**, and its
code works as is.

1. **Session** › **Rooms** › **+ New room**.
2. Type a **Name** (for example `Lab B`) and click **Create**. The room gets the next free number,
   which is written into its origin code.

   ![New room form](../../img/visualizer/rooms-new.png)

Each room card shows its number, name, device count, **Offset** and **Devices**:

![Rooms panel with Room 0 and Lab B](../../img/visualizer/rooms-panel.png)

## 2. Show or print the room's origin code

1. Click the **QR icon** on the room's card, or open **Tools** › **Origin** and pick the **Room**.

   ![Tools › Origin with Room 1 · Lab B selected](../../img/visualizer/origin-card.png)

2. Choose how to put the code in the room:

   | | **Print...** | **Show** |
   |---|---|---|
   | Use for | A fixed installation, the most stable choice | A quick session, or no printer at hand |
   | Size | Printed at **Actual size · 100 %**, with a 10 cm check ruler and cut marks. **Save as PDF...** to print elsewhere | Drawn on this monitor. Tick **Show at true size** for exactly 10 cm |
   | Check | The ruler on the sheet and the code's black area both measure 10 cm | The black area measures 10 cm on the screen |

   ![Print scene origin marker](../../img/visualizer/origin-print.png)

   ![Scene origin marker shown on screen](../../img/visualizer/origin-show.png)

3. **Showing at true size on a monitor that does not report its size:** the dialog says *Unknown
   screen size* or *Not at true size*. Click **Calibrate...**, measure the white bar with a ruler,
   type its length in **Bar length (mm)** and click **Save calibration**. The calibration is kept
   per monitor.

   ![Calibrating the screen for a true-size code](../../img/visualizer/origin-true-size.png)

The black area must measure **10 cm**: a code at another size puts the origin in the wrong place.
Keep the white border around it.

## 3. Place the code

- Lay a printed code **flat** on a table or the floor, fixed with tape, where the scene should
  start. The scene faces the code's **bottom edge** (on the printed sheet, the edge toward the
  ruler).
- A code on a **wall** or an upright monitor works too: the scene stands level in front of it,
  facing out of the wall.
- Avoid tilting it near 45°: it can flip between the two facing rules from one scan to the next.
- Leave it where it is for the whole session. Moving it moves the scene for every device that
  sees it again.

## 4. Anchor each device

For every headset in the room, one after another:

1. Start the player and invite (HoloLens) or pair (Meta Quest, METALENSE 2) the device ([HoloLens 2](./hololens-2.md),
   [Meta Quest and METALENSE 2](./quest-and-metalense-2.md)).
2. Put it on and **look at the code** until the device confirms
   (HoloLens: the *not anchored* message goes; Meta Quest and METALENSE 2: the green outline).
3. Look around the room for a few seconds before taking it off, so the device can save its anchor.

All devices that scanned the same code now share the same origin: content placed on the table is on
the table for everyone.

- **Saved per device.** The anchor survives a restart of the player; a device normally scans once
  per location.
- **Scan again** when the device was moved to another place, the code was moved, the scene looks
  shifted, or tracking was lost and the device cannot find its space again (HoloLens: *Can not
  locate anchor after loss of tracking!*). Walking back toward the code usually recovers it.
- Reinstalling a player signed with another key deletes its saved anchor: scan again.

## 5. Put each device in its room

The room a device is in decides which room offset applies to it.

1. **Session** › **Rooms**. Each room card lists its **Devices**.
2. Check that each device is under the room it physically stands in. To move one, pick the room in
   the list on its row, or drag the row onto another room card.

## 6. Line up several physical rooms

Two physical spaces (two labs, two buildings) each have their own code and their own room. Without
an offset both rooms' origins land on the same point of the shared scene.

1. **Session** › **Rooms**, unfold the second room's card.
2. Under **Offset**, set **Position (m)** (X, Y, Z) and **Yaw** / **Pitch** / **Roll**: where this
   room's origin sits in Room 0's space. It moves the room's devices' view, hands and the content
   placed for them; the rest of the session does not move.
3. **⋯** › **Reset offset** puts it back to zero.

Closing a room moves its devices back to Room 0, and its origin codes stop working until a room
with that number exists again.
