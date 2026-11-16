"""QAROS web app adaptation packages (`.qarapp`): build, sign and verify, standard library only.

Part of the QAROS adaptation kit (`qaros-adaptation package` uses it). Other tools import it as
`qaros_adaptation.qarapp`; its names and behaviour are kept stable for them.

A package is a ZIP holding

    package.json             {"content": "adaptation", "formatVersion": 1, "kind": "web-app-adaptation"}
    adaptation/profile.json  the adaptation's profile with `Files` and, when signed, `Signature`
    adaptation/...           exactly the files `Files` lists

`Files` lists every file of the adaptation folder except profile.json, sorted by path, each with its
lower-case hex SHA-256. `Signature` is an ECDSA P-256 / SHA-256 signature over

    SHA-256("QAROS-ADAPTATION-SIGNATURE-V1" NUL canonical(profile.json without "Signature"))

where canonical() is JSON with sorted keys and no whitespace. The signature value is base64 DER, the
public key base64 DER SubjectPublicKeyInfo, and the key id the lower-case hex SHA-256 of that DER.

The format follows the QAROS reader (`qar-adaptation-tool`); this module writes the same ZIP layout
(deflated entries in manifest order, fixed 1980-01-01 timestamps, UTF-8 names) and checks a package
the way QAROS does on import.

Command line (mirrors qar-adaptation-tool), `python -I -m qaros_adaptation.qarapp <command>`:

    qarapp.py keygen <private.pem> <public.pem>            test key pair (never for releases)
    qarapp.py key-id <public.pem>
    qarapp.py digest <folder>                              base64 digest for `az keyvault key sign`
    qarapp.py attach <folder> <public.pem> <signature> [signer]
    qarapp.py sign <folder> <private.pem> [signer]         local PEM key (tests only)
    qarapp.py export <folder> <package.qarapp>
    qarapp.py verify <package-or-folder> [--public-key <pem>] [--signer <name>]
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import hmac
import json
import re
import secrets
import struct
import subprocess
import sys
import zlib
from dataclasses import dataclass, field
from pathlib import Path

FORMAT_VERSION = 1
PACKAGE_KIND = "web-app-adaptation"
CONTENT_FOLDER = "adaptation"
MANIFEST_NAME = "package.json"
PROFILE_NAME = "profile.json"
EXTENSION = ".qarapp"
SIGNATURE_ALGORITHM = "ECDSA-P256-SHA256"
SIGNING_DOMAIN_TAG = b"QAROS-ADAPTATION-SIGNATURE-V1"
DEFAULT_SIGNER = "Quaternar"

MAX_PATH_BYTES = 512
MAX_ID_LENGTH = 128
MAX_ENTRIES = 4096
MAX_ARCHIVE_BYTES = 256 * 1024 * 1024
MAX_TOTAL_UNCOMPRESSED_BYTES = 256 * 1024 * 1024

_PATH_CHARACTERS = re.compile(r"^[A-Za-z0-9._\-~+@()!,]+$")
_ID_CHARACTERS = re.compile(r"^[A-Za-z0-9._\-]+$")
_VERSION = re.compile(r"^([0-9]+)\.([0-9]+)\.([0-9]+)(?:-(.+))?$")


class PackageError(RuntimeError):
    """A package, folder or key that is invalid, refused or does not verify."""


# --------------------------------------------------------------------------------------------------
# Rules shared with the QAROS reader
# --------------------------------------------------------------------------------------------------


def validate_path(path: str) -> None:
    """A relative `/`-separated bundle path: 1-512 bytes, no empty, `.` or `..` segment, only
    A-Z a-z 0-9 . _ - ~ + @ ( ) ! , per segment."""
    if not path or len(path.encode("utf-8")) > MAX_PATH_BYTES:
        raise PackageError(f"path '{path}' must have 1 to {MAX_PATH_BYTES} bytes")
    for segment in path.split("/"):
        if segment in ("", ".", ".."):
            raise PackageError(f"path '{path}' is not relative or holds an empty, '.' or '..' segment")
        if not _PATH_CHARACTERS.match(segment):
            raise PackageError(f"path '{path}' holds a character other than A-Z a-z 0-9 . _ - ~ + @ ( ) ! , and '/'")


def validate_id(adaptation_id: str) -> None:
    """Reverse DNS style id: 1-128 ASCII letters, digits, `.`, `-`, `_`, starting with a letter or
    digit, not ending in `.`, not a Windows device name."""
    ok = (
        0 < len(adaptation_id) <= MAX_ID_LENGTH
        and _ID_CHARACTERS.match(adaptation_id) is not None
        and adaptation_id[0].isalnum()
        and not adaptation_id.endswith(".")
        and adaptation_id.split(".")[0].upper() not in ("CON", "PRN", "AUX", "NUL")
    )
    if not ok:
        raise PackageError(f"'{adaptation_id}' is not a valid adaptation id (e.g. com.example.viewer)")


@dataclass(frozen=True, order=False)
class Version:
    numbers: tuple[int, int, int]
    pre_release: str = ""

    @staticmethod
    def parse(text: str) -> "Version":
        match = _VERSION.match(text) if isinstance(text, str) else None
        if match is None:
            raise PackageError(f"'{text}' is not a version (major.minor.patch, e.g. 1.0.0)")
        return Version((int(match[1]), int(match[2]), int(match[3])), match[4] or "")

    def _key(self) -> tuple:
        if not self.pre_release:
            return (self.numbers, 1, ())
        identifiers = []
        for part in self.pre_release.split("."):
            # Numeric identifiers sort before alphanumeric ones and compare as numbers.
            identifiers.append((0, int(part), "") if part.isdigit() else (1, 0, part))
        return (self.numbers, 0, tuple(identifiers))

    def __lt__(self, other: "Version") -> bool:
        return self._key() < other._key()

    def __le__(self, other: "Version") -> bool:
        return self._key() <= other._key()

    def __gt__(self, other: "Version") -> bool:
        return self._key() > other._key()

    def __ge__(self, other: "Version") -> bool:
        return self._key() >= other._key()

    def __str__(self) -> str:
        core = ".".join(str(number) for number in self.numbers)
        return f"{core}-{self.pre_release}" if self.pre_release else core


# --------------------------------------------------------------------------------------------------
# JSON
# --------------------------------------------------------------------------------------------------


def canonical_json(value: object) -> bytes:
    """Sorted keys, no whitespace, UTF-8: the bytes signer and verifier hash."""
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False).encode("utf-8")


def pretty_json(value: object) -> bytes:
    """profile.json and package.json as written into a package (two-space indent, sorted keys)."""
    return json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False, allow_nan=False).encode("utf-8")


def parse_json_object(data: bytes, what: str) -> dict:
    try:
        value = json.loads(data.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PackageError(f"{what} is not valid JSON: {error}") from error
    if not isinstance(value, dict):
        raise PackageError(f"{what} is not a JSON object")
    return value


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def signing_digest(profile: dict) -> bytes:
    canonical = {key: value for key, value in profile.items() if key != "Signature"}
    return hashlib.sha256(SIGNING_DOMAIN_TAG + b"\0" + canonical_json(canonical)).digest()


# --------------------------------------------------------------------------------------------------
# ECDSA P-256
# --------------------------------------------------------------------------------------------------

_P = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
_A = _P - 3
_B = 0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B
_N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
_G = (
    0x6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296,
    0x4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5,
)
# SubjectPublicKeyInfo of an uncompressed P-256 point, everything but the 65 point bytes.
_SPKI_PREFIX = bytes.fromhex("3059301306072a8648ce3d020106082a8648ce3d030107034200")
_OID_EC_PUBLIC_KEY = bytes.fromhex("2a8648ce3d0201")
_OID_P256 = bytes.fromhex("2a8648ce3d030107")


def _point_add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    if a[0] == b[0]:
        if (a[1] + b[1]) % _P == 0:
            return None
        slope = (3 * a[0] * a[0] + _A) * pow(2 * a[1], -1, _P) % _P
    else:
        slope = (b[1] - a[1]) * pow(b[0] - a[0], -1, _P) % _P
    x = (slope * slope - a[0] - b[0]) % _P
    return (x, (slope * (a[0] - x) - a[1]) % _P)


def _point_multiply(scalar: int, point):
    result = None
    addend = point
    while scalar:
        if scalar & 1:
            result = _point_add(result, addend)
        addend = _point_add(addend, addend)
        scalar >>= 1
    return result


def _on_curve(point) -> bool:
    x, y = point
    return 0 <= x < _P and 0 <= y < _P and (y * y - (x * x * x + _A * x + _B)) % _P == 0


def _rfc6979_nonce(private: int, digest: bytes) -> int:
    """Deterministic nonce (RFC 6979, HMAC-SHA256)."""
    x = private.to_bytes(32, "big")
    h = (int.from_bytes(digest, "big") % _N).to_bytes(32, "big")
    v = b"\x01" * 32
    k = b"\x00" * 32
    k = hmac.new(k, v + b"\x00" + x + h, hashlib.sha256).digest()
    v = hmac.new(k, v, hashlib.sha256).digest()
    k = hmac.new(k, v + b"\x01" + x + h, hashlib.sha256).digest()
    v = hmac.new(k, v, hashlib.sha256).digest()
    while True:
        v = hmac.new(k, v, hashlib.sha256).digest()
        candidate = int.from_bytes(v, "big")
        if 1 <= candidate < _N:
            return candidate
        k = hmac.new(k, v + b"\x00", hashlib.sha256).digest()
        v = hmac.new(k, v, hashlib.sha256).digest()


def ecdsa_sign(private: int, digest: bytes) -> tuple[int, int]:
    e = int.from_bytes(digest, "big")
    while True:
        k = _rfc6979_nonce(private, digest)
        r = _point_multiply(k, _G)[0] % _N
        s = pow(k, -1, _N) * (e + r * private) % _N
        if r and s:
            return r, s


def ecdsa_verify(public_point, digest: bytes, r: int, s: int) -> bool:
    if not (1 <= r < _N and 1 <= s < _N):
        return False
    e = int.from_bytes(digest, "big")
    w = pow(s, -1, _N)
    point = _point_add(_point_multiply(e * w % _N, _G), _point_multiply(r * w % _N, public_point))
    return point is not None and point[0] % _N == r


# --------------------------------------------------------------------------------------------------
# DER / PEM
# --------------------------------------------------------------------------------------------------


def _der_read(data: bytes, offset: int) -> tuple[int, bytes, int]:
    """(tag, content, next offset) of the DER element at `offset`."""
    if offset + 2 > len(data):
        raise PackageError("truncated DER")
    tag = data[offset]
    length = data[offset + 1]
    offset += 2
    if length & 0x80:
        count = length & 0x7F
        if count == 0 or count > 4 or offset + count > len(data):
            raise PackageError("unsupported DER length")
        length = int.from_bytes(data[offset : offset + count], "big")
        offset += count
    if offset + length > len(data):
        raise PackageError("truncated DER")
    return tag, data[offset : offset + length], offset + length


def _der_children(content: bytes) -> list[tuple[int, bytes]]:
    children = []
    offset = 0
    while offset < len(content):
        tag, value, offset = _der_read(content, offset)
        children.append((tag, value))
    return children


def _der_integer(value: int) -> bytes:
    body = value.to_bytes(max(1, (value.bit_length() + 7) // 8), "big")
    if body[0] & 0x80:
        body = b"\x00" + body
    return b"\x02" + bytes([len(body)]) + body


def signature_to_der(r: int, s: int) -> bytes:
    body = _der_integer(r) + _der_integer(s)
    return b"\x30" + bytes([len(body)]) + body


def signature_from_der(der: bytes) -> tuple[int, int]:
    tag, content, end = _der_read(der, 0)
    children = _der_children(content) if tag == 0x30 else []
    if end != len(der) or len(children) != 2 or any(child[0] != 0x02 for child in children):
        raise PackageError("the signature is not a DER ECDSA signature")
    return int.from_bytes(children[0][1], "big"), int.from_bytes(children[1][1], "big")


def raw_signature_to_der(signature: bytes) -> bytes:
    """A key vault ES256 signature is raw r||s (64 bytes); a package stores DER. DER passes through."""
    if len(signature) == 64:
        return signature_to_der(int.from_bytes(signature[:32], "big"), int.from_bytes(signature[32:], "big"))
    signature_from_der(signature)
    return signature


def pem_body(text: str, label: str) -> bytes:
    match = re.search(rf"-----BEGIN {label}-----(.*?)-----END {label}-----", text, re.S)
    if match is None:
        raise PackageError(f"no '{label}' PEM block")
    return base64.b64decode("".join(match[1].split()))


def public_key_der_from_pem(text: str) -> bytes:
    der = pem_body(text, "PUBLIC KEY")
    public_point_from_der(der)
    return der


def public_key_pem(der: bytes) -> str:
    encoded = base64.b64encode(der).decode("ascii")
    lines = [encoded[index : index + 64] for index in range(0, len(encoded), 64)]
    return "-----BEGIN PUBLIC KEY-----\n" + "\n".join(lines) + "\n-----END PUBLIC KEY-----\n"


def public_point_from_der(der: bytes):
    """The P-256 point of a DER SubjectPublicKeyInfo; refuses any other key type."""
    tag, content, end = _der_read(der, 0)
    children = _der_children(content) if tag == 0x30 else []
    if end != len(der) or len(children) != 2 or children[0][0] != 0x30 or children[1][0] != 0x03:
        raise PackageError("not a DER SubjectPublicKeyInfo")
    algorithm = _der_children(children[0][1])
    if len(algorithm) != 2 or algorithm[0] != (0x06, _OID_EC_PUBLIC_KEY) or algorithm[1] != (0x06, _OID_P256):
        raise PackageError("the public key is not an ECDSA P-256 key")
    bits = children[1][1]
    if len(bits) != 66 or bits[0] != 0 or bits[1] != 4:
        raise PackageError("the public key is not an uncompressed P-256 point")
    point = (int.from_bytes(bits[2:34], "big"), int.from_bytes(bits[34:], "big"))
    if not _on_curve(point):
        raise PackageError("the public key is not on P-256")
    return point


def public_key_der(point) -> bytes:
    return _SPKI_PREFIX + b"\x04" + point[0].to_bytes(32, "big") + point[1].to_bytes(32, "big")


def key_id(public_der: bytes) -> str:
    return sha256_hex(public_der)


def private_key_from_pem(text: str) -> int:
    """The scalar of a P-256 private key PEM: SEC 1 `EC PRIVATE KEY` or PKCS #8 `PRIVATE KEY`."""
    if "BEGIN EC PRIVATE KEY" in text:
        sec1 = pem_body(text, "EC PRIVATE KEY")
    else:
        pkcs8 = pem_body(text, "PRIVATE KEY")
        _, content, _ = _der_read(pkcs8, 0)
        children = _der_children(content)
        if len(children) < 3 or children[1][0] != 0x30 or children[2][0] != 0x04:
            raise PackageError("not a PKCS #8 private key")
        algorithm = _der_children(children[1][1])
        if len(algorithm) != 2 or algorithm[0] != (0x06, _OID_EC_PUBLIC_KEY) or algorithm[1] != (0x06, _OID_P256):
            raise PackageError("the private key is not an ECDSA P-256 key")
        sec1 = children[2][1]
    _, content, _ = _der_read(sec1, 0)
    children = _der_children(content)
    if len(children) < 2 or children[1][0] != 0x04:
        raise PackageError("not a SEC 1 EC private key")
    scalar = int.from_bytes(children[1][1], "big")
    if not 1 <= scalar < _N:
        raise PackageError("the private key is out of range")
    return scalar


