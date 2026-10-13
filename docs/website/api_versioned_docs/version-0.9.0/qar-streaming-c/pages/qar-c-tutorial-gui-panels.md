---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/pages/qar-c-tutorial-gui-panels
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - page

---

<div class="doxyPage">

# GUI Panel Operations




## Overview {#gui_panels_overview}


<ul class="doxyList ">
<li>Obtain a session (rejoin, or onboard with the hub pairing code)</li>
<li>Get-or-create a GUI panel by its stable common name</li>
<li>Adjust pose, size, state, and navigate to content</li>
<li>Enumerate GUI panels to inspect current values</li>
</ul>

## Prerequisites {#gui_panels_prereq}


<ul class="doxyList ">
<li>Complete the <a href="/api/qar-streaming-c/pages/qar-c-tutorial-app-volumes">App Volume Management</a> tutorial</li>
<li>A running QarOS hub (for the first run, read the pairing code off its onboarding screen)</li>
</ul>

## Build and Run {#gui_panels_build}


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">cmake --build --preset x64-windows-debug --target gui_panel_operations</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">./build/x64-windows/Debug/gui_panel_operations.exe</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">&lt;path-to-qar-streaming-c.dll&gt; [runtime-dir] [pairing-code]</span></span></div>

</div>


## Parse Arguments {#gui_panels_args}


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &lt; 2)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/app-volume-management-c/#a7bf4a3818b91f144183c65e1d555b627">print_usage</a>(argv[0]);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 1;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* library_path = argv[1];</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* runtime_dir = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> runtime_dir_buffer[1024] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(argc &gt;= 3)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = argv[2];</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">else</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#aefc887c8f5c861b4e3949555aedf21c7">get_dir_from_path</a>(library_path, runtime_dir_buffer, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(runtime_dir_buffer)))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        runtime_dir = runtime_dir_buffer;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight">* pairing_code = (argc &gt;= 4) ? argv[3] : NULL;</span></span></div>

</div>


## Set Up the Session {#gui_panels_setup}


<p>The session is obtained with the shared rejoin-or-onboard helper described in the <a href="/api/qar-streaming-c/pages/qar-c-tutorial-onboarding">Onboarding and Rejoin</a> tutorial.</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!<a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga2f183f2df1f2f9db71fe520d154b1606">qar_library_load</a>(library_path))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"Failed to load '%s'.\n"</span><span class="doxyHighlight">, library_path);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 2;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarlibraryinit">QarLibraryInit</a> library_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac75cdcedfeb7cbbfae40405d5821e5b0">qar_library_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    library_init.<a href="/api/qar-streaming-c/structs/qarlibraryinit/#a4a4126bb11230817ddd769e5c2b41df1">enable_console_logging</a> = </span><span class="doxyHighlightKeyword">true</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> library_result = <a href="/api/qar-streaming-c/groups/qar-c-library/#ga7cc03dea04e7fd44d19fe459363d1df0">qar_library_init</a>(&amp;library_init);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(library_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_library_init"</span><span class="doxyHighlight">, library_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 3;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a84d3648b0a3e2280fcb5b2272a753587">QarRuntime</a>* runtime = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarruntimeinit">QarRuntimeInit</a> runtime_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga356269ae6fe65b091aa705537030ed98">qar_runtime_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    runtime_init.<a href="/api/qar-streaming-c/structs/qarruntimeinit/#aa3da4381535f98bb43257058308d9f86">runtime_binaries_folder_path</a> = runtime_dir;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> runtime_result = <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga6a057004f666fdb8632dded376d634c3">qar_runtime_create</a>(&amp;runtime_init, &amp;runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(runtime_result) || runtime == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_runtime_create"</span><span class="doxyHighlight">, runtime_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 4;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qaronboardingid">QarOnboardingId</a> onboarding_id = <a href="/api/qar-streaming-c/groups/qar-c-onboarding/#ga0f002137f2f3d3e5d7b27cc52742cb86">qar_onboarding_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a1c52c4c4175fadcb6c94a9dbe775113d">QarSession</a>* session = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a76cf853d349d580bce283477d0f52789">example_obtain_session</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           runtime,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           pairing_code,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"gui_panel_operations.onboarding-id.txt"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           </span><span class="doxyHighlightStringLiteral">"GUI Panel Peer"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;onboarding_id,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           &amp;session</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">       ) != 0</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">       || session == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-runtime/#ga66609ed3e8d28ebda58404eaabd27188">qar_runtime_destroy</a>(runtime);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-library/#gae9bfab210d6bac6953d61b4b93ceee68">qar_library_destroy</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-dynamic-loading/#ga927983ced8b11c98ef8e033182560572">qar_library_unload</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 5;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>

</div>


