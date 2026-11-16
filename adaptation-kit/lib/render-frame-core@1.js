// @qaros-library render-frame-core@1.1
// @stage afterLoad
//
// Engine-agnostic core every render-frame handler builds on: nothing in this file mentions
// three.js, WebGL or WebGPU. It owns the boilerplate every engine's handler needs regardless of how
// it draws:
// - acknowledging or failing a `qaros-render-frame` request, so native code never waits on a
//   request nothing will answer (`withRenderFrameAcknowledgement`);
// - the request tag that lets native code find the paint carrying a request (`writeRenderFrameTag`,
//   WebGL; a WebGPU handler writes the same two pixels with `queue.writeTexture`);
// - continuous pacing on Chromium's animation tick (`createRenderFrameLoop`): at most the profile's
//   `RenderRate` frames per second (`detail.renderRate`, 1 to 120), 30 when it sets none (since
//   1.1; 1.0 drew up to 60);
// - since 1.1: one handler per page out of several listed (`addRenderFrameHandler`), and the
//   stream surface a handler that keeps the page untouched publishes its atlas to (`streamSurface`).
//
// Inject it before any handler script.
(() => {
	/// Runs `work` synchronously against one render-frame request's `detail`, then acknowledges it.
	/// A rendered frame is only actually visible to CEF's next paint, not the instant `work` returns
	/// -- the double `requestAnimationFrame` gives the compositor two frames to pick it up before
	/// `completeRenderFrame` tells native code the frame is ready to read back. A thrown error -- a
	/// bad camera, a missing renderer, anything -- fails the request instead of silently publishing
	/// a stale or partial frame; native code marks that cycle's frames invalid and moves on.
	///
	/// When `work` returns true it has tagged its frame (`writeRenderFrameTag`): native code then
	/// recognises the exact paint carrying this request by its tag, so the request is acknowledged
	/// at once instead of after two animation frames.
	function withRenderFrameAcknowledgement(detail, work) {
		function complete(tagged) {
			if (tagged === true) window.qaros.completeRenderFrame(detail.requestId);
			else requestAnimationFrame(() => requestAnimationFrame(() => window.qaros.completeRenderFrame(detail.requestId)));
			return true;
		}
		function fail(error) {
			console.error("QAR adaptation: render-frame request failed", error);
			window.qaros.failRenderFrame(detail.requestId, error);
			return false;
		}
		try {
			const result = work();
			return result && typeof result.then === "function" ? result.then(complete, fail) : complete(result);
		} catch (error) { return fail(error); }
	}

	/// Writes the request's tag into `detail.tag` (the atlas's reserved last row): the request id's
	/// low 24 bits as one pixel's RGB and their complement as the next, so native code can find the
	/// paint that carries this request and reject one that does not. Raw scissored clears on the
	/// default framebuffer, so the bytes are exact -- no blending, colour management or sampling.
	/// Returns false when there is no tag to write; an engine that wraps `gl` must reset its own
	/// state cache afterwards (three.js: `renderer.resetState()`).
	function writeRenderFrameTag(gl, detail) {
		const tag = detail.tag;
		if (!gl || !tag || tag.width < 2) {
			return false;
		}
		const id = Number(BigInt(detail.requestId) & 0xffffffn);
		const values = [id, ~id & 0xffffff];
		const flippedY = gl.drawingBufferHeight - tag.y - 1;
		gl.bindFramebuffer(gl.FRAMEBUFFER, null);
		gl.colorMask(true, true, true, true);
		gl.enable(gl.SCISSOR_TEST);
		if (detail.continuous) {
			gl.scissor(detail.atlas.width - 4, flippedY, 4, 1);
			gl.clearColor(0, 0, 0, 0);
			gl.clear(gl.COLOR_BUFFER_BIT);
		}
		values.forEach((value, index) => {
			gl.scissor(tag.x + index, flippedY, 1, 1);
			gl.clearColor((value & 0xff) / 255, ((value >> 8) & 0xff) / 255, ((value >> 16) & 0xff) / 255, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
		});
		return true;
	}

	/// The frames per second a request asks for: the profile's `RenderRate` (`detail.renderRate`),
	/// clamped to 1..120, or `defaultRenderRate` (30) when the profile sets none.
	function renderRateOf(detail, defaultRenderRate = 30) {
		const rate = detail && detail.renderRate > 0 ? detail.renderRate : defaultRenderRate;
		return Math.min(Math.max(rate, 1), 120);
	}

	/// Whether a draw is due at `timestamp` (ms) after one at `lastDraw`: a tick is skipped until a
	/// whole interval of `renderRateOf(detail)` passed, less a few milliseconds of timer jitter, so
	/// 15 at 60 Hz ticks draws every fourth tick, 60 at 120 Hz every second one, and a 60 Hz tick
	/// arriving slightly early is not skipped for a whole frame.
	function isRenderDue(detail, timestamp, lastDraw, defaultRenderRate = 30) {
		if (!Number.isFinite(timestamp)) return true;
		const interval = 1000 / renderRateOf(detail, defaultRenderRate);
		return timestamp - lastDraw >= interval - Math.min(interval * 0.25, 4);
	}

	// Camera updates are a mailbox. Rendering follows Chromium's tick, never its paint reply.
	/// `defaultRenderRate`: frames per second when the request carries no `renderRate`.
	function createRenderFrameLoop(work, defaultRenderRate = 30) {
		let latest = null, acknowledged = null, queued = false, stopped = false;
		let lastDraw = -Infinity, busy = false;
		function tick(timestamp) {
			queued = false;
			if (stopped || !latest) return;
			// Keep the clock running while asynchronous work finishes: waiting for its promise to
			// request the next frame lost a whole tick whenever it resolved after that frame began,
			// halving the frame rate under load.
			if (busy) { schedule(); return; }
			if (!isRenderDue(latest, timestamp, lastDraw, defaultRenderRate)) { schedule(); return; }
			lastDraw = timestamp;
			const detail = latest;
			let result;
			try {
				if (acknowledged !== detail) {
					result = withRenderFrameAcknowledgement(detail, () => work(detail));
					acknowledged = detail;
				} else result = work(detail);
			} catch (error) { window.qaros.failRenderFrame(detail.requestId, error); latest = null; }
			function resume(success) {
				busy = false;
				if (success === false && latest === detail) latest = null;
				if (latest && latest.continuous) schedule();
			}
			if (result && typeof result.then === "function") {
				busy = true;
				if (latest && latest.continuous) schedule();
				result.then(resume, error => { window.qaros.failRenderFrame(detail.requestId, error); resume(false); });
			} else resume(result);
		}
		function schedule() {
			if (!queued && !stopped) { queued = true; requestAnimationFrame(tick); }
		}
		return Object.freeze({
			update(detail) {
				if (stopped) return;
				if (!detail.continuous) {
					latest = null;
					withRenderFrameAcknowledgement(detail, () => work(detail));
					return;
				}
				latest = detail;
				schedule();
			},
			stop() { stopped = true; latest = null; },
			pause() { latest = null; },
		});
	}

	// Several handlers for one page, one of them answering: a three.js profile lists the WebGL and
	// the WebGPU render frame, and each request goes to the first registered handler whose
	// `accepts(detail)` is true (the one for the renderer the page actually created). When none
	// accepts, the first handler gets the request and reports its own, specific error. The
	// registry lives on `window`, so a handler injected again replaces its earlier registration
	// (by `name`) instead of answering twice, and the listeners are installed once per document.
	const registry = window.__qarRenderFrameHandlers || (window.__qarRenderFrameHandlers = { handlers: [], active: null, listening: false });
	function dispatchRenderFrame(detail) {
		const handlers = registry.handlers;
		if (handlers.length === 0) return;
		const accepting = handlers.find((handler) => {
			try { return handler.accepts(detail) === true; } catch { return false; }
		}) || handlers[0];
		if (registry.active && registry.active !== accepting) registry.active.loop.pause();
		registry.active = accepting;
		accepting.loop.update(detail);
	}
	/// `{ name, accepts(detail) -> boolean, loop }`, `loop` from `createRenderFrameLoop`. Returns the
	/// handler's loop. The handler never listens to `qaros-render-frame` itself.
	function addRenderFrameHandler(handler) {
		const index = registry.handlers.findIndex((existing) => existing.name === handler.name);
		if (index >= 0) {
			registry.handlers[index].loop.stop();
			if (registry.active === registry.handlers[index]) registry.active = null;
			registry.handlers[index] = handler;
		} else registry.handlers.push(handler);
		if (!registry.listening) {
			registry.listening = true;
			window.addEventListener("qaros-render-frame", ({ detail }) => dispatchRenderFrame(detail));
			window.addEventListener("qaros-render-stop", () => {
				for (const entry of registry.handlers) entry.loop.pause();
				registry.active = null;
			});
		}
		return handler.loop;
	}

	// The stream surface: a second, independent page (`window.open` of `about:blank` named
	// `__qar_stream_surface__`) whose only content is a `bitmaprenderer` canvas. A handler that keeps
	// the application's own page and canvas untouched (Engine "three.js") draws the atlas elsewhere
	// and transfers a GPU snapshot of it here; native code reads the users' views from this surface
	// and the page preview from the page. One per document, shared by every handler.
	let surfaceWindow = null, surfaceCanvas = null, surfaceContext = null;
	let snapshotsTaken = 0, newestTransferred = 0;
	function surfaceContextFor(width, height) {
		if (!surfaceWindow || surfaceWindow.closed) {
			surfaceWindow = window.open("about:blank", "__qar_stream_surface__");
			if (!surfaceWindow) throw new Error("QAR adaptation: independent streaming surface was blocked.");
			surfaceWindow.document.documentElement.style.cssText = "margin:0;overflow:hidden;background:black";
			surfaceWindow.document.body.style.margin = "0";
			surfaceCanvas = surfaceWindow.document.createElement("canvas");
			surfaceWindow.document.body.appendChild(surfaceCanvas);
			surfaceContext = surfaceCanvas.getContext("bitmaprenderer");
			window.addEventListener("pagehide", () => { if (surfaceWindow && !surfaceWindow.closed) surfaceWindow.close(); });
		}
		if (surfaceCanvas.width !== width) surfaceCanvas.width = width;
		if (surfaceCanvas.height !== height) surfaceCanvas.height = height;
		return surfaceContext;
	}
	const streamSurface = Object.freeze({
		/// Opens the surface (once) and sizes it. Call it at load with 1x1: native code waits for the
		/// first atlas-size paint of the surface, so the compositor must exist before then.
		ensure(width, height) { surfaceContextFor(width, height); },
		/// Publishes `snapshot` (an `ImageBitmap`, or a promise of one) as the `width`x`height` atlas.
		/// Promises resolve out of the loop's way; only the newest one reaches the surface.
		publish(snapshot, width, height) {
			const context = surfaceContextFor(width, height);
			const generation = ++snapshotsTaken;
			Promise.resolve(snapshot).then((bitmap) => {
				if (generation < newestTransferred || !surfaceWindow || surfaceWindow.closed) {
					bitmap.close();
					return;
				}
				newestTransferred = generation;
				context.transferFromImageBitmap(bitmap);
			}, (error) => console.error("QAR adaptation: stream surface snapshot failed", error));
		},
	});

	window.__qarAdaptationCore = Object.freeze({
		addRenderFrameHandler,
		createRenderFrameLoop,
		isRenderDue,
		renderRateOf,
		streamSurface,
		withRenderFrameAcknowledgement,
		writeRenderFrameTag,
	});
})();
