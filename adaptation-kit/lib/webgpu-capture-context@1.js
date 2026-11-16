// @qaros-library webgpu-capture-context@1.0
// @stage preDocument
//
// Canvas takeover for a raw WebGPU page (no engine), the capture half of the WebGPU reference app's
// adaptation (examples/webgpu-reference-app); the WebGPU-shaped twin of webgl-capture-context@1. It
// intercepts:
//
//   1. Acquiring the rendering context: `HTMLCanvasElement.prototype.getContext` is patched for every
//      canvas on the page.
//   2. Sizing its viewport to the whole canvas: a `GPURenderPassEncoder` is a fresh object per pass,
//      so `GPURenderPassEncoder.prototype.setViewport` itself is patched, confining every
//      unsuspended viewport call to the page region, paired with a `setScissorRect` to the same
//      rectangle (a WebGPU viewport alone is not reliably a clip). A render-frame request suspends
//      this (`state.suspendConfinement`, set by webgpu-render-frame-handler@1) to reach the real
//      per-eye viewport.
//
// Per-eye rendering (webgpu-render-frame-handler@1) needs `window.gpuDevice` from the application:
// a `GPUCanvasContext` has no accessor back to the device that configured it. `window.gpuCamera`
// and `window.gpuDraw` are the draw entry point and camera, as in the WebGL reference app.
(() => {
	const pageRegion = { width: __PAGE_REGION_WIDTH__, height: __PAGE_REGION_HEIGHT__ };

	// Reused rather than replaced if this script runs more than once against the same document
	// (injected after load as well as before the document, as a test harness may do).
	window.__qarAdaptationState = window.__qarAdaptationState || {
		canvas: null,
		context: null,
		// Set around a render-frame request's own draws so the very same patched `setViewport` does
		// not re-confine the surface to the page region while a request is being serviced -- see
		// the render-frame handler.
		suspendConfinement: false,
	};
	const state = window.__qarAdaptationState;
	window.__qarAdaptationPageRegion = pageRegion;

	function confinedRect(canvas) {
		// WebGPU viewports and scissors start at the top-left (WebGL's start at the bottom-left), and
		// the page region is the surface's top-left rectangle.
		return { x: 0, y: 0, width: pageRegion.width, height: pageRegion.height };
	}

	if (typeof GPURenderPassEncoder !== "undefined" && !GPURenderPassEncoder.prototype.__qarViewportPatched) {
		const originalSetViewport = GPURenderPassEncoder.prototype.setViewport;
		GPURenderPassEncoder.prototype.setViewport = function (x, y, width, height, minDepth, maxDepth) {
			if (state.suspendConfinement || !state.canvas) {
				return originalSetViewport.call(this, x, y, width, height, minDepth, maxDepth);
			}
			const rect = confinedRect(state.canvas);
			originalSetViewport.call(this, rect.x, rect.y, rect.width, rect.height, minDepth, maxDepth);
			// Belt-and-suspenders alongside the viewport confinement above -- see this file's own top
			// comment on why a plain `setViewport` call is not trusted alone to clip fragments the way
			// `gl.scissor` explicitly does in WebGL. Calls the pass's own *real*, unpatched
			// `setScissorRect` directly; this file never replaces that method, so there is no
			// recursion risk.
			return this.setScissorRect(rect.x, rect.y, rect.width, rect.height);
		};
		GPURenderPassEncoder.prototype.__qarViewportPatched = true;
	}

	function patchContext(context, canvas) {
		if (!context || context.__qarCaptured) {
			return;
		}
		state.canvas = canvas;
		state.context = context;
		context.__qarCaptured = true;
	}

	const originalGetContext = HTMLCanvasElement.prototype.getContext;
	HTMLCanvasElement.prototype.getContext = function (type, attributes) {
		const context = originalGetContext.call(this, type, attributes);
		if (context && type === "webgpu") {
			try {
				patchContext(context, this);
			} catch (error) {
				console.error("QAR adaptation: failed to patch the acquired webgpu context", error);
			}
		}
		return context;
	};

	// The prototype patches above only ever see *future* calls -- worthless if this script somehow
	// runs after the application's own script already acquired its context (the
	// after-load-monkeypatch degraded mode this file's dual-mode design also serves, see this file's
	// earlier doc comment). This scan closes that gap the same way `ProbePageEngine`'s own canvas
	// inventory, and the raw-WebGL reference application's own retroactive scan, already do, and
	// inherits the same best-guess limitation: a canvas that genuinely has no context yet gets one
	// *forced* onto it by this very call. Acceptable here for the same reason it is acceptable there
	// -- this only ever runs once, at the point a document's scripts have already finished setting up
	// their own canvases.
	if (!state.context) {
		for (const canvas of document.querySelectorAll("canvas")) {
			try {
				const context = originalGetContext.call(canvas, "webgpu");
				if (context) {
					patchContext(context, canvas);
					break;
				}
			} catch (error) {
				console.error("QAR adaptation: failed to probe an existing canvas for a webgpu context", error);
			}
		}
	}
})();
