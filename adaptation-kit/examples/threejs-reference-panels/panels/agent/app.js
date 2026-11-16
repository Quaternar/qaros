// An agent panel (opened with the MCP panel_create): its messages go to panel_messages_tail, and
// whatever the agent posts back (panel_post) is shown.
const panel = window.qaros && window.qaros.panel;
let taps = 0;
document.getElementById("ask").addEventListener("click", () => {
	taps++;
	panel && panel.send({ t: "question", text: "hello agent", taps });
});
panel && panel.onMessage((msg) => {
	document.getElementById("answer").textContent = JSON.stringify(msg);
});
panel && panel.send({ t: "ready", panel: "agent" });
