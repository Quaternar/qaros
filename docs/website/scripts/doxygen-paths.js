const path = require("path");

// Kept out of the "doxygen/" tree's watched subfolders so it can't be
// confused with the per-project Doxyfile/xml-output folders that live there.
const TOOLS_DIR = path.join(__dirname, "..", "doxygen", ".tools");

function paths(release) {
	const installDir = path.join(TOOLS_DIR, release.version);
	return {
		installDir,
		archivePath: path.join(TOOLS_DIR, release.archiveName),
		binaryPath: path.join(installDir, release.binaryRelPath),
		stampPath: path.join(installDir, ".stamp"),
	};
}

module.exports = { TOOLS_DIR, paths };
