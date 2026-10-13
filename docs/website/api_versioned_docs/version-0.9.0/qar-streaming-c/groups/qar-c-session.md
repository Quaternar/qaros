---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-session
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Session



## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga9da16c82285a89d4b3edcdd61f1f813f">qar_session_find_render_target</a> (const QarSession *session, const char *connection, QarPeerId *out_peer)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Find an already-connected render target from its connection string. <a href="#ga9da16c82285a89d4b3edcdd61f1f813f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga31f27a3aa22e5cdacc5a008a4a02a586">qar_session_get_graphics_device_id</a> (const QarSession *session, QarGraphicsDeviceId *out_device_id, bool *out_present)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The GPU adapter the session's graphics devices were created on. <a href="#ga31f27a3aa22e5cdacc5a008a4a02a586">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3954edc93631d87c470ee0965fe6bf64">qar_session_get_id</a> (const QarSession *session, QarSessionId *out_session_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the current session id. <a href="#ga3954edc93631d87c470ee0965fe6bf64">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gac86c6b3bc519f110d0d9db8adea0c5fa">qar_session_handle_destroy</a> (QarSession *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a session opaque handle. <a href="#gac86c6b3bc519f110d0d9db8adea0c5fa">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Functions

### qar\_session\_find\_render\_target() {#ga9da16c82285a89d4b3edcdd61f1f813f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_find_render_target (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const char * connection, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peer)</td>
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

<p>Find an already-connected render target from its connection string.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02610">2610</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_get\_graphics\_device\_id() {#ga31f27a3aa22e5cdacc5a008a4a02a586}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_get_graphics_device_id (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/structs/qargraphicsdeviceid">QarGraphicsDeviceId</a> * out_device_id, bool * out_present)</td>
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

<p>The GPU adapter the session's graphics devices were created on.</p>


<p>Every render sender's frame textures live on this adapter, so create your own graphics device on it. It is the adapter actually in use: the one a chained <a href="/api/qar-streaming-c/structs/qargraphicsdeviceid">QarGraphicsDeviceId</a> named, else the one QAROS selected - the GPU of the target named in the launch invite of an application QAROS started - else the one the library picked.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_device_id</td>
<td class="doxyParamItemDescription"><p>Receives id_type, and luid (Windows) or uuid (other platforms). Initialize it with <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae27c16b75b1a929654c669f387846e3e">qar_graphics_device_id_default()</a>; its header is left untouched so it can be chained as it is. Left unchanged when no adapter is known.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_present</td>
<td class="doxyParamItemDescription"><p>Receives whether the session knows its adapter.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>Success, or QAR_STATUS_LOGIC_ERROR for an invalid handle or a null output.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02632">2632</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_get\_id() {#ga3954edc93631d87c470ee0965fe6bf64}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_session_get_id (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/structs/qarsessionid">QarSessionId</a> * out_session_id)</td>
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

<p>Get the current session id.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02606">2606</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_session\_handle\_destroy() {#gac86c6b3bc519f110d0d9db8adea0c5fa}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_session_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * handle)</td>
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

<p>Destroy a session opaque handle.</p>


<p>Tears down the active session while keeping persisted state, so a later qar_runtime_rejoin can restore it without onboarding.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02602">2602</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
