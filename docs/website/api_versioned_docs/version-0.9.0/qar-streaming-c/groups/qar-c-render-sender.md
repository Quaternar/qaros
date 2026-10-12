---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-render-sender
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Render Sender



## Typedefs Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#gad39095a1faef36dcb272ffa8f3a8a641">qar_render_sender_begin_frame_callback_t</a>)(QarResult status, QarRenderFrameInfo *frame_info, void *user_state)</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#ga0fe3becd5642e9732910eb9da2cb6699">qar_render_sender_create_callback_t</a>)(QarResult status, QarRenderSender *out_stream, void *user_state)</td>
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

## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga0173f19b6a3d9b9512465b899da18c9f">qar_render_frame_info_get_view_fov</a> (QarRenderFrameInfo *handle, size_t view_index, QarFov *out_fov)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The field of view of one view, in view space, in radians. A view of eye QAR_VIDEO_FRAME_VIEW_EYE_NONE gets the left eye's. <a href="#ga0173f19b6a3d9b9512465b899da18c9f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga018fad570406f0e5fbffdca125e1ef61">qar_render_frame_info_get_view_pose</a> (QarRenderFrameInfo *handle, size_t view_index, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The pose to render one view with, in the bound app volume's app content space. <a href="#ga018fad570406f0e5fbffdca125e1ef61">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga167970742e5e5b9415f7073b71a74db4">qar_render_frame_info_handle_destroy</a> (QarRenderFrameInfo *handle)</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6bac8c8cb304530f04c4f9a7931f7639">qar_render_frame_info_handle_is_valid</a> (QarRenderFrameInfo *handle)</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gafb6d1202ad0dff37057e498e338cc5e6">qar_render_request_get_stream_id</a> (QarRenderStreamRequest *request, QarStreamId *out_stream_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the stream id from a render request. <a href="#gafb6d1202ad0dff37057e498e338cc5e6">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga96b983e289a2f2eb5545cd3335728c19">qar_render_request_get_target_peer_id</a> (QarRenderStreamRequest *request, QarPeerId *out_peer_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the target peer id from a render request. <a href="#ga96b983e289a2f2eb5545cd3335728c19">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gad34c95971273d1e646f8d8dd484973b1">qar_render_request_handle_destroy</a> (QarRenderStreamRequest *request)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a render stream request handle. <a href="#gad34c95971273d1e646f8d8dd484973b1">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gad94b5f0ba0fb38b6dc25b772e654a99c">qar_render_request_handle_is_valid</a> (QarRenderStreamRequest *request)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if a render stream request handle is valid. <a href="#gad94b5f0ba0fb38b6dc25b772e654a99c">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga86c067e2501014a8ebd9d61c9b2965a3">qar_render_sender_begin_frame</a> (QarRenderSender *stream, QarCancelToken *token, QarRenderFrameInfo **out_frame_info)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Begin producing a new frame. <a href="#ga86c067e2501014a8ebd9d61c9b2965a3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3c2fbd8437dc231709cb692e3862f6cf">qar_render_sender_begin_frame_async</a> (QarRenderSender *stream, qar_render_sender_begin_frame_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Async version of begin_frame. Same poses, same space, same failures and the same wait through a paused stream as <a href="#ga86c067e2501014a8ebd9d61c9b2965a3">qar_render_sender_begin_frame()</a>. <a href="#ga3c2fbd8437dc231709cb692e3862f6cf">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gafb540059b078e8e5b3034811d6d445fd">qar_render_sender_change_layout</a> (QarRenderSender *stream, const QarVideoFrameLayout *layout)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Request a layout change for subsequent frames. <a href="#gafb540059b078e8e5b3034811d6d445fd">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7516be21e690ce67b9b10ef8dd897dde">qar_render_sender_create</a> (QarSession *session, QarRenderSenderInit *init, QarCancelToken *cancel, QarRenderSender **out_stream)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Create a rendering stream sender bound to a session. <a href="#ga7516be21e690ce67b9b10ef8dd897dde">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga0b6267c098c68c87f47d2900c3ca0280">qar_render_sender_create_async</a> (QarSession *session, QarRenderSenderInit *init, qar_render_sender_create_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Async variant of qar_render_sender_create. <a href="#ga0b6267c098c68c87f47d2900c3ca0280">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga188d84f3109392a9d70f8a75d604e8dd">qar_render_sender_frame_cpu</a> (QarRenderSender *stream, QarVideoFrameCpu *out_frame)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get CPU-backed frame access. <a href="#ga188d84f3109392a9d70f8a75d604e8dd">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae189a021de117ab646bd9f73fbe3b2a9">qar_render_sender_frame_d3d11</a> (QarRenderSender *stream, QarVideoFrameD3D11 *out_frame)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get D3D11-backed frame access. Readable before the first begin-frame; writable only inside a frame cycle. <a href="#gae189a021de117ab646bd9f73fbe3b2a9">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaea1e896f67ec6ff23df57d76eaf0def6">qar_render_sender_last_hands</a> (QarRenderSender *stream, QarDeviceHandsWithJoints *out_hands)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Query the newest hands data the device sent since the previous call. <a href="#gaea1e896f67ec6ff23df57d76eaf0def6">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga93f1561b953f2d82892dfc6c92a5b9ed">qar_render_sender_layout</a> (QarRenderSender *stream, QarVideoFrameLayout *out_layout)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Retrieve the negotiated video frame layout. <a href="#ga93f1561b953f2d82892dfc6c92a5b9ed">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga68a7b1e14c6d7f1d9198fdfc222cb0ef">qar_render_sender_show_frame</a> (QarRenderSender *stream, const QarRenderFrameShow *frame_show)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Submit the rendered frame for presentation/streaming. <a href="#ga68a7b1e14c6d7f1d9198fdfc222cb0ef">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga513dbf8f726f67bdf24eb0879b12adeb">qar_render_sender_subscribe_requests</a> (QarSession *session, qar_render_sender_request_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to render stream requests for this application's content. <a href="#ga513dbf8f726f67bdf24eb0879b12adeb">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga041ffabfe8961d8bda55f489b48ced97">qar_render_sender_try_begin_frame</a> (QarRenderSender *stream, QarRenderFrameInfo **out_frame_info, bool *out_ready)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Non-blocking attempt to begin producing a new frame. <a href="#ga041ffabfe8961d8bda55f489b48ced97">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaa483cc940ae56a4d66545020f3957872">qar_render_stream_handle_destroy</a> (QarRenderSender *handle)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Destroy a render stream sender handle. <a href="#gaa483cc940ae56a4d66545020f3957872">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Typedefs

