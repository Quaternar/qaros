---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/files/qar-streaming-c/examples/common-h
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - file

---

<div class="doxyPage">

# `common.h` File

<p>Common export/import and extern "C" macros for the C API. <a href="#details">More...</a></p>

## Included Headers

<div class="doxyIncludesList">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>"
#include &lt;inttypes.h&gt;
#include &lt;stdbool.h&gt;
#include &lt;stdio.h&gt;
#include &lt;stdlib.h&gt;
#include &lt;string.h&gt;
</div>

## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static int</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a76cf853d349d580bce283477d0f52789">example_obtain_session</a> (QarRuntime *runtime, const char *pairing_code, const char *id_file_path, const char *display_name, QarOnboardingId *out_onboarding_id, QarSession **out_session)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[example_onboarding_persistence] <a href="#a76cf853d349d580bce283477d0f52789">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static const char *</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#aefc887c8f5c861b4e3949555aedf21c7">get_dir_from_path</a> (const char *path, char *out_dir, size_t out_sz)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[example_obtain_session] <a href="#aefc887c8f5c861b4e3949555aedf21c7">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a3aa3081ae8027e460338690428b2175e">load_onboarding_id</a> (const char *path, QarOnboardingId *out_id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>[example_onboarding_persistence] <a href="#a3aa3081ae8027e460338690428b2175e">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a> (const char *label, QarResult r)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Log a <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> with a label to stdout. <a href="#a0078bc3763ae2e11c1cd28a58059f73c">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a> (const uint8_t *data, size_t len)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Print a fixed-size identifier as hex bytes separated by colons. <a href="#a4d02de32cbe028f4e7c5011362b1f775">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a17e955cb6bbce3a2e8ae85c627c60e61">save_onboarding_id</a> (const char *path, const QarOnboardingId *id)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Persist an onboarding id (UUID text) so the next run can rejoin. <a href="#a17e955cb6bbce3a2e8ae85c627c60e61">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Description {#details}

<p>Common export/import and extern "C" macros for the C API.</p>

<div class="doxySectionDef">

## Functions

### example\_obtain\_session() {#a76cf853d349d580bce283477d0f52789}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">int example_obtain_session (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a> * runtime, const char * pairing_code, const char * id_file_path, const char * display_name, <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> * out_onboarding_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a> ** out_session)</td>
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

<p>[example_onboarding_persistence]</p>


<p>[example_obtain_session]</p>


<p>Obtain a session the canonical way: rejoin with the persisted onboarding id when one exists, otherwise onboard with the pairing code currently shown on the hub's onboarding screen.</p>


<p>Returns 0 on success; on success *out_session and *out_onboarding_id are set and the id has been persisted to id_file_path for the next run.</p>


<p>Definition at line <a href="#l00091">91</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

### get\_dir\_from\_path() {#aefc887c8f5c861b4e3949555aedf21c7}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">const char * get_dir_from_path (const char * path, char * out_dir, size_t out_sz)</td>
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

<p>[example_obtain_session]</p>


