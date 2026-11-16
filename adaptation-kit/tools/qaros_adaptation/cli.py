"""qaros-adaptation: create, check, test, package, pin and verify QAROS web app adaptations.

    qaros-adaptation [--kit <dir>] <command> ...

    init <template> <dir> [--name N] [--address A] [--engine E] [--display-name D] [--id I]
    check <dir> [--ignore CODE ...] [--strict]
    test <dir> [--node PATH] [--no-kit-tests]
    package <dir> [--out PATH] [--unpacked] [--version X.Y.Z] [--sign KEY.pem]
                  [--key-vault V --key-name K --public-key PUB.pem] [--signer NAME]
    pin <dir> <library>
    verify <dir> [--instance ID | --address URL] [--host H] [--port P]
    index <kit> [--version V] [--out PATH] [--check]

The kit is the folder this tool is in (its parent), or `--kit <dir>` (before or after the command).
Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

from . import TOOL_NAME
from . import kit as kitlib
from . import packaging, qarapp, verify
from .profile import (
    PROFILE_SCHEMA_VERSION,
    ProfileError,
    check_adaptation,
    read_profile,
    write_profile,
)

GUIDE = "https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/"
KIT_VARIABLE = "QAROS_ADAPTATION_KIT"
PLACEHOLDERS = ("NAME", "DISPLAY_NAME", "ADDRESS_PATTERN", "ENGINE", "ID")
TEXT_SUFFIXES = {".md", ".js", ".mjs", ".json", ".html", ".css", ".txt"}


class CliError(RuntimeError):
    """A command that cannot run as asked."""


def _kit(args: argparse.Namespace) -> kitlib.Kit:
    return kitlib.load_kit(Path(args.kit) if args.kit else None)


def _folder(path: str) -> Path:
    folder = Path(path).resolve()
    if not (folder / "profile.json").is_file():
        raise CliError(f"{folder} holds no profile.json")
    return folder


# --------------------------------------------------------------------------------------------------
# init
# --------------------------------------------------------------------------------------------------


def templates(kit: Path) -> list[str]:
    folder = kit / "templates"
    return sorted(path.name for path in folder.iterdir() if (path / "profile.json").is_file()) if folder.is_dir() else []


def _slug(text: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-") or "adaptation"


def _ask(label: str, default: str | None, interactive: bool) -> str | None:
    if not interactive:
        return default
    suffix = f" [{default}]" if default else ""
    try:
        answer = input(f"{label}{suffix}: ").strip()
    except EOFError:
        return default
    return answer or default


def _replace_placeholders(value, values: dict[str, str]):
    if isinstance(value, str):
        for key, replacement in values.items():
            value = value.replace("{{" + key + "}}", replacement)
        return value
    if isinstance(value, list):
        return [_replace_placeholders(item, values) for item in value]
    if isinstance(value, dict):
        return {key: _replace_placeholders(item, values) for key, item in value.items()}
    return value


def init_adaptation(
    kit: Path,
    template: str,
    destination: Path,
    *,
    name: str | None = None,
    address: str | None = None,
    engine: str | None = None,
    display_name: str | None = None,
    adaptation_id: str | None = None,
    interactive: bool = False,
) -> dict:
    """Copies `templates/<template>` to `destination` and fills its placeholders. Every other field
    of the template's profile is kept as it is."""
    available = templates(kit)
    if template not in available:
        raise CliError(f"no template '{template}'; the kit has: {', '.join(available) or 'none'}")
    source = kit / "templates" / template
    if destination.exists() and any(destination.iterdir()):
        raise CliError(f"{destination} exists and is not empty")
    template_profile = read_profile(source)

    name = name or _ask("Name (file stem, unique)", _slug(destination.name), interactive)
    display_name = display_name or _ask("Display name", name, interactive)
    address = address or _ask("Address pattern (glob, e.g. https://example.com/app/*)", None, interactive)
    if not address:
        raise CliError("pass --address (the address pattern the adaptation matches, e.g. https://example.com/app/*)")
    default_engine = template_profile.get("Engine", "")
    if "{{" in str(default_engine):
        default_engine = ""
    engine = engine if engine is not None else _ask("Engine (three.js, webgl, webgpu or empty)", default_engine, interactive)
    adaptation_id = adaptation_id or _ask("Id (reverse DNS)", f"com.example.{_slug(name)}", interactive)
    try:
        qarapp.validate_id(adaptation_id)
    except qarapp.PackageError as error:
        raise CliError(str(error)) from error
    values = {
        "NAME": name,
        "DISPLAY_NAME": display_name or name,
        "ADDRESS_PATTERN": address,
        "ENGINE": engine or "",
        "ID": adaptation_id,
    }

    shutil.copytree(source, destination, dirs_exist_ok=True, ignore=shutil.ignore_patterns("node_modules", "__pycache__"))
    for path in destination.rglob("*"):
        if path.is_file() and path.suffix in TEXT_SUFFIXES and path.name != "profile.json":
            text = path.read_text(encoding="utf-8")
            replaced = _replace_placeholders(text, values)
            if replaced != text:
                path.write_bytes(replaced.encode("utf-8"))
    profile = _replace_placeholders(template_profile, values)
    if engine is not None and "Engine" in profile:
        profile["Engine"] = engine
    profile.setdefault("_sv", PROFILE_SCHEMA_VERSION)
    write_profile(destination, profile)
    return profile


