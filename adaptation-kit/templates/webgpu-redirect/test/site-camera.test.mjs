// Tests of this adaptation's site script (scripts/site-camera.js) in a Node vm, with a stand-in
// GPUDevice: each eye gets its own camera buffer, filled with its matrices in the page's layout.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const folder = new URL("../", import.meta.url);
const profile = JSON.parse(readFileSync(new URL("profile.json", folder), "utf8"));
const source = readFileSync(new URL("scripts/site-camera.js", folder), "utf8");
const label = /const CAMERA_BUFFER = "([^"]+)"/.exec(source)[1];

function setup(buffers = new Map([[label, { label, size: 128 }]])) {
	const window = { __qarAdaptationState: { buffersByLabel: buffers } };
	vm.runInNewContext(source, { window, GPUBufferUsage: { UNIFORM: 64, COPY_DST: 8 } });
	return window.__qarAdaptationState;
}

function device() {
	const created = [];
	const writes = [];
	return {
		created, writes,
		createBuffer: (descriptor) => { const buffer = { ...descriptor }; created.push(buffer); return buffer; },
		queue: { writeBuffer: (buffer, offset, data) => writes.push({ buffer, offset, data: Array.from(data) }) },
	};
}

function eye(key, gpu, base) {
	const matrix = (value) => Float32Array.from({ length: 16 }, (_, index) => value + index);
	return { key, device: gpu, cache: {}, viewProjection: matrix(base), inverseViewProjection: matrix(base + 100) };
}

test("each eye gets its own buffer, holding its viewProjection then its inverse", () => {
	const S = setup();
	const gpu = device();
	const left = eye("u1-left", gpu, 0);
	const right = eye("u1-right", gpu, 1000);
	const [[pageBuffer, leftBuffer]] = S.site.eyeBuffers(left);
	const [[, rightBuffer]] = S.site.eyeBuffers(right);
	assert.equal(pageBuffer.label, label);
	assert.notEqual(leftBuffer, rightBuffer);
	assert.equal(leftBuffer.size, 128);
	assert.equal(leftBuffer.usage, 64 | 8);
	assert.equal(gpu.writes[0].data.length, 32);
	assert.equal(gpu.writes[0].data[0], 0);
	assert.equal(gpu.writes[0].data[16], 100);
	assert.equal(gpu.writes[1].data[0], 1000);
});

test("an eye's buffer is created once and rewritten every tick", () => {
	const S = setup();
	const gpu = device();
	const left = eye("u1-left", gpu, 0);
	S.site.eyeBuffers(left);
	S.site.eyeBuffers(left);
	assert.equal(gpu.created.length, 1);
	assert.equal(gpu.writes.length, 2);
});

test("a page buffer that was not captured fails with the label to fix", () => {
	const S = setup(new Map());
	assert.throws(() => S.site.eyeBuffers(eye("u1-left", device(), 0)), /QAR adaptation: .*CAMERA_BUFFER/);
});

test("profile: redirect capture before the document, core, presenter, then this script", () => {
	const names = [
		...profile.Scripts.filter((script) => script.Stage === "preDocument"),
		...profile.Scripts.filter((script) => script.Stage === "afterLoad"),
	].map((script) => script.Library || script.File);
	assert.equal(names.join(), "webgpu-redirect-capture@1,render-frame-core@1,webgpu-redirect-presenter@1,scripts/site-camera.js");
	assert.equal(profile.ContinuousRendering, true, "the presenter redraws the whole atlas every tick");
});
