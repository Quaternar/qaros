---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarappvolumegestureallowedpeersext
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarAppVolumeGestureAllowedPeersExt` Struct

<p>Extension: the peers whose gestures may move or scale the app. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarAppVolumeGestureAllowedPeersExt { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">size_t</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a80ed46010d241d7408f9c73d901c72b8">allowed_peer_count</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> *</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a1b1f5777cf44904aca3ac29bc7ccfda0">allowed_peers</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarstructureheader">QarStructureHeader</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a30178534b2b1038c9169871349ed701b">header</a></td>
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

<p>Extension: the peers whose gestures may move or scale the app.</p>


<p>Chain from QarAppVolumeGestureConfiguration::header.next. A configuration without it, or with no peers in it, lets every peer's gestures move and scale the app. A gesture from a peer not in the list still reaches the app, unmapped, so the app knows about it; only the app transform is kept from it. A configuration is replaced whole, so chain the extension every time the configuration is changed. Read the list back with <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga75360b0c91e05e88f2055956f68446bb">qar_app_volume_get_gesture_allowed_peers()</a>.</p>


<p>Whatever the list, only one peer at a time manipulates a volume: the first whose manipulation binds holds it until that manipulation ends. See <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gaaa82c086a96788ab7036cdf116ac3ac2">qar_app_volume_get_gesture_holder()</a>.</p>


<p>Definition at line 1227 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### allowed\_peer\_count {#a80ed46010d241d7408f9c73d901c72b8}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">size_t QarAppVolumeGestureAllowedPeersExt::allowed_peer_count</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01231">1231</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### allowed\_peers {#a1b1f5777cf44904aca3ac29bc7ccfda0}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">const QarPeerId* QarAppVolumeGestureAllowedPeersExt::allowed_peers</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01230">1230</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### header {#a30178534b2b1038c9169871349ed701b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarStructureHeader QarAppVolumeGestureAllowedPeersExt::header</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01229">1229</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
