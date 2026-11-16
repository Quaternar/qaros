"""`check`: the profile schema, library references and `@needs` order, and the pitfall lints."""

from __future__ import annotations

from .helpers import Scratch, adaptation_profile, make_adaptation, make_kit, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402
from qaros_adaptation.profile import app_scale_for, check_adaptation, matches_address  # noqa: E402

SINGLE_TEXEL_PACK = (
    "const pack = `float d = texelFetch(depthTexture, ivec2(column * 2, row * 2), 0).r; // nearest? no\n"
    "float scaled = min(floor(clamp(d, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0);`;\n"
)
NEAREST_PACK = (
    "const pack = `float d = 1.0; for (int i = 0; i < 4; ++i) { d = min(d, texelFetch(depthTexture, p(i), 0).r); }\n"
    "float scaled = min(floor(clamp(d, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0);`;\n"
)
MEAN_ALPHA_PACK = (
    "if (view.dataType === 'alpha') {}\n"
    "const pack = `float a = (texelFetch(c, p0, 0).a + texelFetch(c, p1, 0).a + texelFetch(c, p2, 0).a + texelFetch(c, p3, 0).a) * 0.25;`;\n"
)
MAX_ALPHA_PACK = (
    "if (view.dataType === 'alpha') {}\n"
    "const pack = `float a = 0.0; for (int i = 0; i < 4; ++i) { a = max(a, texelFetch(c, p(i), 0).a); }`;\n"
)


