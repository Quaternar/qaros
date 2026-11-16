# Recipe 2: thin lines, MSAA and RenderScale

1-px lines on a page that asks for `antialias: true`. The users' eye views are rendered at `RenderScale` of each user's resolution, and without multisampling unless the profile sets `Antialias` (the page's MSAA applies to its own canvas only).
At `RenderScale` 0.5 a line one page pixel wide covers half an eye pixel: it survives (lines rasterize at least one pixel wide) but every pixel of it is a full eye pixel, twice as wide in the headset.
This profile sets `RenderScale` 1.0 and `Antialias` 4, the settings for line-heavy content (CAD, maps, charts, grids): the eye views are multisampled like the page's canvas, so lines thinner than a pixel stay continuous instead of dotted. Keep `RenderScale` 0.5 to 0.75 without `Antialias` for shaded scenes where the cost (about 1 ms of GPU time per frame at 1920 x 1080) matters more than edges.
Depth stays correct at any scale: each half-resolution depth sample keeps the nearest depth of its block.
