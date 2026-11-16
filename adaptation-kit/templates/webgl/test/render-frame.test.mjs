// Tests of this adaptation's render-frame handler (scripts/render-frame.js) in a Node vm: camera
// math, the offscreen layout, and one request end to end through the kit's render-frame-core@1
// (when run by `qaros-adaptation test`, which sets QAROS_ADAPTATION_KIT) or a stand-in core.
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";

const folder = new URL("../", import.meta.url);
const profile = JSON.parse(readFileSync(new URL("profile.json", folder), "utf8"));
const source = readFileSync(new URL("scripts/render-frame.js", folder), "utf8");
const kit = process.env.QAROS_ADAPTATION_KIT;
const coreFile = kit ? path.join(kit, "lib", "render-frame-core@1.js") : null;

// A WebGL2 context that records every call: constants are numbers, methods record their arguments.
function recordingGl() {
	const calls = [];
	const gl = new Proxy({}, {
		get(target, name) {
			if (typeof name !== "string") return undefined;
			if (/^[A-Z0-9_]+$/.test(name)) return name.length;
			return (...args) => { calls.push([name, ...args]); };
		},
	});
	return { gl, calls };
}

function harness({ realCore }) {
	const listeners = new Map();
	const answers = [];
	const window = {
		qaros: {
			completeRenderFrame: (id) => answers.push(["complete", id]),
			failRenderFrame: (id, error) => answers.push(["fail", id, String(error)]),
		},
		addEventListener: (type, listener) => listeners.set(type, [...(listeners.get(type) || []), listener]),
		removeEventListener() {},
		dispatch: (type, detail) => (listeners.get(type) || []).forEach((listener) => listener({ type, detail })),
	};
	const context = { window, requestAnimationFrame: (callback) => setTimeout(() => callback(performance.now()), 0), performance, setTimeout, BigInt, console };
	window.requestAnimationFrame = context.requestAnimationFrame;
	if (realCore) {
		vm.runInNewContext(readFileSync(coreFile, "utf8").replaceAll("__PAGE_REGION_WIDTH__", "1024").replaceAll("__PAGE_REGION_HEIGHT__", "1024"), context);
	} else {
		window.__qarAdaptationCore = {
			createRenderFrameLoop: (work) => ({ update: (detail) => answers.push(["work", work(detail)]), stop() {}, pause() {} }),
			writeRenderFrameTag: () => true,
		};
	}
	const packed = [];
	window.__qarWebglAtlasPack = {
		createAtlasPacker: () => ({
			ensurePass: (gl, width, height) => ({ framebuffer: "pass", width, height }),
			copyEyesToAtlas: (gl, pass, atlasHeight, eyes) => packed.push({ pass, atlasHeight, eyes }),
		}),
	};
	const recording = recordingGl();
	window.__qarAdaptationState = { gl: recording.gl, canvas: {} };
	const drawn = [];
	window.qarDrawEye = (gl, camera, eye) => drawn.push({ camera, eye: eye.name });
	vm.runInNewContext(source, context);
	return { window, answers, packed, drawn, calls: recording.calls, state: window.__qarAdaptationState };
}

const fov = { angleLeft: -Math.PI / 4, angleRight: Math.PI / 4, angleUp: Math.PI / 4, angleDown: -Math.PI / 4 };
function camera(x) {
	return { pose: { position: [x, 1.6, 0], orientation: [0, 0, 0, 1] }, fov, nearFar: { near: 0.1, far: 100 }, depthConvention: "regular" };
}
function request(continuous = false) {
	const view = (eye, dataType, x) => ({ eye, dataType, logicalWidth: 64, logicalHeight: 32, rect: { x, y: 1024, width: 64, height: 32 } });
	return {
		requestId: 7, continuous, atlas: { width: 2048, height: 2048 }, tag: { x: 0, y: 2047, width: 2, height: 1 },
		targets: [{
			id: "user-1",
			cameras: { left: camera(-0.03), right: camera(0.03) },
			views: [view("left", "color", 0), view("right", "color", 64), view("left", "depth", 128), view("right", "depth", 192)],
		}],
	};
}

