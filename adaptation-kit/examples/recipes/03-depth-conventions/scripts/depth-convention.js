// Stage: afterLoad, after the render-frame libraries. Recipe 3: reports the depth convention native
// code asks for. The kit's render-frame libraries render standard depth (cleared to 1, the far
// plane, nearest = smallest): the "regular" convention. A request asking for another one
// ("inverted" for reversed Z, "infiniteFar") is reported once in the console.
(() => {
	const reported = new Set();
	window.addEventListener("qaros-render-frame", ({ detail }) => {
		for (const target of detail.targets || []) {
			const convention = target.cameras && target.cameras.left && target.cameras.left.depthConvention;
			if (!convention || convention === "regular" || reported.has(convention)) continue;
			reported.add(convention);
			console.warn(`QAR adaptation: the request asks for "${convention}" depth; this page packs "regular" depth.`);
		}
	});
})();
