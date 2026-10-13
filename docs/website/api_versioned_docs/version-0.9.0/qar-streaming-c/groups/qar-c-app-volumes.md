---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-app-volumes
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# App Volumes



## Typedefs Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#ga48b2d003e51adfe466ac386ff56e8520">qar_app_volume_gesture_event_callback_t</a>)(QarAppVolumeGesture *gesture, void *user_state)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Receives one mapped gesture on a subscribed app volume. <a href="#ga48b2d003e51adfe466ac386ff56e8520">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">typedef</td>
<td class="doxyMemberIndexItemName" align="left" valign="top">void(* <a href="#ga7847366f9e5f7a4b2ef741c0e88d842f">qar_app_volume_update_callback_t</a>)(QarAppVolume *handle, void *user_state)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7989f43826213560076d3e89365def90">qar_app_volume_gesture_get_classification</a> (QarAppVolumeGesture *gesture, QarGestureKind *out_kind, QarGesturePhase *out_phase)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>What this gesture is and where in its lifecycle it sits. <a href="#ga7989f43826213560076d3e89365def90">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga686acec23deaa823d19cb150208daf89">qar_app_volume_gesture_get_deltas</a> (QarAppVolumeGesture *gesture, QarVector3 *out_translation_delta, QarQuaternion *out_rotation_delta, bool *out_was_mapped_to_app_transform)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The rigid motion accumulated since the gesture started, and whether the library already applied it. <a href="#ga686acec23deaa823d19cb150208daf89">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gac5716d790a0a4f31bb120d10448532d3">qar_app_volume_gesture_get_gesture_id</a> (QarAppVolumeGesture *gesture, uint64_t *out_gesture_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The gesture's lifecycle id. <a href="#gac5716d790a0a4f31bb120d10448532d3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gac71bf487f19e0675f487ba6ecdae7ddf">qar_app_volume_gesture_get_handedness</a> (QarAppVolumeGesture *gesture, QarHandedness *out_handedness)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Which hand the gesture came from. <a href="#gac71bf487f19e0675f487ba6ecdae7ddf">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaecb4a7a69567906fcc5d8a078dcf3819">qar_app_volume_gesture_get_lifecycle</a> (QarAppVolumeGesture *gesture, QarPeerId *out_source_peer_id, QarAppVolumeId *out_target_app_volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The lifecycle this gesture belongs to: the peer and the target volume. <a href="#gaecb4a7a69567906fcc5d8a078dcf3819">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga912768a3013ba55c6816dae5dabc1de3">qar_app_volume_gesture_get_mode</a> (QarAppVolumeGesture *gesture, QarAppVolumeGestureModeId *out_mode_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The gesture mode whose rule mapped the gesture. <a href="#ga912768a3013ba55c6816dae5dabc1de3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6a0103b0bcb19c7ab12302f9d8b83882">qar_app_volume_gesture_get_points</a> (QarAppVolumeGesture *gesture, QarVector3 *out_action_point, bool *out_has_action_point, QarVector3 *out_start_point, bool *out_has_start_point)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Where the gesture is acting now, and where its lifecycle began. <a href="#ga6a0103b0bcb19c7ab12302f9d8b83882">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3d0c820d6bd1690c16dbaf4dcdaa6367">qar_app_volume_gesture_get_timestamp</a> (QarAppVolumeGesture *gesture, QarTimePoint *out_timestamp)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>When the recogniser emitted this gesture, on its own clock. <a href="#ga3d0c820d6bd1690c16dbaf4dcdaa6367">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga58d2b802d6e38245adadb3a077875894">qar_app_volume_get_active_gesture_mode</a> (QarAppVolume *handle, const QarPeerId *peer, QarAppVolumeGestureModeId *out_mode_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The gesture mode a peer's next gesture maps through. <a href="#ga58d2b802d6e38245adadb3a077875894">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3fc609ca21893d773e27d9fa2e4e0e34">qar_app_volume_get_app_pose</a> (QarAppVolume *handle, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get app pose for an app volume handle. <a href="#ga3fc609ca21893d773e27d9fa2e4e0e34">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gab1076b6070e5c652d8659e9adfd98db0">qar_app_volume_get_app_scale</a> (QarAppVolume *handle, float *out_scale)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get app scale for an app volume handle. <a href="#gab1076b6070e5c652d8659e9adfd98db0">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf3e5a0be14fa7200ca037fef9ee4c66e">qar_app_volume_get_app_world_anchor</a> (QarAppVolume *handle, QarAppWorldAnchor *out_anchor)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get app world anchor for an app volume handle. <a href="#gaf3e5a0be14fa7200ca037fef9ee4c66e">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf8c0c2d59ba67ee5edcea8c8556851c6">qar_app_volume_get_display_name</a> (QarAppVolume *handle, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the display name string for an app volume. <a href="#gaf8c0c2d59ba67ee5edcea8c8556851c6">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2fc57555f013f36d64071c4f7444a415">qar_app_volume_get_editing_status</a> (QarAppVolume *handle, QarAppVolumeEditingStatus *out_status)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get current editing lock status and editor peer. <a href="#ga2fc57555f013f36d64071c4f7444a415">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga75360b0c91e05e88f2055956f68446bb">qar_app_volume_get_gesture_allowed_peers</a> (QarAppVolume *handle, QarPeerId *out_peers, size_t peer_capacity, size_t *out_peer_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The peers whose gestures may move or scale the app in this volume. <a href="#ga75360b0c91e05e88f2055956f68446bb">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga5bd2461175d72e5852a695eed1fab921">qar_app_volume_get_gesture_configuration</a> (QarAppVolume *handle, QarAppVolumeGestureConfiguration *out_config)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the default gesture mode's configuration for an app volume handle. <a href="#ga5bd2461175d72e5852a695eed1fab921">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaaa82c086a96788ab7036cdf116ac3ac2">qar_app_volume_get_gesture_holder</a> (QarAppVolume *handle, QarPeerId *out_peer, bool *out_is_held)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Which peer is manipulating the app in this volume, if one is. <a href="#gaaa82c086a96788ab7036cdf116ac3ac2">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7a3c643fb7520ca53166f0713fd676b4">qar_app_volume_get_gesture_mode</a> (QarAppVolume *handle, QarAppVolumeGestureModeId mode_id, QarAppVolumeGestureConfiguration *out_config)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>One gesture mode's rules. <a href="#ga7a3c643fb7520ca53166f0713fd676b4">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7d24689fac07f3c51f7d71daaabb9c7b">qar_app_volume_get_gesture_mode_ids</a> (QarAppVolume *handle, QarAppVolumeGestureModeId *out_mode_ids, size_t mode_capacity, size_t *out_mode_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>The ids of the gesture modes an app volume defines. <a href="#ga7d24689fac07f3c51f7d71daaabb9c7b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf404764ed9c2c6186ee524be5b10cf0d">qar_app_volume_get_id</a> (QarAppVolume *handle, QarAppVolumeId *out_id)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2ef27f79813e6b84c2447a280e8631b9">qar_app_volume_get_latest_app_pose</a> (QarSession *session, const QarAppVolumeId *volume_id, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the latest locally known fast-path app pose for this app volume. <a href="#ga2ef27f79813e6b84c2447a280e8631b9">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga3f442b19c3a356ef0cfdcb193afbaf3f">qar_app_volume_get_latest_app_scale</a> (QarSession *session, const QarAppVolumeId *volume_id, float *out_scale)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the latest locally known fast-path app scale for this app volume. <a href="#ga3f442b19c3a356ef0cfdcb193afbaf3f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga4007afa654390fadd509d0ea6a19a704">qar_app_volume_get_latest_pose</a> (QarSession *session, const QarAppVolumeId *volume_id, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the latest locally known fast-path pose for this app volume. <a href="#ga4007afa654390fadd509d0ea6a19a704">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga28ed3ba4c12104298235fb4d0a614ba7">qar_app_volume_get_latest_size</a> (QarSession *session, const QarAppVolumeId *volume_id, QarAppVolumeSize *out_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the latest locally known fast-path size for this app volume. <a href="#ga28ed3ba4c12104298235fb4d0a614ba7">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaa57bb480cd560ecfb95d190b866cec34">qar_app_volume_get_lifetime_status</a> (QarAppVolume *handle, QarAppVolumeLifetimeStatus *out_status)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the lifetime status (active/closed). <a href="#gaa57bb480cd560ecfb95d190b866cec34">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga0b2414ac9f19997acac1e9ba039b6afc">qar_app_volume_get_pose</a> (QarAppVolume *handle, QarPose *out_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the pose of an app volume. <a href="#ga0b2414ac9f19997acac1e9ba039b6afc">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga96545524dc43b5038573f31a963694df">qar_app_volume_get_size</a> (QarAppVolume *handle, QarAppVolumeSize *out_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the current size of an app volume. <a href="#ga96545524dc43b5038573f31a963694df">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaea671b39b22c73687ca865d66b073c2d">qar_app_volume_get_used_by_peers</a> (QarAppVolume *handle, QarPeerId *out_peers, size_t peers_buffer_size, size_t *out_peers_written)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Enumerate peers using the volume into provided buffer. <a href="#gaea671b39b22c73687ca865d66b073c2d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf1a8148682c1f0a52a4f454a5de37324">qar_app_volume_get_used_by_peers_count</a> (QarAppVolume *handle, size_t *out_peer_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get number of peers currently using this volume. <a href="#gaf1a8148682c1f0a52a4f454a5de37324">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaf7c6b09bc22d0cc36286a4c96f08fdff">qar_app_volume_handle_destroy</a> (QarAppVolume *handle)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga70c58f6db3167f6d4de03c682d5e4d8c">qar_app_volume_handle_is_valid</a> (QarAppVolume *handle)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gab61a244bdab51ef38d6920aa9674f8b3">qar_app_volumes_change_app_pose</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarPose *app_pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update app pose for an existing app volume. <a href="#gab61a244bdab51ef38d6920aa9674f8b3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7585bf066d0d3a26353e92b696987fec">qar_app_volumes_change_app_scale</a> (QarSession *session, const QarAppVolumeId *volume_id, float scale)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Low-latency fast-path update for latest app scale. <a href="#ga7585bf066d0d3a26353e92b696987fec">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2831f7924d7aaf86b6c77279c4a3f748">qar_app_volumes_change_app_world_anchor</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarGeoAnchorFrame *anchor)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Set app world anchor metadata. <a href="#ga2831f7924d7aaf86b6c77279c4a3f748">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga65913c52d6f65b97c63f28ef0e3c1967">qar_app_volumes_change_display_name</a> (QarSession *session, const QarAppVolumeId *volume_id, const char *display_name)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga4b7168291f07557695fc83803face6f9">qar_app_volumes_change_gesture_configuration</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarAppVolumeGestureConfiguration *config)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update the default gesture mode and the allowed peers of an existing app volume. <a href="#ga4b7168291f07557695fc83803face6f9">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gab701d8ba8fe7b55d8ed753f7bb788c9c">qar_app_volumes_change_gesture_mode</a> (QarSession *session, const QarAppVolumeId *volume_id, QarAppVolumeGestureModeId mode_id, const QarAppVolumeGestureConfiguration *config)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Define a gesture mode, or replace the mode with that id. <a href="#gab701d8ba8fe7b55d8ed753f7bb788c9c">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga0f57f76fe79b59f34ed4c4d2d2e2875d">qar_app_volumes_change_pose</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarPose *pose)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Low-latency fast-path update for latest pose. <a href="#ga0f57f76fe79b59f34ed4c4d2d2e2875d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga13c4244a8b083c92d0b137a0e585c42f">qar_app_volumes_change_size</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarAppVolumeSize *size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Resize an app volume. <a href="#ga13c4244a8b083c92d0b137a0e585c42f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae6c1882410f4181aa0b39ebdcda3a56f">qar_app_volumes_clear_all_peer_gesture_modes</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Drop every peer's own gesture mode, so all follow the default. <a href="#gae6c1882410f4181aa0b39ebdcda3a56f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaec59bb278f3aeb12dfe4d0b6061bfe57">qar_app_volumes_clear_app_world_anchor</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Clear app world anchor metadata. <a href="#gaec59bb278f3aeb12dfe4d0b6061bfe57">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae799744493713eff4b5aa8e4c15e8c15">qar_app_volumes_clear_peer_gesture_mode</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarPeerId *peer)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Drop one peer's own gesture mode, so it follows the default again. <a href="#gae799744493713eff4b5aa8e4c15e8c15">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga93cae823d562ebf2ede131be358721ad">qar_app_volumes_close_volume</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Close an app volume. <a href="#ga93cae823d562ebf2ede131be358721ad">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga76e9061c6e95e92787686a130604a140">qar_app_volumes_get_or_create</a> (QarSession *session, const QarAppVolumeInit *init, QarAppVolumeId *out_volume)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Get the app volume identified by init-&gt;common_name, creating it (via init) if it does not exist yet. <a href="#ga76e9061c6e95e92787686a130604a140">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gabc30bca0770f1919108af4a4595f964b">qar_app_volumes_remove_gesture_mode</a> (QarSession *session, const QarAppVolumeId *volume_id, QarAppVolumeGestureModeId mode_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Remove a gesture mode. <a href="#gabc30bca0770f1919108af4a4595f964b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gae7d3be5f4cb66ac069112f19b18468af">qar_app_volumes_select_gesture_mode</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarPeerId *peer, QarAppVolumeGestureModeId mode_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Switch one peer, or every peer without its own mode, to a gesture mode. <a href="#gae7d3be5f4cb66ac069112f19b18468af">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gaebae79cbe16d978fa8a168b8b6f72f4a">qar_app_volumes_start_editing</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Request editing lock for the volume. <a href="#gaebae79cbe16d978fa8a168b8b6f72f4a">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga11bb3d726c65a5a196b899a912f0437f">qar_app_volumes_stop_editing</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Release editing lock for the volume. <a href="#ga11bb3d726c65a5a196b899a912f0437f">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga7f99e6edff35577580493375fcc80871">qar_app_volumes_subscribe_all_gesture_events</a> (QarSession *session, const QarAppVolumeId *volume_id, qar_app_volume_gesture_event_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to every gesture kind on one app volume, through one callback. <a href="#ga7f99e6edff35577580493375fcc80871">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gad460dbbdc714a5886ea59df4c5a6ae6a">qar_app_volumes_subscribe_gesture_events</a> (QarSession *session, const QarAppVolumeId *volume_id, QarGestureKind gesture_kind, qar_app_volume_gesture_event_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to one kind of mapped gesture on a specific app volume. <a href="#gad460dbbdc714a5886ea59df4c5a6ae6a">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga688219c3c6d44076df53c1ee0a12f685">qar_app_volumes_subscribe_updates</a> (QarSession *session, qar_app_volume_update_callback_t callback, void *user_state, QarCancelToken *token)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Subscribe to updates for app volumes. <a href="#ga688219c3c6d44076df53c1ee0a12f685">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga1f7530a91be626f8dcea4650a00648b3">qar_app_volumes_update_used_by_peers</a> (QarSession *session, const QarAppVolumeId *volume_id, const QarPeerId *peer_additions, size_t additions_count, const QarPeerId *peer_removals, size_t removals_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Update which peers use the volume. <a href="#ga1f7530a91be626f8dcea4650a00648b3">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga93710b7daad904b3307807ed2eb9f6a8">qar_query_app_volumes</a> (QarSession *session, QarAppVolume **out_handles, size_t handles_buffer_size, size_t *out_handles_written)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Enumerate app volumes into provided array of handles. <a href="#ga93710b7daad904b3307807ed2eb9f6a8">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gadf709eb3fd8918f941eb4759fbfb9f4b">qar_query_app_volumes_count</a> (QarSession *session, size_t *out_count)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Query number of app volumes known to the session. <a href="#gadf709eb3fd8918f941eb4759fbfb9f4b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Typedefs

