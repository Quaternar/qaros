# Raw WebGL reference application

Facet Lab: a hand-written WebGL2 viewer with left and right interface panels, a notification toast
stack, a modal popup, and a HUD overlay drawn on top of its canvas. It contains no QAROS code, and
imports no rendering engine, scene graph or matrix library: every shape (`src/geometry.js`), shader
(`src/main.js`) and matrix operation (`src/mat4.js`) is written out by hand.

This is the harder of the two application shapes (see `../threejs-reference-app/` for the
recognised-engine case): there is no library global to recognise, so the adaptation finds the
renderer by intercepting the standard Canvas/WebGL API.

The application exposes `window.glCamera` (the current view/projection matrices) and `window.glDraw`
(a function that issues one frame's draw calls) for console debugging, an ordinary habit for a
hand-rolled WebGL project. The adaptation drives them to render each user's eyes. In an
unfamiliar application, finding such an entry point is what the authoring tools' heap snapshots and
live evaluation are for.

## Build

```powershell
npm install
npm run build   # static assets -> dist/, loadable from the filesystem (file:///.../dist/index.html)
```

`npm run dev` starts a Vite dev server for working on the application itself. `dist/` is a build
product and is not committed.

## Adaptation

`adaptation/profile.json` names only kit libraries (`../../lib/`), resolved by QAROS at load time.
Page region 400x240.

| Script | Library | Does |
|---|---|---|
| `capture-context` | `webgl-capture-context@1` | patches `getContext` for every canvas, confines the captured context's `viewport` to the page region |
| `render-frame-core` | `render-frame-core@1` | acknowledges or fails each request, writes the request tag |
| `ui-confinement` | `ui-confinement@1` | confines the application's `#app-ui` container to the page region |
| `render-frame-handler` | `webgl-render-frame-handler@1` | per request, drives `window.glDraw()` once per eye and packs depth (nearest of each 2x2 block) |

`webgl-render-frame-handler@1` is written for this application's `glDraw`/`glCamera`: treat it as an
example of a hand-written handler to copy and adapt, not a general one. The profile's `Channel` is
`debug`: it is shown in Developer mode only.
