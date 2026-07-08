# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

## Prerequisites

- Node.js >= 18 (see `engines` in `package.json`).
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

### Diagrams (SVG source, PNG on the site)

The hand-drawn diagrams live as SVG under `assets/diagrams/` — deliberately
*outside* `static/`, which Docusaurus copies into the built site verbatim, so
the editable source is never itself shipped to the browser. Doc pages embed
the `.png` rasterized from each SVG instead, since Docusaurus-hosted static
sites don't reliably serve/display raw SVGs everywhere they're linked.
`npm run docs:diagrams` (via `sharp`, a dependency already declared in
`package.json`) renders every `assets/diagrams/*.svg` into a same-named
`.png` under `static/img/diagrams/`; that output directory is gitignored in
full and regenerated on every build — never edit or commit a `.png` there
directly. It runs automatically as part of `npm run docs:generate`, so
`npm run start` / `npm run build` / `npm run dev` all keep the PNGs in sync
with their SVG sources — edit the `.svg` under `assets/diagrams/` and rerun
one of those (or just `npm run docs:diagrams` directly).

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
and `static/img/diagrams/*.png` through `npm run docs:generate`.

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