# --------------------------------------------------------------------------------------------------
# test
# --------------------------------------------------------------------------------------------------


def _used_libraries(folder: Path, kit: kitlib.Kit) -> set[str]:
    """Names of the kit libraries the adaptation references or pins."""
    names = set()
    for entry in read_profile(folder).get("Scripts") or []:
        if not isinstance(entry, dict):
            continue
        if isinstance(entry.get("Library"), str):
            try:
                names.add(kitlib.parse_reference(entry["Library"])[0])
            except kitlib.KitError:
                pass
        elif isinstance(entry.get("File"), str) and (folder / entry["File"]).is_file():
            try:
                header = kitlib.parse_header((folder / entry["File"]).read_text(encoding="utf-8"))
            except (kitlib.KitError, UnicodeDecodeError):
                header = None
            if header is not None:
                names.add(header.name)
    return names


def kit_tests_for(kit: kitlib.Kit, names: set[str]) -> list[Path]:
    """The kit's library tests (`lib/tests/*.test.mjs`) that name one of `names` (`<name>@<major>`
    or `<name>.js`)."""
    folder = kit.lib / "tests"
    if not folder.is_dir() or not names:
        return []
    pattern = re.compile(r"(?<![\w-])(?:" + "|".join(re.escape(name) for name in sorted(names)) + r")(?:@\d+(?:\.js)?|\.js)")
    return sorted(path for path in folder.glob("*.test.mjs") if pattern.search(path.read_text(encoding="utf-8")))


def run_tests(folder: Path, kit: kitlib.Kit, node: str = "node", kit_tests: bool = True) -> int:
    own = sorted((folder / "test").glob("*.test.mjs")) if (folder / "test").is_dir() else []
    library_tests = kit_tests_for(kit, _used_libraries(folder, kit)) if kit_tests else []
    files = own + library_tests
    if not files:
        print(f"{folder.name}: no tests (add test/*.test.mjs)")
        return 0
    executable = shutil.which(node) or node
    environment = dict(os.environ)
    environment[KIT_VARIABLE] = str(kit.directory)
    print(f"{folder.name}: {len(own)} own and {len(library_tests)} kit test files")
    try:
        completed = subprocess.run([executable, "--test", *map(str, files)], cwd=folder, env=environment, check=False)
    except FileNotFoundError as error:
        raise CliError(f"Node.js was not found ({node}); install it or pass --node") from error
    return completed.returncode


# --------------------------------------------------------------------------------------------------
# pin
# --------------------------------------------------------------------------------------------------


