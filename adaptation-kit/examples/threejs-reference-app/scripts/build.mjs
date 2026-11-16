// Builds this application to static assets loadable straight off the filesystem -- the
// requirement `npm run dev`'s Vite server does not itself satisfy. Two things a plain `vite build`
// does not give, found empirically while getting the browser-and-GPU integration test to load this
// application at all:
//
// - Vite's own HTML generation always emits the entry point as `<script type="module">`, even when
//   the internal bundle format is configured as a classic script. A module script's dependency
//   graph is fetched the same way `fetch()`/XHR is, which Chromium (and this repository's pinned
//   CEF build) refuses across two distinct `file://` paths -- even ones in the same directory --
//   so the bundle silently never executes when the page is opened as a local file. A classic
//   script has no such restriction: it loads as a plain resource fetch, the same way an `<img>` or
//   `<link>` tag's URL does.
// - A root-relative asset path (Vite's default `base`) resolves against the filesystem root, not
//   the `dist` directory, once the page is no longer served from a domain root.
//
// So this script bypasses Vite's build/HTML pipeline entirely for the production bundle (`npm run
// dev` still uses Vite, where native ES modules are exactly what makes its dev server fast) and
// bundles the one entry point directly with esbuild as a single self-executing classic script, then
// copies `index.html` into `dist/`, rewriting its `<script>`/`<link>` tags to reference the built,
// relatively-pathed assets instead of the source ones.
import { build } from "esbuild";
import { readFileSync, writeFileSync, mkdirSync, rmSync, cpSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(scriptsDirectory, "..");
const distDirectory = path.join(appRoot, "dist");
const assetsDirectory = path.join(distDirectory, "assets");

rmSync(distDirectory, { recursive: true, force: true });
mkdirSync(assetsDirectory, { recursive: true });
cpSync(path.join(appRoot, "assets"), assetsDirectory, { recursive: true });

await build({
	entryPoints: [path.join(appRoot, "src", "main.js")],
	bundle: true,
	format: "iife",
	minify: true,
	outfile: path.join(assetsDirectory, "bundle.js"),
});

const style = readFileSync(path.join(appRoot, "src", "style.css"), "utf8");
writeFileSync(path.join(assetsDirectory, "style.css"), style, "utf8");

let html = readFileSync(path.join(appRoot, "index.html"), "utf8");
html = html
	.replace(
		'<link rel="stylesheet" href="./src/style.css" />',
		'<link rel="stylesheet" href="./assets/style.css" />'
	)
	.replace(
		'<script type="module" src="./src/main.js"></script>',
		'<script src="./assets/bundle.js"></script>'
	);
if (html.includes("./src/")) {
	throw new Error(
		"build.mjs: index.html still references a ./src/ path after rewriting -- update this " +
			"script's replacements to match a change made to index.html."
	);
}
writeFileSync(path.join(distDirectory, "index.html"), html, "utf8");

console.log(`Built ${distDirectory}`);
