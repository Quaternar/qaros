---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/structs/qarrendersenderinit
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - struct

---

<div class="doxyPage">

# `QarRenderSenderInit` Struct

<p>Rendering stream sender configuration. <a href="#details">More...</a></p>

## Declaration

<div class="doxyDeclaration">
struct QarRenderSenderInit { ... }
</div>

## Included Headers

<div class="doxyIncludesList">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar-streaming-c/include/qar_streaming.h</a>&gt;
</div>

## Public Member Attributes Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> *</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a9e643a49c3b4a8fa70593769f31b046e">app_volume_id</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a6359f08dfcf95d5248972f66a4978b9b">QarPixelFormat</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a8b79293b9343958e90e1c75953b68039">color_format</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Color pixel format. <a href="#a8b79293b9343958e90e1c75953b68039">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a6359f08dfcf95d5248972f66a4978b9b">QarPixelFormat</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a007a194ff6d030b9eed9c65de41300bb">depth_format</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Depth pixel format. <a href="#a007a194ff6d030b9eed9c65de41300bb">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#acacb647b1555ebdf5278e51a65caf333">enable_auto_reconnects</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarrenderframeview">QarRenderFrameView</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a563df772af437a104fca55d2c70d344f">frame_views</a>[QAR_MAX_FRAME_VIEWS]</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">size_t</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a14e322d8bb8fb44ecd0a66587eebb6d2">frame_views_count</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a51f21577060dbb5ba4f1ce0d1eaba53f">QarGraphicsAPI</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a5395c0cc66f54d9db58ece93f8405422">graphics_api</a></td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a978f5e057d79b81674fe93f71a826025">header</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Extensible header; see the backend-specific stream parameter structs. <a href="#a978f5e057d79b81674fe93f71a826025">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#aaea789b09c5fd7e1b629228641bbe8a3">peer_id</a></td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a7bc6cb027be2084342bb0a1b84bc92f8">QarFrameLayout</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a6ffbea39964d85e2bb20b14fd5f469a6">texture_layout</a></td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Frame layout configuration. <a href="#a6ffbea39964d85e2bb20b14fd5f469a6">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}

<p>Rendering stream sender configuration.</p>


<p>Backend-specific graphics context/device configuration is supplied through <span class="doxyComputerOutput">header.next</span>. Exactly one extension is required for a GPU API:</p>


<ul class="doxyList ">
<li>QAR_GRAPHICS_API_D3D11 requires <a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11">QarStreamParamsD3D11</a>.</li>
<li>QAR_GRAPHICS_API_OPENGL requires exactly one of QarStreamParamsOpenGlEgl, QarStreamParamsOpenGlWgl or QarStreamParamsOpenGlGlx, whichever matches the context you own.</li>
<li>QAR_GRAPHICS_API_VULKAN requires QarStreamParamsVulkan. QAR_GRAPHICS_API_CPU supports no extension and requires <span class="doxyComputerOutput">header.next</span> to be null. The selected extension's <span class="doxyComputerOutput">header.next</span> must also be null.</li>
</ul>

<p>Definition at line 1761 of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>


<div class="doxySectionDef">

## Public Member Attributes

### app\_volume\_id {#a9e643a49c3b4a8fa70593769f31b046e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">const QarAppVolumeId* QarRenderSenderInit::app_volume_id</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Required. The app volume this stream is displayed in, which is what places it in the session's shared 3D scene. Create the volume first with <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga76e9061c6e95e92787686a130604a140">qar_app_volumes_get_or_create()</a> and pass the id it wrote out; a stream without one is not positioned in the scene and no viewer can show it.</p>


<p>It also defines the sender's <b>app content space</b>: every pose, hand joint, radius and velocity the sender hands the application is in it. While the volume does not resolve, the sender hands out nothing: begin-frame fails with QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED and the hands are reported untracked.</p>


<p>NULL is rejected with QAR_STATUS_APP_VOLUME_INVALID_ID.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01809">1809</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### color\_format {#a8b79293b9343958e90e1c75953b68039}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarPixelFormat QarRenderSenderInit::color_format</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Color pixel format.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01812">1812</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### depth\_format {#a007a194ff6d030b9eed9c65de41300bb}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarPixelFormat QarRenderSenderInit::depth_format</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Depth pixel format.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01815">1815</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### enable\_auto\_reconnects {#acacb647b1555ebdf5278e51a65caf333}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool QarRenderSenderInit::enable_auto_reconnects</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>In case of stream failure caused by network issue or other side crash we will automatically restart stream and try to recreate a stream transparently in the background</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01777">1777</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### frame\_views {#a563df772af437a104fca55d2c70d344f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarRenderFrameView QarRenderSenderInit::frame_views[QAR_MAX_FRAME_VIEWS]</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Array of views in the single video frame.</p>


<p><b>The order is guaranteed.</b> The views of the layout (<a href="/api/qar-streaming-c/groups/qar-c-render-sender/#ga93f1561b953f2d82892dfc6c92a5b9ed">qar_render_sender_layout()</a>) and of every frame this sender hands back (qar_render_sender_frame_*()) come in exactly the order declared here, each with the <span class="doxyComputerOutput">eye</span> and <span class="doxyComputerOutput">data_type</span> declared at its position. Textures come in the order the views first use them: one per view for QAR_FRAME_LAYOUT_SEPARATED_TEXTURES, one per data type for side by side and layered, and each view's <span class="doxyComputerOutput">texture_index</span> names its texture. A view the frame layout provides but you did not declare follows the declared ones.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01793">1793</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### frame\_views\_count {#a14e322d8bb8fb44ecd0a66587eebb6d2}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">size_t QarRenderSenderInit::frame_views_count</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01794">1794</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### graphics\_api {#a5395c0cc66f54d9db58ece93f8405422}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarGraphicsAPI QarRenderSenderInit::graphics_api</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Selects the backend and therefore the matching optional header extension.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01819">1819</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### header {#a978f5e057d79b81674fe93f71a826025}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarStructureHeader QarRenderSenderInit::header</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Extensible header; see the backend-specific stream parameter structs.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01764">1764</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### peer\_id {#aaea789b09c5fd7e1b629228641bbe8a3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarPeerId QarRenderSenderInit::peer_id</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">




<p>Required. The peer this stream renders <em>for</em> - the headset or viewer that will receive these frames, never this process' own peer id. See "Basic flow" in api.h for how to find target peer ids.</p>


<p><a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaef75fee91478dda131b45d4307a13f8c">qar_peer_id_unique()</a> does <b>not</b> produce a usable value here: it mints a fresh id belonging to no peer, so a stream created with one is negotiated with nobody and never produces a frame.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01772">1772</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### texture\_layout {#a6ffbea39964d85e2bb20b14fd5f469a6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarFrameLayout QarRenderSenderInit::texture_layout</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Frame layout configuration.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l01780">1780</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

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
