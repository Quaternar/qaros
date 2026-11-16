"""An adaptation folder's `profile.json`: read, validate and rewrite.

Rewrites (`init`, `pin`, `package`) keep every field they do not change, unknown ones included, in
their order, so a profile written by a newer QAROS or tool survives an older one.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from pathlib import Path

from . import kit as kitlib
from . import qarapp

PROFILE_NAME = "profile.json"
#: The profile schema version this tool writes and fully knows (`_sv`).
PROFILE_SCHEMA_VERSION = 3
CHANNELS = ("public", "developer", "debug")
DEFAULT_CHANNEL = "public"
PAGE_REGION_TOKENS = ("__PAGE_REGION_WIDTH__", "__PAGE_REGION_HEIGHT__")

#: Every top-level field this tool knows; others are kept but reported, since they are usually typos.
KNOWN_FIELDS = {
    "_sv", "Name", "AddressPattern", "Engine", "PageRegion", "ShareBrowserProfile", "Scripts",
    "AppScale", "AppScaleByAddress", "RenderScale", "ContinuousRendering", "TransportAlpha", "Id",
    "DisplayName", "Version", "DerivedFrom", "Panels", "StartAddress", "CommonName", "Files",
    "Signature", "Channel", "RenderRate", "Antialias",
}
#: Samples per pixel `Antialias` may ask for; the page takes the largest its GPU supports up to it.
ANTIALIAS_SAMPLES = (1, 2, 4, 8, 16)
KNOWN_SCRIPT_FIELDS = {"Name", "Stage", "JavaScript", "SourceUrl", "Url", "url", "File", "Library"}
REQUIRED_FIELDS = {
    "Name": str,
    "AddressPattern": str,
    "Engine": str,
    "PageRegion": dict,
    "ShareBrowserProfile": bool,
    "Scripts": list,
    "Id": str,
    "Version": str,
}


class ProfileError(RuntimeError):
    """A profile that cannot be read or rewritten."""


# --------------------------------------------------------------------------------------------------
# Read and write
# --------------------------------------------------------------------------------------------------


def read_profile(folder: Path) -> dict:
    path = folder / PROFILE_NAME
    if not path.is_file():
        raise ProfileError(f"{path} does not exist")
    try:
        value = json.loads(path.read_bytes().decode("utf-8-sig"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ProfileError(f"{path} is not valid JSON: {error}") from error
    if not isinstance(value, dict):
        raise ProfileError(f"{path} is not a JSON object")
    return value


def profile_bytes(profile: dict) -> bytes:
    return (json.dumps(profile, indent=2, ensure_ascii=False) + "\n").encode("utf-8")


def write_profile(folder: Path, profile: dict) -> None:
    (folder / PROFILE_NAME).write_bytes(profile_bytes(profile))


def matches_address(pattern: str, address: str) -> bool:
    """QAROS's address glob: `*` any run (including `/`), `?` exactly one character, nothing else
    special."""
    regex = "".join(".*" if char == "*" else "." if char == "?" else re.escape(char) for char in pattern)
    return re.fullmatch(regex, address, re.S) is not None


def app_scale_for(profile: dict, address: str) -> float | None:
    """The app scale QAROS applies for `address`: the first matching `AppScaleByAddress` entry,
    else `AppScale`, else None (the volume keeps its own scale)."""
    for entry in profile.get("AppScaleByAddress") or []:
        if isinstance(entry, dict) and matches_address(str(entry.get("AddressPattern", "")), address):
            return entry.get("AppScale")
    return profile.get("AppScale")


# --------------------------------------------------------------------------------------------------
# Findings
# --------------------------------------------------------------------------------------------------


@dataclass(frozen=True)
class Finding:
    level: str  # "error" or "warning"
    code: str
    message: str

    def __str__(self) -> str:
        return f"{self.level}: [{self.code}] {self.message}"


@dataclass
class Report:
    findings: list[Finding] = field(default_factory=list)

    def error(self, code: str, message: str) -> None:
        self.findings.append(Finding("error", code, message))

    def warning(self, code: str, message: str) -> None:
        self.findings.append(Finding("warning", code, message))

    @property
    def errors(self) -> list[Finding]:
        return [finding for finding in self.findings if finding.level == "error"]

    @property
    def warnings(self) -> list[Finding]:
        return [finding for finding in self.findings if finding.level == "warning"]

    @property
    def ok(self) -> bool:
        return not self.errors


# --------------------------------------------------------------------------------------------------
# Scripts
# --------------------------------------------------------------------------------------------------


@dataclass
class ScriptInfo:
    """One script entry, resolved: what it provides (`name@major`) and needs."""

    index: int
    name: str
    stage: str
    kind: str  # "library", "file" or "inline"
    reference: str = ""  # Library value
    file: str = ""  # File value
    provides: str = ""  # name@major of a kit library or a pinned copy
    needs: tuple[str, ...] = ()
    text: str = ""  # own script source (File or inline), for the lints


def _number(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _check_scalars(profile: dict, report: Report) -> None:
    sv = profile.get("_sv")
    if sv is None:
        report.error(
            "schema",
            f"profile.json has no \"_sv\": {PROFILE_SCHEMA_VERSION}; QAROS reads a profile without it as the first "
            "format and replaces its Id, DisplayName and Version",
        )
    elif not isinstance(sv, int) or isinstance(sv, bool) or sv < 1:
        report.error("schema", f"_sv must be a positive integer, not {sv!r}")
    elif sv > PROFILE_SCHEMA_VERSION:
        report.error("schema", f"_sv {sv} is newer than this kit knows ({PROFILE_SCHEMA_VERSION}); use the kit of a newer QAROS")
    elif sv < PROFILE_SCHEMA_VERSION:
        report.warning("schema", f"_sv {sv} is an earlier profile format; QAROS migrates it on load")

    for name, kind in REQUIRED_FIELDS.items():
        if name not in profile:
            report.error("schema", f"required field '{name}' is missing")
        elif not isinstance(profile[name], kind):
            report.error("schema", f"'{name}' must be a {kind.__name__ if kind is not dict else 'object'}")
    for name in sorted(set(profile) - KNOWN_FIELDS):
        report.warning("unknown-field", f"field '{name}' is not known to this kit (kept as is; a typo?)")

    if isinstance(profile.get("Id"), str):
        try:
            qarapp.validate_id(profile["Id"])
        except qarapp.PackageError as error:
            report.error("schema", str(error))
    if isinstance(profile.get("Version"), str):
        try:
            qarapp.Version.parse(profile["Version"])
        except qarapp.PackageError as error:
            report.error("schema", str(error))
    if isinstance(profile.get("Name"), str) and not profile["Name"].strip():
        report.error("schema", "'Name' is empty")
    if isinstance(profile.get("AddressPattern"), str) and not profile["AddressPattern"].strip():
        report.error("schema", "'AddressPattern' is empty")
    for name in ("DisplayName", "StartAddress", "CommonName", "Panels"):
        if name in profile and not isinstance(profile[name], str):
            report.error("schema", f"'{name}' must be a string")
    if isinstance(profile.get("Panels"), str):
        try:
            qarapp.validate_path(profile["Panels"])
        except qarapp.PackageError as error:
            report.error("schema", f"Panels: {error}")
    for name in ("ContinuousRendering", "TransportAlpha"):
        if name in profile and not isinstance(profile[name], bool):
            report.error("schema", f"'{name}' must be true or false")

    region = profile.get("PageRegion")
    if isinstance(region, dict):
        for name in ("Width", "Height"):
            value = region.get(name)
            if not isinstance(value, int) or isinstance(value, bool) or value < 0:
                report.error("schema", f"PageRegion.{name} must be a non-negative integer")

    if "AppScale" in profile and not (_number(profile["AppScale"]) and profile["AppScale"] > 0):
        report.error("schema", "'AppScale' must be a number above 0")
    if "RenderScale" in profile and not (_number(profile["RenderScale"]) and 0 < profile["RenderScale"] <= 1):
        report.error("schema", "'RenderScale' must be a number in (0, 1]")
    if "RenderRate" in profile and not (_number(profile["RenderRate"]) and 1 <= profile["RenderRate"] <= 120):
        report.error("schema", "'RenderRate' must be frames per second in [1, 120] (absent: 30)")
    antialias = profile.get("Antialias", 1)
    if isinstance(antialias, bool) or antialias not in ANTIALIAS_SAMPLES:
        report.error("schema", "'Antialias' must be samples per pixel, one of 1, 2, 4, 8, 16 (absent: 1)")
    if "Channel" in profile and profile["Channel"] not in CHANNELS:
        report.error("schema", f"'Channel' must be one of {', '.join(CHANNELS)}, not {profile['Channel']!r}")

    by_address = profile.get("AppScaleByAddress")
    if by_address is not None:
        if not isinstance(by_address, list):
            report.error("schema", "'AppScaleByAddress' must be a list of {\"AddressPattern\", \"AppScale\"}")
        else:
            for position, entry in enumerate(by_address):
                where = f"AppScaleByAddress[{position}]"
                if not isinstance(entry, dict):
                    report.error("schema", f"{where} must be an object")
                    continue
                if not isinstance(entry.get("AddressPattern"), str) or not entry["AddressPattern"]:
                    report.error("schema", f"{where}.AddressPattern must be a non-empty string")
                if not (_number(entry.get("AppScale")) and entry["AppScale"] > 0):
                    report.error("schema", f"{where}.AppScale must be a number above 0")


def _script_entries(folder: Path, profile: dict, kit: kitlib.Kit | None, report: Report) -> list[ScriptInfo]:
    scripts = profile.get("Scripts")
    if not isinstance(scripts, list):
        return []
    infos: list[ScriptInfo] = []
    names: set[str] = set()
    for position, entry in enumerate(scripts):
        if not isinstance(entry, dict):
            report.error("script", f"Scripts[{position}] must be an object")
            continue
        name = entry.get("Name") if isinstance(entry.get("Name"), str) else ""
        where = f"script '{name}'" if name else f"Scripts[{position}]"
        if not name:
            report.warning("script", f"{where} has no Name; QAROS names a failing script by it")
        elif name in names:
            report.warning("script", f"{where}: the name is used twice")
        names.add(name)
        for key in sorted(set(entry) - KNOWN_SCRIPT_FIELDS):
            report.warning("unknown-field", f"{where}: field '{key}' is not known to this kit (kept as is)")
        stage = entry.get("Stage", "afterLoad")
        if stage not in kitlib.STAGES:
            report.error(
                "script",
                f"{where}: Stage {stage!r} is not preDocument or afterLoad (QAROS would run it as afterLoad)",
            )
            stage = "afterLoad"
        has_file = "File" in entry and entry["File"] is not None
        has_library = "Library" in entry and entry["Library"] is not None
        info = ScriptInfo(position, name, stage, "inline")
        if has_file and has_library:
            report.error("script", f"{where} names both a File and a Library; they are mutually exclusive")
            continue
        if has_library:
            info.kind = "library"
            info.reference = entry["Library"]
            try:
                library_name, major = kitlib.parse_reference(entry["Library"])
            except kitlib.KitError as error:
                report.error("library", f"{where}: {error}")
                continue
            if kitlib.is_legacy_reference(entry["Library"]):
                report.warning(
                    "legacy-library",
                    f"{where}: Library '{entry['Library']}' is the earlier form; it means `{library_name}@1`, write that",
                )
            if kit is not None:
                try:
                    library = kit.resolve(entry["Library"])
                except kitlib.KitError as error:
                    report.error("library", f"{where} {error}")
                    continue
                info.provides, info.needs = library.key, library.needs
                if library.stage != stage:
                    report.error("script", f"{where}: {library.key} runs at {library.stage}, the entry says {stage}")
            else:
                info.provides = f"{library_name}@{major}"
        elif has_file:
            info.kind = "file"
            info.file = entry["File"]
            if not isinstance(info.file, str):
                report.error("script", f"{where}: File must be a string")
                continue
            try:
                qarapp.validate_path(info.file)
            except qarapp.PackageError as error:
                report.error("script", f"{where}: {error}")
                continue
            path = folder / info.file
            if not path.is_file():
                report.error("script", f"{where}: file '{info.file}' does not exist")
                continue
            try:
                info.text = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                report.error("script", f"{where}: '{info.file}' is not UTF-8")
                continue
            try:
                header = kitlib.parse_header(info.text, info.file)
            except kitlib.KitError as error:
                report.error("script", str(error))
                header = None
            if header is not None:
                # A pinned copy of a kit library: it provides and needs what its header says.
                info.provides, info.needs = header.key, header.needs
                if header.stage != stage:
                    report.error("script", f"{where}: the pinned {header.key} runs at {header.stage}, the entry says {stage}")
        else:
            text = entry.get("JavaScript")
            if not isinstance(text, str) or not text.strip():
                report.error("script", f"{where} has no File, Library or JavaScript")
                continue
            info.text = text
        infos.append(info)
    return infos


def _check_needs(infos: list[ScriptInfo], report: Report) -> None:
    """Every library's `@needs` must run before it: preDocument scripts run first (in declared
    order), then afterLoad ones."""
    ordered = [info for info in infos if info.stage == "preDocument"] + [info for info in infos if info.stage == "afterLoad"]
    provided: set[str] = set()
    for info in ordered:
        for need in info.needs:
            if need not in provided:
                report.error(
                    "needs",
                    f"script '{info.name}' ({info.provides}) needs {need} before it; list a script with "
                    f"\"Library\": \"{need}\" earlier (preDocument scripts run before afterLoad ones)",
                )
        if info.provides:
            if info.provides in provided:
                report.warning("needs", f"script '{info.name}': {info.provides} is listed twice")
            provided.add(info.provides)


def check_adaptation(folder: Path, kit: kitlib.Kit | None, ignore: set[str] | frozenset[str] = frozenset()) -> Report:
    """Schema, library references and the known-pitfall lints (lints.py) of one adaptation folder.
    `ignore` names lint codes to skip."""
    from . import lints  # noqa: PLC0415 (lints imports this module)

    report = Report()
    try:
        profile = read_profile(folder)
    except ProfileError as error:
        report.error("schema", str(error))
        return report
    _check_scalars(profile, report)
    infos = _script_entries(folder, profile, kit, report)
    _check_needs(infos, report)
    if "Files" in profile or "Signature" in profile:
        try:
            qarapp.verify_folder(folder, None)
        except qarapp.PackageError as error:
            report.error("manifest", f"{error} (remove Files and Signature from a source folder: `package` writes them)")
    lints.run(folder, profile, infos, report)
    if ignore:
        report.findings = [finding for finding in report.findings if finding.level == "error" or finding.code not in ignore]
    return report


def library_keys(profile: dict) -> list[str]:
    """`name@major` of every Library reference of the profile, in order."""
    keys = []
    for entry in profile.get("Scripts") or []:
        if isinstance(entry, dict) and isinstance(entry.get("Library"), str):
            try:
                name, major = kitlib.parse_reference(entry["Library"])
            except kitlib.KitError:
                continue
            keys.append(f"{name}@{major}")
    return keys

