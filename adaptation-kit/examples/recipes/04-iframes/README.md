# Recipe 4: iframes and multi-frame pages

`index.html` is a gallery whose 3D view is `frame.html` in an iframe (the threejs.org examples case). QAROS streams the renderer of the top-level page, so the gallery alone never starts.
`scripts/open-framed-page.js` opens the framed page at top level (`location.replace`); the profile's `AddressPattern` matches both pages, so the same scripts adapt the framed one.
Same-origin child frames that render themselves share the top-level page's adaptation state (`threejs-capture@1`), but only the top-level page is streamed.
`qaros-adaptation check` warns when a pattern matches index or gallery pages.