### qar\_app\_volume\_gesture\_event\_callback\_t {#ga48b2d003e51adfe466ac386ff56e8520}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_app_volume_gesture_event_callback_t)(QarAppVolumeGesture *gesture, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Receives one mapped gesture on a subscribed app volume.</p>


<p>Documented on the public declaration in &lt;qar_streaming_c/v0/api.h&gt;.</p>


<p><span class="doxyComputerOutput">gesture</span> is an opaque handle to library-owned storage, valid <b>only for the duration of this call</b>. Read what you need through the qar_app_volume_gesture_get_*() accessors and copy anything the application keeps; the handle must not be stored. <span class="doxyComputerOutput">user_state</span> is the pointer handed to the subscribe call. The callback runs on a library background thread, so it must not block and must not call back into a subscribe function.</p>


<p>It is a handle rather than a struct so the event can gain fields - a scale delta, a per-hand breakdown, a confidence - by appending an accessor, which is an additive change, instead of growing a struct, which would break every compiled consumer. This mirrors qar_gui_panel_gesture_event_callback_t.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03754">3754</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_update\_callback\_t {#ga7847366f9e5f7a4b2ef741c0e88d842f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">typedef void(* qar_app_volume_update_callback_t)(QarAppVolume *handle, void *user_state)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03736">3736</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<div class="doxySectionDef">

