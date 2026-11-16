#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Release a QAROS app repository on every merge to main that changes behaviour.

Used by .github/workflows/app-release.yml; the rules are in docs/adr/0002-app-release-on-merge.md.

    app_release.py check   --title T --labels a,b [--base-ref origin/main]
    app_release.py release [--before SHA] [--dry-run]
    app_release.py apply   --version X.Y.Z [--date YYYY-MM-DD]

`check` prints what merging a pull request releases (and fails when its title and labels define
no bump). `release` decides the bump from the pull requests merged since the last release, writes
the version into every file that carries it, moves the changelog's Unreleased section under the
new version, commits `chore(release): vX.Y.Z`, tags it, pushes both, and starts the repository's
release build. `apply` only rewrites the files (for a local dry run).

The repository describes its files in .github/app-release.json:

    {
      "version_files": [
        {"path": "VERSION", "pattern": "^(?P<version>\\d+\\.\\d+\\.\\d+)$"},
        {"path": "blender/addon/__init__.py",
         "pattern": "\"version\": \\((?P<version>\\d+, \\d+, \\d+)\\)", "format": "tuple"},
        {"path": "Cargo.lock", "pattern": "...", "count": 5}
      ],
      "changelog": "CHANGELOG.md",
      "release": {"workflow": "release.yml", "inputs": {"tag": "{tag}"}}
    }

The first entry is the version file (VERSION). Every pattern has a named group `version`; each
entry must match `count` times (default 1), and every match must hold the current version.
`release` either names a workflow to dispatch on the new tag (inputs may use {tag} and
{version}), or is {"github_release": true} to publish a GitHub release with the changelog
section as its notes.

Standard library only; runs on Python 3.9 and later.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, Iterable, Optional

# ---------------------------------------------------------------------------------------------
# Bump levels

NONE, PATCH, MINOR, BREAKING, MAJOR = "none", "patch", "minor", "breaking", "major"
# BREAKING is a breaking change named by a title or a changelog section: major from 1.0.0 on,
# minor below. MAJOR is only ever set by the release:major label.
_ORDER = {NONE: 0, PATCH: 1, MINOR: 2, BREAKING: 3, MAJOR: 4}

LABEL_PREFIX = "release:"
LABEL_LEVELS = {"major": MAJOR, "minor": MINOR, "patch": PATCH, "none": NONE}

TYPE_LEVELS = {
    "feat": MINOR,
    "fix": PATCH,
    "perf": PATCH,
    "revert": PATCH,
    "docs": NONE,
    "ci": NONE,
    "chore": NONE,
    "test": NONE,
    "tests": NONE,
    "refactor": NONE,
    "style": NONE,
    "build": NONE,
}

TITLE_RE = re.compile(r"^(?P<type>[A-Za-z]+)(?:\((?P<scope>[^()]*)\))?(?P<bang>!)?: (?P<desc>\S.*)$")
BREAKING_RE = re.compile(r"\bBREAKING[ -]CHANGES?\b|^BREAKING\b|\bBREAKING:", re.MULTILINE)
RELEASE_SUBJECT_RE = re.compile(r"^chore\(release\): v\d+\.\d+\.\d+$")

# Changelog categories and the least bump a release that ships them needs.
CATEGORY_LEVELS = {
    "breaking": BREAKING,
    "removed": BREAKING,
    "features": MINOR,
    "added": MINOR,
    "enhancements": MINOR,
    "changed": MINOR,
    "deprecated": MINOR,
    "bug fixes": PATCH,
    "fixed": PATCH,
    "fixes": PATCH,
    "security": PATCH,
}


class ReleaseError(Exception):
    """A problem a person must fix; the message says how."""


def max_level(levels: Iterable[str]) -> str:
    result = NONE
    for level in levels:
        if _ORDER[level] > _ORDER[result]:
            result = level
    return result


