---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-gui
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# GUI Panels



## Typedefs Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#ga3433e0955033839df9d08901b0812355">qar_gui_panel_gesture_event_callback_t</a>)(QarGuiPanelGesture *gesture, void *user_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Receives one recognised gesture on a subscribed GUI panel. <a href="#ga3433e0955033839df9d08901b0812355">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#gae9ec14111fb83b2f45d5ddcc71937c14">qar_gui_panel_update_callback_t</a>)(QarGuiPanel *handle, void *user_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaac2a5f5e3e610cf6dcd4147c018e94f1">qar_gui_panel_gesture_get_classification</a> (QarGuiPanelGesture *gesture, QarGuiPanelGestureKind *out_kind, QarGesturePhase *out_phase)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>What this gesture is and where in its lifecycle it sits. <a href="#gaac2a5f5e3e610cf6dcd4147c018e94f1">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gac9f3ead40f527f6eda8a55f8d692a082">qar_gui_panel_gesture_get_handedness</a> (QarGuiPanelGesture *gesture, QarHandedness *out_handedness)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Which hand of the producing peer made the contact. <a href="#gac9f3ead40f527f6eda8a55f8d692a082">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga4a37c9310fcc249ec95303f15cd53fe1">qar_gui_panel_gesture_get_lifecycle</a> (QarGuiPanelGesture *gesture, QarPeerId *out_source_peer_id, uint64_t *out_gesture_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The lifecycle this gesture belongs to: the peer and the gesture id. <a href="#ga4a37c9310fcc249ec95303f15cd53fe1">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga135abbc6f2a7e26302dd84ac6945250c">qar_gui_panel_gesture_get_panel_id</a> (QarGuiPanelGesture *gesture, QarGuiPanelId *out_panel_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The panel this gesture landed on. <a href="#ga135abbc6f2a7e26302dd84ac6945250c">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf91fcb018de856aa34beea1158a52f65">qar_gui_panel_gesture_get_positions</a> (QarGuiPanelGesture *gesture, QarGuiPanelPoint *out_position, QarGuiPanelPoint *out_start_position)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Where the contact is now, and where its lifecycle began. <a href="#gaf91fcb018de856aa34beea1158a52f65">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaa4e68890f1ab895617d676873a7d5751">qar_gui_panel_gesture_get_tap_count</a> (QarGuiPanelGesture *gesture, uint8_t *out_tap_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>How many taps in this run, for QAR_GUI_PANEL_GESTURE_KIND_TAP. <a href="#gaa4e68890f1ab895617d676873a7d5751">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga38bdebba031094f4d36d4264d7c5db0d">qar_gui_panel_gesture_get_timestamp</a> (QarGuiPanelGesture *gesture, QarTimePoint *out_timestamp)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>When the recogniser emitted this gesture, on its own clock. <a href="#ga38bdebba031094f4d36d4264d7c5db0d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga9264a27d815a25f020739e3188653c0d">qar_gui_panel_get_content_stream_id</a> (QarGuiPanel *handle, QarStreamId *out_stream_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6cfc87c7d9e21c00552e3b50c5bd6ea2">qar_gui_panel_get_content_type</a> (QarGuiPanel *handle, QarGuiPanelContentType *out_type)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga393c23e3b8f738f005a99245cd923807">qar_gui_panel_get_content_uri</a> (QarGuiPanel *handle, char *out_uri, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga8ecec7a810bdfa32c313eb6e7577772e">qar_gui_panel_get_display_name</a> (QarGuiPanel *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gacc1f3b175faf098aab42a4e7e4be4c88">qar_gui_panel_get_id</a> (QarGuiPanel *handle, QarGuiPanelId *out_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga27f046c6022215093fa6305ee1aa95a1">qar_gui_panel_get_pose</a> (QarGuiPanel *handle, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga95a05d8ba3e6aaabaed399d2531307d0">qar_gui_panel_get_size</a> (QarGuiPanel *handle, QarGuiPanelSize *out_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga21a6da5242fbf890ba915c6252a54823">qar_gui_panel_get_state</a> (QarGuiPanel *handle, QarGuiPanelState *out_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae087726c925d1a3b715dd13ead00e39b">qar_gui_panel_get_visible_to_peers</a> (QarGuiPanel *handle, QarPeerId *out_peers, size_t peers_buffer_size, size_t *out_peers_written)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The peers the panel is narrowed to. An empty list means the panel is visible to every peer, not to none. <a href="#gae087726c925d1a3b715dd13ead00e39b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae387ab53a92d47c88ae6716ba3012340">qar_gui_panel_get_visible_to_peers_count</a> (QarGuiPanel *handle, size_t *out_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Number of peers in the panel's visible-to list. 0 means the panel is visible to every peer (see <a href="#ga5223541fa74a3915450b014b5ab3ab4e">qar_gui_panels_update_visible_to()</a>). <a href="#gae387ab53a92d47c88ae6716ba3012340">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga337b3189bf4ff1b129765f11ae09f91e">qar_gui_panel_handle_destroy</a> (QarGuiPanel *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga12a421b4e64da270fe24994df84e513b">qar_gui_panel_handle_is_valid</a> (QarGuiPanel *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae5f7577e17d42493143bd37b31971c9f">qar_gui_panels_change_size</a> (QarSession *session, const QarGuiPanelId *id, const QarGuiPanelSize *size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Change panel size and content scale. <a href="#gae5f7577e17d42493143bd37b31971c9f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7151e36d0c019e4eb5a30648f9ef738d">qar_gui_panels_close_panel</a> (QarSession *session, const QarGuiPanelId *id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Close and remove a panel. <a href="#ga7151e36d0c019e4eb5a30648f9ef738d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae564de938ca8c589d2a533dc7df71496">qar_gui_panels_get_or_create</a> (QarSession *session, const QarGuiPanelInit *init, QarGuiPanelId *out_panel_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the panel identified by init-&gt;common_name, creating it (via init) if it does not exist yet. <a href="/api/qar-streaming-c/structs/qarguipanelinit">QarGuiPanelInit</a> supports <a href="/api/qar-streaming-c/structs/qarguipanelurlext">QarGuiPanelUrlExt</a> for browser content and <a href="/api/qar-streaming-c/structs/qarguipanelstreamidext">QarGuiPanelStreamIdExt</a> for logical-stream content. Chain exactly one through QarGuiPanelInit::header.next. With neither extension, the panel starts with the default website content. Stream receiver resolution is taken from the session-owned sender identified by <a href="/api/qar-streaming-c/structs/qarguipanelstreamidext/#a4c0cbf762b7ea3b19924d73ce42dd5bd">QarGuiPanelStreamIdExt::stream_id</a>. <a href="#gae564de938ca8c589d2a533dc7df71496">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga52f5db0bab40989e7aa4c57a2f280dbc">qar_gui_panels_navigate_to_uri</a> (QarSession *session, const QarGuiPanelId *id, const char *uri)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Navigate an existing website panel to a URI. <a href="#ga52f5db0bab40989e7aa4c57a2f280dbc">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gadb86352d6421388d3979163ac2541516">qar_gui_panels_set_state</a> (QarSession *session, const QarGuiPanelId *id, QarGuiPanelState state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf5eed29c5451cd7b21d1987a66ff2fcc">qar_gui_panels_set_stream</a> (QarSession *session, const QarGuiPanelId *id, const QarStreamId *stream_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2a2007e302a7f04e2802a2f068f16a54">qar_gui_panels_subscribe_all_gesture_events</a> (QarSession *session, const QarGuiPanelId *id, qar_gui_panel_gesture_event_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to every gesture kind on one panel, through one callback. <a href="#ga2a2007e302a7f04e2802a2f068f16a54">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga05f67502af5c591de8f1535df62bf5ae">qar_gui_panels_subscribe_gesture_events</a> (QarSession *session, const QarGuiPanelId *id, QarGuiPanelGestureKind gesture_kind, qar_gui_panel_gesture_event_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to the gestures the rendering peers report for one panel. <a href="#ga05f67502af5c591de8f1535df62bf5ae">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gad8b91f89d8e711acfda677ffa85d0be4">qar_gui_panels_subscribe_panel_updates</a> (QarSession *session, const QarGuiPanelId *id, qar_gui_panel_update_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to updates for a specific panel. <a href="#gad8b91f89d8e711acfda677ffa85d0be4">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga82a174ff07daf6f0c08da4202f344447">qar_gui_panels_subscribe_updates</a> (QarSession *session, qar_gui_panel_update_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to updates for all panels. <a href="#ga82a174ff07daf6f0c08da4202f344447">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga88a8b5119ef11a2dc70d8b054c2ea95f">qar_gui_panels_update_pose</a> (QarSession *session, const QarGuiPanelId *id, const QarPose *pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update the pose of a panel. <a href="#ga88a8b5119ef11a2dc70d8b054c2ea95f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga5223541fa74a3915450b014b5ab3ab4e">qar_gui_panels_update_visible_to</a> (QarSession *session, const QarGuiPanelId *id, const QarPeerId **peer_ids_additions, size_t additions_count, const QarPeerId **peer_ids_removals, size_t removals_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update which peers can see the panel. <a href="#ga5223541fa74a3915450b014b5ab3ab4e">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga49e5b5fe0aac0457c7649b15f68cc737">qar_query_gui_panels</a> (QarSession *session, QarGuiPanel **out_handles, size_t handles_buffer_size, size_t *out_handles_written)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2efbe85e332acbdae9d4069b7a217ebc">qar_query_gui_panels_count</a> (QarSession *session, size_t *out_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Typedefs

