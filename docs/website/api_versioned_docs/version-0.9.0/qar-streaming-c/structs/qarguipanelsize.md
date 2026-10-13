---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarguipanelsize
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarGuiPanelSize` Struct

<p>The three frames every GUI panel API is expressed in. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarGuiPanelSize { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">float</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ab010ed5f80f1d333bf08edf1fb15684e">content_scale</a></td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a1f80787522197037b60623db51f58f39">height_meters</a></td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a636241e50d8269f2da8cd06244fd1e45">width_meters</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Extent of the panel surface along panel-space X, in metres. <a href="#a636241e50d8269f2da8cd06244fd1e45">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}

<p>The three frames every GUI panel API is expressed in.</p>


<p><b>Room space</b> is the session's shared tracking space: right-handed, metres, +Y up. Every <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> in the GUI panel API - <a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">QarGuiPanelInit::pose</a>, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga88a8b5119ef11a2dc70d8b054c2ea95f">qar_gui_panels_update_pose()</a>, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga27f046c6022215093fa6305ee1aa95a1">qar_gui_panel_get_pose()</a> - is given in it.</p>


<p><b>Panel space</b> is the frame that pose defines. Its origin is the midpoint of the panel's top edge - the top <em>centre</em>, not a corner - and its axes are right-handed, described here as a viewer facing the panel front sees them:</p>


<ul class="doxyList ">
<li>+X runs along the top edge to the viewer's left,</li>
<li>+Y runs up,</li>
<li><b>-Z</b> is the front-face normal and points at the viewer. The front normal is -Z, not +Z: left x up points <em>away</em> from the viewer, so a +Z viewer-facing normal would not be right-handed. The renderer agrees - the panel surface sits at its bounds' minimum Z and the interaction volume grows along StereoKit's vec3_forward, (0, 0, -1). The panel surface therefore covers x in [-width_meters/2, +width_meters/2] and y in [-height_meters, 0], hanging below its own pose.</li>
</ul>

<p><b>Panel content space</b> is the 2D pixel raster drawn onto that surface: the browser bitmap of a QAR_GUI_PANEL_CONTENT_TYPE_WEBSITE_URI panel, or the decoded video image of a QAR_GUI_PANEL_CONTENT_TYPE_STREAM_ID one. Its origin is the panel's top-left corner as the viewer sees it, +X runs right and +Y runs down, and it is the frame <a href="/api/qar-streaming-c/structs/qarguipanelpoint">QarGuiPanelPoint</a> reports positions in. Panel content space runs backwards along both panel axes: content x = 0 sits at panel-space +width_meters/2 and content y = 0 at panel-space 0, the top edge, so content x advances along panel-space -X and content y along panel-space -Y. Panel content space is mapped onto panel space by <a href="#ab010ed5f80f1d333bf08edf1fb15684e">QarGuiPanelSize::content_scale</a>.</p>


<p>Definition at line 834 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### content\_scale {#ab010ed5f80f1d333bf08edf1fb15684e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarGuiPanelSize::content_scale</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Panel-content-space-per-panel-space zoom of the rendered raster. 1.0 fits the content across the full panel surface, 2.0 shows it at twice the size so only half of it fits, and 0.5 shows it at half size. It changes how the content is drawn and how a contact maps back to content pixels; it does not change width_meters or height_meters.</p>


<p>A QAR_GUI_PANEL_CONTENT_TYPE_WEBSITE_URI panel takes it as the page's zoom, as a desktop browser's Ctrl+ and Ctrl- do: the page is drawn that much larger and reflowed to the panel's width, so nothing is cut off at the sides. Only the application sets it; the panel's + and - buttons resize the panel instead (ADR-0165, ADR-0167).</p>


<p>A QAR_GUI_PANEL_CONTENT_TYPE_STREAM_ID panel ignores it: its video is always drawn at the full panel width and scrolled vertically by the renderer when it is taller than the panel (ADR-0150).</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00856">856</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### height\_meters {#a1f80787522197037b60623db51f58f39}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarGuiPanelSize::height_meters</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Extent of the panel surface along panel-space -Y, in metres. The surface hangs below the pose, which sits on its top edge.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00840">840</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### width\_meters {#a636241e50d8269f2da8cd06244fd1e45}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">float QarGuiPanelSize::width_meters</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Extent of the panel surface along panel-space X, in metres.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l00837">837</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
