// Stage: afterLoad, after render-frame-core@1 and webgl-atlas-pack@1. Needs webgl-capture-context@1
// (preDocument), which captured the page's WebGL2 context in `__qarAdaptationState.gl`.
//
// The render-frame handler of this adaptation. For every `qaros-render-frame` request it renders
// every user's eyes into one offscreen colour + depth pass of webgl-atlas-pack@1, which copies colour
// and packs depth (and alpha) into the atlas, then tags the paint. The kit libraries do the protocol,
// packing and pacing; this file owns only how the page draws one eye.
//
// EDIT `drawEye` below: draw the page's scene once with the given camera into the bound framebuffer.
// The usual way is to call the page's own draw function with its camera matrices swapped for the
// eye's (find them with the Visualizer's authoring tools: heap snapshot, live evaluation). Each eye
// gets fresh matrices: a reused, mutated array makes every user see the first user's camera.
(() => {
	const core = window.__qarAdaptationCore;
	const state = (window.__qarAdaptationState = window.__qarAdaptationState || {});
	// Scene units per protocol (room) unit. AppScale in the profile already scales positions and
	// near/far into scene units; use this only when the page's own matrices need another factor.
	const SCENE_SCALE = 1;

	// Draws the page's scene for one eye. `gl`: the page's context, framebuffer and viewport already
	// set to the eye's rectangle. `camera`: { view, projection } (column-major Float32Array(16),
	// standard WebGL depth: near 0, far 1), `eye`: { name, width, height, protocolCamera }.
	function drawEye(gl, camera, eye) {
		if (typeof window.qarDrawEye === "function") return window.qarDrawEye(gl, camera, eye);
		throw new Error("QAR adaptation: edit drawEye in scripts/render-frame.js to draw the page's scene for one eye");
	}

	// Asymmetric perspective projection from the request's field of view (radians).
	function projection(fov, near, far) {
		const n = near * SCENE_SCALE, f = far * SCENE_SCALE;
		const l = Math.tan(fov.angleLeft) * n, r = Math.tan(fov.angleRight) * n;
		const t = Math.tan(fov.angleUp) * n, b = Math.tan(fov.angleDown) * n;
		return new Float32Array([2 * n / (r - l), 0, 0, 0, 0, 2 * n / (t - b), 0, 0,
			(r + l) / (r - l), (t + b) / (t - b), -(f + n) / (f - n), -1, 0, 0, -2 * f * n / (f - n), 0]);
	}

	// World-to-camera matrix of a pose { position, orientation [x, y, z, w] }: the inverse of the
	// rigid camera-to-world transform.
	function viewMatrix({ position, orientation: [x, y, z, w] }) {
		const p = position.map((value) => value * SCENE_SCALE);
		// Rows of the camera-to-world rotation = columns of its inverse.
		const r00 = 1 - 2 * (y * y + z * z), r01 = 2 * (x * y - z * w), r02 = 2 * (x * z + y * w);
		const r10 = 2 * (x * y + z * w), r11 = 1 - 2 * (x * x + z * z), r12 = 2 * (y * z - x * w);
		const r20 = 2 * (x * z - y * w), r21 = 2 * (y * z + x * w), r22 = 1 - 2 * (x * x + y * y);
		return new Float32Array([r00, r01, r02, 0, r10, r11, r12, 0, r20, r21, r22, 0,
			-(r00 * p[0] + r10 * p[1] + r20 * p[2]), -(r01 * p[0] + r11 * p[1] + r21 * p[2]), -(r02 * p[0] + r12 * p[1] + r22 * p[2]), 1]);
	}

	// One row per user in the offscreen pass; `camera.viewport` is the eye's rectangle there, y from
	// the bottom (what copyEyesToAtlas reads). A depth view with eye "none" folds into the left eye.
	function collectEyes(targets) {
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
				eye.protocolCamera = target.cameras[eye.name];
				eye.camera = { viewport: { x: rowX, y: rowY, z: eye.width, w: eye.height } };
				rowX += eye.width;
				rowHeight = Math.max(rowHeight, eye.height);
				eyes.push(eye);
			}
			width = Math.max(width, rowX);
			rowY += rowHeight;
		}
		return { eyes, width, height: rowY };
	}

	let packer = null;

	function render(detail) {
		const gl = state.gl;
		if (!gl) throw new Error("QAR adaptation: no WebGL2 context captured yet; the page has not rendered");
		packer = packer || window.__qarWebglAtlasPack.createAtlasPacker();
		const { eyes, width, height } = collectEyes(detail.targets);
		state.suspendConfinement = true;
		try {
			if (eyes.length > 0) {
				const pass = packer.ensurePass(gl, width, height);
				gl.bindFramebuffer(gl.FRAMEBUFFER, pass.framebuffer);
				gl.disable(gl.SCISSOR_TEST);
				gl.colorMask(true, true, true, true);
				gl.depthMask(true);
				gl.clearColor(0, 0, 0, 1);
				gl.clearDepth(1);
				gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
				for (const eye of eyes) {
					const { x, y, z, w } = eye.camera.viewport;
					gl.viewport(x, y, z, w);
					const c = eye.protocolCamera;
					drawEye(gl, { view: viewMatrix(c.pose), projection: projection(c.fov, c.nearFar.near, c.nearFar.far) }, eye);
				}
				packer.copyEyesToAtlas(gl, pass, detail.atlas.height, eyes);
			}
		} finally {
			state.suspendConfinement = false;
		}
		return core.writeRenderFrameTag(gl, detail);
	}

	// For tests and the authoring tools.
	state.renderFrame = { projection, viewMatrix, collectEyes, render };

	// Continuous pacing (profile `ContinuousRendering: true`): every animation tick redraws every
	// user from the newest cameras and tags the paint.
	if (state.renderFrameLoop) state.renderFrameLoop.stop();
	const loop = (state.renderFrameLoop = core.createRenderFrameLoop(render));
	window.addEventListener("qaros-render-frame", ({ detail }) => loop.update(detail));
	window.addEventListener("qaros-render-stop", () => loop.pause());
})();
