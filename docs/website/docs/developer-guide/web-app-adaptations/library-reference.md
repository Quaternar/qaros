---
sidebar_position: 3
title: Library Reference
description: Every adaptation kit library - its stage, what it needs, the globals and hooks it provides, what it guarantees and what it does not cover.
---

# Library Reference

Each library is one file in the kit's `lib/`, named `<name>@<major>.js`. Its header comment states
its exact version (`// @qaros-library name@major.minor`), its stage (`// @stage`) and the libraries
it needs (`// @needs`); the header is the authoritative description, this page the overview.
Reference a library from a profile with `"Library": "<name>@<major>"`
([libraries by name](index.md#libraries-by-name-and-major-version)).

| Library | Stage | Needs | For |
|---|---|---|---|
| [`render-frame-core@1`](#render-frame-core1) | afterLoad | | every handler |
| [`threejs-capture@1`](#threejs-capture1) | preDocument | | three.js |
| [`webgl-atlas-pack@1`](#webgl-atlas-pack1) | afterLoad | | three.js `WebGLRenderer`, raw WebGL |
| [`threejs-render-frame@1`](#threejs-render-frame1) | afterLoad | core, `threejs-capture@1`, `webgl-atlas-pack@1` | three.js `WebGLRenderer` |
| [`threejs-webgpu-render-frame@1`](#threejs-webgpu-render-frame1) | afterLoad | core, `threejs-capture@1` | three.js `WebGPURenderer` |
| [`ui-confinement@1`](#ui-confinement1) | afterLoad | | pages with an `#app-ui` container |
| [`webgl-capture-context@1`](#webgl-capture-context1) | preDocument | | raw WebGL2 |
| [`webgl-render-frame-handler@1`](#webgl-render-frame-handler1) | afterLoad | core, `webgl-capture-context@1` | the WebGL reference app (example) |
| [`webgpu-capture-context@1`](#webgpu-capture-context1) | preDocument | | raw WebGPU exposing its device |
| [`webgpu-render-frame-handler@1`](#webgpu-render-frame-handler1) | afterLoad | core, `webgpu-capture-context@1` | the WebGPU reference app (example) |
| [`webgpu-redirect-capture@1`](#webgpu-redirect-capture1) | preDocument | | WebGPU with its own loop |
| [`webgpu-redirect-presenter@1`](#webgpu-redirect-presenter1) | afterLoad | core, `webgpu-redirect-capture@1` | WebGPU with its own loop |

"core" is `render-frame-core@1`. List a library after every library it needs; `check` enforces it.

## Contracts every packer keeps

The libraries that write the atlas share one contract, so native code reads every adaptation the
same way ([atlas packing](recipes.md#1-atlas-packing)):

- **Colour:** one eye pixel per atlas pixel, opaque.
- **Depth:** raw device depth (standard Z, cleared to 1), 24-bit in RGB, at half resolution; each
  sample is the **nearest depth of its 2x2 block**; saturated at the far plane.
- **Alpha** (`TransportAlpha`): half resolution, three rows per atlas row; each sample is the
  **largest coverage of its 2x2 block**.
- **Edges:** an empty pixel in a block that holds geometry takes the colour of the block's nearest
  pixel, so colour, depth and alpha describe the same block.
- **Tag:** two exact pixels in the atlas's last row identify the request the paint answers.

## render-frame-core@1

The engine-agnostic protocol layer. Inject it before any handler. Provides
`window.__qarAdaptationCore`:

| Member | Does |
|---|---|
| `withRenderFrameAcknowledgement(detail, work)` | Runs `work` for one request and acknowledges it; a throw or rejected promise fails it. A `true` result means the frame is tagged and is acknowledged at once |
| `writeRenderFrameTag(gl, detail)` | WebGL: writes the request tag. A WebGPU handler writes the same two pixels itself |
| `createRenderFrameLoop(work)` | Continuous pacing: draws the newest cameras on Chromium's animation tick, at most `RenderRate` frames per second (since 1.1; 1.0 drew up to 60). Returns `{ update, pause, stop }` |
| `isRenderDue(detail, timestamp, lastDraw)`, `renderRateOf(detail)` | The same pacing rule for a hand-written loop (since 1.1) |
| `addRenderFrameHandler({ name, accepts, loop })` | Several handlers on one page, one answering: each request goes to the first whose `accepts(detail)` is true (since 1.1) |
| `streamSurface` | The separate surface an `Engine: "three.js"` handler publishes its atlas to, leaving the page untouched (since 1.1) |

- Guarantees: every request is acknowledged or failed exactly once, so no request is left waiting;
  `qaros-render-stop` pauses every loop.

## threejs-capture@1

Finds the page's three.js renderers and scenes without any global from the page, and keeps the
page's own render inside the page region.

- Installs the hook three.js announces every `WebGLRenderer`, `WebGPURenderer` and `Scene` to, so
  module, bundled and global builds are all found. Must run `preDocument`.
- Keeps state in `window.__qarAdaptationState` (`renderer`, `scene`, `camera`, `scenes`,
  `renderers`).
- Hooks a site script may set:

| Hook | Asked | Default |
|---|---|---|
| `selectScene(rendered)` | once per frame with this frame's `[{ scene, camera }]` | the last scene drawn with a perspective camera (a HUD with an orthographic camera is skipped) |
| `chooseRenderer(renderers)` | once per frame with every captured renderer | every renderer is the streamed one (right for single-renderer pages) |

- Only the chosen renderer is resized to the atlas and confined; the others render untouched.

## webgl-atlas-pack@1

Packs WebGL2 eye images into the atlas by the [shared contract](#contracts-every-packer-keeps).

- `window.__qarWebglAtlasPack.createAtlasPacker()` returns `{ ensurePass, copyEyesToAtlas }`:
  render every eye into the pass (an RGBA8 colour texture and a 24-bit depth texture, standard
  depth cleared to 1), then `copyEyesToAtlas` writes colour, depth and alpha into each view's
  rectangle.
- Engine-agnostic: use it for a raw WebGL handler instead of writing a packer.

## threejs-render-frame@1

Answers render-frame requests for a three.js `WebGLRenderer` page from the page's own renderer and
scene.

- One `renderer.render` per tick for every eye of every user (an `ArrayCamera`): culling, shadows
  and lights run once. three.js applies the page's tone mapping and output colour space.
- Each eye camera takes the page camera's layers. Users' views never show `scene.background`.
- `window.__qarAdaptationState.sceneFromRoom = () => ({ position, quaternion })` places the users'
  room in the scene (a Z-up scene, or a room anchored where the page's camera looks).
- Publishes the atlas to the stream surface; the page's own canvas and view stay as they were.
- List [`threejs-webgpu-render-frame@1`](#threejs-webgpu-render-frame1) too and one profile covers
  both renderers.
- Not covered: post-processing (`EffectComposer` passes run on the page's own frame only), a
  logarithmic depth buffer, multisampling of the eye views.

## threejs-webgpu-render-frame@1

The same atlas and request tag as `threejs-render-frame@1`, for a three.js `WebGPURenderer` page
(the threejs.org `webgpu_*` examples, apps built on `three/webgpu`).

- Each eye is rendered by three.js into a render target with a depth texture; three.js's own output
  pass applies tone mapping and the output colour space; WGSL passes on the page's `GPUDevice` pack
  the atlas. The page's own canvas and frame are never touched.
- Same hooks as above (`sceneFromRoom`, layers, no scene background).
- Reads three.js internals no public API exposes; each is checked and named in the error when
  missing. Verified with three.js r180.
- Not covered: the WebGL 2 fallback backend of `WebGPURenderer`, post-processing
  (`PostProcessing` / `RenderPipeline`), a logarithmic depth buffer, multisampling of the eye views.

## ui-confinement@1

Keeps a page's own interface inside the page region.

- Works only for pages whose whole interface sits under one `#app-ui` container (`position: fixed`,
  full viewport, `overflow: hidden`, children `position: absolute`). The kit's reference apps
  follow this; most third-party sites do not, and keep their layout without it.
- See [UI confinement](recipes.md#5-ui-confinement).

## webgl-capture-context@1

Canvas takeover for a raw WebGL2 page.

- Patches `getContext` for every canvas, so it needs no global from the page. Must run
  `preDocument`. State: `window.__qarAdaptationState` (`canvas`, `gl`, `suspendConfinement`).
- Confines every viewport the page sets to the page region, unless a handler sets
  `suspendConfinement` while it draws users' eyes.

## webgl-render-frame-handler@1

The WebGL reference app's handler: drives the page's `window.glDraw()` with `window.glCamera`
matrices per eye. An **example** of a hand-written handler, not a general one: the `webgl` template
starts from the same structure with `webgl-atlas-pack@1`.

## webgpu-capture-context@1

Canvas takeover for a raw WebGPU page, the WebGPU twin of `webgl-capture-context@1`.

- Patches `getContext` and `GPURenderPassEncoder.prototype.setViewport` (with a matching scissor) to
  confine the page's own viewport to the page region.
- The page must expose `window.gpuDevice`: a WebGPU canvas context has no way back to its device.

## webgpu-render-frame-handler@1

The WebGPU reference app's handler: drives `window.gpuDraw(pass, pipeline, rect)` with
`window.gpuCamera` per eye and packs depth with WGSL. An **example**: copy and adapt it, or use the
`webgpu` template.

- One submit per destination: a uniform buffer written twice before one submit gives every eye the
  second value.

## webgpu-redirect-capture@1

For a WebGPU page with its own render loop that exposes nothing.

- Makes the adaptation the only caller of the real `getCurrentTexture()`: the page draws into an
  offscreen texture of its own size. A second caller on the same canvas would defeat what the
  first drew, so drawing beside such a page cannot work.
- Records the page's last frame: every render pass that drew into that texture, its attachments
  and commands, every bind group's descriptor, every labelled buffer (`buffersByLabel`).
- Touches no WebGPU interface at load (they do not exist yet at `preDocument`); patches install on
  the first `getContext("webgpu")`.

## webgpu-redirect-presenter@1

Presents the redirected page. On each tick it:

1. replays the recorded frame once per eye, into per-eye copies of the page's attachments
   (G-buffers, depth), with that eye's camera buffers substituted;
2. packs colour (with the empty-pixel colour fill, since 1.1), depth and coverage alpha by the
   [shared contract](#contracts-every-packer-keeps);
3. copies the page's own frame into the page region only;
4. writes the request tag.

- Needs `ContinuousRendering: true`; draws at most `RenderRate` per second.
- The site supplies only the camera, in an afterLoad script after the presenter:

```js
window.__qarAdaptationState.site = {
  eyeBuffers(eye) {      // required: [[pageBuffer, eyeBuffer], ...], one eye buffer per eye
    // write eye.viewProjection (column-major, WebGPU 0..1 depth) into this eye's own buffer
  },
  beginTick() {},        // optional: once per tick
  nearFarScale: 1,       // optional: page units per scene unit
};
```

- Compute passes are not replayed: the page's loop runs them once per frame, so every user sees
  the same simulation.
- See [WebGPU redirect presenter](recipes.md#7-webgpu-redirect-presenter).
