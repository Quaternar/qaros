"""Read back the kit's WebGL depth pack shader (webgl-atlas-pack@1, used by three.js and raw
WebGL) in a live Web App Source's WebGL2 context.

Talks to the Visualizer MCP (`POST /mcp`, JSON-RPC `tools/call` of `devtools_send`), which needs
the MCP endpoint enabled and the Visualizer in Developer UI mode. `qaros-adaptation verify` runs
this check together with the depth, thin-line and alpha coverage checks.

    python -I verify-depth.py --instance <id> [--port 17872] [--host 127.0.0.1] [--library <file>]
"""
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from qaros_adaptation import verify  # noqa: E402

DEFAULT_LIBRARY = Path(__file__).resolve().parent.parent / "lib" / "webgl-atlas-pack@1.js"


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--instance", required=True)
    # The Visualizer binds QAR_MCP_SERVER_PORT, else QAR_VIZ_AUTOMATION_PORT, else 17872.
    parser.add_argument("--port", type=int, default=verify.default_port())
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--library", type=Path, default=DEFAULT_LIBRARY, help="the library (or pinned copy) to read the shaders from")
    options = parser.parse_args()
    try:
        actual = verify.pack_readback(verify.Mcp(options.host, options.port), options.instance, options.library)
    except (verify.VerifyError, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(f"near/mid/far depth packing OK: {actual}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
