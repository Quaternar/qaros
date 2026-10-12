---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-peer
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Peer Management



## Typedefs Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#ga21ece6cc236c3f8ba69ffa702b02cb81">qar_peer_update_callback_t</a>)(QarPeerSpec *handle, void *user_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Callback invoked when a peer spec changes. <a href="#ga21ece6cc236c3f8ba69ffa702b02cb81">More...</a></p>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga134eea0b4354b3b44a1acfc2fb5bba41">qar_peer_spec_get_app_custom_peer_info</a> (QarPeerSpec *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get custom peer info string (application-defined). <a href="#ga134eea0b4354b3b44a1acfc2fb5bba41">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf8eb8e79b55a5aa82ac5a9cb51908609">qar_peer_spec_get_app_state</a> (QarPeerSpec *handle, QarAppState *out_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get application state for this peer. <a href="#gaf8eb8e79b55a5aa82ac5a9cb51908609">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga037f5fefac98d219d829a76a86138a87">qar_peer_spec_get_app_version</a> (QarPeerSpec *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get application version string. <a href="#ga037f5fefac98d219d829a76a86138a87">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gadda6ee675dc92a57c40fa8056bae8bd5">qar_peer_spec_get_display_name</a> (QarPeerSpec *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get display name string into provided buffer. <a href="#gadda6ee675dc92a57c40fa8056bae8bd5">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gad6b5719db7ca585044a848adb524c9a5">qar_peer_spec_get_id</a> (QarPeerSpec *handle, QarPeerId *out_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get peer id from spec. <a href="#gad6b5719db7ca585044a848adb524c9a5">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae4d8aacb56cc485bf011e082537d215d">qar_peer_spec_get_room_tag</a> (QarPeerSpec *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get current room tag string. <a href="#gae4d8aacb56cc485bf011e082537d215d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga48a2d8fd66d34104a05fcd4181102d26">qar_peer_spec_get_version_id</a> (QarPeerSpec *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get internal version id string. <a href="#ga48a2d8fd66d34104a05fcd4181102d26">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga49d9669e5f3c5ba0b106bf9d3af34955">qar_peer_spec_handle_destroy</a> (QarPeerSpec *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a peer spec handle. <a href="#ga49d9669e5f3c5ba0b106bf9d3af34955">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga0219d0aaaeae5075f29650585ba99e9a">qar_peer_spec_handle_is_valid</a> (QarPeerSpec *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if a peer spec handle is valid. <a href="#ga0219d0aaaeae5075f29650585ba99e9a">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga68f7921fe6b6a8c3614a02efb168fcd3">qar_peer_spec_is_target_app</a> (QarPeerSpec *handle, bool *out_is_target_app)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Whether this peer is a target app: a device or viewer that shows rendered content (a headset, the Android streamer, a visualizer receiver). <a href="#ga68f7921fe6b6a8c3614a02efb168fcd3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae4d60a12fcbf093437a8e925f9318b53">qar_peer_subscribe_updates</a> (QarSession *session, qar_peer_update_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe for asynchronous peer spec updates. <a href="#gae4d60a12fcbf093437a8e925f9318b53">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga60a66cf46946cf8040271ebb0ebfae86">qar_peer_update_display_name</a> (QarSession *session, const char *name)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update current peer's display name. <a href="#ga60a66cf46946cf8040271ebb0ebfae86">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga865af89c95133decbe63c0a9bb159e7f">qar_query_peer_specs</a> (QarSession *session, QarPeerSpec **out_handles, size_t handles_buffer_size, size_t *out_handles_written)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Enumerate peer specs into user-provided array. <a href="#ga865af89c95133decbe63c0a9bb159e7f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaafa17968693ca1c85d0748e34fcd2093">qar_query_peer_specs_count</a> (QarSession *session, size_t *out_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Query number of known peer specs. <a href="#gaafa17968693ca1c85d0748e34fcd2093">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gadb8e55937c482ffddce390fa49200ec8">qar_session_get_my_spec</a> (const QarSession *session, QarPeerSpec **out_handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get spec describing the current device/peer of a session. <a href="#gadb8e55937c482ffddce390fa49200ec8">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Typedefs

### qar\_peer\_update\_callback\_t {#ga21ece6cc236c3f8ba69ffa702b02cb81}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_peer_update_callback_t)(QarPeerSpec *handle, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Callback invoked when a peer spec changes.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02865">2865</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<div class="doxySectionDef">

## Functions

