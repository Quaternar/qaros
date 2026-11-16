# {{DISPLAY_NAME}}

A QAROS web app adaptation for a WebGPU page that exposes nothing, made from the adaptation kit's
`webgpu-redirect` template. The page's own loop keeps running into an offscreen texture; the
presenter replays its last frame once per user eye with that eye's camera.

| Script | Stage | What it does |
|---|---|---|
| `webgpu-redirect-capture@1` | preDocument | redirects the page's canvas into an offscreen texture, records its frame |
| `render-frame-core@1` | afterLoad | the render-frame protocol: acknowledge, fail, tag, pacing |
| `webgpu-redirect-presenter@1` | afterLoad | replays the frame per eye, packs depth and alpha, presents the page region |
| `scripts/site-camera.js` | afterLoad | **yours:** which buffer holds the page's camera, and its layout |

## The one thing to write

Set `CAMERA_BUFFER` (the label of the page's camera uniform buffer) and `CAMERA_LAYOUT` (the
matrices the page's shaders read from it) in `scripts/site-camera.js`. List the captured buffers in
the page with `[...__qarAdaptationState.buffersByLabel.keys()]`.

- Compute passes run once per page frame, not per eye: every user sees the same simulation.

## Work on it

```
qaros-adaptation check .     # profile, library references, known pitfalls
qaros-adaptation test .      # test/*.test.mjs and the kit tests of the libraries used
qaros-adaptation package .   # the .qarapp to import into QAROS (--sign key.pem to sign)
qaros-adaptation verify .    # depth, thin-line and alpha coverage of the running source
```

Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/
