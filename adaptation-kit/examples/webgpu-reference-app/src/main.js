// Hand-written WebGPU: no rendering engine, no matrix library, no model loader -- everything this
// application draws comes from geometry.js's procedural generators, wgpu-utils.js's own thin
// device/shader-module/buffer wrappers, and mat4.js's own matrix helpers. It carries no QAR awareness
// whatsoever: nothing here references `window.qaros`, dispatches or listens for a
// "qaros-render-frame" event, or knows that it might one day be adapted.
//
// The one concession to realism rather than obscurity, matching the other two reference applications
// (`window.THREE`/`scene`/`camera`/`renderer` for three.js, `window.glCamera`/`glDraw` for raw
// WebGL): this application exposes `window.gpuDevice`, `window.gpuCamera` (the current view/projection
// matrices) and `window.gpuDraw` (a plain function that issues this frame's draw calls into a caller-
// supplied render pass) for console debugging and quick experimentation. `window.gpuDevice` is not
// merely a convenience the way the other two applications' equivalents are: WebGPU's
// `GPUCanvasContext` has no public accessor back to the `GPUDevice` that configured it, so an
// adaptation capturing only the canvas context (the way the kit's
// lib/webgl-capture-context@1.js captures only the `WebGL2RenderingContext`) genuinely cannot reach the device by
// any other route. None of these three names belong to any known rendering library, so
// `ProbePageEngine`'s `kProbeKnownEngines` list has nothing to match here, exactly like the raw-WebGL
// reference application.
//
// `window.gpuDraw` takes the active `GPURenderPassEncoder` as a parameter rather than drawing into
// implicit global state the way `window.glDraw()` does: WebGPU's API is encoder-based, not
// state-machine-based like WebGL, so there is no "currently bound context" for a parameterless
// function to draw into. This is the natural WebGPU-shaped equivalent of the same convention, not an
// added hook for anything external.
import { createCheckedShaderModule, initWebGpu, createUniformBuffer, createVertexBuffer, createIndexBuffer } from "./wgpu-utils.js";
import { createBox, createOctahedron, createCone, createDisc } from "./geometry.js";
import { identity, lookAt, perspectiveFromFov, modelFromPositionYaw } from "./mat4.js";

const canvas = document.querySelector("#viewport");

const shaderCode = `
struct Uniforms {
	model: mat4x4<f32>,
	view: mat4x4<f32>,
	projection: mat4x4<f32>,
	baseColorAndLight: vec4<f32>,
	lightDirAndAmbient: vec4<f32>,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) worldNormal: vec3<f32>,
}

@vertex
fn vertexMain(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>) -> VertexOutput {
	var out: VertexOutput;
	out.position = uniforms.projection * uniforms.view * uniforms.model * vec4<f32>(position, 1.0);
	out.worldNormal = (uniforms.model * vec4<f32>(normal, 0.0)).xyz;
	return out;
}

@fragment
fn fragmentMain(in: VertexOutput) -> @location(0) vec4<f32> {
	let normal = normalize(in.worldNormal);
	let lightDir = normalize(-uniforms.lightDirAndAmbient.xyz);
	let diffuse = max(dot(normal, lightDir), 0.0);
	let shaded = uniforms.baseColorAndLight.xyz * (uniforms.lightDirAndAmbient.w + diffuse * uniforms.baseColorAndLight.w);
	return vec4<f32>(shaded, 1.0);
}
`;

// 3 * mat4x4<f32> (64 bytes each) + 2 * vec4<f32> (16 bytes each).
const UNIFORM_BUFFER_SIZE = 224;

function writeObjectUniforms(device, buffer, model, view, projection, baseColor, lightIntensity, lightDirection, ambient) {
	const data = new Float32Array(UNIFORM_BUFFER_SIZE / 4);
	data.set(model, 0);
	data.set(view, 16);
	data.set(projection, 32);
	data.set([baseColor[0], baseColor[1], baseColor[2], lightIntensity], 48);
	data.set([lightDirection[0], lightDirection[1], lightDirection[2], ambient], 52);
	device.queue.writeBuffer(buffer, 0, data.buffer, data.byteOffset, data.byteLength);
}

