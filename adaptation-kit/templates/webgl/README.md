# {{DISPLAY_NAME}}

A QAROS web app adaptation for a raw WebGL2 page (no engine), made from the adaptation kit's
`webgl` template.

| Script | Stage | What it does |
|---|---|---|
| `webgl-capture-context@1` | preDocument | captures the page's WebGL2 context, confines its own view to the page region |
| `render-frame-core@1` | afterLoad | the render-frame protocol: acknowledge, fail, tag, pacing |
| `webgl-atlas-pack@1` | afterLoad | colour, nearest-of-2x2 depth and alpha into the atlas |
| `scripts/render-frame.js` | afterLoad | **yours:** draws every user's eyes into the offscreen pass |

## The one thing to write

`drawEye(gl, camera, eye)` in `scripts/render-frame.js` draws the page's scene once with
`camera.view` and `camera.projection` (column-major, standard WebGL depth). Usually that means
calling the page's own draw function with its camera matrices swapped for the eye's. Find them with
the Visualizer's authoring tools (heap snapshot, live evaluation). Everything else (cameras per eye,
offscreen layout, packing, tags, pacing) is done.

## Work on it

```
qaros-adaptation check .     # profile, library references, known pitfalls
qaros-adaptation test .      # test/*.test.mjs and the kit tests of the libraries used
qaros-adaptation package .   # the .qarapp to import into QAROS (--sign key.pem to sign)
qaros-adaptation verify .    # depth, thin-line and alpha coverage of the running source
```

Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/
