---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - file

---

<div class="doxyPage">

# `app_volume_management.c` File

<p>Creates an app volume and enumerates existing volumes in the session. <a href="#details">More...</a></p>

## Included Headers

<div class="doxyIncludesList">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>"
#include &lt;math.h&gt;
#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>&gt;
</div>

## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a364b40d134d4fa7f72be8e50632f6bde">configure_gestures</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[app_gestures] <a href="#a364b40d134d4fa7f72be8e50632f6bde">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qargeoanchorframe">QarGeoAnchorFrame</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#aa91e710b938273f1219cfade9ed20d73">geo_anchor_from_lat_lon</a> (double lat_deg, double lon_deg, double height_m)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[app_gestures] <a href="#aa91e710b938273f1219cfade9ed20d73">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#af198e6c5f83fc195ef3c0c99fe4cd5d6">hit_test_room_point</a> (QarSession *session, const QarAppVolumeId *volume_id, QarVector3 room_point)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#accd1bb4133381383f3aa612fe4924efb">is_inside_volume</a> (const QarAppVolumeSize *size, QarVector3 volume_point)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>True when a point in app volume space lies inside the cuboid. <a href="#accd1bb4133381383f3aa612fe4924efb">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">int</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a3c04138a5bfe5d72780bb7e82a18e627">main</a> (int argc, char **argv)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[app_room_to_app] <a href="#a3c04138a5bfe5d72780bb7e82a18e627">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ac9cfc96dfe2facfa6ca4939058f48b5a">pin_and_unpin_world_anchor</a> (QarSession *session, const QarAppVolumeId *volume_id)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a7bf4a3818b91f144183c65e1d555b627">print_usage</a> (const char *program_name)</td>
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
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#afd3b6928958ab167e118b84e15711785">room_to_volume</a> (const QarPose *volume_pose, QarVector3 room_point)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Room space -&gt; app volume space. The volume pose is the cuboid centre, so the result is relative to that centre. <a href="#afd3b6928958ab167e118b84e15711785">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a153832beff30ac69c56193700ee25021">rotate_by_inverse</a> (QarQuaternion q, QarVector3 v)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[app_world_anchor] <a href="#a153832beff30ac69c56193700ee25021">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a39fd8a6c4b440b04779481b7f607253d">volume_to_app_content</a> (const QarPose *app_pose, float app_scale, QarVector3 volume_point)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>App volume space -&gt; app content space: undo the app pose (volume metres, unscaled), then divide by app_scale (room metres per app metre). <a href="#a39fd8a6c4b440b04779481b7f607253d">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}

<p>Creates an app volume and enumerates existing volumes in the session.</p>

<div class="doxySectionDef">

## Functions

### configure\_gestures() {#a364b40d134d4fa7f72be8e50632f6bde}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void configure_gestures (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>[app_gestures]</p>

<p>Definition at line <a href="#l00094">94</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### geo\_anchor\_from\_lat\_lon() {#aa91e710b938273f1219cfade9ed20d73}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarGeoAnchorFrame geo_anchor_from_lat_lon (double lat_deg, double lon_deg, double height_m)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>[app_gestures]</p>


<p>[app_world_anchor]</p>


<p>Build an east/up/south anchor frame at a WGS84 latitude/longitude (degrees) and ellipsoid height (metres).</p>


<p>Definition at line <a href="#l00127">127</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### hit\_test\_room\_point() {#af198e6c5f83fc195ef3c0c99fe4cd5d6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void hit_test_room_point (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> room_point)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="#l00229">229</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### is\_inside\_volume() {#accd1bb4133381383f3aa612fe4924efb}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool is_inside_volume (const <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> * size, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> volume_point)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>True when a point in app volume space lies inside the cuboid.</p>

<p>Definition at line <a href="#l00221">221</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### main() {#a3c04138a5bfe5d72780bb7e82a18e627}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">int main (int argc, char ** argv)</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>[app_room_to_app]</p>

<p>Definition at line <a href="#l00263">263</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### pin\_and\_unpin\_world\_anchor() {#ac9cfc96dfe2facfa6ca4939058f48b5a}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void pin_and_unpin_world_anchor (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> * session, const <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> * volume_id)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="#l00160">160</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### print\_usage() {#a7bf4a3818b91f144183c65e1d555b627}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void print_usage (const char * program_name)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">



<p>Definition at line <a href="#l00078">78</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### room\_to\_volume() {#afd3b6928958ab167e118b84e15711785}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarVector3 room_to_volume (const <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * volume_pose, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> room_point)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Room space -&gt; app volume space. The volume pose is the cuboid centre, so the result is relative to that centre.</p>

<p>Definition at line <a href="#l00198">198</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### rotate\_by\_inverse() {#a153832beff30ac69c56193700ee25021}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarVector3 rotate_by_inverse (<a href="/api/qar-streaming-c/structs/qarquaternion">QarQuaternion</a> q, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> v)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>[app_world_anchor]</p>


<p>[app_room_to_app]</p>


<p>Rotate v by the inverse of the unit quaternion q.</p>


<p>Definition at line <a href="#l00181">181</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

