// Reference test of the half-resolution depth and alpha packing in webgl-atlas-pack@1 (used by
// threejs-render-frame@1 and raw WebGL handlers). The
// block reductions are the shader's own GLSL (`blockFunctions`), run here as JavaScript: they are
// scalar on purpose, so a few mechanical rewrites turn them into a function. The test then packs
// eye images the way the pack shaders do and unpacks them the way native code does (each sample
// nearest-upsampled over its 2x2 block), and checks that a 1-px line keeps its depth and its
// coverage in every block it crosses.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const renderSource = readFileSync(new URL("../webgl-atlas-pack@1.js", import.meta.url), "utf8");

function shaderBlockFunctions() {
	const match = /const blockFunctions = `([\s\S]*?)`;/.exec(renderSource);
	assert.ok(match, "webgl-atlas-pack@1.js no longer defines blockFunctions");
	const glsl = match[1];
	const javascript = glsl
		.replace(/\b(?:float|int|bool)\s+(\w+)\(([^)]*)\)\s*\{/g, (_, name, parameters) =>
			`function ${name}(${parameters.replace(/\b(?:float|int|bool)\s+/g, "")}) {`)
		.replace(/\b(?:float|int|bool)\s+(\w+)\s*=/g, "let $1 =")
		.replace(/\b(min|max)\(/g, "Math.$1(");
	const build = new Function("eyeSize", "eyeDepth", "eyeAlpha", `${javascript}\nreturn { blockDepth, blockAlpha };`);
	return (image) => build(
		{ x: image.width, y: image.height },
		(x, y) => image.depth[y * image.width + x],
		(x, y) => image.alpha[y * image.width + x],
	);
}

const LineDepth = 0.25;

// A 1-px line, one texel per step along its major axis (no antialiasing, as the eye pass draws),
// counted from the eye's top row like the block functions.
function lineImage(width, height, angleDegrees, offset) {
	const depth = new Float32Array(width * height).fill(1);
	const alpha = new Float32Array(width * height);
	const radians = (angleDegrees * Math.PI) / 180;
	const centreX = width / 2 + offset;
	const centreY = height / 2 + offset * 0.37;
	const halfLength = Math.min(width, height) * 0.4;
	const x0 = centreX - halfLength * Math.cos(radians);
	const y0 = centreY - halfLength * Math.sin(radians);
	const x1 = centreX + halfLength * Math.cos(radians);
	const y1 = centreY + halfLength * Math.sin(radians);
	const steps = Math.ceil(Math.max(Math.abs(x1 - x0), Math.abs(y1 - y0)));
	for (let step = 0; step <= steps; ++step) {
		const x = Math.floor(x0 + ((x1 - x0) * step) / steps);
		const y = Math.floor(y0 + ((y1 - y0) * step) / steps);
		if (x < 0 || y < 0 || x >= width || y >= height) continue;
		depth[y * width + x] = LineDepth;
		alpha[y * width + x] = 1;
	}
	return { width, height, depth, alpha };
}

// What native code reconstructs: each half-resolution sample over its 2x2 block.
function packAndUnpack(image, reduce) {
	const unpacked = new Float32Array(image.width * image.height);
	for (let y = 0; y < image.height; ++y) {
		for (let x = 0; x < image.width; ++x) {
			unpacked[y * image.width + x] = reduce(x >> 1, y >> 1);
		}
	}
	return unpacked;
}

const blockFunctionsFor = shaderBlockFunctions();
const angles = [0, 7, 15, 30, 45, 60, 75, 90, 105, 135, 160];
const offsets = [0, 0.5, 1, 1.5];

test("the pack shaders use the block reductions", () => {
	assert.match(renderSource, /float rawDepth = blockDepth\(column, row\);/);
	assert.match(renderSource, /return blockAlpha\(column, row\);/);
});

test("a block holding any texel of a 1-px line packs the line's depth", () => {
	for (const angle of angles) {
		for (const offset of offsets) {
			const image = lineImage(96, 64, angle, offset);
			const { blockDepth } = blockFunctionsFor(image);
			const depth = packAndUnpack(image, blockDepth);
			for (let index = 0; index < depth.length; ++index) {
				if (image.alpha[index] > 0) {
					assert.equal(depth[index], LineDepth, `angle ${angle}, offset ${offset}, texel ${index}`);
				}
			}
		}
	}
});

test("a block keeps its nearest depth and an empty block stays at the far plane", () => {
	const image = { width: 4, height: 2, depth: Float32Array.from([0.9, 0.4, 1, 1, 0.7, 0.6, 1, 1]), alpha: new Float32Array(8) };
	const { blockDepth } = blockFunctionsFor(image);
	assert.equal(blockDepth(0, 0), Math.fround(0.4));
	assert.equal(blockDepth(1, 0), 1);
});

test("alpha keeps a 1-px line's coverage instead of averaging it away", () => {
	for (const angle of angles) {
		for (const offset of offsets) {
			const image = lineImage(96, 64, angle, offset);
			const { blockAlpha } = blockFunctionsFor(image);
			const alpha = packAndUnpack(image, blockAlpha);
			for (let index = 0; index < alpha.length; ++index) {
				if (image.alpha[index] > 0) {
					assert.equal(alpha[index], 1, `angle ${angle}, offset ${offset}, texel ${index}`);
				}
			}
		}
	}
});

test("alpha of a block below the eye's last row is empty", () => {
	const image = { width: 2, height: 3, depth: new Float32Array(6).fill(1), alpha: new Float32Array(6).fill(1) };
	const { blockAlpha } = blockFunctionsFor(image);
	assert.equal(blockAlpha(0, 1), 1);
	assert.equal(blockAlpha(0, 2), 0);
});
