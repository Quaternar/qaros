# SPDX-License-Identifier: MIT
"""Unit tests of app_release.py: python -m unittest discover -s .github/app-release -v"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import app_release as ar  # noqa: E402


class Classify(unittest.TestCase):
    def level(self, title, labels=()):
        return ar.classify(title, labels)[0]

    def test_title_types(self):
        self.assertEqual(self.level("feat: grab to move"), ar.MINOR)
        self.assertEqual(self.level("feat(panels): open sooner"), ar.MINOR)
        self.assertEqual(self.level("fix: crash at stop"), ar.PATCH)
        self.assertEqual(self.level("perf: cache meshes"), ar.PATCH)
        self.assertEqual(self.level("revert: feat: grab"), ar.PATCH)
        for kind in ("docs", "ci", "chore", "test", "refactor", "style", "build"):
            self.assertEqual(self.level(f"{kind}: something"), ar.NONE, kind)

    def test_breaking(self):
        self.assertEqual(self.level("feat!: drop --old"), ar.BREAKING)
        self.assertEqual(self.level("fix(api)!: rename"), ar.BREAKING)
        self.assertEqual(self.level("feat: BREAKING CHANGE drop --old"), ar.BREAKING)
        self.assertEqual(self.level("feat: BREAKING: drop --old"), ar.BREAKING)

    def test_case_of_type(self):
        self.assertEqual(self.level("Fix: crash"), ar.PATCH)

    def test_label_wins(self):
        self.assertEqual(self.level("docs: readme", ["release:patch"]), ar.PATCH)
        self.assertEqual(self.level("feat: x", ["bug", "release:none"]), ar.NONE)
        self.assertEqual(self.level("feat: x", ["release:major"]), ar.MAJOR)
        self.assertEqual(self.level("not conventional", ["release:minor"]), ar.MINOR)

    def test_errors(self):
        for title, labels in [("Add a thing", []), ("feature: x", []), ("fix:no space", []),
                              ("fix: x", ["release:minor", "release:patch"]),
                              ("fix: x", ["release:huge"])]:
            with self.assertRaises(ar.ReleaseError, msg=title):
                ar.classify(title, labels)


class Bump(unittest.TestCase):
    def test_levels(self):
        self.assertEqual(ar.bump("0.4.1", ar.PATCH), "0.4.2")
        self.assertEqual(ar.bump("0.4.1", ar.MINOR), "0.5.0")
        self.assertEqual(ar.bump("0.4.1", ar.MAJOR), "1.0.0")
        self.assertEqual(ar.bump("1.2.3", ar.MAJOR), "2.0.0")

    def test_breaking_is_minor_below_one(self):
        self.assertEqual(ar.bump("0.4.1", ar.BREAKING), "0.5.0")
        self.assertEqual(ar.bump("1.4.1", ar.BREAKING), "2.0.0")

    def test_none_is_an_error(self):
        with self.assertRaises(ar.ReleaseError):
            ar.bump("0.4.1", ar.NONE)


CHANGELOG = """# Changelog

Intro.

## [Unreleased]

### Features

- A thing.

### Bug fixes

- Another thing.

## [0.1.0] - 2026-11-14

### Features

- First.

