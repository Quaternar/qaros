// @qaros-library threejs-render-frame@1.1
// @stage afterLoad
// @needs render-frame-core@1 threejs-capture@1 webgl-atlas-pack@1
//
// Answers each `qaros-render-frame` request for a three.js `WebGLRenderer` page by drawing every
// connected user's views from the application's own renderer and scene, found by threejs-capture@1.
// It needs no global from the application: no `window.THREE`, `window.scene` or `window.camera`.
// List threejs-webgpu-render-frame@1 as well and one profile covers both renderers: each request
// goes to the library for the renderer the page created (render-frame-core@1
// `addRenderFrameHandler`).
//
// One animation tick, one scene render: every eye of every user is drawn by a single `renderer.render`
// with an `ArrayCamera`, so the scene is culled, its shadows rendered and its lights set up once per
// tick, and each eye only re-issues the draw calls into its own viewport. The eyes are drawn into
// the offscreen pass of webgl-atlas-pack@1 (an RGBA8 colour texture and a 24-bit depth texture),
// which three.js treats as the canvas: for the duration of that render the context's
// `bindFramebuffer(null)` binds it instead. So three.js applies tone mapping and the output colour
// space exactly as it does for the application's own canvas, and passes that switch to a render
// target and back (shadow maps, transmission, reflectors) come back to it. webgl-atlas-pack@1 then
// packs colour, depth and alpha of each eye into the atlas, which is published to the stream
// surface (render-frame-core@1 `streamSurface`); the application's own view is drawn again after.
//
// The camera classes come from the renderer itself: `renderer.xr.getCamera()` is an `ArrayCamera`,
// whose base class is `PerspectiveCamera`. three.js culls with the array camera's own frustum, which
// cannot be one eye's when users stand apart, so it is an orthographic box large enough to keep
// everything; each eye's viewport clips the rest. The eye cameras take the application camera's
// layers, so what the application hides by layer stays hidden.
//
// The users' room is the scene's own space unless a profile places it: `window.__qarAdaptationState
// .sceneFromRoom = () => ({ position: [x, y, z], quaternion: [x, y, z, w] })`, asked once per tick,
// is the room's pose in the scene (a Z-up scene, or one followed by the application's own camera).
// The eye cameras are children of that pose, so depth stays along each eye's own view.
//
// Depth: the raw post-projection depth (standard depth, the "regular" convention), packed by
// webgl-atlas-pack@1. Native code is told the projection convention per camera and reconstructs
// from it. A logarithmic depth buffer is not linear post-projection depth and is not supported.
//
// Not covered: post-processing (EffectComposer passes run on the application's own frame only),
// a `WebGPURenderer` (threejs-webgpu-render-frame@1), and multisampling of the eye views (they
// render below the receiver's resolution and the sender upsamples them; a resolve per request
// costs more than the edges gain).
(() => {
	const core = window.__qarAdaptationCore;
	const state = window.__qarAdaptationState;

	const eyeCameras = new Map();
	let cameraClasses = null;
	let arrayCamera = null;
	let packer = null;

	function atlasPacker() {
		if (!packer) {
			const atlasPack = window.__qarWebglAtlasPack;
			if (!atlasPack) throw new Error("QAR adaptation: threejs-render-frame@1 needs webgl-atlas-pack@1, injected before it.");
			packer = atlasPack.createAtlasPacker();
		}
		return packer;
	}

	// A `WebGPURenderer` (whatever its backend) is threejs-webgpu-render-frame@1's.
	function isWebGpuRenderer(renderer) {
		if (!renderer) return false;
		if (renderer.isWebGPURenderer) return true;
		const context = typeof renderer.getContext === "function" ? renderer.getContext() : null;
		return Boolean(context) && typeof context.createShader !== "function";
	}

	function findCameraClasses(renderer) {
		if (cameraClasses) {
			return cameraClasses;
		}
		const xrCamera = renderer.xr && typeof renderer.xr.getCamera === "function" ? renderer.xr.getCamera() : null;
		if (xrCamera && xrCamera.isArrayCamera) {
			const ArrayCamera = xrCamera.constructor;
			const PerspectiveCamera = Object.getPrototypeOf(ArrayCamera.prototype).constructor;
			cameraClasses = { ArrayCamera, PerspectiveCamera };
		} else if (window.THREE && window.THREE.ArrayCamera) {
			cameraClasses = { ArrayCamera: window.THREE.ArrayCamera, PerspectiveCamera: window.THREE.PerspectiveCamera };
		} else {
			throw new Error("QAR adaptation: cannot find three.js camera classes on the renderer.");
		}
		return cameraClasses;
	}

	// The room's pose in the scene; every eye camera is its child.
	let roomInScene = null;
	function updateRoomInScene(classes) {
		if (!roomInScene) {
			const Object3D = Object.getPrototypeOf(Object.getPrototypeOf(classes.PerspectiveCamera.prototype)).constructor;
			roomInScene = new Object3D();
		}
		const pose = typeof state.sceneFromRoom === "function" ? state.sceneFromRoom() : null;
		roomInScene.position.fromArray(pose ? pose.position : [0, 0, 0]);
		roomInScene.quaternion.fromArray(pose ? pose.quaternion : [0, 0, 0, 1]);
		roomInScene.updateMatrixWorld(true);
	}

	function getEyeCamera(classes, key) {
		let camera = eyeCameras.get(key);
		if (!camera) {
			camera = new classes.PerspectiveCamera();
			camera.viewport = { x: 0, y: 0, z: 1, w: 1 };
			roomInScene.add(camera);
			eyeCameras.set(key, camera);
		}
		return camera;
	}

	// An asymmetric projection from the four frustum angles native code supplies: the two eyes of a
	// stereo pair are not generally symmetric.
	function configureCamera(camera, protocolCamera, layersMask) {
		const { pose, fov, nearFar } = protocolCamera;
		camera.position.fromArray(pose.position);
		camera.quaternion.set(pose.orientation[0], pose.orientation[1], pose.orientation[2], pose.orientation[3]);
		camera.near = nearFar.near;
		camera.far = nearFar.far;
		camera.projectionMatrix.makePerspective(
			Math.tan(fov.angleLeft) * camera.near,
			Math.tan(fov.angleRight) * camera.near,
			Math.tan(fov.angleUp) * camera.near,
			Math.tan(fov.angleDown) * camera.near,
			camera.near,
			camera.far,
			camera.coordinateSystem
		);
		camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
		camera.layers.mask = layersMask;
		camera.updateMatrixWorld(true);
	}

	function ensureArrayCamera(classes) {
		if (arrayCamera) {
			return arrayCamera;
		}
		arrayCamera = new classes.ArrayCamera([]);
		const extent = 1e6;
		arrayCamera.projectionMatrix.makeOrthographic(
			-extent,
			extent,
			extent,
			-extent,
			-extent,
			extent,
			arrayCamera.coordinateSystem
		);
		arrayCamera.projectionMatrixInverse.copy(arrayCamera.projectionMatrix).invert();
		return arrayCamera;
	}

	// One entry per eye that has a view: its camera, its colour, depth and packed-alpha views (any
	// may be absent) and the size they are rendered at.
	function collectEyes(classes, targets, layersMask) {
		const eyes = [];
		for (const target of targets) {
			const byEye = new Map();
			for (const view of target.views) {
				const eye = byEye.get(view.eye) || { name: view.eye, target };
				eye[view.dataType === "depth" || view.dataType === "alpha" ? view.dataType : "color"] = view;
				byEye.set(view.eye, eye);
			}
			for (const eye of byEye.values()) {
				const sizeView = eye.color || eye.depth;
				eye.width = sizeView.logicalWidth;
				eye.height = sizeView.logicalHeight;
				// Each depth view comes from its own eye's render, or the far eye's depth would be
				// warped against the wrong view.
				const side = eye.name === "right" ? "right" : "left";
				eye.camera = getEyeCamera(classes, `${target.id}:${side}`);
				configureCamera(eye.camera, target.cameras[side], layersMask);
				eyes.push(eye);
			}
		}
		return eyes;
	}

	// One row per user; each eye's camera viewport is its rectangle, y counted from the bottom.
	function layoutEyes(eyes) {
		let width = 0;
		let rowY = 0;
		let rowTarget = null;
		let rowX = 0;
		let rowHeight = 0;
		for (const eye of eyes) {
			if (eye.target !== rowTarget) {
				rowY += rowHeight;
				rowTarget = eye.target;
				rowX = 0;
				rowHeight = 0;
			}
			Object.assign(eye.camera.viewport, { x: rowX, y: rowY, z: eye.width, w: eye.height });
			rowX += eye.width;
			rowHeight = Math.max(rowHeight, eye.height);
			width = Math.max(width, rowX);
		}
		return { width, height: rowY + rowHeight };
	}

	// Users see only what has depth: the mixer places every pixel by its depth and clips it to the
	// app volume, and a pixel at the far plane is "nothing here". An opaque material that writes no
	// depth (a ground plane drawn with depthWrite: false, a common trick for floors and decals) would
	// vanish from every user's view while the page still shows it, so for the users' views every
	// opaque material writes depth. Transparent materials keep theirs. Returns what to restore.
	function writeDepthForOpaqueMaterials(scene) {
		const changed = [];
		const writeDepth = (material) => {
			if (!material || material.depthWrite !== false || material.transparent) return;
			if (material.opacity !== undefined && material.opacity < 1) return;
			material.depthWrite = true;
			changed.push(material);
		};
		scene.traverse((object) => {
			if (!object.isMesh) return;
			if (Array.isArray(object.material)) object.material.forEach(writeDepth);
			else writeDepth(object.material);
		});
		return () => changed.forEach((material) => { material.depthWrite = false; });
	}

	// three.js thinks it draws to the canvas; the context binds the eye framebuffer instead.
	function renderIntoPass(renderer, gl, scene, camera, framebuffer) {
		const bindFramebuffer = gl.bindFramebuffer;
		const drawBuffers = gl.drawBuffers;
		const ownBindFramebuffer = Object.hasOwn(gl, "bindFramebuffer");
		const ownDrawBuffers = Object.hasOwn(gl, "drawBuffers");
		let redirected = false;
		renderer.setRenderTarget(null);
		renderer.resetState();
		gl.bindFramebuffer = (target, buffer) => {
			if (target === gl.FRAMEBUFFER || target === gl.DRAW_FRAMEBUFFER) redirected = buffer === null;
			return bindFramebuffer.call(gl, target, buffer === null ? framebuffer : buffer);
		};
		// three.js selects BACK for its canvas, but the redirected framebuffer owns COLOR_ATTACHMENT0.
		// Intermediate render targets retain their own attachment selection.
		gl.drawBuffers = (buffers) => drawBuffers.call(gl,
			redirected ? buffers.map((buffer) => buffer === gl.BACK ? gl.COLOR_ATTACHMENT0 : buffer) : buffers);
		// Users get the scene without its background: in AR the room shows behind it. Clearing to
		// the page's background colour instead would blend that colour into every antialiased edge
		// (the resolved edge pixel mixes object and clear colour), a coloured fringe in the headset.
		const background = scene.background;
		scene.background = null;
		const restoreDepthWrite = writeDepthForOpaqueMaterials(scene);
		try {
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
			renderer.setScissorTest(false);
			renderer.clear(true, true, true);
			renderer.render(scene, camera);
		} finally {
			restoreDepthWrite();
			scene.background = background;
			if (ownBindFramebuffer) gl.bindFramebuffer = bindFramebuffer;
			else delete gl.bindFramebuffer;
			if (ownDrawBuffers) gl.drawBuffers = drawBuffers;
			else delete gl.drawBuffers;
			bindFramebuffer.call(gl, gl.FRAMEBUFFER, null);
		}
	}

	function renderAllEyes(classes, renderer, gl, scene, eyes, layersMask) {
		const size = layoutEyes(eyes);
		const target = atlasPacker().ensurePass(gl, size.width, size.height);
		const camera = ensureArrayCamera(classes);
		camera.cameras = eyes.map((eye) => eye.camera);
		camera.layers.mask = layersMask;
		// Only the transparent objects' draw order uses the array camera's pose (in scene space).
		eyes[0].camera.matrixWorld.decompose(camera.position, camera.quaternion, camera.scale);
		renderIntoPass(renderer, gl, scene, camera, target.framebuffer);
		return target;
	}

	const surface = core.streamSurface;
	if (!surface || typeof core.addRenderFrameHandler !== "function") {
		throw new Error("QAR adaptation: threejs-render-frame@1 needs render-frame-core@1.1 or newer, injected before it.");
	}
	// Establish the compositor before native code waits for the first atlas-size paint.
	surface.ensure(1, 1);
	const loop = core.createRenderFrameLoop((detail) => {
		const renderer = state && state.renderer;
		const scene = state && state.scene;
		if (!renderer || !scene) {
			throw new Error("QAR adaptation: no captured three.js renderer/scene yet -- the application has not rendered a frame.");
		}
		const gl = typeof renderer.getContext === "function" ? renderer.getContext() : null;
		if (!gl || typeof gl.createShader !== "function") {
			throw new Error("QAR adaptation: this renderer uses WebGPU; threejs-render-frame@1 requires a WebGL renderer (list threejs-webgpu-render-frame@1 for a WebGPURenderer).");
		}
		const atlasHeight = detail.atlas.height;
		if (gl.drawingBufferWidth !== detail.atlas.width || gl.drawingBufferHeight !== atlasHeight)
			renderer.setSize(renderer.domElement.clientWidth, renderer.domElement.clientHeight);
		// Diagnostic, reported once: the canvas must cover the atlas 1:1 or every view and the
		// render-frame tag land at different rows than native code reads.
		if (
			!state.canvasSizeReported &&
			(gl.drawingBufferWidth !== detail.atlas.width || gl.drawingBufferHeight !== atlasHeight)
		) {
			state.canvasSizeReported = true;
			const rect = renderer.domElement.getBoundingClientRect();
			throw new Error(
				`QAR adaptation: canvas buffer ${gl.drawingBufferWidth}x${gl.drawingBufferHeight}, ` +
					`css ${rect.left},${rect.top} ${rect.width}x${rect.height}, ` +
					`inner ${window.innerWidth}x${window.innerHeight}, dpr ${window.devicePixelRatio}, ` +
					`atlas ${detail.atlas.width}x${atlasHeight}`
			);
		}

		const classes = findCameraClasses(renderer);
		const layersMask = state.camera && state.camera.layers ? state.camera.layers.mask : 1;
		updateRoomInScene(classes);
		state.suspendConfinement = true;
		try {
			const eyes = collectEyes(classes, detail.targets, layersMask);
			if (eyes.length > 0) {
				const target = renderAllEyes(classes, renderer, gl, scene, eyes, layersMask);
				atlasPacker().copyEyesToAtlas(gl, target, atlasHeight, eyes);
			}
		} finally {
			state.suspendConfinement = false;
		}
		const tagged = core.writeRenderFrameTag(gl, detail);
		// Snapshot the atlas before restoring the application's own view. The original WebGL
		// context stays with the app, retaining its GPU environment maps and resources. Default
		// options keep the copy on the GPU (~0.1 ms); "none" for alpha or colour space forces a
		// CPU readback (~118 ms at 8000x4096). Every atlas byte is opaque, so nothing is converted.
		// Not awaited: the copy is queued on the GPU now, and the loop draws again next tick.
		surface.publish(createImageBitmap(renderer.domElement), detail.atlas.width, atlasHeight);
		renderer.resetState();
		if (state.camera) renderer.render(scene, state.camera);
		return tagged;
	});
	// Registered, not listening itself: a profile may also list threejs-webgpu-render-frame@1, and
	// each request goes to the one for the renderer the page created.
	core.addRenderFrameHandler({ name: "threejs-webgl", accepts: () => !isWebGpuRenderer(state.renderer), loop });
})();
