---
title: Stream to a 3D Monitor
sidebar_position: 5
description: Add a receiver on the Hub PC, open its view, pick the stereo output that matches your 3D display (Schneider Digital 3D PluraView, passive 3D monitors, 3D TVs, Acer SpatialLabs, anaglyph), choose the displays and calibrate.
---

# Stream to a 3D Monitor

A **receiver** makes the Hub PC itself a target: it receives the mixed, stereo picture of the
session like a headset does. Its **view** then sends that picture to a 3D display in the format the
display expects.

**Source apps → mixer → receiver on this PC → view (output format) → 3D display**

## 1. Add a receiver

1. Start a source app so there is something to see (**Sources** › for example **StereoKit** ›
   **Launch**).
2. Open the **Stream Graph** tab. At its bottom is the **Receivers** strip.
3. Click **+ Add receiver**. A pill *Display Receiver 1* appears in the strip, a workspace tab of
   the same name opens, and the graph shows *1 of 1 targets are showing your apps* with the
   receiver as **TARGET** (*Showing*, its delay and frame rate).

   ![Stream Graph with a new receiver in the Receivers strip](../../img/visualizer/add-receiver.png)

4. Wait until the pill's dot turns green, then click the receiver's tab. It shows the streamed
   scene; the buttons above it switch colour, colour and depth or depth, and mono or stereo (both
   eyes side by side, as here).

   ![The receiver's view in stereo](../../img/visualizer/receiver-view.png)

On the pill: **⏸ / ▶** pauses and resumes the receiver, the **eye** reopens its view, the **trash**
removes it. Closing the view does not stop the receiver. Double-click the name to rename it.
**View** › **Hub Display** › **+ Add receiver** adds one too.

## 2. Open the view's output settings

1. Click **View** in the left rail. The **Hub Display** panel opens; its summary reads *1 receiver
   · 1 receiving · 1 view*.
2. Under **Views**, click the **Display Receiver 1** row. Its editor opens: **Stream**, **Output**,
   then the cards **Depth**, **Screen**, **Calibration** (Stereo monitors only), **Pop-out fix**,
   **Presets** and **Manual frustum**.

   ![Hub Display with the view editor open](../../img/visualizer/stereo-display-view.png)

3. Choose the **Output** for your display (next section).

## 3. Pick the stereo output for your display

![Output formats](../../img/visualizer/stereo-output-combo.png)

| Output | Picture sent to the display | For |
|---|---|---|
| **Texture grid window** | A window inside the Visualizer (default) | Checking the picture, no 3D |
| **Stereo monitors** | One full-screen window per eye, each on its own display. Has **Calibration** | Two-screen beam-splitter monitors, mirror rigs, two projectors with polarising filters |
| **Interlaced 3D monitor** | One full-screen window, left and right eye on alternating rows | Passive (polarised, FPR) 3D monitors |
| **Side-by-side 3D TV** | One full-screen window, left eye in the left half, right eye in the right half | 3D TVs and projectors in SBS mode, autostereoscopic displays that convert SBS |
| **Top / bottom 3D TV** | One full-screen window, left eye on top, right eye below | 3D TVs and projectors in TB mode |
| **Anaglyph glasses** | One full-screen window, colour-coded eyes. **Screen** › **Glasses**: **Red / cyan** or **Green / magenta** | Any monitor with coloured glasses |

### Common 3D displays

| Display | How it makes 3D | Output | Notes |
|---|---|---|---|
| **Schneider Digital 3D PluraView** (up to 28", FHD to 4K per eye) | Two stacked screens and a beam-splitter mirror, passive polarised glasses. One screen per eye | **Stereo monitors** | Windows sees two displays: put **Left eye** on one and **Right eye** on the other, then [calibrate](#5-calibrate-a-two-screen-rig). Each eye keeps full resolution |
| Two projectors with polarising filters, mirror rigs | One projector or screen per eye | **Stereo monitors** | Calibrate so the two pictures overlap exactly |
| **Passive 3D monitors** (film-patterned retarder, e.g. LG passive 3D, Zalman, Miracube) | Odd and even rows polarised for opposite eyes | **Interlaced 3D monitor** | Run the monitor at its native resolution and the window full screen, or the rows do not line up. Each eye gets half the vertical resolution |
| **3D TVs and 3D projectors** | Frame-packed by the TV from a side-by-side or top/bottom picture | **Side-by-side 3D TV** or **Top / bottom 3D TV** | Switch the TV to the same 3D mode (SBS or TB) with its remote |
| **Acer SpatialLabs View / View Pro** (glasses-free, eye-tracked) | Lenticular lens with eye tracking | **Side-by-side 3D TV** | Acer's **SpatialLabs Go** turns a full-screen side-by-side window into 3D. Not yet verified with QAROS |
| **ASUS Spatial Vision** laptops (Vivobook Pro 16X 3D OLED, ProArt Studiobook 16 3D OLED) | Lenticular OLED with eye tracking | — | ASUS documents its own apps and OpenXR, no converter for a full-screen side-by-side window. Not supported by QAROS yet |
| Any monitor | Red/cyan or green/magenta glasses | **Anaglyph glasses** | Colours are distorted. Match **Glasses** to the glasses you hand out, or the image ghosts |

Vendor names are trademarks of their owners. Check the display's manual for the input formats it
accepts.

## 4. Choose the displays

After you pick a full-screen output, the view shows **Show output windows** and the display
choice:

| Output | Display choice |
|---|---|
| **Interlaced**, **Side-by-side**, **Top / bottom**, **Anaglyph** | **Display**: one button per display (`Display 1`, `Display 2`, …) |
| **Stereo monitors** | **Left eye** and **Right eye**, one button per display each, plus **Swap left / right** to trade them |

1. Connect the 3D display and set Windows to **Extend these displays**.
2. Click its button under **Display** (or under **Left eye** and **Right eye**). With **Show output
   windows** ticked, a borderless full-screen window opens on it.

   ![Side-by-side 3D TV output on Display 1](../../img/visualizer/stereo-display-choice.png)

   ![Stereo monitors: one display per eye](../../img/visualizer/stereo-monitors.png)

3. If depth looks inverted, tick **Screen** › **Swap eyes** (in **Stereo monitors**, use **Swap left
   / right** instead).
4. **Screen** › **Show calibration pattern** replaces the video with a test pattern, useful to set
   up the display; untick it to get the video back.

To move the picture to another display, click another display button. To close the output, untick **Show
output windows** (the view keeps its settings), or set **Output** back to **Texture grid
window**.

- **Two views on the same display:** a *Monitor conflict* asks **Allow** or **Keep Closed**.
- **Display unplugged:** the view says the saved display is absent. Pick an available one.
- **Only one monitor:** the full-screen window covers the Visualizer. Use Alt+Tab to get back and
  untick **Show output windows**.

## 5. Calibrate a two-screen rig

Only **Stereo monitors** has a **Calibration** card: it shifts and scales each eye's picture so the
two screens overlap exactly, as a beam-splitter or projector rig needs. It lists the shifts of
both eyes together and of each eye (**Both X/Y**, **Left X/Y**, **Right X/Y**, in pixels) and each
eye's scale.

![Calibration card](../../img/visualizer/calibration-card.png)

1. In **Calibration**, click **Start wizard**. Both eye windows open full screen with the
   calibration pattern, and the *Stereo Calibration Wizard* opens over the Visualizer. While it is
   open, the card offers **Focus wizard** (bring it back to the front) and **Stop** (close it,
   keeping what it set).

   ![Stereo calibration wizard](../../img/visualizer/stereo-calibration-wizard.png)

2. Walk the seven steps with **Next** (the **Step** list jumps back to any step, values are kept):

   | Step | Set |
   |---|---|
   | 1. Screen Setup | The real **Resolution** and **Screen Size**, **Left Screen** / **Right Screen** |
   | 2. Left Scale X/Y | Size of the left picture |
   | 3. Right Scale X/Y | Size of the right picture |
   | 4. Left shift | Position of the left picture |
   | 5. Right shift | Position of the right picture |
   | 6. Global shift | Both pictures together, until the patterns overlap |
   | 7. Save | A **Preset Name** (for example `Rig A`), then **Finish Calibration** |

3. The patterns disappear and the aligned video returns. **Presets** shows the preset's name, and
   **Calibration** shows the shifts (px) and scales the wizard set.

- **Cancel** closes the wizard but keeps the values set so far.
- **Calibration** › **Reset** returns to no shift and no scale; **Presets** › **Reset** brings the
  saved preset back.
- A preset survives restarts, so a fixed rig is calibrated once.
