// Tests of this adaptation's bridge (scripts/app-bridge.js) in a Node vm: it publishes the page's
// WebGPU entry points under the names webgpu-render-frame-handler@1 reads, live, and reports the
// missing ones by name.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const folder = new URL("../", import.meta.url);
const profile = JSON.parse(readFileSync(new URL("profile.json", folder), "utf8"));
const source = readFileSync(new URL("scripts/app-bridge.js", folder), "utf8");

function run(window, frames = []) {
	const errors = [];
	const context = {
		window,
		requestAnimationFrame: (callback) => frames.push(callback),
		console: { error: (message) => errors.push(message) },
	};
	vm.runInNewContext(source, context);
	return { errors, frames };
}

function completeApp() {
	return {
		device: {}, format: "bgra8unorm", camera: { view: new Float32Array(16), projection: new Float32Array(16) },
		draw() {}, colorPipeline: {}, depthOnlyPipeline: {}, ensureDepthTexture() {},
	};
}

test("publishes the page's entry points live, under the handler's names", () => {
	const window = {};
	run(window);
	assert.equal(window.gpuDraw, undefined, "nothing before the page has started");
	window.app = completeApp();
	assert.equal(window.gpuDraw, window.app.draw);
	assert.equal(window.gpuCamera, window.app.camera);
	assert.equal(window.gpuCanvasFormat, "bgra8unorm");
	assert.equal(window.__qarAdaptationState.bridge.missing().length, 0);
});

test("leaves a name the page publishes itself alone", () => {
	const own = () => "own";
	const window = { gpuDraw: own };
	run(window);
	window.app = completeApp();
	assert.equal(window.gpuDraw, own);
});

test("reports the missing entry points by name once the page had time to start", () => {
	const window = { app: { ...completeApp(), depthOnlyPipeline: undefined, format: 42 } };
	const { errors, frames } = run(window);
	assert.equal(window.__qarAdaptationState.bridge.missing().join(), "gpuCanvasFormat,gpuDepthOnlyPipeline");
	while (frames.length > 0) frames.shift()();
	assert.equal(errors.length, 1);
	assert.match(errors[0], /^QAR adaptation: .*gpuCanvasFormat, gpuDepthOnlyPipeline/);
});

test("profile: capture before the document, core and bridge before the handler", () => {
	const names = [
		...profile.Scripts.filter((script) => script.Stage === "preDocument"),
		...profile.Scripts.filter((script) => script.Stage === "afterLoad"),
	].map((script) => script.Library || script.File);
	const handler = names.indexOf("webgpu-render-frame-handler@1");
	assert.ok(names.indexOf("webgpu-capture-context@1") === 0);
	assert.ok(names.indexOf("render-frame-core@1") < handler);
	assert.ok(names.indexOf("scripts/app-bridge.js") < handler);
});