def private_key_pem(scalar: int) -> str:
    """SEC 1 `EC PRIVATE KEY` PEM with the curve and public key."""
    point = _point_multiply(scalar, _G)
    public_bits = b"\x00\x04" + point[0].to_bytes(32, "big") + point[1].to_bytes(32, "big")
    body = (
        b"\x02\x01\x01"
        + b"\x04\x20" + scalar.to_bytes(32, "big")
        + b"\xa0\x0a\x06\x08" + _OID_P256
        + b"\xa1\x44\x03\x42" + public_bits
    )
    der = b"\x30" + bytes([len(body)]) + body
    encoded = base64.b64encode(der).decode("ascii")
    lines = [encoded[index : index + 64] for index in range(0, len(encoded), 64)]
    return "-----BEGIN EC PRIVATE KEY-----\n" + "\n".join(lines) + "\n-----END EC PRIVATE KEY-----\n"


@dataclass(frozen=True)
class SigningKey:
    """A local PEM key, for tests and local runs only: release keys stay in the key vault."""

    scalar: int

    @staticmethod
    def load(path: Path) -> "SigningKey":
        return SigningKey(private_key_from_pem(path.read_text(encoding="ascii")))

    @property
    def public_der(self) -> bytes:
        return public_key_der(_point_multiply(self.scalar, _G))

    def sign(self, digest: bytes) -> bytes:
        return signature_to_der(*ecdsa_sign(self.scalar, digest))


