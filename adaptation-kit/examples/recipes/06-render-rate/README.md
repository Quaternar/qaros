# Recipe 6: continuous rendering and RenderRate

`ContinuousRendering: true`: the page draws every user's views on Chromium's animation tick from the newest cameras, without waiting for a paint to be read back (`render-frame-core@1` `createRenderFrameLoop`). Without it each request draws once (on demand): simpler, about a third of the frame rate.
`RenderRate` (1 to 120 frames per second) caps that loop; a profile without one draws at **30**. The loop skips ticks until the next frame is due and keeps acknowledging requests as before.

| `RenderRate` | For |
|---|---|
| 15 | slowly changing content (dashboards, maps, documents), heavy scenes, battery-powered hosts |
| 30 (default) | most scenes |
| 60 | smooth animation and fast motion (this kit's samples, games) |

This profile sets 30; change it to 15 or 60 to compare. Hand-written loops follow the same rule with `render-frame-core@1` `isRenderDue(detail, timestamp, lastDraw)`.
