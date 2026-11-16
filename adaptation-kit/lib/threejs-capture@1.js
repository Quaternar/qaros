// @qaros-library threejs-capture@1.0
// @stage preDocument
//
// Shared by every three.js application's adaptation profile: it finds the application's own
// renderer (`WebGLRenderer` or `WebGPURenderer`) and the scene it draws without the application
// exposing either, and confines the application's own per-frame render to the reserved page region.
//
// How it finds them: three.js announces every `WebGLRenderer` and `Scene` it constructs to a global
// `__THREE_DEVTOOLS__` event target when one exists (the hook the three.js browser devtools use),
// whether the application imports three.js as a module, bundles it or loads it as a global. This
// script runs before any of the application's own scripts, so it installs that target first and
// receives every instance. `window.renderer`, if an application assigns one, is trapped as well.
//
// The scene the per-user views draw is the one the application itself last rendered with a
// perspective camera, not a fixed global: an application that also draws a HUD scene with an
// orthographic camera is not mistaken for it. A profile whose application needs another choice sets
// `window.__qarAdaptationState.selectScene = (rendered) => scene` in a script of its own; `rendered`
// is this frame's `[{ scene, camera }]` in the order the application drew them.
//
// `WebGLRenderer` assigns `render` as an own instance property in its constructor, so the instance's
// own property is patched, never `WebGLRenderer.prototype.render`. A `WebGPURenderer` is announced
// and patched the same way; threejs-render-frame@1 streams a `WebGLRenderer`,
// threejs-webgpu-render-frame@1 a `WebGPURenderer`.
//
// An application with several renderers (one per panel, say) streams only one of them: a profile
// sets `window.__qarAdaptationState.chooseRenderer = (renderers) => renderer` in a script of its
// own, asked once per animation frame with every captured renderer. Only the chosen one is
// atlas-sized, confined and streamed; the others render untouched. Without it every captured
// renderer is treated as the streamed one, as a single-renderer application needs.
(() => {
	const pageRegion = { width: __PAGE_REGION_WIDTH__, height: __PAGE_REGION_HEIGHT__ };
	window.__qarAdaptationPageRegion = pageRegion;
	let atlasExtent = null;

	const isTopLevel = window === window.top;
	window.__qarResizePage = (width, height, atlasWidth, atlasHeight) => {
		pageRegion.width = width;
		pageRegion.height = height;
		atlasExtent = { width: atlasWidth, height: atlasHeight };
		state.atlasExtent = atlasExtent;

		const renderer = state.renderer;
		if (renderer) {
			const owner = renderer.domElement.ownerDocument?.defaultView;
			renderer.setSize(owner && owner !== window ? owner.innerWidth : width, owner && owner !== window ? owner.innerHeight : height);
		}
		window.dispatchEvent(new Event("resize"));
	};

	let inheritedState = null;
	try { if (!isTopLevel) inheritedState = window.top.__qarAdaptationState; } catch {}
	const state = (window.__qarAdaptationState = inheritedState || window.__qarAdaptationState || {});
	state.renderer = state.renderer || null;
	state.scene = state.scene || null;
	state.camera = state.camera || null;
	state.suspendConfinement = false;
	state.scenes = state.scenes || new Set();
	state.renderers = state.renderers || new Set();

	let streamedRenderer = null;
	let streamedRendererChosen = false;
	function isStreamedRenderer(renderer) {
		if (typeof state.chooseRenderer !== "function") return true;
		if (!streamedRendererChosen) {
			streamedRendererChosen = true;
			streamedRenderer = state.chooseRenderer(state.renderers);
			requestAnimationFrame(() => (streamedRendererChosen = false));
		}
		return streamedRenderer === renderer;
	}

	let renderedThisFrame = [];
	let frameScheduled = false;

	// The canvas's displayed size in backing-store pixels, at the atlas's top-left corner: the
	// reserved page region, which per-user views are never placed in.
	function displayedRegion(renderer) {
		const canvas = renderer.domElement;
		const ratio = typeof renderer.getPixelRatio === "function" ? renderer.getPixelRatio() : 1;
		const cssWidth = Number.isFinite(canvas.clientWidth) ? canvas.clientWidth : pageRegion.width;
		const cssHeight = Number.isFinite(canvas.clientHeight) ? canvas.clientHeight : pageRegion.height;
		const width = Math.min(canvas.width, Math.max(1, Math.round(cssWidth * ratio)));
		const height = Math.min(canvas.height, Math.max(1, Math.round(cssHeight * ratio)));
		return { width, height };
	}

	// Shows only `region` of the backing store, at the canvas's own CSS size, so the page displays
	// the application's view 1:1 while the rest of the buffer holds the atlas. Without a region the
	// whole buffer shows, as on an ordinary page.
	function cropDisplay(canvas, region) {
		const crop = region
			? `inset(0px ${canvas.width - region.width}px ${canvas.height - region.height}px 0px)`
			: "";
		if (canvas.style.objectViewBox !== crop) canvas.style.objectViewBox = crop;
	}

	function confineToPageRegion(renderer, camera) {
		// A gallery's child document owns a normal canvas, not the top-level streaming atlas.
		// Follow the frame's real viewport after the gallery's responsive layout has changed.
		if (!isTopLevel) {
			pageRegion.width = window.innerWidth;
			pageRegion.height = window.innerHeight;
		}
		const canvas = renderer.domElement;
		const height = canvas.height;
		const width = canvas.width;
		renderer.setScissorTest(true);
		const region = isTopLevel && (atlasExtent || state.atlasExtent) ? displayedRegion(renderer) : null;
		if (region) {
			// While streaming, the backing store is the whole atlas. The application's own view
			// needs only what the page shows: rendering it at atlas size made every frame fill
			// 32.8 MP and queued seconds of frames behind the user's input.
			const ratio = typeof renderer.getPixelRatio === "function" ? renderer.getPixelRatio() : 1;
			const y = (height - region.height) / ratio;
			renderer.setViewport(0, y, region.width / ratio, region.height / ratio);
			renderer.setScissor(0, y, region.width / ratio, region.height / ratio);
		} else {
			renderer.setViewport(0, 0, width, height);
			renderer.setScissor(0, 0, width, height);
		}
		cropDisplay(canvas, region);

		const pageAspect = pageRegion.width / pageRegion.height;
		if (camera && camera.isPerspectiveCamera && !camera.isArrayCamera && camera.aspect !== pageAspect) {
			camera.aspect = pageAspect;
			camera.updateProjectionMatrix();
		}
	}

	// Decided once per animation frame, from everything the application drew in it.
	function chooseScene() {
		frameScheduled = false;
		const rendered = renderedThisFrame;
		renderedThisFrame = [];
		if (rendered.length === 0) {
			return;
		}
		let choice = null;
		if (typeof state.selectScene === "function") {
			choice = state.selectScene(rendered);
		}
		if (!choice) {
			const perspective = rendered.findLast(({ camera }) => camera && camera.isPerspectiveCamera);
			choice = (perspective || rendered[rendered.length - 1]).scene;
		}
		const entry = rendered.findLast(({ scene }) => scene === choice);
		state.scene = choice;
		state.camera = entry ? entry.camera : state.camera;
	}

	function adoptRenderer(renderer) {
		if (
			!renderer ||
			renderer.__qarCaptured ||
			typeof renderer.render !== "function" ||
			(typeof renderer.getContext !== "function" && !renderer.isWebGPURenderer)
		) {
			return;
		}
		state.renderers.add(renderer);
		const originalSetSize = renderer.setSize;
		if (typeof originalSetSize === "function") renderer.setSize = function (width, height, updateStyle) {
			// Only the GPU backing store is atlas-sized. DOM geometry, controls and the document
			// retain the application's ordinary CSS viewport and browser input coordinates.
			const extent = atlasExtent || state.atlasExtent;
			if (extent && isStreamedRenderer(this)) {
				originalSetSize.call(this, extent.width, extent.height, false);
				if (updateStyle !== false) {
					this.domElement.style.width = `${width}px`;
					this.domElement.style.height = `${height}px`;
				}
				return;
			}
			return originalSetSize.call(this, width, height, updateStyle);
		};
		// A renderer that stopped being the streamed one (a layout change) gets its own size back.
		function releaseAtlas(self) {
			const canvas = self.domElement;
			const extent = atlasExtent || state.atlasExtent;
			if (!extent || canvas.width !== extent.width || canvas.height !== extent.height) return;
			originalSetSize.call(self, canvas.clientWidth, canvas.clientHeight, false);
			cropDisplay(canvas, null);
			if (state.renderer === self) state.renderer = null;
		}
		const originalRender = renderer.render;
		renderer.render = function (scene, camera) {
			// Environment maps and other intermediate targets own their viewport. Canvas-space
			// confinement clips those smaller targets and can leave every lit material black.
			const canvasPass = typeof this.getRenderTarget !== "function" || this.getRenderTarget() === null;
			if (canvasPass && scene && scene.isScene) this.__qarLastScene = scene;
			if (!state.suspendConfinement && !isStreamedRenderer(this)) {
				releaseAtlas(this);
				return originalRender.call(this, scene, camera);
			}
			if (!state.suspendConfinement && canvasPass) {
				state.renderer = renderer;
				if (scene && scene.isScene) {
					renderedThisFrame.push({ scene, camera });
					if (!frameScheduled) {
						frameScheduled = true;
						requestAnimationFrame(chooseScene);
					}
					if (!state.scene) {
						state.scene = scene;
						state.camera = camera;
					}
				}
				confineToPageRegion(renderer, camera);
			}
			return originalRender.call(this, scene, camera);
		};
		renderer.__qarCaptured = true;
	}

	function observe(object) {
		if (!object) {
			return;
		}
		if (object.isScene) {
			state.scenes.add(object);
			return;
		}
		try {
			adoptRenderer(object);
		} catch (error) {
			console.error("QAR adaptation: failed to patch a three.js renderer", error);
		}
	}

	const devtools =
		typeof window.__THREE_DEVTOOLS__ === "object" && window.__THREE_DEVTOOLS__ instanceof EventTarget
			? window.__THREE_DEVTOOLS__
			: new EventTarget();
	window.__THREE_DEVTOOLS__ = devtools;
	devtools.addEventListener("observe", (event) => observe(event.detail));

	if (window.renderer) {
		observe(window.renderer);
	} else {
		let capturedRenderer;
		Object.defineProperty(window, "renderer", {
			configurable: true,
			enumerable: true,
			get() {
				return capturedRenderer;
			},
			set(value) {
				capturedRenderer = value;
				observe(value);
			},
		});
	}
})();