def pin_library(folder: Path, kit: kitlib.Kit, reference: str, force: bool = False) -> tuple[kitlib.Library, Path, int]:
    """Replaces every script entry naming `reference` (`name`, `name@major` or `name.js`) by a copy
    of the kit's file at `scripts/<name>@<major>.js`. The copy keeps its header, so the pinned
    version stays visible. Returns the library, the copy and the number of entries changed."""
    wanted_major = None
    if "@" in reference or reference.endswith(".js"):
        wanted_name, wanted_major = kitlib.parse_reference(reference)
    else:
        wanted_name = reference
    profile = read_profile(folder)
    scripts = profile.get("Scripts") or []
    matches = []
    for position, entry in enumerate(scripts):
        if not isinstance(entry, dict) or not isinstance(entry.get("Library"), str):
            continue
        try:
            name, major = kitlib.parse_reference(entry["Library"])
        except kitlib.KitError:
            continue
        if name == wanted_name and (wanted_major is None or major == wanted_major):
            matches.append((position, major))
    if not matches:
        raise CliError(f"no script of {folder.name} references library '{reference}'")
    majors = {major for _, major in matches}
    if len(majors) > 1:
        raise CliError(f"'{reference}' matches several majors ({', '.join(f'@{m}' for m in sorted(majors))}); name one")
    library = kit.resolve(f"{wanted_name}@{majors.pop()}")
    data = kit.read(library)
    relative = f"scripts/{library.name}@{library.major}.js"
    target = folder / relative
    if target.exists() and target.read_bytes() != data and not force:
        raise CliError(f"{relative} exists and differs from the kit's {library.key} {library.version}; pass --force to replace it")
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
    for position, _ in matches:
        entry = scripts[position]
        # Keep the entry's other fields, in order, with File where Library was.
        scripts[position] = {("File" if key == "Library" else key): (relative if key == "Library" else value) for key, value in entry.items()}
    write_profile(folder, profile)
    return library, target, len(matches)


# --------------------------------------------------------------------------------------------------
# verify
# --------------------------------------------------------------------------------------------------


def _pack_shader_library(folder: Path, kit: kitlib.Kit) -> Path | None:
    """The first script of the profile (kit library or own file) that defines the WebGL depth pack
    shaders (`fullScreenVertex`, `packFragment`): webgl-atlas-pack@1, or a pinned or own copy."""
    for entry in read_profile(folder).get("Scripts") or []:
        if not isinstance(entry, dict):
            continue
        path = None
        if isinstance(entry.get("Library"), str):
            try:
                path = kit.path(kit.resolve(entry["Library"]))
            except kitlib.KitError:
                continue
        elif isinstance(entry.get("File"), str):
            path = folder / entry["File"]
        if path is None or not path.is_file():
            continue
        text = path.read_text(encoding="utf-8")
        if "const packFragment = `" in text and "const fullScreenVertex = `" in text:
            return path
    return None


