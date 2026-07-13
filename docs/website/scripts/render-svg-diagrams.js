// Rasterizes the hand-authored diagram SVGs under assets/diagrams (the
// editable source, outside `static/` so it is never shipped as-is) into
// PNGs under static/img/diagrams (build output, gitignored). Docusaurus
// copies everything under `static/` verbatim into the built site, so
// keeping the SVG source out of that tree is what stops both the SVG and
// the PNG from shipping to the browser — doc pages only ever reference the
// PNG. Run via `npm run docs:diagrams` (also wired into `docs:generate` and
// the `docs:watch` file watcher).
const fs = require("fs");
const path = require("path");
const sharp = require("sharp");

// SVG user units -> output pixels. The source SVGs have no explicit
// width/height (only a viewBox), so sharp maps units to pixels at
// density/72; 120 yields ~1600px wide PNGs for the 960-wide diagrams, sharp
// at typical documentation display sizes.
const DENSITY = 120;

// Each group rasterizes the editable SVG sources under `assets/<dir>` (kept
// outside `static/` so the SVGs never ship) into PNGs under
// `static/img/<dir>` (build output, gitignored). Pages reference only the PNG.
// `maxWidth` caps the rasterized size for assets that don't need the full
// density-based resolution (e.g. logos displayed small).
const GROUPS = [
	{ source: "diagrams", output: "diagrams" },
	{ source: "brand", output: "brand", maxWidth: 512 },
];

async function renderOne(svgPath, outputDir, maxWidth) {
	const pngPath = path.join(
		outputDir,
		path.basename(svgPath).replace(/\.svg$/i, ".png")
	);
	let pipeline = sharp(svgPath, { density: DENSITY });
	if (maxWidth) {
		pipeline = pipeline.resize({ width: maxWidth, withoutEnlargement: true });
	}
	await pipeline.png().toFile(pngPath);
	console.log(
		`docs:diagrams: ${path.basename(svgPath)} -> ${path.relative(process.cwd(), pngPath)}`
	);
}

async function renderGroup(group) {
	const sourceDir = path.join(__dirname, "..", "assets", group.source);
	const outputDir = path.join(__dirname, "..", "static", "img", group.output);
	const entries = fs.existsSync(sourceDir) ? fs.readdirSync(sourceDir) : [];
	const svgFiles = entries.filter((name) => name.toLowerCase().endsWith(".svg"));

	if (svgFiles.length === 0) {
		console.log("docs:diagrams: no SVGs found under", sourceDir);
		return;
	}

	fs.mkdirSync(outputDir, { recursive: true });

	for (const name of svgFiles) {
		await renderOne(path.join(sourceDir, name), outputDir, group.maxWidth);
	}
}

async function main() {
	for (const group of GROUPS) {
		await renderGroup(group);
	}
}

main().catch((err) => {
	console.error("docs:diagrams failed:", err);
	process.exit(1);
});
