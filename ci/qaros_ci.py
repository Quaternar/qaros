"""Shared helpers of ``ci/run.py``: the env file, tool checks, child processes, the native folders
and the C API runtime download. Standard library only; Windows and Linux."""

from __future__ import annotations

import os
import re
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

QAROS_ROOT = Path(__file__).resolve().parent.parent
QAROS_ENV_FILE = QAROS_ROOT / "ci" / "ci.env"
# A repository that embeds qaros as a submodule (the Quaternar one) keeps its own env file at its
# root, next to the qaros folder; it points the native folders at that repository's build.
EMBEDDING_ENV_FILE = QAROS_ROOT.parent / "ci.env"

WINDOWS = sys.platform == "win32"
# Keys whose value is a path: a relative one is taken relative to the env file's folder.
PATH_KEY_SUFFIXES = ("_BIN", "_DIR", "_PATH", "_FILE")

BASE_LIBRARY = "qar-streaming-c.dll" if WINDOWS else "libqar-streaming-c.so"
EXT_LIBRARY = "qar-streaming-c-ext.dll" if WINDOWS else "libqar-streaming-c-ext.so"
HUB_TEST_EXT_LIBRARY = "qar-hub-test-ext.dll" if WINDOWS else "libqar-hub-test-ext.so"
C_TEST_EXECUTABLE = "qar-streaming-c_dynamic_gtests" + (".exe" if WINDOWS else "")

# The CMake targets whose output folders hold the native files (named in failure messages).
NATIVE_TARGETS = {
    "QAROS_NATIVE_BIN": "qar-streaming-c-ext_gtests",
    "QAROS_HUB_TEST_EXT_BIN": "qar-hub-test-ext_gtests",
    "QAROS_C_TESTS_BIN": "qar-streaming-c_dynamic_gtests",
}


class CiError(Exception):
    """A failure with a message for the user; ``run.py`` prints it and exits 1."""


# ---------------------------------------------------------------------------------------------
# Env file


def default_env_file() -> Path:
    """The embedding repository's file when qaros sits inside one, else qaros' own."""
    return EMBEDDING_ENV_FILE if EMBEDDING_ENV_FILE.is_file() else QAROS_ENV_FILE