## Functions

### qar\_app\_volume\_gesture\_get\_classification() {#ga7989f43826213560076d3e89365def90}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_classification (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a53f8af808cd6fba6e9ad4d03f44fcde5">QarGestureKind</a> * out_kind, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#aafb0b9385056ee6a2ccd4985bd066a9c">QarGesturePhase</a> * out_phase)</td>
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

<p>What this gesture is and where in its lifecycle it sits.</p>


<p>Returned together because a consumer dispatches on both at once. Both are enums, so a future gesture kind or phase is a new enumerator rather than a change to this call. <span class="doxyComputerOutput">out_kind</span> always equals the kind the subscription asked for; read it so one callback can serve several subscriptions.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_kind</td>
<td class="doxyParamItemDescription"><p>Receives the gesture kind; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_phase</td>
<td class="doxyParamItemDescription"><p>Receives started, updated, ended, canceled or instant; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03881">3881</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_deltas() {#ga686acec23deaa823d19cb150208daf89}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_deltas (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> * out_translation_delta, <a href="/api/qar-streaming-c/structs/qarquaternion">QarQuaternion</a> * out_rotation_delta, bool * out_was_mapped_to_app_transform)</td>
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

<p>The rigid motion accumulated since the gesture started, and whether the library already applied it.</p>


