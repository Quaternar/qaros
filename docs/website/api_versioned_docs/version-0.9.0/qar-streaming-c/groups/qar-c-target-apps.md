---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-target-apps
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Target Apps



## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga872331f80701d4fe4f35293b0492ecfc">qar_session_invite_target_app</a> (QarSession *session, const QarSessionInviteTargetAppInit *init, qar_progress_callback_t on_progress, void *progress_state, QarCancelToken *cancel, QarPeerId *out_peer)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Ask this machine's hub to start the target app for one AR device. <a href="#ga872331f80701d4fe4f35293b0492ecfc">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gabc4f5dcaa403173487a890a42b189c22">qar_session_invite_target_app_async</a> (QarSession *session, const QarSessionInviteTargetAppInit *init, qar_target_app_invite_result_callback_t result_callback, qar_progress_callback_t update_callback, void *user_state, QarCancelToken *cancel)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Ask this machine's hub to start the target app for one AR device. <a href="#gabc4f5dcaa403173487a890a42b189c22">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6d86b858ae95c80909817a97f3cdce42">qar_session_kill_target_app</a> (QarSession *session, const QarPeerId *target_app, qar_progress_callback_t on_progress, void *progress_state, QarCancelToken *cancel)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Ask this machine's hub to stop a target app it started. <a href="#ga6d86b858ae95c80909817a97f3cdce42">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga899c8270511e09594f580cd4d78bd999">qar_session_kill_target_app_async</a> (QarSession *session, const QarPeerId *target_app, qar_target_app_kill_result_callback_t result_callback, qar_progress_callback_t update_callback, void *user_state, QarCancelToken *cancel)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Ask this machine's hub to stop a target app it started. <a href="#ga899c8270511e09594f580cd4d78bd999">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga112880c4eab7a9efe9682eef3b383d10">qar_target_app_connection_string_android</a> (const char *host, char *buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Build the connection string for an Android streamer target app. <a href="#ga112880c4eab7a9efe9682eef3b383d10">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6ddc4b4052aff76c3dd06e47f1836cb8">qar_target_app_connection_string_hololens</a> (const char *host, char *buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Build the connection string for a HoloLens target app. <a href="#ga6ddc4b4052aff76c3dd06e47f1836cb8">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga281806137d9d02453e731d27276e1ee3">qar_target_app_connection_string_quest</a> (const char *host, char *buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Build the connection string for a Meta Quest target app. <a href="#ga281806137d9d02453e731d27276e1ee3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}



<p>Starting and stopping the target app for one AR device. A <em>target app</em> is the process a hub runs on this machine to drive a HoloLens or an Android headset; the hub owns its whole lifecycle, and these calls ask the hub to act.</p>


<p>Two rules hold for every call in this group.</p>


<ul class="doxyList ">
<li><b>Only this machine's hub answers.</b> Both the invite and the kill carry this machine's hardware id, and every other hub in the session ignores them. An application can start and stop target apps on its own hub and no other. An operator working in the QAROS Visualizer can reach any hub; an application cannot, by construction.</li>
<li><b>An invite is one-time.</b> The hub remembers the device so it shows up in the visualizer's device list, but never marks it for automatic re-invite. Only an operator in the visualizer can do that.</li>
</ul>

<p>A device is served by one hub at a time. Inviting a device that another hub is already hosting stops it there first, then starts it here.</p>


<div class="doxySectionDef">

## Functions

### qar\_session\_invite\_target\_app() {#ga872331f80701d4fe4f35293b0492ecfc}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_invite_target_app (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarsessioninvitetargetappinit">QarSessionInviteTargetAppInit</a> * init, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aedac0d2e5a28073a86b3505097fdf964">qar_progress_callback_t</a> on_progress, void * progress_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * cancel, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peer)</td>
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

<p>Ask this machine's hub to start the target app for one AR device.</p>


<p>Blocking form of qar_session_invite_target_app_async. Returns once the hub has started the target app and its peer has joined the session.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">init</td>
<td class="doxyParamItemDescription"><p>Invite parameters; see <a href="/api/qar-streaming-c/structs/qarsessioninvitetargetappinit">QarSessionInviteTargetAppInit</a>.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">on_progress</td>
<td class="doxyParamItemDescription"><p>Optional progress callback (can be NULL). Never fires after this call returns.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">progress_state</td>
<td class="doxyParamItemDescription"><p>User pointer passed to on_progress unchanged.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">cancel</td>
<td class="doxyParamItemDescription"><p>Optional cancellation token (can be NULL).</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_peer</td>
<td class="doxyParamItemDescription"><p>Receives the target app's peer id. Untouched on failure.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Return Values</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">QAR_STATUS_ARGUMENT_NOT_SUPPORTED</td>
<td class="doxyParamItemDescription"><p>the connection string is malformed, or names a device kind that cannot be invited - rebuild it with a qar_target_app_connection_string_* helper.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02684">2684</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_invite\_target\_app\_async() {#gabc4f5dcaa403173487a890a42b189c22}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_invite_target_app_async (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarsessioninvitetargetappinit">QarSessionInviteTargetAppInit</a> * init, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#add4949d3d69ddab4b833633bd6775ef6">qar_target_app_invite_result_callback_t</a> result_callback, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aedac0d2e5a28073a86b3505097fdf964">qar_progress_callback_t</a> update_callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * cancel)</td>
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