### qar\_gui\_panel\_gesture\_event\_callback\_t {#ga3433e0955033839df9d08901b0812355}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_gui_panel_gesture_event_callback_t)(QarGuiPanelGesture *gesture, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Receives one recognised gesture on a subscribed GUI panel.</p>


<p>Documented on the public declaration in &lt;qar_streaming_c/v0/api.h&gt;.</p>


<p><span class="doxyComputerOutput">gesture</span> is an opaque handle to library-owned storage, valid <b>only for the duration of this call</b>. Read what you need through the qar_gui_panel_gesture_get_*() accessors and copy anything the application keeps; the handle must not be stored. <span class="doxyComputerOutput">user_state</span> is the pointer handed to the subscribe call. The callback runs on a library background thread, so it must not block and must not call back into a subscribe function.</p>


<p>It is a handle rather than a struct so the event can gain fields - a contact's depth off the glass, a pressure, a viewport rect - by appending an accessor, which is an additive change, instead of growing a struct, which would break every compiled consumer.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03375">3375</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_update\_callback\_t {#gae9ec14111fb83b2f45d5ddcc71937c14}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_gui_panel_update_callback_t)(QarGuiPanel *handle, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03331">3331</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<div class="doxySectionDef">

## Functions

### qar\_gui\_panel\_gesture\_get\_classification() {#gaac2a5f5e3e610cf6dcd4147c018e94f1}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_classification (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a8f8a9bac2855a062686bcffd7808c9cd">QarGuiPanelGestureKind</a> * out_kind, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aafb0b9385056ee6a2ccd4985bd066a9c">QarGesturePhase</a> * out_phase)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>What this gesture is and where in its lifecycle it sits.</p>