class CheckTests(Scratch):
    def setUp(self) -> None:
        super().setUp()
        self.kit = kitlib.load_kit(make_kit(self.dir))

    def check(self, profile: dict | None = None, files: dict | None = None, ignore=frozenset()):
        folder = make_adaptation(self.dir, f"a{len(list(self.dir.iterdir()))}", profile, files)
        return check_adaptation(folder, self.kit, set(ignore))

    def codes(self, report, level: str) -> list[str]:
        return [finding.code for finding in report.findings if finding.level == level]

    def messages(self, report) -> str:
        return "\n".join(str(finding) for finding in report.findings)

    def test_valid_profile_has_no_findings(self) -> None:
        report = self.check(adaptation_profile(FutureField={"from": "a newer QAROS"}))
        self.assertTrue(report.ok, self.messages(report))
        self.assertEqual(self.codes(report, "warning"), ["unknown-field"], self.messages(report))

    def test_file_and_library_are_exclusive(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][3]["Library"] = "core@1"
        report = self.check(profile)
        self.assertIn("names both a File and a Library", self.messages(report))
        self.assertFalse(report.ok)

    def test_unknown_library_and_major(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][2]["Library"] = "render@3"
        profile["Scripts"].append({"Name": "x", "Stage": "afterLoad", "Library": "nope@1"})
        report = self.check(profile)
        self.assertIn("script 'render' requires `render@3`, this kit provides @1, @2", self.messages(report))
        self.assertIn("requires `nope@1`, which this kit does not provide", self.messages(report))

    def test_legacy_reference_means_major_one(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][2]["Library"] = "render.js"
        report = self.check(profile)
        self.assertTrue(report.ok, self.messages(report))
        self.assertIn("it means `render@1`", self.messages(report))

    def test_needs_must_run_earlier(self) -> None:
        profile = adaptation_profile()
        scripts = profile["Scripts"]
        scripts[1], scripts[2] = scripts[2], scripts[1]  # render before core
        report = self.check(profile)
        self.assertIn("script 'render' (render@1) needs core@1 before it", self.messages(report))
        # A preDocument library listed after the afterLoad ones still runs first.
        profile = adaptation_profile()
        profile["Scripts"].append(profile["Scripts"].pop(0))
        self.assertTrue(self.check(profile).ok)
        profile = adaptation_profile()
        del profile["Scripts"][0]
        self.assertIn("needs capture@1", self.messages(self.check(profile)))

    def test_stage_must_match_the_library(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][0]["Stage"] = "afterLoad"
        self.assertIn("capture@1 runs at preDocument, the entry says afterLoad", self.messages(self.check(profile)))
        profile["Scripts"][0]["Stage"] = "beforeEverything"
        self.assertIn("is not preDocument or afterLoad", self.messages(self.check(profile)))

    def test_schema_fields(self) -> None:
        cases = {
            "missing _sv": ({"_sv": None}, "has no \"_sv\": 3"),
            "newer _sv": ({"_sv": 4}, "newer than this kit"),
            "id": ({"Id": "-bad id"}, "not a valid adaptation id"),
            "version": ({"Version": "1.0"}, "is not a version"),
            "channel": ({"Channel": "beta"}, "'Channel' must be one of public, developer, debug"),
            "app scale": ({"AppScale": 0}, "'AppScale' must be a number above 0"),
            "render scale": ({"RenderScale": 1.5}, "'RenderScale' must be a number in (0, 1]"),
            "render rate high": ({"RenderRate": 240}, "'RenderRate' must be frames per second in [1, 120]"),
            "render rate type": ({"RenderRate": "fast"}, "'RenderRate' must be frames per second in [1, 120]"),
            "by address type": ({"AppScaleByAddress": {"x": 1}}, "must be a list"),
            "by address entry": ({"AppScaleByAddress": [{"AddressPattern": "", "AppScale": "big"}]},
                                 "AppScaleByAddress[0].AddressPattern must be a non-empty string"),
            "region": ({"PageRegion": {"Width": -1, "Height": 1024}}, "PageRegion.Width must be a non-negative integer"),
            "share": ({"ShareBrowserProfile": "yes"}, "'ShareBrowserProfile' must be a bool"),
        }
        for name, (changes, message) in cases.items():
            with self.subTest(name):
                profile = adaptation_profile(**changes)
                if changes.get("_sv", 3) is None:
                    del profile["_sv"]
                report = self.check(profile)
                self.assertFalse(report.ok)
                self.assertIn(message, self.messages(report))
        for name in ("Name", "Scripts", "Id"):
            with self.subTest(f"missing {name}"):
                profile = adaptation_profile()
                del profile[name]
                self.assertIn(f"required field '{name}' is missing", self.messages(self.check(profile)))

    def test_new_fields_accepted(self) -> None:
        profile = adaptation_profile(
            Channel="developer",
            RenderRate=15,
            AppScaleByAddress=[{"AddressPattern": "https://example.com/big/*", "AppScale": 0.01}],
            PageRegion={"Width": 512, "Height": 512},  # a third-party page may use less than 1024
        )
        self.assertTrue(self.check(profile).ok)

    def test_missing_file_and_empty_script(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"].append({"Name": "gone", "Stage": "afterLoad", "File": "scripts/gone.js"})
        profile["Scripts"].append({"Name": "empty", "Stage": "afterLoad", "JavaScript": ""})
        report = self.check(profile)
        self.assertIn("file 'scripts/gone.js' does not exist", self.messages(report))
        self.assertIn("script 'empty' has no File, Library or JavaScript", self.messages(report))

    def test_pinned_copy_provides_its_library(self) -> None:
        profile = adaptation_profile()
        profile["Scripts"][1] = {"Name": "core", "Stage": "afterLoad", "File": "scripts/core@1.js"}
        files = {"scripts/site.js": "x", "scripts/core@1.js": "// @qaros-library core@1.2\n// @stage afterLoad\nwindow.core = 1;\n"}
        self.assertTrue(self.check(profile, files).ok)

    def test_depth_single_texel_lint(self) -> None:
        report = self.check(files={"scripts/site.js": SINGLE_TEXEL_PACK})
        self.assertEqual(self.codes(report, "warning"), ["depth-single-texel"])
        self.assertTrue(report.ok, "lints are warnings")
        self.assertEqual(self.codes(self.check(files={"scripts/site.js": NEAREST_PACK}), "warning"), [])
        self.assertEqual(self.codes(self.check(files={"scripts/site.js": SINGLE_TEXEL_PACK}, ignore={"depth-single-texel"}), "warning"), [])

    def test_alpha_mean_lint(self) -> None:
        self.assertEqual(self.codes(self.check(files={"scripts/site.js": MEAN_ALPHA_PACK}), "warning"), ["alpha-mean"])
        self.assertEqual(self.codes(self.check(files={"scripts/site.js": MAX_ALPHA_PACK}), "warning"), [])

    def test_iframe_lint(self) -> None:
        gallery = adaptation_profile(AddressPattern="https://example.com/examples/*")
        self.assertEqual(self.codes(self.check(gallery), "warning"), ["iframe"])
        handled = {"scripts/site.js": "if (window !== window.top) return;\n"}
        self.assertEqual(self.codes(self.check(gallery, handled), "warning"), [])
        index = adaptation_profile(AddressPattern="https://example.com/")
        self.assertEqual(self.codes(self.check(index), "warning"), ["iframe"])
        self.assertEqual(self.codes(self.check(adaptation_profile(AddressPattern="https://example.com/viewer.html")), "warning"), [])
        comment_only = {"scripts/site.js": "// an iframe would be handled here\n"}
        self.assertEqual(self.codes(self.check(gallery, comment_only), "warning"), ["iframe"])

    def test_page_region_token_lint(self) -> None:
        report = self.check(files={"scripts/site.js": "const w = __PAGE_REGION_WIDTH__;\n"})
        self.assertEqual(self.codes(report, "warning"), ["page-region-token"])
        # A pinned kit copy is written for the substitution `package` makes.
        profile = adaptation_profile()
        profile["Scripts"][0] = {"Name": "capture", "Stage": "preDocument", "File": "scripts/capture@1.js"}
        pinned = {"scripts/site.js": "x", "scripts/capture@1.js": "// @qaros-library capture@1.0\n// @stage preDocument\nconst w = __PAGE_REGION_WIDTH__;\n"}
        self.assertEqual(self.codes(self.check(profile, pinned), "warning"), [])

    def test_command_line(self) -> None:
        kit = self.kit.directory
        good = make_adaptation(self.dir, "good")
        self.assertRuns(run_cli("--kit", kit, "check", good))
        self.assertRuns(run_cli("check", good, "--kit", kit))
        lint = make_adaptation(self.dir, "lint", files={"scripts/site.js": SINGLE_TEXEL_PACK})
        completed = run_cli("--kit", kit, "check", lint)
        self.assertRuns(completed)
        self.assertIn("warning: [depth-single-texel]", completed.stdout)
        self.assertRuns(run_cli("--kit", kit, "check", lint, "--strict"), 1)
        self.assertRuns(run_cli("--kit", kit, "check", lint, "--strict", "--ignore", "depth-single-texel"))
        bad = make_adaptation(self.dir, "bad", adaptation_profile(Channel="nightly"))
        completed = run_cli("--kit", kit, "check", bad)
        self.assertRuns(completed, 1)
        self.assertIn("error: [schema] 'Channel'", completed.stderr)


class AddressTests(Scratch):
    def test_glob_and_app_scale_by_address(self) -> None:
        self.assertTrue(matches_address("https://a.org/*", "https://a.org/x/y.html"))
        self.assertTrue(matches_address("https://a.org/?.html", "https://a.org/b.html"))
        self.assertFalse(matches_address("https://a.org/?.html", "https://a.org/bb.html"))
        self.assertFalse(matches_address("https://a.org/[x].html", "https://a.org/x.html"), "brackets are literal")
        profile = {
            "AppScale": 1,
            "AppScaleByAddress": [
                {"AddressPattern": "https://a.org/small/*", "AppScale": 0.01},
                {"AddressPattern": "https://a.org/*", "AppScale": 0.5},
            ],
        }
        self.assertEqual(app_scale_for(profile, "https://a.org/small/x.html"), 0.01, "first match wins")
        self.assertEqual(app_scale_for(profile, "https://a.org/x.html"), 0.5)
        self.assertEqual(app_scale_for(profile, "https://b.org/x.html"), 1)
        self.assertIsNone(app_scale_for({}, "https://b.org/"))
