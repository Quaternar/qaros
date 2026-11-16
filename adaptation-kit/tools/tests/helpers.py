"""Shared fixtures of the qaros-adaptation tests: a small synthetic kit, adaptations, and a runner of
the command line."""

from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent.parent
KIT = TOOLS.parent
SCRIPT = TOOLS / "qaros-adaptation.py"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

NODE = shutil.which("node")

LIBRARIES = {
    "core@1.js": "// @qaros-library core@1.2\n// @stage afterLoad\n//\n// Core.\nwindow.core = 1;\n",
    "capture@1.js": "// @qaros-library capture@1.0\n// @stage preDocument\n(() => { const w = __PAGE_REGION_WIDTH__; })();\n",
    "render@1.js": "// @qaros-library render@1.4\n// @stage afterLoad\n// @needs core@1 capture@1\nwindow.render = 1;\n",
    "render@2.js": "// @qaros-library render@2.0\n// @stage afterLoad\n// @needs core@1\nwindow.render = 2;\n",
}

LIBRARY_TESTS = {
    "render.test.mjs": (
        'import test from "node:test";\nimport assert from "node:assert/strict";\n'
        'import { readFileSync } from "node:fs";\n'
        'test("render@1 loads", () => {\n'
        '  const text = readFileSync(new URL("../render@1.js", import.meta.url), "utf8");\n'
        '  assert.match(text, /render@1/);\n});\n'
    ),
    "unused.test.mjs": 'import test from "node:test";\ntest("other@1 fails", () => { throw new Error("must not run"); });\n',
}

TEMPLATE_PROFILE = {
    "_sv": 3,
    "Id": "{{ID}}",
    "Name": "{{NAME}}",
    "DisplayName": "{{DISPLAY_NAME}}",
    "Version": "1.0.0",
    "AddressPattern": "{{ADDRESS_PATTERN}}",
    "Engine": "basic-engine",
    "PageRegion": {"Width": 1024, "Height": 1024},
    "ShareBrowserProfile": True,
    "FutureField": {"kept": True},
    "Scripts": [
        {"Name": "capture", "Stage": "preDocument", "Library": "capture@1"},
        {"Name": "core", "Stage": "afterLoad", "Library": "core@1"},
        {"Name": "render", "Stage": "afterLoad", "Library": "render@1"},
        {"Name": "site", "Stage": "afterLoad", "File": "scripts/site.js"},
    ],
}


def write_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def make_kit(root: Path) -> Path:
    kit = root / "kit"
    for name, text in LIBRARIES.items():
        (kit / "lib").mkdir(parents=True, exist_ok=True)
        (kit / "lib" / name).write_text(text, encoding="utf-8")
    for name, text in LIBRARY_TESTS.items():
        (kit / "lib" / "tests").mkdir(parents=True, exist_ok=True)
        (kit / "lib" / "tests" / name).write_text(text, encoding="utf-8")
    template = kit / "templates" / "basic"
    write_json(template / "profile.json", TEMPLATE_PROFILE)
    (template / "scripts").mkdir(parents=True)
    (template / "scripts" / "site.js").write_text("// {{DISPLAY_NAME}} site script\nwindow.site = 1;\n", encoding="utf-8")
    (template / "README.md").write_text("# {{DISPLAY_NAME}}\n", encoding="utf-8")
    (template / "test").mkdir()
    (template / "test" / "site.test.mjs").write_text(
        'import test from "node:test";\nimport assert from "node:assert/strict";\n'
        'import { readFileSync } from "node:fs";\n'
        'test("site script", () => assert.match(readFileSync(new URL("../scripts/site.js", import.meta.url), "utf8"), /window.site/));\n',
        encoding="utf-8",
    )
    return kit


def adaptation_profile(**changes) -> dict:
    profile = json.loads(json.dumps(TEMPLATE_PROFILE))
    del profile["FutureField"]
    profile.update(Id="com.example.basic", Name="basic", DisplayName="Basic", AddressPattern="https://example.com/app.html")
    profile.update(changes)
    return profile


def make_adaptation(root: Path, name: str = "adaptation", profile: dict | None = None, files: dict | None = None) -> Path:
    folder = root / name
    write_json(folder / "profile.json", profile if profile is not None else adaptation_profile())
    files = {"scripts/site.js": "window.site = 1;\n"} if files is None else files
    for relative, text in files.items():
        (folder / relative).parent.mkdir(parents=True, exist_ok=True)
        (folder / relative).write_text(text, encoding="utf-8")
    return folder


def run_cli(*args: str, cwd: Path | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, "-I", "-X", "utf8", str(SCRIPT), *map(str, args)],
        capture_output=True, text=True, encoding="utf-8", cwd=cwd, input="", check=False,
    )


class Scratch(unittest.TestCase):
    def setUp(self) -> None:
        self.dir = Path(tempfile.mkdtemp(prefix="qaros-adaptation-test-"))

    def tearDown(self) -> None:
        shutil.rmtree(self.dir, ignore_errors=True)

    def assertRuns(self, completed: subprocess.CompletedProcess, code: int = 0) -> None:
        self.assertEqual(completed.returncode, code, f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}")