<p>Extract directory portion from a path (both '/' and '\' on Windows).</p>


<p>Definition at line <a href="#l00169">169</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

### load\_onboarding\_id() {#a3aa3081ae8027e460338690428b2175e}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool load_onboarding_id (const char * path, <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> * out_id)</td>
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

<p>[example_onboarding_persistence]</p>


<p>Load a persisted onboarding id (UUID text) from a small state file. Returns false when the file does not exist yet (first run).</p>


<p>Definition at line <a href="#l00043">43</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

### log\_result() {#a0078bc3763ae2e11c1cd28a58059f73c}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void log_result (const char * label, <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> r)</td>
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

<p>Log a <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> with a label to stdout.</p>

<p>Definition at line <a href="#l00025">25</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

### print\_hex\_id() {#a4d02de32cbe028f4e7c5011362b1f775}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void print_hex_id (const uint8_t * data, size_t len)</td>
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

<p>Print a fixed-size identifier as hex bytes separated by colons.</p>

<p>Definition at line <a href="#l00011">11</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

### save\_onboarding\_id() {#a17e955cb6bbce3a2e8ae85c627c60e61}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void save_onboarding_id (const char * path, const <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> * id)</td>
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

<p>Persist an onboarding id (UUID text) so the next run can rejoin.</p>

<p>Definition at line <a href="#l00064">64</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>.</p>

</div>
</div>

</div>

## File Listing

The file content with the documentation metadata removed is:

<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00001"></a>1</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#pragma once</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00002"></a>2</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00003"></a>3</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;inttypes.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00004"></a>4</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;stdbool.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00005"></a>5</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;stdio.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00006"></a>6</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;stdlib.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00007"></a>7</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;string.h&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00008"></a>8</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00009"></a>9</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Print a fixed-size identifier as hex bytes separated by colons. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00010"></a>10</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00011"></a><a href="#a4d02de32cbe028f4e7c5011362b1f775">11</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> uint8_t* data, </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> len)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00012"></a>12</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00013"></a>13</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; len; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00014"></a>14</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00015"></a>15</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"%02X"</span><span class="doxyHighlight">, (</span><span class="doxyHighlightKeywordType">unsigned</span><span class="doxyHighlight">)data[i]);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00016"></a>16</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(i + 1 &lt; len)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00017"></a>17</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00018"></a>18</span><span class="doxyLineContent"><span class="doxyHighlight">            printf(</span><span class="doxyHighlightStringLiteral">":"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00019"></a>19</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00020"></a>20</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00021"></a>21</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00022"></a>22</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00023"></a>23</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Log a \ref QarResult with a label to stdout. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00024"></a>24</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00025"></a><a href="#a0078bc3763ae2e11c1cd28a58059f73c">25</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* label, <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> r)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00026"></a>26</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00027"></a>27</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(r))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00028"></a>28</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00029"></a>29</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"%s: SUCCESS (code=%d)\n"</span><span class="doxyHighlight">, label, (</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight">)r.<a href="/api/qar-streaming-c/structs/qarresult/#aed158a5247abc4cc3600d6a7878f335b">code</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00030"></a>30</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00031"></a>31</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">else</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00032"></a>32</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00033"></a>33</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> msg[256] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00034"></a>34</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-error/#ga9829ff37fa3ea111ed6fba452eaf4587">qar_result_message</a>(r, msg, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(msg));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00035"></a>35</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"%s: ERROR (code=%d) msg='%s'\n"</span><span class="doxyHighlight">, label, (</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight">)r.<a href="/api/qar-streaming-c/structs/qarresult/#aed158a5247abc4cc3600d6a7878f335b">code</a>, msg);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00036"></a>36</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00037"></a>37</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00038"></a>38</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00039"></a>39</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [example_onboarding_persistence]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00040"></a>40</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Load a persisted onboarding id (UUID text) from a small state file.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00041"></a>41</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  Returns false when the file does not exist yet (first run). */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00042"></a>42</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">bool</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00043"></a><a href="#a3aa3081ae8027e460338690428b2175e">43</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a3aa3081ae8027e460338690428b2175e">load_onboarding_id</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* path, <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a>* out_id)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00044"></a>44</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00045"></a>45</span><span class="doxyLineContent"><span class="doxyHighlight">    FILE* file = fopen(path, </span><span class="doxyHighlightStringLiteral">"r"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00046"></a>46</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!file)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00047"></a>47</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00048"></a>48</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">false</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00049"></a>49</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00050"></a>50</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> text[64] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00051"></a>51</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">bool</span><span class="doxyHighlight"> read_ok = fgets(text, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(text), file) != NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00052"></a>52</span><span class="doxyLineContent"><span class="doxyHighlight">    fclose(file);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00053"></a>53</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!read_ok)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00054"></a>54</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00055"></a>55</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">false</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00056"></a>56</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00057"></a>57</span><span class="doxyLineContent"><span class="doxyHighlight">    text[strcspn(text, </span><span class="doxyHighlightStringLiteral">"\r\n"</span><span class="doxyHighlight">)] = </span><span class="doxyHighlightCharLiteral">'\0'</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00058"></a>58</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> parse_result = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga4b7abdce2414a553d0ab58b66d6bc827">qar_uuid_from_string</a>(text, out_id-&gt;<a href="/api/qar-streaming-c/structs/qaronboardingid/#a83beeac6904044d13834636b36783917">data</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00059"></a>59</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(parse_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00060"></a>60</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00061"></a>61</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00062"></a>62</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Persist an onboarding id (UUID text) so the next run can rejoin. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00063"></a>63</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00064"></a><a href="#a17e955cb6bbce3a2e8ae85c627c60e61">64</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a17e955cb6bbce3a2e8ae85c627c60e61">save_onboarding_id</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* path, </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a>* </span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00065"></a>65</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00066"></a>66</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> text[64] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00067"></a>67</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(<a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga90f35de8b39d02531147287bed956aed">qar_uuid_to_string</a>(id-&gt;<a href="/api/qar-streaming-c/structs/qaronboardingid/#a83beeac6904044d13834636b36783917">data</a>, text, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(text))))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00068"></a>68</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00069"></a>69</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Failed to serialize onboarding id\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00070"></a>70</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00071"></a>71</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00072"></a>72</span><span class="doxyLineContent"><span class="doxyHighlight">    FILE* file = fopen(path, </span><span class="doxyHighlightStringLiteral">"w"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00073"></a>73</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!file)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00074"></a>74</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00075"></a>75</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Failed to persist onboarding id to '%s'\n"</span><span class="doxyHighlight">, path);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00076"></a>76</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00077"></a>77</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00078"></a>78</span><span class="doxyLineContent"><span class="doxyHighlight">    fprintf(file, </span><span class="doxyHighlightStringLiteral">"%s\n"</span><span class="doxyHighlight">, text);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00079"></a>79</span><span class="doxyLineContent"><span class="doxyHighlight">    fclose(file);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00080"></a>80</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00081"></a>81</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [example_onboarding_persistence]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00082"></a>82</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00083"></a>83</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [example_obtain_session]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00084"></a>84</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Obtain a session the canonical way: rejoin with the persisted</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00085"></a>85</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  onboarding id when one exists, otherwise onboard with the pairing code</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00086"></a>86</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  currently shown on the hub's onboarding screen.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00087"></a>87</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00088"></a>88</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  Returns 0 on success; on success *out_session and *out_onboarding_id are</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00089"></a>89</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  set and the id has been persisted to id_file_path for the next run. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00090"></a>90</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">int</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00091"></a><a href="#a76cf853d349d580bce283477d0f52789">91</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a76cf853d349d580bce283477d0f52789">example_obtain_session</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00092"></a>92</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a>* runtime,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00093"></a>93</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* pairing_code, </span><span class="doxyHighlightComment">/* may be NULL when a persisted id exists */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00094"></a>94</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* id_file_path,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00095"></a>95</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* display_name,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00096"></a>96</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a>* out_onboarding_id,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00097"></a>97</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>** out_session</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00098"></a>98</span><span class="doxyLineContent"><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00099"></a>99</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00100"></a>100</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarpeerpresentation">QarPeerPresentation</a> presentation = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga9fb2723afdee067c6e3b40315172fad0">qar_peer_presentation_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00101"></a>101</span><span class="doxyLineContent"><span class="doxyHighlight">    presentation.<a href="/api/qar-streaming-c/structs/qarpeerpresentation/#a7b78144161e112a93b4f624721eb5a14">display_name</a> = display_name;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00102"></a>102</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00103"></a>103</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* 1 Silent rejoin with the persisted identity — the normal path. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00104"></a>104</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> saved_id = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga0f002137f2f3d3e5d7b27cc52742cb86">qar_onboarding_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00105"></a>105</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="#a3aa3081ae8027e460338690428b2175e">load_onboarding_id</a>(id_file_path, &amp;saved_id))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00106"></a>106</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00107"></a>107</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarrejoininit">QarRejoinInit</a> rejoin_init = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#gad962a206d6a95e765318e58fa314327e">qar_rejoin_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00108"></a>108</span><span class="doxyLineContent"><span class="doxyHighlight">        rejoin_init.<a href="/api/qar-streaming-c/structs/qarrejoininit/#acc54fdffa3efab6fbf7b1b61520d8f47">onboarding_id</a> = saved_id;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00109"></a>109</span><span class="doxyLineContent"><span class="doxyHighlight">        rejoin_init.<a href="/api/qar-streaming-c/structs/qarrejoininit/#a7b6ea0a5e7ded6f1471aeee147fd07c8">presentation</a> = presentation;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00110"></a>110</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00111"></a>111</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> rejoin_result =</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00112"></a>112</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga5603e1f9a493232af68bba3ce0527edb">qar_runtime_rejoin</a>(runtime, &amp;rejoin_init, NULL, NULL, NULL, out_session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00113"></a>113</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_runtime_rejoin"</span><span class="doxyHighlight">, rejoin_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00114"></a>114</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(rejoin_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00115"></a>115</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00116"></a>116</span><span class="doxyLineContent"><span class="doxyHighlight">            *out_onboarding_id = saved_id;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00117"></a>117</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00118"></a>118</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00119"></a>119</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!<a href="/api/qar-streaming-c/groups/qar-c-error/#ga5e4a211cbea61f9161be837c8a90117b">qar_result_has_code</a>(rejoin_result, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#acaacdad8607932f63f8028df909f1675adca321f83611867921e99c16ce2dd322">QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND</a>))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00120"></a>120</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00121"></a>121</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1; </span><span class="doxyHighlightComment">/* real error (hub unreachable, ...) */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00122"></a>122</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00123"></a>123</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightComment">/* No persisted slot for the id -&gt; fall through to onboarding. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00124"></a>124</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00125"></a>125</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00126"></a>126</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* 2 First connect: onboard with the code shown by the hub. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00127"></a>127</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!pairing_code)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00128"></a>128</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00129"></a>129</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00130"></a>130</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightStringLiteral">"No persisted onboarding id in '%s'. Pass the pairing code shown "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00131"></a>131</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightStringLiteral">"on the hub's onboarding screen.\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00132"></a>132</span><span class="doxyLineContent"><span class="doxyHighlight">            id_file_path</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00133"></a>133</span><span class="doxyLineContent"><span class="doxyHighlight">        );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00134"></a>134</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00135"></a>135</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00136"></a>136</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00137"></a>137</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardinit">QarOnboardInit</a> onboard_init = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga2a31c9abdeb3fc92c413a97ce6dce82c">qar_onboard_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00138"></a>138</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardcodeext">QarOnboardCodeExt</a> onboard_code = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga854c392d477eabbd2876e6f1d6d8f16c">qar_onboard_code_ext_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00139"></a>139</span><span class="doxyLineContent"><span class="doxyHighlight">    onboard_init.<a href="/api/qar-streaming-c/structs/qaronboardinit/#ac055d692343c02999988c5e33f0257b7">presentation</a> = presentation;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00140"></a>140</span><span class="doxyLineContent"><span class="doxyHighlight">    onboard_code.<a href="/api/qar-streaming-c/structs/qaronboardcodeext/#aa3f8fbccf2157ee4af5b3ccd3d277896">code</a> = pairing_code;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00141"></a>141</span><span class="doxyLineContent"><span class="doxyHighlight">    onboard_init.<a href="/api/qar-streaming-c/structs/qaronboardinit/#a8e50f1404dcbac0e16dc6b0f0e447714">header</a>.<a href="/api/qar-streaming-c/structs/qarstructureheader/#a10911ac494fae9cb85f835c9097c8c09">next</a> = &amp;onboard_code.<a href="/api/qar-streaming-c/structs/qaronboardcodeext/#a2db9b1cba35d872b566c9d38e1dffab7">header</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00142"></a>142</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00143"></a>143</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> new_id = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga0f002137f2f3d3e5d7b27cc52742cb86">qar_onboarding_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00144"></a>144</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> onboard_result =</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00145"></a>145</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#gaeb44a81788acdf67c591ecc3f660189a">qar_runtime_onboard</a>(runtime, &amp;onboard_init, NULL, NULL, NULL, &amp;new_id, out_session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00146"></a>146</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_runtime_onboard"</span><span class="doxyHighlight">, onboard_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00147"></a>147</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(onboard_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00148"></a>148</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00149"></a>149</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga5e4a211cbea61f9161be837c8a90117b">qar_result_has_code</a>(onboard_result, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#acaacdad8607932f63f8028df909f1675a7bf9cb273af9b03e266d65b9d9105cba">QAR_STATUS_PAKE_ERROR</a>))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00150"></a>150</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00151"></a>151</span><span class="doxyLineContent"><span class="doxyHighlight">            printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00152"></a>152</span><span class="doxyLineContent"><span class="doxyHighlight">                </span><span class="doxyHighlightStringLiteral">"Pairing code rejected or expired — read the current code "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00153"></a>153</span><span class="doxyLineContent"><span class="doxyHighlight">                </span><span class="doxyHighlightStringLiteral">"off the hub screen and retry.\n"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00154"></a>154</span><span class="doxyLineContent"><span class="doxyHighlight">            );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00155"></a>155</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00156"></a>156</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00157"></a>157</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00158"></a>158</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00159"></a>159</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightComment">/* 3 Persist the id — it is the ticket for every later rejoin/forget. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00160"></a>160</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="#a17e955cb6bbce3a2e8ae85c627c60e61">save_onboarding_id</a>(id_file_path, &amp;new_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00161"></a>161</span><span class="doxyLineContent"><span class="doxyHighlight">    *out_onboarding_id = new_id;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00162"></a>162</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00163"></a>163</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00164"></a>164</span><span class="doxyLineContent"><span class="doxyHighlightComment">//! [example_obtain_session]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00165"></a>165</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00166"></a>166</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Extract directory portion from a path (both '/' and '\\' on Windows).</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00167"></a>167</span><span class="doxyLineContent"><span class="doxyHighlightComment"> */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00168"></a>168</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">*</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00169"></a><a href="#aefc887c8f5c861b4e3949555aedf21c7">169</a></span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#aefc887c8f5c861b4e3949555aedf21c7">get_dir_from_path</a>(</span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* path, </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* out_dir, </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> out_sz)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00170"></a>170</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00171"></a>171</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!path || !out_dir || out_sz == 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00172"></a>172</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00173"></a>173</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00174"></a>174</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00175"></a>175</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> n = strlen(path);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00176"></a>176</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = n;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00177"></a>177</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">while</span><span class="doxyHighlight">(i &gt; 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00178"></a>178</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00179"></a>179</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> c = path[i - 1];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00180"></a>180</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(c == </span><span class="doxyHighlightCharLiteral">'/'</span><span class="doxyHighlight"> || c == </span><span class="doxyHighlightCharLiteral">'\\'</span><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00181"></a>181</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00182"></a>182</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">break</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00183"></a>183</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00184"></a>184</span><span class="doxyLineContent"><span class="doxyHighlight">        --i;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00185"></a>185</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00186"></a>186</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(i == 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00187"></a>187</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00188"></a>188</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightComment">// No separator found; use current directory</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00189"></a>189</span><span class="doxyLineContent"><span class="doxyHighlight">        snprintf(out_dir, out_sz, </span><span class="doxyHighlightStringLiteral">"."</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00190"></a>190</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> out_dir;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00191"></a>191</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00192"></a>192</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> dir_len = (i &gt; out_sz - 1) ? (out_sz - 1) : i;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00193"></a>193</span><span class="doxyLineContent"><span class="doxyHighlight">    memcpy(out_dir, path, dir_len);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00194"></a>194</span><span class="doxyLineContent"><span class="doxyHighlight">    out_dir[dir_len] = </span><span class="doxyHighlightCharLiteral">'\0'</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00195"></a>195</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> out_dir;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00196"></a>196</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>

</div>


<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