<p>Returned together because a consumer dispatches on both at once. Both are enums, so a future gesture kind or phase is a new enumerator rather than a change to this call. <span class="doxyComputerOutput">out_kind</span> always equals the kind the subscription asked for; read it so one callback can serve several subscriptions.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_kind</td>
<td class="doxyParamItemDescription"><p>Receives hover, tap, long press or drag; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_phase</td>
<td class="doxyParamItemDescription"><p>Receives started, updated, ended, canceled or instant; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03514">3514</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_handedness() {#gac9f3ead40f527f6eda8a55f8d692a082}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_handedness (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a656e3047aa5a0e853fd3f9067e0664d6">QarHandedness</a> * out_handedness)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Which hand of the producing peer made the contact.</p>


<p>Diagnostic: only one hand of a peer is active at a time, so a consumer does not need this to arbitrate.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_handedness</td>
<td class="doxyParamItemDescription"><p>Receives QAR_HANDEDNESS_LEFT or QAR_HANDEDNESS_RIGHT; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03576">3576</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_lifecycle() {#ga4a37c9310fcc249ec95303f15cd53fe1}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_lifecycle (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_source_peer_id, uint64_t * out_gesture_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>The lifecycle this gesture belongs to: the peer and the gesture id.</p>


<p>They are returned together because they are one key. The gesture id is unique only within <span class="doxyComputerOutput">out_source_peer_id</span>, so a consumer that stores lifecycles must key on the pair. The id is non-zero and stable for one hover or drag lifecycle, and 0 for tap and long press, which are instant.</p>


