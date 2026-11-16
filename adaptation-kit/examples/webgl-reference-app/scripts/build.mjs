// Builds this application to static assets loadable straight off the filesystem -- see
// ../../threejs-reference-app/scripts/build.mjs's own doc comment for the two `vite build` defects
// (a `type="module"` entry point refused across two distinct `file://` paths by this repository's
// pinned CEF build, and a root-relative default `base`) this script exists to route around, found
// while getting that application's own static assets to load as `file://`. Both apply here unchanged,
// so this file mirrors that one rather than rediscovering the same fix: esbuild bundles the one entry
// point directly as a single self-executing classic script, then `index.html` is copied with its
// `<script>`/`<link>` tags rewritten to reference it and the copied stylesheet by relative path.
// `npm run dev` still uses Vite, where native ES modules are exactly what makes it fast.
import { build } from "esbuild";
import { readFileSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(scriptsDirectory, "..");
const distDirectory = path.join(appRoot, "dist");
const assetsDirectory = path.join(distDirectory, "assets");

rmSync(distDirectory, { recursive: true, force: true });
mkdirSync(assetsDirectory, { recursive: true });

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