<p>Ask this machine's hub to start the target app for one AR device.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">init</td>
<td class="doxyParamItemDescription"><p>Invite parameters; see <a href="/api/qar-streaming-c/structs/qarsessioninvitetargetappinit">QarSessionInviteTargetAppInit</a>.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">result_callback</td>
<td class="doxyParamItemDescription"><p>Fires exactly once with the outcome. Required.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">update_callback</td>
<td class="doxyParamItemDescription"><p>Optional progress callback (can be NULL).</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>User pointer passed to both callbacks unchanged.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">cancel</td>
<td class="doxyParamItemDescription"><p>Optional cancellation token (can be NULL).</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02702">2702</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_kill\_target\_app() {#ga6d86b858ae95c80909817a97f3cdce42}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_kill_target_app (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * target_app, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aedac0d2e5a28073a86b3505097fdf964">qar_progress_callback_t</a> on_progress, void * progress_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * cancel)</td>
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

<p>Ask this machine's hub to stop a target app it started.</p>


<p>Stops the target app and the whole streaming pipeline feeding it. A request for a target app this machine's hub did not start is not served by any hub, so it ends in a timeout rather than an authorization error - there is deliberately no way for an application to learn about, or act on, another hub's target apps.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">target_app</td>
<td class="doxyParamItemDescription"><p>Peer id returned by qar_session_invite_target_app.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">on_progress</td>
<td class="doxyParamItemDescription"><p>Optional progress callback (can be NULL).</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">progress_state</td>
<td class="doxyParamItemDescription"><p>User pointer passed to on_progress unchanged.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">cancel</td>
<td class="doxyParamItemDescription"><p>Optional cancellation token (can be NULL).</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02726">2726</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_kill\_target\_app\_async() {#ga899c8270511e09594f580cd4d78bd999}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_kill_target_app_async (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * target_app, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1550b8f3b42e6419c67ae5c17b915871">qar_target_app_kill_result_callback_t</a> result_callback, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aedac0d2e5a28073a86b3505097fdf964">qar_progress_callback_t</a> update_callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * cancel)</td>
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

<p>Ask this machine's hub to stop a target app it started.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">target_app</td>
<td class="doxyParamItemDescription"><p>Peer id returned by qar_session_invite_target_app.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">result_callback</td>
<td class="doxyParamItemDescription"><p>Fires exactly once with the outcome. Required.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">update_callback</td>
<td class="doxyParamItemDescription"><p>Optional progress callback (can be NULL).</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>User pointer passed to both callbacks unchanged.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">cancel</td>
<td class="doxyParamItemDescription"><p>Optional cancellation token (can be NULL).</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02743">2743</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_target\_app\_connection\_string\_android() {#ga112880c4eab7a9efe9682eef3b383d10}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_target_app_connection_string_android (const char * host, char * buffer, size_t buffer_size)</td>
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

<p>Build the connection string for an Android streamer target app.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">host</td>
<td class="doxyParamItemDescription"><p>Device hostname or IP address. Must not be NULL or empty.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer</td>
<td class="doxyParamItemDescription"><p>Output buffer for the NUL-terminated connection string.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer_size</td>
<td class="doxyParamItemDescription"><p>Size of buffer in bytes.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02778">2778</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_target\_app\_connection\_string\_hololens() {#ga6ddc4b4052aff76c3dd06e47f1836cb8}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_target_app_connection_string_hololens (const char * host, char * buffer, size_t buffer_size)</td>
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

<p>Build the connection string for a HoloLens target app.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">host</td>
<td class="doxyParamItemDescription"><p>Device hostname or IP address. Must not be NULL or empty.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer</td>
<td class="doxyParamItemDescription"><p>Output buffer for the NUL-terminated connection string.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer_size</td>
<td class="doxyParamItemDescription"><p>Size of buffer in bytes.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02758">2758</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_target\_app\_connection\_string\_quest() {#ga281806137d9d02453e731d27276e1ee3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_target_app_connection_string_quest (const char * host, char * buffer, size_t buffer_size)</td>
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

<p>Build the connection string for a Meta Quest target app.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">host</td>
<td class="doxyParamItemDescription"><p>Device hostname or IP address. Must not be NULL or empty.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer</td>
<td class="doxyParamItemDescription"><p>Output buffer for the NUL-terminated connection string.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">buffer_size</td>
<td class="doxyParamItemDescription"><p>Size of buffer in bytes.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02768">2768</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
