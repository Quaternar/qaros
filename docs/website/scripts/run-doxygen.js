// Runs the Doxygen binary provisioned by download-doxygen.js against the
// qar-streaming-c Doxyfile. Doxygen resolves the Doxyfile's relative paths
// (OUTPUT_DIRECTORY, INPUT, ...) against the current working directory, so
// this chdirs into the Doxyfile's folder first rather than passing an
// absolute config path from the website root.
const fs = require("fs");
const path = require("path");
const { execFileSync } = require("child_process");
const { getRelease } = require("./doxygen-release");
const { paths } = require("./doxygen-paths");

function resolveDoxygenBinary() {
	if (process.env.QAROS_DOXYGEN_BIN) {
		return process.env.QAROS_DOXYGEN_BIN;
	}
	const release = getRelease();
	if (release) {
		const { binaryPath } = paths(release);
		if (fs.existsSync(binaryPath)) {
			return binaryPath;
		}
	}
	console.warn(
		'No self-provisioned Doxygen binary found; falling back to "doxygen" on PATH. ' +
			"Make sure it is >= 1.9.3 (older versions don't emit Doxyfile.xml in their XML " +
			"output, which doxygen2docusaurus requires)."
	);
	return "doxygen";
}

const doxygenBin = resolveDoxygenBinary();
const doxyfileDir = path.join(__dirname, "..", "doxygen", "qar-streaming-c");

execFileSync(doxygenBin, ["Doxyfile"], { cwd: doxyfileDir, stdio: "inherit" });
