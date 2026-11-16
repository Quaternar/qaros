# Recipe 3: depth conventions

Spheres at several distances. A page packs **device depth** (the raw post-projection value, non-linear), never linear or metric depth: native code is told the near/far and convention of each camera and reconstructs metres itself.
Each request names the convention per camera (`target.cameras.left.depthConvention`): `regular` (standard Z, cleared to 1, nearest = smallest), `inverted` (reversed Z) or `infiniteFar`. The kit's libraries render `regular` depth with the request's near/far.
An app that uses reversed Z or a logarithmic depth buffer for its own view is fine (the eye views use their own cameras), but a hand-written handler must pack what the request asks for; `scripts/depth-convention.js` reports a mismatch.
WebGL and WebGPU differ in clip-space z (-1..1 against 0..1), not in window depth: both pack the same 0..1 value.
