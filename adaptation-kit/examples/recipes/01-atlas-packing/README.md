# Recipe 1: atlas packing

1-px lines and a box, streamed with colour, depth and (`TransportAlpha`) alpha. Depth and alpha travel at half resolution, one sample per 2x2 block: a sample that kept one texel lost a 1-px line in about half its blocks, so users saw it dashed.
The kit's packers keep the **nearest depth** and the **largest coverage** of each block, and give an empty pixel in a covered block the colour of its nearest pixel, so colour, depth and alpha agree.
`?webgpu` draws the same scene with `WebGPURenderer`: the profile lists both three.js render-frame libraries and the one for the page's renderer answers.
Packer rule tests: `lib/tests/threejs-depth-pack.test.mjs` (GLSL) and `lib/tests/webgpu-depth-pack.test.mjs` (WGSL).