def classify(title: str, labels: Iterable[str] = ()) -> tuple[str, str]:
    """The bump a pull request asks for, and why. A release:* label wins over the title."""
    release_labels = sorted({l.strip() for l in labels if l.strip().lower().startswith(LABEL_PREFIX)})
    if len(release_labels) > 1:
        raise ReleaseError(f"more than one release label ({', '.join(release_labels)}): keep one")
    if release_labels:
        value = release_labels[0][len(LABEL_PREFIX):].strip().lower()
        if value not in LABEL_LEVELS:
            raise ReleaseError(
                f"unknown label {release_labels[0]!r}: use release:major, release:minor, "
                "release:patch or release:none")
        return LABEL_LEVELS[value], f"label {release_labels[0]}"

    title = title.strip()
    match = TITLE_RE.match(title)
    if not match:
        raise ReleaseError(
            f"title {title!r} is not a Conventional Commit ('type(scope): what changed', e.g. "
            "'fix: ...' or 'feat(panels): ...'); fix the title or add a release:* label")
    kind = match.group("type").lower()
    if kind not in TYPE_LEVELS:
        raise ReleaseError(
            f"type {kind!r} in title {title!r} is unknown: use one of "
            f"{', '.join(sorted(TYPE_LEVELS))}, or add a release:* label")
    if match.group("bang") or BREAKING_RE.search(match.group("desc")):
        return BREAKING, f"title type {kind}! (breaking)"
    return TYPE_LEVELS[kind], f"title type {kind}"


# ---------------------------------------------------------------------------------------------
# Versions

VERSION_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
TAG_RE = re.compile(r"^v(\d+)\.(\d+)\.(\d+)(?:-(.+))?$")


def parse_version(text: str) -> tuple[int, int, int]:
    match = VERSION_RE.match(text.strip())
    if not match:
        raise ReleaseError(f"{text!r} is not a version X.Y.Z")
    return int(match.group(1)), int(match.group(2)), int(match.group(3))


def format_version(parts: tuple[int, int, int]) -> str:
    return ".".join(str(p) for p in parts)


def resolve_level(level: str, base: tuple[int, int, int]) -> str:
    """A breaking change is a major bump from 1.0.0 on and a minor bump below it."""
    if level == BREAKING:
        return MAJOR if base[0] >= 1 else MINOR
    return level


def bump(base: str, level: str) -> str:
    x, y, z = parse_version(base)
    level = resolve_level(level, (x, y, z))
    if level == MAJOR:
        return f"{x + 1}.0.0"
    if level == MINOR:
        return f"{x}.{y + 1}.0"
    if level == PATCH:
        return f"{x}.{y}.{z + 1}"
    raise ReleaseError(f"no version bump for level {level!r}")


# ---------------------------------------------------------------------------------------------
# Version files


@dataclass
class VersionFile:
    path: str
    pattern: str
    format: str = "dotted"
    count: int = 1

    def render(self, version: str) -> str:
        if self.format == "dotted":
            return version
        if self.format == "tuple":
            return ", ".join(version.split("."))
        raise ReleaseError(f"{self.path}: unknown format {self.format!r} (dotted or tuple)")

    def parse(self, text: str) -> str:
        if self.format == "tuple":
            return ".".join(p.strip() for p in text.split(","))
        return text.strip()


@dataclass
class Config:
    version_files: list[VersionFile]
    changelog: Optional[str] = "CHANGELOG.md"
    release: dict = field(default_factory=dict)

    @staticmethod
    def load(path: Path) -> "Config":
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except FileNotFoundError:
            raise ReleaseError(f"{path} is missing: it lists the files that carry the version")
        except json.JSONDecodeError as error:
            raise ReleaseError(f"{path} is not valid JSON: {error}")
        files = [VersionFile(**entry) for entry in data.get("version_files", [])]
        if not files:
            raise ReleaseError(f"{path}: version_files is empty")
        return Config(files, data.get("changelog", "CHANGELOG.md"), data.get("release", {}))