def keygen() -> SigningKey:
    return SigningKey(secrets.randbelow(_N - 1) + 1)


def verify_signature_value(public_der: bytes, digest: bytes, value_der: bytes) -> bool:
    try:
        r, s = signature_from_der(value_der)
    except PackageError:
        return False
    return ecdsa_verify(public_point_from_der(public_der), digest, r, s)


@dataclass(frozen=True)
class KeyVaultKey:
    vault: str
    name: str

    def sign(self, digest: bytes) -> bytes:
        """ES256 signature of `digest` by the key in Azure Key Vault (`az keyvault key sign`), as DER."""
        command = [
            "az", "keyvault", "key", "sign",
            "--vault-name", self.vault, "--name", self.name,
            "--algorithm", "ES256", "--digest", base64.b64encode(digest).decode("ascii"),
            "--output", "json",
        ]
        completed = subprocess.run(command, capture_output=True, text=True, check=False, shell=sys.platform == "win32")
        if completed.returncode != 0:
            raise PackageError(f"az keyvault key sign failed: {completed.stderr.strip()}")
        answer = json.loads(completed.stdout)
        signature = answer.get("signature") or answer.get("result")
        if not signature:
            raise PackageError(f"az keyvault key sign answered no signature: {completed.stdout.strip()}")
        return raw_signature_to_der(decode_base64(signature))


