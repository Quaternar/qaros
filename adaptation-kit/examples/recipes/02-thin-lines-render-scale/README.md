# Recipe 2: thin lines, MSAA and RenderScale

1-px lines on a page that asks for `antialias: true`. The users' eye views are rendered without multisampling (the page's MSAA applies to its own canvas only) and at `RenderScale` of each user's resolution.
At `RenderScale` 0.5 a line one page pixel wide covers half an eye pixel: it survives (lines rasterize at least one pixel wide) but every pixel of it is a full eye pixel, twice as wide in the headset.
This profile sets `RenderScale` 1.0, the setting for line-heavy content (CAD, maps, charts); keep 0.5 to 0.75 for shaded scenes where the cost matters more than edges.
Depth stays correct at any scale: each half-resolution depth sample keeps the nearest depth of its block.
