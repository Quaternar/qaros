// @qaros-library webgpu-render-frame-handler@1.0
// @stage afterLoad
// @needs render-frame-core@1 webgpu-capture-context@1
//
// The render-frame half of the WebGPU reference app's adaptation (examples/webgpu-reference-app):
// drives the application's own `window.gpuDraw(pass, pipeline, viewportRect)` entry point with a
// view/projection matrix pair per requested eye. It never constructs geometry, loads a scene or
// owns any application state of its own. An example of a hand-written handler for one
// application: copy and adapt it. A WebGPU page with its own render loop that exposes nothing uses
// webgpu-redirect-capture@1 and webgpu-redirect-presenter@1 instead.
//
// It runs as an injected classic script, so the few matrix helpers it needs are self-contained.
//
// Uniform buffers: `device.queue.writeBuffer` copies bytes when it is called, ordered on the queue
// timeline, so one buffer reused by two destinations whose draws are both still unsubmitted gives
// both the second write. Every destination (each target's left eye, right eye and depth) is
// therefore recorded and submitted before the next destination writes its buffers.
//
// Depth: the raw post-projection depth, never linearised. The app's projection maps to WebGPU's
// [0, 1] clip z, and window-space z is in [0, 1] on both APIs, so the convention is "regular" like
// the WebGL reference app's. Depth is rendered at full resolution into a `depth24plus` texture by a
// depth-only pipeline, then packed by a full-screen pass with `textureLoad` (no sampler): half
// linear resolution, each sample the nearest depth of its 2x2 block, two side-by-side tiles of half
// the view's width and a quarter of its height. WebGPU's fragment coordinates start at the
// top-left, like the atlas, so no rectangle is flipped.
(() => {
	const core = window.__qarAdaptationCore;
	const state = window.__qarAdaptationState;

	// --- Self-contained matrix helpers (see this file's own top comment on why these are not
	// imported from the application's src/mat4.js). Mirrors that file's `fromQuaternion`/`viewFromPose` exactly, and
	// its `perspectiveFromAngles` using the same WebGPU zero-to-one third row. ---

	function fromQuaternion(out, x, y, z, w) {
		const xx = x * x;
		const yy = y * y;
		const zz = z * z;
		const xy = x * y;
		const xz = x * z;
		const yz = y * z;
		const wx = w * x;
		const wy = w * y;
		const wz = w * z;
		out[0] = 1 - 2 * (yy + zz);
		out[1] = 2 * (xy + wz);
		out[2] = 2 * (xz - wy);
		out[3] = 0;
		out[4] = 2 * (xy - wz);
		out[5] = 1 - 2 * (xx + zz);
		out[6] = 2 * (yz + wx);
		out[7] = 0;
		out[8] = 2 * (xz + wy);
		out[9] = 2 * (yz - wx);
		out[10] = 1 - 2 * (xx + yy);
		out[11] = 0;
		out[12] = 0;
		out[13] = 0;
		out[14] = 0;
		out[15] = 1;
		return out;
	}

	function viewFromPose(out, position, orientation) {
		const rotation = fromQuaternion(new Float32Array(16), orientation[0], orientation[1], orientation[2], orientation[3]);
		out[0] = rotation[0];
		out[1] = rotation[4];
		out[2] = rotation[8];
		out[3] = 0;
		out[4] = rotation[1];
		out[5] = rotation[5];
		out[6] = rotation[9];
		out[7] = 0;
		out[8] = rotation[2];
		out[9] = rotation[6];
		out[10] = rotation[10];
		out[11] = 0;
		out[15] = 1;
		const px = position[0];
		const py = position[1];
		const pz = position[2];
		out[12] = -(out[0] * px + out[4] * py + out[8] * pz);
		out[13] = -(out[1] * px + out[5] * py + out[9] * pz);
		out[14] = -(out[2] * px + out[6] * py + out[10] * pz);
		return out;
	}

	function perspectiveFromAngles(out, angleLeft, angleRight, angleUp, angleDown, near, far) {
		const left = Math.tan(angleLeft) * near;
		const right = Math.tan(angleRight) * near;
		const top = Math.tan(angleUp) * near;
		const bottom = Math.tan(angleDown) * near;
		const x = (2 * near) / (right - left);
		const y = (2 * near) / (top - bottom);
		const a = (right + left) / (right - left);
		const b = (top + bottom) / (top - bottom);
		const c = far / (near - far);
		const d = (far * near) / (near - far);
		out.set([x, 0, 0, 0, 0, y, 0, 0, a, b, c, -1, 0, 0, d, 0]);
		return out;
	}

	const eyeMatrices = new Map();
	function getEyeMatrices(key) {
		let entry = eyeMatrices.get(key);
		if (!entry) {
			entry = { view: new Float32Array(16), projection: new Float32Array(16) };
			eyeMatrices.set(key, entry);
		}
		return entry;
	}

	function configureCamera(matrices, protocolCamera) {
		const { pose, fov, nearFar } = protocolCamera;
		viewFromPose(matrices.view, pose.position, pose.orientation);
		perspectiveFromAngles(matrices.projection, fov.angleLeft, fov.angleRight, fov.angleUp, fov.angleDown, nearFar.near, nearFar.far);
	}

	function setCamera(matrices) {
		window.gpuCamera.view = matrices.view;
		window.gpuCamera.projection = matrices.projection;
	}

	// --- A tiny, self-contained "clear this scissored rectangle" pipeline. WebGPU's `loadOp: "clear"`
	// always clears the *entire* attachment, not merely the current viewport/scissor -- there is no
	// native scoped clear the way WebGL's `gl.clear()` (with `SCISSOR_TEST` enabled) provides. This
	// draws a full-screen triangle placed at NDC z = 1.0 (the far plane, matching this application's
	// own default depth-clear value) with depth writes forced on and depth testing disabled, so it
	// unconditionally resets both colour and depth within whatever viewport/scissor is currently set
	// -- exactly the scoped-clear effect the other two reference applications get for free from
	// their own graphics API. ---

	let clearQuadPipeline = null;
	function ensureClearQuadPipeline(device, format) {
		if (clearQuadPipeline) {
			return clearQuadPipeline;
		}
		const module = device.createShaderModule({
			label: "qar-clear-quad",
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
		clearQuadPipeline = device.createRenderPipeline({
			layout: "auto",
			vertex: { module, entryPoint: "vertexMain" },
			fragment: { module, entryPoint: "fragmentMain", targets: [{ format }] },
			primitive: { topology: "triangle-list" },
			depthStencil: { format: "depth24plus", depthWriteEnabled: true, depthCompare: "always" },
		});
		return clearQuadPipeline;
	}

	// --- Depth capture: a genuine WebGPU depth-texture attachment (`window.gpuDepthOnlyPipeline`,
	// created once by src/main.js since it needs that pipeline's own bind group layout), not a
	// colour-channel trick -- the raw post-projection value is simply that attachment's own texel,
	// read directly with `textureLoad` (a regular, non-filtering read of a depth texture, needing no
	// sampler at all). A depth-only render pass (no colour attachments) is valid WebGPU as long as a
	// depth/stencil attachment is present. ---

	const depthCaptureTargets = new Map();
	function depthCaptureTarget(device, width, height) {
		const key = `${width}x${height}`;
		let target = depthCaptureTargets.get(key);
		if (target) {
			return target;
		}
		const texture = device.createTexture({
			label: `qar-depth-capture-${key}`,
			size: [width, height],
			format: "depth24plus",
			usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING,
		});
		target = { texture, view: texture.createView() };
		depthCaptureTargets.set(key, target);
		return target;
	}

	let packPipeline = null;
	let packUniformBuffer = null;
	let packBindGroupLayout = null;
	function ensurePackPipeline(device, format) {
		if (packPipeline) {
			return packPipeline;
		}
		const module = device.createShaderModule({
			label: "qar-depth-pack",
			code: `
struct PackParams {
	rectX: f32,
	rectY: f32,
	halfWidth: f32,
	tileHeight: f32,
}
@group(0) @binding(0) var<uniform> params: PackParams;
@group(0) @binding(1) var depthTexture: texture_depth_2d;

@vertex
fn vertexMain(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
	var positions = array<vec2<f32>, 3>(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
	return vec4<f32>(positions[index], 0.0, 1.0);
}

@fragment
fn fragmentMain(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
	let localX = floor(fragCoord.x - params.rectX);
	let localY = floor(fragCoord.y - params.rectY);
	let tileIndex = select(1.0, 0.0, localX < params.halfWidth);
	// Tile 0 (destination x < halfWidth) reads source rows [0, tileHeight); tile 1 reads source rows
	// [tileHeight, 2*tileHeight) -- mirroring the other two reference applications' own GLSL fold
	// (halfY = packedY + uTileHeight for tile 1, not a subtraction, and no separate y-flip, since
	// WGSL's builtin(position) is already top-left-origin -- see this file's own top comment).
	let halfX = select(localX - params.halfWidth, localX, tileIndex < 0.5);
	let halfY = select(localY + params.tileHeight, localY, tileIndex < 0.5);
	// The nearest depth of the 2x2 full-resolution block this sample stands for (regular depth:
	// smallest is nearest). One texel per block drops a feature thinner than the block -- a 1-px
	// line -- wherever it misses that texel, and the mixer then drops its depthless colour.
	let size = vec2<i32>(textureDimensions(depthTexture));
	var rawDepth = 1.0;
	for (var dy = 0; dy < 2; dy++) {
		for (var dx = 0; dx < 2; dx++) {
			let texel = min(vec2<i32>(i32(halfX) * 2 + dx, i32(halfY) * 2 + dy), size - vec2<i32>(1, 1));
			rawDepth = min(rawDepth, textureLoad(depthTexture, texel, 0));
		}
	}
	// min: 16777215.5 is not an f32, it rounds to 2^24 and the far plane would wrap to black.
	let scaled = min(floor(clamp(rawDepth, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0);
	let r = f32(i32(scaled) % 256) / 255.0;
	let g = f32((i32(scaled) / 256) % 256) / 255.0;
	let b = f32((i32(scaled) / 65536) % 256) / 255.0;
	return vec4<f32>(r, g, b, 1.0);
}
`,
		});
		packBindGroupLayout = device.createBindGroupLayout({
			entries: [
				{ binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
				{ binding: 1, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "depth" } },
			],
		});
		packUniformBuffer = device.createBuffer({
			label: "qar-depth-pack-params",
			size: 16,
			usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
		});
		packPipeline = device.createRenderPipeline({
			layout: device.createPipelineLayout({ bindGroupLayouts: [packBindGroupLayout] }),
			vertex: { module, entryPoint: "vertexMain" },
			fragment: { module, entryPoint: "fragmentMain", targets: [{ format }] },
			primitive: { topology: "triangle-list" },
		});
		return packPipeline;
	}

	/// One destination's colour view: an independent `GPUCommandEncoder`, fully recorded and
	/// submitted before returning -- see this file's own top comment on why that submit boundary,
	/// not merely the scissored rectangle, is what keeps per-object uniform buffers reused between
	/// this destination and the next one safe.
	function renderColorView(device, canvas, colorTextureView, rect, cameraMatrices) {
		const encoder = device.createCommandEncoder();
		const depthView = window.gpuEnsureDepthTexture(canvas.width, canvas.height);
		const pass = encoder.beginRenderPass({
			colorAttachments: [{ view: colorTextureView, loadOp: "load", storeOp: "store" }],
			depthStencilAttachment: { view: depthView, depthLoadOp: "load", depthStoreOp: "store" },
		});
		pass.setViewport(rect.x, rect.y, rect.width, rect.height, 0, 1);
		pass.setScissorRect(rect.x, rect.y, rect.width, rect.height);
		pass.setPipeline(ensureClearQuadPipeline(device, window.gpuCanvasFormat));
		pass.draw(3);
		setCamera(cameraMatrices);
		window.gpuDraw(pass, window.gpuColorPipeline, rect);
		pass.end();
		device.queue.submit([encoder.finish()]);
	}

	function renderDepthView(device, canvas, colorTextureView, view, cameraMatrices) {
		const halfWidth = view.logicalWidth / 2;
		const width = view.logicalWidth;
		const height = view.logicalHeight;
		const target = depthCaptureTarget(device, width, height);

		// Pass one: render the scene at full resolution into the depth-only target, so the pack can
		// keep each 2x2 block's nearest depth.
		{
			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [],
				depthStencilAttachment: { view: target.view, depthClearValue: 1.0, depthLoadOp: "clear", depthStoreOp: "store" },
			});
			pass.setViewport(0, 0, width, height, 0, 1);
			setCamera(cameraMatrices);
			window.gpuDraw(pass, window.gpuDepthOnlyPipeline, { x: 0, y: 0, width, height });
			pass.end();
			device.queue.submit([encoder.finish()]);
		}

		// Pass two: fold that depth texture into the packed two-tile encoding, directly into the
		// shared canvas texture at this view's own reserved rectangle.
		{
			const pipeline = ensurePackPipeline(device, window.gpuCanvasFormat);
			const paramsData = new Float32Array([view.rect.x, view.rect.y, halfWidth, view.rect.height]);
			device.queue.writeBuffer(packUniformBuffer, 0, paramsData.buffer, paramsData.byteOffset, paramsData.byteLength);
			const bindGroup = device.createBindGroup({
				layout: packBindGroupLayout,
				entries: [
					{ binding: 0, resource: { buffer: packUniformBuffer } },
					{ binding: 1, resource: target.texture.createView() },
				],
			});
			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [{ view: colorTextureView, loadOp: "load", storeOp: "store" }],
			});
			pass.setViewport(view.rect.x, view.rect.y, view.rect.width, view.rect.height, 0, 1);
			pass.setScissorRect(view.rect.x, view.rect.y, view.rect.width, view.rect.height);
			pass.setPipeline(pipeline);
			pass.setBindGroup(0, bindGroup);
			pass.draw(3);
			pass.end();
			device.queue.submit([encoder.finish()]);
		}
	}

	// Eagerly warms both pipelines as soon as `window.gpuDevice`/`gpuCanvasFormat` become available,
	// polling rather than waiting for the first render-frame request to trigger lazy creation.
	// **Found empirically while testing this application**: creating a `GPURenderPipeline` for the
	// very first time on this build has an observable, non-trivial latency (shader compilation and
	// pipeline-state-object creation are not necessarily instantaneous just because
	// `device.createRenderPipeline` itself returns synchronously) -- large enough, on a cold first
	// request, to occasionally exceed the browser's render timeout and surface as a timed-out
	// render-frame request, or even to disrupt the accelerated-paint pipeline enough to throw
	// downstream. Warming both pipelines here, well before any real request arrives (this script is
	// injected `afterLoad`, so by the time an operator or a test issues the first request, the
	// application has typically already been running for some time), converts that latency into
	// background work instead of first-request latency. It does not remove the need for a caller to
	// poll for rendered content: an early request can still fail or time out.
	(function warmUpPipelines() {
		const device = window.gpuDevice;
		const format = window.gpuCanvasFormat;
		if (!device || !format) {
			setTimeout(warmUpPipelines, 50);
			return;
		}
		try {
			ensureClearQuadPipeline(device, format);
			ensurePackPipeline(device, format);
		} catch (error) {
			console.error("QAR adaptation: failed to warm up render-frame-handler pipelines", error);
		}
	})();

	/// Performs one render-frame request's actual WebGPU work. Broken out from the event listener
	/// below because it must run *inside a `requestAnimationFrame` callback*, not synchronously in
	/// response to the "qaros-render-frame" event -- see that listener's own doc comment for why.
	function performRender(detail) {
		const device = window.gpuDevice;
		const context = state.context;
		const canvas = state.canvas;
		if (!device || !context || !canvas || typeof window.gpuDraw !== "function") {
			throw new Error("QAR adaptation: no captured WebGPU context/draw function yet -- the application has not rendered a frame.");
		}

		// `setCamera` reassigns `window.gpuCamera.view`/`projection` to point at a *different*
		// array object per eye (see its own doc comment on why this is safe for WebGPU's own
		// explicit-byte-copy uniform uploads, unlike the raw-WebGL reference application's own
		// aliasing hazard); saved and restored here so the application's own default camera --
		// which `updateDefaultCamera` mutates in place on resize, expecting `window.gpuCamera.view`/
		// `projection` to still be its own original arrays -- is not left permanently pointing at
		// whichever eye this request last processed, the same restoration the raw-WebGL reference
		// application's handler (webgl-render-frame-handler@1) performs for the identical reason.
		const originalView = window.gpuCamera.view;
		const originalProjection = window.gpuCamera.projection;
		// Acquired *exactly once* per request and reused for every destination. **Found empirically
		// while testing this application**: calling `context.getCurrentTexture()` again mid-request
		// silently invalidates every texture view obtained from an earlier call in the very same
		// request -- an initial implementation called it once per destination and found every
		// destination but the last reading back as untouched (flat black), with no error reported.
		const colorTextureView = context.getCurrentTexture().createView();
		state.suspendConfinement = true;
		try {
			for (const target of detail.targets) {
				const leftMatrices = getEyeMatrices(`${target.id}:left`);
				configureCamera(leftMatrices, target.cameras.left);
				const rightMatrices = getEyeMatrices(`${target.id}:right`);
				configureCamera(rightMatrices, target.cameras.right);

				for (const view of target.views) {
					if (view.dataType === "depth") {
						// Each depth view is rendered with its own eye's camera; a single depth view
						// (eye "none") uses the left eye's, as the colour views do.
						renderDepthView(device, canvas, colorTextureView, view, view.eye === "right" ? rightMatrices : leftMatrices);
						continue;
					}
					renderColorView(device, canvas, colorTextureView, view.rect, view.eye === "right" ? rightMatrices : leftMatrices);
				}
			}
		} finally {
			window.gpuCamera.view = originalView;
			window.gpuCamera.projection = originalProjection;
			state.suspendConfinement = false;
		}
	}

	window.addEventListener("qaros-render-frame", ({ detail }) => {
		// Deferred into a `requestAnimationFrame` callback rather than run synchronously in response
		// to the event: a `context.getCurrentTexture()` call made from an arbitrary event handler was
		// observed to reach the accelerated-paint output less reliably than one made from inside an
		// animation frame callback, as the application's own `renderOwnFrame` does. It helps, but is
		// not sufficient on its own against the full reference application: with its own continuous
		// render loop and full DOM chrome, users' views were observed to read back black with no GPU
		// validation error, a limitation of this reference application's full scenario, not of the
		// mechanism (a minimal fixture page streams correctly).
		//
		// Deferring into `requestAnimationFrame` changes `work`'s error-reporting shape slightly:
		// `withRenderFrameAcknowledgement`'s own synchronous try/catch (shared with the other two
		// reference applications, so not modified here) cannot see an error thrown from inside this
		// deferred callback, since by the time it runs, that try/catch has already exited normally.
		// This handler therefore reports a thrown error to `window.qaros.failRenderFrame` directly in
		// that case, which is exactly what the shared core's own catch block would otherwise have
		// done.
		core.withRenderFrameAcknowledgement(detail, () => {
			requestAnimationFrame(() => {
				try {
					performRender(detail);
				} catch (error) {
					console.error("QAR adaptation: deferred render-frame work failed", error);
					window.qaros.failRenderFrame(detail.requestId, error);
				}
			});
		});
	});
})();
