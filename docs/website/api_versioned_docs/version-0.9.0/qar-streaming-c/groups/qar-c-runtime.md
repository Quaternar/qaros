---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-runtime
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Runtime



## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6a057004f666fdb8632dded376d634c3">qar_runtime_create</a> (const QarRuntimeInit *init, QarRuntime **out_runtime)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Create a runtime instance that can host sessions and streams. <a href="#ga6a057004f666fdb8632dded376d634c3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a> (QarRuntime *runtime)</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga23511fe544a6e7d8dc75ec86195f8142">qar_runtime_handle_destroy</a> (QarRuntime *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a runtime opaque handle without shutting down the runtime. <a href="#ga23511fe544a6e7d8dc75ec86195f8142">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Functions

### qar\_runtime\_create() {#ga6a057004f666fdb8632dded376d634c3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_runtime_create (const <a href="/api/qar-streaming-c/structs/qarruntimeinit">QarRuntimeInit</a> * init, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a> ** out_runtime)</td>
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

<p>Create a runtime instance that can host sessions and streams.</p>


<p><a href="/api/qar-streaming-c/structs/qarruntimeinit/#aa3da4381535f98bb43257058308d9f86">QarRuntimeInit::runtime_binaries_folder_path</a> must name the SDK <span class="doxyComputerOutput">bin</span> folder, the one the shared library itself was loaded from. That is where the runtime finds <span class="doxyComputerOutput">qar-runtime-launcher</span>, the process that answers <a href="/api/qar-streaming-c/groups/qar-c-target-apps/#ga872331f80701d4fe4f35293b0492ecfc">qar_session_invite_target_app()</a> and starts the device applications. Left empty, the runtime still creates sessions but no peer can ever be invited.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">init</td>
<td class="doxyParamItemDescription"><p>Runtime initialization parameters.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_runtime</td>
<td class="doxyParamItemDescription"><p>Out pointer receiving the created runtime handle.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> Success or error code.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02259">2259</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_runtime\_destroy() {#ga66609ed3e8d28ebda58404eaabd27188}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_runtime_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a> * runtime)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02261">2261</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_runtime\_handle\_destroy() {#ga23511fe544a6e7d8dc75ec86195f8142}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_runtime_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a> * handle)</td>
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

<p>Destroy a runtime opaque handle without shutting down the runtime.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02243">2243</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
