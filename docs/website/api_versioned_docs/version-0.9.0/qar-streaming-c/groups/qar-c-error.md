---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/groups/qar-c-error
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - group

---

<div class="doxyPage">

# Error Handling



## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga2119fef4274d65aedf40dedc975835ca">qar_error_wrap_result</a> (QarResult inner_result, QarStatusCode new_code, const char *new_message)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Wrap an existing result with a new code/message for propagation. <a href="#ga2119fef4274d65aedf40dedc975835ca">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga396284af0a335cc81849a85de1dd37a6">qar_result_error</a> (QarStatusCode code, const char *message)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Construct an error result with code and optional message. <a href="#ga396284af0a335cc81849a85de1dd37a6">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga5e4a211cbea61f9161be837c8a90117b">qar_result_has_code</a> (QarResult result, QarStatusCode code)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Whether the result carries <span class="doxyComputerOutput">code</span>, as its own code or as the code of any cause it wraps (for example a paused stream inside QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME). <a href="#ga5e4a211cbea61f9161be837c8a90117b">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a> (QarResult result)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if result indicates failure. <a href="#ga926cc10284202d684d5cb1908969bd77">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static bool</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a> (QarResult result)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Check if result indicates success. <a href="#ga6a523b6292e03a1886c12d84fcf91826">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static void</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga1003f557ce0a665e120b1e869b35b7eb">qar_result_log_if_error</a> (QarResult result)</td>
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
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga9829ff37fa3ea111ed6fba452eaf4587">qar_result_message</a> (QarResult result, char *out_buffer, size_t buffer_size)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Copy a human-readable message into user buffer (NUL-terminated). <a href="#ga9829ff37fa3ea111ed6fba452eaf4587">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">static <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a></td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#ga8fdf4771365de6214ccfc4bc20c7b2a9">qar_result_success</a> (void)</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Construct a success result (code = QAR_STATUS_SUCCESS). <a href="#ga8fdf4771365de6214ccfc4bc20c7b2a9">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>

## Macro Definitions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">#define</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#gabea8c90eebe5815f6d388c520ac4d3e0">QAR_ASSUME_SUCCESS</a>(RESULT_EXPR)&nbsp;&nbsp;&nbsp;...</td>
</tr>
<tr class="doxyMemberIndexDescription">
<td class="doxyMemberIndexDescriptionLeft"></td>
<td class="doxyMemberIndexDescriptionRight">
<p>Log the error message if the result is an error. <a href="#gabea8c90eebe5815f6d388c520ac4d3e0">More...</a></p>
</td>
</tr>
<tr class="doxyMemberIndexSeparator">
<td class="doxyMemberIndexSeparator" colspan="2"></td>
</tr>

</table>


<div class="doxySectionDef">

## Functions

### qar\_error\_wrap\_result() {#ga2119fef4274d65aedf40dedc975835ca}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_error_wrap_result (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> inner_result, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#acaacdad8607932f63f8028df909f1675">QarStatusCode</a> new_code, const char * new_message)</td>
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

<p>Wrap an existing result with a new code/message for propagation.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02137">2137</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_error() {#ga396284af0a335cc81849a85de1dd37a6}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_result_error (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#acaacdad8607932f63f8028df909f1675">QarStatusCode</a> code, const char * message)</td>
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

<p>Construct an error result with code and optional message.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02125">2125</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_has\_code() {#ga5e4a211cbea61f9161be837c8a90117b}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_result_has_code (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> result, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#acaacdad8607932f63f8028df909f1675">QarStatusCode</a> code)</td>
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

<p>Whether the result carries <span class="doxyComputerOutput">code</span>, as its own code or as the code of any cause it wraps (for example a paused stream inside QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02133">2133</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_is\_error() {#ga926cc10284202d684d5cb1908969bd77}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_result_is_error (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> result)</td>
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

<p>Check if result indicates failure.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02129">2129</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_is\_success() {#ga6a523b6292e03a1886c12d84fcf91826}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">bool qar_result_is_success (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> result)</td>
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

<p>Check if result indicates success.</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02127">2127</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_log\_if\_error() {#ga1003f557ce0a665e120b1e869b35b7eb}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_result_log_if_error (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> result)</td>
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



<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02144">2144</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_message() {#ga9829ff37fa3ea111ed6fba452eaf4587}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">void qar_result_message (<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> result, char * out_buffer, size_t buffer_size)</td>
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

<p>Copy a human-readable message into user buffer (NUL-terminated).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02142">2142</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

### qar\_result\_success() {#ga8fdf4771365de6214ccfc4bc20c7b2a9}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">QarResult qar_result_success (void)</td>
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

<p>Construct a success result (code = QAR_STATUS_SUCCESS).</p>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02122">2122</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<div class="doxySectionDef">

## Macro Definitions

### QAR\_ASSUME\_SUCCESS {#gabea8c90eebe5815f6d388c520ac4d3e0}

<div class="doxyMemberItem">
<div class="doxyMemberProto">
<table class="doxyMemberLabels">
<tr class="doxyMemberLabels">
<td class="doxyMemberLabelsLeft">
<table class="doxyMemberName">
<tr>
<td class="doxyMemberName">#define QAR_ASSUME_SUCCESS(RESULT_EXPR)&nbsp;&nbsp;&nbsp;...</td>
</tr>
</table>
</td>
</tr>
</table>
</div>
<div class="doxyMemberDoc">

<p>Log the error message if the result is an error.</p>

<dl class="doxySectionUser">
<dt>Value</dt>
<dd>
<div class="doxyVerbatim">	do                                                                         \
	{                                                                          \
		<a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> qar_result_tmp__ = (RESULT_EXPR);                            \
		if(!<a href="#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(qar_result_tmp__))                           \
		{                                                                      \
			<a href="#ga1003f557ce0a665e120b1e869b35b7eb">qar_result_log_if_error</a>(qar_result_tmp__);                         \
			exit(EXIT_FAILURE);                                                \
		}                                                                      \
	}                                                                          \
	while(0)
</div>
</dd>
</dl>

<p>Definition at line <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#l02147">2147</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>.</p>

</div>
</div>

</div>

<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
