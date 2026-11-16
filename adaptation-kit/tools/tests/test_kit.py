"""The kit's library headers, index (`index`, `index --check`) and reference resolution."""

from __future__ import annotations

import hashlib
import json

from .helpers import Scratch, make_kit, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402  (helpers puts tools/ on sys.path)


class IndexTests(Scratch):
    def test_index_lists_every_library_sorted_with_hashes(self) -> None:
        kit = make_kit(self.dir)
        index = kitlib.compute_index(kit, "1.2.3")
        self.assertEqual(index["_sv"], 1)
        self.assertEqual(index["KitVersion"], "1.2.3")
        self.assertEqual([(entry["Name"], entry["Major"]) for entry in index["Libraries"]],
                         [("capture", 1), ("core", 1), ("render", 1), ("render", 2)])
        render = index["Libraries"][2]
        self.assertEqual(render, {
            "Name": "render", "Major": 1, "Minor": 4, "Stage": "afterLoad", "Needs": ["core@1", "capture@1"],
            "File": "render@1.js", "Sha256": hashlib.sha256((kit / "lib/render@1.js").read_bytes()).hexdigest(),
        })
        self.assertEqual(kitlib.compute_index(kit)["KitVersion"], "dev")

    def test_index_command_writes_and_checks(self) -> None:
        kit = make_kit(self.dir)
        self.assertRuns(run_cli("index", kit, "--version", "0.11.0"))
        written = json.loads((kit / "lib/index.json").read_text(encoding="utf-8"))
        self.assertEqual(written["KitVersion"], "0.11.0")
        self.assertRuns(run_cli("index", kit, "--check"))
        self.assertRuns(run_cli("index", kit, "--check", "--version", "0.11.0"))
        self.assertRuns(run_cli("index", kit, "--check", "--version", "0.12.0"), 1)

        (kit / "lib/core@1.js").write_text("// @qaros-library core@1.3\n// @stage afterLoad\n", encoding="utf-8")
        completed = run_cli("index", kit, "--check")
        self.assertRuns(completed, 1)
        self.assertIn("core@1: Minor, Sha256 differ", completed.stderr)

        (kit / "lib/extra@1.js").write_text("// @qaros-library extra@1.0\n// @stage afterLoad\n", encoding="utf-8")
        self.assertIn("extra@1 is missing from the index", run_cli("index", kit, "--check").stderr)

    def test_index_out_elsewhere(self) -> None:
        kit = make_kit(self.dir)
        out = self.dir / "staged" / "index.json"
        self.assertRuns(run_cli("index", kit, "--out", out))
        self.assertTrue(out.is_file())
        self.assertFalse((kit / "lib/index.json").exists())
        self.assertRuns(run_cli("index", kit, "--out", out, "--check"))
        self.assertRuns(run_cli("index", kit, "--check"), 1)

    def test_bad_headers_are_refused(self) -> None:
        cases = {
            "core@1.js": ("// just a comment\n", "no '// @qaros-library core@1.<minor>' header"),
            "core@2.js": ("// @qaros-library core@3.0\n// @stage afterLoad\n", "the header names core@3"),
            "x@1.js": ("// @qaros-library x@1.0\n// @stage whenever\n", "@stage"),
            "y@1.js": ("// @qaros-library y@1.0\n// @stage afterLoad\n// @needs nothing-here\n", "not name@major"),
            "z@1.js": ("// @qaros-library z@1.0\n// @stage afterLoad\n// @needs missing@1\n", "needs missing@1"),
            "Bad_Name.js": ("// @qaros-library bad@1.0\n", "named <name>@<major>.js"),
        }
        for name, (text, message) in cases.items():
            with self.subTest(name):
                kit = self.dir / name / "kit"
                (kit / "lib").mkdir(parents=True)
                (kit / "lib" / name).write_text(text, encoding="utf-8")
                with self.assertRaisesRegex(kitlib.KitError, message.replace("(", r"\(").replace(")", r"\)")):
                    kitlib.compute_index(kit)

    def test_header_after_leading_blank_and_bom(self) -> None:
        header = kitlib.parse_header("﻿\n// @qaros-library a-b@3.7\n// @stage preDocument\n// @needs c@1 d@2\ncode();\n// @needs late@1\n")
        self.assertEqual(header, kitlib.LibraryHeader("a-b", 3, 7, "preDocument", ("c@1", "d@2")))
        self.assertIsNone(kitlib.parse_header("// a plain script\nwindow.x = 1;\n"))


class ResolutionTests(Scratch):
    def test_references_and_errors(self) -> None:
        kit = kitlib.load_kit(make_kit(self.dir))
        self.assertFalse(kit.from_index)
        self.assertEqual(kit.resolve("render@2").minor, 0)
        self.assertEqual(kit.resolve("render.js").key, "render@1", "the earlier name.js form means @1")
        with self.assertRaisesRegex(kitlib.KitError, r"requires `render@3`, this kit provides @1, @2"):
            kit.resolve("render@3")
        with self.assertRaisesRegex(kitlib.KitError, r"requires `nope@1`, which this kit does not provide"):
            kit.resolve("nope@1")
        with self.assertRaisesRegex(kitlib.KitError, "not a library reference"):
            kit.resolve("render")

    def test_staged_kit_reads_its_index_and_refuses_a_changed_file(self) -> None:
        directory = make_kit(self.dir)
        kitlib.write_index(directory, "0.11.0")
        kit = kitlib.load_kit(directory)
        self.assertTrue(kit.from_index)
        self.assertEqual(kit.version, "0.11.0")
        library = kit.resolve("core@1")
        self.assertEqual(kit.read(library), (directory / "lib/core@1.js").read_bytes())
        (directory / "lib/core@1.js").write_text("changed", encoding="utf-8")
        with self.assertRaisesRegex(kitlib.KitError, "does not match index.json"):
            kit.read(library)
