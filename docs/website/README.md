# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

## Prerequisites

- Node.js >= 18 (see `engines` in `package.json`).
- .NET SDK >= 8 (C# API reference, see below).
- Stable Rust with the `x86_64-pc-windows-msvc` target (Rust API reference, see below).
- A `tar` binary on `PATH` that can extract `.zip` (Windows 10/11 ships one
  at `%SystemRoot%\System32\tar.exe`) and `.tar.gz` (any Linux/macOS system
  `tar`). Used only to unpack the Doxygen archive below.

Doxygen itself does **not** need to be installed or on `PATH`. `npm install`
downloads a pinned, hash-verified Doxygen release (see
`scripts/doxygen-release.js`) straight from doxygen.nl into
`doxygen/.tools/<version>/` and `npm run build` runs that copy explicitly
(`scripts/run-doxygen.js`) — the repo is standalone and does not read
anything from a sibling Quaternar monorepo checkout or rely on whatever
`doxygen` happens to resolve to on `PATH`. This replaced an earlier setup
that shelled out to a bare `doxygen` and downloaded a fallback via the `doxygen`
npm wrapper package: that wrapper's downloaded copy was never wired onto
`PATH` (so it didn't actually help), its pinned version (1.9.1) predates
Doxygen emitting `Doxyfile.xml` in the XML output (added in 1.9.3, required
by `doxygen2docusaurus`), and the wrapper itself (sourceforge-mirror
redirects, bundled `dmg`/`ftp` deps) has been unreliable/prone to hanging.
CI still separately `apt-get install`s Doxygen, which is now redundant but
harmless.

To point at a different Doxygen binary (e.g. on an unsupported platform),
set `QAROS_DOXYGEN_BIN=/path/to/doxygen` before `npm run build`.

### C# API reference

- Needs the .NET SDK 8+ on `PATH`. Nothing native is built.
- `npm run docs:csharp` (part of `docs:generate`) runs `scripts/run-docfx.js`:
  `dotnet tool restore` installs the DocFX pinned in `dotnet-tools.json`, `docfx metadata`
  (`docfx/qar-streaming-sharp/docfx.json`) reads `qar-streaming-sharp` through Roslyn into
  YAML (gitignored `docfx/qar-streaming-sharp/_yaml/`), and the script renders that model into
  the gitignored `api/qar-streaming-sharp/`.
- Rendered by our script, not DocFX's Markdown template or DocFxMarkdownGen: DocFX's
  YAML model is its stable input format, so layout, anchors and links are ours to control;
  DocFxMarkdownGen (pre-1.0) drops generic arguments (`Result<T>` for `Result<PeerId>`)
  and has no summaries on namespace pages.
- Landing page `/api/qar-streaming-sharp/`: hand-written intro
  `docfx/qar-streaming-sharp/overview.md`, then one table of types per area. Areas and
  their order: `docfx/qar-streaming-sharp/areas.json`. A type no area claims lands
  under "Other" with a build warning. The sidebar uses the same grouping.
- Type pages: summary, declaration, inheritance, then a section per member kind
  (overview table from 3 members), each member a heading with an explicit id; enum
  values are one table with an anchor per value. Crefs become relative `.md` links,
  BCL types link to learn.microsoft.com, source links go to GitHub (`main`, or the
  commit a release snapshot was taken from via `QAROS_SOURCE_REF`).
- Only the public `Quaternar.Qaros` and `Quaternar.Qaros.Mathematics` namespaces are documented; `Quaternar.Qaros.Interop` is internal.
- Generated from the `net8.0` target, so APIs that exist only on .NET 8 are listed too.
- Page content comes from the `///` XML doc comments. A public type or member without
  one has an empty description.

### Rust API reference

- Needs the latest stable Rust on `PATH` with the `x86_64-pc-windows-msvc` target
  (`rustup target add x86_64-pc-windows-msvc`); nothing is linked.
- `npm run docs:rust` (part of `docs:generate`) runs `scripts/run-rustdoc.js`: `cargo doc`
  documents the `qaros` and `qaros-sys` crates for the Windows target (the runtime's
  platform) with the `vulkan` and `d3d11` features, and copies rustdoc's own HTML into the
  gitignored `static/api/qar-streaming-rust/`. The navbar's *Rust API* link reads the
  generated `rust-versions.json`.
- Versioned like the docs, but built, never committed (~14 MB per version): the working copy
  from this checkout, each released version from its `vX.Y.Z` tag (`git archive`), so a
  build needs the tags fetched (CI checks out with `fetch-depth: 0`). Paths: newest release at
  `/api/qar-streaming-rust/`, the working copy at `.../next/` (at the root while nothing is
  released with a Rust binding, through a redirect), older releases at `.../X.Y.Z/`. A
  release whose tag has no Rust binding (0.9.0) gets none, and its pages link the working
  copy's reference.
- Every rustdoc page carries a bar (`--html-before-content`) naming its version and linking
  the other versions and the Developer Guide.

### Versioned docs

- Both docs instances are versioned in lockstep: guides (`docs/` ->
  `versioned_docs/`, `versions.json`) and the generated C and C# references (`api/` ->
  `api_versioned_docs/`, `api_versions.json`). The two lists must match, the config
  refuses to load otherwise (`scripts/docs-versions.js`).
- No version is released yet: the site has only the working copy ("current"),
  labelled `<qaros/VERSION> (next)` (`Next` if `VERSION` is missing).
- After a release: newest release at `/docs/`, `/api/`; the working copy at
  `/docs/next/`, `/api/next/`; older releases at `/docs/X.Y.Z/`, `/api/X.Y.Z/`.
- Release step, from `docs/website`, before bumping `qaros/VERSION`:
  `npm run docs:version -- X.Y.Z` (version defaults to `qaros/VERSION`). It
  regenerates both API references with C# source links pinned to the current qaros
  commit, snapshots both instances, points the snapshot's `@site/api/...` imports
  (C tutorials) at the frozen reference and clears the Docusaurus cache. Commit
  `versioned_docs/`, `versioned_sidebars/`, `versions.json` and their `api_*` twins
  (~6 MB per release, mostly the C reference).
- Absolute `/docs/...` and `/api/...` links in pages are rewritten at build time to the
  page's own version (`rehypeVersionedLinks`), so a release's guides link its own API
  reference. Relative `.md` links are resolved per version by Docusaurus already.
- Navbar: one version dropdown (`custom-docsVersionDropdown`,
  `src/components/DocsVersionDropdown`) switches whichever instance the page belongs
  to and makes that version preferred in the other one. Navbar links are `doc` /
  `docSidebar` items so they follow the browsed version; a plain `to:` (dropdown
  headers, redirects) always opens the newest release.
- Page images are versioned with their pages: they live inside `docs/` and are referenced by
  relative path, so `docs:version` copies them into the snapshot. An absolute `/img/...` image in
  a page fails the build (`remarkVersionedImages`): `static/` is not versioned, so a release would
  show the newest image. `static/` holds only site chrome (logos, favicons, homepage art).
- Remove a version: delete its folders and its entry in both `versions.json` files.

### Diagrams (SVG source, PNG on the site)

- Source: SVG under `assets/diagrams/`, outside `docs/` and `static/`, so it never ships.
- Pages embed a PNG instead (raw SVG does not display reliably everywhere).
- `npm run docs:diagrams` (part of `docs:generate`, so `start`, `build` and `dev` run it) renders
  each SVG to `docs/img/diagrams/<name>.png`. Gitignored and regenerated: edit the SVG, never the PNG.
- Reference it relative to the page: `![Mixer data flow](../img/diagrams/mixer-flow.png)`.
- `docs:version` renders first, snapshots `docs/`, then moves every image of the snapshot into
  `versioned_assets/<name>-<content hash>.<ext>` (committed) and points its pages there. An
  image whose content an earlier version already has is not copied again; only changed images
  add a file. A release therefore keeps its diagrams without duplicating unchanged ones.
- After removing a version, `npm run docs:prune-assets` deletes stored images no frozen page
  uses any more (`docs:version` prunes too).

The SVGs are tracked in Git LFS, via the repo-wide `*.svg`/`*.png` rule in
the top-level `.gitattributes` (see `qaros/.gitattributes`).

## Installation

```bash
npm install
```

`npm ci` may fail on a lockfile mismatch depending on when `package-lock.json`
was last regenerated — prefer `npm install` for local work.

## Local Development

```bash
npm run start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

`npm run start` runs the Doxygen -> XML -> Markdown conversion and the SVG ->
PNG diagram rasterization once up front, then leaves them alone — editing a
hand-written `.md`/`.mdx` page under `docs/` hot-reloads, but editing
`qar-streaming-c/include/qar_streaming.h`, `qar-streaming-c/examples/*.c`,
the Doxyfile, `doxygen2docusaurus.json`, or a diagram `.svg` under
`assets/diagrams/` does not, since those only feed the generated `api/` tree
and `docs/img/diagrams/*.png` through `npm run docs:generate`.

For that, use:

```bash
npm run dev
```

This runs `docusaurus start` and a file watcher (`nodemon-doxygen.json`) side
by side: any change under `qar-streaming-c/include`, `qar-streaming-c/examples`,
the Doxyfile, `doxygen2docusaurus.json`, or `assets/diagrams/*.svg`
re-runs `docs:generate` (Doxygen -> XML -> Markdown, plus the diagram PNGs),
and Docusaurus's own dev server then picks up the regenerated `api/` markdown
and PNGs and hot-reloads as usual. Run `npm run docs:watch` on its own if you
only want the watcher (e.g. alongside a separately running `docusaurus
start`).

## Build

```bash
npm run build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

## Deployment

Using SSH:

```bash
USE_SSH=true npm run deploy
```

Not using SSH:

```bash
GIT_USER=<Your GitHub username> npm run deploy
```

If you are using GitHub pages for hosting, this command is a convenient way to build the website and push to the `gh-pages` branch.