def run_verify(args: argparse.Namespace) -> int:
    folder = _folder(args.dir)
    kit = _kit(args)
    profile = read_profile(folder)
    mcp = verify.Mcp(args.host, args.port)
    instance = args.instance or verify.find_instance(mcp, args.address, None if args.address else profile.get("AddressPattern"))
    failures = []
    if not args.no_readback:
        library = _pack_shader_library(folder, kit)
        if library is not None:
            try:
                print(f"depth pack readback OK: {verify.pack_readback(mcp, instance, library)}")
            except verify.VerifyError as error:
                failures.append(f"depth pack readback: {error}")
    detail = verify.next_request(mcp, instance)
    image = verify.capture_atlas(mcp, instance)
    background = tuple(int(part) for part in args.background.split(","))
    for coverage in verify.analyse(image, detail, background, args.threshold):
        line = (
            f"{coverage.label}: {coverage.blocks} coloured blocks, depth {coverage.depth:.2%}, "
            f"thin {coverage.thin:.2%} of {coverage.thin_blocks}"
        )
        if coverage.alpha_blocks:
            line += f", alpha {coverage.alpha:.2%}"
        print(line + ("; " + "; ".join(coverage.notes) if coverage.notes else ""))
        if coverage.blocks == 0:
            failures.append(f"{coverage.label}: no coloured pixels (is the scene in view? is --background right?)")
        if coverage.depth < args.min_coverage:
            failures.append(f"{coverage.label}: depth coverage {coverage.depth:.2%} below {args.min_coverage:.2%}")
        if coverage.thin < args.min_coverage:
            failures.append(f"{coverage.label}: thin-line depth coverage {coverage.thin:.2%} below {args.min_coverage:.2%}")
        if profile.get("TransportAlpha") and coverage.alpha < args.min_coverage:
            failures.append(f"{coverage.label}: alpha coverage {coverage.alpha:.2%} below {args.min_coverage:.2%}")
    for failure in failures:
        print(f"error: {failure}", file=sys.stderr)
    return 1 if failures else 0


# --------------------------------------------------------------------------------------------------
# Command line
# --------------------------------------------------------------------------------------------------


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog=TOOL_NAME, description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--kit", help="the adaptation kit folder (default: the one this tool belongs to)")
    commands = parser.add_subparsers(dest="command", required=True)

    def command(name: str, help_text: str) -> argparse.ArgumentParser:
        sub = commands.add_parser(name, help=help_text, description=help_text)
        sub.add_argument("--kit", default=argparse.SUPPRESS, help="the adaptation kit folder")
        return sub

    sub = command("init", "create an adaptation from a template")
    sub.add_argument("template", help="threejs, webgl, webgpu or webgpu-redirect")
    sub.add_argument("dir")
    sub.add_argument("--name")
    sub.add_argument("--address", help="address pattern (glob) the adaptation matches")
    sub.add_argument("--engine")
    sub.add_argument("--display-name")
    sub.add_argument("--id", dest="adaptation_id")

    sub = command("check", "validate profile.json, the library references and the known pitfalls")
    sub.add_argument("dir")
    sub.add_argument("--ignore", action="append", default=[], metavar="CODE", help="skip a lint warning (repeatable)")
    sub.add_argument("--strict", action="store_true", help="warnings fail too")

    sub = command("test", "run the adaptation's Node tests and the kit tests of the libraries it uses")
    sub.add_argument("dir")
    sub.add_argument("--node", default="node")
    sub.add_argument("--no-kit-tests", action="store_true")

    sub = command("package", "build the .qarapp package (or the unpacked folder)")
    sub.add_argument("dir")
    sub.add_argument("--out", help="the .qarapp file or a folder for it; with --unpacked, the folder to write")
    sub.add_argument("--unpacked", action="store_true", help="write the package's folder instead of a ZIP")
    sub.add_argument("--version", help="override Version (X.Y.Z)")
    sub.add_argument("--sign", type=Path, metavar="KEY.pem", help="sign with a local PEM private key (tests, local runs)")
    sub.add_argument("--key-vault", help="sign with a key in this Azure Key Vault (needs --public-key)")
    sub.add_argument("--key-name", default="qaros-release")
    sub.add_argument("--public-key", type=Path, help="the signing key's public half (PEM); the package is verified against it")
    sub.add_argument("--signer", default=qarapp.DEFAULT_SIGNER)

    sub = command("pin", "replace a Library reference with a pinned copy (File)")
    sub.add_argument("dir")
    sub.add_argument("library", help="name, name@major or name.js")
    sub.add_argument("--force", action="store_true", help="replace an existing, different pinned copy")

    sub = command("verify", "check depth, thin-line and alpha coverage of a running web app source")
    sub.add_argument("dir")
    sub.add_argument("--instance", help="web app source instance id (default: the one matching the profile)")
    sub.add_argument("--address", help="pick the source loaded at this address")
    sub.add_argument("--host", default="127.0.0.1")
    sub.add_argument("--port", type=int, default=verify.default_port())
    sub.add_argument("--background", default="0,0,0", help="R,G,B of an empty pixel of a user view")
    sub.add_argument("--threshold", type=int, default=16, help="channel distance from --background that counts as colour")
    sub.add_argument("--min-coverage", type=float, default=0.99)
    sub.add_argument("--no-readback", action="store_true", help="skip the depth pack shader readback")

    sub = command("index", "write or check the kit's lib/index.json")
    sub.add_argument("kit_dir", metavar="kit")
    sub.add_argument("--version", help="KitVersion (the QAROS version; default dev)")
    sub.add_argument("--out", type=Path, help="default <kit>/lib/index.json")
    sub.add_argument("--check", action="store_true", help="fail when the index does not match the files")
    return parser


