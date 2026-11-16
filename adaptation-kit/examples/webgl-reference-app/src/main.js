// Hand-written WebGL2: no rendering engine, no matrix library, no model loader -- everything this
// application draws comes from geometry.js's procedural generators, gl-utils.js's own thin
// compile/link wrappers, and mat4.js's own matrix helpers. It carries no QAR awareness whatsoever:
// nothing here references `window.qaros`, dispatches or listens for a "qaros-render-frame" event, or
// knows that it might one day be adapted.
//
// The one concession to realism rather than obscurity, matching the three.js reference application
// alongside this one: it exposes `window.glCamera` (the current view/projection matrices) and
// `window.glDraw` (a plain function reference that issues this frame's draw calls) for console
// debugging and quick experimentation -- an ordinary habit for a hand-rolled WebGL project, not an
// integration point for anything external. Unlike the three.js application's `window.renderer`,
// `window.scene` and `window.camera`, none of these names belong to any known library, so
// `ProbePageEngine`'s `kProbeKnownEngines` list has nothing to match here -- this application
// genuinely exposes no framework global.
import { createProgram, createBuffer } from "./gl-utils.js";
import { createBox, createOctahedron, createIcosahedron, createCone, createDisc } from "./geometry.js";
import { identity, lookAt, perspectiveFromFov, modelFromPositionYaw } from "./mat4.js";

const canvas = document.querySelector("#viewport");
const gl = canvas.getContext("webgl2", { antialias: true, depth: true });
if (!gl) {
	throw new Error("WebGL2 is not available in this browser.");
}

const vertexSource = `#version 300 es
in vec3 aPosition;
in vec3 aNormal;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;
out vec3 vNormal;
void main() {
	vNormal = mat3(uModel) * aNormal;
	gl_Position = uProjection * uView * uModel * vec4(aPosition, 1.0);
}
`;

const fragmentSource = `#version 300 es
precision highp float;
in vec3 vNormal;
uniform vec3 uBaseColor;
uniform vec3 uLightDirection;
uniform float uLightIntensity;
uniform float uAmbient;
out vec4 outColor;
void main() {
	vec3 normal = normalize(vNormal);
	float diffuse = max(dot(normal, normalize(-uLightDirection)), 0.0);
	vec3 shaded = uBaseColor * (uAmbient + diffuse * uLightIntensity);
	outColor = vec4(shaded, 1.0);
}
`;

// createProgram checks gl.getShaderParameter(COMPILE_STATUS)/getProgramParameter(LINK_STATUS) itself
// and throws with the driver's own info log on failure -- see gl-utils.js's own doc comment on why
// that check is never skipped here: WebGL only logs a GLSL compile error to the console, it never
// throws, so a silently broken shader would otherwise present as unexplained black or blank output.
const program = createProgram(gl, vertexSource, fragmentSource);

const attributeLocations = {
	position: gl.getAttribLocation(program, "aPosition"),
	normal: gl.getAttribLocation(program, "aNormal"),
};
const uniformLocations = {
	model: gl.getUniformLocation(program, "uModel"),
	view: gl.getUniformLocation(program, "uView"),
	projection: gl.getUniformLocation(program, "uProjection"),
	baseColor: gl.getUniformLocation(program, "uBaseColor"),
	lightDirection: gl.getUniformLocation(program, "uLightDirection"),
	lightIntensity: gl.getUniformLocation(program, "uLightIntensity"),
	ambient: gl.getUniformLocation(program, "uAmbient"),
};

function createMesh(geometry) {
	const vao = gl.createVertexArray();
	gl.bindVertexArray(vao);
	createBuffer(gl, gl.ARRAY_BUFFER, geometry.positions);
	gl.enableVertexAttribArray(attributeLocations.position);
	gl.vertexAttribPointer(attributeLocations.position, 3, gl.FLOAT, false, 0, 0);
	createBuffer(gl, gl.ARRAY_BUFFER, geometry.normals);
	gl.enableVertexAttribArray(attributeLocations.normal);
	gl.vertexAttribPointer(attributeLocations.normal, 3, gl.FLOAT, false, 0, 0);
	createBuffer(gl, gl.ELEMENT_ARRAY_BUFFER, geometry.indices);
	gl.bindVertexArray(null);
	return { vao, indexCount: geometry.indices.length };
}