### qar\_render\_sender\_begin\_frame\_callback\_t {#gad39095a1faef36dcb272ffa8f3a8a641}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_render_sender_begin_frame_callback_t)(QarResult status, QarRenderFrameInfo *frame_info, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03094">3094</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_create\_callback\_t {#ga0fe3becd5642e9732910eb9da2cb6699}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_render_sender_create_callback_t)(QarResult status, QarRenderSender *out_stream, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02969">2969</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<div class="doxySectionDef">

## Functions

### qar\_render\_frame\_info\_get\_view\_fov() {#ga0173f19b6a3d9b9512465b899da18c9f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_frame_info_get_view_fov (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> * handle, size_t view_index, <a href="/api/qar-streaming-c/structs/qarfov">QarFov</a> * out_fov)</td>
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

<p>The field of view of one view, in view space, in radians. A view of eye QAR_VIDEO_FRAME_VIEW_EYE_NONE gets the left eye's.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03158">3158</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_frame\_info\_get\_view\_pose() {#ga018fad570406f0e5fbffdca125e1ef61}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_frame_info_get_view_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> * handle, size_t view_index, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
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

<p>The pose to render one view with, in the bound app volume's app content space.</p>


<p>The sender has already taken the viewer's room-space pose through the volume's pose, app pose and app scale, so the application renders its scene in its own coordinates. A view of eye QAR_VIDEO_FRAME_VIEW_EYE_NONE gets the head pose.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03151">3151</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_frame\_info\_handle\_destroy() {#ga167970742e5e5b9415f7073b71a74db4}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_render_frame_info_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> * handle)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03141">3141</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_frame\_info\_handle\_is\_valid() {#ga6bac8c8cb304530f04c4f9a7931f7639}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_render_frame_info_handle_is_valid (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> * handle)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l06745">6745</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_request\_get\_stream\_id() {#gafb6d1202ad0dff37057e498e338cc5e6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_request_get_stream_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9d90605b9f2518e846c34579febd51af">QarRenderStreamRequest</a> * request, <a href="/api/qar-streaming-c/structs/qarstreamid">QarStreamId</a> * out_stream_id)</td>
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

<p>Get the stream id from a render request.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02940">2940</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_request\_get\_target\_peer\_id() {#ga96b983e289a2f2eb5545cd3335728c19}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_request_get_target_peer_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9d90605b9f2518e846c34579febd51af">QarRenderStreamRequest</a> * request, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peer_id)</td>
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

