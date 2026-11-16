// Stage: afterLoad. Recipe 4: QAROS streams the renderer of the top-level page. This page shows its
// 3D content in an iframe, so the top-level page has no renderer and the source never starts: open
// the framed page at top level instead (replace, so Back does not return to a page that redirects).
(() => {
	if (window !== window.top) return;
	const frame = document.querySelector("iframe#viewer");
	if (frame && frame.src) location.replace(frame.src);
})();