test("projection: a symmetric 90-degree field of view is the standard WebGL frustum", () => {
	const { state } = harness({ realCore: false });
	const m = state.renderFrame.projection(fov, 0.1, 100);
	assert.ok(Math.abs(m[0] - 1) < 1e-6 && Math.abs(m[5] - 1) < 1e-6);
	assert.ok(Math.abs(m[8]) < 1e-6 && Math.abs(m[9]) < 1e-6);
	assert.ok(Math.abs(m[10] + 100.1 / 99.9) < 1e-5);
	assert.equal(m[11], -1);
});

test("viewMatrix moves the world opposite to the eye", () => {
	const { state } = harness({ realCore: false });
	const m = state.renderFrame.viewMatrix({ position: [1, 2, 3], orientation: [0, 0, 0, 1] });
	assert.equal(Array.from(m.slice(12, 15)).join(), "-1,-2,-3");
	// 90 degrees about Y: the camera looks along world -X, so world +X is behind it (camera +Z).
	const s = Math.SQRT1_2;
	const r = state.renderFrame.viewMatrix({ position: [0, 0, 0], orientation: [0, s, 0, s] });
	const worldX = [r[0], r[1], r[2]];
	assert.ok(Math.abs(worldX[2] - 1) < 1e-6, `world +X maps to camera +Z, got ${worldX}`);
});

test("collectEyes lays each user out in one row of the offscreen pass", () => {
	const { state } = harness({ realCore: false });
	const { eyes, width, height } = state.renderFrame.collectEyes(request().targets);
	assert.equal(JSON.stringify(eyes.map((eye) => [eye.name, eye.camera.viewport.x, eye.camera.viewport.y])), JSON.stringify([["left", 0, 0], ["right", 64, 0]]));
	assert.equal(width, 128);
	assert.equal(height, 32);
	assert.ok(eyes[0].depth && eyes[1].depth, "every eye carries its depth view");
});

test("a request draws every eye with its own camera, packs and tags", () => {
	const run = harness({ realCore: false });
	run.window.dispatch("qaros-render-frame", request(true));
	assert.equal(JSON.stringify(run.answers), JSON.stringify([["work", true]]));
	assert.equal(run.drawn.map((entry) => entry.eye).join(), "left,right");
	assert.notEqual(run.drawn[0].camera.view, run.drawn[1].camera.view, "fresh matrices per eye");
	assert.ok(Math.abs(run.drawn[0].camera.view[12] - 0.03) < 1e-6, "the left eye sits at x = -0.03");
	assert.equal(run.packed.length, 1);
	assert.equal(run.packed[0].atlasHeight, 2048);
	assert.equal(run.state.suspendConfinement, false, "confinement restored");
});

test("with the kit's render-frame-core@1, a request is acknowledged", { skip: !coreFile || !existsSync(coreFile) }, async () => {
	const run = harness({ realCore: true });
	run.window.dispatch("qaros-render-frame", request(false));
	await new Promise((resolve) => setTimeout(resolve, 50));
	assert.equal(JSON.stringify(run.answers.map(([kind, id]) => [kind, id])), JSON.stringify([["complete", 7]]));
	assert.equal(run.drawn.length, 2);
});

test("profile: capture before the document; core and atlas pack before this handler", () => {
	const names = [
		...profile.Scripts.filter((script) => script.Stage === "preDocument"),
		...profile.Scripts.filter((script) => script.Stage === "afterLoad"),
	].map((script) => script.Library || script.File);
	const handler = names.indexOf("scripts/render-frame.js");
	assert.ok(names.indexOf("webgl-capture-context@1") < names.indexOf("render-frame-core@1"));
	assert.ok(names.indexOf("render-frame-core@1") < handler && names.indexOf("webgl-atlas-pack@1") < handler);
});
