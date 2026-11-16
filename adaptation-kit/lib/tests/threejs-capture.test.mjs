import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";
import test from "node:test";

const source = readFileSync(new URL("../threejs-capture@1.js", import.meta.url), "utf8")
	.replaceAll("__PAGE_REGION_WIDTH__", "640")
	.replaceAll("__PAGE_REGION_HEIGHT__", "480");

test("stereo requests reject a WebGPU context with an explicit capability error", async () => {
	const renderSource = readFileSync(new URL("../threejs-render-frame@1.js", import.meta.url), "utf8");
	let render;
	const handlers = [];
	const state = { renderer: { getContext: () => ({}) }, scene: {} };
	const core = {
		createRenderFrameLoop(callback) { render = callback; return {}; },
		addRenderFrameHandler(handler) { handlers.push(handler); return handler.loop; },
		streamSurface: { ensure() {}, publish() {} },
	};
	vm.runInNewContext(renderSource, { window: { __qarAdaptationCore: core, __qarAdaptationState: state } });
	assert.equal(handlers.length, 1);
	assert.equal(handlers[0].accepts(), false, "a WebGPU context belongs to threejs-webgpu-render-frame@1");
	assert.throws(() => render({ atlas: { width: 64, height: 64 }, targets: [] }), /WebGPU.*requires a WebGL renderer/);
	assert.equal(state.suspendConfinement, undefined);
});

function application({ embedded = false, parentState } = {}) {
	const window = {dispatchEvent() {}, innerWidth: 778, innerHeight: 1070};
	window.top = embedded ? {__qarAdaptationState: parentState} : window;
	const callbacks = [];
	const documentStyle = { setProperty() {} };
	vm.runInNewContext(source, {
		window, EventTarget, Event, console, document: {documentElement: {style: documentStyle}},
		requestAnimationFrame: (callback) => callbacks.push(callback),
	});
	const renderer = {
		domElement: { width: 1024, height: 1024, style: {} },
 setSize(w, h) {this.domElement.width = w; this.domElement.height = h;},
		getContext() {}, render() {},
		setScissorTest() {}, setViewport() {}, setScissor() {},
	};
	const announce = (object) => {
		const event = new Event("observe");
		event.detail = object;
		window.__THREE_DEVTOOLS__.dispatchEvent(event);
	};
	announce(renderer);
	return { window, renderer, documentStyle, announce, finishFrame: () => callbacks.splice(0).forEach((callback) => callback()) };
}

function perspectiveCamera() {
	return { isPerspectiveCamera: true, aspect: 640 / 480, updateProjectionMatrix() {} };
}

test("bundled renderer and scene are captured without application globals", () => {
	const app = application();
	const scene = { isScene: true };
	app.announce(scene);
	app.renderer.render(scene, perspectiveCamera());
	app.finishFrame();
	assert.equal(app.window.__qarAdaptationState.renderer, app.renderer);
	assert.equal(app.window.__qarAdaptationState.scene, scene);
	assert.equal(app.window.__qarAdaptationState.scenes.size, 1);
	assert.equal(app.window.THREE, undefined);
	assert.equal(app.window.renderer, undefined);
});

test("last perspective scene wins over an earlier scene and a later orthographic HUD", () => {
	const app = application();
	const first = { isScene: true };
	const last = { isScene: true };
	const camera = perspectiveCamera();
	app.renderer.render(first, perspectiveCamera());
	app.renderer.render(last, camera);
	app.renderer.render({ isScene: true }, { isOrthographicCamera: true });
	app.finishFrame();
	assert.equal(app.window.__qarAdaptationState.scene, last);
	assert.equal(app.window.__qarAdaptationState.camera, camera);
});

test("per-user renders do not replace the application scene", () => {
	const app = application();
	const scene = { isScene: true };
	app.renderer.render(scene, perspectiveCamera());
	app.finishFrame();
	app.window.__qarAdaptationState.suspendConfinement = true;
	app.renderer.render({ isScene: true }, perspectiveCamera());
	app.finishFrame();
	assert.equal(app.window.__qarAdaptationState.scene, scene);
});

test("offscreen environment passes keep their viewport and cannot replace the canvas scene", () => {
	const app = application();
	const scene = { isScene: true };
	app.renderer.render(scene, perspectiveCamera());
	app.finishFrame();
	app.renderer.getRenderTarget = () => ({ width: 256, height: 256 });
	let confined = false;
	app.renderer.setViewport = () => { confined = true; };
	app.renderer.render({ isScene: true }, perspectiveCamera());
	app.finishFrame();
	assert.equal(confined, false, "canvas coordinates would clip a small environment-map target");
	assert.equal(app.window.__qarAdaptationState.scene, scene);
});

test("responsive page retains normal DOM geometry while only its GPU backing store is atlas-sized", () => {
	const app = application();
	const camera = perspectiveCamera();
	const scene = { isScene: true };
	app.renderer.render(scene, camera);
	app.window.__qarResizePage(900, 600, 8000, 4096);
	app.renderer.setSize(900, 600);
	assert.equal(app.renderer.domElement.width, 8000);
	assert.equal(app.renderer.domElement.height, 4096);
	assert.equal(app.documentStyle.scale, undefined);
	assert.equal(app.documentStyle.width, undefined);
	assert.equal(Object.hasOwn(app.renderer.domElement, "clientWidth"), false);
	assert.equal(Object.hasOwn(app.renderer.domElement, "getBoundingClientRect"), false);
	assert.equal(app.renderer.domElement.style.width, "900px");
	assert.equal(app.renderer.domElement.style.height, "600px");
	app.renderer.render(scene, camera);
	assert.equal(camera.aspect, 900 / 600);
});

