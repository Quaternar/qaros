// Stage: afterLoad, after webgpu-redirect-presenter@1. The only site-specific code of this
// adaptation: the page's camera.
//
// webgpu-redirect-capture@1 redirects the page's own WebGPU loop into an offscreen texture and
// records its last frame; webgpu-redirect-presenter@1 replays that frame once per user eye. For each
// eye it asks this script which of the page's buffers hold the camera and gives each eye its own
// copy, with the eye's matrices written in. Every user and eye is then a real camera with its own
// passes and depth, never a crop of another view.
//
// EDIT the two constants below:
// - CAMERA_BUFFER: the label the page gives its camera uniform buffer (`device.createBuffer({ label
//   })`). The capture records every labelled buffer in `__qarAdaptationState.buffersByLabel`; list
//   them in the page with `[...__qarAdaptationState.buffersByLabel.keys()]`. A page without labels
//   needs another way to find the buffer (for example the bind group of its first render pass).
// - CAMERA_LAYOUT: what the page's shaders read from that buffer, in order. Each entry is one 4x4
//   column-major matrix of the eye: "viewProjection", "inverseViewProjection", "view" or
//   "projection" (WebGPU [0, 1] depth, regular convention).
(() => {
	const S = (window.__qarAdaptationState = window.__qarAdaptationState || {});
	const CAMERA_BUFFER = "camera uniforms";
	const CAMERA_LAYOUT = ["viewProjection", "inverseViewProjection"];

	// The eye's camera data, laid out as the page's buffer expects.
	function cameraData(eye) {
		const data = new Float32Array(16 * CAMERA_LAYOUT.length);
		CAMERA_LAYOUT.forEach((name, index) => {
			const matrix = eye[name];
			if (!matrix || matrix.length !== 16) throw new Error(`QAR adaptation: CAMERA_LAYOUT names '${name}', which the eye does not have`);
			data.set(matrix, 16 * index);
		});
		return data;
	}

	S.site = {
		// One [page buffer, eye buffer] pair per buffer the eye needs its own copy of. One buffer per
		// eye: a buffer written twice before one submit gives every eye the last value.
		eyeBuffers(eye) {
			const pageCamera = S.buffersByLabel && S.buffersByLabel.get(CAMERA_BUFFER);
			if (!pageCamera) throw new Error(`QAR adaptation: the page's "${CAMERA_BUFFER}" buffer was not captured; edit CAMERA_BUFFER in scripts/site-camera.js`);
			const data = cameraData(eye);
			if (!eye.cache.camera) {
				eye.cache.camera = eye.device.createBuffer({
					label: `qar camera ${eye.key}`,
					size: Math.max(pageCamera.size || 0, data.byteLength),
					usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
				});
			}
			eye.device.queue.writeBuffer(eye.cache.camera, 0, data);
			return [[pageCamera, eye.cache.camera]];
		},
		// App units per protocol (room) unit, when the page's near/far are not in AppScale's units.
		nearFarScale: 1,
	};
})();
