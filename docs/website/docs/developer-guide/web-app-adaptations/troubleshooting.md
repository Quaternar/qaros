---
sidebar_position: 6
title: Troubleshooting
description: Web app adaptation load errors (update QAROS, kit file changed, Library and File), leftover adaptations and how to uninstall one, pages in iframes, blank views and dashed thin lines.
---

# Troubleshooting

The Visualizer shows why an adaptation did not load in a notification and on its Sources row; the
message starts with the adaptation's name.
`qaros-adaptation check` finds most of these before you package.

## Load errors

| Message | Cause | Do |
|---|---|---|
| "requires `name@2`, this QAROS provides @1: update QAROS" | The adaptation names a library major this QAROS does not ship | Update QAROS. As an author: name the major the oldest QAROS you support has, check with `--kit <that kit>`, or [pin](quick-start.md#pin-a-library) a copy |
| "requires `name@1`, which this QAROS does not provide: update QAROS" | A library newer than this QAROS | As above |
| "uses `name@1`, whose kit file changed since QAROS was installed: reinstall QAROS" | A file in `<install>/share/qaros/adaptation-kit/lib/` no longer matches the hash in `lib/index.json` | Reinstall QAROS. Never edit the installed kit: [pin](quick-start.md#pin-a-library) a copy into your adaptation and change that |
| "requires `name@1`, but this QAROS has no adaptation kit installed: reinstall QAROS" | `share/qaros/adaptation-kit` is missing | Reinstall QAROS |
| "script '...' names both a Library and a File; name one" | One script entry has both keys | Keep `Library` (fixes from QAROS) or `File` (a pinned copy) |
| A signed adaptation whose files changed is not loaded | A file of the package was edited after signing | Package and sign again, or duplicate it in the Visualizer and edit the copy |

## The wrong adaptation is picked

- The most specific `AddressPattern` matching the page wins. An older adaptation with a narrower
  pattern (one page of a site) beats a broader one (the whole site).
- An adaptation QAROS installed from an earlier web app release that the current release no longer
  lists is never picked by address, so it cannot override the one that replaced it. Your own
  imports are never set aside like this.
- To remove an adaptation you installed (an import, or a release leftover): Source Applications,
  the row's **...** menu, **Uninstall web app adaptation...**, then confirm. An installed
  adaptation that no tile opens gets a tile of its own so that it can be removed. Adaptations that
  ship with QAROS cannot be uninstalled.
- Developer and debug adaptations (`Channel`) show only in the Visualizer's Developer mode.

## The source never starts

- **The 3D view is in an iframe.** QAROS streams the top-level page's renderer: open the framed
  page at top level ([iframes](recipes.md#4-iframes-and-multi-frame-pages)).
- **Login page.** No renderer exists until you log in: press **Page**, log in, then probe again.
- **A `preDocument` script failed.** A browser built from a profile runs `preDocument` scripts when
  the page's context is created, before some APIs exist (all WebGPU interfaces). Their errors are
  collected in `window.__qarPreDocumentErrors`; touch late APIs only when the page first uses them,
  as `webgpu-redirect-capture@1` does.

## Users see black, stale or wrong views

| Symptom | Likely cause |
|---|---|
| Black or stale views while frame counters rise (WebGPU) | The page's own loop calls `getCurrentTexture()`: use the [redirect presenter](recipes.md#7-webgpu-redirect-presenter) |
| Every user sees the same view | One camera or one reused matrix array for every eye; a WebGPU uniform buffer written twice before one submit |
| The page's interface in a user's view | With `Engine: "webgl"` / `"webgpu"` a fixed-position panel outside the page region: move it in ([UI confinement](recipes.md#5-ui-confinement)) |
| Effects missing in the headset | Post-processing runs on the page's own frame only; users' views show the plain scene |
| Scene on its side | A Z-up scene: set `sceneFromRoom` ([library reference](library-reference.md#threejs-render-frame1)) |
| Scene too large or cut off | Content outside the app volume is clipped: lower `AppScale`, or `AppScaleByAddress` for pages in other units |
| Content updates feel slow but head motion is smooth | `RenderRate` (30 by default): raise it for fast animation ([render rate](recipes.md#6-continuous-rendering-and-renderrate)) |

## Thin lines look dashed or too thick

- **Dashed:** a packer keeps one texel per 2x2 block, so colour arrives without depth in half the
  blocks and is dropped. Use a kit packer, or keep the nearest depth of each block
  ([atlas packing](recipes.md#1-atlas-packing)). `check` warns (`depth-single-texel`), and
  `qaros-adaptation verify` measures it on the running source.
- **Dim:** alpha averaged over the block (`alpha-mean`): keep the largest coverage.
- **Thick or blocky:** `RenderScale` below 1 makes each eye pixel larger in the headset; use 1.0 for
  line-heavy content ([thin lines](recipes.md#2-thin-lines-msaa-and-renderscale)).

## Depth looks wrong while colour is right

- Linear or metric depth packed instead of device depth, or another near/far than the request's
  ([depth conventions](recipes.md#3-depth-conventions)).
- Background depth black: the pack does not saturate at the far plane.
- A logarithmic depth buffer is not supported for the users' views.
