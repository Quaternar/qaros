#!/usr/bin/env python3
"""Every check and build of the qaros GitHub workflows, runnable locally from any folder:

    python ci/run.py rust [--step fmt,clippy,test,tutorials,guide,e2e] [--e2e]
    python ci/run.py csharp [--filter <dotnet test filter>]
    python ci/run.py c [--filter <gtest filter>] [--timeout <s>]
    python ci/run.py capi                    # c, csharp and the Rust e2e tests on one native folder
    python ci/run.py vulkan-cube [--strict]
    python ci/run.py cpp                     # qar-streaming-cpp compile matrix and a C++-only CMake consumer
    python ci/run.py docs
    python ci/run.py package [--tag vX.Y.Z]  # version checks, the NuGet package and the crates
    python ci/run.py publish --nuget --crates [--dry-run]

Native files (C API runtime, test counterparts, the C test executable) come from environment
variables, never from a build this script starts:

    QAROS_NATIVE_BIN        qar-streaming-c, qar-streaming-c-ext and every library they load
    QAROS_HUB_TEST_EXT_BIN  qar-hub-test-ext (C# hub follow tests; optional, skipped without it)
    QAROS_C_TESTS_BIN       qar-streaming-c_dynamic_gtests, from the same build as QAROS_NATIVE_BIN
                            (a local build only: no package ships it)

They are read from the environment first, then from an env file (``--env-file``; default: the
``ci.env`` of a repository embedding qaros, else ``ci/ci.env``). Without ``QAROS_NATIVE_BIN`` the
native commands download the C API runtime of ``--release`` / ``QAROS_RUNTIME_RELEASE`` with gh
(``QAROS-CApi-*-RelWithDebInfo.zip``: ``bin/`` only). ``--release`` always downloads.

``c`` needs ``QAROS_C_TESTS_BIN`` and fails without it, so it cannot run on a downloaded runtime:
with a download due it refuses before downloading. ``capi`` then skips ``c``, runs ``csharp`` and
the Rust e2e tests, and reports ``c`` as skipped.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import urllib.error
import urllib.request
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


def runtime_release(args: argparse.Namespace) -> str | None:
    """The C API runtime release provide_natives will download, if any: ``--release``; otherwise
    ``QAROS_RUNTIME_RELEASE`` when ``QAROS_NATIVE_BIN`` is not set."""
    if getattr(args, "natives_ready", False):
        return None
    return args.release or (None if env("QAROS_NATIVE_BIN") else env("QAROS_RUNTIME_RELEASE"))


def provide_natives(args: argparse.Namespace) -> None:
    """``--release`` downloads; otherwise a set ``QAROS_NATIVE_BIN`` is used as it is; otherwise
    ``QAROS_RUNTIME_RELEASE`` is downloaded."""
    if getattr(args, "natives_ready", False):
        return
    release = runtime_release(args)
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


C_TESTS_MISSING = ("QAROS_C_TESTS_BIN is not set: the C API suite needs a local build of "
                   f"{C_TEST_EXECUTABLE.removesuffix('.exe')} from the same build as QAROS_NATIVE_BIN "
                   "(a downloaded C API runtime has no tests); set QAROS_C_TESTS_BIN to its output folder")


def c_tests_with_download(release: str) -> str:
    return (f"QAROS_C_TESTS_BIN cannot run against the downloaded C API runtime {release}: the suite "
            "and its natives must come from one build; set QAROS_NATIVE_BIN to that build's output "
            "folder instead of downloading")


def c_tests(args: argparse.Namespace) -> None:
    # Checked before provide_natives: a download unsets QAROS_C_TESTS_BIN.
    if not env("QAROS_C_TESTS_BIN"):
        raise CiError(C_TESTS_MISSING)
    release = runtime_release(args)
    if release:
        raise CiError(c_tests_with_download(release))
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
    # Decided before provide_natives: a download unsets QAROS_C_TESTS_BIN.
    release = runtime_release(args)
    c_skip_reason = None
    if not env("QAROS_C_TESTS_BIN"):
        c_skip_reason = C_TESTS_MISSING
    elif release:
        c_skip_reason = c_tests_with_download(release)
    provide_natives(args)
    failed = []
    suites = [("csharp", csharp), ("rust e2e", rust)]
    skipped = []
    if c_skip_reason:
        print(f"SKIPPED c: {c_skip_reason}", flush=True)
        skipped.append("c")
    else:
        suites.insert(0, ("c", c_tests))
    args.e2e = True
    for name, suite in suites:
        try:
            suite(args)
        except CiError as error:
            print(f"FAILED {name}: {error}", file=sys.stderr, flush=True)
            failed.append(name)
    if failed:
        raise CiError(f"native tests failed: {', '.join(failed)}")
    ran = ", ".join(name for name, _ in suites)
    print(f"Native tests ({ran}) passed" + (f"; skipped: {', '.join(skipped)}." if skipped else "."), flush=True)


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
# cpp: the header-only C++ wrapper


CPP_DIR = QAROS_ROOT / "qar-streaming-cpp"
CPP_HEADERS = CPP_DIR / "include" / "qar_streaming_cpp"
CPP_CHECK = CPP_DIR / "tests" / "compile_check.cpp"
C_INCLUDE = QAROS_ROOT / "qar-streaming-c" / "include"
CPP_STANDARDS = ("c++17", "c++20")
CPP_CONSUMER = """cmake_minimum_required(VERSION 3.29)
project(QarosCppConsumer LANGUAGES CXX)
add_subdirectory("{c}" qar-streaming-c)
add_subdirectory("{cpp}" qar-streaming-cpp)
add_executable(consumer "{check}")
target_link_libraries(consumer PRIVATE qar-streaming-cpp)
target_compile_options(consumer PRIVATE /W4 /WX)
get_property(languages GLOBAL PROPERTY ENABLED_LANGUAGES)
if("C" IN_LIST languages)
  message(FATAL_ERROR "linking qar-streaming-cpp enabled the C language")
