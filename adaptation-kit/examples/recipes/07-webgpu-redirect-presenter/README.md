# Recipe 7: WebGPU redirect presenter

A raw WebGPU page with its own render loop, which calls `getCurrentTexture()` every frame. A second caller of `getCurrentTexture()` on the same canvas (an adaptation drawing users' views) defeats what the first one drew, so such a page cannot be drawn over.
`webgpu-redirect-capture@1` makes the adaptation the only real caller: the page draws into an offscreen texture, and its passes, attachments and bind groups are recorded. `webgpu-redirect-presenter@1` replays the recorded frame once per eye into per-eye copies of its targets, then packs colour (with the empty-pixel fill), nearest-of-2x2 depth and coverage alpha.
The site supplies only the camera (`scripts/site.js`): pairs of (page buffer, eye buffer) holding each eye's view-projection. Compute passes are not replayed, so every user sees the same simulation.
Needs `ContinuousRendering: true`.
