// Stage: afterLoad, after webgpu-redirect-presenter@1. Recipe 7: the only site-specific code. The
// presenter replays the page's frame once per eye; this gives each eye its own copy of the page's
// "recipe camera" buffer, holding the eye's view-projection (one buffer per eye: a buffer written
// twice before one submit gives every eye the last value).
(() => {
	const S = window.__qarAdaptationState;
	S.site = {
		eyeBuffers(eye) {
			const appCamera = S.buffersByLabel.get("recipe camera");
			if (!appCamera) throw new Error("QAR adaptation: the page's camera buffer was not captured");
			if (!eye.cache.camera) {
				eye.cache.camera = eye.device.createBuffer({ label: `qar camera ${eye.key}`, size: 64,
					usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
			}
			eye.device.queue.writeBuffer(eye.cache.camera, 0, eye.viewProjection);
			return [[appCamera, eye.cache.camera]];
		},
	};
})();
