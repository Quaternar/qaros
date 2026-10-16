// Docs versioning shared by docusaurus.config.ts, the link rewriter below and
// scripts/version-docs.js.
//
// Both docs plugin instances (guides `docs`, references `api`) are versioned in
// lockstep: `npm run docs:version` snapshots them together, so versions.json and
// api_versions.json always list the same releases. The newest one is served at
// the root (/docs/, /api/), the unreleased working copy ("current", built from
// main on every push) at /dev/ once a release exists, older releases at /<version>/.
const fs = require("fs");
const path = require("path");

const WEBSITE_DIR = path.join(__dirname, "..");
const VERSION_FILE = path.join(WEBSITE_DIR, "..", "..", "VERSION");
// Content folder of each plugin instance and the URL base it is served under.
const INSTANCES = [
	{ id: "default", route: "docs", current: "docs", versioned: "versioned_docs", versionsFile: "versions.json" },
	{ id: "api", route: "api", current: "api", versioned: "api_versioned_docs", versionsFile: "api_versions.json" },
];

function readJson(file, fallback) {
	return fs.existsSync(file) ? JSON.parse(fs.readFileSync(file, "utf8")) : fallback;
}

// The next planned release, from qaros/VERSION; undefined when the file is missing.
function plannedVersion() {
	if (!fs.existsSync(VERSION_FILE)) return undefined;
	return fs.readFileSync(VERSION_FILE, "utf8").trim() || undefined;
}

// Released versions, newest first. Throws if the two instances drifted apart.
function releasedVersions() {
	const [docs, api] = INSTANCES.map((instance) => readJson(path.join(WEBSITE_DIR, instance.versionsFile), []));
	// scripts/version-docs.js snapshots one instance after the other.
	if (JSON.stringify(docs) !== JSON.stringify(api) && !process.env.QAROS_DOCS_SNAPSHOT_IN_PROGRESS) {
		throw new Error(
			`versions.json ${JSON.stringify(docs)} and api_versions.json ${JSON.stringify(api)} differ. ` +
				"Both docs instances must be versioned together with `npm run docs:version`."
		);
	}
	return docs;
}

// URL segment of the working copy once a release exists; it was /next/ before,
// which the config redirects (docusaurus.config.ts).
const DEV_SEGMENT = "dev";
const LEGACY_DEV_SEGMENT = "next";

// The working copy is what main holds right now, not a planned release: qaros/VERSION
// keeps naming the last release until the bump, so it would label it as that release.
function currentLabel() {
	return "Dev (main)";
}

// Version options of the working copy, shared by both docs instances.
function currentVersionOptions(released = releasedVersions()) {
	const options = { label: currentLabel() };
	return released.length ? { ...options, path: DEV_SEGMENT, banner: "unreleased" } : options;
}

// URL segment a version is served under ("" for the root).
function versionSegment(version, released = releasedVersions()) {
	if (version === "current") return released.length ? DEV_SEGMENT : "";
	return version === released[0] ? "" : version;
}

// Client redirects from the working copy's old /next/ URLs to /dev/.
function legacyDevRedirects(existingPath) {
	const match = new RegExp(`^/(${INSTANCES.map((instance) => instance.route).join("|")})/${DEV_SEGMENT}(/.*)?$`).exec(existingPath);
	return match ? [`/${match[1]}/${LEGACY_DEV_SEGMENT}${match[2] ?? ""}`] : undefined;
}

// Which version a content file belongs to, from its path; undefined outside docs content.
function versionOfFile(filePath) {
	const relative = path.relative(WEBSITE_DIR, filePath).split(path.sep);
	for (const instance of INSTANCES) {
		if (relative[0] === instance.current) return "current";
		const match = relative[0] === instance.versioned && /^version-(.+)$/.exec(relative[1] ?? "");
		if (match) return match[1];
	}
	return undefined;
}

// Rehype plugin: absolute /docs/... and /api/... links written in a page point at
// the root, which serves the newest release. Point them at the version the page
// belongs to instead, so a release's guides link its own API reference and the
// unreleased guides link the unreleased reference. Relative .md links need no
// help: Docusaurus resolves them inside the page's own version.
function rehypeVersionedLinks() {
	const released = releasedVersions();
	const routes = INSTANCES.map((instance) => instance.route).join("|");
	const absolute = new RegExp(`^/(${routes})(?=/|$|#)(/?)(.*)$`);
	return (tree, file) => {
		const version = file.path && versionOfFile(file.path);
		const segment = version && versionSegment(version, released);
		if (!segment) return;
		// Docusaurus has already turned .md links into URLs of the right version.
		const rewrite = (url) =>
			url.replace(absolute, (whole, route, __, rest) => (rest === segment || rest.startsWith(`${segment}/`) ? whole : `/${route}/${segment}/${rest}`));
		walk(tree, (node) => {
			if (node.type === "element" && node.tagName === "a" && typeof node.properties?.href === "string") {
				node.properties.href = rewrite(node.properties.href);
			}
			if ((node.type === "mdxJsxFlowElement" || node.type === "mdxJsxTextElement") && ["a", "Link"].includes(node.name)) {
				for (const attribute of node.attributes ?? []) {
					if (["href", "to"].includes(attribute.name) && typeof attribute.value === "string") {
						attribute.value = rewrite(attribute.value);
					}
				}
			}
		});
	};
}

// Remark plugin, run before Docusaurus resolves images: a page image must be a
// relative path inside the docs folder (e.g. ../img/diagrams/x.png). An
// absolute /img/... path is served from static/, which is not versioned, so an
// older release would silently show the newest image.
function remarkVersionedImages() {
	return (tree, file) => {
		walk(tree, (node) => {
			if (node.type === "image" && typeof node.url === "string" && node.url.startsWith("/")) {
				throw new Error(
					`${file.path}: image '${node.url}' is an absolute path into static/, which is not ` +
						"versioned. Reference it relative to the page, from inside the docs folder " +
						"(diagrams: <relative>/img/diagrams/<name>.png)."
				);
			}
		});
	};
}

function walk(node, visit) {
	visit(node);
	for (const child of node.children ?? []) walk(child, visit);
}

module.exports = {
	INSTANCES,
	WEBSITE_DIR,
	plannedVersion,
	releasedVersions,
	currentLabel,
	currentVersionOptions,
	versionSegment,
	legacyDevRedirects,
	versionOfFile,
	rehypeVersionedLinks,
	remarkVersionedImages,
};