<p><span class="doxyComputerOutput">out_translation_delta</span> is a vector and <span class="doxyComputerOutput">out_rotation_delta</span> a quaternion; together they are one rigid transform, which is why they are read as a pair. Both are in the target volume's <b>app volume space</b> - not app content space, unlike the points - and both are shaped by the volume's mapping rule: the translation is scaled by its precision and scale sensitivity, so it is in metres only for precision 1 with constant sensitivity, and both are masked by its axis flags. QAR_GESTURE_DUAL_POINTER_TRANSLATE_DISTANCE is the exception for the translation: it is the change of the distance between the hands as a vector along their separation in the user's head-yaw frame, whose length is that change, and it is not rotated into the volume.</p>


<p><span class="doxyComputerOutput">out_was_mapped_to_app_transform</span> belongs with them: it is true when the volume's QarGestureAppTransformMapping already folded this delta into the volume's app pose or app scale, so a consumer that applies it again moves the content twice. Reading a delta without it is a double-apply bug, which is why it is not a separate call.</p>


<p>A gesture kind that later gains a scale delta gets its own accessor rather than a fourth out-parameter here.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_translation_delta</td>
<td class="doxyParamItemDescription"><p>Receives the accumulated translation; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_rotation_delta</td>
<td class="doxyParamItemDescription"><p>Receives the accumulated rotation; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_was_mapped_to_app_transform</td>
<td class="doxyParamItemDescription"><p>Receives whether the library already applied the delta; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03957">3957</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_gesture\_id() {#gac5716d790a0a4f31bb120d10448532d3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_gesture_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, uint64_t * out_gesture_id)</td>
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

<p>The gesture's lifecycle id.</p>


<p>The same for a gesture's Started, Updated and Ended or Canceled, and unique within its source peer, so a consumer pairs a lifecycle by the source peer and this id. 0 for a QAR_GESTURE_PHASE_INSTANT gesture, which has no lifecycle.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_gesture_id</td>
<td class="doxyParamItemDescription"><p>Receives the id; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04006">4006</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_handedness() {#gac71bf487f19e0675f487ba6ecdae7ddf}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_handedness (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a656e3047aa5a0e853fd3f9067e0664d6">QarHandedness</a> * out_handedness)</td>
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

<p>Which hand the gesture came from.</p>


<p>A gesture of one hand – hover, click, single-pointer 6DoF – names it: QAR_HANDEDNESS_LEFT or QAR_HANDEDNESS_RIGHT. A gesture of both hands together, QAR_GESTURE_DUAL_POINTER_TRANSLATE_DISTANCE and QAR_GESTURE_DUAL_POINTER_ROTATE, has none and reports QAR_HANDEDNESS_UNSPECIFIED. Two hovers that arrive together are the user's two rays, told apart by this.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_handedness</td>
<td class="doxyParamItemDescription"><p>Receives the hand, or QAR_HANDEDNESS_UNSPECIFIED for a two-hand gesture; never left stale. Must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03990">3990</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_lifecycle() {#gaecb4a7a69567906fcc5d8a078dcf3819}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_lifecycle (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_source_peer_id, <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * out_target_app_volume_id)</td>
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

<p>The lifecycle this gesture belongs to: the peer and the target volume.</p>


<p>They are returned together because they are one key. This event carries no gesture id, so a consumer that stores lifecycles keys on this pair plus the kind from <a href="#ga7989f43826213560076d3e89365def90">qar_app_volume_gesture_get_classification()</a>; a peer runs at most one gesture of a kind against a volume at a time.</p>


<p>The peer is the one whose hand or controller produced the gesture - NOT the peer that recognised it. A mixer runs under its own peer id and publishes for the target peer it serves. The volume is always the one the subscription named, so a callback serving one volume can read it from its user_state instead.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_source_peer_id</td>
<td class="doxyParamItemDescription"><p>Receives the producing peer; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_target_app_volume_id</td>
<td class="doxyParamItemDescription"><p>Receives the volume acted on; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03862">3862</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_mode() {#ga912768a3013ba55c6816dae5dabc1de3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> * out_mode_id)</td>
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

<p>The gesture mode whose rule mapped the gesture.</p>


<p>The source peer's mode when the gesture began: a running gesture keeps it through a mode switch, so its Started, Updated and Ended all report the same mode. Lets an application act on the same gesture differently per mode, such as a click that selects in one mode and measures in another.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_mode_id</td>
<td class="doxyParamItemDescription"><p>Receives the mode; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04022">4022</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_points() {#ga6a0103b0bcb19c7ab12302f9d8b83882}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_points (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> * out_action_point, bool * out_has_action_point, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> * out_start_point, bool * out_has_start_point)</td>
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

<p>Where the gesture is acting now, and where its lifecycle began.</p>


<p>Both are in app content space - the space the source app renders in - and in app units, under the app pose and scale the gesture began with. Both are optional: each point comes with its own validity flag rather than a sentinel value, because the origin is a legal position.</p>


<ul class="doxyList ">
<li>The start point is the hand: where it was when the gesture began, and for a hover where it is now. A two-hand gesture starts midway between the hands.</li>
<li>The action point is the intersection the gesture acts on, or the start point when nothing was intersected, so it is absent only when the start point is too. For a drag it is the content the gesture grabbed.</li>
<li>An absent point is zeroed, never left stale.</li>
</ul>