## Create and Update the Panel {#gui_panels_create}


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelinit">QarGuiPanelInit</a> panel_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga18a71cf97c53dbf7ecb2506d469f2221">qar_gui_panel_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a32f6fbc23514dc738fd2626dd55858f8">common_name</a> = </span><span class="doxyHighlightStringLiteral">"tutorial-panel.examples.qaros"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a7e7ea1e4b461b0b1e741499a9c9c7869">display_name</a> = </span><span class="doxyHighlightStringLiteral">"Tutorial Panel"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = 0.5f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = 1.5f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#ad23ccc05995e70c8ab57cd5f86666f58">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a>.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a> = 1.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    panel_init.<a href="/api/qar-streaming-c/structs/qarguipanelinit/#a99a1421c70ebea7911196f029f64fb40">size</a>.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a> = 0.7f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> panel_id = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga1119ac424b5f2d87acf2e423d565df51">qar_gui_panel_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> create_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#gae564de938ca8c589d2a533dc7df71496">qar_gui_panels_get_or_create</a>(session, &amp;panel_init, &amp;panel_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_gui_panels_get_or_create"</span><span class="doxyHighlight">, create_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(create_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"Created GUI panel with id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(panel_id.<a href="/api/qar-streaming-c/structs/qarguipanelid/#aadfd0f935c84663124e8a52caf0c071d">data</a>, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        printf(</span><span class="doxyHighlightStringLiteral">"\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> new_pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a> = 0.2f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a> = 1.6f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    new_pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_gui_panels_update_pose"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga88a8b5119ef11a2dc70d8b054c2ea95f">qar_gui_panels_update_pose</a>(session, &amp;panel_id, &amp;new_pose)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> new_size = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    new_size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a> = 1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    new_size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a> = 0.6f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightStringLiteral">"qar_gui_panels_change_size"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#gae5f7577e17d42493143bd37b31971c9f">qar_gui_panels_change_size</a>(session, &amp;panel_id, &amp;new_size)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-error/#gabea8c90eebe5815f6d388c520ac4d3e0">QAR_ASSUME_SUCCESS</a>(<a href="/api/qar-streaming-c/groups/qar-c-gui/#gadb86352d6421388d3979163ac2541516">qar_gui_panels_set_state</a>(session, &amp;panel_id, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6ba9e12b2178b4be52f8af3e7d274e39969">QAR_GUI_PANEL_STATE_MINIMIZED</a>));</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-error/#gabea8c90eebe5815f6d388c520ac4d3e0">QAR_ASSUME_SUCCESS</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga52f5db0bab40989e7aa4c57a2f280dbc">qar_gui_panels_navigate_to_uri</a>(session, &amp;panel_id, </span><span class="doxyHighlightStringLiteral">"https://example.com/tutorial"</span><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>

</div>


## Enumerate GUI Panels {#gui_panels_list}


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> panel_count = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> count_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga2efbe85e332acbdae9d4069b7a217ebc">qar_query_gui_panels_count</a>(session, &amp;panel_count);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_gui_panels_count"</span><span class="doxyHighlight">, count_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(count_result) &amp;&amp; panel_count &gt; 0)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>** handles = (<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>**)calloc(panel_count, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>*));</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(handles)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> written = 0;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> list_result = <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga49e5b5fe0aac0457c7649b15f68cc737">qar_query_gui_panels</a>(session, handles, panel_count, &amp;written);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_query_gui_panels"</span><span class="doxyHighlight">, list_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(list_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; written; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a94027a9e702d7d3f95c7d9202e86eb3a">QarGuiPanel</a>* handle = handles[i];</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(!handle)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightKeywordFlow">continue</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarguipanelid">QarGuiPanelId</a> </span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight"> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga1119ac424b5f2d87acf2e423d565df51">qar_gui_panel_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarguipanelsize">QarGuiPanelSize</a> size = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga752d00e64781ba17bd770ea32f24f898">qar_gui_panel_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/structs/qarpose">QarPose</a> pose = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6b">QarGuiPanelState</a> state = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a2e9bb864d68409e9d78a8a5e2e358c6bae47d8fbd35a55e0841f94797fdacce8f">QAR_GUI_PANEL_STATE_VISIBLE</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    </span><span class="doxyHighlightKeywordType">char</span><span class="doxyHighlight"> name[<a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a48c60e5b127e7195a52790c8cbcde601">QAR_MAX_STRING_LENGTH</a>] = { 0 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#gacc1f3b175faf098aab42a4e7e4be4c88">qar_gui_panel_get_id</a>(handle, &amp;</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga8ecec7a810bdfa32c313eb6e7577772e">qar_gui_panel_get_display_name</a>(handle, name, </span><span class="doxyHighlightKeyword">sizeof</span><span class="doxyHighlight">(name));</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga95a05d8ba3e6aaabaed399d2531307d0">qar_gui_panel_get_size</a>(handle, &amp;size);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga27f046c6022215093fa6305ee1aa95a1">qar_gui_panel_get_pose</a>(handle, &amp;pose);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    (void)<a href="/api/qar-streaming-c/groups/qar-c-gui/#ga21a6da5242fbf890ba915c6252a54823">qar_gui_panel_get_state</a>(handle, &amp;state);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span><span class="doxyHighlightStringLiteral">"- Panel id: "</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a4d02de32cbe028f4e7c5011362b1f775">print_hex_id</a>(</span><span class="doxyHighlightKeywordType">id</span><span class="doxyHighlight">.data, <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ae7887fdbfe8d612382c31171d55012dd">QAR_MAX_ID_LENGTH</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    printf(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">" name='%s' size(%.2f x %.2f) position(%.2f, %.2f, "</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        </span><span class="doxyHighlightStringLiteral">"%.2f) state=%d\n"</span><span class="doxyHighlight">,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        name,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a636241e50d8269f2da8cd06244fd1e45">width_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        size.<a href="/api/qar-streaming-c/structs/qarguipanelsize/#a1f80787522197037b60623db51f58f39">height_meters</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a8998e28822a8949bafd32d6fb24084a2">x</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a5c6b4d03c4255d53ee7a387c500fe56b">y</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        pose.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a>,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                        (</span><span class="doxyHighlightKeywordType">int</span><span class="doxyHighlight">)state</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                    <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga337b3189bf4ff1b129765f11ae09f91e">qar_gui_panel_handle_destroy</a>(handle);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">                }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            free(handles);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_gui_panels_close_panel"</span><span class="doxyHighlight">, <a href="/api/qar-streaming-c/groups/qar-c-gui/#ga7151e36d0c019e4eb5a30648f9ef738d">qar_gui_panels_close_panel</a>(session, &amp;panel_id));</span></span></div>

</div>


<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
