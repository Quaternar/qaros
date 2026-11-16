# {{DISPLAY_NAME}}

A QAROS web app adaptation for a three.js page (`WebGLRenderer` or `WebGPURenderer`), made from
the adaptation kit's `threejs` template.

| Script | Stage | What it does |
|---|---|---|
| `threejs-capture@1` | preDocument | finds the page's renderers and scenes, confines its own view to the page region |
| `scripts/scene-selection.js` | preDocument | **yours:** which scene and renderer the users see |
| `render-frame-core@1` | afterLoad | the render-frame protocol: acknowledge, fail, tag, pacing |
| `webgl-atlas-pack@1` | afterLoad | colour, nearest-of-2x2 depth and alpha into the atlas |
| `threejs-render-frame@1` | afterLoad | every user's eyes from a `WebGLRenderer` |
| `threejs-webgpu-render-frame@1` | afterLoad | the same for a `WebGPURenderer` |

Both render-frame libraries are listed: each request goes to the one for the renderer the page
created, so one profile covers both.

## Work on it

```
qaros-adaptation check .     # profile, library references, known pitfalls
qaros-adaptation test .      # test/*.test.mjs and the kit tests of the libraries used
qaros-adaptation package .   # the .qarapp to import into QAROS (--sign key.pem to sign)
qaros-adaptation verify .    # depth, thin-line and alpha coverage of the running source
```

- Tune `AppScale` (room metres per scene unit; `AppScaleByAddress` per address) and `RenderScale`
  in `profile.json`.
- A page with an `#app-ui` container for its interface can add `ui-confinement@1` (afterLoad) to
  keep that interface inside the page region.
- A gallery that shows its examples in an iframe needs a script that opens the framed page at top
  level; `check` warns when the address pattern looks like such a page.

Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/
