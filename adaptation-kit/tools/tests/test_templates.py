"""Every template of the real kit: init -> check -> test -> package (unsigned, signed, unpacked)."""

from __future__ import annotations

import json
import unittest

from .helpers import KIT, NODE, Scratch, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402
from qaros_adaptation import qarapp  # noqa: E402
from qaros_adaptation.cli import templates  # noqa: E402
from qaros_adaptation.profile import check_adaptation  # noqa: E402

TEMPLATES = ("threejs", "webgl", "webgpu", "webgpu-redirect")
ENGINES = {"threejs": "three.js", "webgl": "webgl", "webgpu": "webgpu", "webgpu-redirect": "webgpu"}


class TemplateTests(Scratch):
    def test_the_kit_ships_every_template(self) -> None:
        self.assertEqual(templates(KIT), sorted(TEMPLATES))
        for template in TEMPLATES:
            folder = KIT / "templates" / template
            with self.subTest(template):
                self.assertTrue((folder / "README.md").is_file())
                self.assertTrue(list((folder / "test").glob("*.test.mjs")))

    def test_init_check_test_package(self) -> None:
        kit = kitlib.load_kit(KIT)
        key = qarapp.keygen()  # a throwaway key for this run
        key_pem = self.dir / "key.pem"
        key_pem.write_text(qarapp.private_key_pem(key.scalar), encoding="ascii")
        for template in TEMPLATES:
            with self.subTest(template):
                folder = self.dir / f"my-{template}"
                self.assertRuns(run_cli("init", template, folder, "--address", f"https://example.com/{template}/viewer.html", "--id", f"com.example.{template}"))
                profile = json.loads((folder / "profile.json").read_text(encoding="utf-8"))
                self.assertEqual(profile["Engine"], ENGINES[template])
                self.assertNotIn("{{", json.dumps(profile))

                report = check_adaptation(folder, kit)
                self.assertEqual([str(finding) for finding in report.findings], [], "a fresh template checks clean")
                self.assertRuns(run_cli("check", folder, "--strict"))

                if NODE is not None:
                    self.assertRuns(run_cli("test", folder))

                out = self.dir / "out"
                self.assertRuns(run_cli("package", folder, "--out", out))
                package = out / f"com.example.{template}-1.0.0.qarapp"
                contents = qarapp.read_package(package, None)
                self.assertFalse(any(path.startswith("test/") for path in contents.files))
                libraries = [script["Library"] for script in contents.profile["Scripts"] if "Library" in script]
                self.assertTrue(libraries and all("@" in library for library in libraries))

                self.assertRuns(run_cli("package", folder, "--out", self.dir / "signed", "--sign", key_pem))
                signed = qarapp.read_package(self.dir / "signed" / package.name, [("Quaternar", key.public_der)])
                self.assertTrue(signed.verification.signed)

                unpacked = self.dir / "unpacked" / template
                self.assertRuns(run_cli("package", folder, "--unpacked", "--out", unpacked, "--version", "0.11.0"))
                self.assertEqual(qarapp.verify_folder(unpacked, None).version, "0.11.0")

    @unittest.skipIf(NODE is None, "Node.js is not on PATH")
    def test_template_tests_run_in_place(self) -> None:
        for template in TEMPLATES:
            with self.subTest(template):
                self.assertRuns(run_cli("test", KIT / "templates" / template, "--no-kit-tests"))


class RealKitTests(Scratch):
    def test_real_kit_indexes(self) -> None:
        index = kitlib.compute_index(KIT, "dev")
        names = {entry["Name"] for entry in index["Libraries"]}
        for template in TEMPLATES:
            profile = json.loads((KIT / "templates" / template / "profile.json").read_text(encoding="utf-8"))
            for script in profile["Scripts"]:
                if "Library" in script:
                    self.assertIn(kitlib.parse_reference(script["Library"])[0], names, f"{template}: {script['Library']}")
        out = self.dir / "index.json"
        self.assertRuns(run_cli("index", KIT, "--out", out, "--version", "0.11.0"))
        self.assertRuns(run_cli("index", KIT, "--out", out, "--check", "--version", "0.11.0"))
