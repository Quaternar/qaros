// Freezes the docs of a release: `npm run docs:version -- X.Y.Z`.
//
// Snapshots both docs instances in one go, so they stay in lockstep
// (scripts/docs-versions.js):
//   1. regenerates the C and C# API references (docs:generate), with C# source
//      links pinned to the current qaros commit instead of `main`,
//   2. `docusaurus docs:version` (guides -> versioned_docs/) and
//      `docusaurus docs:version:api` (references -> api_versioned_docs/),
//   3. points the snapshot's `@site/api/...` imports (the C tutorials embed
//      generated pages) at the frozen reference,
//   4. moves the snapshot's images into versioned_assets/, stored once by
//      content: only an image no earlier version has adds a file,
//   5. clears the Docusaurus cache, which does not notice new versions.
// Commit versioned_assets/, versioned_docs/, versioned_sidebars/, versions.json
// and their api_* counterparts. The version defaults to qaros/VERSION.
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");
const { WEBSITE_DIR, plannedVersion, releasedVersions } = require("./docs-versions.js");

function sh(command, env = {}) {
	console.log(`> ${command}`);
	execSync(command, { cwd: WEBSITE_DIR, stdio: "inherit", env: { ...process.env, ...env } });
}

function markdownFiles(dir) {
	return fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
		const full = path.join(dir, entry.name);
		if (entry.isDirectory()) return markdownFiles(full);
		return /\.mdx?$/.test(entry.name) ? [full] : [];
	});
}

function freezeApiImports(version) {
	const dir = path.join(WEBSITE_DIR, "versioned_docs", `version-${version}`);
	let count = 0;
	for (const file of markdownFiles(dir)) {
		const content = fs.readFileSync(file, "utf8");
		const frozen = content.replace(/(["'])@site\/api\//g, `$1@site/api_versioned_docs/version-${version}/`);
		if (frozen !== content) {
			fs.writeFileSync(file, frozen);
			count++;
		}
	}
	console.log(`Pointed the API imports of ${count} page(s) at api_versioned_docs/version-${version}/.`);
}

// Page assets (diagrams, images) of every frozen version live once, by content,
// in versioned_assets/<name>-<hash>.<ext>: a snapshot adds only the assets whose
// content no earlier version has, and its pages point at the shared file. The
// working copy keeps its own assets under docs/ (relative paths, see README).
const ASSET_EXTENSIONS = /\.(png|jpe?g|gif|svg|webp|avif)$/i;
const ASSET_STORE = path.join(WEBSITE_DIR, "versioned_assets");
// A relative asset reference in a page: Markdown image/link target, HTML src/href, require().
const ASSET_REFERENCE = /(\]\(|\b(?:src|href)=["']|require\(["'])(\.{1,2}\/[^)"'\s]+)/g;

function allFiles(dir) {
	if (!fs.existsSync(dir)) return [];
	return fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
		const full = path.join(dir, entry.name);
		return entry.isDirectory() ? allFiles(full) : [full];
	});
}

function storeAsset(file) {
	const hash = require("crypto").createHash("sha256").update(fs.readFileSync(file)).digest("hex").slice(0, 12);
	const ext = path.extname(file);
	const stored = path.join(ASSET_STORE, `${path.basename(file, ext)}-${hash}${ext}`);
	if (!fs.existsSync(stored)) {
		fs.mkdirSync(ASSET_STORE, { recursive: true });
		fs.copyFileSync(file, stored);
		console.log(`New asset ${path.relative(WEBSITE_DIR, stored)}`);
	}
	return stored;
}

function toPosixRelative(fromDir, to) {
	const relative = path.relative(fromDir, to).split(path.sep).join("/");
	return relative.startsWith(".") ? relative : `./${relative}`;
}

function removeEmptyDirs(dir) {
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		if (entry.isDirectory()) removeEmptyDirs(path.join(dir, entry.name));
	}
	if (fs.readdirSync(dir).length === 0) fs.rmdirSync(dir);
}

function dedupeAssets(snapshotDir) {
	const moved = new Map(); // absolute path in the snapshot -> stored file
	for (const file of allFiles(snapshotDir).filter((f) => ASSET_EXTENSIONS.test(f))) {
		moved.set(path.resolve(file), storeAsset(file));
	}
	for (const page of markdownFiles(snapshotDir)) {
		const content = fs.readFileSync(page, "utf8");
		const rewritten = content.replace(ASSET_REFERENCE, (whole, prefix, target) => {
			const [file, suffix = ""] = target.split(/(?=[?#])/);
			const stored = moved.get(path.resolve(path.dirname(page), file));
			return stored ? `${prefix}${toPosixRelative(path.dirname(page), stored)}${suffix}` : whole;
		});
		if (rewritten !== content) fs.writeFileSync(page, rewritten);
	}
	for (const file of moved.keys()) fs.rmSync(file);
	removeEmptyDirs(snapshotDir);
	console.log(`${path.relative(WEBSITE_DIR, snapshotDir)}: ${moved.size} asset(s) moved to versioned_assets/.`);
}

// Drops stored assets no frozen page references any more (a version was removed).
function pruneAssetStore() {
	const pages = ["versioned_docs", "api_versioned_docs"]
		.map((dir) => path.join(WEBSITE_DIR, dir))
		.filter((dir) => fs.existsSync(dir))
		.flatMap(markdownFiles);
	const used = new Set();
	for (const page of pages) {
		for (const [, , target] of fs.readFileSync(page, "utf8").matchAll(ASSET_REFERENCE)) {
			used.add(path.resolve(path.dirname(page), target.split(/(?=[?#])/)[0]));
		}
	}
	for (const stored of allFiles(ASSET_STORE)) {
		if (!used.has(path.resolve(stored))) {
			fs.rmSync(stored);
			console.log(`Pruned unused asset ${path.relative(WEBSITE_DIR, stored)}`);
		}
	}
}

function freeze(version) {
	if (!version || !/^\d+\.\d+\.\d+$/.test(version)) {
		console.error("Usage: npm run docs:version -- X.Y.Z  (defaults to qaros/VERSION)");
		process.exit(1);
	}
	if (releasedVersions().includes(version)) {
		console.error(`Docs version ${version} already exists.`);
		process.exit(1);
	}
	if (plannedVersion() && plannedVersion() !== version) {
		console.warn(`Warning: snapshotting ${version}, but qaros/VERSION says ${plannedVersion()}.`);
	}

	const sourceRef = execSync("git rev-parse HEAD", { cwd: WEBSITE_DIR }).toString().trim();
	sh("npm run docs:generate", { QAROS_SOURCE_REF: sourceRef });
	sh(`npx docusaurus docs:version ${version}`, { QAROS_DOCS_SNAPSHOT_IN_PROGRESS: "1" });
	sh(`npx docusaurus docs:version:api ${version}`, { QAROS_DOCS_SNAPSHOT_IN_PROGRESS: "1" });
	freezeApiImports(version);
	dedupeAssets(path.join(WEBSITE_DIR, "versioned_docs", `version-${version}`));
	dedupeAssets(path.join(WEBSITE_DIR, "api_versioned_docs", `version-${version}`));
	pruneAssetStore();
	sh("npx docusaurus clear");
	console.log(`Docs ${version} frozen. Commit versioned_assets/, versioned_docs/, versioned_sidebars/, versions.json, api_versioned_docs/, api_versioned_sidebars/ and api_versions.json.`);
}

// `--prune-assets` alone: after removing a version, drop the stored assets only it used.
if (process.argv[2] === "--prune-assets") {
	pruneAssetStore();
} else {
	freeze(process.argv[2] ?? plannedVersion());
}