const meshes = {
	box: createMesh(createBox(1.1)),
	octahedron: createMesh(createOctahedron(0.75)),
	icosahedron: createMesh(createIcosahedron(0.75)),
	cone: createMesh(createCone(0.65, 1.3, 24)),
	ground: createMesh(createDisc(6, 48)),
};

const orbitRadius = 2.4;
const objectDefinitions = [
	{ name: "Amber block", mesh: meshes.box, color: new Float32Array([0.85, 0.55, 0.22]) },
	{ name: "Coral spike", mesh: meshes.octahedron, color: new Float32Array([0.87, 0.32, 0.28]) },
	{ name: "Steel facet", mesh: meshes.icosahedron, color: new Float32Array([0.28, 0.5, 0.78]) },
	{ name: "Violet cone", mesh: meshes.cone, color: new Float32Array([0.55, 0.35, 0.78]) },
];
const objects = objectDefinitions.map((definition, index) => ({
	...definition,
	baseAngle: (index / objectDefinitions.length) * Math.PI * 2,
	visible: true,
	model: identity(),
}));
const ground = {
	mesh: meshes.ground,
	color: new Float32Array([0.08, 0.09, 0.1]),
	model: modelFromPositionYaw(identity(), [0, -1.4, 0], 0),
};

let sceneRotation = 0;
let rotationSpeed = 0.3;
let lightIntensity = 1.3;
const ambient = 0.35;
const lightDirection = new Float32Array([-0.4, -1.0, -0.3]);

// `window.glCamera` holds the *current* view/projection matrices this application's own animation
// loop drives every frame; the adaptation (see the kit's lib/webgl-render-frame-handler@1.js) overwrites both
// in place, once per connected user's eye, immediately before calling `window.glDraw()` directly --
// the same "hand the application's own draw entry point a throwaway camera" technique the three.js
// reference application uses via `renderer.render(scene, camera)`, generalised to a plain function and
// a plain matrix pair since raw WebGL has no scene-graph object to call a method on.
window.glCamera = { view: identity(), projection: identity() };

function updateDefaultCamera() {
	const aspect = canvas.width / Math.max(canvas.height, 1);
	perspectiveFromFov(window.glCamera.projection, (50 * Math.PI) / 180, aspect, 0.1, 100);
	lookAt(window.glCamera.view, [0, 1.6, 6], [0, 0, 0], [0, 1, 0]);
}

function drawObject(object) {
	gl.bindVertexArray(object.mesh.vao);
	gl.uniformMatrix4fv(uniformLocations.model, false, object.model);
	gl.uniform3fv(uniformLocations.baseColor, object.color);
	gl.drawElements(gl.TRIANGLES, object.mesh.indexCount, gl.UNSIGNED_SHORT, 0);
	gl.bindVertexArray(null);
}

// The single entry point that draws this scene into whatever viewport/scissor rectangle and
// framebuffer are already bound, using whatever `window.glCamera.view`/`projection` currently hold.
// It never touches the viewport, the scissor rectangle or which framebuffer is bound, and never
// clears -- the caller (this application's own animation loop below, or the adaptation rendering a
// user's eye) owns all of that, exactly so an external caller can safely redirect where this draws
// without this function fighting back every frame.
window.glDraw = function drawScene() {
	gl.useProgram(program);
	gl.uniformMatrix4fv(uniformLocations.view, false, window.glCamera.view);
	gl.uniformMatrix4fv(uniformLocations.projection, false, window.glCamera.projection);
	gl.uniform3fv(uniformLocations.lightDirection, lightDirection);
	gl.uniform1f(uniformLocations.lightIntensity, lightIntensity);
	gl.uniform1f(uniformLocations.ambient, ambient);
	drawObject(ground);
	for (const object of objects) {
		if (!object.visible) {
			continue;
		}
		const angle = object.baseAngle + sceneRotation;
		modelFromPositionYaw(object.model, [Math.cos(angle) * orbitRadius, 0, Math.sin(angle) * orbitRadius], sceneRotation);
		drawObject(object);
	}
};

