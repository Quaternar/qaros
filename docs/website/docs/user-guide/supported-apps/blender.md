---
title: Blender
sidebar_position: 4
description: Stream a Blender scene into QAROS with the QAROS ANARI Streaming add-on. A quick start, then step by step - set Blender up (found by the Visualizer, added in Sources, or installed by hand), import a turbofan engine GLB, stream it to a headset, edit it live, follow the viewport shading and choose the renderer - and what makes a scene stream well, performance tips and troubleshooting.
---

# Blender

Your open Blender scene, live in the room: edits in Blender show up within a moment, and a **Blender** panel beside the app volume edits the selected object from the headset. This page walks through it with a turbofan engine: import it into Blender and stream it with the **QAROS** tab. About 15 minutes.

![A turbofan engine streamed from Blender, seen in a Display Receiver, with the Blender controls panel beside its app volume](../../img/supported-apps/blender-tutorial/receiver-turbine.webp)

You will:

- set up Blender with the QAROS add-on, one of three ways;
- import the turbine from a GLB file and place it;
- stream it and view it in a headset or a Display Receiver;
- edit the selected part live, and leave a part out of the stream;
- see the stream follow Blender's viewport shading, and choose the renderer.

The steps match add-on **0.6.4**. The screenshots show 0.6.2 in Blender 5.2; the **QAROS** tab of 0.6.4 differs only in the selection status of Step 6.

## Quick start

Already know Blender and QAROS? Five steps:

