"""`init` (from a template) and `pin` (a Library reference to a pinned copy)."""

from __future__ import annotations

import json

from .helpers import Scratch, adaptation_profile, make_adaptation, make_kit, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402
from qaros_adaptation.cli import CliError, init_adaptation, pin_library  # noqa: E402
from qaros_adaptation.profile import check_adaptation  # noqa: E402


def read(folder):
    return json.loads((folder / "profile.json").read_text(encoding="utf-8"))


class InitTests(Scratch):
    def setUp(self) -> None:
        super().setUp()
        self.kit = make_kit(self.dir)

    def test_init_fills_placeholders_and_keeps_the_rest(self) -> None:
        folder = self.dir / "My Viewer"
        completed = run_cli("--kit", self.kit, "init", "basic", folder, "--address", "https://example.com/viewer/*", "--display-name", "My \"Viewer\"")
        self.assertRuns(completed)
        profile = read(folder)
        self.assertEqual(profile["Name"], "my-viewer")
        self.assertEqual(profile["Id"], "com.example.my-viewer")
        self.assertEqual(profile["DisplayName"], 'My "Viewer"', "JSON-escaped, not pasted into the text")
        self.assertEqual(profile["AddressPattern"], "https://example.com/viewer/*")
        self.assertEqual(profile["Engine"], "basic-engine", "the template's engine by default")
        self.assertEqual(profile["FutureField"], {"kept": True}, "unknown template fields are kept")
        self.assertEqual(list(profile)[:3], ["_sv", "Id", "Name"], "field order is kept")
        self.assertEqual((folder / "README.md").read_text(encoding="utf-8"), '# My "Viewer"\n')
        self.assertIn('My "Viewer" site script', (folder / "scripts/site.js").read_text(encoding="utf-8"))
        self.assertTrue((folder / "test/site.test.mjs").is_file())
        self.assertTrue(check_adaptation(folder, kitlib.load_kit(self.kit)).ok)

    def test_init_options(self) -> None:
        folder = self.dir / "x"
        profile = init_adaptation(self.kit, "basic", folder, name="n", address="https://a.org/x.html", engine="webgl", adaptation_id="org.a.x")
        self.assertEqual((profile["Name"], profile["Engine"], profile["Id"], profile["DisplayName"]), ("n", "webgl", "org.a.x", "n"))

    def test_init_refusals(self) -> None:
        with self.assertRaisesRegex(CliError, "no template 'nope'; the kit has: basic"):
            init_adaptation(self.kit, "nope", self.dir / "a", address="https://a.org/")
        with self.assertRaisesRegex(CliError, "pass --address"):
            init_adaptation(self.kit, "basic", self.dir / "b")
        with self.assertRaisesRegex(CliError, "not a valid adaptation id"):
            init_adaptation(self.kit, "basic", self.dir / "c", address="https://a.org/", adaptation_id="bad id")
        occupied = self.dir / "d"
        occupied.mkdir()
        (occupied / "file").write_text("x", encoding="utf-8")
        with self.assertRaisesRegex(CliError, "not empty"):
            init_adaptation(self.kit, "basic", occupied, address="https://a.org/")
        completed = run_cli("--kit", self.kit, "init", "basic", self.dir / "e")
        self.assertRuns(completed, 1)
        self.assertIn("pass --address", completed.stderr, "no prompt without a terminal")


class PinTests(Scratch):
    def setUp(self) -> None:
        super().setUp()
        self.kit_dir = make_kit(self.dir)
        self.kit = kitlib.load_kit(self.kit_dir)

    def test_pin_replaces_the_reference_and_keeps_the_entry(self) -> None:
        profile = adaptation_profile(FutureField=[1, 2])
        profile["Scripts"][2] = {"Name": "render", "Stage": "afterLoad", "Library": "render@1", "Extra": "kept"}
        folder = make_adaptation(self.dir, "a", profile)
        library, target, count = pin_library(folder, self.kit, "render")
        self.assertEqual((library.key, count), ("render@1", 1))
        self.assertEqual(target, folder / "scripts/render@1.js")
        self.assertTrue(target.read_text(encoding="utf-8").startswith("// @qaros-library render@1.4\n"), "the header shows the pinned version")
        written = read(folder)
        self.assertEqual(written["Scripts"][2], {"Name": "render", "Stage": "afterLoad", "File": "scripts/render@1.js", "Extra": "kept"})
        self.assertEqual(list(written["Scripts"][2]), ["Name", "Stage", "File", "Extra"])
        self.assertEqual(written["FutureField"], [1, 2])
        self.assertTrue(check_adaptation(folder, self.kit).ok, "a pinned copy still provides render@1 to the needs check")

    def test_pin_legacy_reference_and_command_line(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][1]["Library"] = "core.js"
        folder = make_adaptation(self.dir, "b", profile)
        completed = run_cli("pin", folder, "core.js", "--kit", self.kit_dir)
        self.assertRuns(completed)
        self.assertIn("pinned core@1 1.2", completed.stdout)
        self.assertEqual(read(folder)["Scripts"][1]["File"], "scripts/core@1.js")

    def test_pin_refusals(self) -> None:
        folder = make_adaptation(self.dir, "c")
        with self.assertRaisesRegex(CliError, "references library 'nope'"):
            pin_library(folder, self.kit, "nope")
        with self.assertRaisesRegex(CliError, "references library 'render@2'"):
            pin_library(folder, self.kit, "render@2")
        (folder / "scripts/render@1.js").write_text("// my edit\n", encoding="utf-8")
        with self.assertRaisesRegex(CliError, "pass --force"):
            pin_library(folder, self.kit, "render@1")
        pin_library(folder, self.kit, "render@1", force=True)