function resize() {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	updateDefaultCamera();
}
window.addEventListener("resize", resize);
resize();

// --- Left panel: the scene's own object list, with a visibility toggle per entry. ---

const objectList = document.querySelector("#object-list");
for (const object of objects) {
	const item = document.createElement("li");
	const checkbox = document.createElement("input");
	checkbox.type = "checkbox";
	checkbox.checked = true;
	checkbox.addEventListener("change", () => {
		object.visible = checkbox.checked;
		pushToast(`${object.name} ${checkbox.checked ? "shown" : "hidden"}`);
	});
	const label = document.createElement("span");
	label.textContent = object.name;
	item.append(checkbox, label);
	objectList.append(item);
}

// --- Right panel: settings that actually drive the scene. ---

const rotationSlider = document.querySelector("#rotation-speed");
rotationSlider.addEventListener("input", () => {
	rotationSpeed = Number(rotationSlider.value) / 100;
});

const lightSlider = document.querySelector("#light-intensity");
lightSlider.addEventListener("input", () => {
	lightIntensity = (Number(lightSlider.value) / 100) * 2;
});

// --- Modal popup, opened automatically once and reopenable from the settings panel. ---

const modalBackdrop = document.querySelector("#modal-backdrop");
document.querySelector("#modal-close").addEventListener("click", () => {
	modalBackdrop.classList.add("hidden");
});
document.querySelector("#open-about").addEventListener("click", () => {
	modalBackdrop.classList.remove("hidden");
});
setTimeout(() => modalBackdrop.classList.remove("hidden"), 300);

// --- Toast stack: notifications the app raises on its own, plus ones the user triggers above. ---

const toastStack = document.querySelector("#toast-stack");
const maxToasts = 4;

function pushToast(message) {
	const toast = document.createElement("div");
	toast.className = "toast";
	toast.textContent = message;
	toastStack.append(toast);
	while (toastStack.children.length > maxToasts) {
		toastStack.firstElementChild.remove();
	}
	setTimeout(() => toast.remove(), 6000);
}

pushToast("Scene loaded");
let toastCounter = 0;
const toastMessages = ["Buffers uploaded", "Recalculated lighting", "Camera idle", "Shaders warm"];
setInterval(() => {
	pushToast(toastMessages[toastCounter % toastMessages.length]);
	toastCounter += 1;
}, 2500);

// --- HUD overlay: drawn as ordinary DOM on top of the canvas, never part of its pixels. ---

const hudFps = document.querySelector("#hud-fps");
let frameCount = 0;
let fpsWindowStart = performance.now();

function updateHud(now) {
	frameCount += 1;
	const elapsed = now - fpsWindowStart;
	if (elapsed >= 500) {
		hudFps.textContent = Math.round((frameCount * 1000) / elapsed).toString();
		frameCount = 0;
		fpsWindowStart = now;
	}
}

// --- The render loop itself: an ordinary requestAnimationFrame loop over raw WebGL2 calls. ---

gl.enable(gl.DEPTH_TEST);

let lastTime = performance.now();
function frame(now) {
	const deltaSeconds = Math.min((now - lastTime) / 1000, 0.1);
	lastTime = now;
	sceneRotation += rotationSpeed * deltaSeconds;

	// This is the one call an adaptation script has to intercept to take canvas ownership: with no
	// adaptation installed it simply sizes the viewport to the whole canvas, exactly as it looks here.
	// See the kit's lib/webgl-capture-context@1.js's own doc comment for how (and why) it does that without any
	// cooperation from this application at all.
	gl.viewport(0, 0, canvas.width, canvas.height);
	gl.clearColor(0.04, 0.05, 0.045, 1);
	gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
	window.glDraw();

	updateHud(now);
	requestAnimationFrame(frame);
}
requestAnimationFrame(frame);