1. Start QAROS and add a **Display Receiver**, or start the player on your paired headset ([Step 1](#step-1-start-qaros-and-add-a-receiver)).
2. In the Visualizer, **Sources** > your Blender's row > **Install QAROS add-on** > **Launch** ([Step 3](#way-1-found-by-the-visualizer)). Approve Blender under **Waiting for approval** the first time.
3. Open or import your scene in that Blender.
4. In the 3D Viewport, press `N`, open the **QAROS** tab and click **Fit Scene to Volume** (**View** panel).
5. Look at it in the headset or the Display Receiver. Edit in Blender; the stream follows within a moment.

The rest of this page does the same step by step with a turbine, and then covers [what streams well](#what-streams-well), [performance](#performance) and [troubleshooting](#if-something-is-off).

## Before you start

| | Needed |
|---|---|
| **QAROS ANARI Streaming** add-on | 0.6.4, the latest (step 3 installs it) |
| QAROS | installed and running on this PC, with the **ANARI device** component (default from 0.10.0). 0.9.0 or newer for the add-on; the Visualizer installs it from 0.10.0. Metal reflecting its surroundings ([Step 9](#step-9-shading-and-renderer)) needs QAROS 0.10.0 or newer |
| Blender | **4.5 LTS to 5.2**, on the same PC (Windows). Older versions and Blender 5.3 are not supported yet |
| GPU | OpenGL 4.3 for the VisGL renderer (any recent NVIDIA, AMD or Intel GPU). OSPRay and Helide run on the CPU; VisRTX needs an NVIDIA RTX GPU and is optional |
| Viewer | a Display Receiver in the Visualizer, or a headset paired with this QAROS ([Before You Start](./start-qaros-and-add-a-receiver.md)) |

```mermaid
flowchart LR
  blender["Blender scene"] --> addon["QAROS ANARI<br/>Streaming add-on"]
  addon --> anari["ANARI renderer<br/>(VisGL, OSPRay, Helide, VisRTX)"]
  anari --> runtime["QAROS runtime"]
  runtime --> hub["QAROS Hub"]
  hub --> headset["Headset or<br/>Display Receiver"]
```

The picture in the headset is not Blender's viewport: an ANARI renderer inside Blender draws it ([Step 9](#step-9-shading-and-renderer)).

## The model

**[Download turbofan.glb](https://github.com/Quaternar/qaros-blender/raw/v0.6.4/demo/assets/turbofan.glb)** (11 MB, glTF binary).

- Work: "Turbine | Turbofan Engine | Jet Engine" by [blenderbirb](https://sketchfab.com/blenderbirb).
- Licence: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Credit the author when you show or share it.
- Source: [Sketchfab](https://sketchfab.com/3d-models/turbine-turbofan-engine-jet-engine-74c6aceed86b4a41aaad3b93afc3e262). The download above is the unchanged model.
- Contents: 19 meshes, about 236,000 triangles, 6 materials with constant colours and no textures. Everything in it streams.

## Step 1: Start QAROS and add a receiver

Follow [Before You Start: Start QAROS and Add a Receiver](./start-qaros-and-add-a-receiver.md): start QAROS, open the Visualizer, and add a **Display Receiver** (**+ Add receiver** at the bottom of the **Stream Graph**), or start the player app on your paired headset. Without one, Blender streams to nobody and you see nothing.

## Step 2: Install Blender

- Download an LTS release from [blender.org/download/lts](https://www.blender.org/download/lts/): **[Blender 4.5 LTS](https://www.blender.org/download/lts/4-5/)** is the oldest version the add-on supports.
- Newer Blender releases up to 5.2 work too. Get them from [blender.org/download](https://www.blender.org/download/). Blender 5.3 is not supported yet: the add-on does not start there, and the Visualizer's Blender tile says so instead of offering it.
- Installer, zip or Steam: all work. Install it on the QAROS PC.

## Step 3: Set up Blender with QAROS

Three ways, in this order. Start with the first; if it does not find your Blender, take the second; the third always works.

| Way | You do | QAROS does |
|---|---|---|
| [1. Found by the Visualizer](#way-1-found-by-the-visualizer) | press **Install QAROS add-on**, then **Launch** | finds Blender, installs and enables the add-on, starts Blender streaming |
| [2. Added in Sources](#way-2-add-blender-in-sources) | point QAROS at `blender.exe` once | the same as way 1, for a Blender it did not find |
| [3. By hand](#way-3-by-hand) | install the add-on in Blender, start QAROS, press **Start Streaming** | nothing until you press it |

### Way 1: Found by the Visualizer

The Visualizer finds every Blender on the PC and installs the add-on into it. It looks in `Program Files`, your user's `Programs` folder, the Windows list of installed programs, and Steam.

1. Start **QAROS** from the Start menu. The Visualizer opens.
2. Click **Sources** on the left rail. Type `Blender` in the search field.
   - One row per Blender version, for example **Blender 5.2**.
   - **Detected:** QAROS found it. **Installable** / **Installed** / **Needs update:** the add-on's state in that Blender.
   - **Not available:** a Blender the add-on does not support yet, such as 5.3. Its row cannot launch.

   ![Sources filtered to Blender: Blender 5.2 Detected with the add-on Installable, Blender 5.3 Not available](../../img/supported-apps/blender-tutorial/sources-blender-tiles.webp)

3. Click the row of your Blender. The **Detected app** dialog opens.
4. At the top, under **QAROS add-on**, click **Install QAROS add-on** (or **Update QAROS add-on**).
   - QAROS downloads the latest add-on, checks it, copies it into that Blender's add-ons folder and enables it.
   - Nothing else in Blender changes.

   ![The Detected app dialog for Blender 5.2: QAROS add-on Installable with the latest version, Install QAROS add-on, then the preset, and Launch at the bottom](../../img/supported-apps/blender-tutorial/sources-blender-tile-dialog.webp)

5. The section then shows **Installed** and the version.

   ![The Detected app dialog after the install: QAROS add-on Installed with its version, Launch at the bottom](../../img/supported-apps/blender-tutorial/sources-addon-installed.webp)

6. Click **Launch** at the bottom of the dialog. Blender starts and streams its scene by itself.
   - First time only: QAROS asks whether Blender may join. In **Sources**, under **Waiting for approval**, click **Approve** within 120 s ([Approved local apps](../../operator-guide/source-applications.md#approved-local-apps)).
   - **Running** shows Blender as **Connected**. **Stop** quits that Blender (it saves `quit.blend` first: **File > Recover > Last Session**).

   ![Running: Blender 5.2, QAROS API app, Connected, with a Stop button](../../img/supported-apps/blender-tutorial/sources-blender-running.webp)

No row for your Blender, or a greyed-out **Blender** row? Take way 2.

### Way 2: Add Blender in Sources

Point QAROS at the Blender it did not find. It then gets its own row, as in way 1.

1. In **Sources**, click **+** beside the search field. **Add source app** opens.

   ![Add source app: Add OpenXR app, Web app, Import app package, Open QAROS supported app](../../img/supported-apps/blender-tutorial/sources-add-source-app.webp)

2. Click **Open QAROS supported app**.
3. **Executable:** click **Browse** and pick `blender.exe` in your Blender folder, for example `C:\Program Files\Blender Foundation\Blender 4.5\blender.exe` or the folder you unzipped a blender.org download into. Leave **Name** empty.
   - The dialog confirms: *Add app gives this Blender its own tile, with the QAROS add-on.*
   - *This Blender has its tile already* means way 1 found it: use its row instead.

   ![Open QAROS supported app: Executable set to an unzipped Blender's blender.exe, Name empty, the note that it gets its own tile, Add app](../../img/supported-apps/blender-tutorial/sources-open-supported-app.webp)

4. Click **Add app**. Your Blender gets its own row, for example **Blender 4.5**.
5. Continue with step 3 of way 1: click the row, **Install QAROS add-on**, **Launch**.

**Locate...** does the same from the Blender row: press it on the greyed-out **Blender** row (or **Locate another folder...** in a Blender row's **...** menu) and pick the folder with `blender.exe` ([Locating an app QAROS did not find](../../operator-guide/source-applications.md#locating-an-app-qaros-did-not-find)).

**Why not a plain app?** A Blender row installs the add-on into that Blender for you, keeps it up to date, and starts Blender with the add-on loaded and streaming by itself. A plain app entry that only names `blender.exe` starts Blender without any of that: the **QAROS** tab is missing unless you enabled the add-on yourself, and nothing streams until you press **Start Streaming**.

:::note
Way 2 needs QAROS 0.10.0 or newer. An older QAROS adds a plain app from **Open QAROS supported app**, which starts Blender without the add-on: remove it (its **...** menu) and use **Locate...**.
:::

### Way 3: By hand

Always works, also when QAROS cannot reach the internet. Use it to troubleshoot.

1. **[Download the QAROS Blender add-on](https://github.com/Quaternar/qaros-blender/releases/download/v0.6.4/qaros_blender_anari-0.6.4.zip)** (`qaros_blender_anari-0.6.4.zip`), or the newest from the [releases page](https://github.com/Quaternar/qaros-blender/releases/latest).
2. In Blender: **Edit > Preferences > Add-ons**, then **Install from Disk** (in the menu at the top right), and pick the zip.
3. Enable **QAROS ANARI Streaming**. Leave **ANARI folder** in its preferences empty.
4. Start **QAROS** from the Start menu (or run `qaros.exe` in the QAROS `bin` folder).
5. In Blender's 3D Viewport, press `N` and click the **QAROS** tab, then **Start Streaming** ([Step 7](#step-7-stream-it)).

The Visualizer leaves a hand-installed add-on alone: its row says so. To let QAROS manage it again, remove your copy from Blender and use way 1.

### Several Blender versions

- Each Blender version QAROS finds gets its own row, for example **Blender 4.5** and **Blender 5.2** side by side.
- Add another folder with **Locate another folder...** in a Blender row's **...** menu.
- Or **Duplicate as new preset** in the row's **...** menu and set its **Executable**.
- To start Blender from a preset without streaming at once, set `QAR_BLENDER_AUTO_START=0` in that preset; **Start Streaming** in the **QAROS** tab then starts it.

![The row's ... menu with Duplicate as new preset, beside two Blender rows](../../img/supported-apps/sources-row-menu.png)

## The QAROS tab

The add-on puts a **QAROS** tab in the sidebar of the 3D Viewport. Open it with `N`, or **View > Sidebar**, then click **QAROS** on the sidebar's edge. It is built like Blender's own tabs: panels you can fold, and a tooltip on every button.

![Blender with the turbine imported and streaming, the QAROS tab open in the sidebar](../../img/supported-apps/blender-tutorial/blender-overview.webp)

| Stopped | Streaming | Lower down |
|---|---|---|
| ![QAROS tab while stopped: Off, Start Streaming, Active Object, View, Renderer, Streams To, Headset](../../img/supported-apps/blender-tutorial/qaros-tab-stopped.webp) | ![QAROS tab while streaming: Streaming to 1 user, Stop and restart, Active Object, View, Renderer](../../img/supported-apps/blender-tutorial/qaros-tab-streaming.webp) | ![Lower panels: View, Renderer, Streams To with All devices and one Display Receiver, Headset with Navigation and Object selection, Diagnostics](../../img/supported-apps/blender-tutorial/qaros-tab-streams-to.webp) |

| Panel | What it shows or does |
|---|---|
| **QAROS** | the status: **Off**, **Joining QAROS**, **Streaming to N users**, or why it stopped (the icon in the header says the same). **Start Streaming**, or **Stop** with a restart button beside it. The **App volume** name: empty is **Blender** (**Blender 2** for a second Blender); change it only while stopped |
| **Active Object** | the object Blender's selection makes active, with its eye (**Stream to QAROS**), what the stream does with it, and its material ([Step 6](#step-6-select-a-part-and-set-its-look)) |
| **View** | the stream's shading, following the viewport by default, and **Fit Scene to Volume** ([Step 9](#step-9-shading-and-renderer)) |
| **Renderer** | the renderer that draws the stream, what it can show of your materials, **Frame Rate**, **Resolution** and the VisRTX materials |
| **Streams To** | **All devices**, or a checkbox per headset or receiver; a dot marks one that receives the stream now |
| **Headset** | what hand gestures in the app volume do: **Navigation** or **Object selection**; whether the controls panel beside the app volume is open |
| **Diagnostics** (folded) | objects and triangles streamed, objects outside the volume, and every material approximation or object left out; a warning sign in its header when there is one |

## Step 4: Import the turbine

1. In Blender's default scene, select the **Cube** and press `X` > **Delete**.
2. **File > Import > glTF 2.0 (.glb/.gltf)**.
3. Pick `turbofan.glb`, click **Import glTF 2.0**. The default options are fine.

   ![Blender's glTF import file browser with turbofan.glb selected and Import glTF 2.0](../../img/supported-apps/blender-tutorial/import-gltf-dialog.webp)

4. The engine appears at the world origin, about 4 m long, along the X axis.

The import makes one empty, **Sketchfab_model**, with the engine parts as its children (**Object_2**, **Object_3**, ...).

:::note
Is your scene in millimetres or centimetres (**Scene Properties > Units > Unit Scale** below 1)? The importer then scales the engine to match: in millimetres it is about 4000 units long. The stream counts Blender units and ignores **Unit Scale**, so such an engine streams 1000 times too large: press **Fit Scene to Volume** (step 7).
:::

## Step 5: Scale and place it

The world origin sits at the centre of the app volume. One Blender unit is 0.25 m in the room by default, so the 4 m engine is 1 m long there.

- **Move it:** select **Sketchfab_model** in the Outliner, press `G`, or type **Location** in the sidebar's **Item** tab. Keep it near the origin.
- **Scale it:** press `S`, or set **Scale** on **Sketchfab_model**. All parts follow.
- **Turn it:** press `R`, for example `R` `Z` `90` to face it along Y.
- Skip exact sizing: **Fit Scene to Volume** (step 7) fits whatever you have.

Objects hidden in the viewport (eye icon in the Outliner) are not streamed.

## Step 6: Select a part and set its look

The stream follows Blender's selection and the selected object's own settings.

1. Click a part in the viewport or the Outliner, for example the yellow rings (**Object_5**).
2. **Active Object** in the QAROS tab shows it:
   - **Streamed, outlined in the headset**: it is in the stream and selected, so the headset outlines it;
   - **Streamed, selected (VisGL draws no outline)**: the same on VisGL, which draws no outline;
   - **Streamed**: it is in the stream;
   - its material, with the values the stream uses.

   ![Active Object: Object_5, Streamed, material accent-yellow with its Viewport Color](../../img/supported-apps/blender-tutorial/qaros-tab-active-object.webp)

   The headset outlines the selected object and its children, on VisRTX, OSPRay and Helide ([Step 9](#step-9-shading-and-renderer)).

3. **Object Properties** (orange square): **Location**, **Rotation**, **Scale** stream as you change them. **Transform** in **Active Object** has the same values.
4. **Material Properties** (red sphere): edit the **Principled BSDF**.

   ![Material Properties of the selected part: accent-yellow, Principled BSDF with Base Color, Metallic, Roughness, IOR, Alpha](../../img/supported-apps/blender-tutorial/selected-object-material.webp)

   - Streamed: **Base Color**, **Metallic**, **Roughness**, **Alpha**, **Emission Color** and **Strength**. In **Solid** shading, the material's **Viewport Display > Color**.
   - Use constant values. An input linked to a texture or node streams the material's viewport colour.
   - The turbine's materials are shared by several parts: a change recolours all of them.

**Leave a part out of the stream:** click the eye beside the object's name in **Active Object**. The part stays in Blender and leaves the headset at once; the line below reads **Left out of the stream**. Click the eye again to bring it back. The choice is saved with the `.blend` file.

| In Blender | In the headset |
|---|---|
| ![Active Object: Object_9 with its eye closed, Left out of the stream](../../img/supported-apps/blender-tutorial/qaros-tab-left-out.webp) | ![The turbine in the Display Receiver without its outer casing rings](../../img/supported-apps/blender-tutorial/receiver-left-out.webp) |

## Step 7: Stream it

If you launched Blender from the Visualizer (ways 1 and 2), it already streams: skip to point 6.

1. Make sure QAROS is running (Start menu > **QAROS**).
2. Open the **QAROS** tab (`N`).
3. Optional: type an **App volume** name, for example `Turbine`.
4. Click **Start Streaming**.
5. First time only: click **Approve** under **Waiting for approval** in the Visualizer's **Sources** within 120 s.
6. The status reads **Streaming to 1 user** once a device watches.
7. Click **Fit Scene to Volume** (in **View**) to fill the app volume with the engine.

To pick devices, untick **All devices** under **Streams To** and tick the ones you want. The change applies at once.

## Step 8: View it in the headset

1. Put on the paired headset, or open a Display Receiver. The engine sits in its app volume, with a **Blender** panel beside it.
2. Walk around it and look through the open casing.
3. Interact. **Headset** in the QAROS tab (or the panel's **Tools** tab) chooses what hand gestures do:

   | Do | **Navigation** (default) | **Object selection** |
   |---|---|---|
   | Double tap a part | selects the whole model (here **Sketchfab_model**, the top of the part's parent chain) in Blender too | the same |
   | Click with the hand ray | nothing | selects the single part it hits |
   | One-hand drag | moves your view; Blender's scene stays unchanged | moves the selection, when the drag starts on it, in Blender too |
   | Two hands apart or together | zooms the view | scales the selection |

   A model imported as one empty with parts below it (as the turbine is) moves and scales as a whole after a double tap. Click a part in **Object selection** to move only that part.

   | On the panel | Does |
   |---|---|
   | **Object** tab | edits the selected part's location, rotation, scale |
   | **Material** tab | edits base colour, roughness, metallic |
   | **Frame** tab | sets the current frame, plays the animation |
   | **Panels** tab | Blender's Transform, Relations and Visibility panels |
   | **Undo** / **Redo** | steps back through edits |

4. Edit in Blender at the same time: every change shows in the room within a moment.

## Step 9: Shading and renderer

The headset picture does not come from Blender's viewport: a renderer inside Blender draws it. It follows the viewport's shading, though.

**Viewport shading** (the four spheres at the top right of the viewport, or `Z`): the stream switches with it within a moment.

| Wireframe | Material Preview |
|---|---|
| ![Viewport in Wireframe shading beside the stream, also in wireframe](../../img/supported-apps/blender-tutorial/shading-wireframe.webp) | ![Viewport in Material Preview shading beside the stream, where the metal parts reflect the studio light](../../img/supported-apps/blender-tutorial/shading-material-preview.webp) |

| Shading | The stream shows |
|---|---|
| **Wireframe** | the edges of every mesh, faces empty |
| **Solid** | matte surfaces in each material's **Viewport Display > Color** |
| **Material Preview** | the materials (step 6), lit like the viewport: by its studio light (HDRI, with its strength and rotation), or by the scene's lamps and World with its **Scene Lights** / **Scene World** options |
| **Rendered** | the materials with the scene's lamps and World; the studio light when **Scene World** is off |

- **View** in the QAROS tab can fix one shading for the stream instead: its first button (a chain link, **Follow Viewport**) follows the viewport, the others keep one shading whatever the viewport shows. The line below names the shading in use.
- A change rebuilds the scene in the new shading, which takes a moment on a large scene.
- On VisGL, metal reflects its surroundings, as in Material Preview and EEVEE: the World's environment image, or the studio light. This needs QAROS 0.10.0 or newer; with an older QAROS, metal shows only the lamps' highlights.

**The renderer** is the drop-down in the **Renderer** panel:

| Renderer | Runs on | Look | Use it for |
|---|---|---|---|
| **Automatic** (default) | the first that starts: VisGL, VisRTX, OSPRay, Helide | as that renderer | most PCs |
| **VisGL** | GPU, OpenGL 4.3 | lit, shadows, metallic, roughness, emission, reflections of the World or studio light; no selection outline | headsets: fastest |
| **VisRTX** | NVIDIA RTX GPU | ray traced; grain in a moving view, or smooth with **VisRTX** materials **Matte** | comparing with a ray tracer |
| **OSPRay** | CPU | flat shading, hard shadows; colour only | PCs without a usable GPU |
| **Helide** | CPU | simple reference shading | checking a result |

- Choose one while streaming: the stream restarts on it at once, in the same app volume. Stopped, it applies at the next **Start Streaming**.
- **VisRTX's first start takes a few minutes** on a new PC or after a driver update, while it prepares its GPU programs; later starts take a second. Meanwhile the stream stays on the renderer it had, and the tab reads **Starting VisRTX... the first start can take a few minutes**. It switches by itself once VisRTX is ready. If VisRTX fails, or is not ready within ten minutes, the stream stays where it was and the tab says why.
- The line below the drop-down says which renderer runs and what it can show of your materials.
- The selection is outlined in orange in the headset on VisRTX, OSPRay and Helide. VisGL draws no outline.

## Step 10: Stop

- **Stop** in the QAROS tab. The app volume closes; Blender stays open.
- Or **Stop** on Blender's card under **Running** in the Visualizer: that quits Blender.
- **File > Save** keeps the scene, its **App volume** name, its **Streams To** choice and every part's eye.

## What streams well

| | Streams well | Why |
|---|---|---|
| **Size and place** | a model near the world origin, a few Blender units across; else **Fit Scene to Volume** | the origin is the volume centre and 1 unit is 0.25 m; objects completely outside the volume are not streamed |
| **Units** | **Unit Scale** 1 (metres), or a press of **Fit Scene to Volume** after import | the stream ignores **Unit Scale**: a scene in millimetres streams 1000 times too large |
| **Objects** | up to about 10,000 objects on VisGL (an RTX 3070 Ti holds 30 fps for two users there); few materials | objects that do not move are merged per material, so objects sharing a material cost little; an object you move streams on its own until it settles |
| **Triangles** | up to a few hundred thousand (the turbine has 236,000) | **Diagnostics** counts what is streamed |
| **Materials** | constant **Principled BSDF** values; one material shared by many parts | textures and node inputs stream the material's viewport colour |
| **Lights** | a sun or a few lamps, or the World; on VisGL metal reflects the World or the studio light | without them, surfaces look flat and metal looks dull |
| **Edits** | moving, turning and scaling objects | changing a mesh's shape (Edit Mode, an animated modifier) sends the whole mesh again, which takes longer on a large mesh |

## Performance

- **Renderer:** **VisGL** for headsets. OSPRay and Helide render on the CPU and slow down with several users; VisRTX is ray traced and heavier on the GPU.
- **Frame Rate** (**Renderer** panel, default 30): the frames per second each user's headset gets. Lower leaves the renderer time for more users: scene changes then show later, head motion stays smooth.
- **Resolution** (**Renderer** panel): 0 is automatic, full resolution on VisGL and VisRTX and half on OSPRay and Helide. 0.25 to 1 renders each eye at that share of the headset's resolution.
- **Users:** each device that watches gets its own picture. Untick the ones you do not need under **Streams To**. The add-on's **Users** preference (2 by default) caps how many are served at once.
- **Scene:** leave out what you do not need in the headset (eye in **Active Object**), and share materials between parts where you can.
- **Shading:** a change of viewport shading rebuilds the stream's scene. On a large scene, fix the stream's shading in **View** while you switch the viewport around.

## Limits

- **Materials:** only constant **Principled BSDF** values stream (base colour, metallic, roughness, alpha, emission). Not textures, vertex colours, normal maps, glass, coat, sheen or subsurface.
- **Lights:** a sun stays a sun; point, spot and area lamps become point lights. The World streams in **Rendered** shading or with Material Preview's **Scene World** option; otherwise Material Preview streams the viewport's studio light. Reflections of either need VisGL and QAROS 0.10.0 or newer.
- **Not streamed:** Gaussian splats, point clouds, meshes without faces, objects hidden in the viewport, and objects completely outside the app volume.
- **One app volume per Blender.** A second Blender streams into *Blender 2*.
- **Scale:** Blender's world origin sits at the volume centre, Z up is up, and 1 Blender unit is 0.25 m by default. The scene's **Unit Scale** is ignored.
- A change of the add-on's **ANARI folder** preference needs a Blender restart. Leave it empty: the add-on then uses the ANARI device QAROS installed.
- A Blender you started yourself is never closed by QAROS; **Stop** in the Visualizer quits only a Blender it launched.

## If something is off

**Setup**

| Symptom | Cause | Fix |
|---|---|---|
| No **QAROS** tab in Blender | Blender started without the add-on | Start it from its Blender row in **Sources** (way 1 or 2), or install and enable the add-on by hand (way 3) |
| Your Blender has no row | QAROS did not find it | Way 2: **+** > **Open QAROS supported app**, or **Locate...** |
| No **Install QAROS add-on** on the row; it says *Installed by hand, not by QAROS* | the add-on was installed by hand | Update it by hand (way 3), or remove it from Blender and let QAROS install it |
| The add-on refuses to start and says why | Blender 5.3 or newer, which it does not support yet | Use Blender 4.5 LTS to 5.2 |
| Status stays at **Joining QAROS** | Blender waits for approval | Approve it in the Visualizer's **Sources** under **Waiting for approval**; the request expires after 120 s |
| No renderer loads | QAROS has no ANARI device, or the add-on looks in the wrong folder | Install QAROS with the **ANARI device** component; leave the add-on's **ANARI folder** preference empty |

**In the headset**

| Symptom | Cause | Fix |
|---|---|---|
| Nothing in the headset | the device is not in **Streams To**, or not paired | Tick it (or **All devices**) under **Streams To**; pair the headset |
| A second headset shows nothing | the add-on serves at most **Users** devices (preference, 2 by default) | Raise **Users** in the add-on's preferences, then restart streaming |
| Model tiny, huge or cut off | its size in Blender units, or a **Unit Scale** below 1 (millimetres, centimetres) | **Fit Scene to Volume** |
| A part is missing | its eye in **Active Object** is closed, it is hidden in the viewport, or it is outside the volume | Open the eye, unhide it, or **Fit Scene to Volume**; **Diagnostics** lists what was left out |
| A double tap selects the whole model, not the part | a double tap takes the top of the part's parent chain | Choose **Object selection** and click the part with the hand ray |
| Metal looks dull or dark | no lamps or World in the stream, **Solid** shading, a renderer other than VisGL, or QAROS older than 0.10.0 | Use **Material Preview** or **Rendered** on VisGL, or add a lamp |
| No orange outline on the selection | VisGL draws none | Choose VisRTX, OSPRay or Helide to see it |

**Materials**

| Symptom | Cause | Fix |
|---|---|---|
| A colour change does nothing | the edited node does not feed **Material Output**, or the input is linked to a texture or node | Edit the **Principled BSDF** that feeds **Material Output**, with a constant value; in **Solid** shading, edit **Viewport Display > Color** |
| A part shows its viewport colour instead of its texture | textures do not stream | Set a constant **Base Color** |
| Alpha stays opaque | the renderer cannot blend (**Diagnostics** says so) | Choose VisGL |
| Glass, transmission, coat or sheen have no effect | they do not stream | Use **Alpha** for see-through parts |

**Speed**

| Symptom | Cause | Fix |
|---|---|---|
| Slow, all CPU cores busy | the renderer is OSPRay or Helide | Choose **VisGL** in **Renderer** |
| **Starting VisRTX...** for minutes | VisRTX's first start on this PC or driver | Wait: the stream stays on the current renderer and switches by itself |
| Low frame rate with several users | each user gets their own picture | Lower **Frame Rate** or **Resolution**, or stream to fewer devices ([Performance](#performance)) |

More, for every add-on setting: the [QAROS Blender add-on user guide](https://github.com/Quaternar/qaros-blender/blob/main/docs/user-guide.md).