<p>Get the target peer id from a render request.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02936">2936</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_request\_handle\_destroy() {#gad34c95971273d1e646f8d8dd484973b1}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_render_request_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9d90605b9f2518e846c34579febd51af">QarRenderStreamRequest</a> * request)</td>
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

<p>Destroy a render stream request handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02934">2934</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_request\_handle\_is\_valid() {#gad94b5f0ba0fb38b6dc25b772e654a99c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_render_request_handle_is_valid (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9d90605b9f2518e846c34579febd51af">QarRenderStreamRequest</a> * request)</td>
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

<p>Check if a render stream request handle is valid.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l06751">6751</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_begin\_frame() {#ga86c067e2501014a8ebd9d61c9b2965a3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_begin_frame (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> ** out_frame_info)</td>
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

<p>Begin producing a new frame.</p>


<p>Returns per-frame information such as per-eye pose/FOV to render with. The poses are in the bound app volume's app content space (ADR-0253).</p>


<p>Fails with QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED, without beginning a frame, while the app volume does not resolve: app content space is undefined then, and no room-space pose is handed out in its place. The failure is recoverable - keep calling, a frame begins once the volume resolves. The call is held about 16 ms first, so a retrying loop does not spin.</p>


<p>Waits, rather than fails, while the receiver has paused the stream (for example, its display is not visible): the sender and its stream stay up and the frame begins as soon as the receiver resumes. Cancel <span class="doxyComputerOutput">token</span> to stop waiting; the call then fails.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03068">3068</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_begin\_frame\_async() {#ga3c2fbd8437dc231709cb692e3862f6cf}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_begin_frame_async (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="#gad39095a1faef36dcb272ffa8f3a8a641">qar_render_sender_begin_frame_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Async version of begin_frame. Same poses, same space, same failures and the same wait through a paused stream as <a href="#ga86c067e2501014a8ebd9d61c9b2965a3">qar_render_sender_begin_frame()</a>.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03100">3100</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_change\_layout() {#gafb540059b078e8e5b3034811d6d445fd}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_change_layout (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, const <a href="/api/qar-streaming-c/structs/qarvideoframelayout">QarVideoFrameLayout</a> * layout)</td>
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

<p>Request a layout change for subsequent frames.</p>


<p>Not callable inside a frame cycle: between a successful begin-frame and its <a href="#ga68a7b1e14c6d7f1d9198fdfc222cb0ef">qar_render_sender_show_frame()</a> it returns QAR_STATUS_LOGIC_ERROR and changes nothing. The views and textures of later frames keep the order given in <span class="doxyComputerOutput">layout</span>.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03003">3003</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_create() {#ga7516be21e690ce67b9b10ef8dd897dde}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_create (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/structs/qarrendersenderinit">QarRenderSenderInit</a> * init, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * cancel, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> ** out_stream)</td>
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

