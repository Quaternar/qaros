// @qaros-library threejs-webgpu-render-frame@1.1
// @stage afterLoad
// @needs render-frame-core@1 threejs-capture@1
//
// Answers each `qaros-render-frame` request for a three.js `WebGPURenderer` page (the threejs.org
// `webgpu_*` examples, any app built on `three/webgpu`; verified with r180 and r187) with the same
// atlas and request tag as threejs-render-frame@1 does for a `WebGLRenderer`: colour per eye, depth
// at half resolution as the nearest of each 2x2 block, alpha coverage that matches the colour, and
// colour fill of empty pixels in covered blocks. List both libraries (after threejs-capture@1 and
// render-frame-core@1) and one profile covers both renderers: render-frame-core@1
// `addRenderFrameHandler` gives each request to the library for the renderer the page created.
//
// Per tick:
//   1. every eye is rendered by three.js itself into one `RenderTarget` of this library (half-float
//      colour, with a `DepthTexture`), one `renderer.render` per eye into its own viewport, with the
//      application's scene, lights and layers. Standard depth: cleared to 1, the far plane.
//   2. a `QuadMesh` whose `NodeMaterial` is TSL `renderOutput(texture(eye colour, screenUV),
//      renderer.toneMapping, renderer.outputColorSpace)` converts that colour into a second, 8-bit
//      `RenderTarget`, the tone mapping and output colour space the page's own canvas gets.
//   3. WGSL passes on the renderer's own `GPUDevice` (`renderer.backend.device`, the targets' GPU
//      textures through `renderer.backend.get(texture).texture`) write the atlas into an
//      `OffscreenCanvas`: the colour of each eye (an empty pixel in a block holding geometry takes
//      the colour of the block's nearest pixel), the packed depth and, under `TransportAlpha`, the
//      packed alpha; then the request tag. The atlas is published to the stream surface
//      (render-frame-core@1 `streamSurface`). The application's own canvas and frame are untouched.
//
// three.js's classes and TSL come from the page's own three.js module: the import map's
// `three/webgpu` or `three` entry, or a loaded `three.webgpu.js`, imported again (a module is
// evaluated once per URL, so this is the instance the page uses; it is taken only when the renderer
// is an instance of its `WebGPURenderer`). A bundled app exposes no such module: then the
// `RenderTarget` class and the target's depth texture come from the renderer's own objects
// (feature-detected), and the colour pass applies the sRGB output encoding itself, without tone
// mapping (reported once in the console).
//
// The block functions (`blockDepth`, `blockCoverage`, `fillTexel`) are the same WGSL as
// webgpu-redirect-presenter@1's and the same rule as webgl-atlas-pack@1's GLSL;
// tests/webgpu-depth-pack.test.mjs runs both copies as JavaScript.
//
// The users' room and the scene: as threejs-render-frame@1 (`window.__qarAdaptationState
// .sceneFromRoom`, the application camera's layers, opaque materials write depth for the users'
// views, no scene background).
//
// Not covered: a `WebGPURenderer` on its WebGL 2 fallback backend, post-processing
// (`PostProcessing` / `RenderPipeline` run on the application's own frame only), a logarithmic depth
// buffer, and multisampling of the eye views.
(() => {
	const core = window.__qarAdaptationCore;
	const state = window.__qarAdaptationState;
	if (!core || !core.streamSurface || typeof core.addRenderFrameHandler !== "function") {
		throw new Error("QAR adaptation: threejs-webgpu-render-frame@1 needs render-frame-core@1.1 or newer, injected before it.");
	}

	// three.js constants, unchanged across releases.
	const UnsignedByteType = 1009;
	const HalfFloatType = 1016;
	const NearestFilter = 1003;
	const LinearToneMapping = 1;
	const NoToneMapping = 0;

	const eyeCameras = new Map();
	let classes = null;
	let resolvingClasses = null;
	let roomInScene = null;
	let eyeTarget = null;
	let outputTarget = null;
	let outputQuad = null;
	let outputKey = null;
	let toneMappingReported = false;

	// The specifiers the page's three.js module may be imported by: the import map's entries
	// (imported by key, so the map resolves them as for the page) and loaded three.webgpu builds.
	function threeModuleSpecifiers() {
		const specifiers = [];
		for (const script of document.querySelectorAll('script[type="importmap"]')) {
			try {
				const imports = JSON.parse(script.textContent).imports || {};
				for (const key of ["three/webgpu", "three"]) if (typeof imports[key] === "string") specifiers.push(key);
			} catch {}
		}
		const loaded = typeof performance !== "undefined" && performance.getEntriesByType ? performance.getEntriesByType("resource") : [];
		for (const entry of loaded) if (/\/three\.webgpu(\.min)?\.js(\?|#|$)/.test(entry.name)) specifiers.push(entry.name);
		return [...new Set(specifiers)];
	}

	async function pageThreeModule(renderer) {
		for (const specifier of threeModuleSpecifiers()) {
			try {
				const module = await import(specifier);
				const { WebGPURenderer, RenderTarget, DepthTexture, QuadMesh, NodeMaterial, TSL } = module;
				if (typeof WebGPURenderer !== "function" || !(renderer instanceof WebGPURenderer)) continue;
				if (!RenderTarget || !DepthTexture || !QuadMesh || !NodeMaterial || !TSL) continue;
				if (typeof TSL.renderOutput !== "function" || typeof TSL.texture !== "function" || !TSL.screenUV) continue;
				return module;
			} catch {}
		}
		return null;
	}

	// Without the page's module: a `RenderTarget` the renderer made itself (its frame buffer target;
	// asked with a tone mapping set for one call, since a page with neither tone mapping nor an output
	// conversion has none).
	function rendererRenderTargetClass(renderer) {
		let target = renderer._frameBufferTarget || null;
		if (!target && typeof renderer._getFrameBufferTarget === "function") {
			const toneMapping = renderer.toneMapping;
			renderer.toneMapping = LinearToneMapping;
			try { target = renderer._getFrameBufferTarget(); } catch { target = null; } finally { renderer.toneMapping = toneMapping; }
		}
		if (!target || !target.isRenderTarget) {
			throw new Error("QAR adaptation: cannot find three.js's RenderTarget class: the page's three.js module is not importable and the renderer has no render target to take it from.");
		}
		return target.constructor;
	}

	// The camera classes come from the renderer's XR camera (an `ArrayCamera`, whose base class is
	// `PerspectiveCamera`) whichever way the rest is found.
	async function findClasses(renderer) {
		const xrCamera = renderer.xr && typeof renderer.xr.getCamera === "function" ? renderer.xr.getCamera() : null;
		if (!xrCamera || !xrCamera.isArrayCamera) throw new Error("QAR adaptation: cannot find three.js camera classes on the renderer.");
		const PerspectiveCamera = Object.getPrototypeOf(xrCamera.constructor.prototype).constructor;
		const Object3D = Object.getPrototypeOf(Object.getPrototypeOf(PerspectiveCamera.prototype)).constructor;
		const module = await pageThreeModule(renderer);
		if (module) {
			const { RenderTarget, DepthTexture, QuadMesh, NodeMaterial, TSL } = module;
			return { PerspectiveCamera, Object3D, RenderTarget, DepthTexture, QuadMesh, NodeMaterial, TSL, source: "module" };
		}
		return { PerspectiveCamera, Object3D, RenderTarget: rendererRenderTargetClass(renderer), source: "renderer" };
	}

	// A colour the renderer's `getClearColor` copies into, and `setClearColor` takes back.
	function clearColorHolder() {
		return { isColor: true, r: 0, g: 0, b: 0, copy(color) { this.r = color.r; this.g = color.g; this.b = color.b; return this; } };
	}

	function updateRoomInScene() {
		if (!roomInScene) roomInScene = new classes.Object3D();
		const pose = typeof state.sceneFromRoom === "function" ? state.sceneFromRoom() : null;
		roomInScene.position.fromArray(pose ? pose.position : [0, 0, 0]);
		roomInScene.quaternion.fromArray(pose ? pose.quaternion : [0, 0, 0, 1]);
		roomInScene.updateMatrixWorld(true);
	}

	function getEyeCamera(key) {
		let camera = eyeCameras.get(key);
		if (!camera) {
			camera = new classes.PerspectiveCamera();
			roomInScene.add(camera);
			eyeCameras.set(key, camera);
		}
		return camera;
	}

	// An asymmetric projection from the request's four frustum angles. The camera takes the
	// renderer's coordinate system first: a camera in another one is reset by three.js to its own
	// symmetric projection before rendering.
	function configureCamera(camera, protocolCamera, layersMask, coordinateSystem) {
		const { pose, fov, nearFar } = protocolCamera;
		camera.coordinateSystem = coordinateSystem;
		camera.position.fromArray(pose.position);
		camera.quaternion.set(pose.orientation[0], pose.orientation[1], pose.orientation[2], pose.orientation[3]);
		camera.near = nearFar.near;
		camera.far = nearFar.far;
		camera.projectionMatrix.makePerspective(
			Math.tan(fov.angleLeft) * camera.near,
			Math.tan(fov.angleRight) * camera.near,
			Math.tan(fov.angleUp) * camera.near,
			Math.tan(fov.angleDown) * camera.near,
			camera.near,
			camera.far,
			coordinateSystem
		);
		camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
		camera.layers.mask = layersMask;
		camera.updateMatrixWorld(true);
	}

	// One entry per eye that has a view, laid out one row per user. `x`, `y` count from the
	// top-left of the eye target, as WebGPU viewports do.
	function collectEyes(targets, layersMask, coordinateSystem) {
		const eyes = [];
		let rowY = 0;
		let width = 0;
		for (const target of targets) {
			const byEye = new Map();
			for (const view of target.views) {
				const name = view.eye === "right" ? "right" : "left";
				const eye = byEye.get(name) || { name };
				eye[view.dataType === "depth" || view.dataType === "alpha" ? view.dataType : "color"] = view;
				byEye.set(name, eye);
			}
			let rowX = 0;
			let rowHeight = 0;
			for (const eye of byEye.values()) {
				const sizeView = eye.color || eye.depth;
				eye.width = sizeView.logicalWidth;
				eye.height = sizeView.logicalHeight;
				eye.x = rowX;
				eye.y = rowY;
				eye.key = `${target.id}:${eye.name}`;
				eye.camera = getEyeCamera(eye.key);
				configureCamera(eye.camera, target.cameras[eye.name], layersMask, coordinateSystem);
				rowX += eye.width;
				rowHeight = Math.max(rowHeight, eye.height);
				eyes.push(eye);
			}
			width = Math.max(width, rowX);
			rowY += rowHeight;
		}
		return { eyes, width, height: rowY };
	}

	// Users see only what has depth (see threejs-render-frame@1): every opaque material writes depth
	// for the users' views. Returns what to restore.
	function writeDepthForOpaqueMaterials(scene) {
		const changed = [];
		const writeDepth = (material) => {
			if (!material || material.depthWrite !== false || material.transparent) return;
			if (material.opacity !== undefined && material.opacity < 1) return;
			material.depthWrite = true;
			changed.push(material);
		};
		scene.traverse((object) => {
			if (!object.isMesh) return;
			if (Array.isArray(object.material)) object.material.forEach(writeDepth);
			else writeDepth(object.material);
		});
		return () => changed.forEach((material) => { material.depthWrite = false; });
	}

	function ensureTargets(width, height) {
		if (eyeTarget && eyeTarget.width === width && eyeTarget.height === height) return;
		if (eyeTarget) eyeTarget.dispose();
		if (outputTarget) outputTarget.dispose();
		const options = { generateMipmaps: false, minFilter: NearestFilter, magFilter: NearestFilter };
		eyeTarget = new classes.RenderTarget(width, height, { ...options, type: HalfFloatType, depthBuffer: true });
		if (classes.DepthTexture) eyeTarget.depthTexture = new classes.DepthTexture(width, height);
		outputTarget = classes.QuadMesh
			? new classes.RenderTarget(width, height, { ...options, type: UnsignedByteType, depthBuffer: false })
			: null;
		outputKey = null;
	}

	// The output conversion of the page's own canvas (tone mapping, output colour space), in TSL.
	function updateOutputQuad(renderer) {
		const key = `${renderer.toneMapping}|${renderer.outputColorSpace}|${eyeTarget.texture.uuid}`;
		if (outputKey === key) return;
		const { TSL } = classes;
		if (!outputQuad) outputQuad = new classes.QuadMesh(new classes.NodeMaterial());
		outputQuad.material.fragmentNode = TSL.renderOutput(TSL.texture(eyeTarget.texture, TSL.screenUV), renderer.toneMapping, renderer.outputColorSpace);
		outputQuad.material.needsUpdate = true;
		outputKey = key;
	}

	// The depth texture of the eye target: ours, or the one three.js gave it.
	function eyeDepthTexture(renderer) {
		if (eyeTarget.depthTexture) return eyeTarget.depthTexture;
		const data = renderer._textures && typeof renderer._textures.get === "function" ? renderer._textures.get(eyeTarget) : null;
		if (!data || !data.depthTexture) throw new Error("QAR adaptation: cannot find the depth texture of the eye render target.");
		return data.depthTexture;
	}

	function renderEyes(renderer, scene, eyes) {
		const previousTarget = renderer.getRenderTarget();
		const autoClear = renderer.autoClear;
		const scissorTest = renderer.getScissorTest();
		const clearColor = renderer.getClearColor(clearColorHolder());
		const clearAlpha = renderer.getClearAlpha();
		const background = scene.background;
		const backgroundNode = scene.backgroundNode;
		// Users get the scene without its background: in AR the room shows behind it, and a clear
		// colour would blend into every antialiased edge.
		scene.background = null;
		if (backgroundNode !== undefined) scene.backgroundNode = null;
		const restoreDepthWrite = writeDepthForOpaqueMaterials(scene);
		try {
			if (outputTarget) updateOutputQuad(renderer);
			renderer.setRenderTarget(eyeTarget);
			renderer.setClearColor(0x000000, 0);
			renderer.autoClear = false;
			renderer.setScissorTest(false);
			eyeTarget.viewport.set(0, 0, eyeTarget.width, eyeTarget.height);
			renderer.clear(true, true, true);
			renderer.setScissorTest(true);
			for (const eye of eyes) {
				eyeTarget.viewport.set(eye.x, eye.y, eye.width, eye.height);
				eyeTarget.scissor.set(eye.x, eye.y, eye.width, eye.height);
				renderer.render(scene, eye.camera);
			}
			renderer.setScissorTest(false);
			if (outputTarget) {
				renderer.setRenderTarget(outputTarget);
				outputQuad.render(renderer);
			}
		} finally {
			restoreDepthWrite();
			scene.background = background;
			if (backgroundNode !== undefined) scene.backgroundNode = backgroundNode;
			renderer.setClearColor(clearColor, clearAlpha);
			renderer.autoClear = autoClear;
			renderer.setScissorTest(scissorTest);
			renderer.setRenderTarget(previousTarget);
		}
	}

	// --- The atlas: WGSL passes on the renderer's device, into an OffscreenCanvas. ---

	const fullScreenVertex = `
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
	var p = array<vec2<f32>, 3>(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
	return vec4<f32>(p[i], 0.0, 1.0);
}`;
	// The kit's block rule, identical to webgpu-redirect-presenter@1's: depth and alpha travel at
	// half resolution, one sample per 2x2 block, nearest-upsampled by native code. A sample keeps
	// what any of its four pixels holds: the nearest depth (standard depth, cleared to 1) and the
	// largest coverage, so a 1-px line keeps its depth and coverage in every block it crosses.
	// `fillTexel`: an empty pixel in a block holding geometry shows the colour of the block's
	// nearest pixel, so colour, depth and alpha describe the same block.
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
	// `rect`: the destination rectangle's origin, then per pass: colour (fill flag, sRGB encode flag), depth
	// (halfWidth, tileHeight). `eye`: the eye's origin and size in the eye targets. Coverage is the
	// converted colour's alpha (cleared to 0), as threejs-render-frame@1 packs it.
	const eyeAccessors = `
struct Params { rect: vec4<i32>, eye: vec4<i32>, }
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var depthTexture: texture_depth_2d;
@group(0) @binding(2) var colorTexture: texture_2d<f32>;
fn eyeWidth() -> i32 { return params.eye.z; }
fn eyeHeight() -> i32 { return params.eye.w; }
fn eyeDepth(x: i32, y: i32) -> f32 { return textureLoad(depthTexture, params.eye.xy + vec2<i32>(x, y), 0); }
fn eyeCoverage(x: i32, y: i32) -> f32 { return textureLoad(colorTexture, params.eye.xy + vec2<i32>(x, y), 0).a; }
fn localPixel(frag: vec4<f32>) -> vec2<i32> { return vec2<i32>(floor(frag.xy)) - params.rect.xy; }
${blockFunctions}`;
	// Opaque: alpha travels in its own packed view, never in the colour pixel. The sRGB encoding is
	// applied here only when no output pass converted the colour (no page module).
	const colorPass = `${eyeAccessors}
fn srgbEncode(linear: vec3<f32>) -> vec3<f32> {
	let c = clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0));
	return select(1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3<f32>(0.0031308));
}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let local = localPixel(frag);
	var texel = local;
	if (params.rect.z != 0) { texel = fillTexel(local.x, local.y); }
	var color = textureLoad(colorTexture, params.eye.xy + texel, 0).rgb;
	if (params.rect.w != 0) { color = srgbEncode(color); }
	return vec4<f32>(color, 1.0);
}`;
	// Two side-by-side tiles of half the view's width and a quarter of its height, row 0 the view's
	// top row; 24-bit unsigned normalized little-endian RGB, saturated before splitting bytes (at
	// depth 1, float32 rounds 16777215 + 0.5 to 16777216, which would wrap the far plane to black).
	const depthPass = `${eyeAccessors}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let local = localPixel(frag);
	let halfWidth = params.rect.z;
	let right = local.x >= halfWidth;
	let column = select(local.x, local.x - halfWidth, right);
	let row = select(local.y, local.y + params.rect.w, right);
	let d = blockDepth(column, row);
	let scaled = u32(min(floor(clamp(d, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0));
	return vec4<f32>(f32(scaled & 255u) / 255.0, f32((scaled >> 8u) & 255u) / 255.0, f32((scaled >> 16u) & 255u) / 255.0, 1.0);
}`;
	// Three consecutive half-resolution rows per atlas row (R, G, B).
	const alphaPass = `${eyeAccessors}
@fragment fn fs(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
	let local = localPixel(frag);
	let row = local.y * 3;
	return vec4<f32>(blockCoverage(local.x, row), blockCoverage(local.x, row + 1), blockCoverage(local.x, row + 2), 1.0);
}`;

	let atlas = null;
	function ensureAtlas(device, width, height) {
		if (!atlas || atlas.device !== device) {
			const canvas = new OffscreenCanvas(width, height);
			const context = canvas.getContext("webgpu");
			if (!context) throw new Error("QAR adaptation: no WebGPU context for the atlas canvas.");
			const format = navigator.gpu.getPreferredCanvasFormat();
			context.configure({ device, format, alphaMode: "opaque", usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_DST });
			const layout = device.createBindGroupLayout({ entries: [
				{ binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
				{ binding: 1, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "depth" } },
				{ binding: 2, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "unfilterable-float" } }] });
			const pipelineLayout = device.createPipelineLayout({ bindGroupLayouts: [layout] });
			const pipeline = (label, code) => {
				const module = device.createShaderModule({ label, code: fullScreenVertex + code });
				return device.createRenderPipeline({ label, layout: pipelineLayout,
					vertex: { module, entryPoint: "vs" }, fragment: { module, entryPoint: "fs", targets: [{ format }] },
					primitive: { topology: "triangle-list" } });
			};
			if (atlas) for (const buffer of atlas.buffers.values()) buffer.destroy();
			atlas = { device, canvas, context, format, layout, buffers: new Map(),
				color: pipeline("qar-threejs-color", colorPass), depth: pipeline("qar-threejs-depth", depthPass),
				alpha: pipeline("qar-threejs-alpha", alphaPass) };
		}
		if (atlas.canvas.width !== width) atlas.canvas.width = width;
		if (atlas.canvas.height !== height) atlas.canvas.height = height;
		return atlas;
	}

	// One uniform buffer per draw: every draw of a tick is in one submit, so a shared buffer would
	// give every draw the last write.
	function paramsBuffer(device, key, values) {
		let buffer = atlas.buffers.get(key);
		if (!buffer) {
			buffer = device.createBuffer({ size: 32, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
			atlas.buffers.set(key, buffer);
		}
		device.queue.writeBuffer(buffer, 0, new Int32Array(values));
		return buffer;
	}

	function packEyes(device, colorTexture, depthTexture, eyes, texture, encodeSrgb) {
		const encoder = device.createCommandEncoder({ label: "qar-threejs-atlas" });
		encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView(), loadOp: "clear",
			clearValue: { r: 0, g: 0, b: 0, a: 1 }, storeOp: "store" }] }).end();
		const depthView = depthTexture.createView({ aspect: "depth-only" });
		const colorView = colorTexture.createView();
		const used = new Set();
		const draw = (key, pipeline, rect, extra, eye) => {
			used.add(key);
			const buffer = paramsBuffer(device, key, [rect.x, rect.y, extra[0], extra[1], eye.x, eye.y, eye.width, eye.height]);
			const bindGroup = device.createBindGroup({ layout: atlas.layout, entries: [
				{ binding: 0, resource: { buffer } }, { binding: 1, resource: depthView }, { binding: 2, resource: colorView }] });
			const pass = encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView(), loadOp: "load", storeOp: "store" }] });
			pass.setViewport(rect.x, rect.y, rect.width, rect.height, 0, 1);
			pass.setScissorRect(rect.x, rect.y, rect.width, rect.height);
			pass.setPipeline(pipeline);
			pass.setBindGroup(0, bindGroup);
			pass.draw(3);
			pass.end();
		};
		for (const eye of eyes) {
			if (eye.color) draw(`${eye.key}:color`, atlas.color, eye.color.rect, [eye.depth ? 1 : 0, encodeSrgb ? 1 : 0], eye);
			if (eye.depth) draw(`${eye.key}:depth`, atlas.depth, eye.depth.rect, [eye.depth.logicalWidth / 2, eye.depth.rect.height], eye);
			if (eye.alpha) draw(`${eye.key}:alpha`, atlas.alpha, eye.alpha.rect, [0, 0], eye);
		}
		device.queue.submit([encoder.finish()]);
		for (const [key, buffer] of atlas.buffers) {
			if (used.has(key)) continue;
			buffer.destroy();
			atlas.buffers.delete(key);
		}
	}

	// The request id's low 24 bits as one pixel's RGB and their complement as the next, exact bytes.
	function writeTag(device, texture, detail) {
		const tag = detail.tag;
		if (!tag || tag.width < 2) return false;
		const bgra = atlas.format.startsWith("bgra");
		const pixel = (v) => { const r = v & 255, g = (v >> 8) & 255, b = (v >> 16) & 255; return bgra ? [b, g, r, 255] : [r, g, b, 255]; };
		const id = Number(BigInt(detail.requestId) & 0xffffffn);
		device.queue.writeTexture({ texture, origin: { x: tag.x, y: tag.y } }, new Uint8Array([...pixel(id), ...pixel(~id & 0xffffff)]),
			{ bytesPerRow: 8 }, { width: 2, height: 1 });
		return true;
	}

	function isWebGpuRenderer(renderer) {
		return Boolean(renderer && renderer.isWebGPURenderer);
	}

	function gpuTexture(backend, texture, what) {
		const data = typeof backend.get === "function" ? backend.get(texture) : null;
		if (!data || !data.texture) throw new Error(`QAR adaptation: the WebGPU backend has no GPU texture for the ${what}.`);
		return data.texture;
	}

	function drawFrame(renderer, scene, detail) {
		const backend = renderer.backend;
		const device = backend.device;
		const layersMask = state.camera && state.camera.layers ? state.camera.layers.mask : 1;
		updateRoomInScene();
		state.suspendConfinement = true;
		let tagged = false;
		try {
			const { eyes, width, height } = collectEyes(detail.targets, layersMask, renderer.coordinateSystem);
			const { context } = ensureAtlas(device, detail.atlas.width, detail.atlas.height);
			const texture = context.getCurrentTexture();
			if (eyes.length > 0) {
				ensureTargets(width, height);
				renderEyes(renderer, scene, eyes);
				const encodeSrgb = !outputTarget && renderer.outputColorSpace === "srgb";
				if (!outputTarget && renderer.toneMapping !== NoToneMapping && !toneMappingReported) {
					toneMappingReported = true;
					console.warn("QAR adaptation: the page's three.js module is not importable; users' views get the sRGB output encoding without the page's tone mapping.");
				}
				const colorGpu = gpuTexture(backend, (outputTarget || eyeTarget).texture, "eye colour");
				const depthGpu = gpuTexture(backend, eyeDepthTexture(renderer), "eye depth");
				packEyes(device, colorGpu, depthGpu, eyes, texture, encodeSrgb);
			} else {
				const encoder = device.createCommandEncoder();
				encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView(), loadOp: "clear",
					clearValue: { r: 0, g: 0, b: 0, a: 1 }, storeOp: "store" }] }).end();
				device.queue.submit([encoder.finish()]);
			}
			tagged = writeTag(device, texture, detail);
		} finally {
			state.suspendConfinement = false;
		}
		core.streamSurface.publish(atlas.canvas.transferToImageBitmap(), detail.atlas.width, detail.atlas.height);
		return tagged;
	}

	// The first request waits for three.js's classes (an asynchronous import); later ones draw at
	// once.
	function render(detail) {
		const renderer = state && state.renderer;
		const scene = state && state.scene;
		if (!renderer || !scene) {
			throw new Error("QAR adaptation: no captured three.js renderer/scene yet -- the application has not rendered a frame.");
		}
		if (!isWebGpuRenderer(renderer)) throw new Error("QAR adaptation: threejs-webgpu-render-frame@1 streams a WebGPURenderer only.");
		const backend = renderer.backend;
		if (!backend || !backend.isWebGPUBackend || !backend.device) {
			throw new Error("QAR adaptation: this WebGPURenderer runs on its WebGL 2 fallback; threejs-webgpu-render-frame@1 needs the WebGPU backend.");
		}
		if (renderer.logarithmicDepthBuffer) throw new Error("QAR adaptation: a logarithmic depth buffer cannot be streamed with depth.");
		if (classes) return drawFrame(renderer, scene, detail);
		if (!resolvingClasses) {
			// `state.threejsClassSource` ("module" or "renderer") tells an author which way was found.
			resolvingClasses = findClasses(renderer).then((found) => { classes = found; state.threejsClassSource = found.source; return found; },
				(error) => { resolvingClasses = null; throw error; });
		}
		return resolvingClasses.then(() => drawFrame(renderer, scene, detail));
	}

	// Establish the compositor before native code waits for the first atlas-size paint.
	core.streamSurface.ensure(1, 1);
	const loop = core.createRenderFrameLoop(render);
	core.addRenderFrameHandler({ name: "threejs-webgpu", accepts: () => isWebGpuRenderer(state.renderer), loop });
})();
