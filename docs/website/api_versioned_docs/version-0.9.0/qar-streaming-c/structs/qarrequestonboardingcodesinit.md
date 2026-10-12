---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarrequestonboardingcodesinit
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarRequestOnboardingCodesInit` Struct

<p>Ask the hub for a new set of one-time onboarding codes. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarRequestOnboardingCodesInit { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">uint32_t</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a79ca692ced0606c1fd29dc6e918d668c">count</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>How many codes, 1 to 64. Also the length of the caller's out array. <a href="#a79ca692ced0606c1fd29dc6e918d668c">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarstructureheader">QarStructureHeader</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#aab3435d4dd42ec17e3c2026b3bddfc45">header</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">uint32_t</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a2823dd15b014c51577c7356ec139e5fa">lifetime_seconds</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Seconds each code stays valid; 0 -&gt; 120. At most 600. <a href="#a2823dd15b014c51577c7356ec139e5fa">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a793f64b3e2aa7e0ff6cc9f78ba3c25d1">planned_peer_id</a></td>
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

<p>Ask the hub for a new set of one-time onboarding codes.</p>


<p>Every request mints codes of its own: codes other callers hold and the code on the hub's onboarding screen stay valid, and none of the new ones is announced or displayed. Each code works once and lapses after its lifetime.</p>


<p>Definition at line 1524 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### count {#a79ca692ced0606c1fd29dc6e918d668c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">uint32_t QarRequestOnboardingCodesInit::count</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>How many codes, 1 to 64. Also the length of the caller's out array.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01529">1529</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### header {#aab3435d4dd42ec17e3c2026b3bddfc45}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarStructureHeader QarRequestOnboardingCodesInit::header</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>QAR_STRUCTURE_TYPE_SESSION_REQUEST_ONBOARDING_CODES_INIT</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01526">1526</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### lifetime\_seconds {#a2823dd15b014c51577c7356ec139e5fa}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">uint32_t QarRequestOnboardingCodesInit::lifetime_seconds</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Seconds each code stays valid; 0 -&gt; 120. At most 600.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01531">1531</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### planned\_peer\_id {#a793f64b3e2aa7e0ff6cc9f78ba3c25d1}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarPeerId QarRequestOnboardingCodesInit::planned_peer_id</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Zero -&gt; any device may use the codes. Otherwise a device pairing with one of them can only join under this peer id (<a href="/api/qar-streaming-c/structs/qaronboardpeeridext">QarOnboardPeerIdExt</a>).</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01534">1534</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