def rewrite_version(text: str, entry: VersionFile, old: str, new: str) -> str:
    """Replaces the version in every match of the entry's pattern; each must hold `old`."""
    regex = re.compile(entry.pattern, re.MULTILINE)
    matches = list(regex.finditer(text))
    if len(matches) != entry.count:
        raise ReleaseError(
            f"{entry.path}: the pattern matches {len(matches)} times, expected {entry.count}; "
            "fix .github/app-release.json")
    pieces, last = [], 0
    for match in matches:
        found = entry.parse(match.group("version"))
        if found != old:
            raise ReleaseError(f"{entry.path} has version {found}, VERSION has {old}: make them equal")
        start, end = match.span("version")
        pieces += [text[last:start], entry.render(new)]
        last = end
    pieces.append(text[last:])
    return "".join(pieces)


def read_current_version(root: Path, config: Config) -> str:
    entry = config.version_files[0]
    text = (root / entry.path).read_text(encoding="utf-8")
    match = re.search(entry.pattern, text, re.MULTILINE)
    if not match:
        raise ReleaseError(f"{entry.path} holds no version X.Y.Z")
    version = entry.parse(match.group("version"))
    parse_version(version)
    return version


# ---------------------------------------------------------------------------------------------
# Changelog

UNRELEASED_RE = re.compile(r"^## \[?Unreleased\]?[ \t]*$", re.MULTILINE | re.IGNORECASE)
SECTION_RE = re.compile(r"^## ", re.MULTILINE)
CATEGORY_RE = re.compile(r"^### +(.+?)\s*$", re.MULTILINE)
LINK_UNRELEASED_RE = re.compile(r"^\[Unreleased\]: *(\S+)[ \t]*$", re.MULTILINE | re.IGNORECASE)


def unreleased_section(text: str) -> tuple[int, int, str]:
    """(start of the body, end of the body, body) of the Unreleased section."""
    match = UNRELEASED_RE.search(text)
    if not match:
        raise ReleaseError("the changelog has no '## [Unreleased]' section")
    start = match.end()
    following = SECTION_RE.search(text, start)
    end = following.start() if following else len(text)
    # A link reference block at the end of the file is not part of the section.
    link = LINK_UNRELEASED_RE.search(text, start, end)
    if link:
        end = link.start()
    return start, end, text[start:end]


def changelog_floor(text: str) -> tuple[str, list[str]]:
    """The least bump the Unreleased entries need, and the categories that have entries."""
    _, _, body = unreleased_section(text)
    found, level = [], NONE
    parts = CATEGORY_RE.split(body)
    # parts: [before, name1, body1, name2, body2, ...]
    for name, content in zip(parts[1::2], parts[2::2]):
        if not re.search(r"^\s*[-*] +\S", content, re.MULTILINE):
            continue
        found.append(name)
        level = max_level([level, CATEGORY_LEVELS.get(name.strip().lower(), NONE)])
    return level, found


def release_changelog(text: str, version: str, date: str, repo_url: str = "",
                      fallback_notes: Iterable[str] = ()) -> tuple[str, str]:
    """Moves the Unreleased entries under `## [version] - date`; returns (text, release notes)."""
    if re.search(rf"^## \[?{re.escape(version)}\]?", text, re.MULTILINE):
        raise ReleaseError(f"the changelog already has a section for {version}")
    start, end, body = unreleased_section(text)
    notes = body.strip("\n")
    if not notes.strip():
        notes = "\n".join(f"- {n}" for n in fallback_notes) or "- No user-visible changes."
    section = f"\n\n## [{version}] - {date}\n\n{notes}\n\n"
    text = text[:start] + section + text[end:].lstrip("\n")
    link = LINK_UNRELEASED_RE.search(text)
    if link and repo_url:
        new_links = (f"[Unreleased]: {repo_url}/compare/v{version}...HEAD\n"
                     f"[{version}]: {repo_url}/releases/tag/v{version}")
        text = text[:link.start()] + new_links + text[link.end():]
    return text.rstrip("\n") + "\n", notes


# ---------------------------------------------------------------------------------------------
# What merged since the last release


