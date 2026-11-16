# WebGPU reference application

Voxel Atelier: a hand-written WebGPU viewer with left and right interface panels, a notification
toast stack, a modal popup, and a HUD overlay drawn on top of its canvas. It contains no QAROS code,
and imports no rendering engine, scene graph or matrix library: every shape (`src/geometry.js`), WGSL
shader (`src/main.js`) and matrix operation (`src/mat4.js`) is written out by hand, the WebGPU twin
of `../webgl-reference-app/`.

WebGPU needs a secure-context origin: load the page from `file://` or `https://`. A top-level
`data:` page gets an opaque origin, and `navigator.gpu` is hidden there.

The application exposes more globals than the WebGL one, because WebGPU's API shape requires it:
`window.gpuDevice`, `window.gpuCamera` (view/projection matrices), `window.gpuDraw` (issues one
frame's draw calls into a given render pass, pipeline and viewport rectangle),
`window.gpuCanvasFormat`, `window.gpuColorPipeline`, `window.gpuDepthOnlyPipeline`,
`window.gpuBindGroupLayout` and `window.gpuEnsureDepthTexture`. `gpuDevice` and `gpuCanvasFormat`
are not a convenience: a `GPUCanvasContext` has no accessor back to the device or the format it was
configured with. The pipeline globals let the adaptation reuse the application's own pipelines.

## Known limitation: a page whose own loop calls `getCurrentTexture()`

This application renders every frame from its own `requestAnimationFrame` loop, which calls
`context.getCurrentTexture()` every frame. In the QAROS browser, any `getCurrentTexture()` call on a
canvas by a second caller (here, the adaptation drawing the users' views) defeats the content the
first one drew: the paint keeps arriving, with the wrong content. Suppressing the application's loop
and driving it from the adaptation does not change that. So with this application's own loop
running, `webgpu-render-frame-handler@1` does not reach the users' views; against a page with no
competing loop it does.

A WebGPU page with its own loop is streamed the other way: `webgpu-redirect-capture@1` makes the
adaptation the only caller of the real `getCurrentTexture()` (the application draws into an
offscreen texture) and `webgpu-redirect-presenter@1` replays its frame per eye. See
`../recipes/07-webgpu-redirect-presenter/`.

## Build

```powershell
npm install
npm run build   # static assets -> dist/, loadable from the filesystem (file:///.../dist/index.html)
```

`npm run dev` starts a Vite dev server for working on the application itself. `dist/` is a build
product and is not committed.

## Adaptation

`adaptation/profile.json` names only kit libraries (`../../lib/`), resolved by QAROS at load time.
Page region 360x210.

| Script | Library | Does |
|---|---|---|
| `capture-context` | `webgpu-capture-context@1` | patches `getContext` for every canvas, confines `GPURenderPassEncoder.prototype.setViewport` (with a matching scissor) to the page region |
| `render-frame-core` | `render-frame-core@1` | acknowledges or fails each request |
| `ui-confinement` | `ui-confinement@1` | confines the application's `#app-ui` container to the page region |
| `render-frame-handler` | `webgpu-render-frame-handler@1` | per request, drives `window.gpuDraw` once per eye, renders depth at full resolution and packs it (nearest of each 2x2 block) |

`webgpu-render-frame-handler@1` is written for this application's globals: an example of a
hand-written handler, not a general one. It submits every destination before writing the next
one's uniform buffers (`queue.writeBuffer` copies at call time, so a buffer shared by two
unsubmitted draws gives both the last value). The profile's `Channel` is `debug`: it is shown in
Developer mode only.
