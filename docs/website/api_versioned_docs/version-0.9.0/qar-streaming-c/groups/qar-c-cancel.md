---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-cancel
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Cancellation



## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3fa6944a5465d960a15a70d96e769c5e">qar_cancel_token_cancel</a> (QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Signal cancellation on a token. <a href="#ga3fa6944a5465d960a15a70d96e769c5e">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga01f2a7a51d0b7813a8e4b8bbb2cba6cf">qar_cancel_token_cancel_after</a> (QarCancelToken *token, uint32_t timeout_ms)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Schedule cancellation after timeout_ms. <a href="#ga01f2a7a51d0b7813a8e4b8bbb2cba6cf">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2c72ce9d5c23a1223d56d0dfe4472e6b">qar_cancel_token_create</a> (QarCancelToken **out_token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Create a new cancellation token (non-signaled). <a href="#ga2c72ce9d5c23a1223d56d0dfe4472e6b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf8dce2d2931e3d6a4f6a23f99e129bc9">qar_cancel_token_create_with_timeout</a> (QarCancelToken **out_token, uint32_t timeout_ms)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Create a token that auto-cancels after timeout_ms. <a href="#gaf8dce2d2931e3d6a4f6a23f99e129bc9">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gabcc3ab8cd3c4576b6b90fb135b18e827">qar_cancel_token_handle_destroy</a> (QarCancelToken *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a token handle, cancelling what it owns. <a href="#gabcc3ab8cd3c4576b6b90fb135b18e827">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga37002d2d7b54c8f0f3c6336dde751c89">qar_cancel_token_is_cancelled</a> (const QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if token was canceled. <a href="#ga37002d2d7b54c8f0f3c6336dde751c89">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gacf241cf6d9f265f18f6aa2458abe4427">qar_cancel_token_is_timeout</a> (const QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if token reached its timeout. <a href="#gacf241cf6d9f265f18f6aa2458abe4427">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}



<p><a id="qar_c_subscription_end"></a><b>Ending a subscription.</b> Every <span class="doxyComputerOutput">qar_*_subscribe_*</span> function hands its subscription to the cancel token passed to it (where a NULL token is allowed, the subscription is the session's alone). These calls end it: <a href="#ga3fa6944a5465d960a15a70d96e769c5e">qar_cancel_token_cancel()</a> and <a href="#gabcc3ab8cd3c4576b6b90fb135b18e827">qar_cancel_token_handle_destroy()</a> on that token, <a href="/api/qar-streaming-c/groups/qar-c-session/#gac86c6b3bc519f110d0d9db8adea0c5fa">qar_session_handle_destroy()</a> on its session, <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy()</a>. When one of them returns:</p>


<ul class="doxyList ">
<li>a callback of the subscription that was running on another thread has returned: the call waited for it,</li>
<li>no callback of the subscription starts afterwards.</li>
</ul>

<p>So <span class="doxyComputerOutput">user_state</span> only has to stay valid until that call returns, and may be freed right after it.</p>


<p><b>From inside the subscription's own callback</b> a call cannot wait for the callback it is made from. <a href="#ga3fa6944a5465d960a15a70d96e769c5e">qar_cancel_token_cancel()</a> and <a href="#gabcc3ab8cd3c4576b6b90fb135b18e827">qar_cancel_token_handle_destroy()</a> are allowed there: they return without waiting for that callback, which then finishes normally, and no further callback of the subscription starts. Keep <span class="doxyComputerOutput">user_state</span> valid until that callback returns. Do not destroy the session or the library from a callback.</p>


<p>Waiting for a callback means: do not end a subscription while holding a lock its callback takes, and do not end subscription A from a callback of subscription B while a callback of B can be ending A.</p>


<p>A token's timeout (<a href="#gaf8dce2d2931e3d6a4f6a23f99e129bc9">qar_cancel_token_create_with_timeout()</a>, <a href="#ga01f2a7a51d0b7813a8e4b8bbb2cba6cf">qar_cancel_token_cancel_after()</a>) stops its subscriptions shortly after it expires but does not wait for a running callback. Cancel or destroy the token before freeing <span class="doxyComputerOutput">user_state</span>.</p>


<div class="doxySectionDef">

## Functions

### qar\_cancel\_token\_cancel() {#ga3fa6944a5465d960a15a70d96e769c5e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_cancel_token_cancel (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Signal cancellation on a token.</p>


<p>Ends every subscription started with it: returns once no callback of those subscriptions is running on another thread, and none starts afterwards. Called from inside one of their callbacks it does not wait for that callback; see <a href="#qar_c_subscription_end">Ending a subscription</a>.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02222">2222</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_cancel\_after() {#ga01f2a7a51d0b7813a8e4b8bbb2cba6cf}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_cancel_token_cancel_after (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token, uint32_t timeout_ms)</td>
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

<p>Schedule cancellation after timeout_ms.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02225">2225</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_create() {#ga2c72ce9d5c23a1223d56d0dfe4472e6b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_cancel_token_create (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> ** out_token)</td>
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

<p>Create a new cancellation token (non-signaled).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02202">2202</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_create\_with\_timeout() {#gaf8dce2d2931e3d6a4f6a23f99e129bc9}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_cancel_token_create_with_timeout (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> ** out_token, uint32_t timeout_ms)</td>
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

<p>Create a token that auto-cancels after timeout_ms.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02204">2204</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_handle\_destroy() {#gabcc3ab8cd3c4576b6b90fb135b18e827}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_cancel_token_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * handle)</td>
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

<p>Destroy a token handle, cancelling what it owns.</p>


<p>Ends every subscription started with it, waiting for a running callback; see <a href="#qar_c_subscription_end">Ending a subscription</a>.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02213">2213</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_is\_cancelled() {#ga37002d2d7b54c8f0f3c6336dde751c89}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_cancel_token_is_cancelled (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Check if token was canceled.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02227">2227</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_cancel\_token\_is\_timeout() {#gacf241cf6d9f265f18f6aa2458abe4427}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_cancel_token_is_timeout (const <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Check if token reached its timeout.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02229">2229</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