<p>Create a rendering stream sender bound to a session.</p>


<p>**<span class="doxyComputerOutput">init-&gt;peer_id</span> is the target this stream renders for**, not this process' own id; see "Basic flow" at the top of this header for where it comes from. The sender attaches that target to <span class="doxyComputerOutput">init-&gt;app_volume_id</span>. A fresh <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaef75fee91478dda131b45d4307a13f8c">qar_peer_id_unique()</a> belongs to no peer: a stream created with one negotiates with nobody and never produces a frame.</p>


<p>**<span class="doxyComputerOutput">init-&gt;app_volume_id</span> must name an app volume that already exists.** The volume is what places the stream in the shared 3D scene, so create it first with <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga76e9061c6e95e92787686a130604a140">qar_app_volumes_get_or_create()</a> and pass the id it returns.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session handle.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">init</td>
<td class="doxyParamItemDescription"><p>Stream configuration (views, formats, target peer, app volume).</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">cancel</td>
<td class="doxyParamItemDescription"><p>Optional cancellation token for creation.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_stream</td>
<td class="doxyParamItemDescription"><p>Receives the created stream sender handle.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> QAR_STATUS_APP_VOLUME_INVALID_ID when no app volume id is given.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02963">2963</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_create\_async() {#ga0b6267c098c68c87f47d2900c3ca0280}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_create_async (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/structs/qarrendersenderinit">QarRenderSenderInit</a> * init, <a href="#ga0fe3becd5642e9732910eb9da2cb6699">qar_render_sender_create_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Async variant of qar_render_sender_create.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02973">2973</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_frame\_cpu() {#ga188d84f3109392a9d70f8a75d604e8dd}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_frame_cpu (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/structs/qarvideoframecpu">QarVideoFrameCpu</a> * out_frame)</td>
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

<p>Get CPU-backed frame access.</p>


<p>The frame exists from creation, so it may be read – textures, views, sizes – before the first begin-frame. Its contents and synchronization may be written only inside a frame cycle.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03013">3013</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_frame\_d3d11() {#gae189a021de117ab646bd9f73fbe3b2a9}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_frame_d3d11 (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/structs/qarvideoframed3d11">QarVideoFrameD3D11</a> * out_frame)</td>
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

<p>Get D3D11-backed frame access. Readable before the first begin-frame; writable only inside a frame cycle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03021">3021</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_last\_hands() {#gaea1e896f67ec6ff23df57d76eaf0def6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_last_hands (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/structs/qardevicehandswithjoints">QarDeviceHandsWithJoints</a> * out_hands)</td>
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

<p>Query the newest hands data the device sent since the previous call.</p>


<p>Drains everything that arrived and returns the last sample. The poses are in the bound app volume's app content space, like the view poses, and so are the joint radii and velocities measured beside them: radii in app units, velocities in app axes, linear ones in app units per second (ADR-0253). Never room space - hand data a peer publishes on a data stream is in that peer's room space, but this call converts it. Samples arrive only while the target peer is focused on the volume and not editing it, and while the volume resolves; otherwise both hands are untracked. When nothing arrived since the previous call, the last sample is returned again while it is younger than a second, so polling faster than the device sends never blinks the hands out between its samples. Once no sample has arrived for a second, both hands are reported untracked: a device that stopped sending has no hands. Poll it once per frame.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">stream</td>
<td class="doxyParamItemDescription"><p>The render sender.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_hands</td>
<td class="doxyParamItemDescription"><p>Receives the newest sample, the last one while it is younger than a second, or a sample with both hands untracked.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>Success, or an error once the hand stream has closed.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03134">3134</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_layout() {#ga93f1561b953f2d82892dfc6c92a5b9ed}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_layout (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/structs/qarvideoframelayout">QarVideoFrameLayout</a> * out_layout)</td>
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

