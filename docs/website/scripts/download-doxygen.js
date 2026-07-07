// Downloads, hash-verifies, and extracts the pinned Doxygen release into
// doxygen/.tools/<version>/, so the docs build is self-contained and does
// not depend on any Doxygen found on PATH or in the Quaternar monorepo's
// build/tools/. Idempotent via a stamp file, mirroring the
// download_and_verify/extract_and_stamp pattern in
// qar-streaming/cmake/HostToolsWindows.cmake.
const fs = require("fs");
const path = require("path");
const https = require("https");
const crypto = require("crypto");
const { execFileSync } = require("child_process");
const { getRelease, VERSION } = require("./doxygen-release");
const { TOOLS_DIR, paths } = require("./doxygen-paths");

function download(url, dest) {
	return new Promise((resolve, reject) => {
		const file = fs.createWriteStream(dest);
		function request(currentUrl, redirectsLeft) {
			https
				.get(currentUrl, (response) => {
					if (
						response.statusCode >= 300 &&
						response.statusCode < 400 &&
						response.headers.location
					) {
						response.resume();
						if (redirectsLeft <= 0) {
							reject(new Error(`Too many redirects downloading ${url}`));
							return;
						}
						request(response.headers.location, redirectsLeft - 1);
						return;
					}
					if (response.statusCode !== 200) {
						reject(new Error(`Download failed with status ${response.statusCode} for ${currentUrl}`));
						return;
					}
					response.pipe(file);
				})
				.on("error", reject);
		}
		file.on("finish", () => file.close(resolve));
		file.on("error", reject);
		request(url, 5);
	});
}

function sha256File(filePath) {
	return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

async function main() {
	const release = getRelease();
	if (!release) {
		console.warn(
			`No pinned Doxygen ${VERSION} release for platform "${process.platform}". ` +
				"Install Doxygen >= 1.9.3 yourself and make sure it is on PATH " +
				"(older versions don't emit Doxyfile.xml, which doxygen2docusaurus requires)."
		);
		return;
	}

	const { installDir, archivePath, binaryPath, stampPath } = paths(release);

	if (
		fs.existsSync(stampPath) &&
		fs.readFileSync(stampPath, "utf8") === release.sha256 &&
		fs.existsSync(binaryPath)
	) {
		console.log(`Doxygen ${release.version} already installed at ${binaryPath}`);
		return;
	}

	fs.mkdirSync(TOOLS_DIR, { recursive: true });

	if (!fs.existsSync(archivePath) || sha256File(archivePath) !== release.sha256) {
		console.log(`Downloading Doxygen ${release.version} from ${release.url} ...`);
		await download(release.url, archivePath);
	}

	const actualHash = sha256File(archivePath);
	if (actualHash !== release.sha256) {
		fs.rmSync(archivePath, { force: true });
		throw new Error(
			`Hash mismatch for ${archivePath}: expected ${release.sha256}, got ${actualHash}. ` +
				"Removed corrupted download, please retry."
		);
	}

	fs.rmSync(installDir, { recursive: true, force: true });
	fs.mkdirSync(installDir, { recursive: true });

	// Pass a relative archive path: GNU tar misparses an absolute Windows
	// path like "C:\foo\bar.zip" as a "host:path" remote-archive spec
	// because of the drive-letter colon.
	const relativeArchivePath = path.relative(installDir, archivePath);
	console.log(`Extracting ${archivePath} to ${installDir} ...`);
	execFileSync("tar", ["-xf", relativeArchivePath], { cwd: installDir, stdio: "inherit" });

	if (!fs.existsSync(binaryPath)) {
		throw new Error(`Extraction of ${archivePath} did not produce expected binary at ${binaryPath}`);
	}
	if (process.platform !== "win32") {
		fs.chmodSync(binaryPath, 0o755);
	}

	fs.writeFileSync(stampPath, release.sha256);
	console.log(`Doxygen ${release.version} ready at ${binaryPath}`);
}

main().catch((error) => {
	console.error(error.message || error);
	process.exitCode = 1;
});
