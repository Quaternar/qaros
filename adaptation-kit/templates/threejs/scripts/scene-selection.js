// Stage: preDocument, after threejs-capture@1. The site-specific part of this adaptation: which of
// the page's three.js scenes and renderers the users see. Edit the two functions below for your page.
//
// threejs-capture@1 records every scene the page renders each animation frame, with its camera, and
// every renderer it creates. Without this script it streams the scene last rendered with a
// perspective camera, from every renderer. That is right for most pages; a page that also renders a
// preview, a minimap or a second view needs a choice:
//
// - `selectScene(rendered)`: `rendered` is this frame's `[{ scene, camera }]` in drawing order.
//   Return the scene to stream, or null to keep the default.
// - `chooseRenderer(renderers)`: `renderers` is the set of captured renderers. Return the one to
//   stream; the others render untouched.
(() => {
	const state = (window.__qarAdaptationState = window.__qarAdaptationState || {});

	// The number of objects in a scene, its own included.
	function objectCount(scene) {
		let count = 0;
		if (typeof scene.traverse === "function") scene.traverse(() => ++count);
		else count = 1 + (scene.children ? scene.children.length : 0);
		return count;
	}

	// Default: the largest scene drawn with a perspective camera this frame. A HUD or preview scene
	// is usually smaller, or drawn with an orthographic camera.
	state.selectScene = (rendered) => {
		let best = null;
		let bestCount = -1;
		for (const { scene, camera } of rendered) {
			if (!scene || !camera || !camera.isPerspectiveCamera) continue;
			const count = objectCount(scene);
			if (count > bestCount) {
				best = scene;
				bestCount = count;
			}
		}
		return best;
	};

	// Default: the renderer with the largest canvas, the page's main view.
	state.chooseRenderer = (renderers) => {
		let best = null;
		let bestArea = -1;
		for (const renderer of renderers) {
			const canvas = renderer && renderer.domElement;
			const area = canvas ? (canvas.clientWidth || canvas.width || 0) * (canvas.clientHeight || canvas.height || 0) : 0;
			if (area > bestArea) {
				best = renderer;
				bestArea = area;
			}
		}
		return best;
	};
})();