<p>Returned together because they are read together: from the start point to the action point is the ray the user is pointing along, which an app can draw itself. The travel of a gesture is its delta, not their difference.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture</td>
<td class="doxyParamItemDescription"><p>Gesture handed to the callback; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_action_point</td>
<td class="doxyParamItemDescription"><p>Receives the point the gesture is applied to; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_has_action_point</td>
<td class="doxyParamItemDescription"><p>Receives whether <span class="doxyComputerOutput">out_action_point</span> was filled; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_start_point</td>
<td class="doxyParamItemDescription"><p>Receives where this lifecycle began; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_has_start_point</td>
<td class="doxyParamItemDescription"><p>Receives whether <span class="doxyComputerOutput">out_start_point</span> was filled; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03916">3916</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_gesture\_get\_timestamp() {#ga3d0c820d6bd1690c16dbaf4dcdaa6367}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_gesture_get_timestamp (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ad150e3590c19eda1170b6ad38f28f1da">QarAppVolumeGesture</a> * gesture, <a href="/api/qar-streaming-c/structs/qartimepoint">QarTimePoint</a> * out_timestamp)</td>
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

<p>When the recogniser emitted this gesture, on its own clock.</p>


<p>Not the device's sample time: device clocks are neither comparable nor monotonic across producers, so this is comparable only within one source peer id.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03971">3971</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_active\_gesture\_mode() {#ga58d2b802d6e38245adadb3a077875894}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_active_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * peer, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> * out_mode_id)</td>
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

<p>The gesture mode a peer's next gesture maps through.</p>


<p>The peer's own mode when it has one, the default otherwise. With a NULL <span class="doxyComputerOutput">peer</span>, the default. A mode that is no longer defined reads as QAR_APP_VOLUME_GESTURE_MODE_DEFAULT, which is also what the mapper uses.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>The app volume; must be valid.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer</td>
<td class="doxyParamItemDescription"><p>The peer, or NULL for the default.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_mode_id</td>
<td class="doxyParamItemDescription"><p>Receives the mode; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, or QAR_STATUS_APP_VOLUME_INVALID_ID for an invalid handle.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04338">4338</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_app\_pose() {#ga3fc609ca21893d773e27d9fa2e4e0e34}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_app_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
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

<p>Get app pose for an app volume handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04116">4116</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_app\_scale() {#gab1076b6070e5c652d8659e9adfd98db0}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_app_scale (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, float * out_scale)</td>
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

<p>Get app scale for an app volume handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04119">4119</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_app\_world\_anchor() {#gaf3e5a0be14fa7200ca037fef9ee4c66e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_app_world_anchor (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarappworldanchor">QarAppWorldAnchor</a> * out_anchor)</td>
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

<p>Get app world anchor for an app volume handle.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04121">4121</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_display\_name() {#gaf8c0c2d59ba67ee5edcea8c8556851c6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_display_name (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, char * out_buffer, size_t buffer_size)</td>
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

<p>Get the display name string for an app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04033">4033</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_editing\_status() {#ga2fc57555f013f36d64071c4f7444a415}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_editing_status (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarappvolumeeditingstatus">QarAppVolumeEditingStatus</a> * out_status)</td>
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

<p>Get current editing lock status and editor peer.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04047">4047</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_gesture\_allowed\_peers() {#ga75360b0c91e05e88f2055956f68446bb}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_gesture_allowed_peers (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peers, size_t peer_capacity, size_t * out_peer_count)</td>
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

<p>The peers whose gestures may move or scale the app in this volume.</p>


<p>Empty means every peer's may. Copies at most <span class="doxyComputerOutput">peer_capacity</span> peers into <span class="doxyComputerOutput">out_peers</span> and reports how many there are in <span class="doxyComputerOutput">out_peer_count</span>, so a call with a capacity of 0 and a NULL <span class="doxyComputerOutput">out_peers</span> asks for the count alone. Set with <a href="/api/qar-streaming-c/structs/qarappvolumegestureallowedpeersext">QarAppVolumeGestureAllowedPeersExt</a>.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>The app volume; must be valid.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_peers</td>
<td class="doxyParamItemDescription"><p>Receives up to <span class="doxyComputerOutput">peer_capacity</span> peers; may be NULL when <span class="doxyComputerOutput">peer_capacity</span> is 0.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer_capacity</td>
<td class="doxyParamItemDescription"><p>How many peers <span class="doxyComputerOutput">out_peers</span> holds.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_peer_count</td>
<td class="doxyParamItemDescription"><p>Receives how many peers are allowed, which may exceed <span class="doxyComputerOutput">peer_capacity</span>; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, or QAR_STATUS_APP_VOLUME_INVALID_ID for an invalid handle.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04067">4067</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_gesture\_configuration() {#ga5bd2461175d72e5852a695eed1fab921}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_gesture_configuration (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> * out_config)</td>
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

<p>Get the default gesture mode's configuration for an app volume handle.</p>


<p>Returns the rules of QAR_APP_VOLUME_GESTURE_MODE_DEFAULT; read another mode's with <a href="#ga7a3c643fb7520ca53166f0713fd676b4">qar_app_volume_get_gesture_mode()</a>. Mapping rules are returned in priority order. Earlier entries in <span class="doxyComputerOutput">mapping_rules</span> have higher priority than later entries.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04152">4152</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_gesture\_holder() {#gaaa82c086a96788ab7036cdf116ac3ac2}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_gesture_holder (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peer, bool * out_is_held)</td>
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

<p>Which peer is manipulating the app in this volume, if one is.</p>


<p>One peer at a time moves or scales the app with a gesture: the first whose manipulation binds holds the volume until it ends, and every other peer's manipulations are refused meanwhile. Their hover and click still arrive.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>The app volume; must be valid.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_peer</td>
<td class="doxyParamItemDescription"><p>Receives the holder; the default peer id when none holds it. Must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_is_held</td>
<td class="doxyParamItemDescription"><p>Receives whether a peer holds it; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, or QAR_STATUS_APP_VOLUME_INVALID_ID for an invalid handle.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04088">4088</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_gesture\_mode() {#ga7a3c643fb7520ca53166f0713fd676b4}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> mode_id, <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> * out_config)</td>
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

