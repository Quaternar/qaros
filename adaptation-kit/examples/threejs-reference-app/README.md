# three.js reference application

Aurora Model Viewer: an ordinary three.js model viewer with left and right interface panels, a
notification toast stack, a modal popup, and a HUD overlay drawn on top of its canvas. It contains no
QAROS code: no reference to `window.qaros`, no `qaros-render-frame` listener. It stands in for a
third-party application nobody integrated with QAROS, so its adaptation (`adaptation/profile.json`)
takes over its canvas from the outside, as it would for an application you do not control.

The left sidebar switches between the Geometry gallery and a glTF AVIF example in place
(`assets/avif-triangle.gltf`, whose material references `assets/helmet-texture.avif`), so one
browser can check several scenes.

Like many published three.js demos it assigns `window.THREE`, `window.scene`, `window.camera` and
`window.renderer` for console debugging. The adaptation does not need them: it finds the renderer
and scene through three.js's devtools hook.

## Build

```powershell
npm install
npm run build   # static assets -> dist/, loadable from the filesystem (file:///.../dist/index.html)
```

`npm run dev` starts a Vite dev server for working on the application itself. `dist/` is a build
product and is not committed.

## Adaptation

`adaptation/profile.json` names only kit libraries (`../../lib/`), resolved by QAROS at load time:

| Script | Library | Does |
|---|---|---|
| `capture-renderer` | `threejs-capture@1` | finds the renderer and scene, keeps the application's own renders in the page region (480x270) |
| `render-frame-core` | `render-frame-core@1` | acknowledges each request, picks the render-frame library for the renderer, owns the stream surface |
| `ui-confinement` | `ui-confinement@1` | confines the application's `#app-ui` container to the page region |
| `webgl-atlas-pack` | `webgl-atlas-pack@1` | colour, nearest-of-2x2 depth and coverage alpha into the atlas (WebGL) |
| `render-frame-webgl` | `threejs-render-frame@1` | renders every user's eyes for a `WebGLRenderer` |
| `render-frame-webgpu` | `threejs-webgpu-render-frame@1` | the same for a `WebGPURenderer` |

This is the pattern for any three.js profile: list both render-frame libraries and one profile
covers either renderer. `ui-confinement@1` is for applications that follow the `#app-ui`
convention; a third-party website usually keeps its own layout without it.

The profile's `Channel` is `debug`: it is shown in Developer mode only.