<p>The peer is the one whose hand produced the gesture - NOT the peer that recognised it. A mixer runs under its own peer id and publishes for the target peer it serves.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_source_peer_id</td>
<td class="doxyParamItemDescription"><p>Receives the producing peer; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_gesture_id</td>
<td class="doxyParamItemDescription"><p>Receives the lifecycle id; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03495">3495</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_panel\_id() {#ga135abbc6f2a7e26302dd84ac6945250c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_panel_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * out_panel_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>The panel this gesture landed on.</p>


<p>Always the panel the subscription named, so a callback serving one panel can skip this and read it from its user_state instead.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03550">3550</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_positions() {#gaf91fcb018de856aa34beea1158a52f65}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_positions (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarguipanelpoint">QarGuiPanelPoint</a> * out_position, <a href="/api/qar-streaming-c/structs/qarguipanelpoint">QarGuiPanelPoint</a> * out_start_position)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Where the contact is now, and where its lifecycle began.</p>


<p>Returned together because they are read together: a drag's travel is <span class="doxyComputerOutput">out_position</span> minus <span class="doxyComputerOutput">out_start_position</span>, and a frame-to-frame delta is the difference against the previous event of the same lifecycle. For instant kinds - tap and long press - the two are equal.</p>


<p>Both are in panel-content-space pixels; see <a href="/api/qar-streaming-c/structs/qarguipanelpoint">QarGuiPanelPoint</a>. A contact outside the letterboxed content rectangle cannot <em>start</em> a lifecycle, but a live one follows the clamped pixel to the edge and stays there, so a drag that overshoots commits its end value.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_position</td>
<td class="doxyParamItemDescription"><p>Receives the current contact position; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_start_position</td>
<td class="doxyParamItemDescription"><p>Receives where this lifecycle began; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03538">3538</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_tap\_count() {#gaa4e68890f1ab895617d676873a7d5751}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_tap_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, uint8_t * out_tap_count)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>How many taps in this run, for QAR_GUI_PANEL_GESTURE_KIND_TAP.</p>


<p>1, 2, 3 ... for taps close enough together in time and space. 0 for every other kind.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03586">3586</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_gesture\_get\_timestamp() {#ga38bdebba031094f4d36d4264d7c5db0d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_gesture_get_timestamp (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9261b59276b0eef7ca6c02e9a821980a">QarGuiPanelGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qartimepoint">QarTimePoint</a> * out_timestamp)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>When the recogniser emitted this gesture, on its own clock.</p>


