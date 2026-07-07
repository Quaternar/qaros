// Pinned, hash-verified Doxygen release metadata per host platform.
// URLs point at doxygen.nl directly (same source used by the Quaternar
// monorepo's HostToolsWindows.cmake for its own bundled Doxygen), not the
// sourceforge-mirror-based npm "doxygen" wrapper package, which has been
// unreliable (redirect chains that hang) for this project in the past.
const path = require("path");

const VERSION = require("./doxygen-version");

const RELEASES = {
	win32: {
		archiveName: `doxygen-${VERSION}.windows.x64.bin.zip`,
		sha256: "03f98f585acee18df0575262feffccc8a93a5aeacd4ee8c21872aeef12532244",
		// The Windows archive has no top-level folder.
		binaryRelPath: "doxygen.exe",
	},
	linux: {
		archiveName: `doxygen-${VERSION}.linux.bin.tar.gz`,
		sha256: "dda773bdc62384b7d796fe8b6c5029daad72483e4c8ad4abf6ee9fb98b649388",
		binaryRelPath: path.join(`doxygen-${VERSION}`, "bin", "doxygen"),
	},
};

function getRelease() {
	const release = RELEASES[process.platform];
	if (!release) {
		return null;
	}
	return {
		version: VERSION,
		url: `https://www.doxygen.nl/files/${release.archiveName}`,
		archiveName: release.archiveName,
		sha256: release.sha256,
		binaryRelPath: release.binaryRelPath,
	};
}

module.exports = { getRelease, VERSION };
