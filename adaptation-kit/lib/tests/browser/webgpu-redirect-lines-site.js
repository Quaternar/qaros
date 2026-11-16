// Stage: afterLoad, after webgpu-redirect-presenter@1. The site script of webgpu-redirect-lines.html:
// each eye gets its own copy of the page's "fixture camera" buffer holding the eye's
// view-projection.
(() => {
	const S = window.__qarAdaptationState;
	S.site = {
		eyeBuffers(eye) {
			const appCamera = S.buffersByLabel.get("fixture camera");
			if (!appCamera) throw new Error("QAR adaptation: the fixture's camera buffer was not captured");
			if (!eye.cache.camera) {
				eye.cache.camera = eye.device.createBuffer({ label: `qar camera ${eye.key}`, size: 64,
					usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
			}
			eye.device.queue.writeBuffer(eye.cache.camera, 0, eye.viewProjection);
			return [[appCamera, eye.cache.camera]];
		},
	};
})();