def parse_env_file(path: Path) -> dict[str, str]:
    """``KEY=VALUE`` lines; blank lines and ``#`` comment lines are skipped, a value may be quoted.
    A relative value of a key ending in ``_BIN``, ``_DIR``, ``_PATH`` or ``_FILE`` is resolved
    against the file's folder."""
    values: dict[str, str] = {}
    for number, raw in enumerate(path.read_text(encoding="utf-8-sig").splitlines(), start=1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        key, separator, value = line.partition("=")
        key = key.strip()
        if not separator or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", key):
            raise CiError(f"{path}:{number}: expected KEY=VALUE, got {raw!r}")
        value = value.strip()
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
            value = value[1:-1]
        if value and key.endswith(PATH_KEY_SUFFIXES) and not Path(value).is_absolute():
            value = str((path.parent / value).resolve())
        values[key] = value
    return values


def load_env_file(path: Path | None) -> Path:
    """Applies the env file to ``os.environ``: a variable already set (and not empty) wins. Every
    child process gets the result. Returns the file used."""
    chosen = path if path is not None else default_env_file()
    if not chosen.is_file():
        raise CiError(f"env file {chosen} does not exist")
    for key, value in parse_env_file(chosen).items():
        if not os.environ.get(key):
            os.environ[key] = value
    print(f"env file: {chosen}", flush=True)
    return chosen


def env(key: str) -> str | None:
    value = os.environ.get(key)
    return value if value else None


# ---------------------------------------------------------------------------------------------
# Tools and processes


def require_tool(name: str, hint: str) -> str:
    found = shutil.which(name)
    if found is None:
        raise CiError(f"missing tool '{name}': {hint}")
    return found


def succeeds(command: list[str]) -> bool:
    try:
        return subprocess.run(command, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0
    except OSError:
        return False


def output_of(command: list[str]) -> str:
    return subprocess.run(command, check=True, capture_output=True, text=True).stdout


def run(command: list[str], cwd: Path = QAROS_ROOT, environment: dict[str, str] | None = None,
        timeout: float | None = None) -> None:
    """Runs ``command`` (its output goes to this console) and fails on a non-zero exit code."""
    executable = shutil.which(command[0], path=(environment or os.environ).get("PATH")) or command[0]
    print(f"[{cwd}] {' '.join(command)}", flush=True)
    try:
        completed = subprocess.run([executable, *command[1:]], cwd=cwd, env=environment, timeout=timeout)
    except subprocess.TimeoutExpired:
        raise CiError(f"'{' '.join(command)}' did not finish within {timeout:.0f} s") from None
    if completed.returncode != 0:
        raise CiError(f"'{' '.join(command)}' failed (exit code {completed.returncode})")


# ---------------------------------------------------------------------------------------------
# Native folders


def native_folder(key: str, required_files: tuple[str, ...], hint: str = "") -> Path:
    """The folder ``key`` names, holding ``required_files``; fails naming the variable, the folder
    and the CMake target that produces it."""
    value = env(key)
    target = NATIVE_TARGETS.get(key)
    produced_by = f" (the output folder of the CMake target {target})" if target else ""
    if value is None:
        raise CiError(f"{key} is not set: set it in the environment or the env file to the folder holding "
                      f"{', '.join(required_files)}{produced_by}{hint}")
    folder = Path(value)
    missing = [name for name in required_files if not (folder / name).is_file()]
    if missing:
        build = f"; build {target} first" if target else ""
        raise CiError(f"{key} is {folder}, which holds no {' and no '.join(missing)}{build}{hint}")
    return folder


# ---------------------------------------------------------------------------------------------
# C API runtime release


def download_runtime(release: str, repository: str, directory: Path) -> Path:
    """Downloads the ``QAROS-CApi-*-RelWithDebInfo.zip`` asset of ``release`` with gh and unpacks it
    into ``directory`` (emptied first). Returns the unpacked folder (``bin/``, ``include/``)."""
    gh = require_tool("gh", "install the GitHub CLI from https://cli.github.com")
    if not env("GH_TOKEN") and not env("GITHUB_TOKEN") and not succeeds([gh, "auth", "status"]):
        raise CiError(f"gh is not logged in: run 'gh auth login', or set GH_TOKEN to a token that can read "
                      f"{repository} releases")
    if not succeeds([gh, "release", "view", release, "--repo", repository]):
        raise CiError(f"release {release} of {repository} not found (or not readable with this token). "
                      f"{release_origin(release, repository)}")
    if directory.exists():
        shutil.rmtree(directory)
    directory.mkdir(parents=True)
    run([gh, "release", "download", release, "--repo", repository,
         "--pattern", "QAROS-CApi-*-RelWithDebInfo.zip", "--dir", str(directory)])
    archives = sorted(directory.glob("QAROS-CApi-*-RelWithDebInfo.zip"))
    if not archives:
        raise CiError(f"release {release} of {repository} has no QAROS-CApi-*-RelWithDebInfo.zip asset. "
                      f"{release_origin(release, repository)}")
    with zipfile.ZipFile(archives[0]) as archive:
        archive.extractall(directory)
    # The ZIP holds one folder named after itself.
    unpacked = directory / archives[0].stem
    print(f"C API runtime: {archives[0].name} -> {unpacked}", flush=True)
    return unpacked


def release_origin(release: str, repository: str) -> str:
    """Where a release comes from, for the messages of a failed download."""
    if release.startswith("dev-"):
        branch = release[len("dev-"):]
        return (f"{release} is published by .github/workflows/windows-release.yml of {repository} running on "
                f"the branch {branch} (a push there, or workflow_dispatch with publish).")
    return f"A v<version> release is published by .github/workflows/windows-release.yml of {repository} on its tag."


def use_runtime(unpacked: Path) -> None:
    """Points the native variables at an unpacked C API runtime, for this process and its children.
    It has no qar-hub-test-ext and no C API suite: those are built only in the repository that
    embeds qaros, and another build's would not match these natives."""
    os.environ["QAROS_NATIVE_BIN"] = str(unpacked / "bin")
    os.environ.pop("QAROS_C_TESTS_BIN", None)
    os.environ.pop("QAROS_HUB_TEST_EXT_BIN", None)


# ---------------------------------------------------------------------------------------------
# Visual Studio


def visual_studio_environment(base: dict[str, str]) -> dict[str, str]:
    """``base`` after Visual Studio's ``vcvars64.bat`` (what a developer shell sets: compiler, its
    CMake and Ninja on PATH). Windows only."""
    vswhere = Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")) / \
        "Microsoft Visual Studio" / "Installer" / "vswhere.exe"
    if not vswhere.is_file():
        raise CiError(f"missing tool 'vswhere' ({vswhere}): install Visual Studio 2022 with the C++ workload")
    installation = output_of([str(vswhere), "-latest", "-products", "*", "-property", "installationPath"]).strip()
    if not installation:
        raise CiError("no Visual Studio found: install Visual Studio 2022 with the C++ workload")
    vcvars = Path(installation) / "VC" / "Auxiliary" / "Build" / "vcvars64.bat"
    if not vcvars.is_file():
        raise CiError(f"{vcvars} missing: add the C++ workload to Visual Studio")
    completed = subprocess.run(f'cmd /d /s /c ""{vcvars}" >nul && set"', env=base, capture_output=True,
                               text=True, shell=False)
    if completed.returncode != 0:
        raise CiError(f"{vcvars} failed (exit code {completed.returncode}): {completed.stderr.strip()}")
    result: dict[str, str] = {}
    for line in completed.stdout.splitlines():
        key, separator, value = line.partition("=")
        if separator and key:
            result[key] = value
    return result