<p>One gesture mode's rules.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>The app volume; must be valid.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">mode_id</td>
<td class="doxyParamItemDescription"><p>The mode to read.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_config</td>
<td class="doxyParamItemDescription"><p>Receives the mode's rules in priority order, with <span class="doxyComputerOutput">header.next</span> NULL. Must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, QAR_STATUS_APP_VOLUME_INVALID_ID for an invalid handle, or QAR_STATUS_APP_VOLUME_GESTURE_MODE_UNKNOWN for a mode the volume does not have.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04319">4319</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_gesture\_mode\_ids() {#ga7d24689fac07f3c51f7d71daaabb9c7b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_gesture_mode_ids (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> * out_mode_ids, size_t mode_capacity, size_t * out_mode_count)</td>
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

<p>The ids of the gesture modes an app volume defines.</p>


<p>Always includes QAR_APP_VOLUME_GESTURE_MODE_DEFAULT. Copies at most <span class="doxyComputerOutput">mode_capacity</span> ids into <span class="doxyComputerOutput">out_mode_ids</span> and reports how many there are in <span class="doxyComputerOutput">out_mode_count</span>, so a call with a capacity of 0 and a NULL <span class="doxyComputerOutput">out_mode_ids</span> asks for the count alone.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">handle</td>
<td class="doxyParamItemDescription"><p>The app volume; must be valid.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_mode_ids</td>
<td class="doxyParamItemDescription"><p>Receives up to <span class="doxyComputerOutput">mode_capacity</span> ids; may be NULL when <span class="doxyComputerOutput">mode_capacity</span> is 0.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">mode_capacity</td>
<td class="doxyParamItemDescription"><p>How many ids <span class="doxyComputerOutput">out_mode_ids</span> holds.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">out_mode_count</td>
<td class="doxyParamItemDescription"><p>Receives how many modes there are, which may exceed <span class="doxyComputerOutput">mode_capacity</span>; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, or QAR_STATUS_APP_VOLUME_INVALID_ID for an invalid handle.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04300">4300</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_id() {#gaf404764ed9c2c6186ee524be5b10cf0d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_id (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * out_id)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04031">4031</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_latest\_app\_pose() {#ga2ef27f79813e6b84c2447a280e8631b9}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_latest_app_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
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

<p>Get the latest locally known fast-path app pose for this app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04126">4126</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_latest\_app\_scale() {#ga3f442b19c3a356ef0cfdcb193afbaf3f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_latest_app_scale (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, float * out_scale)</td>
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

<p>Get the latest locally known fast-path app scale for this app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04131">4131</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_latest\_pose() {#ga4007afa654390fadd509d0ea6a19a704}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_latest_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
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

<p>Get the latest locally known fast-path pose for this app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04135">4135</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_latest\_size() {#ga28ed3ba4c12104298235fb4d0a614ba7}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_latest_size (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> * out_size)</td>
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

<p>Get the latest locally known fast-path size for this app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04139">4139</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_lifetime\_status() {#gaa57bb480cd560ecfb95d190b866cec34}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_lifetime_status (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a70b470a0c02bc58de619b41adf85dd0b">QarAppVolumeLifetimeStatus</a> * out_status)</td>
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

<p>Get the lifetime status (active/closed).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04043">4043</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_pose() {#ga0b2414ac9f19997acac1e9ba039b6afc}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * out_pose)</td>
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

<p>Get the pose of an app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04038">4038</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_size() {#ga96545524dc43b5038573f31a963694df}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_size (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> * out_size)</td>
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

<p>Get the current size of an app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04041">4041</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_used\_by\_peers() {#gaea671b39b22c73687ca865d66b073c2d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_used_by_peers (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * out_peers, size_t peers_buffer_size, size_t * out_peers_written)</td>
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

<p>Enumerate peers using the volume into provided buffer.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04097">4097</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_get\_used\_by\_peers\_count() {#gaf1a8148682c1f0a52a4f454a5de37324}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volume_get_used_by_peers_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle, size_t * out_peer_count)</td>
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

<p>Get number of peers currently using this volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04093">4093</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_handle\_destroy() {#gaf7c6b09bc22d0cc36286a4c96f08fdff}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_app_volume_handle_destroy (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04029">4029</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volume\_handle\_is\_valid() {#ga70c58f6db3167f6d4de03c682d5e4d8c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_app_volume_handle_is_valid (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> * handle)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04028">4028</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_app\_pose() {#gab61a244bdab51ef38d6920aa9674f8b3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_app_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * app_pose)</td>
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

<p>Update app pose for an existing app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03707">3707</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_app\_scale() {#ga7585bf066d0d3a26353e92b696987fec}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_app_scale (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, float scale)</td>
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

<p>Low-latency fast-path update for latest app scale.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03713">3713</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_app\_world\_anchor() {#ga2831f7924d7aaf86b6c77279c4a3f748}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_app_world_anchor (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qargeoanchorframe">QarGeoAnchorFrame</a> * anchor)</td>
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

<p>Set app world anchor metadata.</p>


<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_ARGUMENT_NOT_SUPPORTED when <span class="doxyComputerOutput">anchor's</span> handedness is QAR_HANDEDNESS_UNSPECIFIED.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03722">3722</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_display\_name() {#ga65913c52d6f65b97c63f28ef0e3c1967}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_display_name (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const char * display_name)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03673">3673</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_gesture\_configuration() {#ga4b7168291f07557695fc83803face6f9}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_gesture_configuration (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> * config)</td>
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

<p>Update the default gesture mode and the allowed peers of an existing app volume.</p>


<p>Replaces the rules of QAR_APP_VOLUME_GESTURE_MODE_DEFAULT and the allowed peers (<a href="/api/qar-streaming-c/structs/qarappvolumegestureallowedpeersext">QarAppVolumeGestureAllowedPeersExt</a>, empty when not chained). Every other mode is kept; define those with <a href="#gab701d8ba8fe7b55d8ed753f7bb788c9c">qar_app_volumes_change_gesture_mode()</a>.</p>


<p>The order of <span class="doxyComputerOutput">config-&gt;mapping_rules</span> defines runtime priority. Earlier rules have higher priority and can suppress lower-priority gestures when both are active at the same time for the same app volume and source peer.</p>


