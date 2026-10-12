---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarsessioninvitetargetappinit
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarSessionInviteTargetAppInit` Struct

<p>Ask this machine's hub to start a target app for one AR device. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarSessionInviteTargetAppInit { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">const char *</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ac70e9b50cd2fc54e76a5f5040de918b5">connection_string</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Required; see above for the accepted spellings. <a href="#ac70e9b50cd2fc54e76a5f5040de918b5">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarstructureheader">QarStructureHeader</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a9ce894c5c8af0f50b9eb078b6d3596f4">header</a></td>
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

## Description {#details}

<p>Ask this machine's hub to start a target app for one AR device.</p>


<p><span class="doxyComputerOutput">connection_string</span> names the device kind and its address:</p>


<ul class="doxyList ">
<li><span class="doxyComputerOutput">"hololens:&lt;host&gt;"</span> — HoloLens</li>
<li><span class="doxyComputerOutput">"quest:&lt;host&gt;"</span> — Meta Quest</li>
<li><span class="doxyComputerOutput">"android:&lt;host&gt;"</span> — Android streamer <span class="doxyComputerOutput">&lt;host&gt;</span> is a hostname or an IP address; the two are interchangeable. A bare <span class="doxyComputerOutput">"&lt;host&gt;"</span> with no scheme is accepted as the legacy spelling of <span class="doxyComputerOutput">"hololens:&lt;host&gt;"</span>.</li>
</ul>

<p>Only these device kinds can be invited. Anything else — a visualizer, a desktop peer — is rejected with QAR_STATUS_ARGUMENT_NOT_SUPPORTED.</p>


<p>The invite is served only by the hub running on this machine, and is always a one-time invite: the hub remembers the device so it is visible in the visualizer, but never marks it for automatic re-invite. Only an operator acting in the visualizer can do that.</p>


<p>If another hub in the federation is already hosting a target app for the same host, that one is stopped before this one starts — a device is served by one hub at a time.</p>


<p>Definition at line 1560 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### connection\_string {#ac70e9b50cd2fc54e76a5f5040de918b5}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">const char* QarSessionInviteTargetAppInit::connection_string</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Required; see above for the accepted spellings.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01565">1565</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### header {#a9ce894c5c8af0f50b9eb078b6d3596f4}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarStructureHeader QarSessionInviteTargetAppInit::header</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>QAR_STRUCTURE_TYPE_SESSION_INVITE_TARGET_APP_INIT</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01563">1563</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

The documentation for this struct was generated from the following file:

<ul>
<li><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a></li>
</ul>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
