---
title: "Tutorial: Stream a Turbine from Blender"
sidebar_label: "Tutorial: Blender Turbine"
sidebar_position: 5
description: Install the QAROS add-on from the Visualizer, import a turbofan engine GLB into Blender, stream it to a headset, edit it live and choose the renderer.
---

# Tutorial: Stream a Turbine from Blender

Import a turbofan engine into Blender and stream it to a headset with the **QAROS** tab. About 15 minutes.

![A turbofan engine streamed from Blender, seen in a Display Receiver, with the Blender controls panel beside its app volume](../../img/supported-apps/blender-tutorial/receiver-turbine.webp)

You will:

- install Blender and the QAROS add-on (from the Visualizer, or by hand);
- import the turbine from a GLB file and place it;
- stream it and view it in a headset or a Display Receiver;
- edit the selected part live;
- choose the renderer and see what viewport shading does.

The screenshots show add-on **0.4.1** in Blender 4.5 LTS. For the add-on in general, see [Blender](./blender.md).

## Before you start

| | Needed |
|---|---|
| QAROS | installed and running on this PC, with the **ANARI device** component (default from 0.10.0) |
| Blender | **4.5 LTS or newer**, on the same PC (Windows). Older versions are not supported |
| GPU | OpenGL 4.3 for the VisGL renderer (any recent NVIDIA, AMD or Intel GPU) |
| Viewer | a headset paired with this QAROS ([Pair a Headset](../../operator-guide/visualizer-tutorials/pair-a-headset.md)), or a Display Receiver ([Stream to a 3D Monitor](../../operator-guide/visualizer-tutorials/stream-to-a-3d-monitor.md)) |

## The model