test("while streaming the application's own view renders only the displayed corner of the atlas", () => {
	const app = application();
	const viewports = [];
	app.renderer.setViewport = (...rect) => viewports.push(rect);
	const camera = perspectiveCamera();
	const scene = { isScene: true };
	app.renderer.render(scene, camera);
	assert.deepEqual(viewports.at(-1), [0, 0, 1024, 1024]);
	assert.equal(app.renderer.domElement.style.objectViewBox, "");

	app.window.__qarResizePage(900, 600, 8000, 4096);
	app.renderer.setSize(900, 600);
	Object.assign(app.renderer.domElement, { clientWidth: 900, clientHeight: 600 });
	app.renderer.render(scene, camera);
	// Top-left of the atlas (three.js viewports count from the bottom), at the displayed size.
	assert.deepEqual(viewports.at(-1), [0, 4096 - 600, 900, 600]);
	assert.equal(app.renderer.domElement.style.objectViewBox, "inset(0px 7100px 3496px 0px)");
});

test("gallery iframe rendering follows its own responsive viewport without restyling it", () => {
	const app = application({ embedded: true });
	const camera = perspectiveCamera();
	app.renderer.setSize(778, 1070);
	app.renderer.render({ isScene: true }, camera);
	assert.equal(camera.aspect, 778 / 1070);
	assert.equal(app.renderer.domElement.style.position, undefined);
	app.window.innerWidth = 230;
	app.window.innerHeight = 1004;
	app.renderer.setSize(230, 1004);
	app.renderer.render({ isScene: true }, camera);
	assert.equal(camera.aspect, 230 / 1004);
	assert.equal(app.renderer.domElement.width, 230);
});

test("same-origin gallery scenes share the streaming state without altering iframe geometry", () => {
 const parentState = {atlasExtent:{width:8000,height:4096}};
 const app = application({embedded:true,parentState});
 const scene = {isScene:true};
 app.renderer.setSize(230,1004);
 app.renderer.render(scene,perspectiveCamera());app.finishFrame();
 assert.equal(app.window.__qarAdaptationState,parentState);
 assert.equal(parentState.scene,scene);
 assert.equal(app.renderer.domElement.width,8000);
 assert.equal(app.renderer.domElement.style.width,"230px");
 assert.equal(app.renderer.domElement.style.height,"1004px");
 assert.equal(Object.hasOwn(app.renderer.domElement,"clientWidth"),false);
});

test("with several renderers only the one chooseRenderer picks is atlas-sized and streamed", () => {
	const app = application();
	const other = {
		domElement: { width: 300, height: 200, style: {}, clientWidth: 300, clientHeight: 200 },
		setSize(w, h) { this.domElement.width = w; this.domElement.height = h; },
		getContext() {}, render() {},
		setScissorTest() {}, setViewport() {}, setScissor() {},
	};
	app.announce(other);
	Object.assign(app.renderer.domElement, { clientWidth: 900, clientHeight: 600 });
	const state = app.window.__qarAdaptationState;
	let chosen = app.renderer;
	state.chooseRenderer = (renderers) => {
		assert.equal(renderers.size, 2);
		return chosen;
	};
	const streamedScene = { isScene: true };
	const otherScene = { isScene: true };
	app.window.__qarResizePage(900, 600, 8000, 4096);
	app.renderer.setSize(900, 600);
	other.setSize(300, 200);
	app.renderer.render(streamedScene, perspectiveCamera());
	other.render(otherScene, perspectiveCamera());
	app.finishFrame();
	assert.equal(app.renderer.domElement.width, 8000);
	assert.equal(other.domElement.width, 300, "an unchosen renderer keeps its own backing store");
	assert.equal(state.renderer, app.renderer);
	assert.equal(state.scene, streamedScene);
	assert.equal(other.__qarLastScene, otherScene);

	// The layout changed: the other renderer is chosen, the first one gets its own size back.
	chosen = other;
	app.renderer.render(streamedScene, perspectiveCamera());
	app.finishFrame();
	assert.equal(app.renderer.domElement.width, 900);
	assert.equal(app.renderer.domElement.style.objectViewBox, "");
	other.setSize(300, 200);
	assert.equal(other.domElement.width, 8000);
});

test("the WebGPU render frame answers only a WebGPURenderer, on its WebGPU backend", () => {
	const renderSource = readFileSync(new URL("../threejs-webgpu-render-frame@1.js", import.meta.url), "utf8");
	let render;
	const handlers = [];
	const state = { renderer: null, scene: {} };
	const core = {
		createRenderFrameLoop(callback) { render = callback; return {}; },
		addRenderFrameHandler(handler) { handlers.push(handler); return handler.loop; },
		streamSurface: { ensure() {}, publish() {} },
	};
	vm.runInNewContext(renderSource, { window: { __qarAdaptationCore: core, __qarAdaptationState: state } });
	assert.deepEqual(handlers.map(({ name }) => name), ["threejs-webgpu"]);
	assert.equal(handlers[0].accepts(), false);
	state.renderer = { getContext: () => ({ createShader() {} }) };
	assert.equal(handlers[0].accepts(), false, "a WebGLRenderer stays with threejs-render-frame@1");
	state.renderer = { isWebGPURenderer: true, backend: { isWebGLBackend: true } };
	assert.equal(handlers[0].accepts(), true);
	assert.throws(() => render({ atlas: { width: 64, height: 64 }, targets: [] }), /WebGL 2 fallback/);
	assert.equal(state.suspendConfinement, undefined);
});
