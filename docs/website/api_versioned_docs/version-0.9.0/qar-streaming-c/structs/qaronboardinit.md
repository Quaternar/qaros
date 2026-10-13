---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qaronboardinit
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarOnboardInit` Struct

<p>Base init for full onboarding. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarOnboardInit { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarstructureheader">QarStructureHeader</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a8e50f1404dcbac0e16dc6b0f0e447714">header</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a70cfe279c5e45670c33956ad9cca3792">onboarding_id</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Zero -&gt; new identity; previously returned id -&gt; re-enroll that slot. <a href="#a70cfe279c5e45670c33956ad9cca3792">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarpeerpresentation">QarPeerPresentation</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ac055d692343c02999988c5e33f0257b7">presentation</a></td>
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

<p>Base init for full onboarding.</p>


<p>The actual onboarding mode is selected by chaining exactly one of the mode extensions through header.next:</p>


<ul class="doxyList ">
<li><a href="/api/qar-streaming-c/structs/qaronboardcodeext">QarOnboardCodeExt</a> for code onboarding</li>
<li><a href="/api/qar-streaming-c/structs/qaronboardinviteext">QarOnboardInviteExt</a> for a full invite blob</li>
</ul>

<p>Definition at line 1422 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### header {#a8e50f1404dcbac0e16dc6b0f0e447714}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarStructureHeader QarOnboardInit::header</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_INIT</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01424">1424</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### onboarding\_id {#a70cfe279c5e45670c33956ad9cca3792}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarOnboardingId QarOnboardInit::onboarding_id</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Zero -&gt; new identity; previously returned id -&gt; re-enroll that slot.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01426">1426</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### presentation {#ac055d692343c02999988c5e33f0257b7}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarPeerPresentation QarOnboardInit::presentation</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>display_name also feeds the sanitized debug label in cert CN and the storage-slot directory.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01429">1429</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
