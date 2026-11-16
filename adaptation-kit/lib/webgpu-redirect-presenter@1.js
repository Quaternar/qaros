// @qaros-library webgpu-redirect-presenter@1.1
// @stage afterLoad
// @needs render-frame-core@1 webgpu-redirect-capture@1
//
// Presenter for a WebGPU page whose own loop was redirected offscreen by webgpu-redirect-capture@1.
// It is the only caller of the real getCurrentTexture() and, on every animation tick:
//   1. replays the app's last recorded frame once per eye: every render pass, in order, into
//      per-eye copies of the app's attachments (G-buffers, depth), the pass that drew into the app
//      texture into the eye's colour target, with the site's per-eye camera buffers substituted in
//      every bind group that referenced the app's camera buffers;
//   2. writes each eye's colour into its atlas rect, packs depth from the eye's copy of the last
//      depth attachment (24-bit, half resolution, two tiles, each sample the nearest of its 2x2
//      block, saturated at the far plane) and, under `TransportAlpha`, packs coverage alpha (the
//      largest of the block: 1 where geometry wrote depth, 0 where it stayed clear). With a depth
//      view an empty pixel next to geometry takes the colour of its block's nearest pixel (since
//      1.1), so the clear colour never reaches users at an edge's depth;
//   3. copies the app's own frame into the page region only (top-left), so the page preview stays
//      the original page and never spills over a user's view;
//   4. writes the request tag with exact bytes (queue.writeTexture, after the submit).
// Compute passes are not replayed: the app's own loop keeps running them once per app frame (light
// or particle simulation), so every eye sees the same simulation state.
// Pacing is continuous (profile `ContinuousRendering: true`): the whole atlas is redrawn at most
// `RenderRate` times per second (30 when the profile sets none).
//
// Site-specific code supplies only the camera, in a later afterLoad script:
//   window.__qarAdaptationState.site = {
//     beginTick() {}            // optional: once per tick, before any eye
//     eyeBuffers(eye) { ... }   // required: [[appBuffer, eyeBuffer], ...]; the site writes this
//                               // eye's camera into its own eyeBuffer (one buffer per eye: a buffer
//                               // written twice before one submit gives every eye the last value)
//     nearFarScale: 1,          // optional: app units per protocol scene unit
//   };
// `eye` = { key, device, camera (protocol camera), view, projection, viewProjection,
//           inverseViewProjection (column-major Float32Array, WebGPU [0,1] depth, "regular"),
//           width, height, cache (per-eye object for the site's own buffers) }.
(() => {
	const core = window.__qarAdaptationCore;
	const S = window.__qarAdaptationState;
	let latest = null, acknowledged = null, packPipelines = null;
	const eyes = new Map();
	S.stats = { ticks: 0, drawn: 0, errors: 0, lastError: "", requests: 0 };

	function multiply(a, b) { // column-major a*b
		const out = new Float32Array(16);
		for (let col = 0; col < 4; ++col)
			for (let row = 0; row < 4; ++row) {
				let sum = 0;
				for (let k = 0; k < 4; ++k) sum += a[k * 4 + row] * b[col * 4 + k];
				out[col * 4 + row] = sum;
			}
		return out;
	}
	function invert(m) { // general 4x4 inverse, column-major
		const inv = new Float32Array(16);
		inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15] + m[9] * m[7] * m[14] + m[13] * m[6] * m[11] - m[13] * m[7] * m[10];
		inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15] - m[8] * m[7] * m[14] - m[12] * m[6] * m[11] + m[12] * m[7] * m[10];
		inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15] + m[8] * m[7] * m[13] + m[12] * m[5] * m[11] - m[12] * m[7] * m[9];
		inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14] - m[8] * m[6] * m[13] - m[12] * m[5] * m[10] + m[12] * m[6] * m[9];
		inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15] - m[9] * m[3] * m[14] - m[13] * m[2] * m[11] + m[13] * m[3] * m[10];
		inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15] + m[8] * m[3] * m[14] + m[12] * m[2] * m[11] - m[12] * m[3] * m[10];
		inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15] - m[8] * m[3] * m[13] - m[12] * m[1] * m[11] + m[12] * m[3] * m[9];
		inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14] + m[8] * m[2] * m[13] + m[12] * m[1] * m[10] - m[12] * m[2] * m[9];
		inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15] + m[5] * m[3] * m[14] + m[13] * m[2] * m[7] - m[13] * m[3] * m[6];
		inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15] - m[4] * m[3] * m[14] - m[12] * m[2] * m[7] + m[12] * m[3] * m[6];
		inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15] + m[4] * m[3] * m[13] + m[12] * m[1] * m[7] - m[12] * m[3] * m[5];
		inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14] - m[4] * m[2] * m[13] - m[12] * m[1] * m[6] + m[12] * m[2] * m[5];
		inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11] - m[5] * m[3] * m[10] - m[9] * m[2] * m[7] + m[9] * m[3] * m[6];
		inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11] + m[4] * m[3] * m[10] + m[8] * m[2] * m[7] - m[8] * m[3] * m[6];
		inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11] - m[4] * m[3] * m[9] - m[8] * m[1] * m[7] + m[8] * m[3] * m[5];
		inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10] + m[4] * m[2] * m[9] + m[8] * m[1] * m[6] - m[8] * m[2] * m[5];
		const det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
		for (let i = 0; i < 16; ++i) inv[i] /= det;
		return inv;
	}
	function viewFromPose(position, q, translationScale) {
		const [x, y, z, w] = q;
		const out = new Float32Array(16); // view = [R^T | -R^T p]
		out[0] = 1 - 2 * (y * y + z * z); out[4] = 2 * (x * y + w * z); out[8] = 2 * (x * z - w * y);
		out[1] = 2 * (x * y - w * z); out[5] = 1 - 2 * (x * x + z * z); out[9] = 2 * (y * z + w * x);
		out[2] = 2 * (x * z + w * y); out[6] = 2 * (y * z - w * x); out[10] = 1 - 2 * (x * x + y * y);
		out[15] = 1;
		const px = position[0] * translationScale, py = position[1] * translationScale, pz = position[2] * translationScale;
		out[12] = -(out[0] * px + out[4] * py + out[8] * pz);
		out[13] = -(out[1] * px + out[5] * py + out[9] * pz);
		out[14] = -(out[2] * px + out[6] * py + out[10] * pz);
		return out;
	}
	// WebGPU clip z in [0,1] with the protocol's near/far: depthConvention "regular". Scaling near,
	// far and the eye position by the same factor leaves window depth unchanged.
	function projection(fov, near, far) {
		const l = Math.tan(fov.angleLeft) * near, r = Math.tan(fov.angleRight) * near;
		const t = Math.tan(fov.angleUp) * near, b = Math.tan(fov.angleDown) * near;
		return new Float32Array([2 * near / (r - l), 0, 0, 0, 0, 2 * near / (t - b), 0, 0,
			(r + l) / (r - l), (t + b) / (t - b), far / (near - far), -1, 0, 0, far * near / (near - far), 0]);
	}

	const fullScreenVertex = `
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
	var p = array<vec2<f32>, 3>(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
	return vec4<f32>(p[i], 0.0, 1.0);
}`;
	// The block rule of every packer in the kit (webgl-atlas-pack@1 has the GLSL twin): depth and
	// alpha travel at half resolution, one sample per 2x2 block of eye pixels, and native code
	// nearest-upsamples each sample back over its block. A sample keeps what any of its four pixels
	// holds, not one fixed pixel: a feature thinner than the block (a 1-px line) misses a fixed
	// pixel in about half the blocks, and its colour then reaches the mixer with no depth and is
	// dropped. Depth is "regular" here (cleared to 1, the far plane), so the nearest is the smallest;
	// coverage is the largest of the block. `fillTexel` is the colour rule that goes with them: an
	// empty pixel (depth at the clear value) in a block that holds geometry shows the colour of the
	// block's nearest pixel, whose depth and coverage the block packs, so the clear colour never
	// reaches users at an edge's depth. Each shader defines `eyeDepth`, `eyeCoverage`, `eyeWidth`
	// and `eyeHeight` (pixels counted from the eye's top-left). Kept scalar so
	// tests/webgpu-depth-pack.test.mjs can run them as JavaScript.
	const blockFunctions = `
fn blockDepth(column: i32, row: i32) -> f32 {
	var nearest = 1.0;
	for (var dy = 0; dy < 2; dy++) {
		for (var dx = 0; dx < 2; dx++) {
			let x = min(column * 2 + dx, eyeWidth() - 1);
			let y = min(row * 2 + dy, eyeHeight() - 1);
			nearest = min(nearest, eyeDepth(x, y));
		}
	}
	return nearest;
}
fn blockCoverage(column: i32, row: i32) -> f32 {
	if (row * 2 >= eyeHeight()) { return 0.0; }
	var coverage = 0.0;
	for (var dy = 0; dy < 2; dy++) {
		for (var dx = 0; dx < 2; dx++) {
			let x = min(column * 2 + dx, eyeWidth() - 1);
			let y = min(row * 2 + dy, eyeHeight() - 1);
			coverage = max(coverage, eyeCoverage(x, y));
		}
	}
	return coverage;
}
fn fillTexel(x: i32, y: i32) -> vec2<i32> {
	var texel = vec2<i32>(x, y);
	if (eyeDepth(x, y) < 1.0) { return texel; }
	let left = x - x % 2;
	let top = y - y % 2;
	var nearest = 1.0;
	for (var dy = 0; dy < 2; dy++) {
		for (var dx = 0; dx < 2; dx++) {
			let tx = min(left + dx, eyeWidth() - 1);
			let ty = min(top + dy, eyeHeight() - 1);
			let depth = eyeDepth(tx, ty);
			if (depth < nearest) {
				nearest = depth;
				texel = vec2<i32>(tx, ty);
			}
		}
	}
	return texel;
}`;
	// The eye's own depth: per-eye textures, the eye's pixels from (0, 0). A WebGPU canvas has no
	// meaningful colour alpha, so coverage is 1 where the replay wrote depth, 0 where it stayed at
	// the clear value.
	const eyeAccessors = `
@group(0) @binding(1) var depthTexture: texture_depth_2d;
fn eyeWidth() -> i32 { return i32(textureDimensions(depthTexture).x); }
fn eyeHeight() -> i32 { return i32(textureDimensions(depthTexture).y); }
fn eyeDepth(x: i32, y: i32) -> f32 { return textureLoad(depthTexture, vec2<i32>(x, y), 0); }
fn eyeCoverage(x: i32, y: i32) -> f32 { return select(0.0, 1.0, eyeDepth(x, y) < 1.0); }
${blockFunctions}`;
	// Same layout as webgl-atlas-pack@1's, top-left origin: each half-resolution sample is
	// `blockDepth`, two tiles side by side, saturated before splitting bytes (at depth 1, float32
	// rounds 16777215 + 0.5 to 16777216, which would wrap the far plane to black).
	const depthPack = `
struct Params { rectX: f32, rectY: f32, halfWidth: f32, tileHeight: f32, }
@group(0) @binding(0) var<uniform> params: Params;
${eyeAccessors}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let localX = i32(floor(frag.x - params.rectX));
	let localY = i32(floor(frag.y - params.rectY));
	let halfWidth = i32(params.halfWidth);
	let tileHeight = i32(params.tileHeight);
	let right = localX >= halfWidth;
	let column = select(localX, localX - halfWidth, right);
	let row = select(localY, localY + tileHeight, right);
	let d = blockDepth(column, row);
	let scaled = u32(min(floor(clamp(d, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0));
	return vec4<f32>(f32(scaled & 255u) / 255.0, f32((scaled >> 8u) & 255u) / 255.0, f32((scaled >> 16u) & 255u) / 255.0, 1.0);
}`;
	// TransportAlpha: alpha at half linear resolution, three consecutive half rows per atlas row in
	// R, G, B, each sample `blockCoverage`; opaque.
	const alphaPack = `
struct Params { rectX: f32, rectY: f32, pad0: f32, pad1: f32, }
@group(0) @binding(0) var<uniform> params: Params;
${eyeAccessors}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let column = i32(floor(frag.x - params.rectX));
	let row = i32(floor(frag.y - params.rectY)) * 3;
	return vec4<f32>(blockCoverage(column, row), blockCoverage(column, row + 1), blockCoverage(column, row + 2), 1.0);
}`;
	// Colour with a depth view: one eye pixel per atlas pixel through `fillTexel`. Without a depth
	// view the colour is a plain texture copy.
	const colorFill = `
struct Params { rectX: f32, rectY: f32, pad0: f32, pad1: f32, }
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(2) var colorTexture: texture_2d<f32>;
${eyeAccessors}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let texel = fillTexel(i32(floor(frag.x - params.rectX)), i32(floor(frag.y - params.rectY)));
	return textureLoad(colorTexture, texel, 0);
}`;

	function ensurePackPipelines(device, format) {
		if (packPipelines) return packPipelines;
		const layout = device.createBindGroupLayout({ entries: [
			{ binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
			{ binding: 1, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "depth" } }] });
		const colorLayout = device.createBindGroupLayout({ entries: [
			{ binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
			{ binding: 1, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "depth" } },
			{ binding: 2, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "unfilterable-float" } }] });
		const pipeline = (label, code, bindGroupLayout) => {
			const module = device.createShaderModule({ label, code: fullScreenVertex + code });
			return device.createRenderPipeline({ label, layout: device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] }),
				vertex: { module, entryPoint: "vs" }, fragment: { module, entryPoint: "fs", targets: [{ format }] },
				primitive: { topology: "triangle-list" } });
		};
		packPipelines = { layout, colorLayout, depth: pipeline("qar-depth-pack", depthPack, layout),
			alpha: pipeline("qar-alpha-pack", alphaPack, layout), color: pipeline("qar-color-fill", colorFill, colorLayout) };
		return packPipelines;
	}

	// The attachment textures of the recorded frame, other than the app texture: each eye renders
	// into its own copy, sized to its colour view (never to the app's canvas).
	function frameTextures(frame) {
		const textures = [];
		for (const pass of frame.passes) {
			for (const a of [...pass.colorAttachments, pass.depthStencilAttachment]) {
				const texture = a && a.view && a.view.__qarTexture;
				if (texture && texture !== S.appTexture && !textures.includes(texture)) textures.push(texture);
			}
		}
		return textures;
	}
	function lastDepthTexture(frame) {
		for (let i = frame.passes.length - 1; i >= 0; --i) {
			const a = frame.passes[i].depthStencilAttachment;
			if (a && a.view && a.view.__qarTexture) return a.view.__qarTexture;
		}
		throw new Error("QAR adaptation: the app's frame has no depth attachment to pack");
	}

	// The same textures in the same order: the app records a new frame object every frame, but its
	// attachments are normally the same textures for its whole life.
	function sameTextures(a, b) {
		if (!a || a.length !== b.length) return false;
		return a.every((texture, index) => texture === b[index]);
	}
	function destroyEye(eye) {
		eye.color.destroy();
		for (const texture of eye.textures.values()) texture.destroy();
		eye.depthParams.destroy();
		eye.alphaParams.destroy();
		eye.colorParams.destroy();
	}

	// Per-eye state: colour target, copies of the frame's textures, pack buffers, bind-group cache.
	// Rebuilt only when the eye's size or the app's set of attachment textures changes; the old GPU
	// resources are destroyed at once instead of waiting for garbage collection. (Comparing the
	// per-frame Set object instead recreated every eye's G-buffer on every tick: VRAM grew by
	// gigabytes and two users dropped to a few frames per second.)
	function eyeState(device, format, key, width, height, frame) {
		let eye = eyes.get(key);
		if (eye && eye.width === width && eye.height === height && sameTextures(eye.sourceTextures, frame.textures)) return eye;
		const cache = eye ? eye.cache : {};
		if (eye) destroyEye(eye); // the cache (site buffers) stays with the eye
		eye = { key, width, height, cache, sourceTextures: frame.textures, textures: new Map(), bindGroups: new Map(),
			color: device.createTexture({ label: `qar-eye-color ${key}`, size: [width, height], format,
				usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC | GPUTextureUsage.TEXTURE_BINDING }),
			depthParams: device.createBuffer({ size: 16, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }),
			alphaParams: device.createBuffer({ size: 16, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }),
			colorParams: device.createBuffer({ size: 16, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }) };
		for (const texture of frame.textures) {
			eye.textures.set(texture, device.createTexture({ label: `qar-eye ${texture.label || texture.format} ${key}`,
				size: [width, height], format: texture.format,
				usage: texture.usage | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING }));
		}
		eyes.set(key, eye);
		return eye;
	}

	function eyeView(eye, view) {
		if (view.__qarTexture === S.appTexture) return eye.color.createView();
		const texture = eye.textures.get(view.__qarTexture);
		return texture ? texture.createView(view.__qarViewDescriptor) : view;
	}
	function substituteResource(eye, resource, buffers) {
		if (resource instanceof GPUTextureView) return eyeView(eye, resource);
		if (resource instanceof GPUBuffer) return buffers.get(resource) || resource;
		if (resource && resource.buffer instanceof GPUBuffer && buffers.has(resource.buffer)) return { ...resource, buffer: buffers.get(resource.buffer) };
		return resource;
	}
	// A bind group that referenced a substituted texture or buffer is rebuilt once per eye and cached.
	function eyeBindGroup(device, eye, group, buffers) {
		const descriptor = group && group.__qarDescriptor;
		if (!descriptor) return group;
		let rebuilt = eye.bindGroups.get(group);
		if (rebuilt === undefined) {
			let changed = false;
			const entries = descriptor.entries.map((entry) => {
				const resource = substituteResource(eye, entry.resource, buffers);
				changed = changed || resource !== entry.resource;
				return { ...entry, resource };
			});
			rebuilt = changed ? device.createBindGroup({ ...descriptor, label: `${descriptor.label || "bind group"} ${eye.key}`, entries }) : null;
			eye.bindGroups.set(group, rebuilt);
		}
		return rebuilt || group;
	}

	// Viewport and scissor are dropped: they address the app's canvas, the eye target is whole.
	function replayFrame(device, encoder, eye, frame, buffers) {
		for (const pass of frame.passes) {
			const colorAttachments = pass.colorAttachments.map(a => (a ? { ...a, view: eyeView(eye, a.view), resolveTarget: undefined } : a));
			const depth = pass.depthStencilAttachment;
			const encoderPass = encoder.beginRenderPass({ colorAttachments,
				depthStencilAttachment: depth ? { ...depth, view: eyeView(eye, depth.view) } : undefined });
			for (const [op, args] of pass.commands) {
				if (op === "setViewport" || op === "setScissorRect") continue;
				if (op === "setBindGroup") { encoderPass.setBindGroup(args[0], eyeBindGroup(device, eye, args[1], buffers), ...args.slice(2)); continue; }
				encoderPass[op](...args);
			}
			encoderPass.end();
		}
	}

	function packPass(encoder, texture, pipeline, bindGroup, rect) {
		const pass = encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView(), loadOp: "load", storeOp: "store" }] });
		pass.setViewport(rect.x, rect.y, rect.width, rect.height, 0, 1);
		pass.setScissorRect(rect.x, rect.y, rect.width, rect.height);
		pass.setPipeline(pipeline);
		pass.setBindGroup(0, bindGroup);
		pass.draw(3);
		pass.end();
	}
	function packDepth(device, encoder, texture, eye, depthTexture, depthView) {
		const pipelines = packPipelines;
		const halfWidth = depthView.logicalWidth / 2;
		device.queue.writeBuffer(eye.depthParams, 0, new Float32Array([depthView.rect.x, depthView.rect.y, halfWidth, depthView.rect.height]));
		const bindGroup = device.createBindGroup({ layout: pipelines.layout, entries: [
			{ binding: 0, resource: { buffer: eye.depthParams } }, { binding: 1, resource: depthTexture.createView({ aspect: "depth-only" }) }] });
		packPass(encoder, texture, pipelines.depth, bindGroup, depthView.rect);
	}
	function fillColor(device, encoder, texture, eye, depthTexture, colorView) {
		const pipelines = packPipelines;
		device.queue.writeBuffer(eye.colorParams, 0, new Float32Array([colorView.rect.x, colorView.rect.y, 0, 0]));
		const bindGroup = device.createBindGroup({ layout: pipelines.colorLayout, entries: [
			{ binding: 0, resource: { buffer: eye.colorParams } }, { binding: 1, resource: depthTexture.createView({ aspect: "depth-only" }) },
			{ binding: 2, resource: eye.color.createView() }] });
		packPass(encoder, texture, pipelines.color, bindGroup, colorView.rect);
	}
	function packAlpha(device, encoder, texture, eye, depthTexture, alphaView) {
		const pipelines = packPipelines;
		device.queue.writeBuffer(eye.alphaParams, 0, new Float32Array([alphaView.rect.x, alphaView.rect.y, 0, 0]));
		const bindGroup = device.createBindGroup({ layout: pipelines.layout, entries: [
			{ binding: 0, resource: { buffer: eye.alphaParams } }, { binding: 1, resource: depthTexture.createView({ aspect: "depth-only" }) }] });
		packPass(encoder, texture, pipelines.alpha, bindGroup, alphaView.rect);
	}

	// With Engine "webgpu" the page paint is the atlas: the canvas covers it 1:1 at the top-left.
	function layoutCanvas(detail) {
		const c = S.canvas;
		if (c.width !== detail.atlas.width || c.height !== detail.atlas.height) { c.width = detail.atlas.width; c.height = detail.atlas.height; }
		const dpr = window.devicePixelRatio || 1;
		const css = `position:fixed;left:0;top:0;margin:0;max-width:none;max-height:none;width:${detail.atlas.width / dpr}px;height:${detail.atlas.height / dpr}px;`;
		if (c.style.cssText !== css) c.style.cssText = css;
	}

	function writeTag(device, texture, detail) {
		const tag = detail.tag;
		if (!tag || tag.width < 2) return false;
		const bgra = S.format.startsWith("bgra");
		const pixel = v => { const r = v & 255, g = (v >> 8) & 255, b = (v >> 16) & 255; return bgra ? [b, g, r, 255] : [r, g, b, 255]; };
		if (detail.continuous) {
			device.queue.writeTexture({ texture, origin: { x: detail.atlas.width - 4, y: tag.y } }, new Uint8Array(16), { bytesPerRow: 16 }, { width: 4, height: 1 });
		}
		const id = Number(BigInt(detail.requestId) & 0xffffffn);
		device.queue.writeTexture({ texture, origin: { x: tag.x, y: tag.y } }, new Uint8Array([...pixel(id), ...pixel(~id & 0xffffff)]), { bytesPerRow: 8 }, { width: 2, height: 1 });
		return true;
	}

	function drawTargets(device, format, encoder, texture, detail) {
		const site = S.site;
		if (!site || typeof site.eyeBuffers !== "function") throw new Error("QAR adaptation: no site script set __qarAdaptationState.site");
		ensurePackPipelines(device, format);
		const frame = S.lastFrame;
		if (!frame.textures) { frame.textures = frameTextures(frame); frame.depth = lastDepthTexture(frame); }
		const scale = site.nearFarScale || 1;
		if (site.beginTick) site.beginTick();
		// A depth view's eye can be "none" (one depth view per target): it belongs to the left eye.
		const sideOf = v => (v.eye === "right" ? "right" : "left");
		const used = new Set();
		for (const target of detail.targets) {
			for (const view of target.views.filter(v => v.dataType === "color")) {
				const camera = target.cameras[view.eye];
				const key = `${target.id}:${view.eye}`;
				used.add(key);
				const eye = eyeState(device, format, key, view.rect.width, view.rect.height, frame);
				const viewMatrix = viewFromPose(camera.pose.position, camera.pose.orientation, scale);
				const projectionMatrix = projection(camera.fov, camera.nearFar.near * scale, camera.nearFar.far * scale);
				const viewProjection = multiply(projectionMatrix, viewMatrix);
				const buffers = new Map(site.eyeBuffers({ key, device, camera, view: viewMatrix, projection: projectionMatrix,
					viewProjection, inverseViewProjection: invert(viewProjection), width: eye.width, height: eye.height, cache: eye.cache }));
				replayFrame(device, encoder, eye, frame, buffers);
				const depthTexture = eye.textures.get(frame.depth);
				const extras = target.views.filter(v => v.dataType !== "color" && sideOf(v) === view.eye);
				// With a depth view the colour follows the block rule (`fillTexel`); without one a
				// plain copy keeps every pixel as the app drew it.
				if (extras.some(v => v.dataType === "depth")) fillColor(device, encoder, texture, eye, depthTexture, view);
				else encoder.copyTextureToTexture({ texture: eye.color }, { texture, origin: { x: view.rect.x, y: view.rect.y } }, [view.rect.width, view.rect.height]);
				for (const extra of extras) {
					if (extra.dataType === "depth") packDepth(device, encoder, texture, eye, depthTexture, extra);
					else if (extra.dataType === "alpha") packAlpha(device, encoder, texture, eye, depthTexture, extra);
				}
			}
		}
		// A user who left releases its eyes' GPU resources (and the site's buffers in its cache).
		for (const [key, eye] of eyes) {
			if (used.has(key)) continue;
			destroyEye(eye);
			for (const value of Object.values(eye.cache)) if (value instanceof GPUBuffer) value.destroy();
			eyes.delete(key);
		}
	}

	// The app's own frame. While streaming it goes into the page region and nowhere else: an app
	// frame larger than the region would otherwise be copied over the users' views and cut them.
	function copyAppFrame(encoder, texture, detail) {
		let width = Math.min(S.appSize[0], texture.width), height = Math.min(S.appSize[1], texture.height);
		if (detail && detail.pageRegion) {
			width = Math.min(width, detail.pageRegion.width);
			height = Math.min(height, detail.pageRegion.height);
		}
		if (width > 0 && height > 0) encoder.copyTextureToTexture({ texture: S.appTexture }, { texture }, [width, height]);
	}

	function present(detail) {
		const device = S.device, format = S.format;
		if (!device || !format || !S.context || !S.appTexture) {
			if (detail) throw new Error("QAR adaptation: WebGPU app has not rendered yet");
			return false;
		}
		if (detail) layoutCanvas(detail);
		S.presenting = true;
		try {
			const texture = S.realGetCurrentTexture(S.context); // the only real call per tick
			const encoder = device.createCommandEncoder({ label: "qar-present" });
			const drawn = Boolean(detail && S.lastFrame);
			if (drawn) drawTargets(device, format, encoder, texture, detail);
			copyAppFrame(encoder, texture, detail);
			device.queue.submit([encoder.finish()]);
			if (drawn) S.stats.drawn++;
			return drawn ? writeTag(device, texture, detail) : false;
		} finally {
			S.presenting = false;
		}
	}

	// Never let an exception escape requestAnimationFrame: pages such as webgpu-samples install
	// global error handlers that put up an error dialog.
	let lastDraw = -Infinity;
	function tick(timestamp) {
		requestAnimationFrame(tick);
		S.stats.ticks++;
		const detail = latest;
		// While streaming, at most the profile's `RenderRate` frames per second (`detail.renderRate`,
		// 1 to 120, 30 when it sets none; since 1.1): a tick before the next draw is due leaves the
		// canvas as it is (no new frame at all, so the atlas keeps the last tagged one), the page
		// preview included.
		if (detail && Number.isFinite(timestamp)) {
			const rate = Math.min(Math.max(detail.renderRate > 0 ? detail.renderRate : 30, 1), 120);
			const interval = 1000 / rate;
			if (timestamp - lastDraw < interval - Math.min(interval * 0.25, 4)) return;
			lastDraw = timestamp;
		}
		try {
			if (detail && acknowledged !== detail) {
				acknowledged = detail;
				core.withRenderFrameAcknowledgement(detail, () => present(detail));
			} else present(detail);
		} catch (error) {
			S.stats.errors++;
			S.stats.lastError = String((error && error.stack) || error);
		}
	}
	requestAnimationFrame(tick);

	window.addEventListener("qaros-render-frame", ({ detail }) => { S.stats.requests++; latest = detail; });
	window.addEventListener("qaros-render-stop", () => { latest = null; acknowledged = null; });
})();
