// @qaros-library webgl-capture-context@1.0
// @stage preDocument
//
// Canvas takeover for a raw WebGL2 page (no engine), the capture half of the WebGL reference app's
// adaptation (examples/webgl-reference-app). It runs before any of the application's own scripts,
// the only stage that can intercept how the application acquires its rendering context, and
// intercepts two things every WebGL2 application does whatever library it uses:
//
//   1. Acquiring its rendering context: `HTMLCanvasElement.prototype.getContext` is patched for every
//      canvas on the page, so the adaptation needs no cooperation from the application and no
//      global it exposes.
//   2. Sizing its viewport to the whole canvas every frame: the captured context instance's own
//      `viewport` (never `WebGL2RenderingContext.prototype`) confines every viewport call the
//      application makes to the page region, unless a render-frame request has suspended that
//      confinement (`state.suspendConfinement`, set by webgl-render-frame-handler@1).
//
// Per-eye rendering (webgl-render-frame-handler@1) also needs the application to expose a draw
// entry point (`window.glDraw`) and a camera matrix pair (`window.glCamera`): raw WebGL has no
// scene graph to call a method on. Finding such an entry point in an unfamiliar application is
// what the authoring tools' heap snapshots and live evaluation are for; the reference app exposes
// it plainly.
(() => {
	const pageRegion = { width: __PAGE_REGION_WIDTH__, height: __PAGE_REGION_HEIGHT__ };

	// Reused rather than replaced if this script runs more than once against the same document
	// (injected after load as well as before the document, as a test harness may do).
	window.__qarAdaptationState = window.__qarAdaptationState || {
		canvas: null,
		gl: null,
		originalViewport: null,
		// Set around a render-frame request's own draws so the very same patched `viewport` does not
		// re-confine the surface to the page region while a request is being serviced -- see
		// the render-frame handler.
		suspendConfinement: false,
	};
	const state = window.__qarAdaptationState;
	window.__qarAdaptationPageRegion = pageRegion;

	function confineToPageRegion(gl, canvas, originalViewport) {
		const y = canvas.height - pageRegion.height;
		gl.enable(gl.SCISSOR_TEST);
		originalViewport.call(gl, 0, y, pageRegion.width, pageRegion.height);
		gl.scissor(0, y, pageRegion.width, pageRegion.height);
	}

	function patchContext(gl, canvas) {
		if (!gl || gl.__qarCaptured) {
			return;
		}
		const originalViewport = gl.viewport.bind(gl);
		state.canvas = canvas;
		state.gl = gl;
		state.originalViewport = originalViewport;
		// This is "canvas takeover" for an application with no scene-graph render() call to wrap:
		// every ordinary viewport call the application makes on its own animation-frame loop, forever,
		// is redirected to the reserved page region of the shared surface instead of the whole canvas
		// -- without the application ever being told its viewport changed. A render-frame request
		// suspends this to reach the *real* `gl.viewport`, since it needs the actual per-eye rectangle
		// native code asked for, not the page region.
		gl.viewport = function (x, y, width, height) {
			if (state.suspendConfinement) {
				return originalViewport(x, y, width, height);
			}
			return confineToPageRegion(gl, canvas, originalViewport);
		};
		gl.__qarCaptured = true;
	}

	const originalGetContext = HTMLCanvasElement.prototype.getContext;
	HTMLCanvasElement.prototype.getContext = function (type, attributes) {
		const context = originalGetContext.call(this, type, attributes);
		if (context && (type === "webgl2" || type === "webgl")) {
			try {
				patchContext(context, this);
			} catch (error) {
				console.error("QAR adaptation: failed to patch the acquired rendering context", error);
			}
		}
		return context;
	};

	// The prototype patch above only ever sees *future* `getContext` calls -- worthless if this
	// script somehow runs after the application's own script already acquired its context (the
	// after-load-monkeypatch degraded mode this file's dual-mode design also serves, see this file's
	// earlier doc comment on `window.__qarAdaptationState`). This scan closes that gap the same way
	// `ProbePageEngine`'s own canvas inventory already does (see `PageCanvasProbe`'s doc comment,
	// `qar_cef_browser/CefVulkanBrowser.hpp`), and inherits that same best-guess limitation rather
	// than avoiding it: calling `getContext` again with the type a context was already created with
	// is non-destructive and simply returns that same context, but a canvas that genuinely has no
	// context yet gets one *forced* onto it by this very call, exactly as `PageCanvasProbe`'s own
	// doc comment discloses. Acceptable here for the same reason it is acceptable there -- this only
	// ever runs once, at the point a document's scripts have already finished setting up their own
	// canvases, so a canvas with no context at this point genuinely has none to disturb.
	for (const canvas of document.querySelectorAll("canvas")) {
		for (const type of ["webgl2", "webgl"]) {
			try {
				const context = originalGetContext.call(canvas, type);
				if (context) {
					patchContext(context, canvas);
					break;
				}
			} catch (error) {
				console.error("QAR adaptation: failed to probe an existing canvas for a rendering context", error);
			}
		}
	}
})();
