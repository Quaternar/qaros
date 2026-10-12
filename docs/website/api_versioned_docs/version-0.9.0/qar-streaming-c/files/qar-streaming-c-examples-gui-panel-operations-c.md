---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/files/qar-streaming-c/examples/gui-panel-operations-c
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - file

---

<div class="doxyPage">

# `gui_panel_operations.c` File

<p>Demonstrates creating, updating, and enumerating GUI panels. <a href="#details">More...</a></p>

## Included Headers

<div class="doxyIncludesList">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>"
#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>&gt;
</div>

## Functions Index

<table class="doxyMembersIndex">

<tr class="doxyMemberIndexItem">
<td class="doxyMemberIndexItemType" align="left" valign="top">int</td>
<td class="doxyMemberIndexItemName" align="left" valign="top"><a href="#a3c04138a5bfe5d72780bb7e82a18e627">main</a> (int argc, char **argv)</td>
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

</table>

## Description {#details}

<p>Demonstrates creating, updating, and enumerating GUI panels.</p>

<div class="doxySectionDef">

## Functions

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



<p>Definition at line <a href="#l00067">67</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/gui-panel-operations-c">gui_panel_operations.c</a>.</p>

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



<p>Definition at line <a href="#l00052">52</a> of file <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/gui-panel-operations-c">gui_panel_operations.c</a>.</p>

</div>
</div>

</div>

## File Listing

The file content with the documentation metadata removed is:

