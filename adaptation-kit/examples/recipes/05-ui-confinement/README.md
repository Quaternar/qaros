# Recipe 5: UI confinement

A page whose whole interface sits under one `#app-ui` container (`position: fixed`, full viewport, `overflow: hidden`, children `position: absolute`).
`ui-confinement@1` sizes `#app-ui` to the page region (`PageRegion`), so the interface stays where the page preview shows it, and the users' views carry only the 3D scene.
Use it only for pages that follow this convention (the kit's reference apps do); a third-party website keeps its own layout, and the render-frame libraries already draw users' views without its HTML.
`threejs-capture@1` keeps the app's own canvas render inside the page region for the same reason.