<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> QAR_STATUS_APP_VOLUME_GESTURE_CONFIGURATION_INVALID when a rule could never work – an enabled DualPointerRotate rule that moves the app with <span class="doxyComputerOutput">rotation_axes</span> set to none, which would take two-hand gestures over and turn nothing. The volume keeps its previous configuration. Adding a volume with such a configuration fails the same way.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04172">4172</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_gesture\_mode() {#gab701d8ba8fe7b55d8ed753f7bb788c9c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> mode_id, const <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> * config)</td>
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

<p>Define a gesture mode, or replace the mode with that id.</p>


<p>A mode is a set of mapping rules – what each gesture does and how its ray looks – that peers can be switched to with <a href="#gae7d3be5f4cb66ac069112f19b18468af">qar_app_volumes_select_gesture_mode()</a>, so a user's gestures and rays change with one small call instead of a whole configuration. Every volume has QAR_APP_VOLUME_GESTURE_MODE_DEFAULT; any other id is the application's to choose. Other modes and the allowed peers are kept.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>The app volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">mode_id</td>
<td class="doxyParamItemDescription"><p>The mode to define or replace.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">config</td>
<td class="doxyParamItemDescription"><p>The mode's rules, in priority order. Its <span class="doxyComputerOutput">header.next</span> must be NULL: the allowed peers are the volume's, for every mode, and are set with <a href="#ga4b7168291f07557695fc83803face6f9">qar_app_volumes_change_gesture_configuration()</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, QAR_STATUS_ARGUMENT_NOT_SUPPORTED for a chained extension, or QAR_STATUS_APP_VOLUME_GESTURE_CONFIGURATION_INVALID for a rule that could never work, as with <a href="#ga4b7168291f07557695fc83803face6f9">qar_app_volumes_change_gesture_configuration()</a>.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04200">4200</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_pose() {#ga0f57f76fe79b59f34ed4c4d2d2e2875d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_pose (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * pose)</td>
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

<p>Low-latency fast-path update for latest pose.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03732">3732</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_change\_size() {#ga13c4244a8b083c92d0b137a0e585c42f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_change_size (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> * size)</td>
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

<p>Resize an app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03679">3679</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_clear\_all\_peer\_gesture\_modes() {#gae6c1882410f4181aa0b39ebdcda3a56f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_clear_all_peer_gesture_modes (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
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

<p>Drop every peer's own gesture mode, so all follow the default.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>The app volume; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04279">4279</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_clear\_app\_world\_anchor() {#gaec59bb278f3aeb12dfe4d0b6061bfe57}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_clear_app_world_anchor (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
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

<p>Clear app world anchor metadata.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03728">3728</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_clear\_peer\_gesture\_mode() {#gae799744493713eff4b5aa8e4c15e8c15}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_clear_peer_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * peer)</td>
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

<p>Drop one peer's own gesture mode, so it follows the default again.</p>


<p>Does nothing for a peer that has none.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>The app volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer</td>
<td class="doxyParamItemDescription"><p>The peer; must not be NULL.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, or QAR_STATUS_LOGIC_ERROR for a NULL argument.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04268">4268</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_close\_volume() {#ga93cae823d562ebf2ede131be358721ad}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_close_volume (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
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

<p>Close an app volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03685">3685</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_get\_or\_create() {#ga76e9061c6e95e92787686a130604a140}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_get_or_create (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeinit">QarAppVolumeInit</a> * init, <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * out_volume)</td>
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

<p>Get the app volume identified by init-&gt;common_name, creating it (via init) if it does not exist yet.</p>


<p>Fails with QAR_STATUS_ARGUMENT_NOT_SUPPORTED when init-&gt;has_app_world_anchor is set and init-&gt;app_world_anchor.handedness is QAR_HANDEDNESS_UNSPECIFIED.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03668">3668</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_remove\_gesture\_mode() {#gabc30bca0770f1919108af4a4595f964b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_remove_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> mode_id)</td>
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

<p>Remove a gesture mode.</p>


<p>A peer switched to the removed mode, or the default selection naming it, falls back to QAR_APP_VOLUME_GESTURE_MODE_DEFAULT. A gesture already running in it finishes in it.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>The app volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">mode_id</td>
<td class="doxyParamItemDescription"><p>The mode to remove.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, QAR_STATUS_APP_VOLUME_GESTURE_CONFIGURATION_INVALID for QAR_APP_VOLUME_GESTURE_MODE_DEFAULT, which cannot be removed, or QAR_STATUS_APP_VOLUME_GESTURE_MODE_UNKNOWN for a mode the volume does not have.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04223">4223</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_select\_gesture\_mode() {#gae7d3be5f4cb66ac069112f19b18468af}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_select_gesture_mode (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * peer, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a44c9a5683f6b602e9ca1f6610121fde5">QarAppVolumeGestureModeId</a> mode_id)</td>
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

<p>Switch one peer, or every peer without its own mode, to a gesture mode.</p>


<p>With <span class="doxyComputerOutput">peer</span>, that peer's gestures map through <span class="doxyComputerOutput">mode_id</span> from now on, whatever the default is. With a NULL <span class="doxyComputerOutput">peer</span>, <span class="doxyComputerOutput">mode_id</span> becomes the default every peer without its own mode follows; peers switched individually stay in theirs (clear them with <a href="#gae799744493713eff4b5aa8e4c15e8c15">qar_app_volumes_clear_peer_gesture_mode()</a> or <a href="#gae6c1882410f4181aa0b39ebdcda3a56f">qar_app_volumes_clear_all_peer_gesture_modes()</a>).</p>


<p>A gesture already running keeps the mode it began in, so a drag never changes its mapping halfway; the switch applies from the next gesture. The peer's rays take the new mode's styles as its next gesture is mapped.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session that owns the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>The app volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">peer</td>
<td class="doxyParamItemDescription"><p>The peer to switch, or NULL for the default.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">mode_id</td>
<td class="doxyParamItemDescription"><p>The mode to switch to; the volume must define it.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p>QAR_STATUS_SUCCESS, QAR_STATUS_LOGIC_ERROR for a NULL argument, or QAR_STATUS_APP_VOLUME_GESTURE_MODE_UNKNOWN for a mode the volume does not have.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04251">4251</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_start\_editing() {#gaebae79cbe16d978fa8a168b8b6f72f4a}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_start_editing (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
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

