---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarappvolumesize
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarAppVolumeSize` Struct

<p>The three frames every app volume API is expressed in. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarAppVolumeSize { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">float</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a4588bfc36f49bf06870b9ccc4e2532be">height_meters</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Cuboid extent along app-volume-space Y - the vertical rise - in metres. <a href="#a4588bfc36f49bf06870b9ccc4e2532be">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">float</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a9a1934c1fc17783ffff15717e8821564">length_meters</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">float</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#aae5fa8eb081d2d741815132b89eb5837">width_meters</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Cuboid extent along app-volume-space X, in metres. <a href="#aae5fa8eb081d2d741815132b89eb5837">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}

<p>The three frames every app volume API is expressed in.</p>


<p>An app volume is a cuboid placed in the room that reserves the region a remote 3D application renders into.</p>


<p><b>Room space</b> is the session's shared tracking space: right-handed, metres, +Y up. <a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">QarAppVolumeInit::pose</a>, <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga0f57f76fe79b59f34ed4c4d2d2e2875d">qar_app_volumes_change_pose()</a> and <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga0b2414ac9f19997acac1e9ba039b6afc">qar_app_volume_get_pose()</a> are given in it.</p>


<p><b>App volume space</b> is the frame that pose defines. Its origin is the <em>centre of the cuboid</em> - not a corner, and not a point on the floor - and its axes are right-handed, laid out so that an unrotated volume stands upright in the room:</p>


<ul class="doxyList ">
<li>+X spans width_meters,</li>
<li>+Y spans height_meters and points up,</li>
<li>+Z spans length_meters, the horizontal depth. The cuboid therefore covers x in [-width_meters/2, +width_meters/2], y in [-height_meters/2, +height_meters/2] and z in [-length_meters/2, +length_meters/2].</li>
</ul>

<p><b>App content space</b> is the coordinate system of the application rendering into the volume. <a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a53c0d7e8913ea5794ae8a7e149a15706">QarAppVolumeInit::app_pose</a> and <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gab61a244bdab51ef38d6920aa9674f8b3">qar_app_volumes_change_app_pose()</a> place that application's origin inside app volume space, in volume metres; <a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a2cddc63926ca9a508a916e1519b9525c">QarAppVolumeInit::app_scale</a> converts app metres into volume metres. Nothing clips the application to the cuboid - the volume states where the app is meant to be, and gestures and layout are derived from it.</p>


<p>Definition at line 985 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### height\_meters {#a4588bfc36f49bf06870b9ccc4e2532be}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarAppVolumeSize::height_meters</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Cuboid extent along app-volume-space Y - the vertical rise - in metres.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00993">993</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### length\_meters {#a9a1934c1fc17783ffff15717e8821564}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarAppVolumeSize::length_meters</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Cuboid extent along app-volume-space Z - the horizontal depth - in metres.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00991">991</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### width\_meters {#aae5fa8eb081d2d741815132b89eb5837}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarAppVolumeSize::width_meters</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Cuboid extent along app-volume-space X, in metres.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00988">988</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