def decode_base64(value: str) -> bytes:
    """Standard or URL-safe base64, padding optional (Key Vault answers base64url without padding)."""
    value = value.strip().replace("-", "+").replace("_", "/")
    return base64.b64decode(value + "=" * (-len(value) % 4), validate=True)


def signature_block(signer: str, public_der: bytes, value_der: bytes) -> dict:
    return {
        "Algorithm": SIGNATURE_ALGORITHM,
        "KeyId": key_id(public_der),
        "PublicKey": base64.b64encode(public_der).decode("ascii"),
        "Signer": signer,
        "Value": base64.b64encode(value_der).decode("ascii"),
    }


# --------------------------------------------------------------------------------------------------
# Adaptation folders
# --------------------------------------------------------------------------------------------------


def file_manifest(folder: Path) -> list[dict]:
    """Every regular file of `folder` except profile.json, sorted by path, with its SHA-256."""
    manifest = []
    for path in folder.rglob("*"):
        relative = path.relative_to(folder).as_posix()
        if path.is_symlink() or (not path.is_file() and not path.is_dir()):
            raise PackageError(f"'{relative}' is a link or special file; an adaptation holds plain files only")
        if path.is_dir() or relative == PROFILE_NAME:
            continue
        validate_path(relative)
        manifest.append({"Path": relative, "Sha256": sha256_hex(path.read_bytes())})
    manifest.sort(key=lambda entry: entry["Path"].encode("utf-8"))
    return manifest


