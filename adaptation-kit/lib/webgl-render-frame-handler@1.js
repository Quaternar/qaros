// @qaros-library webgl-render-frame-handler@1.0
// @stage afterLoad
// @needs render-frame-core@1 webgl-capture-context@1
//
// The render-frame half of the WebGL reference app's adaptation (examples/webgl-reference-app):
// drives the application's own `window.glDraw()` entry point with a view/projection matrix pair
// per requested eye, into the context webgl-capture-context@1 captured. It never constructs
// geometry, loads a scene or owns any application state of its own. It is an example of a
// hand-written handler for one application, not a general one: copy and adapt it.
//
// It runs as an injected classic script, not an ES module, so the few matrix and shader helpers it
// needs are self-contained here rather than imported from the application.
//
// Depth: the raw post-projection depth of a full-resolution DEPTH_COMPONENT24 texture, never
// linearised; the reference app's projections are standard finite-far perspective, so every
// target's depth convention is "regular". It is packed at half linear resolution in two
// side-by-side tiles of half the view's width and a quarter of its height, each sample the nearest
// depth of its 2x2 block, the same atlas contract as webgl-atlas-pack@1.
(() => {
	const core = window.__qarAdaptationCore;
	const state = window.__qarAdaptationState;

	// --- Self-contained matrix helpers (see this file's own top comment on why these are not
	// imported from the application's src/mat4.js). Mirrors that file's `fromQuaternion`/`viewFromPose`/
	// `perspectiveFromAngles` exactly. ---

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
		const c = -(far + near) / (far - near);
		const d = -(2 * far * near) / (far - near);
		out.set([x, 0, 0, 0, 0, y, 0, 0, a, b, c, -1, 0, 0, d, 0]);
		return out;
	}

	// --- Self-contained shader compilation, checked explicitly for the same reason gl-utils.js's own
	// doc comment gives: WebGL never throws on a GLSL compile error, only logs it, so a silently
	// broken pack shader would present as depth that reads back as all zero with no error anywhere
	// reachable from this script. ---

	function compileShader(gl, type, source) {
		const shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);
		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			const log = gl.getShaderInfoLog(shader);
			gl.deleteShader(shader);
			throw new Error(`QAR adaptation: depth-pack shader failed to compile: ${log}`);
		}
		return shader;
	}

	function createProgram(gl, vertexSource, fragmentSource) {
		const vertexShader = compileShader(gl, gl.VERTEX_SHADER, vertexSource);
		const fragmentShader = compileShader(gl, gl.FRAGMENT_SHADER, fragmentSource);
		const program = gl.createProgram();
		gl.attachShader(program, vertexShader);
		gl.attachShader(program, fragmentShader);
		gl.linkProgram(program);
		gl.deleteShader(vertexShader);
		gl.deleteShader(fragmentShader);
		if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
			const log = gl.getProgramInfoLog(program);
			gl.deleteProgram(program);
			throw new Error(`QAR adaptation: depth-pack program failed to link: ${log}`);
		}
		return program;
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

	function flippedY(atlasHeight, rect) {
		return atlasHeight - rect.y - rect.height;
	}

	// Swaps `window.glCamera`'s own matrix *references* to this eye's matrices, rather than copying
	// their sixteen floats into the pair of arrays `window.glCamera` started life with (which
	// `TypedArray.prototype.set` would do just as correctly on paper). Found empirically, the same
	// way as the atlas row orientation -- by a symptom that looked like a completely
	// different bug: with `.set()`-copying, every connected target's rendered colour and depth came
	// back byte-for-byte *identical* despite each target's own distinct camera being read and used
	// (per-eye `configureCamera` output verified distinct via `Runtime.evaluate`), as if only the
	// very first target's camera had ever taken effect. Every other piece of this application's own
	// state (`sceneRotation`, `object.model`, `object.visible`) is copied or mutated in place exactly
	// as ordinarily as this camera pair once was, and none of it showed the same symptom -- only the
	// values `gl.uniformMatrix4fv` reads on `window.glCamera.view`/`projection` specifically did.
	// Reassigning the two properties to a *different* `Float32Array` object per call, instead of
	// reusing one and rewriting its contents, made the symptom disappear completely and reliably.
	// This module never root-caused *why* -- the honest, disclosed answer is a shrug: whether it is
	// this build's ANGLE/D3D command-buffer layer eliding a uniform re-upload it believes (wrongly)
	// is unchanged because the array's own identity did not change, or something else in this
	// specific combination of CEF/ANGLE/ES3 layers, is not something this reference application's
	// own tests can distinguish. What matters operationally: an adaptation driving a raw WebGL
	// application's own camera uniforms through a *reused, mutated* buffer is not safe to assume
	// correct on this stack, and assigning a fresh object each time is the workaround.
	function setCamera(matrices) {
		window.glCamera.view = matrices.view;
		window.glCamera.projection = matrices.projection;
	}

	function renderColorView(gl, atlasHeight, rect) {
		gl.bindFramebuffer(gl.FRAMEBUFFER, null);
		const y = flippedY(atlasHeight, rect);
		gl.viewport(rect.x, y, rect.width, rect.height);
		gl.enable(gl.SCISSOR_TEST);
		gl.scissor(rect.x, y, rect.width, rect.height);
		gl.clearColor(0, 0, 0, 1);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
		window.glDraw();
	}

	// --- Depth capture: an actual WebGL2 depth-texture-backed framebuffer, not a colour-channel
	// trick -- this application's own `window.glDraw()` already writes ordinary hardware depth via
	// normal depth testing, so the raw post-projection value is simply the depth attachment's own
	// texel, sampled directly (a regular, non-shadow `sampler2D` read of a depth texture is valid
	// GLSL ES 3.00). A small colour renderbuffer is attached alongside the depth texture purely to
	// keep the framebuffer unconditionally complete; its contents are never read. ---

	const depthTargets = new Map();
	function depthCaptureTarget(gl, width, height) {
		const key = `${width}x${height}`;
		let target = depthTargets.get(key);
		if (target) {
			return target;
		}
		const framebuffer = gl.createFramebuffer();
		gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
		const depthTexture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, depthTexture);
		gl.texImage2D(gl.TEXTURE_2D, 0, gl.DEPTH_COMPONENT24, width, height, 0, gl.DEPTH_COMPONENT, gl.UNSIGNED_INT, null);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, depthTexture, 0);
		const colorRenderbuffer = gl.createRenderbuffer();
		gl.bindRenderbuffer(gl.RENDERBUFFER, colorRenderbuffer);
		gl.renderbufferStorage(gl.RENDERBUFFER, gl.RGBA8, width, height);
		gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.RENDERBUFFER, colorRenderbuffer);
		const status = gl.checkFramebufferStatus(gl.FRAMEBUFFER);
		if (status !== gl.FRAMEBUFFER_COMPLETE) {
			throw new Error(`QAR adaptation: depth-capture framebuffer incomplete (status ${status})`);
		}
		gl.bindFramebuffer(gl.FRAMEBUFFER, null);
		target = { framebuffer, depthTexture };
		depthTargets.set(key, target);
		return target;
	}

	let packPipeline = null;
	function ensurePackPipeline(gl) {
		if (packPipeline) {
			return packPipeline;
		}
		const vertexSource = `#version 300 es
in vec2 aPosition;
out vec2 vUv;
void main() {
	vUv = aPosition * 0.5 + 0.5;
	gl_Position = vec4(aPosition, 0.0, 1.0);
}
`;
		// The same fold as webgl-atlas-pack@1's pack shader (this file's top comment says why it is
		// self-contained).
		const fragmentSource = `#version 300 es
precision highp float;
uniform sampler2D uDepthTexture;
uniform float uHalfWidth;
uniform float uTileHeight;
in vec2 vUv;
out vec4 outColor;
void main() {
	float packedWidth = uHalfWidth * 2.0;
	float halfHeight = uTileHeight * 2.0;
	float packedX = floor(vUv.x * packedWidth);
	float packedY = floor((1.0 - vUv.y) * uTileHeight);
	float tileIndex = packedX < uHalfWidth ? 0.0 : 1.0;
	float halfX = tileIndex < 0.5 ? packedX : packedX - uHalfWidth;
	float halfY = tileIndex < 0.5 ? packedY : packedY + uTileHeight;
	// The nearest depth of the 2x2 full-resolution block this sample stands for (regular depth:
	// smallest is nearest). One texel per block drops a feature thinner than the block -- a 1-px
	// line -- wherever it misses that texel, and the mixer then drops its depthless colour.
	ivec2 size = textureSize(uDepthTexture, 0);
	float rawDepth = 1.0;
	for (int dy = 0; dy < 2; ++dy) {
		for (int dx = 0; dx < 2; ++dx) {
			int x = min(int(halfX) * 2 + dx, size.x - 1);
			int y = min(int(halfY) * 2 + dy, size.y - 1);
			rawDepth = min(rawDepth, texelFetch(uDepthTexture, ivec2(x, size.y - 1 - y), 0).r);
		}
	}
	// min: 16777215.5 is not a float32, it rounds to 2^24 and the far plane would wrap to black.
	float scaled = min(floor(clamp(rawDepth, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0);
	float r = mod(scaled, 256.0);
	float g = mod(floor(scaled / 256.0), 256.0);
	float b = mod(floor(scaled / 65536.0), 256.0);
	outColor = vec4(r / 255.0, g / 255.0, b / 255.0, 1.0);
}
`;
		const program = createProgram(gl, vertexSource, fragmentSource);
		const vao = gl.createVertexArray();
		gl.bindVertexArray(vao);
		const buffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
		// One oversized triangle covering the whole clip-space square -- avoids a seam down the
		// diagonal of a two-triangle quad, at the cost of overdraw entirely outside [-1, 1] that the
		// viewport clips away for free.
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
		const positionLocation = gl.getAttribLocation(program, "aPosition");
		gl.enableVertexAttribArray(positionLocation);
		gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);
		gl.bindVertexArray(null);
		packPipeline = {
			program,
			vao,
			depthTextureLocation: gl.getUniformLocation(program, "uDepthTexture"),
			halfWidthLocation: gl.getUniformLocation(program, "uHalfWidth"),
			tileHeightLocation: gl.getUniformLocation(program, "uTileHeight"),
		};
		return packPipeline;
	}

	function renderDepthView(gl, atlasHeight, view, cameraMatrices) {
		const halfWidth = view.logicalWidth / 2;
		// Rendered at full resolution, so the pack can keep each block's nearest depth.
		const target = depthCaptureTarget(gl, view.logicalWidth, view.logicalHeight);

		gl.bindFramebuffer(gl.FRAMEBUFFER, target.framebuffer);
		gl.viewport(0, 0, view.logicalWidth, view.logicalHeight);
		gl.disable(gl.SCISSOR_TEST);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
		setCamera(cameraMatrices);
		window.glDraw();

		const pack = ensurePackPipeline(gl);
		gl.bindFramebuffer(gl.FRAMEBUFFER, null);
		const y = flippedY(atlasHeight, view.rect);
		gl.viewport(view.rect.x, y, view.rect.width, view.rect.height);
		gl.enable(gl.SCISSOR_TEST);
		gl.scissor(view.rect.x, y, view.rect.width, view.rect.height);
		gl.disable(gl.DEPTH_TEST);
		gl.useProgram(pack.program);
		gl.bindVertexArray(pack.vao);
		gl.activeTexture(gl.TEXTURE0);
		gl.bindTexture(gl.TEXTURE_2D, target.depthTexture);
		gl.uniform1i(pack.depthTextureLocation, 0);
		gl.uniform1f(pack.halfWidthLocation, halfWidth);
		gl.uniform1f(pack.tileHeightLocation, view.rect.height);
		gl.drawArrays(gl.TRIANGLES, 0, 3);
		gl.bindVertexArray(null);
		gl.enable(gl.DEPTH_TEST);
	}

	window.addEventListener("qaros-render-frame", ({ detail }) => {
		core.withRenderFrameAcknowledgement(detail, () => {
			if (!state.gl || !state.canvas || typeof window.glDraw !== "function") {
				throw new Error(
					"QAR adaptation: no captured WebGL context/draw function yet -- the application has " +
						"not rendered a frame."
				);
			}
			const gl = state.gl;
			const atlasHeight = detail.atlas.height;

			// `setCamera` reassigns `window.glCamera.view`/`projection` to a *different* object per
			// eye (see its own doc comment); saved and restored here so the application's own default
			// camera -- the very same `window.glCamera` object its `resize()` handler mutates in
			// place -- is not left permanently pointing at whichever eye this request processed last.
			const originalView = window.glCamera.view;
			const originalProjection = window.glCamera.projection;
			state.suspendConfinement = true;
			try {
				for (const target of detail.targets) {
					const leftMatrices = getEyeMatrices(`${target.id}:left`);
					configureCamera(leftMatrices, target.cameras.left);
					const rightMatrices = getEyeMatrices(`${target.id}:right`);
					configureCamera(rightMatrices, target.cameras.right);

					for (const view of target.views) {
						if (view.dataType === "depth") {
							// Each depth view is rendered with its own eye's camera; a single depth
							// view (eye "none") uses the left eye's, as the colour views do.
							renderDepthView(gl, atlasHeight, view, view.eye === "right" ? rightMatrices : leftMatrices);
							continue;
						}
						setCamera(view.eye === "right" ? rightMatrices : leftMatrices);
						renderColorView(gl, atlasHeight, view.rect);
					}
				}
			} finally {
				window.glCamera.view = originalView;
				window.glCamera.projection = originalProjection;
				state.suspendConfinement = false;
			}
		});
	});
})();
