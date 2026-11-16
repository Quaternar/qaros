// Stage: afterLoad, after render-frame-core@1 and before webgpu-render-frame-handler@1.
//
// webgpu-render-frame-handler@1 draws every user's eyes by calling the page's own WebGPU entry
// points under fixed names on `window`. This bridge publishes them under those names, read live
// from wherever the page keeps them. A page written for QAROS may set the names itself; this script
// then leaves them alone.
//
// EDIT `sources` below: each getter returns the page's value, or undefined until the page has it.
//
// | Name | What the handler needs |
// |---|---|
// | gpuDevice | the GPUDevice the page configured its canvas with |
// | gpuCanvasFormat | the canvas's texture format |
// | gpuCamera | an object whose `view` and `projection` (column-major Float32Array(16), WebGPU [0, 1] depth) the draw reads; the handler swaps them per eye |
// | gpuDraw(pass, pipeline, rect) | records the scene's draw calls into `pass` with `pipeline`, viewport `rect` {x, y, width, height} |
// | gpuColorPipeline | the page's colour pipeline |
// | gpuDepthOnlyPipeline | the same vertex stage with no fragment stage, writing depth24plus |
// | gpuEnsureDepthTexture(width, height) | a depth texture view of that size (made once, reused) |
(() => {
	const state = (window.__qarAdaptationState = window.__qarAdaptationState || {});
	const app = () => window.app || {};

	const sources = {
		gpuDevice: () => app().device,
		gpuCanvasFormat: () => app().format,
		gpuCamera: () => app().camera,
		gpuDraw: () => app().draw,
		gpuColorPipeline: () => app().colorPipeline,
		gpuDepthOnlyPipeline: () => app().depthOnlyPipeline,
		gpuEnsureDepthTexture: () => app().ensureDepthTexture,
	};
	const kinds = {
		gpuDevice: "object",
		gpuCanvasFormat: "string",
		gpuCamera: "object",
		gpuDraw: "function",
		gpuColorPipeline: "object",
		gpuDepthOnlyPipeline: "object",
		gpuEnsureDepthTexture: "function",
	};

	for (const [name, get] of Object.entries(sources)) {
		const own = Object.getOwnPropertyDescriptor(window, name);
		if (own && ("get" in own || own.value !== undefined)) continue; // the page publishes it itself
		Object.defineProperty(window, name, { configurable: true, enumerable: true, get });
	}

	// The names the page does not provide (yet).
	function missing() {
		return Object.keys(kinds).filter((name) => {
			const value = window[name];
			return value === undefined || value === null || typeof value !== kinds[name];
		});
	}
	state.bridge = { missing };

	// Report once if the page still lacks an entry point a few seconds after load, so the failing
	// name shows in the Visualizer's events instead of a generic handler error per request.
	let frames = 0;
	function watch() {
		if (missing().length === 0) return;
		if (++frames < 300) {
			requestAnimationFrame(watch);
			return;
		}
		console.error(`QAR adaptation: the page does not provide ${missing().join(", ")}; edit scripts/app-bridge.js`);
	}
	if (typeof requestAnimationFrame === "function") requestAnimationFrame(watch);
})();