<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00001"></a>1</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \file gui_panel_operations.c</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00002"></a>2</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  \brief Demonstrates creating, updating, and enumerating GUI panels.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00003"></a>3</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *  \example gui_panel_operations.c</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00004"></a>4</span><span class="doxyLineContent"><span class="doxyHighlightComment"> */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00005"></a>5</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00006"></a>6</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \page qar_c_tutorial_gui_panels GUI Panel Operations</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00007"></a>7</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \tableofcontents</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00008"></a>8</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00009"></a>9</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_overview Overview</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00010"></a>10</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Obtain a session (rejoin, or onboard with the hub pairing code)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00011"></a>11</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Get-or-create a GUI panel by its stable common name</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00012"></a>12</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Adjust pose, size, state, and navigate to content</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00013"></a>13</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Enumerate GUI panels to inspect current values</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00014"></a>14</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00015"></a>15</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_prereq Prerequisites</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00016"></a>16</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - Complete the \ref qar_c_tutorial_app_volumes tutorial</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00017"></a>17</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * - A running QarOS hub (for the first run, read the pairing code off its</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00018"></a>18</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *   onboarding screen)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00019"></a>19</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00020"></a>20</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_build Build and Run</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00021"></a>21</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \code{.bash}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00022"></a>22</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * cmake --build --preset x64-windows-debug --target gui_panel_operations</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00023"></a>23</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * ./build/x64-windows/Debug/gui_panel_operations.exe</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00024"></a>24</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * &lt;path-to-qar-streaming-c.dll&gt; [runtime-dir] [pairing-code]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00025"></a>25</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \endcode</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00026"></a>26</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00027"></a>27</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_args Parse Arguments</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00028"></a>28</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet gui_panel_operations.c gui_args</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00029"></a>29</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00030"></a>30</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_setup Set Up the Session</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00031"></a>31</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * The session is obtained with the shared rejoin-or-onboard helper described</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00032"></a>32</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * in the \ref qar_c_tutorial_onboarding tutorial.</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00033"></a>33</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet gui_panel_operations.c gui_setup</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00034"></a>34</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00035"></a>35</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_create Create and Update the Panel</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00036"></a>36</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet gui_panel_operations.c gui_create</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00037"></a>37</span><span class="doxyLineContent"><span class="doxyHighlightComment"> *</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00038"></a>38</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \section gui_panels_list Enumerate GUI Panels</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00039"></a>39</span><span class="doxyLineContent"><span class="doxyHighlightComment"> * \snippet gui_panel_operations.c gui_list</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00040"></a>40</span><span class="doxyLineContent"><span class="doxyHighlightComment"> */</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00041"></a>41</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00042"></a>42</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include "<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h">common.h</a>"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00043"></a>43</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00044"></a>44</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#ifndef QAR_ENABLE_DYNAMIC_LOADING</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00045"></a>45</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#define QAR_ENABLE_DYNAMIC_LOADING</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00046"></a>46</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#endif</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00047"></a>47</span><span class="doxyLineContent"><span class="doxyHighlightPreprocessor">#include &lt;<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h">qar_streaming.h</a>&gt;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00048"></a>48</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00049"></a>49</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#af720efda6e2714bd1d8a447ba7243b08">QAR_IMPLEMENT_DYNAMIC_LOADING</a>()</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00050"></a>50</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00051"></a>51</span><span class="doxyLineContent"><span class="doxyHighlight">static </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00052"></a>52</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a7bf4a3818b91f144183c65e1d555b627">print_usage</a>(const </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* program_name)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00053"></a>53</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00054"></a>54</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* name = program_name ? program_name : </span><span class="doxyHighlightStringLiteral">"gui_panel_operations"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00055"></a>55</span><span class="doxyLineContent"><span class="doxyHighlight">    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00056"></a>56</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"Usage: %s &lt;path-to-qar-streaming-c-library&gt; [runtime-binaries-dir] "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00057"></a>57</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"[pairing-code]\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00058"></a>58</span><span class="doxyLineContent"><span class="doxyHighlight">        name</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00059"></a>59</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00060"></a>60</span><span class="doxyLineContent"><span class="doxyHighlight">    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00061"></a>61</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"The pairing code is required on the first run only; later runs "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00062"></a>62</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"rejoin with the persisted onboarding id.\n"</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00063"></a>63</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00064"></a>64</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00065"></a>65</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00066"></a>66</span><span class="doxyLineContent"><span class="doxyHighlightKeywordType">int</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00067"></a>67</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="#a3c04138a5bfe5d72780bb7e82a18e627">main</a>(</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight"> argc, </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">** argv)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00068"></a>68</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00069"></a>69</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_args]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00070"></a>70</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &lt; 2)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00071"></a>71</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00072"></a>72</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="#a7bf4a3818b91f144183c65e1d555b627">print_usage</a>(argv[0]);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00073"></a>73</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00074"></a>74</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00075"></a>75</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00076"></a>76</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* library_path = argv[1];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00077"></a>77</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* runtime_dir = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00078"></a>78</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> runtime_dir_buffer[1024] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00079"></a>79</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00080"></a>80</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &gt;= 3)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00081"></a>81</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00082"></a>82</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = argv[2];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00083"></a>83</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00084"></a>84</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">else</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#aefc887c8f5c861b4e3949555aedf21c7">get_dir_from_path</a>(library_path, runtime_dir_buffer, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(runtime_dir_buffer)))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00085"></a>85</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00086"></a>86</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = runtime_dir_buffer;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00087"></a>87</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00088"></a>88</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00089"></a>89</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* pairing_code = (argc &gt;= 4) ? argv[3] : NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00090"></a>90</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_args]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00091"></a>91</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00092"></a>92</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_setup]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00093"></a>93</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!<a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga2f183f2df1f2f9db71fe520d154b1606">qar_library_load</a>(library_path))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00094"></a>94</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00095"></a>95</span><span class="doxyLineContent"><span class="doxyHighlight">        fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"Failed to load '%s'.\n"</span><span class="doxyHighlight">, library_path);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00096"></a>96</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 2;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00097"></a>97</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00098"></a>98</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00099"></a>99</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarlibraryinit">QarLibraryInit</a> library_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac75cdcedfeb7cbbfae40405d5821e5b0">qar_library_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00100"></a>100</span><span class="doxyLineContent"><span class="doxyHighlight">    library_init.<a href="/api/qar-streaming-c/structs/qarlibraryinit/#a4a4126bb11230817ddd769e5c2b41df1">enable_console_logging</a> = </span><span class="doxyHighlightKeyword">true</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00101"></a>101</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> library_result = <a href="/api/qar-streaming-c/groups/qar-c-library/#ga7cc03dea04e7fd44d19fe459363d1df0">qar_library_init</a>(&amp;library_init);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00102"></a>102</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(library_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00103"></a>103</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00104"></a>104</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_library_init"</span><span class="doxyHighlight">, library_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00105"></a>105</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00106"></a>106</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 3;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00107"></a>107</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00108"></a>108</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00109"></a>109</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a>* runtime = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00110"></a>110</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarruntimeinit">QarRuntimeInit</a> runtime_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga356269ae6fe65b091aa705537030ed98">qar_runtime_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00111"></a>111</span><span class="doxyLineContent"><span class="doxyHighlight">    runtime_init.<a href="/api/qar-streaming-c/structs/qarruntimeinit/#aa3da4381535f98bb43257058308d9f86">runtime_binaries_folder_path</a> = runtime_dir;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00112"></a>112</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> runtime_result = <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga6a057004f666fdb8632dded376d634c3">qar_runtime_create</a>(&amp;runtime_init, &amp;runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00113"></a>113</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(runtime_result) || runtime == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00114"></a>114</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00115"></a>115</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_runtime_create"</span><span class="doxyHighlight">, runtime_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00116"></a>116</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00117"></a>117</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00118"></a>118</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 4;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00119"></a>119</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00120"></a>120</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00121"></a>121</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> onboarding_id = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga0f002137f2f3d3e5d7b27cc52742cb86">qar_onboarding_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00122"></a>122</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00123"></a>123</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a76cf853d349d580bce283477d0f52789">example_obtain_session</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00124"></a>124</span><span class="doxyLineContent"><span class="doxyHighlight">           runtime,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00125"></a>125</span><span class="doxyLineContent"><span class="doxyHighlight">           pairing_code,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00126"></a>126</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"gui_panel_operations.onboarding-id.txt"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00127"></a>127</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"GUI Panel Peer"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00128"></a>128</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;onboarding_id,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00129"></a>129</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;session</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00130"></a>130</span><span class="doxyLineContent"><span class="doxyHighlight">       ) != 0</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00131"></a>131</span><span class="doxyLineContent"><span class="doxyHighlight">       || session == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00132"></a>132</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00133"></a>133</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a>(runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00134"></a>134</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00135"></a>135</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00136"></a>136</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 5;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00137"></a>137</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00138"></a>138</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_setup]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00139"></a>139</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00140"></a>140</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_create]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00141"></a>141</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelinit">QarGuiPanelInit</a> panel_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga18a71cf97c53dbf7ecb2506d469f2221">qar_gui_panel_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00142"></a>142</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a32f6fbc23514dc738fd2626dd55858f8">common_name</a> = </span><span class="doxyHighlightStringLiteral">"tutorial-panel.examples.qaros"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00143"></a>143</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a7e7ea1e4b461b0b1e741499a9c9c7869">display_name</a> = </span><span class="doxyHighlightStringLiteral">"Tutorial Panel"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00144"></a>144</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00145"></a>145</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = 0.5f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00146"></a>146</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = 1.5f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00147"></a>147</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00148"></a>148</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00149"></a>149</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a>.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a> = 1.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00150"></a>150</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a>.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a> = 0.7f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00151"></a>151</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00152"></a>152</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> panel_id = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga1119ac424b5f2d87acf2e423d565df51">qar_gui_panel_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00153"></a>153</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> create_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#gae564de938ca8c589d2a533dc7df71496">qar_gui_panels_get_or_create</a>(session, &amp;panel_init, &amp;panel_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00154"></a>154</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_gui_panels_get_or_create"</span><span class="doxyHighlight">, create_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00155"></a>155</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(create_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00156"></a>156</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00157"></a>157</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Created GUI panel with id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00158"></a>158</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(panel_id.<a href="/api/qar-streaming-c/structs/qarguipanelid/#aadfd0f935c84663124e8a52caf0c071d">data</a>, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00159"></a>159</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00160"></a>160</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00161"></a>161</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00162"></a>162</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> new_pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00163"></a>163</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = 0.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00164"></a>164</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = 1.6f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00165"></a>165</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00166"></a>166</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00167"></a>167</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_gui_panels_update_pose"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga88a8b5119ef11a2dc70d8b054c2ea95f">qar_gui_panels_update_pose</a>(session, &amp;panel_id, &amp;new_pose)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00168"></a>168</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00169"></a>169</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00170"></a>170</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> new_size = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00171"></a>171</span><span class="doxyLineContent"><span class="doxyHighlight">    new_size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a> = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00172"></a>172</span><span class="doxyLineContent"><span class="doxyHighlight">    new_size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a> = 0.6f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00173"></a>173</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00174"></a>174</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_gui_panels_change_size"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#gae5f7577e17d42493143bd37b31971c9f">qar_gui_panels_change_size</a>(session, &amp;panel_id, &amp;new_size)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00175"></a>175</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00176"></a>176</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00177"></a>177</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-error/#gabea8c90eebe5815f6d388c520ac4d3e0">QAR_ASSUME_SUCCESS</a>(<a href="/api/qar-streaming-c/groups/qar-c-gui/#gadb86352d6421388d3979163ac2541516">qar_gui_panels_set_state</a>(session, &amp;panel_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6ba9e12b2178b4be52f8af3e7d274e39969">QAR_GUI_PANEL_STATE_MINIMIZED</a>));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00178"></a>178</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00179"></a>179</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-error/#gabea8c90eebe5815f6d388c520ac4d3e0">QAR_ASSUME_SUCCESS</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00180"></a>180</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga52f5db0bab40989e7aa4c57a2f280dbc">qar_gui_panels_navigate_to_uri</a>(session, &amp;panel_id, </span><span class="doxyHighlightStringLiteral">"https://example.com/tutorial"</span><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00181"></a>181</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00182"></a>182</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_create]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00183"></a>183</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00184"></a>184</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_list]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00185"></a>185</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> panel_count = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00186"></a>186</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> count_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga2efbe85e332acbdae9d4069b7a217ebc">qar_query_gui_panels_count</a>(session, &amp;panel_count);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00187"></a>187</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_gui_panels_count"</span><span class="doxyHighlight">, count_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00188"></a>188</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(count_result) &amp;&amp; panel_count &gt; 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00189"></a>189</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00190"></a>190</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>** handles = (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>**)calloc(panel_count, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>*));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00191"></a>191</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(handles)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00192"></a>192</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00193"></a>193</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> written = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00194"></a>194</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> list_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga49e5b5fe0aac0457c7649b15f68cc737">qar_query_gui_panels</a>(session, handles, panel_count, &amp;written);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00195"></a>195</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_gui_panels"</span><span class="doxyHighlight">, list_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00196"></a>196</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(list_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00197"></a>197</span><span class="doxyLineContent"><span class="doxyHighlight">            {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00198"></a>198</span><span class="doxyLineContent"><span class="doxyHighlight">                </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; written; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00199"></a>199</span><span class="doxyLineContent"><span class="doxyHighlight">                {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00200"></a>200</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>* handle = handles[i];</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00201"></a>201</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!handle)</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00202"></a>202</span><span class="doxyLineContent"><span class="doxyHighlight">                    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00203"></a>203</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightKeywordFlow">continue</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00204"></a>204</span><span class="doxyLineContent"><span class="doxyHighlight">                    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00205"></a>205</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00206"></a>206</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> </span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight"> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga1119ac424b5f2d87acf2e423d565df51">qar_gui_panel_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00207"></a>207</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> size = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00208"></a>208</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00209"></a>209</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6b">QarGuiPanelState</a> state = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6bae47d8fbd35a55e0841f94797fdacce8f">QAR_GUI_PANEL_STATE_VISIBLE</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00210"></a>210</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> name[<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a48c60e5b127e7195a52790c8cbcde601">QAR_MAX_STRING_LENGTH</a>] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00211"></a>211</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00212"></a>212</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#gacc1f3b175faf098aab42a4e7e4be4c88">qar_gui_panel_get_id</a>(handle, &amp;</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00213"></a>213</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga8ecec7a810bdfa32c313eb6e7577772e">qar_gui_panel_get_display_name</a>(handle, name, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(name));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00214"></a>214</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga95a05d8ba3e6aaabaed399d2531307d0">qar_gui_panel_get_size</a>(handle, &amp;size);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00215"></a>215</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga27f046c6022215093fa6305ee1aa95a1">qar_gui_panel_get_pose</a>(handle, &amp;pose);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00216"></a>216</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga21a6da5242fbf890ba915c6252a54823">qar_gui_panel_get_state</a>(handle, &amp;state);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00217"></a>217</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00218"></a>218</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span><span class="doxyHighlightStringLiteral">"- Panel id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00219"></a>219</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">.data, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00220"></a>220</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00221"></a>221</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">" name='%s' size(%.2f x %.2f) position(%.2f, %.2f, "</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00222"></a>222</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">"%.2f) state=%d\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00223"></a>223</span><span class="doxyLineContent"><span class="doxyHighlight">                        name,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00224"></a>224</span><span class="doxyLineContent"><span class="doxyHighlight">                        size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00225"></a>225</span><span class="doxyLineContent"><span class="doxyHighlight">                        size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00226"></a>226</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00227"></a>227</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00228"></a>228</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00229"></a>229</span><span class="doxyLineContent"><span class="doxyHighlight">                        (</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight">)state</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00230"></a>230</span><span class="doxyLineContent"><span class="doxyHighlight">                    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00231"></a>231</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00232"></a>232</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga337b3189bf4ff1b129765f11ae09f91e">qar_gui_panel_handle_destroy</a>(handle);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00233"></a>233</span><span class="doxyLineContent"><span class="doxyHighlight">                }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00234"></a>234</span><span class="doxyLineContent"><span class="doxyHighlight">            }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00235"></a>235</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00236"></a>236</span><span class="doxyLineContent"><span class="doxyHighlight">            free(handles);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00237"></a>237</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00238"></a>238</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00239"></a>239</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00240"></a>240</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_gui_panels_close_panel"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga7151e36d0c019e4eb5a30648f9ef738d">qar_gui_panels_close_panel</a>(session, &amp;panel_id));</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00241"></a>241</span><span class="doxyLineContent"><span class="doxyHighlightComment">    //! [gui_list]</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00242"></a>242</span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00243"></a>243</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-session/#gac86c6b3bc519f110d0d9db8adea0c5fa">qar_session_handle_destroy</a>(session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00244"></a>244</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a>(runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00245"></a>245</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> destroy_result = <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00246"></a>246</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_library_destroy"</span><span class="doxyHighlight">, destroy_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00247"></a>247</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00248"></a>248</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyLineNumber"><a id="l00249"></a>249</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>

</div>


<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