def read_profile(folder: Path) -> dict:
    path = folder / PROFILE_NAME
    if not path.is_file():
        raise PackageError(f"{path} does not exist")
    return parse_json_object(path.read_bytes(), str(path))


def check_profile(profile: dict, files: set[str] | None = None) -> None:
    """Id, version, panels folder and script files: what QAROS refuses on import."""
    validate_id(profile.get("Id", ""))
    Version.parse(profile.get("Version", ""))
    validate_path(profile.get("Panels", "panels"))
    for script in profile.get("Scripts", []):
        script_file = script.get("File")
        if script_file and files is not None and script_file not in files:
            raise PackageError(f"script '{script.get('Name')}' names file '{script_file}', which is not in the adaptation")


def _compare_manifest(listed: list, actual: list[dict], label: str) -> None:
    expected: dict[str, str] = {}
    for entry in listed:
        if not isinstance(entry, dict) or not isinstance(entry.get("Path"), str) or not isinstance(entry.get("Sha256"), str):
            raise PackageError(f"'{label}': the file manifest does not parse")
        if entry["Path"] in expected:
            raise PackageError(f"'{label}': the file manifest lists '{entry['Path']}' twice")
        expected[entry["Path"]] = entry["Sha256"]
    for entry in actual:
        if entry["Path"] not in expected:
            raise PackageError(f"'{label}': file '{entry['Path']}' is not in the file manifest")
        if expected.pop(entry["Path"]) != entry["Sha256"]:
            raise PackageError(f"'{label}': file '{entry['Path']}' was changed (its SHA-256 does not match the manifest)")
    if expected:
        raise PackageError(f"'{label}': file '{min(expected)}' is missing")


@dataclass(frozen=True)
class Verification:
    adaptation_id: str
    version: str
    signer: str = ""
    key_id: str = ""

    @property
    def signed(self) -> bool:
        return bool(self.signer)


def verify_profile_and_files(profile: dict, actual: list[dict], trusted: list[tuple[str, bytes]] | None) -> Verification:
    """Files exactly `Files`; a signature must verify and, when `trusted` is given, be by one of
    those (name, public key DER). With `trusted` None the signature is checked against its own key."""
    label = profile.get("Id") or profile.get("Name") or "adaptation"
    listed = profile.get("Files") or []
    if not isinstance(listed, list):
        raise PackageError(f"'{label}': the file manifest does not parse")
    _compare_manifest(listed, actual, label)
    check_profile(profile, {entry["Path"] for entry in actual})
    block = profile.get("Signature")
    if block is None:
        return Verification(profile["Id"], profile["Version"])
    if not isinstance(block, dict):
        raise PackageError(f"Adaptation '{label}': the signature block is not an object")
    if block.get("Algorithm") != SIGNATURE_ALGORITHM:
        raise PackageError(f"Adaptation '{label}': unsupported signature algorithm '{block.get('Algorithm')}'")
    try:
        public_der = base64.b64decode(block.get("PublicKey", ""), validate=True)
        value = base64.b64decode(block.get("Value", ""), validate=True)
    except ValueError as error:
        raise PackageError(f"Adaptation '{label}': the signature block is not valid base64") from error
    if key_id(public_der) != block.get("KeyId"):
        raise PackageError(f"Adaptation '{label}': the signature's key id does not match its public key")
    signer = block.get("Signer", "")
    if trusted is not None:
        names = [name for name, der in trusted if der == public_der]
        if not names:
            raise PackageError(f"Adaptation '{label}': signed by '{signer}' with key {block.get('KeyId')}, which is not trusted")
        if names[0] != signer:
            raise PackageError(f"Adaptation '{label}': the signature names '{signer}' but its key belongs to '{names[0]}'")
    if not verify_signature_value(public_der, signing_digest(profile), value):
        raise PackageError(f"Adaptation '{label}': its signature does not verify: profile.json or its file manifest was changed")
    return Verification(profile["Id"], profile["Version"], signer, block["KeyId"])


