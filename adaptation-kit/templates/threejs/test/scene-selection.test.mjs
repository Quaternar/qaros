// Tests of this adaptation's own script (scripts/scene-selection.js) in a Node vm, and of the
// profile's script order. Run: qaros-adaptation test <this folder> (or node --test test/).
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const folder = new URL("../", import.meta.url);
const profile = JSON.parse(readFileSync(new URL("profile.json", folder), "utf8"));
const source = readFileSync(new URL("scripts/scene-selection.js", folder), "utf8");

function load(window = {}) {
	vm.runInNewContext(source, { window });
	return window.__qarAdaptationState;
}

function scene(objects) {
	return { traverse: (visit) => { for (let i = 0; i < objects; ++i) visit({}); } };
}

const perspective = { isPerspectiveCamera: true };
const orthographic = { isOrthographicCamera: true };

test("selectScene streams the largest scene drawn with a perspective camera", () => {
	const state = load();
	const main = scene(40);
	const preview = scene(5);
	const hud = scene(100);
	const rendered = [
		{ scene: preview, camera: perspective },
		{ scene: main, camera: perspective },
		{ scene: hud, camera: orthographic },
	];
	assert.equal(state.selectScene(rendered), main);
});

test("selectScene keeps the default when no perspective scene was drawn", () => {
	const state = load();
	assert.equal(state.selectScene([{ scene: scene(3), camera: orthographic }]), null);
});

test("chooseRenderer streams the renderer with the largest canvas", () => {
	const state = load();
	const small = { domElement: { clientWidth: 200, clientHeight: 100 } };
	const large = { domElement: { clientWidth: 1280, clientHeight: 720 } };
	assert.equal(state.chooseRenderer(new Set([small, large])), large);
});

test("the script keeps the state threejs-capture created", () => {
	const window = { __qarAdaptationState: { renderer: "kept" } };
	const state = load(window);
	assert.equal(state.renderer, "kept");
	assert.equal(typeof state.selectScene, "function");
});

test("profile: capture and scene selection before the document, the core before both handlers", () => {
	const ordered = [
		...profile.Scripts.filter((script) => script.Stage === "preDocument"),
		...profile.Scripts.filter((script) => script.Stage === "afterLoad"),
	];
	const names = ordered.map((script) => script.Name);
	assert.ok(names.indexOf("capture-renderer") < names.indexOf("scene-selection"));
	const core = names.indexOf("render-frame-core");
	assert.ok(core >= 0 && core < names.indexOf("render-frame-handler") && core < names.indexOf("render-frame-handler-webgpu"));
	const libraries = profile.Scripts.filter((script) => script.Library).map((script) => script.Library);
	assert.ok(libraries.includes("threejs-render-frame@1"), "WebGLRenderer pages");
	assert.ok(libraries.includes("threejs-webgpu-render-frame@1"), "WebGPURenderer pages");
	for (const script of profile.Scripts) {
		assert.ok(!(script.File && script.Library), `${script.Name}: File and Library are exclusive`);
	}
});
