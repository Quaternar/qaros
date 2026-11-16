# {{DISPLAY_NAME}}

A QAROS web app adaptation for a WebGPU page that exposes its draw entry points, made from the
adaptation kit's `webgpu` template. For a WebGPU page you cannot change, start from the
`webgpu-redirect` template instead.

| Script | Stage | What it does |
|---|---|---|
| `webgpu-capture-context@1` | preDocument | captures the page's WebGPU canvas context, confines its own view to the page region |
| `render-frame-core@1` | afterLoad | the render-frame protocol: acknowledge, fail, tag, pacing |
| `scripts/app-bridge.js` | afterLoad | **yours:** publishes the page's device, camera, pipelines and draw function |
| `webgpu-render-frame-handler@1` | afterLoad | draws every user's eyes with them, packs depth into the atlas |

## The one thing to write

Point the getters in `scripts/app-bridge.js` at where the page keeps each entry point (the table at
the top of that file lists what the handler needs). A page written for QAROS may set the
`window.gpu*` names itself; the bridge then leaves them alone. A missing entry point is reported by
name in the Visualizer's events.

## Work on it

```
qaros-adaptation check .     # profile, library references, known pitfalls
qaros-adaptation test .      # test/*.test.mjs and the kit tests of the libraries used
qaros-adaptation package .   # the .qarapp to import into QAROS (--sign key.pem to sign)
qaros-adaptation verify .    # depth, thin-line and alpha coverage of the running source
```

Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/
