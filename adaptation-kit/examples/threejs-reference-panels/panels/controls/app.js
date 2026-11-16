// The controls panel: asks the page for the next colour and shows how often it changed.
const panel = window.qaros && window.qaros.panel;
let id = 0;
document.getElementById("next").addEventListener("click", () => {
	panel && panel.send({ t: "call", id: ++id, name: "nextColor", args: [] });
});
panel && panel.onMessage((msg) => {
	if (msg && msg.t === "state" && msg.key === "count") {
		document.getElementById("count").textContent = String(msg.value);
	}
});
// Ask for the state on load: a reopened panel may show an older page's state.
panel && panel.send({ t: "call", id: ++id, name: "getState", args: [] });