@dataclass
class Change:
    """One merged pull request, or one commit pushed to main without one."""
    ref: str          # "#12" or a short commit hash
    title: str
    labels: list[str]
    url: str = ""

    def releases(self) -> bool:
        try:
            return classify(self.title, self.labels)[0] != NONE
        except ReleaseError:
            return False

    def note(self) -> str:
        desc = TITLE_RE.match(self.title)
        text = desc.group("desc") if desc else self.title
        text = text[:1].upper() + text[1:]
        return f"{text} ([{self.ref}]({self.url}))" if self.url else f"{text} ({self.ref})"


@dataclass
class Plan:
    current: str               # what VERSION holds: the last release
    version: Optional[str]     # the version to release, None for no release
    level: str
    reasons: list[str]
    changes: list[Change]

    @property
    def tag(self) -> str:
        return f"v{self.version}"


def plan_release(changes: list[Change], current: str, changelog_text: Optional[str],
                 existing_tags: Iterable[str] = ()) -> Plan:
    """Decides the version a release of `changes` gets."""
    levels, reasons = [], []
    for change in changes:
        try:
            level, why = classify(change.title, change.labels)
        except ReleaseError as error:
            # A merged pull request cannot be fixed any more: it releases nothing, loudly.
            level, why = NONE, f"no bump ({error})"
        levels.append(level)
        reasons.append(f"{change.ref} {change.title!r}: {level} ({why})")
    level = max_level(levels)
    if level != NONE and changelog_text is not None:
        floor, categories = changelog_floor(changelog_text)
        if _ORDER[floor] > _ORDER[level]:
            reasons.append(f"CHANGELOG Unreleased has {', '.join(categories)}: at least {floor}")
            level = floor
    if level == NONE:
        return Plan(current, None, NONE, reasons, changes)
    version = bump(current, level)
    newer = [t for t in existing_tags if _tag_core(t) and _tag_core(t) >= parse_version(version)]
    if newer:
        raise ReleaseError(
            f"tag(s) {', '.join(sorted(newer))} already name {version} or later; release by hand "
            "with the next free number, see the versioning decision record")
    return Plan(current, version, resolve_level(level, parse_version(current)), reasons, changes)


def _tag_core(tag: str) -> Optional[tuple[int, int, int]]:
    match = TAG_RE.match(tag)
    return (int(match.group(1)), int(match.group(2)), int(match.group(3))) if match else None


# ---------------------------------------------------------------------------------------------
# git and GitHub


def run(args: list[str], cwd: Path, check: bool = True) -> subprocess.CompletedProcess:
    result = subprocess.run(args, cwd=str(cwd), text=True, capture_output=True, encoding="utf-8")
    if check and result.returncode != 0:
        raise ReleaseError(f"{' '.join(args)} failed:\n{result.stderr.strip()}")
    return result


def git(root: Path, *args: str, check: bool = True) -> str:
    return run(["git", *args], root, check).stdout.strip()


def last_release_tag(root: Path) -> Optional[str]:
    tag = git(root, "describe", "--tags", "--abbrev=0", "--match", "v[0-9]*", "--exclude", "v*-*",
              "HEAD", check=False)
    return tag or None


def all_tags(root: Path) -> list[str]:
    return [t for t in git(root, "tag", "--list", "v[0-9]*").splitlines() if t]


def head_release_tag(root: Path) -> Optional[str]:
    for tag in git(root, "tag", "--points-at", "HEAD", "--list", "v[0-9]*").splitlines():
        if TAG_RE.match(tag) and "-" not in tag:
            return tag
    return None


def gh_api(path: str) -> object:
    result = subprocess.run(["gh", "api", "-H", "Accept: application/vnd.github+json", path],
                            text=True, capture_output=True, encoding="utf-8")
    if result.returncode != 0:
        raise ReleaseError(f"gh api {path} failed: {result.stderr.strip()}")
    return json.loads(result.stdout or "null")