def verify_folder(folder: Path, trusted: list[tuple[str, bytes]] | None) -> Verification:
    return verify_profile_and_files(read_profile(folder), file_manifest(folder), trusted)


def prepare_signing(folder: Path) -> tuple[dict, bytes]:
    """The profile with `Files` recomputed and no `Signature`, and the digest a signer signs."""
    profile = read_profile(folder)
    files = file_manifest(folder)
    check_profile(profile, {entry["Path"] for entry in files})
    profile.pop("Signature", None)
    if files:
        profile["Files"] = files
    else:
        profile.pop("Files", None)
    return profile, signing_digest(profile)


def write_manifest(folder: Path) -> dict:
    """Writes `Files` (unsigned) into the folder's profile.json."""
    profile, _ = prepare_signing(folder)
    (folder / PROFILE_NAME).write_bytes(pretty_json(profile))
    return profile


def attach_signature(folder: Path, signer: str, public_der: bytes, value_der: bytes) -> Verification:
    """Writes `Files` and a `Signature` made over `digest(folder)` elsewhere, then verifies."""
    profile, digest = prepare_signing(folder)
    if not verify_signature_value(public_der, digest, value_der):
        raise PackageError(f"{folder}: the signature does not match the folder's digest (did a file change?)")
    profile["Signature"] = signature_block(signer, public_der, value_der)
    (folder / PROFILE_NAME).write_bytes(pretty_json(profile))
    return verify_folder(folder, [(signer, public_der)])


def sign_folder(folder: Path, signer: str, sign, public_der: bytes) -> Verification:
    """`sign(digest) -> DER signature` from a local key or a key vault."""
    _, digest = prepare_signing(folder)
    return attach_signature(folder, signer, public_der, sign(digest))


# --------------------------------------------------------------------------------------------------
# ZIP
# --------------------------------------------------------------------------------------------------

_DOS_DATE = (0 << 9) | (1 << 5) | 1  # 1980-01-01
_DOS_TIME = 0
_FLAG_UTF8 = 0x0800
_FLAG_ENCRYPTED = 0x0001
_VERSION_NEEDED = 20


def _deflate(data: bytes) -> bytes:
    compressor = zlib.compressobj(9, zlib.DEFLATED, -zlib.MAX_WBITS, 8, zlib.Z_DEFAULT_STRATEGY)
    return compressor.compress(data) + compressor.flush()


def write_zip(entries: list[tuple[str, bytes]]) -> bytes:
    """Deflated entries in the given order with fixed timestamps: equal entries, equal bytes."""
    archive = bytearray()
    directory = bytearray()
    for name, data in entries:
        encoded = name.encode("utf-8")
        compressed = _deflate(data)
        crc = zlib.crc32(data) & 0xFFFFFFFF
        offset = len(archive)
        if max(len(data), len(compressed), offset) >= 0xFFFFFFFF:
            raise PackageError(f"entry '{name}' is too large for an adaptation package")
        common = struct.pack("<HHHHIII", _FLAG_UTF8, 8, _DOS_TIME, _DOS_DATE, crc, len(compressed), len(data))
        archive += struct.pack("<IH", 0x04034B50, _VERSION_NEEDED) + common + struct.pack("<HH", len(encoded), 0)
        archive += encoded + compressed
        directory += struct.pack("<IHH", 0x02014B50, _VERSION_NEEDED, _VERSION_NEEDED) + common
        directory += struct.pack("<HHHHHII", len(encoded), 0, 0, 0, 0, 0, offset) + encoded
    if len(entries) > 0xFFFF:
        raise PackageError("the adaptation has too many files for an adaptation package")
    directory_offset = len(archive)
    archive += directory
    archive += struct.pack("<IHHHHIIH", 0x06054B50, 0, 0, len(entries), len(entries), len(directory), directory_offset, 0)
    return bytes(archive)


