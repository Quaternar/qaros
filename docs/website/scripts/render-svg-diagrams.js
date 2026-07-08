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

const SOURCE_DIR = path.join(__dirname, "..", "assets", "diagrams");
const OUTPUT_DIR = path.join(__dirname, "..", "static", "img", "diagrams");

// SVG user units -> output pixels. The source diagrams have no explicit
// width/height (only a viewBox), so sharp maps units to pixels at
// density/72; 120 yields ~1600px wide PNGs for the 960-wide diagrams, sharp
// at typical documentation display sizes.
const DENSITY = 120;

async function renderOne(svgPath, outputDir) {
	const pngPath = path.join(
		outputDir,
		path.basename(svgPath).replace(/\.svg$/i, ".png")
	);
	await sharp(svgPath, { density: DENSITY }).png().toFile(pngPath);
	console.log(
		`docs:diagrams: ${path.basename(svgPath)} -> ${path.relative(process.cwd(), pngPath)}`
	);
}

async function main() {
	const entries = fs.existsSync(SOURCE_DIR) ? fs.readdirSync(SOURCE_DIR) : [];
	const svgFiles = entries.filter((name) => name.toLowerCase().endsWith(".svg"));

	if (svgFiles.length === 0) {
		console.log("docs:diagrams: no SVGs found under", SOURCE_DIR);
		return;
	}

	fs.mkdirSync(OUTPUT_DIR, { recursive: true });

	for (const name of svgFiles) {
		await renderOne(path.join(SOURCE_DIR, name), OUTPUT_DIR);
	}
}

main().catch((err) => {
	console.error("docs:diagrams failed:", err);
	process.exit(1);
});
