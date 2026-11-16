import * as THREE from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";

// Many published three.js demos expose their library, renderer, scene and camera on `window`
// purely so a developer can poke at the live scene from the browser console -- a documentation/
// debugging convention, not an integration point for anything external, and this app is no
// exception. It carries no QAR awareness: nothing here references `window.qaros`, dispatches or
// listens for a "qaros-render-frame" event, or knows that it might one day be adapted.
window.THREE = THREE;

const canvas = document.querySelector("#viewport");
const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
renderer.setPixelRatio(1);
renderer.setSize(window.innerWidth, window.innerHeight, false);
window.renderer = renderer;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x0d1117);
window.scene = scene;

const camera = new THREE.PerspectiveCamera(
	50,
	window.innerWidth / window.innerHeight,
	0.1,
	100
);
camera.position.set(0, 1.6, 6);
camera.lookAt(0, 0, 0);
window.camera = camera;

const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
scene.add(ambientLight);
const keyLight = new THREE.DirectionalLight(0xffffff, 1.4);
keyLight.position.set(4, 6, 3);
scene.add(keyLight);

const ground = new THREE.Mesh(
	new THREE.CircleGeometry(6, 48),
	new THREE.MeshStandardMaterial({ color: 0x141a24, roughness: 1 })
);
ground.rotation.x = -Math.PI / 2;
ground.position.y = -1.4;
scene.add(ground);

const objectGroup = new THREE.Group();
scene.add(objectGroup);

const galleryDefinitions = [
	{ name: "Sapphire cube", geometry: new THREE.BoxGeometry(1, 1, 1), color: 0x3a7bd5 },
	{ name: "Amber torus", geometry: new THREE.TorusGeometry(0.6, 0.22, 24, 48), color: 0xffb84d },
	{ name: "Jade icosahedron", geometry: new THREE.IcosahedronGeometry(0.7, 0), color: 0x35d0a0 },
	{ name: "Coral cone", geometry: new THREE.ConeGeometry(0.6, 1.2, 32), color: 0xff6f61 },
	{ name: "Pearl sphere", geometry: new THREE.SphereGeometry(0.65, 32, 16), color: 0xe8e8f0 },
];

const gltfLoader = new GLTFLoader();

let meshes = [];

function releaseObjects() {
	for (const entry of meshes) {
		entry.mesh.material.dispose();
	}
	meshes = [];
	objectGroup.clear();
}

function renderObjectList() {
	objectList.replaceChildren();
	for (const entry of meshes) {
		const item = document.createElement("li");
		const checkbox = document.createElement("input");
		checkbox.type = "checkbox";
		checkbox.checked = true;
		checkbox.addEventListener("change", () => {
			entry.mesh.visible = checkbox.checked;
			pushToast(`${entry.name} ${checkbox.checked ? "shown" : "hidden"}`);
		});
		const label = document.createElement("span");
		label.textContent = entry.name;
		item.append(checkbox, label);
		objectList.append(item);
	}
}

function selectGallery() {
	releaseObjects();
	meshes = galleryDefinitions.map((definition, index) => {
		const mesh = new THREE.Mesh(
			definition.geometry,
			new THREE.MeshStandardMaterial({
				color: definition.color,
				roughness: 0.35,
				metalness: 0.1,
			})
		);
		mesh.userData.assetFormat = definition.asset ?? "procedural";
		mesh.userData.textureFormat = definition.texture ?? "none";
		const angle = (index / galleryDefinitions.length) * Math.PI * 2;
		mesh.position.set(Math.cos(angle) * 1.5, index === 1 ? -1.1 : 0, Math.sin(angle) * 1.5);
		objectGroup.add(mesh);
		return { ...definition, mesh };
	});
	renderObjectList();
	pushToast("Loaded geometry gallery");
}

function selectGltfAvif() {
	releaseObjects();
	gltfLoader.load("./assets/avif-triangle.gltf", (gltf) => {
		const model = gltf.scene;
		model.scale.setScalar(1.4);
		objectGroup.add(model);
		meshes = [{ name: "glTF AVIF triangle", mesh: model }];
		renderObjectList();
		pushToast("Loaded glTF AVIF example");
	});
}

function selectExample(name) {
	if (name === "gltf-avif") {
		selectGltfAvif();
	} else {
		selectGallery();
	}
	for (const button of document.querySelectorAll("#example-list button")) {
		button.classList.toggle("active", button.dataset.example === name);
	}
}

function resize() {
	camera.aspect = window.innerWidth / window.innerHeight;
	camera.updateProjectionMatrix();
	renderer.setSize(window.innerWidth, window.innerHeight, false);
}
window.addEventListener("resize", resize);

// --- Toast stack: notifications the app raises on its own, plus ones the user triggers. ---

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

const objectList = document.querySelector("#object-list");
for (const button of document.querySelectorAll("#example-list button")) {
	button.addEventListener("click", () => selectExample(button.dataset.example));
}
selectExample("gallery");

// --- Right panel: settings that actually drive the scene. ---

let rotationSpeed = 0.35;
let lightIntensity = 1.4;

const rotationSlider = document.querySelector("#rotation-speed");
rotationSlider.addEventListener("input", () => {
	rotationSpeed = Number(rotationSlider.value) / 100;
});

const lightSlider = document.querySelector("#light-intensity");
lightSlider.addEventListener("input", () => {
	lightIntensity = (Number(lightSlider.value) / 100) * 2;
	keyLight.intensity = lightIntensity;
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

pushToast("Scene loaded");
let toastCounter = 0;
const toastMessages = ["Autosaved", "Lighting recalculated", "Camera idle", "Model cached"];
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

// --- The render loop itself: a perfectly ordinary three.js animation loop. ---

renderer.setAnimationLoop((now) => {
	objectGroup.rotation.y += rotationSpeed * 0.016;
	renderer.render(scene, camera);
	updateHud(now);
});
