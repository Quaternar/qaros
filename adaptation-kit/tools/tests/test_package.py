"""`package`: the `.qarapp` and the unpacked folder, signed and unsigned, and what the build writes
into the copy (manifest, version, page region) while keeping everything else."""

from __future__ import annotations

import json

from .helpers import Scratch, adaptation_profile, make_adaptation, make_kit, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402
from qaros_adaptation import packaging, qarapp  # noqa: E402


class PackageTests(Scratch):
    def setUp(self) -> None:
        super().setUp()
        self.kit_dir = make_kit(self.dir)
        self.kit = kitlib.load_kit(self.kit_dir)
        profile = adaptation_profile(FutureField={"from": "a newer QAROS"})
        profile["Scripts"][0]["FutureScriptField"] = 7
        self.source = make_adaptation(self.dir, "source", profile, {
            "scripts/site.js": "const region = [__PAGE_REGION_WIDTH__, __PAGE_REGION_HEIGHT__];\n",
            "panels/controls/index.html": "<!doctype html>\n",
            "test/site.test.mjs": "// authoring only\n",
            "README.md": "# Basic\n",
        })
        # A throwaway key, made for this run only; no key is ever committed.
        self.key = qarapp.keygen()
        self.key_pem = self.dir / "key.pem"
        self.key_pem.write_text(qarapp.private_key_pem(self.key.scalar), encoding="ascii")
        self.public_pem = self.dir / "key.pub.pem"
        self.public_pem.write_text(qarapp.public_key_pem(self.key.public_der), encoding="ascii")

    def test_unsigned_package(self) -> None:
        result = packaging.build_package(self.source, self.dir / "out", kit=self.kit)
        self.assertEqual(result.path.name, "com.example.basic-1.0.0.qarapp")
        contents = qarapp.read_package(result.path, None)
        self.assertFalse(contents.verification.signed)
        self.assertEqual(sorted(contents.files), ["README.md", "panels/controls/index.html", "profile.json", "scripts/site.js"])
        self.assertEqual(contents.files["scripts/site.js"], b"const region = [1024, 1024];\n")
        profile = contents.profile
        self.assertEqual(profile["FutureField"], {"from": "a newer QAROS"}, "unknown fields are kept")
        self.assertEqual(profile["Scripts"][0]["FutureScriptField"], 7)
        self.assertEqual(profile["Scripts"][2]["Library"], "render@1", "library references stay references")
        self.assertEqual([entry["Path"] for entry in profile["Files"]], ["README.md", "panels/controls/index.html", "scripts/site.js"])
        self.assertIn("__PAGE_REGION_WIDTH__", (self.source / "scripts/site.js").read_text(encoding="utf-8"), "the source is never written")
        self.assertNotIn("Files", json.loads((self.source / "profile.json").read_text(encoding="utf-8")))

    def test_signed_package_is_deterministic_and_verifies(self) -> None:
        signing = packaging.signing_options(self.key_pem)
        first = packaging.build_package(self.source, self.dir / "a" / "first.qarapp", signing=signing, kit=self.kit)
        second = packaging.build_package(self.source, self.dir / "b" / "second.qarapp", signing=signing, kit=self.kit)
        self.assertEqual(first.path.read_bytes(), second.path.read_bytes())
        contents = qarapp.read_package(first.path, [("Quaternar", self.key.public_der)])
        self.assertTrue(contents.verification.signed)
        other = qarapp.keygen()
        with self.assertRaisesRegex(qarapp.PackageError, "not trusted"):
            qarapp.read_package(first.path, [("Quaternar", other.public_der)])

    def test_unpacked_folder_with_version(self) -> None:
        destination = self.dir / "install" / "share" / "basic"
        result = packaging.build_package(self.source, destination, unpacked=True, version="2.3.4", kit=self.kit)
        self.assertEqual(result.path, destination.resolve())
        profile = json.loads((destination / "profile.json").read_text(encoding="utf-8"))
        self.assertEqual(profile["Version"], "2.3.4")
        self.assertNotIn("Signature", profile)
        self.assertFalse((destination / "test").exists(), "authoring files are left out")
        self.assertEqual(qarapp.verify_folder(destination, None).version, "2.3.4")
        # Rebuilding replaces the folder.
        (self.source / "scripts/site.js").write_text("window.changed = 1;\n", encoding="utf-8")
        packaging.build_package(self.source, destination, unpacked=True, kit=self.kit)
        self.assertEqual((destination / "scripts/site.js").read_text(encoding="utf-8"), "window.changed = 1;\n")
        qarapp.verify_folder(destination, None)

    def test_refusals(self) -> None:
        with self.assertRaisesRegex(packaging.BuildError, "does not pass check"):
            broken = make_adaptation(self.dir, "broken", adaptation_profile(Channel="nightly"))
            packaging.build_package(broken, self.dir / "out", kit=self.kit)
        manifest = make_adaptation(self.dir, "manifest", adaptation_profile(Files=[]))
        with self.assertRaisesRegex(packaging.BuildError, "holds 'Files'"):
            packaging.build_package(manifest, self.dir / "out", kit=self.kit)
        occupied = self.dir / "occupied"
        occupied.mkdir()
        (occupied / "keep.txt").write_text("mine", encoding="utf-8")
        with self.assertRaisesRegex(packaging.BuildError, "refusing to replace it"):
            packaging.build_package(self.source, occupied, unpacked=True, kit=self.kit)
        with self.assertRaisesRegex(packaging.BuildError, "'2.0' is not a version"):
            packaging.build_package(self.source, self.dir / "out", version="2.0", kit=self.kit)

    def test_command_line(self) -> None:
        out = self.dir / "cli-out"
        completed = run_cli("package", self.source, "--kit", self.kit_dir, "--out", out, "--sign", self.key_pem, "--public-key", self.public_pem)
        self.assertRuns(completed)
        self.assertIn("signed by Quaternar", completed.stdout)
        package = out / "com.example.basic-1.0.0.qarapp"
        self.assertTrue(qarapp.read_package(package, [("Quaternar", self.key.public_der)]).verification.signed)

        unpacked = self.dir / "cli-unpacked"
        self.assertRuns(run_cli("--kit", self.kit_dir, "package", self.source, "--unpacked", "--out", unpacked, "--version", "0.11.0"))
        self.assertEqual(json.loads((unpacked / "profile.json").read_text(encoding="utf-8"))["Version"], "0.11.0")

        wrong = qarapp.keygen()
        wrong_pub = self.dir / "wrong.pub.pem"
        wrong_pub.write_text(qarapp.public_key_pem(wrong.public_der), encoding="ascii")
        completed = run_cli("package", self.source, "--kit", self.kit_dir, "--out", out, "--sign", self.key_pem, "--public-key", wrong_pub)
        self.assertRuns(completed, 1)
        self.assertIn("does not belong to --public-key", completed.stderr)

    def test_source_in_an_app_adaptation_folder(self) -> None:
        """CMake packages `<app>/adaptation/`: File paths are relative to that folder."""
        app = self.dir / "my-app"
        folder = make_adaptation(app, "adaptation")
        (app / "index.html").write_text("<!doctype html>\n", encoding="utf-8")
        destination = self.dir / "staged" / "my-app"
        self.assertRuns(run_cli("package", folder, "--unpacked", "--out", destination, "--version", "1.2.3", "--kit", self.kit_dir))
        self.assertEqual(sorted(path.name for path in destination.rglob("*") if path.is_file()), ["profile.json", "site.js"])
