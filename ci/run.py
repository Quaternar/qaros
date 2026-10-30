#!/usr/bin/env python3
"""Every check and build of the qaros GitHub workflows, runnable locally from any folder:

    python ci/run.py rust [--step fmt,clippy,test,tutorials,guide,e2e] [--e2e]
    python ci/run.py csharp [--filter <dotnet test filter>]
    python ci/run.py c [--filter <gtest filter>] [--timeout <s>]
    python ci/run.py capi                    # c, csharp and the Rust e2e tests on one native folder
    python ci/run.py vulkan-cube [--strict]
    python ci/run.py docs

Native files (C API runtime, test counterparts, the C test executable) come from environment
variables, never from a build this script starts:

    QAROS_NATIVE_BIN        qar-streaming-c, qar-streaming-c-ext and every library they load
    QAROS_HUB_TEST_EXT_BIN  qar-hub-test-ext (C# hub follow tests; optional, skipped without it)
    QAROS_C_TESTS_BIN       qar-streaming-c_dynamic_gtests, from the same build as QAROS_NATIVE_BIN

They are read from the environment first, then from an env file (``--env-file``; default: the
``ci.env`` of a repository embedding qaros, else ``ci/ci.env``). Without ``QAROS_NATIVE_BIN`` the
native commands download the C API runtime of ``--release`` / ``QAROS_RUNTIME_RELEASE`` with gh
(``QAROS-CApi-*-RelWithDebInfo.zip``: ``bin/`` and ``tests/``). ``--release`` always downloads.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import sys
from pathlib import Path

sys.dont_write_bytecode = True  # no __pycache__ in the checkout
sys.path.insert(0, str(Path(__file__).resolve().parent))

from qaros_ci import (  # noqa: E402
    BASE_LIBRARY,
    C_TEST_EXECUTABLE,
    EXT_LIBRARY,
    HUB_TEST_EXT_LIBRARY,
    QAROS_ROOT,
    WINDOWS,
    CiError,
    download_runtime,
    env,
    load_env_file,
    native_folder,
    output_of,
    require_tool,
    run,
    succeeds,
    use_runtime,
    visual_studio_environment,
)

RUST_DIR = QAROS_ROOT / "qar-streaming-rust"
SHARP_DIR = QAROS_ROOT / "qar-streaming-sharp"
WEBSITE_DIR = QAROS_ROOT / "docs" / "website"
RUST_STEPS = ("fmt", "clippy", "test", "tutorials", "guide")
RUST_E2E = "e2e"
RUSTUP_HINT = "install the latest stable Rust from https://rustup.rs"
DOTNET_HINT = "install the .NET 8 SDK (or newer) from https://dot.net"
RUST_DOC_TARGET = "x86_64-pc-windows-msvc"
CUBE_EXE = QAROS_ROOT / "build" / "x64-windows" / "qar-streaming-c" / "examples" / "vulkan_cube" / "Release" / \
    "qar-vulkan-source.exe"


def results_dir(args: argparse.Namespace, name: str) -> Path:
    folder = (Path(args.results_dir) if args.results_dir else QAROS_ROOT / "build" / "test-results") / name
    folder.mkdir(parents=True, exist_ok=True)
    return folder


# ---------------------------------------------------------------------------------------------
# Native folders: given, or downloaded


def provide_natives(args: argparse.Namespace) -> None:
    """``--release`` downloads; otherwise a set ``QAROS_NATIVE_BIN`` is used as it is; otherwise
    ``QAROS_RUNTIME_RELEASE`` is downloaded."""
    if getattr(args, "natives_ready", False):
        return
    release = args.release or (None if env("QAROS_NATIVE_BIN") else env("QAROS_RUNTIME_RELEASE"))
    if release:
        repository = env("QAROS_RUNTIME_REPOSITORY") or "Quaternar/Quaternar"
        directory = Path(env("QAROS_RUNTIME_DIR") or QAROS_ROOT / "build" / "qaros-capi-runtime") / release
        use_runtime(download_runtime(release, repository, directory))
    native_folder("QAROS_NATIVE_BIN", (BASE_LIBRARY, EXT_LIBRARY),
                  " (or set QAROS_RUNTIME_RELEASE / pass --release to download a C API runtime)")
    args.natives_ready = True


# ---------------------------------------------------------------------------------------------
# rust


def rust(args: argparse.Namespace) -> None:
    steps = [RUST_E2E] if args.e2e else (args.step or list(RUST_STEPS))
    require_tool("cargo", RUSTUP_HINT)
    if "fmt" in steps and not succeeds(["cargo", "fmt", "--version"]):
        raise CiError("missing tool 'rustfmt': run 'rustup component add rustfmt'")
    if "clippy" in steps and not succeeds(["cargo", "clippy", "--version"]):
        raise CiError("missing tool 'clippy': run 'rustup component add clippy'")
    if RUST_E2E in steps:
        provide_natives(args)
    commands = {
        "fmt": ["cargo", "fmt", "--all", "--", "--check"],
        "clippy": ["cargo", "clippy", "--workspace", "--all-targets", "--features",
                   "qaros/vulkan,qaros-examples/vulkan,qar-streaming-rust-ext/e2e", "--", "-D", "warnings"],
        "test": ["cargo", "test", "--workspace"],
        "tutorials": ["cargo", "xtask", "tutorials", "--check"],
        "guide": ["cargo", "xtask", "guide", "--check"],
        RUST_E2E: ["cargo", "test", "-p", "qar-streaming-rust-ext", "--features", "e2e", "--", "--nocapture"],
    }
    for step in steps:
        print(f"== rust {step}", flush=True)
        run(commands[step], cwd=RUST_DIR)
    print(f"Rust: {', '.join(steps)} passed.", flush=True)


# ---------------------------------------------------------------------------------------------
# csharp


def csharp(args: argparse.Namespace) -> None:
    if not WINDOWS:
        raise CiError("the C# binding tests drive Direct3D 11 and Vulkan: they run on Windows only")
    require_tool("dotnet", DOTNET_HINT)
    if not re.search(r"^Microsoft\.NETCore\.App 8\.", output_of(["dotnet", "--list-runtimes"]), re.MULTILINE):
        raise CiError("missing the .NET 8 runtime (the tests run on net8.0): install it from https://dot.net")
    provide_natives(args)
    native = env("QAROS_NATIVE_BIN")
    results = results_dir(args, "csharp")
    command = ["dotnet", "test", "tests/QarStreaming.Tests/QarStreaming.Tests.csproj", "-c", "Release",
               "-f", "net8.0", f"-p:QarStreamingNativeBinDir={native}"]
    hub = env("QAROS_HUB_TEST_EXT_BIN")
    if hub:
        native_folder("QAROS_HUB_TEST_EXT_BIN", (HUB_TEST_EXT_LIBRARY,))
    else:
        # No hub-test-ext for these natives (a downloaded runtime has none): never mix in another
        # build's through the props.
        command.append("-p:QarSkipHubTestExt=true")
    if args.filter:
        command += ["--filter", args.filter]
    command += ["--logger", "trx;LogFileName=csharp-binding-tests.trx", "--results-directory", str(results)]
    print("== csharp", flush=True)
    run(command, cwd=SHARP_DIR)
    print(f"C#: passed. Results: {results}", flush=True)


# ---------------------------------------------------------------------------------------------
# c


def c_tests(args: argparse.Namespace) -> None:
    provide_natives(args)
    tests = native_folder("QAROS_C_TESTS_BIN", (C_TEST_EXECUTABLE,))
    results = results_dir(args, "c")
    test_filter = args.filter or env("QAROS_C_TEST_FILTER") or "*"
    timeout = float(args.timeout or env("QAROS_C_TEST_TIMEOUT") or 1800)
    print(f"== c ({C_TEST_EXECUTABLE}, filter {test_filter}, timeout {timeout:.0f} s)", flush=True)
    # The suite loads qar-streaming-c and the ext from QAROS_NATIVE_BIN (in the environment). Its
    # working directory is a results folder: it writes logs and dumps there.
    run([str(tests / C_TEST_EXECUTABLE), f"--gtest_filter={test_filter}", "--gtest_brief=1",
         f"--gtest_output=xml:{results / 'c-api-tests.xml'}"], cwd=results, timeout=timeout)
    print(f"C: passed. Results: {results}", flush=True)


# ---------------------------------------------------------------------------------------------
# capi: c, csharp and the Rust e2e tests against one native folder


def capi(args: argparse.Namespace) -> None:
    provide_natives(args)
    failed = []
    suites = [("c", c_tests), ("csharp", csharp), ("rust e2e", rust)]
    args.e2e = True
    for name, suite in suites:
        try:
            suite(args)
        except CiError as error:
            print(f"FAILED {name}: {error}", file=sys.stderr, flush=True)
            failed.append(name)
    if failed:
        raise CiError(f"native tests failed: {', '.join(failed)}")
    print("Native tests (c, csharp, rust e2e) passed.", flush=True)


# ---------------------------------------------------------------------------------------------
# vulkan-cube


def hide_vulkan_sdk(environment: dict[str, str], strict: bool) -> dict[str, str]:
    """A Vulkan SDK on the host would hide a gap in the documented setup (vcpkg brings headers,
    loader and glslc). ``strict`` (CI) fails on one; otherwise it is removed from the build's
    environment."""
    result = dict(environment)
    sdk = result.get("VULKAN_SDK")
    if sdk:
        if strict:
            raise CiError(f"VULKAN_SDK is set ({sdk}): the build would not prove the documented setup")
        print(f"hiding the Vulkan SDK from the build: VULKAN_SDK ({sdk})", flush=True)
        del result["VULKAN_SDK"]
    glslc = "glslc.exe" if WINDOWS else "glslc"
    entries = result.get("PATH", "").split(os.pathsep)
    holding = [entry for entry in entries if entry and (Path(entry) / glslc).is_file()]
    if holding:
        if strict:
            raise CiError(f"glslc is on PATH ({holding[0]}): the build would not prove the documented setup")
        print(f"hiding the Vulkan SDK from the build: PATH entries {', '.join(holding)}", flush=True)
        result["PATH"] = os.pathsep.join(entry for entry in entries if entry not in holding)
    return result


def vulkan_cube(args: argparse.Namespace) -> None:
    if not WINDOWS:
        raise CiError("the Vulkan cube build follows the Windows getting-started page (preset x64-windows): "
                      "Windows only")
    environment = hide_vulkan_sdk(dict(os.environ), args.strict)
    # Developer shell, as the page says: Visual Studio's own compiler, CMake and Ninja.
    environment = hide_vulkan_sdk(visual_studio_environment(environment), True)
    for tool in ("cmake", "ninja", "cl"):
        if not shutil.which(tool, path=environment.get("PATH")):
            raise CiError(f"'{tool}' is not on PATH in the Visual Studio developer environment: add the C++ "
                          "workload (with its CMake tools) to Visual Studio")
    print("== vulkan-cube", flush=True)
    run(["cmake", "--preset", "x64-windows", "-DBUILD_VULKAN_EXAMPLE=ON",
         "-DVCPKG_MANIFEST_FEATURES=examples-vulkan"], environment=environment)
    run(["cmake", "--build", "--preset", "x64-windows-release", "--target", "qar-vulkan-source"],
        environment=environment)
    if not CUBE_EXE.is_file():
        raise CiError(f"{CUBE_EXE} missing after the build")
    run([str(CUBE_EXE), "--help"])
    print("Vulkan cube build passed.", flush=True)


# ---------------------------------------------------------------------------------------------
# docs


def docs(args: argparse.Namespace) -> None:
    require_tool("node", "install Node.js 24 from https://nodejs.org")
    require_tool("npm", "install Node.js 24 from https://nodejs.org (npm comes with it)")
    node = output_of(["node", "--version"]).strip()
    if not node.startswith("v24."):
        print(f"warning: Node.js {node}; CI builds with Node.js 24", flush=True)
    require_tool("dotnet", DOTNET_HINT)
    if not re.search(r"^(8|9|[1-9][0-9])\.", output_of(["dotnet", "--list-sdks"]), re.MULTILINE):
        raise CiError("missing the .NET SDK 8 or newer (DocFX generates the C# API reference): " + DOTNET_HINT)
    require_tool("cargo", RUSTUP_HINT)
    require_tool("rustup", RUSTUP_HINT)
    if RUST_DOC_TARGET not in output_of(["rustup", "target", "list", "--installed"]).split():
        raise CiError(f"missing the Rust target {RUST_DOC_TARGET} (rustdoc documents the binding for it): "
                      f"run 'rustup target add {RUST_DOC_TARGET}'")
    require_tool("git", "install Git from https://git-scm.com")
    if not output_of(["git", "-C", str(QAROS_ROOT), "tag", "--list", "v*"]).strip():
        raise CiError(f"no v<version> tags in {QAROS_ROOT} (the Rust API reference of each release is built "
                      "from its tag): run 'git fetch --tags'")
    print("== docs", flush=True)
    run(["npm", "ci"], cwd=WEBSITE_DIR)
    run(["npm", "run", "build"], cwd=WEBSITE_DIR)
    print(f"Docs website built: {WEBSITE_DIR / 'build'}", flush=True)


# ---------------------------------------------------------------------------------------------


def comma_list(choices: tuple[str, ...]):
    def parse(text: str) -> list[str]:
        values = [value.strip() for value in text.split(",") if value.strip()]
        unknown = [value for value in values if value not in choices]
        if unknown:
            raise argparse.ArgumentTypeError(f"unknown {', '.join(unknown)}; choose from {', '.join(choices)}")
        return values
    return parse


def parser() -> argparse.ArgumentParser:
    common = argparse.ArgumentParser(add_help=False)
    common.add_argument("--env-file", type=Path, help="env file to read (default: see the module docstring)")
    natives = argparse.ArgumentParser(add_help=False)
    natives.add_argument("--release", help="download this release's C API runtime (vX.Y.Z or dev-<branch>)")
    natives.add_argument("--results-dir", help="where test results go, one folder per suite (default build/test-results)")

    root = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = root.add_subparsers(dest="command", required=True)

    rust_parser = commands.add_parser("rust", parents=[common, natives], help="Rust binding checks")
    rust_parser.add_argument("--step", type=comma_list((*RUST_STEPS, RUST_E2E)),
                             help=f"comma list of {', '.join((*RUST_STEPS, RUST_E2E))} (default: all but e2e)")
    rust_parser.add_argument("--e2e", action="store_true", help="only the integration tests (QAROS_NATIVE_BIN)")
    rust_parser.set_defaults(handler=rust)

    csharp_parser = commands.add_parser("csharp", parents=[common, natives], help="C# binding tests")
    csharp_parser.add_argument("--filter", help="dotnet test --filter expression")
    csharp_parser.set_defaults(handler=csharp)

    c_parser = commands.add_parser("c", parents=[common, natives], help="C API suite against QAROS_NATIVE_BIN")
    c_parser.add_argument("--filter", help="gtest filter (default QAROS_C_TEST_FILTER, else all)")
    c_parser.add_argument("--timeout", type=float, help="seconds for the whole run (default QAROS_C_TEST_TIMEOUT, else 1800)")
    c_parser.set_defaults(handler=c_tests)

    capi_parser = commands.add_parser("capi", parents=[common, natives],
                                      help="c, csharp and the Rust e2e tests against one native folder")
    capi_parser.set_defaults(handler=capi, step=None, filter=None, timeout=None)

    cube_parser = commands.add_parser("vulkan-cube", parents=[common], help="Vulkan cube build as documented")
    cube_parser.add_argument("--strict", action="store_true", help="fail on a Vulkan SDK on the host (CI)")
    cube_parser.set_defaults(handler=vulkan_cube)

    docs_parser = commands.add_parser("docs", parents=[common], help="documentation website build")
    docs_parser.set_defaults(handler=docs)
    return root


def main() -> int:
    args = parser().parse_args()
    try:
        load_env_file(args.env_file)
        args.handler(args)
    except CiError as error:
        print(f"error: {error}", file=sys.stderr, flush=True)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
