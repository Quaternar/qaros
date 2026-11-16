// afterLoad adaptation script: opens the "controls" served panel (panels/controls) and answers it.
// The sample of a web app source with its own AR panels (see the GUI panel authoring guide).
// The page may be an ES module (no window.THREE, no window.scene): the scene comes from the
// three.js capture state, and colours are set on a material the scene already has.
(() => {
	if (!window.qaros || !window.qaros.panels) {
		return;
	}
	const colors = [0x7a2a1d, 0x1f6b3a, 0x3a3a8a, 0xd0a020];
	let count = 0;
	let tinted = null;
	const sceneOf = () => (window.__qarAdaptationState && window.__qarAdaptationState.scene) || null;
	const postState = (name, user) => qaros.panels.post(name, { t: "state", key: "count", value: count }, user);
	// The mesh with the most vertices gets a material of its own, so only it changes colour.
	function tintable(scene) {
		if (tinted) {
			return tinted;
		}
		let biggest = null;
		scene.traverse((o) => {
			if (!o.isMesh || !o.material || Array.isArray(o.material) || !o.material.color) {
				return;
			}
			const size = o.geometry && o.geometry.attributes && o.geometry.attributes.position
				? o.geometry.attributes.position.count : 0;
			if (!biggest || size > biggest.size) {
				biggest = { mesh: o, size };
			}
		});
		if (!biggest) {
			return null;
		}
		biggest.mesh.material = biggest.mesh.material.clone();
		tinted = biggest.mesh.material;
		return tinted;
	}
	// Treat every message as user input: act only on the calls this page exposes.
	qaros.panels.onMessage((name, user, msg) => {
		if (!msg || msg.t !== "call") {
			return;
		}
		if (msg.name === "nextColor") {
			count++;
			const scene = sceneOf();
			const material = scene && tintable(scene);
			if (material) {
				material.color.setHex(colors[count % colors.length]);
			}
			qaros.panels.post(name, { t: "result", id: msg.id, ok: !!material }, user);
			postState(null);
		}
		if (msg.name === "getState") {
			postState(name, user);
		}
	});
	qaros.panels
		.open({
			name: "controls",
			displayName: "Model controls",
			entry: "controls/index.html",
			sizeMeters: { width: 0.45, height: 0.35 },
			offset: { x: 0.6, y: 0.1, z: 0.3 },
		})
		// A reopened panel keeps its loaded page: post the state again after every open.
		.then(() => postState("controls"))
		.catch((error) => console.error("QAR adaptation: the controls panel did not open", error));
})();