def read_zip(archive: bytes) -> list[tuple[str, bytes]]:
    """Every file entry (directories skipped), CRC checked, under the QAROS reader's caps."""
    if len(archive) > MAX_ARCHIVE_BYTES:
        raise PackageError(f"the package is larger than {MAX_ARCHIVE_BYTES} bytes")
    if len(archive) < 22:
        raise PackageError("not a valid ZIP: the file is too short")
    eocd = archive.rfind(b"PK\x05\x06", max(0, len(archive) - 22 - 0xFFFF))
    if eocd < 0:
        raise PackageError("not a valid ZIP: no end of central directory")
    count, size, offset = struct.unpack_from("<HII", archive, eocd + 10)
    if size == 0xFFFFFFFF or offset == 0xFFFFFFFF:
        raise PackageError("ZIP64 archives are not supported")
    if count > MAX_ENTRIES:
        raise PackageError(f"the package has more than {MAX_ENTRIES} entries")
    if offset + size > eocd:
        raise PackageError("not a valid ZIP: the central directory is out of range")
    entries = []
    total = 0
    position = offset
    for _ in range(count):
        if position + 46 > eocd or archive[position : position + 4] != b"PK\x01\x02":
            raise PackageError("not a valid ZIP: a central directory entry is damaged")
        flags, method = struct.unpack_from("<HH", archive, position + 8)
        crc, compressed_size, size_ = struct.unpack_from("<III", archive, position + 16)
        name_length, extra_length, comment_length = struct.unpack_from("<HHH", archive, position + 28)
        (local,) = struct.unpack_from("<I", archive, position + 42)
        name = archive[position + 46 : position + 46 + name_length].decode("utf-8", errors="strict")
        position += 46 + name_length + extra_length + comment_length
        if position > eocd:
            raise PackageError("not a valid ZIP: a central directory entry is out of range")
        if name.endswith("/"):
            continue
        if flags & _FLAG_ENCRYPTED:
            raise PackageError(f"entry '{name}' is encrypted")
        if method not in (0, 8):
            raise PackageError(f"entry '{name}' uses compression method {method}")
        total += size_
        if total > MAX_TOTAL_UNCOMPRESSED_BYTES:
            raise PackageError(f"the package's files exceed {MAX_TOTAL_UNCOMPRESSED_BYTES} bytes")
        if local + 30 > len(archive) or archive[local : local + 4] != b"PK\x03\x04":
            raise PackageError(f"the local header of '{name}' is damaged")
        local_name, local_extra = struct.unpack_from("<HH", archive, local + 26)
        start = local + 30 + local_name + local_extra
        if start + compressed_size > len(archive):
            raise PackageError(f"the data of '{name}' is out of range")
        raw = archive[start : start + compressed_size]
        if method == 0:
            if compressed_size != size_:
                raise PackageError(f"stored entry '{name}' has two sizes")
            data = raw
        else:
            decompressor = zlib.decompressobj(-zlib.MAX_WBITS)
            data = decompressor.decompress(raw, size_ + 1)
            if not decompressor.eof or len(data) != size_:
                raise PackageError(f"entry '{name}' does not inflate to its declared size")
        if zlib.crc32(data) & 0xFFFFFFFF != crc:
            raise PackageError(f"entry '{name}' fails its CRC check")
        entries.append((name, bytes(data)))
    return entries


# --------------------------------------------------------------------------------------------------
# Packages
# --------------------------------------------------------------------------------------------------


def package_manifest_bytes() -> bytes:
    return pretty_json({"formatVersion": FORMAT_VERSION, "kind": PACKAGE_KIND, "content": CONTENT_FOLDER})


def package_file_name(adaptation_id: str, version: str) -> str:
    return f"{adaptation_id}-{version}{EXTENSION}"


def export_package(folder: Path, package: Path) -> Verification:
    """Writes `folder` (whose profile.json already holds `Files`, see write_manifest or
    sign_folder) as a package. Refuses a folder whose files do not match its manifest."""
    verification = verify_folder(folder, None)
    profile_bytes = (folder / PROFILE_NAME).read_bytes()
    profile = parse_json_object(profile_bytes, PROFILE_NAME)
    entries = [(MANIFEST_NAME, package_manifest_bytes()), (f"{CONTENT_FOLDER}/{PROFILE_NAME}", profile_bytes)]
    for entry in profile.get("Files") or []:
        entries.append((f"{CONTENT_FOLDER}/{entry['Path']}", (folder / entry["Path"]).read_bytes()))
    package.parent.mkdir(parents=True, exist_ok=True)
    temporary = package.with_name(package.name + ".tmp")
    temporary.write_bytes(write_zip(entries))
    temporary.replace(package)
    return verification


@dataclass
class PackageContents:
    format_version: int
    kind: str
    profile: dict
    verification: Verification
    files: dict[str, bytes] = field(default_factory=dict)