### qar\_peer\_spec\_get\_app\_custom\_peer\_info() {#ga134eea0b4354b3b44a1acfc2fb5bba41}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_app_custom_peer_info (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get custom peer info string (application-defined).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02811">2811</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_app\_state() {#gaf8eb8e79b55a5aa82ac5a9cb51908609}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_app_state (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a7acd6ca429e5534b5ea381fb3238392b">QarAppState</a> * out_state)</td>
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

<p>Get application state for this peer.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02816">2816</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_app\_version() {#ga037f5fefac98d219d829a76a86138a87}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_app_version (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get application version string.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02807">2807</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_display\_name() {#gadda6ee675dc92a57c40fa8056bae8bd5}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_display_name (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get display name string into provided buffer.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02802">2802</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_id() {#gad6b5719db7ca585044a848adb524c9a5}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_id)</td>
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

<p>Get peer id from spec.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02799">2799</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_room\_tag() {#gae4d8aacb56cc485bf011e082537d215d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_room_tag (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get current room tag string.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02822">2822</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_get\_version\_id() {#ga48a2d8fd66d34104a05fcd4181102d26}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_get_version_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get internal version id string.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02818">2818</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_handle\_destroy() {#ga49d9669e5f3c5ba0b106bf9d3af34955}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_peer_spec_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle)</td>
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

<p>Destroy a peer spec handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02796">2796</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_handle\_is\_valid() {#ga0219d0aaaeae5075f29650585ba99e9a}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_peer_spec_handle_is_valid (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle)</td>
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

<p>Check if a peer spec handle is valid.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02794">2794</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_spec\_is\_target\_app() {#ga68f7921fe6b6a8c3614a02efb168fcd3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_spec_is_target_app (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> * handle, bool * out_is_target_app)</td>
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

<p>Whether this peer is a target app: a device or viewer that shows rendered content (a headset, the Android streamer, a visualizer receiver).</p>


<p>See "Basic flow" at the top of this header for how a source application uses it to find the targets it renders for.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>Peer spec handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_is_target_app</td>
<td class="doxyParamItemDescription"><p>Receives true for a target app, false for anything else (a hub, a source application, pipeline infrastructure).</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>Success, or QAR_STATUS_LOGIC_ERROR for an invalid handle or a null output.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02839">2839</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_subscribe\_updates() {#gae4d60a12fcbf093437a8e925f9318b53}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_subscribe_updates (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="#ga21ece6cc236c3f8ba69ffa702b02cb81">qar_peer_update_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Subscribe for asynchronous peer spec updates.</p>


<p>The push form of <a href="#ga865af89c95133decbe63c0a9bb159e7f">qar_query_peer_specs()</a>, and the way to learn that a peer invited with <a href="/api/qar-streaming-c/groups/qar-c-target-apps/#ga872331f80701d4fe4f35293b0492ecfc">qar_session_invite_target_app()</a> has joined and reached QAR_APP_STATE_RUNNING.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Required; cancelling or destroying it ends the subscription and waits for a running callback, see <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02879">2879</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_peer\_update\_display\_name() {#ga60a66cf46946cf8040271ebb0ebfae86}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_peer_update_display_name (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const char * name)</td>
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

<p>Update current peer's display name.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02863">2863</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_peer\_specs() {#ga865af89c95133decbe63c0a9bb159e7f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_peer_specs (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> ** out_handles, size_t handles_buffer_size, size_t * out_handles_written)</td>
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

<p>Enumerate peer specs into user-provided array.</p>


<p>Size the array with <a href="#gaafa17968693ca1c85d0748e34fcd2093">qar_query_peer_specs_count()</a>. The list also contains this process' own peer; <a href="#gadb8e55937c482ffddce390fa49200ec8">qar_session_get_my_spec()</a> identifies it. Match <a href="#gad6b5719db7ca585044a848adb524c9a5">qar_peer_spec_get_id()</a> against the id <a href="/api/qar-streaming-c/groups/qar-c-target-apps/#ga872331f80701d4fe4f35293b0492ecfc">qar_session_invite_target_app()</a> returned to find an invited peer, and read <a href="#gaf8eb8e79b55a5aa82ac5a9cb51908609">qar_peer_spec_get_app_state()</a> to see whether it is ready.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02855">2855</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_peer\_specs\_count() {#gaafa17968693ca1c85d0748e34fcd2093}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_peer_specs_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, size_t * out_count)</td>
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

<p>Query number of known peer specs.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02845">2845</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_get\_my\_spec() {#gadb8e55937c482ffddce390fa49200ec8}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_get_my_spec (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5962a4a5b94738c2492f69f710497430">QarPeerSpec</a> ** out_handle)</td>
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

<p>Get spec describing the current device/peer of a session.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02842">2842</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
