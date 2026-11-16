// Reference test of the WGSL block rule shared by webgpu-redirect-presenter@1 and
// threejs-webgpu-render-frame@1: depth = nearest of each 2x2 block, coverage = largest of the
// block, and an empty pixel in a covered block takes the colour of the block's nearest pixel. The
// block functions are the libraries' own WGSL (`blockFunctions`), run here as JavaScript: they are
// scalar on purpose, so a few mechanical rewrites turn them into functions. The test packs eye
// images the way the pack shaders do and unpacks them the way native code does (each sample
// nearest-upsampled over its 2x2 block).
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const libraries = ["webgpu-redirect-presenter@1.js", "threejs-webgpu-render-frame@1.js"];
const sources = Object.fromEntries(libraries.map((name) => [name, readFileSync(new URL(`../${name}`, import.meta.url), "utf8")]));

function wgslBlockFunctions(name) {
	const match = /const blockFunctions = `([\s\S]*?)`;/.exec(sources[name]);
	assert.ok(match, `${name} no longer defines blockFunctions`);
	return match[1];
}

function compile(wgsl) {
	const javascript = wgsl
		.replace(/fn\s+(\w+)\(([^)]*)\)\s*->\s*[\w<>]+\s*\{/g, (_, name, parameters) =>
			`function ${name}(${parameters.replace(/:\s*[\w<>]+/g, "")}) {`)
		.replace(/\b(?:var|let)\s+(\w+)\s*=/g, "let $1 =")
		.replace(/vec2<i32>\(/g, "vec2(")
		.replace(/\b(min|max)\(/g, "Math.$1(");
	const build = new Function("eyeWidth", "eyeHeight", "eyeDepth", "eyeCoverage", "vec2",
		`${javascript}\nreturn { blockDepth, blockCoverage, fillTexel };`);
	return (image) => build(
		() => image.width,
		() => image.height,
		(x, y) => image.depth[y * image.width + x],
		(x, y) => (image.depth[y * image.width + x] < 1 ? 1 : 0),
		(x, y) => ({ x, y }),
	);
}

const LineDepth = 0.25;

// A 1-px line, one pixel per step along its major axis, counted from the eye's top row. The
// line's colour index is its pixel index + 1; empty pixels have colour 0 (the clear colour).
function lineImage(width, height, angleDegrees, offset) {
	const depth = new Float32Array(width * height).fill(1);
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
		depth[y * width + x] = LineDepth + (x + y) * 1e-4;
	}
	return { width, height, depth };
}

const angles = [0, 7, 15, 30, 45, 60, 75, 90, 105, 135, 160];
const offsets = [0, 0.5, 1, 1.5];
const images = angles.flatMap((angle) => offsets.map((offset) => ({ angle, offset, image: lineImage(96, 64, angle, offset) })));

test("both WebGPU packers carry the same block functions", () => {
	assert.equal(wgslBlockFunctions(libraries[0]), wgslBlockFunctions(libraries[1]));
});

test("the WGSL passes use the block functions", () => {
	for (const name of libraries) {
		assert.match(sources[name], /blockDepth\(column, row\)/, name);
		assert.match(sources[name], /blockCoverage\([^)]*row \+ 2\)/, name);
		assert.match(sources[name], /fillTexel\(/, name);
	}
	// The presenter fills colour only where a depth view is packed; otherwise it copies.
	assert.match(sources[libraries[0]], /dataType === "depth"\)\) fillColor\(/);
});

for (const name of libraries) {
	const functionsFor = compile(wgslBlockFunctions(name));

	test(`${name}: every pixel of a 1-px line keeps depth and coverage after unpacking`, () => {
		for (const { angle, offset, image } of images) {
			const { blockDepth, blockCoverage } = functionsFor(image);
			for (let y = 0; y < image.height; ++y) {
				for (let x = 0; x < image.width; ++x) {
					if (image.depth[y * image.width + x] >= 1) continue;
					const where = `angle ${angle}, offset ${offset}, pixel ${x},${y}`;
					assert.ok(blockDepth(x >> 1, y >> 1) < 1, where);
					assert.equal(blockCoverage(x >> 1, y >> 1), 1, where);
				}
			}
		}
	});

	test(`${name}: the filled colour, depth and coverage describe the same block`, () => {
		for (const { angle, offset, image } of images) {
			const { blockDepth, blockCoverage, fillTexel } = functionsFor(image);
			for (let y = 0; y < image.height; ++y) {
				for (let x = 0; x < image.width; ++x) {
					const where = `angle ${angle}, offset ${offset}, pixel ${x},${y}`;
					const depth = blockDepth(x >> 1, y >> 1);
					const texel = fillTexel(x, y);
					const sourceDepth = image.depth[texel.y * image.width + texel.x];
					assert.equal(Math.abs(texel.x - x) <= 1 && Math.abs(texel.y - y) <= 1, true, where);
					if (depth < 1) {
						// A covered block: its every pixel shows a covered pixel's colour, so no
						// clear-colour pixel travels at the line's depth.
						assert.ok(sourceDepth < 1, `clear colour at the line's depth, ${where}`);
						assert.equal(blockCoverage(x >> 1, y >> 1), 1, where);
					} else {
						assert.deepEqual(texel, { x, y }, where);
						assert.equal(blockCoverage(x >> 1, y >> 1), 0, where);
					}
				}
			}
		}
	});

	test(`${name}: a block keeps its nearest depth and colour, an empty block the far plane`, () => {
		const image = { width: 4, height: 2, depth: Float32Array.from([0.9, 0.4, 1, 1, 1, 0.6, 1, 1]) };
		const { blockDepth, fillTexel } = functionsFor(image);
		assert.equal(blockDepth(0, 0), Math.fround(0.4));
		assert.equal(blockDepth(1, 0), 1);
		assert.deepEqual(fillTexel(0, 1), { x: 1, y: 0 }, "an empty pixel takes the nearest pixel's colour");
		assert.deepEqual(fillTexel(0, 0), { x: 0, y: 0 }, "a covered pixel keeps its own colour");
	});

	test(`${name}: coverage of a block below the eye's last row is empty`, () => {
		const image = { width: 2, height: 3, depth: new Float32Array(6).fill(0.5) };
		const { blockCoverage } = functionsFor(image);
		assert.equal(blockCoverage(0, 1), 1);
		assert.equal(blockCoverage(0, 2), 0);
	});
}