<p>Not the device's sample time: device clocks are neither comparable nor monotonic across producers, so this is comparable only within one source peer id.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03561">3561</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_content\_stream\_id() {#ga9264a27d815a25f020739e3188653c0d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_content_stream_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/structs/qarstreamid">QarStreamId</a> * out_stream_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03625">3625</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_content\_type() {#ga6cfc87c7d9e21c00552e3b50c5bd6ea2}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_content_type (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a27d1c6547e0403704e5ef37fdc041bf1">QarGuiPanelContentType</a> * out_type)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03621">3621</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_content\_uri() {#ga393c23e3b8f738f005a99245cd923807}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_content_uri (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, char * out_uri, size_t buffer_size)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03617">3617</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_display\_name() {#ga8ecec7a810bdfa32c313eb6e7577772e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_display_name (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, char * out_buffer, size_t buffer_size)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03607">3607</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_id() {#gacc1f3b175faf098aab42a4e7e4be4c88}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * out_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03605">3605</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_pose() {#ga27f046c6022215093fa6305ee1aa95a1}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03612">3612</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_size() {#ga95a05d8ba3e6aaabaed399d2531307d0}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_size (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> * out_size)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03615">3615</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_state() {#ga21a6da5242fbf890ba915c6252a54823}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_state (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6b">QarGuiPanelState</a> * out_state)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03630">3630</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_visible\_to\_peers() {#gae087726c925d1a3b715dd13ead00e39b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_visible_to_peers (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peers, size_t peers_buffer_size, size_t * out_peers_written)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>The peers the panel is narrowed to. An empty list means the panel is visible to every peer, not to none.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03644">3644</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_get\_visible\_to\_peers\_count() {#gae387ab53a92d47c88ae6716ba3012340}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panel_get_visible_to_peers_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle, size_t * out_count)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Number of peers in the panel's visible-to list. 0 means the panel is visible to every peer (see <a href="#ga5223541fa74a3915450b014b5ab3ab4e">qar_gui_panels_update_visible_to()</a>).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03636">3636</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_handle\_destroy() {#ga337b3189bf4ff1b129765f11ae09f91e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_gui_panel_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03602">3602</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panel\_handle\_is\_valid() {#ga12a421b4e64da270fe24994df84e513b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_gui_panel_handle_is_valid (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> * handle)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03600">3600</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_change\_size() {#gae5f7577e17d42493143bd37b31971c9f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_change_size (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, const <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> * size)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Change panel size and content scale.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03274">3274</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_close\_panel() {#ga7151e36d0c019e4eb5a30648f9ef738d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_close_panel (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Close and remove a panel.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03284">3284</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_get\_or\_create() {#gae564de938ca8c589d2a533dc7df71496}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_get_or_create (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelinit">QarGuiPanelInit</a> * init, <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * out_panel_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Get the panel identified by init-&gt;common_name, creating it (via init) if it does not exist yet. <a href="/api/qar-streaming-c/structs/qarguipanelinit">QarGuiPanelInit</a> supports <a href="/api/qar-streaming-c/structs/qarguipanelurlext">QarGuiPanelUrlExt</a> for browser content and <a href="/api/qar-streaming-c/structs/qarguipanelstreamidext">QarGuiPanelStreamIdExt</a> for logical-stream content. Chain exactly one through QarGuiPanelInit::header.next. With neither extension, the panel starts with the default website content. Stream receiver resolution is taken from the session-owned sender identified by <a href="/api/qar-streaming-c/structs/qarguipanelstreamidext/#a4c0cbf762b7ea3b19924d73ce42dd5bd">QarGuiPanelStreamIdExt::stream_id</a>.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03262">3262</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_navigate\_to\_uri() {#ga52f5db0bab40989e7aa4c57a2f280dbc}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_navigate_to_uri (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, const char * uri)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Navigate an existing website panel to a URI.</p>


<p>Stream panels return QAR_STATUS_GUI_PANEL_CHANGE_NOT_PERMITTED because a panel's content type is fixed at creation.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03292">3292</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_set\_state() {#gadb86352d6421388d3979163ac2541516}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_set_state (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6b">QarGuiPanelState</a> state)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03278">3278</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_set\_stream() {#gaf5eed29c5451cd7b21d1987a66ff2fcc}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_set_stream (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, const <a href="/api/qar-streaming-c/structs/qarstreamid">QarStreamId</a> * stream_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Replace an existing stream panel's StreamId. The ID must identify a video sender owned by this session; its current layout determines receiver resolution. Website panels return QAR_STATUS_GUI_PANEL_CHANGE_NOT_PERMITTED.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03301">3301</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_subscribe\_all\_gesture\_events() {#ga2a2007e302a7f04e2802a2f068f16a54}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_subscribe_all_gesture_events (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, <a href="#ga3433e0955033839df9d08901b0812355">qar_gui_panel_gesture_event_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Subscribe to every gesture kind on one panel, through one callback.</p>


<p>The unfiltered counterpart of <a href="#ga05f67502af5c591de8f1535df62bf5ae">qar_gui_panels_subscribe_gesture_events()</a>: same stream, same handle, no kind filter. It is a separate function rather than a sentinel value in QarGuiPanelGestureKind, because that enum is also what <a href="#gaac2a5f5e3e610cf6dcd4147c018e94f1">qar_gui_panel_gesture_get_classification()</a> returns and a value that is not a kind has no meaning there.</p>


<p><b>Prefer this when you take more than one kind.</b> Everything arrives on one reader, so a hover and the drag that grows out of the same contact keep the order the producer published them in. Several per-kind subscriptions are separate readers and have no order relative to each other. Dispatch on the kind from <a href="#gaac2a5f5e3e610cf6dcd4147c018e94f1">qar_gui_panel_gesture_get_classification()</a>.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session owning the panel; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">id</td>
<td class="doxyParamItemDescription"><p>Panel to observe; must not be NULL, and the default all-zero id is rejected with QAR_STATUS_GUI_PANEL_INVALID_ID.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">callback</td>
<td class="doxyParamItemDescription"><p>Invoked for every gesture event of every kind; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>Opaque pointer passed back to <span class="doxyComputerOutput">callback</span>; may be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Cancel token ending the subscription, or NULL to keep it alive until the session is destroyed. See <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> Success once the subscription is running.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03471">3471</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_subscribe\_gesture\_events() {#ga05f67502af5c591de8f1535df62bf5ae}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_subscribe_gesture_events (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a8f8a9bac2855a062686bcffd7808c9cd">QarGuiPanelGestureKind</a> gesture_kind, <a href="#ga3433e0955033839df9d08901b0812355">qar_gui_panel_gesture_event_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Subscribe to the gestures the rendering peers report for one panel.</p>


<p>Panel input is one stream of gestures: hover, tap, long press and drag, each carrying the QarGesturePhase lifecycle and a gesture id that pairs a started phase with its updates and its end. The peer that draws the panel recognises them from its user's hand joints - the thresholds are physical, in millimetres of hand travel against tracking jitter, which a consumer seeing only content pixels cannot reproduce. The consumer decides for itself whether a drag is a slider or a scroll.</p>


<p><b>One subscription delivers one kind</b>, the same way <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gad460dbbdc714a5886ea59df4c5a6ae6a">qar_app_volumes_subscribe_gesture_events()</a> does. A consumer that wants several kinds subscribes several times. Every kind still travels on the one panel topic and the filtering happens on the receiving side, so a subscription never misses an event of its kind; what it does not get is an order guarantee <em>between</em> its subscriptions. Where that matters - a hover and a drag from the same contact - take both kinds on one subscription's worth of state and reconcile them on the gesture id, which is shared across kinds within a source peer.</p>


<p><b>Several producers publish here.</b> Recognition happens once per user, in the mixer that serves that user, so a panel several users can see carries one publisher per user on this single topic. Therefore:</p>


<ul class="doxyList ">
<li>The gesture id is unique only within a source peer id. Key lifecycles on the pair that <a href="#ga4a37c9310fcc249ec95303f15cd53fe1">qar_gui_panel_gesture_get_lifecycle()</a> returns together: two users' first drags both carry gesture id 1. A consumer that keys on the gesture id alone works perfectly until a second user joins.</li>
<li>Ordering holds within one producer's stream, not across producers. That is sufficient, because no lifecycle spans two producers.</li>
<li>A source peer id never seen before is a new user, not an error, and per-user interaction state - a slider's value while it is being dragged - is held per peer so two users do not fight over one.</li>
<li>A live lifecycle is republished every 250 ms even when nothing changed. Treat one silent for three of those intervals as cancelled. That, and not the peer list, is what recovers a producer that crashed: the source peer id is the headset, while the process that can die is the mixer, which the event does not name.</li>
<li>A started phase that was never seen is recoverable, because every update carries its start position.</li>
</ul>

<p>Every panel reports gestures, QAR_GUI_PANEL_CONTENT_TYPE_WEBSITE_URI panels included: nothing branches on content type on the publish path.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session owning the panel; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">id</td>
<td class="doxyParamItemDescription"><p>Panel to observe; must not be NULL, and the default all-zero id is rejected with QAR_STATUS_GUI_PANEL_INVALID_ID.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture_kind</td>
<td class="doxyParamItemDescription"><p>The single gesture kind this subscription delivers; subscribe again for another kind.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">callback</td>
<td class="doxyParamItemDescription"><p>Invoked for every gesture event of that kind; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>Opaque pointer passed back to <span class="doxyComputerOutput">callback</span>; may be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Cancel token ending the subscription, or NULL to keep it alive until the session is destroyed. See <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> Success once the subscription is running.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03436">3436</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_subscribe\_panel\_updates() {#gad8b91f89d8e711acfda677ffa85d0be4}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_subscribe_panel_updates (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, <a href="#gae9ec14111fb83b2f45d5ddcc71937c14">qar_gui_panel_update_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Subscribe to updates for a specific panel.</p>


<p><span class="doxyComputerOutput">token</span> is required; see <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a</a>subscription" for when callbacks stop and how long <span class="doxyComputerOutput">user_state</span> must live.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03352">3352</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_subscribe\_updates() {#ga82a174ff07daf6f0c08da4202f344447}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_subscribe_updates (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="#gae9ec14111fb83b2f45d5ddcc71937c14">qar_gui_panel_update_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Subscribe to updates for all panels.</p>


<p><span class="doxyComputerOutput">token</span> is required; see <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a</a>subscription" for when callbacks stop and how long <span class="doxyComputerOutput">user_state</span> must live.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03340">3340</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_update\_pose() {#ga88a8b5119ef11a2dc70d8b054c2ea95f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_update_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, const <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * pose)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Update the pose of a panel.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03269">3269</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_gui\_panels\_update\_visible\_to() {#ga5223541fa74a3915450b014b5ab3ab4e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_gui_panels_update_visible_to (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> * id, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> ** peer_ids_additions, size_t additions_count, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> ** peer_ids_removals, size_t removals_count)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Update which peers can see the panel.</p>


<p>The visible-to list narrows who sees the panel. <b>An empty list means the panel is visible to every peer</b>: a new panel starts empty, and removing the last peer from the list makes the panel visible to everyone again, never to no one. To hide a panel from everyone, change its state instead.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the panel.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">id</td>
<td class="doxyParamItemDescription"><p>Panel to change.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer_ids_additions</td>
<td class="doxyParamItemDescription"><p>Peers to add; may be NULL when additions_count is 0.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">additions_count</td>
<td class="doxyParamItemDescription"><p>Number of entries in <span class="doxyComputerOutput">peer_ids_additions</span>.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer_ids_removals</td>
<td class="doxyParamItemDescription"><p>Peers to remove; may be NULL when removals_count is 0.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">removals_count</td>
<td class="doxyParamItemDescription"><p>Number of entries in <span class="doxyComputerOutput">peer_ids_removals</span>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>Success, or the panel error (e.g. QAR_STATUS_GUI_PANEL_INVALID_ID).</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03323">3323</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_gui\_panels() {#ga49e5b5fe0aac0457c7649b15f68cc737}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_gui_panels (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a> ** out_handles, size_t handles_buffer_size, size_t * out_handles_written)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03593">3593</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_gui\_panels\_count() {#ga2efbe85e332acbdae9d4069b7a217ebc}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_gui_panels_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, size_t * out_count)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel inline">inline</span>
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03591">3591</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
