"""The adaptation kit's libraries: one file per `name@major` in `<kit>/lib`, each with a header

    // @qaros-library <name>@<major>.<minor>
    // @stage preDocument|afterLoad
    // @needs <name>@<major> ...      (optional)

and the index QAROS resolves `Library` references with, `<kit>/lib/index.json`:

    {"_sv": 1, "KitVersion": "<QAROS version or dev>",
     "Libraries": [{"Name", "Major", "Minor", "Stage", "Needs", "File", "Sha256"}, ...]}

sorted by name, then major. The index is generated when the kit is staged (never committed); run
from the repository, the tool computes it in memory from the files.
"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass, field
from pathlib import Path

INDEX_SCHEMA_VERSION = 1
INDEX_NAME = "index.json"
DEV_VERSION = "dev"
STAGES = ("preDocument", "afterLoad")

_FILE_NAME = re.compile(r"^([a-z0-9][a-z0-9-]*)@([0-9]+)\.js$")
_REFERENCE = re.compile(r"^([a-z0-9][a-z0-9-]*)@([0-9]+)$")
_LEGACY_REFERENCE = re.compile(r"^([a-z0-9][a-z0-9-]*)\.js$")
_LIBRARY_LINE = re.compile(r"^//\s*@qaros-library\s+([a-z0-9][a-z0-9-]*)@([0-9]+)\.([0-9]+)\s*$")
_STAGE_LINE = re.compile(r"^//\s*@stage\s+(\S+)\s*$")
_NEEDS_LINE = re.compile(r"^//\s*@needs\s+(.*?)\s*$")


class KitError(RuntimeError):
    """A kit, library, header, index or reference that is invalid or not provided."""


def default_kit_directory() -> Path:
    """The kit this tool belongs to: the parent of `tools/`."""
    return Path(__file__).resolve().parent.parent.parent


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


# --------------------------------------------------------------------------------------------------
# References
# --------------------------------------------------------------------------------------------------


def parse_reference(reference: str) -> tuple[str, int]:
    """`name@major`, or the earlier build-time form `name.js`, which means `name@1`."""
    if not isinstance(reference, str):
        raise KitError(f"a library reference is a string such as 'render-frame-core@1', not {reference!r}")
    match = _REFERENCE.match(reference)
    if match:
        return match[1], int(match[2])
    match = _LEGACY_REFERENCE.match(reference)
    if match:
        return match[1], 1
    raise KitError(f"'{reference}' is not a library reference (name@major, e.g. render-frame-core@1)")


def is_legacy_reference(reference: str) -> bool:
    return isinstance(reference, str) and _LEGACY_REFERENCE.match(reference) is not None


# --------------------------------------------------------------------------------------------------
# Headers
# --------------------------------------------------------------------------------------------------


@dataclass(frozen=True)
class LibraryHeader:
    name: str
    major: int
    minor: int
    stage: str
    needs: tuple[str, ...] = ()

    @property
    def key(self) -> str:
        return f"{self.name}@{self.major}"


def parse_header(text: str, what: str = "library") -> LibraryHeader | None:
    """The header of the file's leading comment block, or None when it has no `@qaros-library`
    line. A header with a `@qaros-library` line but no valid `@stage` is an error."""
    name = major = minor = stage = None
    needs: list[str] = []
    for line in text.lstrip("﻿").splitlines():
        line = line.strip()
        if not line:
            continue
        if not line.startswith("//"):
            break
        if match := _LIBRARY_LINE.match(line):
            name, major, minor = match[1], int(match[2]), int(match[3])
        elif match := _STAGE_LINE.match(line):
            stage = match[1]
        elif match := _NEEDS_LINE.match(line):
            for need in match[1].split():
                parsed = _REFERENCE.match(need)
                if parsed is None:
                    raise KitError(f"{what}: '@needs {need}' is not name@major")
                needs.append(need)
        elif "@qaros-library" in line:
            raise KitError(f"{what}: '{line}' is not '// @qaros-library <name>@<major>.<minor>'")
    if name is None:
        return None
    if stage not in STAGES:
        raise KitError(f"{what}: the header has no '// @stage preDocument' or '// @stage afterLoad' line")
    return LibraryHeader(name, major, minor, stage, tuple(needs))


# --------------------------------------------------------------------------------------------------
# Libraries and the index
# --------------------------------------------------------------------------------------------------


@dataclass(frozen=True)
class Library:
    name: str
    major: int
    minor: int
    stage: str
    needs: tuple[str, ...]
    file: str
    sha256: str

    @property
    def key(self) -> str:
        return f"{self.name}@{self.major}"

    @property
    def version(self) -> str:
        return f"{self.major}.{self.minor}"

    def index_entry(self) -> dict:
        return {
            "Name": self.name,
            "Major": self.major,
            "Minor": self.minor,
            "Stage": self.stage,
            "Needs": list(self.needs),
            "File": self.file,
            "Sha256": self.sha256,
        }

    @staticmethod
    def from_index_entry(entry: dict) -> "Library":
        try:
            library = Library(
                str(entry["Name"]),
                int(entry["Major"]),
                int(entry.get("Minor", 0)),
                str(entry.get("Stage", "afterLoad")),
                tuple(str(need) for need in entry.get("Needs", [])),
                str(entry["File"]),
                str(entry["Sha256"]).lower(),
            )
        except (KeyError, TypeError, ValueError) as error:
            raise KitError(f"index entry {entry!r} does not parse: {error}") from error
        return library


def read_library(path: Path) -> Library:
    """A library file of `<kit>/lib`: its name must match its header."""
    match = _FILE_NAME.match(path.name)
    if match is None:
        raise KitError(f"{path.name}: a library file is named <name>@<major>.js")
    header = parse_header(path.read_text(encoding="utf-8"), path.name)
    if header is None:
        raise KitError(f"{path.name}: no '// @qaros-library {match[1]}@{match[2]}.<minor>' header in its first lines")
    if (header.name, header.major) != (match[1], int(match[2])):
        raise KitError(f"{path.name}: the header names {header.name}@{header.major}, the file name {match[1]}@{match[2]}")
    return Library(header.name, header.major, header.minor, header.stage, header.needs, path.name, sha256_file(path))


def scan_libraries(kit: Path) -> list[Library]:
    lib = kit / "lib"
    if not lib.is_dir():
        raise KitError(f"{kit} is not an adaptation kit: it has no lib/ folder")
    libraries = [read_library(path) for path in lib.glob("*.js") if path.is_file()]
    libraries.sort(key=lambda library: (library.name, library.major))
    keys = {library.key for library in libraries}
    for library in libraries:
        for need in library.needs:
            if need not in keys:
                raise KitError(f"{library.file}: needs {need}, which the kit does not provide")
    return libraries


def compute_index(kit: Path, version: str | None = None) -> dict:
    return {
        "_sv": INDEX_SCHEMA_VERSION,
        "KitVersion": version or DEV_VERSION,
        "Libraries": [library.index_entry() for library in scan_libraries(kit)],
    }


def index_bytes(index: dict) -> bytes:
    return (json.dumps(index, indent=2, ensure_ascii=False) + "\n").encode("utf-8")


def write_index(kit: Path, version: str | None = None, out: Path | None = None) -> Path:
    out = out or kit / "lib" / INDEX_NAME
    data = index_bytes(compute_index(kit, version))
    out.parent.mkdir(parents=True, exist_ok=True)
    if not out.is_file() or out.read_bytes() != data:
        temporary = out.with_name(out.name + ".tmp")
        temporary.write_bytes(data)
        temporary.replace(out)
    return out


def check_index(kit: Path, index_path: Path | None = None, version: str | None = None) -> list[str]:
    """Differences between an existing index and the kit's files; empty when it matches."""
    index_path = index_path or kit / "lib" / INDEX_NAME
    if not index_path.is_file():
        return [f"{index_path} does not exist"]
    try:
        index = json.loads(index_path.read_text(encoding="utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        return [f"{index_path} is not valid JSON: {error}"]
    expected = compute_index(kit, version)
    errors = []
    if not isinstance(index, dict):
        return [f"{index_path} is not a JSON object"]
    if index.get("_sv") != INDEX_SCHEMA_VERSION:
        errors.append(f"_sv is {index.get('_sv')!r}, expected {INDEX_SCHEMA_VERSION}")
    if version is not None and index.get("KitVersion") != version:
        errors.append(f"KitVersion is {index.get('KitVersion')!r}, expected {version!r}")
    listed = {f"{entry.get('Name')}@{entry.get('Major')}": entry for entry in index.get("Libraries", []) if isinstance(entry, dict)}
    wanted = {f"{entry['Name']}@{entry['Major']}": entry for entry in expected["Libraries"]}
    for key in sorted(set(listed) | set(wanted)):
        if key not in listed:
            errors.append(f"{key} is missing from the index")
        elif key not in wanted:
            errors.append(f"{key} is in the index but not in lib/")
        elif listed[key] != wanted[key]:
            fields = sorted(name for name in set(listed[key]) | set(wanted[key]) if listed[key].get(name) != wanted[key].get(name))
            errors.append(f"{key}: {', '.join(fields)} differ from the file")
    if not errors and index.get("Libraries") != expected["Libraries"]:
        errors.append("the libraries are not sorted by name and major")
    return errors


@dataclass
class Kit:
    directory: Path
    version: str
    libraries: dict[str, Library] = field(default_factory=dict)
    from_index: bool = False

    @property
    def lib(self) -> Path:
        return self.directory / "lib"

    def path(self, library: Library) -> Path:
        return self.lib / library.file

    def majors(self, name: str) -> list[int]:
        return sorted(library.major for library in self.libraries.values() if library.name == name)

    def resolve(self, reference: str) -> Library:
        """The library `reference` names; the error says what the kit provides instead."""
        name, major = parse_reference(reference)
        library = self.libraries.get(f"{name}@{major}")
        if library is not None:
            return library
        majors = self.majors(name)
        if not majors:
            raise KitError(f"requires `{name}@{major}`, which this kit does not provide (kit {self.version})")
        provided = ", ".join(f"@{value}" for value in majors)
        raise KitError(f"requires `{name}@{major}`, this kit provides {provided} (kit {self.version})")

    def read(self, library: Library) -> bytes:
        """The library's bytes, checked against the index."""
        data = self.path(library).read_bytes()
        if hashlib.sha256(data).hexdigest() != library.sha256:
            raise KitError(f"{library.file} does not match {INDEX_NAME} (re-run `index`)")
        return data


def load_kit(directory: Path | None = None) -> Kit:
    """The kit at `directory` (default: the tool's own): from `lib/index.json` when the kit was
    staged, else computed from the files (repository)."""
    directory = (directory or default_kit_directory()).resolve()
    index_path = directory / "lib" / INDEX_NAME
    if not index_path.is_file():
        libraries = scan_libraries(directory)
        return Kit(directory, DEV_VERSION, {library.key: library for library in libraries}, False)
    try:
        index = json.loads(index_path.read_text(encoding="utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise KitError(f"{index_path} is not valid JSON: {error}") from error
    if not isinstance(index, dict) or not isinstance(index.get("Libraries"), list):
        raise KitError(f"{index_path} has no Libraries list")
    if isinstance(index.get("_sv"), int) and index["_sv"] > INDEX_SCHEMA_VERSION:
        raise KitError(f"{index_path} has schema version {index['_sv']}, newer than this tool reads ({INDEX_SCHEMA_VERSION})")
    libraries = [Library.from_index_entry(entry) for entry in index["Libraries"]]
    return Kit(directory, str(index.get("KitVersion", DEV_VERSION)), {library.key: library for library in libraries}, True)
