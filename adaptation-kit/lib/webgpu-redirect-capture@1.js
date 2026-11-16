// @qaros-library webgpu-redirect-capture@1.0
// @stage preDocument
//
// Generic WebGPU capture that makes the adaptation the ONLY caller of the real
// GPUCanvasContext.getCurrentTexture: the app's own loop keeps running but draws into an
// offscreen "app texture"; webgpu-redirect-presenter@1 (afterLoad) copies it into the page region
// each tick. It also records the app's last frame -- every render pass of the command encoder that
// drew into the app texture, with its attachments and commands, and the descriptor of every bind
// group -- so the presenter can replay the whole frame per eye with per-eye cameras and per-eye
// copies of the app's intermediate targets (G-buffers, depth).
//
// A pre-document script runs when the page's context is created, before Chromium exposes the
// WebGPU interfaces (GPUAdapter, GPUCanvasContext, GPUTextureUsage are undefined then). So nothing
// GPU-related is touched at load: HTMLCanvasElement.getContext (always present) installs the GPU
// patches on the first getContext("webgpu"), and the device comes from context.configure().
(() => {
	const S = (window.__qarAdaptationState = window.__qarAdaptationState || {});
	Object.assign(S, { canvas: null, context: null, device: null, format: null, appTexture: null,
		appSize: null, appFrames: 0, realCalls: 0, presenting: false, lastFrame: null,
		buffersByLabel: new Map(), gpuPatched: false });

	function installGpuPatches() {
		if (S.gpuPatched || typeof GPUCanvasContext === "undefined") return;
		S.gpuPatched = true;
		const origConfigure = GPUCanvasContext.prototype.configure;
		GPUCanvasContext.prototype.configure = function (config) {
			if (this === S.context && !S.presenting) {
				S.device = config.device;
				S.format = config.format;
				// COPY_DST: the presenter copies eyes / page region and writes tag bytes into it.
				config = { ...config, usage: (config.usage ?? GPUTextureUsage.RENDER_ATTACHMENT) | GPUTextureUsage.COPY_DST };
			}
			return origConfigure.call(this, config);
		};
		const origGetCurrentTexture = GPUCanvasContext.prototype.getCurrentTexture;
		S.realGetCurrentTexture = (context) => { S.realCalls++; return origGetCurrentTexture.call(context); };
		GPUCanvasContext.prototype.getCurrentTexture = function () {
			if (this !== S.context || S.presenting || !S.device) return origGetCurrentTexture.call(this);
			// The app's size, frozen at its first frame: the presenter later resizes the canvas to
			// the atlas, and the app keeps drawing at the size it was built for.
			if (!S.appSize) S.appSize = [this.canvas.width, this.canvas.height];
			if (!S.appTexture) {
				S.appTexture = S.device.createTexture({ label: "qar-app-texture", size: S.appSize, format: S.format,
					usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC | GPUTextureUsage.TEXTURE_BINDING });
			}
			S.appFrames++;
			return S.appTexture;
		};
		// Neither a view nor a bind group can be asked what it was made from: remember it.
		const origCreateView = GPUTexture.prototype.createView;
		GPUTexture.prototype.createView = function (...args) {
			const view = origCreateView.apply(this, args);
			view.__qarTexture = this;
			view.__qarViewDescriptor = args[0];
			return view;
		};
		const D = GPUDevice.prototype;
		const origCreateBindGroup = D.createBindGroup;
		D.createBindGroup = function (descriptor) {
			const group = origCreateBindGroup.call(this, descriptor);
			if (!S.presenting) group.__qarDescriptor = descriptor;
			return group;
		};
		const origCreateBuffer = D.createBuffer;
		D.createBuffer = function (descriptor) {
			const buffer = origCreateBuffer.call(this, descriptor);
			if (!S.presenting && descriptor && descriptor.label) S.buffersByLabel.set(descriptor.label, buffer);
			return buffer;
		};
		// Record every render pass of an encoder; the frame that drew into the app texture is kept.
		const E = GPUCommandEncoder.prototype;
		const origBegin = E.beginRenderPass;
		E.beginRenderPass = function (descriptor) {
			const pass = origBegin.call(this, descriptor);
			if (!S.presenting) {
				// Attachment views are captured now: apps reuse and mutate their descriptor objects.
				const record = { kind: "render", commands: [],
					colorAttachments: descriptor.colorAttachments.map(a => (a ? { ...a } : a)),
					depthStencilAttachment: descriptor.depthStencilAttachment ? { ...descriptor.depthStencilAttachment } : undefined };
				pass.__qarCommands = record.commands;
				(this.__qarPasses = this.__qarPasses || []).push(record);
			}
			return pass;
		};
		const origFinish = E.finish;
		E.finish = function (...args) {
			const passes = this.__qarPasses;
			if (passes && S.appTexture && passes.some(p => p.colorAttachments.some(a => a && a.view && a.view.__qarTexture === S.appTexture))) {
				S.lastFrame = { passes };
			}
			return origFinish.apply(this, args);
		};
		const P = GPURenderPassEncoder.prototype;
		for (const op of ["setPipeline", "setBindGroup", "setVertexBuffer", "setIndexBuffer", "setViewport", "setScissorRect",
			"setBlendConstant", "setStencilReference", "draw", "drawIndexed", "drawIndirect", "drawIndexedIndirect"]) {
			const orig = P[op];
			if (!orig) continue;
			P[op] = function (...args) {
				// Typed-array arguments (dynamic offsets) are copied: the app may reuse the array.
				if (this.__qarCommands) this.__qarCommands.push([op, args.map(a => (ArrayBuffer.isView(a) ? a.slice() : a))]);
				return orig.apply(this, args);
			};
		}
	}

	const origGetContext = HTMLCanvasElement.prototype.getContext;
	HTMLCanvasElement.prototype.getContext = function (type, attributes) {
		if (type === "webgpu") installGpuPatches();
		const context = origGetContext.call(this, type, attributes);
		if (context && type === "webgpu" && !S.context) { S.canvas = this; S.context = context; }
		return context;
	};
	// Applied later (script_reload through CDP), the interfaces may already exist.
	installGpuPatches();
})();
