// Generates the versioned Rust API reference (qar-streaming-rust) with rustdoc.
//
// `cargo doc` documents the `qaros` and `qaros-sys` crates for the Windows target, the runtime's
// platform, with the `vulkan` and `d3d11` features, so the reference shows every API whatever
// host builds the site. Only the target's standard library is needed (rustup adds it); nothing
// is linked.
//
// Versions follow the docs (scripts/docs-versions.js): the working copy is built from this
// checkout, and every released version from its `v<version>` tag, extracted with `git archive`,
// so nothing generated is committed. A release whose tag has no Rust binding gets no reference.
// The HTML is copied as is into static/api/qar-streaming-rust/<segment>/: the newest release at
// the root, the working copy under next/ (at the root while nothing is released), older releases
// under their version. Every page carries a bar naming its version and linking the others.
// rust-versions.json lists what was built; docusaurus.config.ts turns it into the navbar link.
const fs = require("fs");
const os = require("os");
const path = require("path");
const { execFileSync } = require("child_process");
const { releasedVersions, versionSegment, currentLabel } = require("./docs-versions.js");

const WEBSITE_DIR = path.join(__dirname, "..");
const REPO_ROOT = path.join(WEBSITE_DIR, "..", "..");
const RUST_SUBDIR = "qar-streaming-rust";
const TARGET = "x86_64-pc-windows-msvc";
const TARGET_ROOT = path.join(WEBSITE_DIR, ".rustdoc-target");
const OUT_DIR = path.join(WEBSITE_DIR, "static", "api", "qar-streaming-rust");
const URL_BASE = "/api/qar-streaming-rust";
const VERSIONS_JSON = path.join(WEBSITE_DIR, "rust-versions.json");

function git(args, options = {}) {
	return execFileSync("git", args, { cwd: REPO_ROOT, encoding: "utf8", ...options });
}

function tagHasRustBinding(tag) {
	try {
		git(["cat-file", "-e", `${tag}:${RUST_SUBDIR}/crates/qaros/Cargo.toml`], { stdio: "ignore" });
		return true;
	} catch {
		return false;
	}
}

// The released sources of the binding, extracted from the tag into a temporary folder.
function extractTag(tag) {
	const folder = fs.mkdtempSync(path.join(os.tmpdir(), "qaros-rustdoc-"));
	const archive = path.join(folder, "source.tar");
	git(["archive", "--format=tar", `--output=${archive}`, tag, RUST_SUBDIR]);
	execFileSync("tar", ["-xf", archive, "-C", folder], { stdio: "inherit" });
	return { folder, rustDir: path.join(folder, RUST_SUBDIR) };
}

function urlOf(segment) {
	return segment ? `${URL_BASE}/${segment}/` : `${URL_BASE}/`;
}

// The bar rustdoc puts above every page (`--html-before-content`).
function versionBar(build, builds) {
	const links = builds
		.map((other) =>
			other === build
				? `<strong>${other.label}</strong>`
				: `<a href="${urlOf(other.segment)}qaros/index.html">${other.label}</a>`
		)
		.join(" · ");
	return `<div style="padding:6px 12px;font-size:14px;border-bottom:1px solid #8884">QAROS Rust API ${build.label} · versions: ${links} · <a href="${build.guides}">Developer Guide</a></div>\n`;
}

function cargoDoc(rustDir, build, barFile) {
	const targetDir = path.join(TARGET_ROOT, build.name);
	execFileSync(
		"cargo",
		["doc", "--no-deps", "--locked", "--target", TARGET, "-p", "qaros", "-p", "qaros-sys", "--features", "qaros/vulkan,qaros/d3d11"],
		{
			cwd: rustDir,
			stdio: "inherit",
			env: { ...process.env, CARGO_TARGET_DIR: targetDir, RUSTDOCFLAGS: `--html-before-content ${barFile}` },
		}
	);
	const outDir = path.join(OUT_DIR, build.segment);
	fs.mkdirSync(outDir, { recursive: true });
	fs.cpSync(path.join(targetDir, TARGET, "doc"), outDir, { recursive: true });
}

// A redirect at the root when no release has a reference of its own there.
function writeRootRedirect(builds) {
	if (builds.some((build) => build.segment === "")) return;
	const target = `${urlOf(builds[0].segment)}qaros/index.html`;
	fs.writeFileSync(
		path.join(OUT_DIR, "index.html"),
		`<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0; url=${target}"><a href="${target}">QAROS Rust API</a>\n`
	);
}

function plannedBuilds() {
	const released = releasedVersions();
	const current = versionSegment("current", released);
	const builds = [
		{ name: "current", label: currentLabel(), segment: current, guides: `/docs/${current ? `${current}/` : ""}developer-guide/` },
	];
	for (const version of released) {
		const tag = `v${version}`;
		if (!tagHasRustBinding(tag)) {
			console.log(`docs:rust: ${tag} has no Rust binding (or is not fetched); no reference for ${version}.`);
			continue;
		}
		const segment = versionSegment(version, released);
		builds.push({ name: version, label: version, segment, tag, guides: `/docs/${segment ? `${segment}/` : ""}developer-guide/` });
	}
	return builds;
}

function main() {
	const builds = plannedBuilds();
	fs.rmSync(OUT_DIR, { recursive: true, force: true });
	fs.mkdirSync(TARGET_ROOT, { recursive: true });
	try {
		for (const build of builds) {
			const barFile = path.join(TARGET_ROOT, `bar-${build.name}.html`);
			fs.writeFileSync(barFile, versionBar(build, builds));
			if (!build.tag) {
				cargoDoc(path.join(REPO_ROOT, RUST_SUBDIR), build, barFile);
				continue;
			}
			const { folder, rustDir } = extractTag(build.tag);
			try {
				cargoDoc(rustDir, build, barFile);
			} finally {
				fs.rmSync(folder, { recursive: true, force: true });
			}
		}
	} catch (error) {
		console.error(
			"Rust API generation failed. It needs the latest stable Rust on PATH with the " +
				`${TARGET} target (\`rustup target add ${TARGET}\`), and the release tags fetched.`
		);
		throw error;
	}
	writeRootRedirect(builds);
	const versions = Object.fromEntries(builds.map((build) => [build.name, `${urlOf(build.segment)}qaros/index.html`]));
	fs.writeFileSync(VERSIONS_JSON, `${JSON.stringify(versions, null, "\t")}\n`);
	console.log(`docs:rust: ${builds.map((build) => `${build.label} -> ${urlOf(build.segment)}`).join(", ")}`);
}

main();