def read_package(package: Path, trusted: list[tuple[str, bytes]] | None) -> PackageContents:
    """Reads and verifies a package the way QAROS imports one."""
    entries = read_zip(package.read_bytes())
    manifests = [data for name, data in entries if name == MANIFEST_NAME]
    if not manifests:
        raise PackageError("the file is not an adaptation package: it has no package.json")
    manifest = parse_json_object(manifests[0], MANIFEST_NAME)
    format_version = manifest.get("formatVersion")
    if not isinstance(format_version, int) or isinstance(format_version, bool) or format_version < 1:
        raise PackageError("package.json has no formatVersion (a positive integer)")
    if format_version > FORMAT_VERSION:
        raise PackageError(f"the package format version {format_version} is newer than this tool reads ({FORMAT_VERSION})")
    kind = manifest.get("kind", "")
    if kind != PACKAGE_KIND:
        raise PackageError(f"the package kind '{kind}' is not '{PACKAGE_KIND}'")
    content = manifest.get("content", CONTENT_FOLDER)
    validate_path(content)
    prefix = content + "/"
    files: dict[str, bytes] = {}
    for name, data in entries:
        if name == MANIFEST_NAME:
            continue
        if not name.startswith(prefix):
            raise PackageError(f"the package holds '{name}', which is outside its content folder '{content}'")
        path = name[len(prefix) :]
        validate_path(path)
        if path in files:
            raise PackageError(f"the package holds '{name}' twice")
        files[path] = data
    if PROFILE_NAME not in files:
        raise PackageError(f"the package has no {content}/{PROFILE_NAME}")
    profile = parse_json_object(files[PROFILE_NAME], PROFILE_NAME)
    actual = sorted(
        ({"Path": path, "Sha256": sha256_hex(data)} for path, data in files.items() if path != PROFILE_NAME),
        key=lambda entry: entry["Path"].encode("utf-8"),
    )
    verification = verify_profile_and_files(profile, actual, trusted)
    return PackageContents(format_version, kind, profile, verification, files)


def trusted_signers(public_key: Path | None, signer: str) -> list[tuple[str, bytes]] | None:
    if public_key is None:
        return None
    return [(signer, public_key_der_from_pem(public_key.read_text(encoding="ascii")))]


# --------------------------------------------------------------------------------------------------
# Command line
# --------------------------------------------------------------------------------------------------


def _main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    command = commands.add_parser("keygen", help="test key pair (never for releases)")
    command.add_argument("private_pem", type=Path)
    command.add_argument("public_pem", type=Path)
    command = commands.add_parser("key-id")
    command.add_argument("public_pem", type=Path)
    command = commands.add_parser("digest")
    command.add_argument("folder", type=Path)
    command = commands.add_parser("attach")
    command.add_argument("folder", type=Path)
    command.add_argument("public_pem", type=Path)
    command.add_argument("signature", help="base64 (or base64url) signature, raw r||s or DER")
    command.add_argument("signer", nargs="?", default=DEFAULT_SIGNER)
    command = commands.add_parser("sign", help="sign with a local PEM key (tests only)")
    command.add_argument("folder", type=Path)
    command.add_argument("private_pem", type=Path)
    command.add_argument("signer", nargs="?", default=DEFAULT_SIGNER)
    command = commands.add_parser("export")
    command.add_argument("folder", type=Path)
    command.add_argument("package", type=Path)
    command = commands.add_parser("verify")
    command.add_argument("target", type=Path)
    command.add_argument("--public-key", type=Path, help="trusted public key (PEM); without it a signature is checked against its own key")
    command.add_argument("--signer", default=DEFAULT_SIGNER)
    args = parser.parse_args(argv)

    if args.command == "keygen":
        key = keygen()
        args.private_pem.write_text(private_key_pem(key.scalar), encoding="ascii")
        args.public_pem.write_text(public_key_pem(key.public_der), encoding="ascii")
        print(f"key id {key_id(key.public_der)}")
    elif args.command == "key-id":
        print(key_id(public_key_der_from_pem(args.public_pem.read_text(encoding="ascii"))))
    elif args.command == "digest":
        print(base64.b64encode(prepare_signing(args.folder)[1]).decode("ascii"))
    elif args.command == "attach":
        public_der = public_key_der_from_pem(args.public_pem.read_text(encoding="ascii"))
        attach_signature(args.folder, args.signer, public_der, raw_signature_to_der(decode_base64(args.signature)))
        print(f"attached and verified {args.folder}")
    elif args.command == "sign":
        key = SigningKey.load(args.private_pem)
        sign_folder(args.folder, args.signer, key.sign, key.public_der)
        print(f"signed {args.folder} as {args.signer}")
    elif args.command == "export":
        profile = read_profile(args.folder)
        if not profile.get("Files"):
            write_manifest(args.folder)
        result = export_package(args.folder, args.package)
        print(f"exported {result.adaptation_id} {result.version}{' (signed)' if result.signed else ''}")
    elif args.command == "verify":
        trusted = trusted_signers(args.public_key, args.signer)
        if args.target.is_dir():
            result = verify_folder(args.target, trusted)
        else:
            result = read_package(args.target, trusted).verification
        if not result.signed:
            state = "unsigned"
        elif trusted is None:
            state = f"signature valid, claims signer '{result.signer}' (no trusted key given)"
        else:
            state = f"signed by {result.signer}"
        print(f"ok {result.adaptation_id} {result.version}, {state}")
    return 0


def main() -> int:
    try:
        return _main(sys.argv[1:])
    except (PackageError, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