[Unreleased]: https://github.com/o/r/commits/main
"""


class Changelog(unittest.TestCase):
    def test_floor(self):
        self.assertEqual(ar.changelog_floor(CHANGELOG), (ar.MINOR, ["Features", "Bug fixes"]))
        breaking = CHANGELOG.replace("### Bug fixes", "### Breaking")
        self.assertEqual(ar.changelog_floor(breaking)[0], ar.BREAKING)
        empty = "## [Unreleased]\n\n### Features\n\n## [0.1.0]\n"
        self.assertEqual(ar.changelog_floor(empty), (ar.NONE, []))

    def test_move(self):
        text, notes = ar.release_changelog(CHANGELOG, "0.2.0", "2026-11-16", "https://github.com/o/r")
        self.assertIn("## [Unreleased]\n\n## [0.2.0] - 2026-11-16\n\n### Features\n\n- A thing.", text)
        self.assertIn("- Another thing.\n\n## [0.1.0] - 2026-11-14", text)
        self.assertTrue(notes.startswith("### Features\n\n- A thing."))
        self.assertIn("[Unreleased]: https://github.com/o/r/compare/v0.2.0...HEAD\n"
                      "[0.2.0]: https://github.com/o/r/releases/tag/v0.2.0\n", text)
        self.assertEqual(ar.unreleased_section(text)[2].strip(), "")
        with self.assertRaises(ar.ReleaseError):
            ar.release_changelog(text, "0.2.0", "2026-11-16")

    def test_move_without_links_and_last_section(self):
        text, _ = ar.release_changelog("# C\n\n## Unreleased\n\n- x\n", "1.0.0", "2026-01-02")
        self.assertEqual(text, "# C\n\n## Unreleased\n\n## [1.0.0] - 2026-01-02\n\n- x\n")

    def test_empty_unreleased_uses_fallback(self):
        text = "# C\n\n## [Unreleased]\n\n## [0.1.0] - 2026-01-01\n"
        new, notes = ar.release_changelog(text, "0.1.1", "2026-01-02", fallback_notes=["Fix (#3)"])
        self.assertEqual(notes, "- Fix (#3)")
        self.assertIn("## [0.1.1] - 2026-01-02\n\n- Fix (#3)\n\n## [0.1.0]", new)


class VersionFiles(unittest.TestCase):
    def test_plain(self):
        entry = ar.VersionFile("VERSION", r"^(?P<version>\d+\.\d+\.\d+)$")
        self.assertEqual(ar.rewrite_version("0.5.0\n", entry, "0.5.0", "0.6.0"), "0.6.0\n")

    def test_tuple(self):
        entry = ar.VersionFile("__init__.py", r'"version": \((?P<version>\d+, \d+, \d+)\)', "tuple")
        text = 'bl_info = {\n    "version": (0, 5, 0),\n}\n'
        self.assertEqual(ar.rewrite_version(text, entry, "0.5.0", "0.5.1"),
                         'bl_info = {\n    "version": (0, 5, 1),\n}\n')

    def test_count_and_mismatch(self):
        entry = ar.VersionFile("Cargo.lock", r'^name = "(?:a|b)"\nversion = "(?P<version>[^"]+)"', count=2)
        text = 'name = "a"\nversion = "0.2.0"\n\nname = "c"\nversion = "0.2.0"\n\nname = "b"\nversion = "0.2.0"\n'
        out = ar.rewrite_version(text, entry, "0.2.0", "0.3.0")
        self.assertEqual(out.count('"0.3.0"'), 2)
        self.assertIn('name = "c"\nversion = "0.2.0"', out)
        with self.assertRaises(ar.ReleaseError):
            ar.rewrite_version(text.replace('"b"', '"x"'), entry, "0.2.0", "0.3.0")
        with self.assertRaises(ar.ReleaseError):
            ar.rewrite_version(text, entry, "0.1.0", "0.3.0")

    def test_apply_keeps_line_endings(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "VERSION").write_bytes(b"0.1.0\r\n")
            (root / "CHANGELOG.md").write_bytes(b"# C\r\n\r\n## [Unreleased]\r\n\r\n- x\r\n")
            config = ar.Config([ar.VersionFile("VERSION", r"^(?P<version>\d+\.\d+\.\d+)$")])
            plan = ar.Plan("0.1.0", "0.1.1", ar.PATCH, [], [])
            ar.apply_release(root, config, plan, "2026-11-16")
            self.assertEqual((root / "VERSION").read_bytes(), b"0.1.1\r\n")
            self.assertIn(b"## [0.1.1] - 2026-11-16\r\n\r\n- x\r\n", (root / "CHANGELOG.md").read_bytes())

    def test_package_xml(self):
        entry = ar.VersionFile("package.xml", r"<version>(?P<version>[^<]+)</version>")
        self.assertEqual(ar.rewrite_version("  <version>0.4.0</version>\n", entry, "0.4.0", "0.4.1"),
                         "  <version>0.4.1</version>\n")


class Plan(unittest.TestCase):
    def change(self, title, labels=()):
        return ar.Change("#1", title, list(labels))

    def test_max_of_changes(self):
        plan = ar.plan_release([self.change("fix: a"), self.change("feat: b"), self.change("docs: c")],
                               "0.4.1", None)
        self.assertEqual((plan.version, plan.level), ("0.5.0", ar.MINOR))

    def test_nothing(self):
        plan = ar.plan_release([self.change("docs: a"), self.change("chore(release): v1.0.0")],
                               "0.4.1", CHANGELOG)
        self.assertIsNone(plan.version)

    def test_invalid_merged_title_releases_nothing(self):
        plan = ar.plan_release([self.change("Update things")], "0.4.1", None)
        self.assertIsNone(plan.version)
        self.assertIn("no bump", plan.reasons[0])

    def test_changelog_floor_raises(self):
        plan = ar.plan_release([self.change("fix: a")], "0.4.1", CHANGELOG)
        self.assertEqual(plan.version, "0.5.0")
        breaking = CHANGELOG.replace("### Bug fixes", "### Breaking")
        plan = ar.plan_release([self.change("fix: a")], "1.4.1", breaking)
        self.assertEqual(plan.version, "2.0.0")

    def test_version_zero_before_the_first_release(self):
        plan = ar.plan_release([self.change("feat: a")], "0.0.0", None)
        self.assertEqual(plan.version, "0.1.0")

    def test_existing_tag_blocks(self):
        with self.assertRaises(ar.ReleaseError):
            ar.plan_release([self.change("fix: a")], "0.4.1", None, ["v0.4.2-rc.1"])
        plan = ar.plan_release([self.change("fix: a")], "0.4.1", None, ["v0.4.1", "v0.4.0"])
        self.assertEqual(plan.version, "0.4.2")

    def test_note(self):
        self.assertEqual(ar.Change("#7", "fix(ui): panel flicker", [], "https://x/7").note(),
                         "Panel flicker ([#7](https://x/7))")


class ReleaseBuild(unittest.TestCase):
    plan = ar.Plan("0.1.0", "0.2.0", ar.MINOR, [], [])

    def test_dispatch(self):
        config = ar.Config([], release={"workflow": "release.yml", "inputs": {"tag": "{tag}", "v": "{version}"}})
        self.assertEqual(ar.release_build_commands(config, self.plan, "o/r", "n.md", False),
                         [["gh", "workflow", "run", "release.yml", "--repo", "o/r", "--ref", "v0.2.0",
                           "-f", "tag=v0.2.0", "-f", "v=0.2.0"]])
        self.assertEqual(ar.release_build_commands(config, self.plan, "o/r", "n.md", True), [])

    def test_github_release(self):
        config = ar.Config([], release={"github_release": True})
        self.assertEqual(ar.release_build_commands(config, self.plan, "o/qaros-qt", "n.md", False)[0][:4],
                         ["gh", "release", "create", "v0.2.0"])


def git(root, *args):
    return subprocess.run(["git", *args], cwd=root, check=True, text=True, capture_output=True).stdout.strip()


class EndToEnd(unittest.TestCase):
    """`release` against a local bare repository standing in for GitHub."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.remote, self.work = base / "remote.git", base / "work"
        git(base, "init", "--bare", "-b", "main", str(self.remote))
        git(base, "clone", str(self.remote), str(self.work))
        for key, value in [("user.name", "t"), ("user.email", "t@example.com"), ("commit.gpgsign", "false"),
                           ("tag.gpgsign", "false")]:
            git(self.work, "config", key, value)
        (self.work / ".github").mkdir()
        (self.work / ".github" / "app-release.json").write_text(json.dumps({
            "version_files": [
                {"path": "VERSION", "pattern": r"^(?P<version>\d+\.\d+\.\d+)$"},
                {"path": "package.xml", "pattern": r"<version>(?P<version>[^<]+)</version>"}],
            "changelog": "CHANGELOG.md"}))
        (self.work / "VERSION").write_text("0.1.0\n")
        (self.work / "package.xml").write_text("<version>0.1.0</version>\n")
        (self.work / "CHANGELOG.md").write_text("# C\n\n## [Unreleased]\n\n## [0.1.0] - 2026-01-01\n")
        git(self.work, "add", ".")
        git(self.work, "commit", "-m", "chore: start")
        git(self.work, "tag", "v0.1.0")
        git(self.work, "push", "origin", "main", "--tags")

    def tearDown(self):
        self.tmp.cleanup()

    def merge(self, title, labels=(), entry=None):
        if entry:
            text = (self.work / "CHANGELOG.md").read_text()
            (self.work / "CHANGELOG.md").write_text(text.replace("## [Unreleased]\n", f"## [Unreleased]\n\n{entry}\n", 1))
        (self.work / "f.txt").write_text(title)
        git(self.work, "add", ".")
        git(self.work, "commit", "-m", title)
        git(self.work, "push", "origin", "main")
        sha = git(self.work, "rev-parse", "HEAD")
        self.pulls[sha] = [{"number": len(self.pulls) + 10, "title": title, "merged_at": "x",
                            "base": {"ref": "main"}, "labels": [{"name": l} for l in labels],
                            "html_url": f"https://github.com/o/r/pull/{len(self.pulls) + 10}"}]

    pulls: dict = {}

    def release(self, *extra):
        api = lambda path: self.pulls.get(path.split("/")[-2], [])  # noqa: E731
        parser_args = ["--root", str(self.work), "release", "--repo", "o/r", "--date", "2026-11-16", *extra]
        original = ar.gh_api
        ar.gh_api = api
        try:
            return ar.main(parser_args)
        finally:
            ar.gh_api = original

    def test_release_and_idempotent(self):
        self.pulls = {}
        self.merge("docs: readme")
        self.assertEqual(self.release(), 0)
        self.assertEqual(git(self.work, "tag", "--list", "v0.1.1"), "")
        self.merge("fix: crash", entry="### Bug fixes\n\n- No crash.")
        self.merge("feat: grab", ["release:patch"])
        self.assertEqual(self.release(), 0)
        # The changelog's Bug fixes needs a patch; the label asks patch: 0.1.1.
        remote_main = git(self.remote, "log", "-1", "--format=%s", "main")
        self.assertEqual(remote_main, "chore(release): v0.1.1")
        self.assertEqual(git(self.remote, "tag", "--list", "v0.1.1"), "v0.1.1")
        self.assertEqual((self.work / "VERSION").read_text(), "0.1.1\n")
        self.assertEqual((self.work / "package.xml").read_text(), "<version>0.1.1</version>\n")
        log = (self.work / "CHANGELOG.md").read_text()
        self.assertIn("## [Unreleased]\n\n## [0.1.1] - 2026-11-16\n\n### Bug fixes\n\n- No crash.", log)
        # Running again releases nothing: HEAD carries the tag.
        self.assertEqual(self.release(), 0)
        self.assertEqual(git(self.remote, "rev-list", "--count", "main"), "5")

    def test_main_moves_during_release(self):
        self.pulls = {}
        self.merge("feat: one")
        # Another clone merges after this one fetched: the push is rejected, the run starts again.
        other = Path(self.tmp.name) / "other"
        git(Path(self.tmp.name), "clone", str(self.remote), str(other))
        git(other, "config", "user.email", "o@example.com")
        git(other, "config", "user.name", "o")
        original_git = ar.git
        state = {"pushed": False}

        def racing_git(root, *args, check=True):
            result = original_git(root, *args, check=check)
            if args[:1] == ("tag",) and args[1:2] == ("-a",) and not state["pushed"]:
                state["pushed"] = True
                (other / "g.txt").write_text("x")
                original_git(other, "add", ".")
                original_git(other, "commit", "-m", "fix: two")
                original_git(other, "push", "origin", "main")
                self.pulls[original_git(other, "rev-parse", "HEAD")] = [
                    {"number": 99, "title": "fix: two", "merged_at": "x", "base": {"ref": "main"}, "labels": []}]
            return result

        ar.git = racing_git
        try:
            self.assertEqual(self.release(), 0)
        finally:
            ar.git = original_git
        self.assertEqual(git(self.remote, "tag", "--list", "v0.2.0"), "v0.2.0")
        self.assertEqual(git(self.remote, "log", "-2", "--format=%s", "main").splitlines(),
                         ["chore(release): v0.2.0", "fix: two"])

    def test_version_must_hold_the_last_release(self):
        self.pulls = {}
        (self.work / "VERSION").write_text("0.2.0\n")
        self.merge("fix: one")
        self.assertEqual(self.release(), 1)
        self.assertEqual(git(self.remote, "tag", "--list", "v0.*"), "v0.1.0")

    def test_without_tags_uses_the_pushed_range(self):
        self.pulls = {}
        git(self.work, "tag", "-d", "v0.1.0")
        git(self.work, "push", "origin", ":refs/tags/v0.1.0")
        before = git(self.work, "rev-parse", "HEAD")
        self.merge("fix: one")
        self.merge("docs: guide")
        self.merge("fix: two")
        self.assertEqual(self.release("--before", before), 0)
        log = (self.work / "CHANGELOG.md").read_text()
        # The empty Unreleased section is filled from the releasing pull requests only.
        self.assertIn("- One ([#10](https://github.com/o/r/pull/10))\n", log)
        self.assertIn("- Two ([#12](https://github.com/o/r/pull/12))\n", log)
        self.assertNotIn("Guide", log)
        self.assertEqual(git(self.remote, "tag", "--list", "v0.*"), "v0.1.1")
        body = git(self.remote, "log", "-1", "--format=%b", "main")
        self.assertIn("fix: one", body)
        self.assertIn("fix: two", body)

    def test_dry_run_changes_files_only(self):
        self.pulls = {}
        self.merge("fix: one")
        self.assertEqual(self.release("--dry-run"), 0)
        self.assertEqual((self.work / "VERSION").read_text(), "0.1.1\n")
        self.assertEqual((self.work / "package.xml").read_text(), "<version>0.1.1</version>\n")
        self.assertEqual(git(self.work, "tag", "--list", "v0.1.1"), "")
        self.assertEqual(git(self.remote, "log", "-1", "--format=%s", "main"), "fix: one")


if __name__ == "__main__":
    unittest.main()