<p>Retrieve the negotiated video frame layout.</p>


<p>Available as soon as the sender exists: it is what the stream request settled on, so it can be read before the first begin-frame and while a transfer is reconnecting. Size lasting resources from this rather than from a frame that has not arrived yet.</p>


<p>Its views come in exactly the order declared in <a href="/api/qar-streaming-c/structs/qarrendersenderinit/#a563df772af437a104fca55d2c70d344f">QarRenderSenderInit::frame_views</a>, and every frame's views and textures match it; after <a href="#gafb540059b078e8e5b3034811d6d445fd">qar_render_sender_change_layout()</a>, in the order of that layout.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02992">2992</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_show\_frame() {#ga68a7b1e14c6d7f1d9198fdfc222cb0ef}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_show_frame (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, const <a href="/api/qar-streaming-c/structs/qarrenderframeshow">QarRenderFrameShow</a> * frame_show)</td>
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

<p>Submit the rendered frame for presentation/streaming.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03109">3109</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_subscribe\_requests() {#ga513dbf8f726f67bdf24eb0879b12adeb}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_subscribe_requests (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a36f54e9799d7a3846d466730cf096828">qar_render_sender_request_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Subscribe to render stream requests for this application's content.</p>


<p>A target requests the content only once it is attached to the application's app volume, so this does not discover targets for a new application - see "Basic flow" at the top of this header for that. It sees requests for a volume something else attached a target to.</p>


<p>In the callback, read <a href="#ga96b983e289a2f2eb5545cd3335728c19">qar_render_request_get_target_peer_id()</a>, then enqueue that id for the application's render thread, which creates one sender (and a camera using its per-frame pose/FOV) per target. Repeated requests for a target already served do not require another sender.</p>


<p>Callbacks run asynchronously, so synchronize handoff to the render thread. Release each delivered request with <a href="#gad34c95971273d1e646f8d8dd484973b1">qar_render_request_handle_destroy()</a>. Keep user_state alive until the call ending the subscription has returned, see <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Active session.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">callback</td>
<td class="doxyParamItemDescription"><p>Required callback for each pending request.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>Application state passed unchanged to the callback.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Required cancellation token owning the subscription; cancelling or destroying it, or destroying the session, ends it.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>Success when subscribed, or a subscription setup error.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02923">2923</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_sender\_try\_begin\_frame() {#ga041ffabfe8961d8bda55f489b48ced97}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_render_sender_try_begin_frame (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * stream, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a> ** out_frame_info, bool * out_ready)</td>
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

<p>Non-blocking attempt to begin producing a new frame.</p>


<p>Starts the begin-frame wait on the first call and polls it on every call without blocking. When the wait has completed, <span class="doxyComputerOutput">out_ready</span> is set to true and <span class="doxyComputerOutput">out_frame_info</span> receives the per-frame information; otherwise <span class="doxyComputerOutput">out_ready</span> is set to false and the caller should poll again later. The returned <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> reports only real failures (invalid arguments, a failed begin-frame); a still-pending wait is reported through <span class="doxyComputerOutput">out_ready</span>, not as an error. Poll with the same <span class="doxyComputerOutput">stream</span> until <span class="doxyComputerOutput">out_ready</span> becomes true. Fails like <a href="#ga86c067e2501014a8ebd9d61c9b2965a3">qar_render_sender_begin_frame()</a>, including QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED while the app volume does not resolve. While the receiver has paused the stream the wait does not complete: <span class="doxyComputerOutput">out_ready</span> stays false and the call succeeds until it resumes.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03089">3089</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_render\_stream\_handle\_destroy() {#gaa483cc940ae56a4d66545020f3957872}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_render_stream_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a> * handle)</td>
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

<p>Destroy a render stream sender handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02898">2898</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