<p>Request editing lock for the volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03698">3698</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_stop\_editing() {#ga11bb3d726c65a5a196b899a912f0437f}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_stop_editing (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
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

<p>Release editing lock for the volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03702">3702</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_subscribe\_all\_gesture\_events() {#ga7f99e6edff35577580493375fcc80871}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_subscribe_all_gesture_events (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="#ga48b2d003e51adfe466ac386ff56e8520">qar_app_volume_gesture_event_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Subscribe to every gesture kind on one app volume, through one callback.</p>


<p>The unfiltered counterpart of <a href="#gad460dbbdc714a5886ea59df4c5a6ae6a">qar_app_volumes_subscribe_gesture_events()</a>: same stream, same handle, no kind filter. It is a separate function rather than a sentinel value in QarGestureKind, because that enum is also what <a href="#ga7989f43826213560076d3e89365def90">qar_app_volume_gesture_get_classification()</a> returns and a value that is not a kind has no meaning there.</p>


<p><b>Prefer this when you take more than one kind.</b> Everything arrives on one reader, so gestures a peer performs against the volume keep the order the producer published them in; several per-kind subscriptions are separate readers with no order relative to each other. Dispatch on the kind from <a href="#ga7989f43826213560076d3e89365def90">qar_app_volume_gesture_get_classification()</a>.</p>


<p>Only the volume's mapping rules decide what is published at all - this removes the C API's kind filter, not the volume's <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a>.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session owning the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>Volume to observe; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">callback</td>
<td class="doxyParamItemDescription"><p>Invoked for every gesture event of every kind; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>Opaque pointer passed back to <span class="doxyComputerOutput">callback</span>; may be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Cancel token owning the subscription; must not be NULL. See <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> Success once the subscription is running.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03835">3835</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_subscribe\_gesture\_events() {#gad460dbbdc714a5886ea59df4c5a6ae6a}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_subscribe_gesture_events (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a53f8af808cd6fba6e9ad4d03f44fcde5">QarGestureKind</a> gesture_kind, <a href="#ga48b2d003e51adfe466ac386ff56e8520">qar_app_volume_gesture_event_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Subscribe to one kind of mapped gesture on a specific app volume.</p>


<p>The gestures a peer performs against the volume are recognised by the peer that renders it and mapped through the volume's <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> before they arrive here, so what the callback receives is already expressed in the target volume's frame and already filtered by that volume's mapping rules.</p>


<p><b>Several producers publish here.</b> Recognition happens once per user, so a volume several users can reach carries one publisher per user on this topic. There is no gesture id: a peer runs at most one gesture of a kind against a volume at a time, so the lifecycle key is the source peer and the target volume from <a href="#gaecb4a7a69567906fcc5d8a078dcf3819">qar_app_volume_gesture_get_lifecycle()</a> together with the kind from <a href="#ga7989f43826213560076d3e89365def90">qar_app_volume_gesture_get_classification()</a>. Ordering holds within one producer's stream, not across producers, which is sufficient because no lifecycle spans two producers. A source peer id never seen before is a new user, not an error, so per-user interaction state is held per peer.</p>


<dl class="doxyParamsList">
<dt class="doxyParamsTableTitle">Parameters</dt>
<dd>
<table class="doxyParamsTable">
<tr class="doxyParamItem">
<td class="doxyParamItemName">session</td>
<td class="doxyParamItemDescription"><p>Session owning the volume; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">volume_id</td>
<td class="doxyParamItemDescription"><p>Volume to observe; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">gesture_kind</td>
<td class="doxyParamItemDescription"><p>The single gesture kind this subscription delivers.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">callback</td>
<td class="doxyParamItemDescription"><p>Invoked for every matching gesture event; must not be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">user_state</td>
<td class="doxyParamItemDescription"><p>Opaque pointer passed back to <span class="doxyComputerOutput">callback</span>; may be NULL.</p></td>
</tr>
<tr class="doxyParamItem">
<td class="doxyParamItemName">token</td>
<td class="doxyParamItemDescription"><p>Cancel token owning the subscription; must not be NULL. See <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a subscription</a>.</p></td>
</tr>
</table>
</dd>
</dl>

<dl class="doxySectionUser">
<dt>Returns</dt>
<dd><p><a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> Success once the subscription is running.</p></dd>
</dl>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03797">3797</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_subscribe\_updates() {#ga688219c3c6d44076df53c1ee0a12f685}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_subscribe_updates (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="#ga7847366f9e5f7a4b2ef741c0e88d842f">qar_app_volume_update_callback_t</a> callback, void * user_state, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#adff046297201925e9eb5f1bfcfcbe3b2">QarCancelToken</a> * token)</td>
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

<p>Subscribe to updates for app volumes.</p>


<p><span class="doxyComputerOutput">token</span> is required; see <a href="/api/qar-streaming-c/groups/qar-c-cancel/#qar_c_subscription_end">Ending a</a>subscription" for when callbacks stop and how long <span class="doxyComputerOutput">user_state</span> must live.</p>


<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03763">3763</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_app\_volumes\_update\_used\_by\_peers() {#ga1f7530a91be626f8dcea4650a00648b3}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_app_volumes_update_used_by_peers (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * peer_additions, size_t additions_count, const <a href="/api/qar-streaming-c/structs/qarpeerid">QarPeerId</a> * peer_removals, size_t removals_count)</td>
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

<p>Update which peers use the volume.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l03689">3689</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_app\_volumes() {#ga93710b7daad904b3307807ed2eb9f6a8}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_app_volumes (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a> ** out_handles, size_t handles_buffer_size, size_t * out_handles_written)</td>
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

<p>Enumerate app volumes into provided array of handles.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04107">4107</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_query\_app\_volumes\_count() {#gadf709eb3fd8918f941eb4759fbfb9f4b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_query_app_volumes_count (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, size_t * out_count)</td>
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

<p>Query number of app volumes known to the session.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l04105">4105</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