def collect_changes(root: Path, revision_range: str, repo: str,
                    api: Callable[[str], object] = gh_api) -> list[Change]:
    """The pull requests (or bare commits) on main's first-parent line in the range."""
    log = git(root, "log", "--first-parent", "--format=%H%x1f%s", revision_range)
    changes, seen = [], set()
    for line in log.splitlines():
        sha, subject = line.split("\x1f", 1)
        if RELEASE_SUBJECT_RE.match(subject):
            continue
        try:
            found = api(f"repos/{repo}/commits/{sha}/pulls") or []
        except ReleaseError as error:
            if "No commit found" not in str(error):
                raise
            found = []  # a local commit GitHub has not seen (a dry run of a branch)
        pulls = [p for p in found if p.get("merged_at") and p.get("base", {}).get("ref") == "main"]
        if pulls:
            for pull in pulls:
                if pull["number"] in seen:
                    continue
                seen.add(pull["number"])
                changes.append(Change(f"#{pull['number']}", pull["title"],
                                      [l["name"] for l in pull.get("labels", [])],
                                      pull.get("html_url", "")))
        elif not subject.startswith("Merge "):
            changes.append(Change(sha[:8], subject, [],
                                  f"https://github.com/{repo}/commit/{sha}" if repo else ""))
    return changes


# ---------------------------------------------------------------------------------------------
# Commands


def apply_release(root: Path, config: Config, plan: Plan, date: str, repo_url: str = "") -> str:
    """Writes the new version and the changelog; returns the release notes."""
    assert plan.version
    for entry in config.version_files:
        path = root / entry.path
        text = path.read_text(encoding="utf-8")
        _write(path, rewrite_version(text, entry, plan.current, plan.version))
    notes = ""
    if config.changelog:
        path = root / config.changelog
        text, notes = release_changelog(path.read_text(encoding="utf-8"), plan.version, date,
                                        repo_url, [c.note() for c in plan.changes if c.releases()])
        _write(path, text)
    return notes


def _write(path: Path, text: str) -> None:
    """Writes `text` (read with universal newlines) with the line endings the file had."""
    newline = "\r\n" if b"\r\n" in path.read_bytes() else "\n"
    with open(path, "w", encoding="utf-8", newline=newline) as handle:
        handle.write(text)


def summary(lines: list[str]) -> None:
    for line in lines:
        print(line)
    target = os.environ.get("GITHUB_STEP_SUMMARY")
    if target:
        with open(target, "a", encoding="utf-8") as handle:
            handle.write("\n".join(lines) + "\n")


def output(**values: str) -> None:
    target = os.environ.get("GITHUB_OUTPUT")
    if target:
        with open(target, "a", encoding="utf-8") as handle:
            for key, value in values.items():
                handle.write(f"{key}={value}\n")


def last_release(root: Path, config: Config) -> tuple[Optional[str], str]:
    """(last release tag, VERSION). VERSION holds the last release; a tag that disagrees fails."""
    current = read_current_version(root, config)
    tag = last_release_tag(root)
    if tag and tag[1:] != current:
        raise ReleaseError(
            f"VERSION holds {current}, but the last release tag is {tag}: VERSION must hold the "
            f"version of the last release. Set it (and every file that carries it) to {tag[1:]}.")
    return tag, current


def changelog_text(root: Path, config: Config) -> Optional[str]:
    if config.changelog and (root / config.changelog).exists():
        return (root / config.changelog).read_text(encoding="utf-8")
    return None