**[Download turbofan.glb](https://github.com/Quaternar/qaros-blender/raw/v0.4.1/demo/assets/turbofan.glb)** (11 MB, glTF binary).

- Work: "Turbine | Turbofan Engine | Jet Engine" by [blenderbirb](https://sketchfab.com/blenderbirb).
- Licence: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Credit the author when you show or share it.
- Source: [Sketchfab](https://sketchfab.com/3d-models/turbine-turbofan-engine-jet-engine-74c6aceed86b4a41aaad3b93afc3e262). The download above is the unchanged model.
- Contents: 19 meshes, about 236,000 triangles, 6 materials with constant colours and no textures. Everything in it streams.

## Step 1: Install Blender

- Download an LTS release from [blender.org/download/lts](https://www.blender.org/download/lts/): **[Blender 4.5 LTS](https://www.blender.org/download/lts/4-5/)** is the oldest version the add-on supports.
- Newer Blender releases work too (checked with 5.2). Get them from [blender.org/download](https://www.blender.org/download/).
- Installer, zip or Steam: all work. Install it on the QAROS PC.

## Step 2: Install the add-on from the Visualizer

The Visualizer finds every Blender on the PC and installs the add-on into it.

1. Start **QAROS** from the Start menu. The Visualizer opens.
2. Click **Sources** on the left rail. Type `Blender` in the search field.
   - One row per Blender version, for example **Blender 4.5**.
   - **Detected:** QAROS found it. **Installed** / **Needs update:** the add-on's state.

   ![Sources filtered to Blender: Blender 4.5 needs an update, Blender 5.2 has the add-on installed](../../img/supported-apps/blender-tutorial/sources-blender-tiles.webp)

3. Click the row of your Blender. The **Detected app** dialog opens.
4. Under **QAROS add-on**, click **Install QAROS add-on** (or **Update QAROS add-on**).
   - QAROS downloads the [latest release](https://github.com/Quaternar/qaros-blender/releases/latest), checks it, copies it into that Blender's add-ons folder and enables it.
   - Nothing else in Blender changes.

   ![The Detected app dialog for Blender 4.5: the QAROS add-on section offers Update QAROS add-on](../../img/supported-apps/blender-tutorial/sources-blender-tile-dialog.webp)

5. The section then shows **Installed** and the version.

   ![QAROS add-on: Installed, Version 0.4.1, with Launch](../../img/supported-apps/blender-tutorial/sources-addon-installed.webp)

6. Click **Launch**. Blender starts and streams its scene by itself.
   - First time only: QAROS asks **"Allow Blender to join QAROS?"**. Click **Allow** within 120 s.
   - **Running** shows Blender as **Connected**. **Stop** quits that Blender (it saves `quit.blend` first: **File > Recover > Last Session**).

   ![Running: Blender 4.5, QAROS API app, Connected, with a Stop button](../../img/supported-apps/blender-tutorial/sources-blender-running.webp)

**Blender not in the list?**

- Press **Locate...** on the greyed-out **Blender** row, or **Add Blender folder...**, and pick the folder with `blender.exe`. It then gets its own row with **Install QAROS add-on** ([Locating an app QAROS did not find](../../operator-guide/source-applications.md#locating-an-app-qaros-did-not-find)).
- Or add it as your own preset: **+** (beside the search field) > **Open QAROS supported app**, set **Executable** to `blender.exe`, **Add app**. QAROS does not install the add-on into such a preset: install it by hand (below), and click **Start streaming** in Blender after **Launch**.

![Add source app: Add OpenXR app, Web app, Import app package, Open QAROS supported app](../../img/supported-apps/blender-tutorial/sources-add-source-app.webp)

![Open QAROS supported app: Executable with Browse, Name, Add app](../../img/supported-apps/blender-tutorial/sources-open-supported-app.webp)

### Alternative: install by hand

1. Download `qaros_blender_anari-<version>.zip` from the [latest release](https://github.com/Quaternar/qaros-blender/releases/latest).
2. In Blender: **Edit > Preferences > Add-ons > Install from Disk**, pick the zip.
3. Enable **QAROS ANARI Streaming**.
4. Leave **ANARI folder** in its preferences empty.

The Visualizer leaves a hand-installed add-on alone: the row says so.

## The QAROS tab

The add-on puts a **QAROS** tab in the sidebar of the 3D Viewport. Open it with `N`, or **View > Sidebar**, then click **QAROS** on the sidebar's edge.

![Blender with the turbine imported, the QAROS tab open in the sidebar, the selected part's material on the right](../../img/supported-apps/blender-tutorial/blender-overview.webp)

| Stopped | Streaming |
|---|---|
| ![QAROS tab while stopped: Off, App volume, Streams to All devices, Start streaming](../../img/supported-apps/blender-tutorial/qaros-tab-stopped.webp) | ![QAROS tab while streaming: Streaming to 1 user, Backend visgl, 19 objects, Fit scene to volume, Streams to, Stop streaming](../../img/supported-apps/blender-tutorial/qaros-tab-streaming.webp) |

| Item | What it shows or does |
|---|---|
| status line | **Off**, **Joining QAROS**, **Streaming to N users**, or why it stopped |
| **App volume** | the app volume the scene streams into. Empty: **Blender** (**Blender 2** for a second Blender). Change it only while stopped |
| **Backend:** | the renderer in use (step 8) |
| objects, triangles | what is being streamed |
| material lines | what the renderer can show of your materials |
| **Controls panel beside the app volume** | the headset shows a **Blender** panel for the selected object |
| **Fit scene to volume** | scales and places the scene to fill its app volume |
| **Streams to** | **All devices**, or a checkbox per headset or receiver; the dot marks one receiving now |
| **Start streaming** / **Stop streaming** | joins the QAROS running on this PC and streams, or stops; Blender stays open |

- Red lines name what is not streamed, and why.
- Newer add-on versions add a **Navigation** / **Object selection** choice for hand gestures.

## Step 3: Import the turbine

1. In Blender's default scene, select the **Cube** and press `X` > **Delete**.
2. **File > Import > glTF 2.0 (.glb/.gltf)**.
3. Pick `turbofan.glb`, click **Import glTF 2.0**. The default options are fine.

   ![Blender's glTF import file browser with turbofan.glb selected and Import glTF 2.0](../../img/supported-apps/blender-tutorial/import-gltf-dialog.webp)

4. The engine appears at the world origin, about 4 m long, along the X axis.

The import makes one empty, **Sketchfab_model**, with the engine parts as its children (**Object_2**, **Object_3**, ...).

## Step 4: Scale and place it

The world origin sits at the centre of the app volume. One Blender unit is 0.25 m in the room by default, so the 4 m engine is 1 m long there.

- **Move it:** select **Sketchfab_model** in the Outliner, press `G`, or type **Location** in the sidebar's **Item** tab. Keep it near the origin.
- **Scale it:** press `S`, or set **Scale** on **Sketchfab_model**. All parts follow.
- **Turn it:** press `R`, for example `R` `Z` `90` to face it along Y.
- Skip exact sizing: **Fit scene to volume** (step 6) fits whatever you have.

Objects hidden in the viewport (eye icon in the Outliner) are not streamed.

## Step 5: Select a part and set its look

The stream follows Blender's selection and the selected object's own settings. There are no separate QAROS settings per object.

1. Click a part in the viewport or the Outliner, for example the yellow rings (**Object_5**).
2. **Object Properties** (orange square): **Location**, **Rotation**, **Scale** stream as you change them.
3. **Material Properties** (red sphere): edit the **Principled BSDF**.

   ![Material Properties of the selected part: accent-yellow, Principled BSDF with Base Color, Metallic, Roughness, IOR, Alpha](../../img/supported-apps/blender-tutorial/selected-object-material.webp)

   - Streamed: **Base Color**, **Metallic**, **Roughness**, **Alpha**, **Emission Color** and **Strength**.
   - Use constant values. An input linked to a texture or node streams the material's viewport colour.
   - The turbine's materials are shared by several parts: a change recolours all of them.

## Step 6: Stream it

If you launched Blender from the Visualizer, it already streams: skip to point 6.

1. Make sure QAROS is running (Start menu > **QAROS**).
2. Open the **QAROS** tab (`N`).
3. Optional: type an **App volume** name, for example `Turbine`.
4. Click **Start streaming**.
5. First time only: click **Allow** on **"Allow Blender to join QAROS?"** within 120 s.
6. The status reads **Streaming to 1 user** once a device watches.
7. Click **Fit scene to volume** to fill the app volume with the engine.

To pick devices, untick **All devices** under **Streams to** and tick the ones you want. The change applies at once.

## Step 7: View it in the headset

1. Put on the paired headset, or open a Display Receiver. The engine sits in its app volume, with a **Blender** panel beside it.
2. Walk around it and look through the open casing.
3. Interact:

   | Do | Result |
   |---|---|
   | Click a part | selects it in Blender too |
   | One-hand drag that starts on the selected part | pulls that part out of the engine, in Blender too |
   | One-hand drag anywhere else | moves your view; Blender's scene stays unchanged |
   | Two hands apart or together | zooms the view |
   | **Object** tab on the panel | edits the selected part's location, rotation, scale |
   | **Material** tab on the panel | edits base colour, roughness, metallic |
   | **Undo** / **Redo** on the panel | steps back through edits |

4. Edit in Blender at the same time: every change shows in the room within a moment.

Newer add-on versions select with a double tap in **Navigation**, and move parts only with **Object selection** (QAROS tab, or the panel's **Tools** tab).

## Step 8: Shading and renderer

The headset picture does not come from Blender's viewport. An ANARI renderer (the **backend**) inside Blender draws it.

**Viewport shading** (the four spheres at the top right of the viewport) does not change the stream:

| Wireframe | Material Preview |
|---|---|
| ![Viewport in Wireframe shading while the QAROS tab streams](../../img/supported-apps/blender-tutorial/shading-wireframe.webp) | ![Viewport in Material Preview shading while the QAROS tab streams](../../img/supported-apps/blender-tutorial/shading-material-preview.webp) |

- Use any mode while you work: Wireframe, Solid, Material Preview or Rendered.
- The stream always shows the materials as the backend renders them, with your scene's lamps, plus **Ambient light**.
- Metal reflects only lamps, not Material Preview's studio light: add a lamp or two for highlights.

**The renderer** is the add-on's **Backend** preference: **Edit > Preferences > Add-ons > QAROS ANARI Streaming**.

![The add-on's preferences: ANARI folder empty, Backend VisGL, Users, Frame rate, Render scale, Scale, Near, Far, Ambient light, VisRTX materials](../../img/supported-apps/blender-tutorial/preferences-backend.webp)

| Backend | Runs on | Look | Use it for |
|---|---|---|---|
| **Automatic** (default) | the first that starts: VisGL, VisRTX, OSPRay, Helide | as that backend | most PCs |
| **VisGL** | GPU, OpenGL 4.3 | lit, shadows, metallic, roughness, emission; no selection outline | headsets: fastest |
| **VisRTX** | NVIDIA RTX GPU | ray traced; grain in a moving view, or smooth with **VisRTX materials** Matte | comparing with a ray tracer |
| **OSPRay** | CPU | flat shading, hard shadows; colour only | PCs without a usable GPU |
| **Helide** | CPU | simple reference shading | checking a result |

1. Choose a **Backend**.
2. Quit and restart Blender. The backend is loaded once per Blender session.
3. **Start streaming** again. The **Backend:** line shows the one in use.
4. If the tab says **Restart Blender to use ...**, the restart is still pending.

The selected part is outlined in orange in the headset on VisRTX, OSPRay and Helide. VisGL draws no outline.

## Step 9: Stop

- **Stop streaming** in the QAROS tab. The app volume closes; Blender stays open.
- Or **Stop** on Blender's card under **Running** in the Visualizer: that quits Blender.
- **File > Save** keeps the scene, its **App volume** name and its **Streams to** choice.

## If something is off

| Problem | Fix |
|---|---|
| Status stays at **Joining QAROS** | Click **Allow** in the QAROS prompt (tray, or the Visualizer's **Sources**). |
| No **Install QAROS add-on** on the row | The add-on was installed by hand: update it by hand, or remove it from Blender and let QAROS install it. |
| Nothing in the headset | Check **Streams to**, and that the headset is paired. |
| Engine tiny, huge or cut off | **Fit scene to volume**. |
| Slow, all CPU cores busy | The backend is OSPRay or Helide: choose **VisGL** and restart Blender. |
| A colour change does nothing | Edit the **Principled BSDF** that feeds **Material Output** with a constant value. |

More: [Blender troubleshooting](./blender.md#troubleshooting).
