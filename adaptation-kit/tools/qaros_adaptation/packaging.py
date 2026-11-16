"""`package`: an adaptation folder to a `.qarapp` package or its unpacked folder.

Steps, the same for every caller (the CLI, CMake for the adaptations QAROS ships, a repository's own
build script):

1. `check` the folder against the kit (errors stop the build);
2. copy it to a staging folder, leaving out `test/`, `node_modules/` and `.git/` (authoring files,
   not part of the adaptation);
3. in the copy: set `Version` when given, replace `__PAGE_REGION_WIDTH__` / `__PAGE_REGION_HEIGHT__`
   in the scripts the profile names with `File` from `PageRegion`;
4. write the file manifest (`Files`) and, with a key, the signature (`qarapp.sign_folder`);
5. write the `.qarapp` (or copy the folder, `unpacked`) and read it back the way QAROS imports it.

`Library` references stay references: QAROS resolves them from its installed kit at load time, and
the package signs only its own files. The signed profile records which `name@major` it uses.
"""

from __future__ import annotations

import shutil
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

from . import kit as kitlib
from . import qarapp
from .profile import PAGE_REGION_TOKENS, ProfileError, check_adaptation, read_profile

#: Top-level folders of an adaptation folder that are authoring files, never packaged.
EXCLUDED_FOLDERS = ("test", "node_modules", ".git")

Signer = Callable[[bytes], bytes]


class BuildError(RuntimeError):
    """The adaptation does not check, stage, sign or verify."""


@dataclass(frozen=True)
class SigningOptions:
    """How to sign: `sign(digest) -> DER` and the public key it belongs to."""

    sign: Signer
    public_der: bytes
    signer: str = qarapp.DEFAULT_SIGNER


@dataclass(frozen=True)
class BuildResult:
    path: Path
    profile: dict
    verification: qarapp.Verification
    unpacked: bool


def signing_options(
    private_key: Path | None = None,
    key_vault: str | None = None,
    key_name: str = "qaros-release",
    public_key: Path | None = None,
    signer: str = qarapp.DEFAULT_SIGNER,
) -> SigningOptions | None:
    """The signer for a local PEM key (tests, local runs) or a key in Azure Key Vault (release; needs
    the public key, PEM). None when neither is given."""
    public_der = qarapp.public_key_der_from_pem(public_key.read_text(encoding="ascii")) if public_key else None
    if key_vault and private_key:
        raise BuildError("pass --sign (a PEM key) or --key-vault, not both")
    if key_vault:
        if public_der is None:
            raise BuildError("--key-vault needs --public-key (the key's public half, PEM)")
        return SigningOptions(qarapp.KeyVaultKey(key_vault, key_name).sign, public_der, signer)
    if private_key:
        key = qarapp.SigningKey.load(private_key)
        if public_der is not None and public_der != key.public_der:
            raise BuildError("the private key does not belong to --public-key")
        return SigningOptions(key.sign, key.public_der, signer)
    return None


def substitute_page_region(text: str, profile: dict) -> str:
    region = profile.get("PageRegion") or {}
    text = text.replace(PAGE_REGION_TOKENS[0], str(region.get("Width", 0)))
    return text.replace(PAGE_REGION_TOKENS[1], str(region.get("Height", 0)))


def _ignore(source: Path):
    def ignore(directory: str, names: list[str]) -> list[str]:
        if Path(directory).resolve() != source:
            return []
        return [name for name in names if name in EXCLUDED_FOLDERS]

    return ignore


def check_source(source: Path, kit: kitlib.Kit | None) -> dict:
    """The folder's profile, after `check`; a source folder holds no `Files` or `Signature` (both
    are written here)."""
    try:
        profile = read_profile(source)
    except ProfileError as error:
        raise BuildError(str(error)) from error
    for key in ("Files", "Signature"):
        if key in profile:
            raise BuildError(f"{source / 'profile.json'} holds '{key}'; remove it (package writes it)")
    report = check_adaptation(source, kit)
    if not report.ok:
        raise BuildError(f"{source} does not pass check:\n" + "\n".join(f"  {finding}" for finding in report.errors))
    return profile


def stage(source: Path, staging: Path, version: str | None = None) -> dict:
    """Copies `source` to `staging` (which must not exist) and prepares the copy (steps 2-3)."""
    source = source.resolve()
    shutil.copytree(source, staging, ignore=_ignore(source))
    profile = read_profile(staging)
    if version is not None:
        qarapp.Version.parse(version)
        profile["Version"] = version
    for entry in profile.get("Scripts") or []:
        file = entry.get("File") if isinstance(entry, dict) else None
        if not isinstance(file, str):
            continue
        path = staging / file
        text = path.read_text(encoding="utf-8")
        replaced = substitute_page_region(text, profile)
        if replaced != text:
            path.write_bytes(replaced.encode("utf-8"))
    (staging / "profile.json").write_bytes(qarapp.pretty_json(profile))
    return profile


def default_output(profile: dict, unpacked: bool) -> Path:
    stem = f"{profile['Id']}-{profile['Version']}"
    return Path.cwd() / (stem if unpacked else stem + qarapp.EXTENSION)


def _replace_folder(source: Path, destination: Path) -> None:
    if destination.exists():
        if not destination.is_dir():
            raise BuildError(f"{destination} exists and is not a folder")
        if any(destination.iterdir()) and not (destination / "profile.json").is_file():
            raise BuildError(f"{destination} is not empty and holds no profile.json; refusing to replace it")
        shutil.rmtree(destination)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, destination)


def build_package(
    source: Path,
    out: Path | None = None,
    *,
    unpacked: bool = False,
    version: str | None = None,
    signing: SigningOptions | None = None,
    trusted: list[tuple[str, bytes]] | None = None,
    kit: kitlib.Kit | None = None,
    check: bool = True,
) -> BuildResult:
    """Builds one adaptation. `out`: the `.qarapp` file, or a folder to write `<id>-<version>.qarapp`
    into; with `unpacked`, exactly the folder to write. `trusted`: (signer, public key DER) pairs
    the result must verify against (default: the signing key, when signing)."""
    source = Path(source).resolve()
    if check:
        if kit is None:
            kit = kitlib.load_kit()
        check_source(source, kit)
    scratch = Path(tempfile.mkdtemp(prefix="qaros-adaptation-"))
    try:
        staging = scratch / "adaptation"
        profile = stage(source, staging, version)
        if signing is not None:
            qarapp.sign_folder(staging, signing.signer, signing.sign, signing.public_der)
            if trusted is None:
                trusted = [(signing.signer, signing.public_der)]
        else:
            qarapp.write_manifest(staging)
        if unpacked:
            destination = Path(out).resolve() if out else default_output(profile, True)
            verification = qarapp.verify_folder(staging, trusted)
            _replace_folder(staging, destination)
            verification = qarapp.verify_folder(destination, trusted)
        else:
            if out is None:
                destination = default_output(profile, False)
            elif Path(out).suffix == qarapp.EXTENSION:
                destination = Path(out).resolve()
            else:
                destination = Path(out).resolve() / qarapp.package_file_name(profile["Id"], profile["Version"])
            qarapp.export_package(staging, destination)
            verification = qarapp.read_package(destination, trusted).verification
        if signing is not None and not verification.signed:
            raise BuildError(f"{destination.name} is not signed")
        return BuildResult(destination, qarapp.read_profile(staging), verification, unpacked)
    except qarapp.PackageError as error:
        raise BuildError(str(error)) from error
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
