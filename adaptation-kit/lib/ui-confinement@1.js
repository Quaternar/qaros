// @qaros-library ui-confinement@1.0
// @stage afterLoad
//
// Confines an application's own interface to the page region. Engine-agnostic: the only convention
// it depends on is that the application collects its entire interface (panels, toasts, a popup,
// an overlay) under one `#app-ui` container, positioned `fixed` at the full viewport with
// `overflow: hidden`, each piece `position: absolute` against that container rather than
// `position: fixed` against the viewport. That is ordinary application CSS, not an adaptation hook,
// but it makes shrinking one box the whole of "confine the interface to the page region":
// clipping `#app-ui` clips its absolutely-positioned descendants with it. The kit's reference apps
// follow it; a third-party website usually does not, and keeps its own layout without this
// library.
//
// It is the other half of canvas takeover alongside an engine-specific capture script: that one
// keeps the application's rendered view inside the page region, this one keeps its chrome there.
(() => {
	const pageRegion = window.__qarAdaptationPageRegion || {
		width: __PAGE_REGION_WIDTH__,
		height: __PAGE_REGION_HEIGHT__,
	};
	const style = document.createElement("style");
	style.textContent = `
		#app-ui {
			width: var(--qar-page-width, ${pageRegion.width}px) !important;
			height: var(--qar-page-height, ${pageRegion.height}px) !important;
		}
	`;
	document.head.appendChild(style);
})();