def _print_report(folder: Path, report) -> None:
    for finding in report.findings:
        print(f"{folder.name}: {finding}", file=sys.stderr if finding.level == "error" else sys.stdout)
    print(f"{folder.name}: {len(report.errors)} errors, {len(report.warnings)} warnings")


def dispatch(args: argparse.Namespace) -> int:
    if args.command == "index":
        kit = Path(args.kit_dir).resolve()
        if args.check:
            errors = kitlib.check_index(kit, args.out, args.version)
            for error in errors:
                print(f"error: {error}", file=sys.stderr)
            if not errors:
                print(f"{args.out or kit / 'lib' / kitlib.INDEX_NAME}: matches {kit / 'lib'}")
            return 1 if errors else 0
        out = kitlib.write_index(kit, args.version, args.out)
        print(f"wrote {out}")
        return 0

    kit_dir = Path(args.kit).resolve() if args.kit else kitlib.default_kit_directory()
    if args.command == "init":
        destination = Path(args.dir).resolve()
        profile = init_adaptation(
            kit_dir, args.template, destination,
            name=args.name, address=args.address, engine=args.engine, display_name=args.display_name,
            adaptation_id=args.adaptation_id, interactive=sys.stdin.isatty() and sys.stdout.isatty(),
        )
        print(f"created {destination} ({profile.get('Id')}) from template {args.template}")
        print(f"next: {TOOL_NAME} check {args.dir}; guide: {GUIDE}")
        return 0
    if args.command == "check":
        folder = _folder(args.dir)
        report = check_adaptation(folder, _kit(args), set(args.ignore))
        _print_report(folder, report)
        return 1 if report.errors or (args.strict and report.warnings) else 0
    if args.command == "test":
        return run_tests(_folder(args.dir), _kit(args), args.node, not args.no_kit_tests)
    if args.command == "package":
        signing = packaging.signing_options(args.sign, args.key_vault, args.key_name, args.public_key, args.signer)
        trusted = None
        if args.public_key is not None and signing is not None:
            trusted = [(args.signer, signing.public_der)]
        result = packaging.build_package(
            _folder(args.dir), Path(args.out) if args.out else None,
            unpacked=args.unpacked, version=args.version, signing=signing, trusted=trusted, kit=_kit(args),
        )
        state = f"signed by {result.verification.signer}" if result.verification.signed else "unsigned"
        print(f"{result.path}: ok, {result.verification.adaptation_id} {result.verification.version}, {state}")
        return 0
    if args.command == "pin":
        library, target, count = pin_library(_folder(args.dir), _kit(args), args.library, args.force)
        print(f"pinned {library.key} {library.version} at {target} ({count} script entr{'y' if count == 1 else 'ies'})")
        return 0
    if args.command == "verify":
        return run_verify(args)
    raise CliError(f"unknown command {args.command}")


def main(argv: list[str] | None = None) -> int:
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(errors="backslashreplace")  # a path the console cannot show never fails a command
    args = build_parser().parse_args(argv)
    try:
        return dispatch(args)
    except (CliError, kitlib.KitError, ProfileError, packaging.BuildError, qarapp.PackageError, verify.VerifyError, OSError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