async function main() {
	const { device, context, format } = await initWebGpu(canvas);
	window.gpuDevice = device;
	// Exposed for the same reason `window.gpuDevice` is (see the kit's lib/webgpu-capture-context@1.js's own
	// doc comment): a `GPUCanvasContext` has no public accessor back to its own configured format,
	// and the depth-pack and clear-quad pipelines render-frame-handler.js builds need it to match
	// this canvas's own colour attachment format exactly.
	window.gpuCanvasFormat = format;

	const shaderModule = await createCheckedShaderModule(device, shaderCode, "scene-shader");

	const bindGroupLayout = device.createBindGroupLayout({
		entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } }],
	});
	const pipelineLayout = device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] });
	const vertexBufferLayouts = [
		{ arrayStride: 12, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x3" }] },
		{ arrayStride: 12, attributes: [{ shaderLocation: 1, offset: 0, format: "float32x3" }] },
	];
	const colorPipeline = device.createRenderPipeline({
		layout: pipelineLayout,
		vertex: { module: shaderModule, entryPoint: "vertexMain", buffers: vertexBufferLayouts },
		fragment: { module: shaderModule, entryPoint: "fragmentMain", targets: [{ format }] },
		primitive: { topology: "triangle-list", cullMode: "none" },
		depthStencil: { format: "depth24plus", depthWriteEnabled: true, depthCompare: "less" },
	});
	// A depth-only variant of the same pipeline (no fragment stage, no colour target) -- used by
	// the kit's lib/webgpu-render-frame-handler@1.js to render a target's depth-capture pass without also
	// paying for fragment shading nothing will read. See its own doc comment for how packed depth is
	// produced from this pipeline's output.
	const depthOnlyPipeline = device.createRenderPipeline({
		layout: pipelineLayout,
		vertex: { module: shaderModule, entryPoint: "vertexMain", buffers: vertexBufferLayouts },
		primitive: { topology: "triangle-list", cullMode: "none" },
		depthStencil: { format: "depth24plus", depthWriteEnabled: true, depthCompare: "less" },
	});
	window.gpuColorPipeline = colorPipeline;
	window.gpuDepthOnlyPipeline = depthOnlyPipeline;
	window.gpuBindGroupLayout = bindGroupLayout;

	// A scoped "clear my own viewport/scissor rectangle" pipeline -- see `renderOwnFrame`'s own doc
	// comment on why this application cannot simply use `loadOp: "clear"` on its render pass the way
	// an equivalent WebGL application safely uses `gl.clear()` (WebGL's clear honours an enabled
	// scissor rectangle; WebGPU's `loadOp: "clear"` always clears the *entire* attachment regardless
	// of any viewport/scissor set afterwards). This mirrors the kit's lib/webgpu-render-frame-handler@1.js's
	// own identically-shaped clear-quad pipeline exactly, duplicated here rather than shared: this
	// file cannot depend on the adaptation (which may not even be installed), and the adaptation
	// cannot depend on this file (it runs as an injected snippet, not an importable module).
	const clearQuadModule = device.createShaderModule({
		label: "own-clear-quad",
		code: `
@vertex
fn vertexMain(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
	var positions = array<vec2<f32>, 3>(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
	return vec4<f32>(positions[index], 1.0, 1.0);
}
@fragment
fn fragmentMain() -> @location(0) vec4<f32> {
	return vec4<f32>(0.03, 0.035, 0.04, 1.0);
}
`,
	});
	const clearQuadPipeline = device.createRenderPipeline({
		layout: "auto",
		vertex: { module: clearQuadModule, entryPoint: "vertexMain" },
		fragment: { module: clearQuadModule, entryPoint: "fragmentMain", targets: [{ format }] },
		primitive: { topology: "triangle-list" },
		depthStencil: { format: "depth24plus", depthWriteEnabled: true, depthCompare: "always" },
	});

	function createMesh(geometry) {
		return {
			vertexBuffer: createVertexBuffer(device, geometry.positions, "positions"),
			normalBuffer: createVertexBuffer(device, geometry.normals, "normals"),
			indexBuffer: createIndexBuffer(device, geometry.indices, "indices"),
			indexCount: geometry.indices.length,
		};
	}

	const meshes = {
		box: createMesh(createBox(1.1)),
		octahedron: createMesh(createOctahedron(0.75)),
		cone: createMesh(createCone(0.65, 1.3, 24)),
		ground: createMesh(createDisc(6, 48)),
	};

	const orbitRadius = 2.4;
	const objectDefinitions = [
		{ name: "Teal block", mesh: meshes.box, color: new Float32Array([0.16, 0.55, 0.52]) },
		{ name: "Indigo spike", mesh: meshes.octahedron, color: new Float32Array([0.32, 0.3, 0.72]) },
		{ name: "Rose cone", mesh: meshes.cone, color: new Float32Array([0.72, 0.32, 0.42]) },
	];
	const objects = objectDefinitions.map((definition, index) => {
		const uniformBuffer = createUniformBuffer(device, UNIFORM_BUFFER_SIZE, `${definition.name}-uniforms`);
		const bindGroup = device.createBindGroup({
			layout: bindGroupLayout,
			entries: [{ binding: 0, resource: { buffer: uniformBuffer } }],
		});
		return {
			...definition,
			baseAngle: (index / objectDefinitions.length) * Math.PI * 2,
			visible: true,
			model: identity(),
			uniformBuffer,
			bindGroup,
		};
	});
	const groundUniformBuffer = createUniformBuffer(device, UNIFORM_BUFFER_SIZE, "ground-uniforms");
	const ground = {
		mesh: meshes.ground,
		color: new Float32Array([0.06, 0.07, 0.09]),
		model: modelFromPositionYaw(identity(), [0, -1.4, 0], 0),
		uniformBuffer: groundUniformBuffer,
		bindGroup: device.createBindGroup({
			layout: bindGroupLayout,
			entries: [{ binding: 0, resource: { buffer: groundUniformBuffer } }],
		}),
	};

	let sceneRotation = 0;
	let rotationSpeed = 0.3;
	let lightIntensity = 1.3;
	const ambient = 0.35;
	const lightDirection = new Float32Array([-0.4, -1.0, -0.3]);

	// `window.gpuCamera` holds the *current* view/projection matrices this application's own animation
	// loop drives every frame; the adaptation (see the kit's lib/webgpu-render-frame-handler@1.js) overwrites
	// both immediately before calling `window.gpuDraw(pass, pipeline, viewportRect)` directly -- the
	// same "hand the application's own draw entry point a throwaway camera" technique the other two
	// reference applications use.
	window.gpuCamera = { view: identity(), projection: identity() };

	let depthTexture = null;
	let depthTextureView = null;
	function ensureDepthTexture(width, height) {
		if (depthTexture && depthTexture.width === width && depthTexture.height === height) {
			return depthTextureView;
		}
		if (depthTexture) {
			depthTexture.destroy();
		}
		depthTexture = device.createTexture({
			size: [width, height],
			format: "depth24plus",
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
		});
		depthTextureView = depthTexture.createView();
		return depthTextureView;
	}
	window.gpuEnsureDepthTexture = ensureDepthTexture;

	function updateDefaultCamera() {
		const aspect = canvas.width / Math.max(canvas.height, 1);
		perspectiveFromFov(window.gpuCamera.projection, (50 * Math.PI) / 180, aspect, 0.1, 100);
		lookAt(window.gpuCamera.view, [0, 1.6, 6], [0, 0, 0], [0, 1, 0]);
	}

	function drawObject(pass, object) {
		writeObjectUniforms(
			device,
			object.uniformBuffer,
			object.model,
			window.gpuCamera.view,
			window.gpuCamera.projection,
			object.color,
			lightIntensity,
			lightDirection,
			ambient
		);
		pass.setBindGroup(0, object.bindGroup);
		pass.setVertexBuffer(0, object.mesh.vertexBuffer);
		pass.setVertexBuffer(1, object.mesh.normalBuffer);
		pass.setIndexBuffer(object.mesh.indexBuffer, "uint16");
		pass.drawIndexed(object.mesh.indexCount);
	}

	// The single entry point that draws this scene into whatever render pass is currently open, using
	// whatever `window.gpuCamera.view`/`projection` currently hold. It never begins or ends a pass,
	// never sets a viewport/scissor, and never binds a pipeline other than the one it is told to --
	// the caller (this application's own animation loop below, or the adaptation rendering a user's
	// eye) owns the pass itself, exactly so an external caller can safely redirect where this draws
	// without this function fighting back every frame. `usePipeline` lets the depth-capture pass (see
	// the kit's lib/webgpu-render-frame-handler@1.js) reuse this exact draw sequence with the depth-only
	// pipeline instead of the normal shaded one. `viewportRect` is accepted and ignored by this real
	// application (ordinary 3D geometry does not need to know its own destination rectangle to draw
	// correctly) -- it exists purely so a small, deterministic fixture page can implement the same
	// `window.gpuDraw(pass, pipeline, viewportRect)` contract while actually needing that rectangle
	// (see qar-cef-browser/test/WebAppSourceWebGpuReferenceApp_integration_test.cpp's own
	// `PackedDepthMatchesColorSpatially` fixture), since WebGPU, unlike WebGL, has no way to query a
	// render pass's own currently-set viewport back out.
	window.gpuDraw = function drawScene(pass, usePipeline, viewportRect) {
		pass.setPipeline(usePipeline || colorPipeline);
		drawObject(pass, ground);
		for (const object of objects) {
			if (!object.visible) {
				continue;
			}
			const angle = object.baseAngle + sceneRotation;
			modelFromPositionYaw(object.model, [Math.cos(angle) * orbitRadius, 0, Math.sin(angle) * orbitRadius], sceneRotation);
			drawObject(pass, object);
		}
	};

	function renderOwnFrame() {
		const depthView = ensureDepthTexture(canvas.width, canvas.height);
		const encoder = device.createCommandEncoder();
		// `loadOp`/`depthLoadOp: "load"`, not "clear" -- see `clearQuadPipeline`'s own doc comment on
		// why a genuine full-attachment clear here would periodically wipe out every connected user's
		// own rendered destination elsewhere on the same shared surface (this application's own
		// render loop free-runs continuously, independent of and interleaved with any render-frame
		// request an adaptation is servicing). The clear-quad draw below, issued *after* `setViewport`
		// -- which an installed adaptation's canvas-takeover confinement redirects to the reserved
		// page region, see the kit's lib/webgpu-capture-context@1.js -- reproduces a scoped clear instead,
		// confined to whatever rectangle this application is actually allowed to draw into.
		const pass = encoder.beginRenderPass({
			colorAttachments: [
				{
					view: context.getCurrentTexture().createView(),
					loadOp: "load",
					storeOp: "store",
				},
			],
			depthStencilAttachment: { view: depthView, depthLoadOp: "load", depthStoreOp: "store" },
		});
		// This is the one call an adaptation script has to intercept to take canvas ownership: with no
		// adaptation installed it simply sizes the viewport to the whole canvas, exactly as it looks
		// here. See the kit's lib/webgpu-capture-context@1.js's own doc comment for how (and why) it does that
		// without any cooperation from this application at all.
		pass.setViewport(0, 0, canvas.width, canvas.height, 0, 1);
		pass.setPipeline(clearQuadPipeline);
		pass.draw(3);
		window.gpuDraw(pass);
		pass.end();
		device.queue.submit([encoder.finish()]);
	}

	function resize() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		context.configure({ device, format, alphaMode: "opaque" });
		updateDefaultCamera();
	}
	window.addEventListener("resize", resize);
	resize();

	// A one-time, genuine full-attachment clear (`loadOp: "clear"`) at startup: `renderOwnFrame`'s own
	// per-frame passes deliberately never do this again (see its own doc comment), so the surface's
	// very first frame -- and this application's freshly created depth texture, whose initial content
	// is otherwise unspecified -- needs an explicit one-shot initialization to a known state.
	{
		const initialDepthView = ensureDepthTexture(canvas.width, canvas.height);
		const encoder = device.createCommandEncoder();
		const pass = encoder.beginRenderPass({
			colorAttachments: [
				{
					view: context.getCurrentTexture().createView(),
					clearValue: { r: 0.03, g: 0.035, b: 0.04, a: 1 },
					loadOp: "clear",
					storeOp: "store",
				},
			],
			depthStencilAttachment: {
				view: initialDepthView,
				depthClearValue: 1.0,
				depthLoadOp: "clear",
				depthStoreOp: "store",
			},
		});
		pass.end();
		device.queue.submit([encoder.finish()]);
	}

	// --- Left panel: the scene's own object list, with a visibility toggle per entry. ---

	const objectList = document.querySelector("#object-list");
	for (const object of objects) {
		const item = document.createElement("li");
		const checkbox = document.createElement("input");
		checkbox.type = "checkbox";
		checkbox.checked = true;
		checkbox.addEventListener("change", () => {
			object.visible = checkbox.checked;
			pushToast(`${object.name} ${checkbox.checked ? "shown" : "hidden"}`);
		});
		const label = document.createElement("span");
		label.textContent = object.name;
		item.append(checkbox, label);
		objectList.append(item);
	}

	// --- Right panel: settings that actually drive the scene. ---

	const rotationSlider = document.querySelector("#rotation-speed");
	rotationSlider.addEventListener("input", () => {
		rotationSpeed = Number(rotationSlider.value) / 100;
	});

	const lightSlider = document.querySelector("#light-intensity");
	lightSlider.addEventListener("input", () => {
		lightIntensity = (Number(lightSlider.value) / 100) * 2;
	});

	// --- Modal popup, opened automatically once and reopenable from the settings panel. ---

	const modalBackdrop = document.querySelector("#modal-backdrop");
	document.querySelector("#modal-close").addEventListener("click", () => {
		modalBackdrop.classList.add("hidden");
	});
	document.querySelector("#open-about").addEventListener("click", () => {
		modalBackdrop.classList.remove("hidden");
	});
	setTimeout(() => modalBackdrop.classList.remove("hidden"), 300);

	// --- Toast stack: notifications the app raises on its own, plus ones the user triggers above. ---

	const toastStack = document.querySelector("#toast-stack");
	const maxToasts = 4;

	function pushToast(message) {
		const toast = document.createElement("div");
		toast.className = "toast";
		toast.textContent = message;
		toastStack.append(toast);
		while (toastStack.children.length > maxToasts) {
			toastStack.firstElementChild.remove();
		}
		setTimeout(() => toast.remove(), 6000);
	}

	pushToast("Scene loaded");
	let toastCounter = 0;
	const toastMessages = ["Buffers uploaded", "Pipelines compiled", "Camera idle", "Adapter warm"];
	setInterval(() => {
		pushToast(toastMessages[toastCounter % toastMessages.length]);
		toastCounter += 1;
	}, 2500);

	// --- HUD overlay: drawn as ordinary DOM on top of the canvas, never part of its pixels. ---

	const hudFps = document.querySelector("#hud-fps");
	let frameCount = 0;
	let fpsWindowStart = performance.now();

	function updateHud(now) {
		frameCount += 1;
		const elapsed = now - fpsWindowStart;
		if (elapsed >= 500) {
			hudFps.textContent = Math.round((frameCount * 1000) / elapsed).toString();
			frameCount = 0;
			fpsWindowStart = now;
		}
	}

	// --- The render loop itself: an ordinary requestAnimationFrame loop over WebGPU calls. ---

	let lastTime = performance.now();
	function frame(now) {
		const deltaSeconds = Math.min((now - lastTime) / 1000, 0.1);
		lastTime = now;
		sceneRotation += rotationSpeed * deltaSeconds;

		renderOwnFrame();

		updateHud(now);
		requestAnimationFrame(frame);
	}
	requestAnimationFrame(frame);
}

main().catch((error) => {
	console.error("Voxel Atelier failed to start:", error);
	document.body.textContent = `WebGPU failed to start: ${error && error.message ? error.message : error}`;
});