def cmd_check(args: argparse.Namespace) -> int:
    root, config = Path(args.root), Config.load(Path(args.root) / args.config)
    labels = [l for l in args.labels.split(",") if l.strip()]
    try:
        level, why = classify(args.title, labels)
    except ReleaseError as error:
        print(f"::error title=Release bump::{error}")
        summary(["### Release bump: undefined", "", f"{error}", "", HELP])
        return 1
    _, current = last_release(root, config)
    log = changelog_text(root, config)
    plan = plan_release([Change("this pull request", args.title, labels)], current, log,
                        all_tags(root))
    lines = [f"### Release bump: {plan.level}", "", f"- {why}"]
    lines += [f"- {r}" for r in plan.reasons[1:]]
    if plan.version:
        lines.append(f"- Merging releases **v{plan.version}** (VERSION, the last release: {current}).")
        if args.base_ref and config.changelog:
            changed = git(root, "diff", "--name-only", f"{args.base_ref}...HEAD", check=False)
            if config.changelog not in changed.splitlines():
                print(f"::warning title=Changelog::this pull request releases v{plan.version} "
                      f"but does not change {config.changelog}; add its entry under Unreleased")
                lines.append(f"- {config.changelog} is unchanged: add the entry under Unreleased.")
    else:
        lines.append("- Merging releases nothing.")
    print(f"::notice title=Release bump::{lines[0][4:]}; "
          + (f"merging releases v{plan.version}" if plan.version else "merging releases nothing"))
    summary(lines)
    return 0


HELP = ("Title: a Conventional Commit, `type(scope): what changed`. `feat` releases a minor "
        "version, `fix`, `perf` and `revert` a patch, `feat!` or BREAKING a major (a minor below "
        "1.0.0); `docs`, `ci`, `chore`, `test`, `refactor`, `style` and `build` release nothing. "
        "A label release:major, release:minor, release:patch or release:none overrides the title.")


def cmd_release(args: argparse.Namespace) -> int:
    root, config = Path(args.root), Config.load(Path(args.root) / args.config)
    repo = args.repo or os.environ.get("GITHUB_REPOSITORY", "")
    repo_url = f"https://github.com/{repo}" if repo else ""
    date = args.date or _dt.datetime.now(_dt.timezone.utc).date().isoformat()

    for attempt in range(1, args.attempts + 1):
        if not args.dry_run:
            git(root, "fetch", "--force", "--tags", "origin", "main")
            git(root, "checkout", "--force", "-B", "main", "origin/main")
        done = head_release_tag(root)
        if done:
            summary([f"HEAD is already released as {done}: nothing to do."])
            output(released="false")
            return 0
        tag, current = last_release(root, config)
        if tag:
            revision_range = f"{tag}..HEAD"
        elif args.before and not re.fullmatch(r"0+", args.before) and \
                run(["git", "cat-file", "-e", f"{args.before}^{{commit}}"], root, False).returncode == 0:
            revision_range = f"{args.before}..HEAD"
        else:
            revision_range = "HEAD~1..HEAD"
        changes = collect_changes(root, revision_range, repo, args.api)
        plan = plan_release(changes, current, changelog_text(root, config), all_tags(root))
        lines = [f"### Release: {('v' + plan.version) if plan.version else 'none'}", "",
                 f"- Last release: {tag or current + ' (VERSION, no tag)'}; changes since: `{revision_range}`"]
        lines += [f"- {r}" for r in plan.reasons]
        if not plan.version:
            summary(lines + ["- Nothing to release."])
            output(released="false")
            return 0
        notes = apply_release(root, config, plan, date, repo_url)
        summary(lines)
        if args.dry_run:
            print(f"Dry run: wrote v{plan.version} into the files; no commit, tag or push.")
            print(git(root, "diff", "--stat"))
            output(released="false", version=plan.version, tag=plan.tag)
            return 0

        files = [e.path for e in config.version_files] + ([config.changelog] if config.changelog else [])
        git(root, "add", "--", *files)
        git(root, "commit", "-m", f"chore(release): {plan.tag}", "-m",
            "\n".join(f"- {c.ref} {c.title}" for c in plan.changes))
        git(root, "tag", "-a", plan.tag, "-m", f"{plan.tag}")
        push = run(["git", "push", "--atomic", "origin", "HEAD:refs/heads/main",
                    f"refs/tags/{plan.tag}"], root, check=False)
        if push.returncode == 0:
            output(released="true", version=plan.version, tag=plan.tag)
            start_release_build(root, config, plan, notes, repo, args.tag_push_triggers)
            return 0
        git(root, "tag", "-d", plan.tag)
        err = push.stderr
        if re.search(r"GH006|GH013|protected branch|rule violations|refusing to allow", err, re.I):
            raise ReleaseError(
                "main rejected the release commit:\n" + err.strip() + "\n\nThe workflow token may "
                "not push to main. Add the repository secret QAROS_RELEASE_TOKEN: a token (a GitHub "
                "App installation token or a fine-grained personal access token) with Contents "
                "read and write on this repository that may bypass the rules of main.")
        if not re.search(r"non-fast-forward|fetch first|rejected|stale info|cannot lock ref", err):
            raise ReleaseError(f"git push failed:\n{err.strip()}")
        print(f"main moved while releasing (attempt {attempt}); starting again from the new main.")
    raise ReleaseError(f"main kept moving: gave up after {args.attempts} attempts; rerun the workflow")