endif()
if(TARGET dynamic_loading)
  message(FATAL_ERROR "add_subdirectory(qar-streaming-c) added the C examples")
endif()
"""


def cpp_check_covers_every_header() -> None:
    text = CPP_CHECK.read_text(encoding="utf-8")
    missing = [header.name for header in sorted(CPP_HEADERS.glob("*.hpp"))
               if f"<qar_streaming_cpp/{header.name}>" not in text]
    if missing:
        raise CiError(f"{CPP_CHECK.relative_to(QAROS_ROOT)} does not include {', '.join(missing)}")


def cpp(args: argparse.Namespace) -> None:
    """Every header of qar-streaming-cpp in one translation unit with the dynamic-loading
    implementation, MSVC /W4 /WX in C++17 and C++20, with and without /permissive- (no alternative
    tokens, no deprecated C call), then a C++-only CMake project that adds qar-streaming-c and
    qar-streaming-cpp with add_subdirectory (no C language, no examples)."""
    if not WINDOWS:
        raise CiError("the wrapper check compiles with MSVC: Windows only")
    cpp_check_covers_every_header()
    environment = visual_studio_environment(dict(os.environ))
    for tool in ("cl", "cmake", "ninja"):
        if not shutil.which(tool, path=environment.get("PATH")):
            raise CiError(f"'{tool}' is not on PATH in the Visual Studio developer environment")
    out = QAROS_ROOT / "build" / "cpp-check"
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    for standard in CPP_STANDARDS:
        for conformance in ([], ["/permissive-"]):
            label = " ".join([standard, *conformance]) or standard
            print(f"== cpp {label}", flush=True)
            run(["cl", "/nologo", "/c", "/EHsc", "/W4", "/WX", f"/std:{standard}", *conformance,
                 "/DQAR_ENABLE_DYNAMIC_LOADING", f"/I{C_INCLUDE}", f"/I{CPP_DIR / 'include'}",
                 f"/Fo{out / 'compile_check.obj'}", str(CPP_CHECK)], cwd=out, environment=environment)
    print("== cpp CMake consumer (CXX only)", flush=True)
    consumer = out / "consumer"
    consumer.mkdir()
    (consumer / "CMakeLists.txt").write_text(CPP_CONSUMER.format(
        c=(QAROS_ROOT / "qar-streaming-c").as_posix(), cpp=CPP_DIR.as_posix(), check=CPP_CHECK.as_posix()))
    run(["cmake", "-S", str(consumer), "-B", str(consumer / "build"), "-G", "Ninja",
         "-DCMAKE_BUILD_TYPE=Release"], environment=environment)
    run(["cmake", "--build", str(consumer / "build"), "--parallel", "2"], environment=environment)
    print("C++ wrapper: compile matrix and CMake consumer passed.", flush=True)


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
# package and publish: Quaternar.Qaros on nuget.org, qaros-sys and qaros on crates.io


VERSION_FILE = QAROS_ROOT / "VERSION"
PACKAGES_DIR = QAROS_ROOT / "build" / "packages"
NUGET_PACKAGE = "Quaternar.Qaros"
NUGET_SOURCE = "https://api.nuget.org/v3/index.json"
# In publishing order: qaros depends on qaros-sys.
CRATES = ("qaros-sys", "qaros")
# crates.io refuses API requests without a User-Agent that names the client.
USER_AGENT = "qaros-ci (https://github.com/Quaternar/qaros)"


def release_version() -> str:
    """The version every package ships as: the VERSION file, which the csproj reads as well. The
    Cargo workspace version and its exact pins of the published crates must equal it."""
    import tomllib  # Python 3.11 or newer; only these commands need it

    version = VERSION_FILE.read_text(encoding="utf-8").strip()
    if not re.fullmatch(r"\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?", version):
        raise CiError(f"{VERSION_FILE.name} holds {version!r}, not a version such as 1.2.3 or 1.2.3-rc.1")
    manifest = tomllib.loads((RUST_DIR / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]
    problems = []
    if manifest["package"]["version"] != version:
        problems.append(f"[workspace.package] version is {manifest['package']['version']}")
    for crate in CRATES:
        pin = manifest["dependencies"][crate].get("version")
        if pin != f"={version}":
            problems.append(f"[workspace.dependencies] {crate} has version {pin!r}, expected '={version}'")
    if problems:
        raise CiError(f"qar-streaming-rust/Cargo.toml does not match {VERSION_FILE.name} ({version}): "
                      + "; ".join(problems))
    return version


def check_tag(tag: str | None, version: str) -> None:
    if tag and tag != f"v{version}":
        raise CiError(f"tag {tag} does not match {VERSION_FILE.name}: expected v{version}")


def package(args: argparse.Namespace) -> None:
    """Checks the versions, packs Quaternar.Qaros (with its symbol package) into
    build/packages/nuget and packages and verifies the crates (cargo package builds each one from its
    .crate file, against the packaged qaros-sys)."""
    version = release_version()
    check_tag(args.tag, version)
    require_tool("dotnet", DOTNET_HINT)
    require_tool("cargo", RUSTUP_HINT)
    nuget_dir = PACKAGES_DIR / "nuget"
    if nuget_dir.exists():
        shutil.rmtree(nuget_dir)
    print(f"== package {NUGET_PACKAGE} {version}", flush=True)
    run(["dotnet", "pack", "Quaternar.Qaros.csproj", "-c", "Release", "-o", str(nuget_dir)], cwd=SHARP_DIR)
    nupkg = nuget_dir / f"{NUGET_PACKAGE}.{version}.nupkg"
    if not nupkg.is_file():
        raise CiError(f"{nupkg} missing after dotnet pack")
    print(f"== package crates {', '.join(CRATES)} {version}", flush=True)
    run(["cargo", "package", "--locked", *[arg for crate in CRATES for arg in ("-p", crate)]], cwd=RUST_DIR)
    print(f"Packages {version}: {nuget_dir} and {RUST_DIR / 'target' / 'package'}", flush=True)


def published(url: str) -> dict | None:
    """The JSON at ``url``, or None on 404 (not published yet)."""
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise CiError(f"{url}: HTTP {error.code}") from None


def nuget_has(version: str) -> bool:
    index = published(f"https://api.nuget.org/v3-flatcontainer/{NUGET_PACKAGE.lower()}/index.json")
    return bool(index) and version.lower() in index.get("versions", [])


def crates_io_has(crate: str, version: str) -> bool:
    info = published(f"https://crates.io/api/v1/crates/{crate}/{version}")
    return bool(info) and "version" in info


def publish(args: argparse.Namespace) -> None:
    """Pushes what ``package`` made; a version already published is skipped, so a re-run after a
    partial failure finishes the job. The keys come from the environment: NUGET_API_KEY and
    CARGO_REGISTRY_TOKEN (the workflow gets both by trusted publishing)."""
    if not (args.nuget or args.crates):
        raise CiError("pass --nuget, --crates or both")
    version = release_version()
    check_tag(args.tag, version)
    if args.nuget:
        nupkg = PACKAGES_DIR / "nuget" / f"{NUGET_PACKAGE}.{version}.nupkg"
        if not nupkg.is_file():
            raise CiError(f"{nupkg} missing: run 'python ci/run.py package' first")
        if nuget_has(version):
            print(f"SKIPPED {NUGET_PACKAGE} {version}: already on nuget.org", flush=True)
        elif args.dry_run:
            print(f"dry run: would push {nupkg.name} (and its .snupkg) to {NUGET_SOURCE}", flush=True)
        else:
            key = env("NUGET_API_KEY")
            if not key:
                raise CiError("NUGET_API_KEY is not set")
            require_tool("dotnet", DOTNET_HINT)
            # Not run(): it echoes the command, and the key is on it. The .snupkg next to the
            # .nupkg goes to the symbol server with it.
            print(f"== push {nupkg.name} to {NUGET_SOURCE}", flush=True)
            completed = subprocess.run([shutil.which("dotnet") or "dotnet", "nuget", "push", str(nupkg),
                                        "--source", NUGET_SOURCE, "--api-key", key, "--skip-duplicate"])
            if completed.returncode != 0:
                raise CiError(f"dotnet nuget push {nupkg.name} failed (exit code {completed.returncode})")
    if args.crates:
        require_tool("cargo", RUSTUP_HINT)
        pending = []
        for crate in CRATES:
            if crates_io_has(crate, version):
                print(f"SKIPPED {crate} {version}: already on crates.io", flush=True)
            else:
                pending.append(crate)
        if pending:
            # One cargo publish for all of them (Cargo 1.90 or newer): it publishes in dependency
            # order and verifies qaros against the qaros-sys being published, also in a dry run.
            command = ["cargo", "publish", "--locked", *[arg for crate in pending for arg in ("-p", crate)]]
            if args.dry_run:
                command.append("--dry-run")
            elif not env("CARGO_REGISTRY_TOKEN"):
                raise CiError("CARGO_REGISTRY_TOKEN is not set")
            print(f"== publish {', '.join(pending)} {version}", flush=True)
            run(command, cwd=RUST_DIR)
    print(f"Publish {version}: done" + (" (dry run)." if args.dry_run else "."), flush=True)


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

    cpp_parser = commands.add_parser("cpp", parents=[common],
                                     help="qar-streaming-cpp compile matrix (MSVC) and a C++-only CMake consumer")
    cpp_parser.set_defaults(handler=cpp)

    docs_parser = commands.add_parser("docs", parents=[common], help="documentation website build")
    docs_parser.set_defaults(handler=docs)

    tag = argparse.ArgumentParser(add_help=False)
    tag.add_argument("--tag", help="the release tag (vX.Y.Z); fails unless it matches VERSION")
    package_parser = commands.add_parser("package", parents=[common, tag],
                                         help="version checks, the NuGet package and the crates")
    package_parser.set_defaults(handler=package)

    publish_parser = commands.add_parser("publish", parents=[common, tag],
                                         help="push the packages to nuget.org and crates.io")
    publish_parser.add_argument("--nuget", action="store_true", help=f"push {NUGET_PACKAGE} (NUGET_API_KEY)")
    publish_parser.add_argument("--crates", action="store_true",
                                help=f"publish {', '.join(CRATES)} (CARGO_REGISTRY_TOKEN)")
    publish_parser.add_argument("--dry-run", action="store_true", help="check, push nothing")
    publish_parser.set_defaults(handler=publish)
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