### volume\_to\_app\_content() {#a39fd8a6c4b440b04779481b7f607253d}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarVector3 volume_to_app_content (const <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> * app_pose, float app_scale, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> volume_point)</td>
</tr>
</table>
</td>
<td class="doxyMemberLabelsRight">
<span class="doxyMemberLabels">
<span class="doxyMemberLabel static">static</span>
</span>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>App volume space -&gt; app content space: undo the app pose (volume metres, unscaled), then divide by app_scale (room metres per app metre).</p>

<p>Definition at line <a href="#l00210">210</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c">app_volume_management.c</a>.</p>

</div>
</div>

</div>

## File Listing

The file content with the documentation metadata removed is:

<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00001"></a>1</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \file app_volume_management.c</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00002"></a>2</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  \brief Creates an app volume and enumerates existing volumes in the session.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00003"></a>3</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  \example app_volume_management.c</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00004"></a>4</span><span class="doxyLineContent"><span class="doxyHighlightComment"> */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00005"></a>5</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00006"></a>6</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \page qar_c_tutorial_app_volumes App Volume Management</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00007"></a>7</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \tableofcontents</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00008"></a>8</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00009"></a>9</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_overview Overview</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00010"></a>10</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Obtain a session (rejoin, or onboard with the hub pairing code)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00011"></a>11</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Get-or-create a single app volume by its stable common name</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00012"></a>12</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Enumerate and inspect every app volume handle</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00013"></a>13</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Customise the volume's gesture mapping rules</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00014"></a>14</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Set and clear a world anchor</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00015"></a>15</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Map a room-space point into app content space and hit-test the cuboid</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00016"></a>16</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00017"></a>17</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_prereq Prerequisites</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00018"></a>18</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Complete the \ref qar_c_tutorial_onboarding tutorial</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00019"></a>19</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - A running QarOS hub (for the first run, read the pairing code off its</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00020"></a>20</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *   onboarding screen)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00021"></a>21</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00022"></a>22</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_build Build and Run</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00023"></a>23</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \code{.bash}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00024"></a>24</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * cmake --build --preset x64-windows-debug --target app_volume_management</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00025"></a>25</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * ./build/x64-windows/Debug/app_volume_management.exe</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00026"></a>26</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * &lt;path-to-qar-streaming-c.dll&gt; [runtime-dir] [pairing-code]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00027"></a>27</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \endcode</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00028"></a>28</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00029"></a>29</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_args Parse Arguments</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00030"></a>30</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_args</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00031"></a>31</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00032"></a>32</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_setup Load, Init, and Obtain a Session</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00033"></a>33</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * The session is obtained with the shared rejoin-or-onboard helper described</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00034"></a>34</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * in the \ref qar_c_tutorial_onboarding tutorial.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00035"></a>35</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_setup</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00036"></a>36</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00037"></a>37</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_create Create and List</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00038"></a>38</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_create</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00039"></a>39</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00040"></a>40</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_gestures Customise the Gesture Configuration</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00041"></a>41</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * Start from qar_app_volume_gesture_configuration_default(), which mirrors</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00042"></a>42</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * what a volume created without a configuration gets: two-hand distance -&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00043"></a>43</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * app scale, one-hand 6DoF -&gt; app pose (rotation off), hover, click. Rules are</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00044"></a>44</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * in descending priority. Change a rule in place, then replace the volume's</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00045"></a>45</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * default mode with qar_app_volumes_change_gesture_configuration(). The same</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00046"></a>46</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * struct can instead be passed in QarAppVolumeInit::gesture_configuration, but</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00047"></a>47</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * get-or-create ignores it when the volume already exists.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00048"></a>48</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_gestures</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00049"></a>49</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00050"></a>50</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_anchor Pin to the Earth with a World Anchor</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00051"></a>51</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * A world anchor states where app volume space sits on the Earth, as an ECEF</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00052"></a>52</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * WGS84 frame. Here the volume's +X points east, +Y up and +Z south of a</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00053"></a>53</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * latitude/longitude, which keeps the frame right-handed.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00054"></a>54</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_world_anchor</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00055"></a>55</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00056"></a>56</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section app_volume_hit_test Room-Space Point to App Content Space</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00057"></a>57</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * Gesture points already arrive in app content space. A point the application</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00058"></a>58</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * has only in room space - a GUI panel's position, a device pose - is mapped</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00059"></a>59</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * in two steps: room -&gt; app volume space through the volume pose, whose</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00060"></a>60</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * position is the cuboid **centre**, then app volume -&gt; app content space</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00061"></a>61</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * through the app pose and app scale. The cuboid test happens in app volume</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00062"></a>62</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * space against the half extents.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00063"></a>63</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet app_volume_management.c app_room_to_app</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00064"></a>64</span><span class="doxyLineContent"><span class="doxyHighlightComment"> */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00065"></a>65</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00066"></a>66</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00067"></a>67</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00068"></a>68</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;math.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00069"></a>69</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00070"></a>70</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#ifndef QAR_ENABLE_DYNAMIC_LOADING</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00071"></a>71</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#define QAR_ENABLE_DYNAMIC_LOADING</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00072"></a>72</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#endif</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00073"></a>73</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00074"></a>74</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00075"></a>75</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#af720efda6e2714bd1d8a447ba7243b08">QAR_IMPLEMENT_DYNAMIC_LOADING</a>()</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00076"></a>76</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00077"></a>77</span><span class="doxyLineContent"><span class="doxyHighlight">static </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00078"></a>78</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a7bf4a3818b91f144183c65e1d555b627">print_usage</a>(const </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* program_name)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00079"></a>79</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00080"></a>80</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* name = program_name ? program_name : </span><span class="doxyHighlightStringLiteral">"app_volume_management"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00081"></a>81</span><span class="doxyLineContent"><span class="doxyHighlight">    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00082"></a>82</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"Usage: %s &lt;path-to-qar-streaming-c-library&gt; [runtime-binaries-dir] "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00083"></a>83</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"[pairing-code]\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00084"></a>84</span><span class="doxyLineContent"><span class="doxyHighlight">        name</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00085"></a>85</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00086"></a>86</span><span class="doxyLineContent"><span class="doxyHighlight">    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00087"></a>87</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"The pairing code is required on the first run only; later runs "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00088"></a>88</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"rejoin with the persisted onboarding id.\n"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00089"></a>89</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00090"></a>90</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00091"></a>91</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00092"></a>92</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_gestures]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00093"></a>93</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00094"></a>94</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a364b40d134d4fa7f72be8e50632f6bde">configure_gestures</a>(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session, </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a>* volume_id)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00095"></a>95</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00096"></a>96</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration">QarAppVolumeGestureConfiguration</a> config = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga5d98c4f156839988e58289352ad92e54">qar_app_volume_gesture_configuration_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00097"></a>97</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00098"></a>98</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; config.<a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration/#aa12974dee7186973a4a14c55924c3f80">mapping_rule_count</a>; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00099"></a>99</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00100"></a>100</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule">QarAppVolumeGestureMappingRule</a>* rule = &amp;config.<a href="/api/qar-streaming-c/structs/qarappvolumegestureconfiguration/#a7f4e480e481659916ca69048c5ff7b29">mapping_rules</a>[i];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00101"></a>101</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a6f8e692a4f340124cace32367f1b5332">gesture_kind</a> == <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a53f8af808cd6fba6e9ad4d03f44fcde5ace89dd2a2f2c43aa3976fcb1102ba0de">QAR_GESTURE_SINGLE_POINTER_6DOF</a>)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00102"></a>102</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00103"></a>103</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightComment">/* Finer one-hand drag (2 cm of hand -&gt; 1 cm of app), sliding</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00104"></a>104</span><span class="doxyLineContent"><span class="doxyHighlightComment">             * along the floor plane and turning about the vertical axis. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00105"></a>105</span><span class="doxyLineContent"><span class="doxyHighlight">            rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a5362d138eff6dac2759db8586e50b053">precision</a> = 2.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00106"></a>106</span><span class="doxyLineContent"><span class="doxyHighlight">            rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a1bd34a84d2118fb56ab1b1312e8496ec">translation_axes</a> = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a39d168c033223f533eaeeeb522fc2f6baf2765a32a1b8e89f63de400178ee1f61">QAR_APP_VOLUME_AXIS_X</a> | <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a39d168c033223f533eaeeeb522fc2f6bab4eff00066aca3d4a3e2ee2cc8192b4d">QAR_APP_VOLUME_AXIS_Z</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00107"></a>107</span><span class="doxyLineContent"><span class="doxyHighlight">            rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a750fe6b97955df4176c3aacc258ac94a">rotation_axes</a> = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a39d168c033223f533eaeeeb522fc2f6bae3b51ca72ca91d36f67ffb54f2eaa477">QAR_APP_VOLUME_AXIS_Y</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00108"></a>108</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00109"></a>109</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">else</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a6f8e692a4f340124cace32367f1b5332">gesture_kind</a> == <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a53f8af808cd6fba6e9ad4d03f44fcde5a387d679cc458aa77afe1a2489aa345e5">QAR_GESTURE_CLICK</a>)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00110"></a>110</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00111"></a>111</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightComment">/* Clicks only count when the ray starts inside the box. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00112"></a>112</span><span class="doxyLineContent"><span class="doxyHighlight">            rule-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumegesturemappingrule/#a658858ff324dd6db51dbea89470ab647">enable_gestures_from_outside_of_volume</a> = </span><span class="doxyHighlightKeyword">false</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00113"></a>113</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00114"></a>114</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00115"></a>115</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00116"></a>116</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00117"></a>117</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_app_volumes_change_gesture_configuration"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00118"></a>118</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga4b7168291f07557695fc83803face6f9">qar_app_volumes_change_gesture_configuration</a>(session, volume_id, &amp;config)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00119"></a>119</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00120"></a>120</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00121"></a>121</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_gestures]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00122"></a>122</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00123"></a>123</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_world_anchor]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00124"></a>124</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Build an east/up/south anchor frame at a WGS84 latitude/longitude</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00125"></a>125</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  (degrees) and ellipsoid height (metres). */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00126"></a>126</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qargeoanchorframe">QarGeoAnchorFrame</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00127"></a>127</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#aa91e710b938273f1219cfade9ed20d73">geo_anchor_from_lat_lon</a>(</span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> lat_deg, </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> lon_deg, </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> height_m)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00128"></a>128</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00129"></a>129</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> a = 6378137.0;         </span><span class="doxyHighlightComment">/* WGS84 semi-major axis */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00130"></a>130</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> e2 = 6.69437999014e-3; </span><span class="doxyHighlightComment">/* WGS84 first eccentricity^2 */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00131"></a>131</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> deg = 3.14159265358979323846 / 180.0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00132"></a>132</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> sin_lat = sin(lat_deg * deg);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00133"></a>133</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> cos_lat = cos(lat_deg * deg);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00134"></a>134</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> sin_lon = sin(lon_deg * deg);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00135"></a>135</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> cos_lon = cos(lon_deg * deg);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00136"></a>136</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">double</span><span class="doxyHighlight"> n = a / sqrt(1.0 - e2 * sin_lat * sin_lat);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00137"></a>137</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00138"></a>138</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qargeoanchorframe">QarGeoAnchorFrame</a> frame;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00139"></a>139</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#a43c18adb5621b95c0cbc64941320a87f">world_ref_system</a> = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#af9dfeda9ad976140064a6370708147d4ad9893f8000f8b8bef7b9ed205c72a558">QAR_WORLD_REF_SYSTEM_ECEF_WGS84_METERS</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00140"></a>140</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#a7026969367ed48e782bb129445d12ce2">handedness</a> = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a656e3047aa5a0e853fd3f9067e0664d6a850856f9b8ce9192c0a4a005473a9b02">QAR_HANDEDNESS_RIGHT</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00141"></a>141</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac07bd5f1c3f5c9d7a145be087d4d7f53">origin_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a73bfc74bd5cae9e007a16c45bc1a318c">x</a> = (n + height_m) * cos_lat * cos_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00142"></a>142</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac07bd5f1c3f5c9d7a145be087d4d7f53">origin_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#ace2a55cecb5ef4812e950b81fd2baed9">y</a> = (n + height_m) * cos_lat * sin_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00143"></a>143</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac07bd5f1c3f5c9d7a145be087d4d7f53">origin_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a060c3b166df9b59e60b51315a5c2dea8">z</a> = (n * (1.0 - e2) + height_m) * sin_lat;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00144"></a>144</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* +X east */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00145"></a>145</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#aac50b3e514e19429b503f5cb6de086e9">axis_x_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a73bfc74bd5cae9e007a16c45bc1a318c">x</a> = -sin_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00146"></a>146</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#aac50b3e514e19429b503f5cb6de086e9">axis_x_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#ace2a55cecb5ef4812e950b81fd2baed9">y</a> = cos_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00147"></a>147</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#aac50b3e514e19429b503f5cb6de086e9">axis_x_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a060c3b166df9b59e60b51315a5c2dea8">z</a> = 0.0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00148"></a>148</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* +Y up */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00149"></a>149</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#a6cbcbdb0b8f91f50f7b3ef41ea832543">axis_y_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a73bfc74bd5cae9e007a16c45bc1a318c">x</a> = cos_lat * cos_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00150"></a>150</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#a6cbcbdb0b8f91f50f7b3ef41ea832543">axis_y_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#ace2a55cecb5ef4812e950b81fd2baed9">y</a> = cos_lat * sin_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00151"></a>151</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#a6cbcbdb0b8f91f50f7b3ef41ea832543">axis_y_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a060c3b166df9b59e60b51315a5c2dea8">z</a> = sin_lat;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00152"></a>152</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* +Z south = east x up */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00153"></a>153</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac3ec4509e328d74cd7422dee806ca042">axis_z_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a73bfc74bd5cae9e007a16c45bc1a318c">x</a> = sin_lat * cos_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00154"></a>154</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac3ec4509e328d74cd7422dee806ca042">axis_z_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#ace2a55cecb5ef4812e950b81fd2baed9">y</a> = sin_lat * sin_lon;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00155"></a>155</span><span class="doxyLineContent"><span class="doxyHighlight">    frame.<a href="/api/qar-streaming-c/structs/qargeoanchorframe/#ac3ec4509e328d74cd7422dee806ca042">axis_z_world</a>.<a href="/api/qar-streaming-c/structs/qarvector3d/#a060c3b166df9b59e60b51315a5c2dea8">z</a> = -cos_lat;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00156"></a>156</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> frame;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00157"></a>157</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00158"></a>158</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00159"></a>159</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00160"></a>160</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#ac9cfc96dfe2facfa6ca4939058f48b5a">pin_and_unpin_world_anchor</a>(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session, </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a>* volume_id)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00161"></a>161</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00162"></a>162</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qargeoanchorframe">QarGeoAnchorFrame</a> anchor = <a href="#aa91e710b938273f1219cfade9ed20d73">geo_anchor_from_lat_lon</a>(50.0870, 14.4208, 200.0);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00163"></a>163</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> set_result = <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga2831f7924d7aaf86b6c77279c4a3f748">qar_app_volumes_change_app_world_anchor</a>(session, volume_id, &amp;anchor);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00164"></a>164</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_app_volumes_change_app_world_anchor"</span><span class="doxyHighlight">, set_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00165"></a>165</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(set_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00166"></a>166</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00167"></a>167</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00168"></a>168</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00169"></a>169</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00170"></a>170</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* ... the volume is now pinned; clear it when it should float again. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00171"></a>171</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00172"></a>172</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_app_volumes_clear_app_world_anchor"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00173"></a>173</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gaec59bb278f3aeb12dfe4d0b6061bfe57">qar_app_volumes_clear_app_world_anchor</a>(session, volume_id)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00174"></a>174</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00175"></a>175</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00176"></a>176</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_world_anchor]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00177"></a>177</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00178"></a>178</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_room_to_app]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00179"></a>179</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Rotate v by the inverse of the unit quaternion q. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00180"></a>180</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00181"></a>181</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a153832beff30ac69c56193700ee25021">rotate_by_inverse</a>(<a href="/api/qar-streaming-c/structs/qarquaternion">QarQuaternion</a> q, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> v)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00182"></a>182</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00183"></a>183</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* v' = v + w t + u x t with u = -q.xyz and t = 2 (u x v) */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00184"></a>184</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> ux = -q.<a href="/api/qar-streaming-c/structs/qarquaternion/#aed738efd42014fa19df2b441a2817758">x</a>, uy = -q.<a href="/api/qar-streaming-c/structs/qarquaternion/#a464915e95da94e85a8f9a735a95fd2c6">y</a>, uz = -q.<a href="/api/qar-streaming-c/structs/qarquaternion/#ae97035c3e3947e062d5075630d57d19d">z</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00185"></a>185</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> tx = 2.0f * (uy * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> - uz * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00186"></a>186</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> ty = 2.0f * (uz * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> - ux * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00187"></a>187</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> tz = 2.0f * (ux * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> - uy * v.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00188"></a>188</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> r;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00189"></a>189</span><span class="doxyLineContent"><span class="doxyHighlight">    r.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = v.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> + q.<a href="/api/qar-streaming-c/structs/qarquaternion/#a2ec8f59503aacd69637beb491953590d">w</a> * tx + (uy * tz - uz * ty);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00190"></a>190</span><span class="doxyLineContent"><span class="doxyHighlight">    r.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = v.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> + q.<a href="/api/qar-streaming-c/structs/qarquaternion/#a2ec8f59503aacd69637beb491953590d">w</a> * ty + (uz * tx - ux * tz);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00191"></a>191</span><span class="doxyLineContent"><span class="doxyHighlight">    r.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = v.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> + q.<a href="/api/qar-streaming-c/structs/qarquaternion/#a2ec8f59503aacd69637beb491953590d">w</a> * tz + (ux * ty - uy * tx);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00192"></a>192</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> r;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00193"></a>193</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00194"></a>194</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00195"></a>195</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Room space -&gt; app volume space. The volume pose is the cuboid</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00196"></a>196</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  centre, so the result is relative to that centre. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00197"></a>197</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00198"></a>198</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#afd3b6928958ab167e118b84e15711785">room_to_volume</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a>* volume_pose, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> room_point)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00199"></a>199</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00200"></a>200</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> d;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00201"></a>201</span><span class="doxyLineContent"><span class="doxyHighlight">    d.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> - volume_pose-&gt;<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00202"></a>202</span><span class="doxyLineContent"><span class="doxyHighlight">    d.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> - volume_pose-&gt;<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00203"></a>203</span><span class="doxyLineContent"><span class="doxyHighlight">    d.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> - volume_pose-&gt;<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00204"></a>204</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> <a href="#a153832beff30ac69c56193700ee25021">rotate_by_inverse</a>(volume_pose-&gt;<a href="/api/qar-streaming-c/structs/qarpose/#a6126e2b33a2d253c286ff287f7d73b29">orientation</a>, d);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00205"></a>205</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00206"></a>206</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00207"></a>207</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief App volume space -&gt; app content space: undo the app pose (volume</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00208"></a>208</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  metres, unscaled), then divide by app_scale (room metres per app metre). */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00209"></a>209</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00210"></a>210</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a39fd8a6c4b440b04779481b7f607253d">volume_to_app_content</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a>* app_pose, </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> app_scale, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> volume_point)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00211"></a>211</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00212"></a>212</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> app = <a href="#afd3b6928958ab167e118b84e15711785">room_to_volume</a>(app_pose, volume_point);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00213"></a>213</span><span class="doxyLineContent"><span class="doxyHighlight">    app.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> /= app_scale;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00214"></a>214</span><span class="doxyLineContent"><span class="doxyHighlight">    app.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> /= app_scale;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00215"></a>215</span><span class="doxyLineContent"><span class="doxyHighlight">    app.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> /= app_scale;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00216"></a>216</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> app;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00217"></a>217</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00218"></a>218</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00219"></a>219</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief True when a point in app volume space lies inside the cuboid. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00220"></a>220</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">bool</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00221"></a>221</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#accd1bb4133381383f3aa612fe4924efb">is_inside_volume</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a>* size, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> volume_point)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00222"></a>222</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00223"></a>223</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> fabsf(volume_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>) &lt;= 0.5f * size-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumesize/#aae5fa8eb081d2d741815132b89eb5837">width_meters</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00224"></a>224</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;&amp; fabsf(volume_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>) &lt;= 0.5f * size-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a4588bfc36f49bf06870b9ccc4e2532be">height_meters</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00225"></a>225</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;&amp; fabsf(volume_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>) &lt;= 0.5f * size-&gt;<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a9a1934c1fc17783ffff15717e8821564">length_meters</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00226"></a>226</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00227"></a>227</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00228"></a>228</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00229"></a>229</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#af198e6c5f83fc195ef3c0c99fe4cd5d6">hit_test_room_point</a>(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session, </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a>* volume_id, <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> room_point)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00230"></a>230</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00231"></a>231</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> volume_pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00232"></a>232</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> size = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga9a06d2b56897aa2048f3651defb1c3d4">qar_app_volume_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00233"></a>233</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> app_pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00234"></a>234</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> app_scale = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00235"></a>235</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00236"></a>236</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga4007afa654390fadd509d0ea6a19a704">qar_app_volume_get_latest_pose</a>(session, volume_id, &amp;volume_pose))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00237"></a>237</span><span class="doxyLineContent"><span class="doxyHighlight">       || <a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga28ed3ba4c12104298235fb4d0a614ba7">qar_app_volume_get_latest_size</a>(session, volume_id, &amp;size))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00238"></a>238</span><span class="doxyLineContent"><span class="doxyHighlight">       || <a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga2ef27f79813e6b84c2447a280e8631b9">qar_app_volume_get_latest_app_pose</a>(session, volume_id, &amp;app_pose))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00239"></a>239</span><span class="doxyLineContent"><span class="doxyHighlight">       || <a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga3f442b19c3a356ef0cfdcb193afbaf3f">qar_app_volume_get_latest_app_scale</a>(session, volume_id, &amp;app_scale))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00240"></a>240</span><span class="doxyLineContent"><span class="doxyHighlight">       || app_scale &lt;= 0.0f)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00241"></a>241</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00242"></a>242</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Volume state not available yet\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00243"></a>243</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00244"></a>244</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00245"></a>245</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00246"></a>246</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> in_volume = <a href="#afd3b6928958ab167e118b84e15711785">room_to_volume</a>(&amp;volume_pose, room_point);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00247"></a>247</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> in_app = <a href="#a39fd8a6c4b440b04779481b7f607253d">volume_to_app_content</a>(&amp;app_pose, app_scale, in_volume);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00248"></a>248</span><span class="doxyLineContent"><span class="doxyHighlight">    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00249"></a>249</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"Room point (%.2f, %.2f, %.2f) -&gt; app (%.2f, %.2f, %.2f), %s the "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00250"></a>250</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"volume\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00251"></a>251</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00252"></a>252</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00253"></a>253</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00254"></a>254</span><span class="doxyLineContent"><span class="doxyHighlight">        in_app.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00255"></a>255</span><span class="doxyLineContent"><span class="doxyHighlight">        in_app.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00256"></a>256</span><span class="doxyLineContent"><span class="doxyHighlight">        in_app.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00257"></a>257</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#accd1bb4133381383f3aa612fe4924efb">is_inside_volume</a>(&amp;size, in_volume) ? </span><span class="doxyHighlightStringLiteral">"inside"</span><span class="doxyHighlight"> : </span><span class="doxyHighlightStringLiteral">"outside"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00258"></a>258</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00259"></a>259</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00260"></a>260</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [app_room_to_app]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00261"></a>261</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00262"></a>262</span><span class="doxyLineContent"><span class="doxyHighlightKeywordType">int</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00263"></a>263</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a3c04138a5bfe5d72780bb7e82a18e627">main</a>(</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight"> argc, </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">** argv)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00264"></a>264</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00265"></a>265</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_args]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00266"></a>266</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &lt; 2)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00267"></a>267</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00268"></a>268</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#a7bf4a3818b91f144183c65e1d555b627">print_usage</a>(argv[0]);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00269"></a>269</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00270"></a>270</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00271"></a>271</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00272"></a>272</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* library_path = argv[1];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00273"></a>273</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* runtime_dir = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00274"></a>274</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> runtime_dir_buffer[1024] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00275"></a>275</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00276"></a>276</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &gt;= 3)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00277"></a>277</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00278"></a>278</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = argv[2];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00279"></a>279</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00280"></a>280</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">else</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#aefc887c8f5c861b4e3949555aedf21c7">get_dir_from_path</a>(library_path, runtime_dir_buffer, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(runtime_dir_buffer)))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00281"></a>281</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00282"></a>282</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = runtime_dir_buffer;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00283"></a>283</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00284"></a>284</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00285"></a>285</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* pairing_code = (argc &gt;= 4) ? argv[3] : NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00286"></a>286</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_args]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00287"></a>287</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00288"></a>288</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_setup]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00289"></a>289</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!<a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga2f183f2df1f2f9db71fe520d154b1606">qar_library_load</a>(library_path))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00290"></a>290</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00291"></a>291</span><span class="doxyLineContent"><span class="doxyHighlight">        fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"Failed to load '%s'.\n"</span><span class="doxyHighlight">, library_path);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00292"></a>292</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 2;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00293"></a>293</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00294"></a>294</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00295"></a>295</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarlibraryinit">QarLibraryInit</a> library_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac75cdcedfeb7cbbfae40405d5821e5b0">qar_library_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00296"></a>296</span><span class="doxyLineContent"><span class="doxyHighlight">    library_init.<a href="/api/qar-streaming-c/structs/qarlibraryinit/#a4a4126bb11230817ddd769e5c2b41df1">enable_console_logging</a> = </span><span class="doxyHighlightKeyword">true</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00297"></a>297</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> library_result = <a href="/api/qar-streaming-c/groups/qar-c-library/#ga7cc03dea04e7fd44d19fe459363d1df0">qar_library_init</a>(&amp;library_init);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00298"></a>298</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(library_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00299"></a>299</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00300"></a>300</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_library_init"</span><span class="doxyHighlight">, library_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00301"></a>301</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00302"></a>302</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 3;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00303"></a>303</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00304"></a>304</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00305"></a>305</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a>* runtime = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00306"></a>306</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarruntimeinit">QarRuntimeInit</a> runtime_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga356269ae6fe65b091aa705537030ed98">qar_runtime_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00307"></a>307</span><span class="doxyLineContent"><span class="doxyHighlight">    runtime_init.<a href="/api/qar-streaming-c/structs/qarruntimeinit/#aa3da4381535f98bb43257058308d9f86">runtime_binaries_folder_path</a> = runtime_dir;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00308"></a>308</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00309"></a>309</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> runtime_result = <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga6a057004f666fdb8632dded376d634c3">qar_runtime_create</a>(&amp;runtime_init, &amp;runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00310"></a>310</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(runtime_result) || runtime == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00311"></a>311</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00312"></a>312</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_runtime_create"</span><span class="doxyHighlight">, runtime_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00313"></a>313</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00314"></a>314</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00315"></a>315</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 4;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00316"></a>316</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00317"></a>317</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00318"></a>318</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> onboarding_id = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga0f002137f2f3d3e5d7b27cc52742cb86">qar_onboarding_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00319"></a>319</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00320"></a>320</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a76cf853d349d580bce283477d0f52789">example_obtain_session</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00321"></a>321</span><span class="doxyLineContent"><span class="doxyHighlight">           runtime,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00322"></a>322</span><span class="doxyLineContent"><span class="doxyHighlight">           pairing_code,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00323"></a>323</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"app_volume_management.onboarding-id.txt"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00324"></a>324</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"App Volume Peer"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00325"></a>325</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;onboarding_id,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00326"></a>326</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;session</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00327"></a>327</span><span class="doxyLineContent"><span class="doxyHighlight">       ) != 0</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00328"></a>328</span><span class="doxyLineContent"><span class="doxyHighlight">       || session == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00329"></a>329</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00330"></a>330</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a>(runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00331"></a>331</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00332"></a>332</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00333"></a>333</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 5;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00334"></a>334</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00335"></a>335</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_setup]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00336"></a>336</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00337"></a>337</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_create]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00338"></a>338</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumeinit">QarAppVolumeInit</a> volume_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gafda139e7c1a2424c1c58ebe3c07aa1ba">qar_app_volume_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00339"></a>339</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#adbd20eb46c7e9ef54b656102529b3b0c">common_name</a> = </span><span class="doxyHighlightStringLiteral">"tutorial-volume.examples.qaros"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00340"></a>340</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#adbb5e07d43c6c4458acfb4c7a20220bf">display_name</a> = </span><span class="doxyHighlightStringLiteral">"Tutorial Volume"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00341"></a>341</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00342"></a>342</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00343"></a>343</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a71fc823912cc40c74f798a97aecd77dc">size</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga9a06d2b56897aa2048f3651defb1c3d4">qar_app_volume_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00344"></a>344</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a71fc823912cc40c74f798a97aecd77dc">size</a>.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#aae5fa8eb081d2d741815132b89eb5837">width_meters</a> = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00345"></a>345</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a71fc823912cc40c74f798a97aecd77dc">size</a>.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a9a1934c1fc17783ffff15717e8821564">length_meters</a> = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00346"></a>346</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a71fc823912cc40c74f798a97aecd77dc">size</a>.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a4588bfc36f49bf06870b9ccc4e2532be">height_meters</a> = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00347"></a>347</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00348"></a>348</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> volume_id = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaa01cc36e4fc2dbbd70faf2c78d07d43f">qar_app_volume_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00349"></a>349</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> create_result = <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga76e9061c6e95e92787686a130604a140">qar_app_volumes_get_or_create</a>(session, &amp;volume_init, &amp;volume_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00350"></a>350</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_app_volumes_get_or_create"</span><span class="doxyHighlight">, create_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00351"></a>351</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(create_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00352"></a>352</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00353"></a>353</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Created app volume with id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00354"></a>354</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(volume_id.<a href="/api/qar-streaming-c/structs/qarappvolumeid/#a42729c99b572ee8e41cdc7e7a7ddd932">data</a>, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00355"></a>355</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00356"></a>356</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00357"></a>357</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00358"></a>358</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> volume_count = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00359"></a>359</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> count_result = <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gadf709eb3fd8918f941eb4759fbfb9f4b">qar_query_app_volumes_count</a>(session, &amp;volume_count);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00360"></a>360</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_app_volumes_count"</span><span class="doxyHighlight">, count_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00361"></a>361</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(count_result) &amp;&amp; volume_count &gt; 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00362"></a>362</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00363"></a>363</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a>** handles = (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a>**)calloc(volume_count, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a>*));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00364"></a>364</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(handles)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00365"></a>365</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00366"></a>366</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> written = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00367"></a>367</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> list_result = <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga93710b7daad904b3307807ed2eb9f6a8">qar_query_app_volumes</a>(session, handles, volume_count, &amp;written);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00368"></a>368</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_app_volumes"</span><span class="doxyHighlight">, list_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00369"></a>369</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(list_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00370"></a>370</span><span class="doxyLineContent"><span class="doxyHighlight">            {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00371"></a>371</span><span class="doxyLineContent"><span class="doxyHighlight">                </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; written; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00372"></a>372</span><span class="doxyLineContent"><span class="doxyHighlight">                {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00373"></a>373</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a530cd548ecb0ec991d68ca1ddc47de41">QarAppVolume</a>* handle = handles[i];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00374"></a>374</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!handle)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00375"></a>375</span><span class="doxyLineContent"><span class="doxyHighlight">                    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00376"></a>376</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightKeywordFlow">continue</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00377"></a>377</span><span class="doxyLineContent"><span class="doxyHighlight">                    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00378"></a>378</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00379"></a>379</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> </span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight"> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaa01cc36e4fc2dbbd70faf2c78d07d43f">qar_app_volume_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00380"></a>380</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarappvolumesize">QarAppVolumeSize</a> desc = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga9a06d2b56897aa2048f3651defb1c3d4">qar_app_volume_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00381"></a>381</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00382"></a>382</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> name[<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a48c60e5b127e7195a52790c8cbcde601">QAR_MAX_STRING_LENGTH</a>] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00383"></a>383</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00384"></a>384</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gaf404764ed9c2c6186ee524be5b10cf0d">qar_app_volume_get_id</a>(handle, &amp;</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00385"></a>385</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga96545524dc43b5038573f31a963694df">qar_app_volume_get_size</a>(handle, &amp;desc);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00386"></a>386</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga0b2414ac9f19997acac1e9ba039b6afc">qar_app_volume_get_pose</a>(handle, &amp;pose);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00387"></a>387</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gaf8c0c2d59ba67ee5edcea8c8556851c6">qar_app_volume_get_display_name</a>(handle, name, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(name));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00388"></a>388</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00389"></a>389</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span><span class="doxyHighlightStringLiteral">"- Volume id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00390"></a>390</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">.data, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00391"></a>391</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00392"></a>392</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">" name='%s' size(%.2f x %.2f x %.2f) position(%.2f, "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00393"></a>393</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">"%.2f, %.2f)\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00394"></a>394</span><span class="doxyLineContent"><span class="doxyHighlight">                        name,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00395"></a>395</span><span class="doxyLineContent"><span class="doxyHighlight">                        desc.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#aae5fa8eb081d2d741815132b89eb5837">width_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00396"></a>396</span><span class="doxyLineContent"><span class="doxyHighlight">                        desc.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a9a1934c1fc17783ffff15717e8821564">length_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00397"></a>397</span><span class="doxyLineContent"><span class="doxyHighlight">                        desc.<a href="/api/qar-streaming-c/structs/qarappvolumesize/#a4588bfc36f49bf06870b9ccc4e2532be">height_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00398"></a>398</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00399"></a>399</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00400"></a>400</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00401"></a>401</span><span class="doxyLineContent"><span class="doxyHighlight">                    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00402"></a>402</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00403"></a>403</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#gaf7c6b09bc22d0cc36286a4c96f08fdff">qar_app_volume_handle_destroy</a>(handle);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00404"></a>404</span><span class="doxyLineContent"><span class="doxyHighlight">                }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00405"></a>405</span><span class="doxyLineContent"><span class="doxyHighlight">            }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00406"></a>406</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00407"></a>407</span><span class="doxyLineContent"><span class="doxyHighlight">            free(handles);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00408"></a>408</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00409"></a>409</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00410"></a>410</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [app_create]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00411"></a>411</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00412"></a>412</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(create_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00413"></a>413</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00414"></a>414</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#a364b40d134d4fa7f72be8e50632f6bde">configure_gestures</a>(session, &amp;volume_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00415"></a>415</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#ac9cfc96dfe2facfa6ca4939058f48b5a">pin_and_unpin_world_anchor</a>(session, &amp;volume_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00416"></a>416</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00417"></a>417</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightComment">/* A room point 0.25 m above the volume's initial centre. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00418"></a>418</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarvector3">QarVector3</a> room_point = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaef1944758062eca5652c4f9d552a0e57">qar_vector3_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00419"></a>419</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00420"></a>420</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> + 0.25f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00421"></a>421</span><span class="doxyLineContent"><span class="doxyHighlight">        room_point.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00422"></a>422</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#af198e6c5f83fc195ef3c0c99fe4cd5d6">hit_test_room_point</a>(session, &amp;volume_id, room_point);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00423"></a>423</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00424"></a>424</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00425"></a>425</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* Destroying the handle leaves the session. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00426"></a>426</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-session/#gac86c6b3bc519f110d0d9db8adea0c5fa">qar_session_handle_destroy</a>(session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00427"></a>427</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a>(runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00428"></a>428</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> destroy_result = <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00429"></a>429</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_library_destroy"</span><span class="doxyHighlight">, destroy_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00430"></a>430</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00431"></a>431</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00432"></a>432</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>

</div>


<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