def release_build_commands(config: Config, plan: Plan, repo: str, notes_file: str,
                           tag_push_triggers: bool) -> list[list[str]]:
    """The gh commands that start the release build of the new tag."""
    release = config.release or {}
    workflow = release.get("workflow")
    if workflow:
        if tag_push_triggers:
            return []  # the tag push itself starts the workflow
        command = ["gh", "workflow", "run", workflow, "--repo", repo, "--ref", plan.tag]
        for key, value in (release.get("inputs") or {}).items():
            command += ["-f", f"{key}={str(value).format(tag=plan.tag, version=plan.version)}"]
        return [command]
    if release.get("github_release"):
        name = release.get("name") or repo.split("/")[-1]
        return [["gh", "release", "create", plan.tag, "--repo", repo, "--verify-tag",
                 "--title", f"{name} {plan.version}", "--notes-file", notes_file]]
    return []


def start_release_build(root: Path, config: Config, plan: Plan, notes: str, repo: str,
                        tag_push_triggers: bool) -> None:
    notes_file = root / ".git" / "app-release-notes.md"
    notes_file.write_text(notes + "\n", encoding="utf-8")
    commands = release_build_commands(config, plan, repo, str(notes_file), tag_push_triggers)
    if not commands:
        print(f"{plan.tag} pushed; the tag push starts the release build." if config.release
              else f"{plan.tag} pushed; this repository has no release build.")
    for command in commands:
        print("+ " + " ".join(command))
        run(command, root)


def cmd_apply(args: argparse.Namespace) -> int:
    root, config = Path(args.root), Config.load(Path(args.root) / args.config)
    current = read_current_version(root, config)
    parse_version(args.version)
    plan = Plan(current, args.version, NONE, [], [Change("manual", "release", [])])
    date = args.date or _dt.date.today().isoformat()
    print(apply_release(root, config, plan, date, args.repo_url))
    return 0


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--config", default=".github/app-release.json")
    sub = parser.add_subparsers(dest="command", required=True)

    check = sub.add_parser("check", help="the bump a pull request defines")
    check.add_argument("--title", required=True)
    check.add_argument("--labels", default="", help="comma-separated labels")
    check.add_argument("--base-ref", default="", help="warn when the changelog is unchanged since")
    check.set_defaults(func=cmd_check)

    release = sub.add_parser("release", help="commit, tag and push the release of main")
    release.add_argument("--repo", default="", help="owner/name (default GITHUB_REPOSITORY)")
    release.add_argument("--before", default="", help="main before the push (repositories without tags)")
    release.add_argument("--date", default="")
    release.add_argument("--attempts", type=int, default=5)
    release.add_argument("--dry-run", action="store_true", help="rewrite the files only")
    release.add_argument("--tag-push-triggers", action="store_true",
                         help="the push token starts workflows, so the tag push starts the build")
    release.set_defaults(func=cmd_release, api=gh_api)

    apply = sub.add_parser("apply", help="rewrite the version files and the changelog")
    apply.add_argument("--version", required=True)
    apply.add_argument("--date", default="")
    apply.add_argument("--repo-url", default="")
    apply.set_defaults(func=cmd_apply)

    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except ReleaseError as error:
        message = str(error)
        print(f"::error title=Release::{message.splitlines()[0]}", file=sys.stderr)
        print(message, file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
