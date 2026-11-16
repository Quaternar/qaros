"""`test`: the adaptation's Node tests plus the kit tests of the libraries it uses."""

from __future__ import annotations

import unittest

from .helpers import NODE, Scratch, make_adaptation, make_kit, run_cli

from qaros_adaptation import kit as kitlib  # noqa: E402
from qaros_adaptation.cli import kit_tests_for  # noqa: E402

PASSING = 'import test from "node:test";\ntest("ok", () => {});\n'
FAILING = 'import test from "node:test";\ntest("broken", () => { throw new Error("broken"); });\n'
KIT_VARIABLE = (
    'import test from "node:test";\nimport assert from "node:assert/strict";\n'
    'test("kit", () => assert.ok(process.env.QAROS_ADAPTATION_KIT));\n'
)


class KitTestSelection(Scratch):
    def test_only_tests_naming_a_used_library(self) -> None:
        kit = kitlib.load_kit(make_kit(self.dir))
        self.assertEqual([path.name for path in kit_tests_for(kit, {"render"})], ["render.test.mjs"])
        self.assertEqual(kit_tests_for(kit, {"core"}), [])
        self.assertEqual(kit_tests_for(kit, set()), [])


@unittest.skipIf(NODE is None, "Node.js is not on PATH")
class TestCommand(Scratch):
    def setUp(self) -> None:
        super().setUp()
        self.kit = make_kit(self.dir)

    def test_runs_own_and_kit_tests(self) -> None:
        folder = make_adaptation(self.dir, "a", files={"scripts/site.js": "x", "test/own.test.mjs": PASSING, "test/kit.test.mjs": KIT_VARIABLE})
        completed = run_cli("--kit", self.kit, "test", folder)
        self.assertRuns(completed)
        self.assertIn("2 own and 1 kit test files", completed.stdout)

    def test_a_failing_test_fails_the_command(self) -> None:
        folder = make_adaptation(self.dir, "b", files={"scripts/site.js": "x", "test/own.test.mjs": FAILING})
        self.assertNotEqual(run_cli("--kit", self.kit, "test", folder).returncode, 0)
        self.assertNotEqual(run_cli("--kit", self.kit, "test", folder, "--no-kit-tests").returncode, 0)

    def test_no_tests(self) -> None:
        folder = make_adaptation(self.dir, "c")
        completed = run_cli("--kit", self.kit, "test", folder, "--no-kit-tests")
        self.assertRuns(completed)
        self.assertIn("no tests", completed.stdout)

    def test_missing_node(self) -> None:
        folder = make_adaptation(self.dir, "d", files={"scripts/site.js": "x", "test/own.test.mjs": PASSING})
        completed = run_cli("--kit", self.kit, "test", folder, "--node", str(self.dir / "no-node-here.exe"))
        self.assertRuns(completed, 1)
        self.assertIn("Node.js was not found", completed.stderr)
