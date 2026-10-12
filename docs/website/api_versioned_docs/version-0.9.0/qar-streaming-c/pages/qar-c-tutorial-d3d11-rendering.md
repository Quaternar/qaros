---

# DO NOT EDIT!
# Automatically generated via doxygen2docusaurus by Doxygen.

slug: /qar-streaming-c/pages/qar-c-tutorial-d3d11-rendering
custom_edit_url: null
toc_max_heading_level: 4
keywords:
  - doxygen
  - reference
  - page

---

<div class="doxyPage">

# D3D11 Rendering




## Overview {#d3d11_rendering_overview}


<ul class="doxyList ">
<li>Obtain a session as in earlier tutorials (rejoin or onboard)</li>
<li>Create the app volume the stream is displayed in</li>
<li>Wait for a peer (e.g. a Visualizer) to request a render stream</li>
<li>Create a D3D11 device, chain it into the render sender with <a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11">QarStreamParamsD3D11</a>, and clear each colour view of the sender's shared textures directly on the GPU - no CPU copy</li>
</ul>

## Prerequisites {#d3d11_rendering_prereq}


<ul class="doxyList ">
<li>Complete the <a href="/api/qar-streaming-c/pages/qar-c-tutorial-cpu-rendering">CPU Rendering with a Visualizer</a> tutorial; this one differs only in the graphics backend</li>
<li>Windows with a D3D11.1-capable GPU</li>
<li>A running QarOS hub (for the first run, read the pairing code off its onboarding screen), with a peer that requests a render stream from this application once it is running</li>
</ul>

## Build and Run {#d3d11_rendering_build}


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">cmake --build --preset x64-windows-debug --target d3d11_render_sender</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">./build/x64-windows/Debug/d3d11_render_sender.exe</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">&lt;path-to-qar-streaming-c.dll&gt; [runtime-dir] [pairing-code]</span></span></div>

</div>


## Library, Runtime, Session, and App Volume {#d3d11_rendering_setup}


<p>Argument parsing and session setup match the <a href="/api/qar-streaming-c/pages/qar-c-tutorial-cpu-rendering">CPU Rendering with a Visualizer</a> tutorial. A render sender is displayed in an app volume, so one is created before the sender.</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumeinit">QarAppVolumeInit</a> volume_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gafda139e7c1a2424c1c58ebe3c07aa1ba">qar_app_volume_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#adbd20eb46c7e9ef54b656102529b3b0c">common_name</a> = </span><span class="doxyHighlightStringLiteral">"d3d11-volume.examples.qaros"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#adbb5e07d43c6c4458acfb4c7a20220bf">display_name</a> = </span><span class="doxyHighlightStringLiteral">"D3D11 Tutorial Volume"</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gac50f0d1a5c21bc238f3293230ca94359">qar_pose_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a27fd0d3ea7c3b277ecea35a563680454">pose</a>.<a href="/api/qar-streaming-c/structs/qarpose/#ab84fc35f38bcfc57d7deb9a3daf24cb2">position</a>.<a href="/api/qar-streaming-c/structs/qarvector3/#a6ebf6173153df8c50b63b842204d48b4">z</a> = -1.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    volume_init.<a href="/api/qar-streaming-c/structs/qarappvolumeinit/#a71fc823912cc40c74f798a97aecd77dc">size</a> = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga9a06d2b56897aa2048f3651defb1c3d4">qar_app_volume_size_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarappvolumeid">QarAppVolumeId</a> volume_id = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gaa01cc36e4fc2dbbd70faf2c78d07d43f">qar_app_volume_id_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> volume_result = <a href="/api/qar-streaming-c/groups/qar-c-app-volumes/#ga76e9061c6e95e92787686a130604a140">qar_app_volumes_get_or_create</a>(session, &amp;volume_init, &amp;volume_id);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_app_volumes_get_or_create"</span><span class="doxyHighlight">, volume_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(volume_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#a42996b4bc244553bd3fb38cd960879e1">teardown</a>(runtime, session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 6;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>

</div>


## Create the D3D11 Device {#d3d11_rendering_device}


<p>The sender needs the ID3D11Device1 / ID3D11DeviceContext1 interfaces, so the device is created as usual and queried for them.</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">bool</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#af5d9babb36612bcc61b7064b5a79fc91">create_d3d11</a>(<a href="/api/qar-streaming-c/structs/d3d11objects">D3D11Objects</a>* out_d3d)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> D3D_FEATURE_LEVEL levels[] = { D3D_FEATURE_LEVEL_11_1 };</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11Device* device = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11DeviceContext* context = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    HRESULT hr = D3D11CreateDevice(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        NULL,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        D3D_DRIVER_TYPE_HARDWARE,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        NULL,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        D3D11_CREATE_DEVICE_BGRA_SUPPORT,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        levels,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        1,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        D3D11_SDK_VERSION,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        &amp;device,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        NULL,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        &amp;context</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(FAILED(hr))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"D3D11CreateDevice failed (0x%08lX)\n"</span><span class="doxyHighlight">, (</span><span class="doxyHighlightKeywordType">unsigned</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">long</span><span class="doxyHighlight">)hr);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">false</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    out_d3d-&gt;<a href="/api/qar-streaming-c/structs/d3d11objects/#a2baed448b2af016e4a16e144e9772e0d">device</a> = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    out_d3d-&gt;<a href="/api/qar-streaming-c/structs/d3d11objects/#af5a01b03997a62d70a6489920aeea2b1">context</a> = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    HRESULT device_hr =</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        ID3D11Device_QueryInterface(device, &amp;IID_ID3D11Device1, (</span><span class="doxyHighlightKeywordType">void</span><span class="doxyHighlight">**)&amp;out_d3d-&gt;<a href="/api/qar-streaming-c/structs/d3d11objects/#a2baed448b2af016e4a16e144e9772e0d">device</a>);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    HRESULT context_hr = ID3D11DeviceContext_QueryInterface(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        context, &amp;IID_ID3D11DeviceContext1, (</span><span class="doxyHighlightKeywordType">void</span><span class="doxyHighlight">**)&amp;out_d3d-&gt;<a href="/api/qar-streaming-c/structs/d3d11objects/#af5a01b03997a62d70a6489920aeea2b1">context</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    );</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11DeviceContext_Release(context);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11Device_Release(device);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(FAILED(device_hr) || FAILED(context_hr))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"D3D11.1 interfaces not available\n"</span><span class="doxyHighlight">);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#a20a552b805cf92d893f523aa96e28f87">release_d3d11</a>(out_d3d);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">false</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeyword">true</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>

</div>


## Create the D3D11 Render Sender {#d3d11_rendering_sender}


<p>Select QAR_GRAPHICS_API_D3D11 and chain <a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11">QarStreamParamsD3D11</a> into QarRenderSenderInit::header.next. The bind flags decide how the shared textures can be used; keep acquire_keyed_mutex_sync true so the library acquires and releases each texture's keyed mutex around the frame cycle.</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11">QarStreamParamsD3D11</a> d3d11_params = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gafd5697eabb61fe8d099faa7156ae1c1e">qar_stream_params_d3d11_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#a2906b6ba1976d9c1e95cfd60828a4be8">d3d11_device</a> = d3d.device;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#ad680309ae04fda579fbf290c5edcdb03">d3d11_context</a> = d3d.context;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#a1f6099dbce22f2bd4a8361a6972842de">color_bind_flags</a> = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#a716878f9773a49bd89d2448f3d731920">depth_bind_flags</a> = D3D11_BIND_DEPTH_STENCIL | D3D11_BIND_SHADER_RESOURCE;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#acca712000793736c6942c727a6e06f12">acquire_keyed_mutex_sync</a> = </span><span class="doxyHighlightKeyword">true</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarrendersenderinit">QarRenderSenderInit</a> sender_init = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga5bbb69c69ea0b0c961c86905c4742c1b">qar_render_sender_init_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    sender_init.<a href="/api/qar-streaming-c/structs/qarrendersenderinit/#a5395c0cc66f54d9db58ece93f8405422">graphics_api</a> = <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a51f21577060dbb5ba4f1ce0d1eaba53faaaae2ab20e0932fb013df988f44653df">QAR_GRAPHICS_API_D3D11</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    sender_init.<a href="/api/qar-streaming-c/structs/qarrendersenderinit/#aaea789b09c5fd7e1b629228641bbe8a3">peer_id</a> = request_state.target_peer_id;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    sender_init.<a href="/api/qar-streaming-c/structs/qarrendersenderinit/#a9e643a49c3b4a8fa70593769f31b046e">app_volume_id</a> = &amp;volume_id;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    sender_init.<a href="/api/qar-streaming-c/structs/qarrendersenderinit/#a978f5e057d79b81674fe93f71a826025">header</a>.<a href="/api/qar-streaming-c/structs/qarstructureheader/#a10911ac494fae9cb85f835c9097c8c09">next</a> = &amp;d3d11_params.<a href="/api/qar-streaming-c/structs/qarstreamparamsd3d11/#a37cade1e6411e9204d3e2616805577a3">header</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a5ef0bf17db281fce97fa942ac054f2fc">QarRenderSender</a>* sender = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> sender_result = <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#ga7516be21e690ce67b9b10ef8dd897dde">qar_render_sender_create</a>(session, &amp;sender_init, NULL, &amp;sender);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_render_sender_create"</span><span class="doxyHighlight">, sender_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(sender_result) || sender == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#a20a552b805cf92d893f523aa96e28f87">release_d3d11</a>(&amp;d3d);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#a42996b4bc244553bd3fb38cd960879e1">teardown</a>(runtime, session);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">return</span><span class="doxyHighlight"> 9;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>

</div>


## Render and Submit Frames {#d3d11_rendering_frames}


<p>Inside each begin-frame / show-frame cycle, <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#gae189a021de117ab646bd9f73fbe3b2a9">qar_render_sender_frame_d3d11()</a> hands out the shared textures and the views laid out on them. Each colour view is a pixel rectangle on one texture; here it is cleared to an animated colour with ID3D11DeviceContext1::ClearView. A real renderer draws its scene into the same rectangle, using the view poses from the frame info.</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> frame_count = 300;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> frame_index = 0; frame_index &lt; frame_count; ++frame_index)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#ade0e9e62ce0c96cf65e5e8215027b62c">QarRenderFrameInfo</a>* frame_info = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> begin_result = <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#ga86c067e2501014a8ebd9d61c9b2965a3">qar_render_sender_begin_frame</a>(sender, NULL, &amp;frame_info);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(begin_result) || frame_info == NULL)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_render_sender_begin_frame"</span><span class="doxyHighlight">, begin_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">break</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarvideoframed3d11">QarVideoFrameD3D11</a> frame = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#gabd2bee0a0ec6969478ab9c30e380901c">qar_video_frame_d3d11_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> frame_result = <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#gae189a021de117ab646bd9f73fbe3b2a9">qar_render_sender_frame_d3d11</a>(sender, &amp;frame);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga6a523b6292e03a1886c12d84fcf91826">qar_result_is_success</a>(frame_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#ad9c3b973b579f755c28ce21546ffae1a">clear_color_views</a>(d3d.device, d3d.context, &amp;frame, frame_index);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">else</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_render_sender_frame_d3d11"</span><span class="doxyHighlight">, frame_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightComment">/* Show closes the frame cycle that begin-frame opened. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarrenderframeshow">QarRenderFrameShow</a> show = <a href="/api/qar-streaming-c/groups/qar-c-defaults/#ga237acdde53d69706bfc13c10342cac6e">qar_render_frame_show_default</a>();</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        show.<a href="/api/qar-streaming-c/structs/qarrenderframeshow/#a7875666d3bf70962fb39a21f12c38eef">rendered_near_far</a>.<a href="/api/qar-streaming-c/structs/qarnearfar/#a783bca241e4d8a4880e3353bff0dddca">near_plane</a> = 0.1f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        show.<a href="/api/qar-streaming-c/structs/qarrenderframeshow/#a7875666d3bf70962fb39a21f12c38eef">rendered_near_far</a>.<a href="/api/qar-streaming-c/structs/qarnearfar/#a5434b10697987d5ee57580f00e8fb6cf">far_plane</a> = 10.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/structs/qarresult">QarResult</a> show_result = <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#ga68a7b1e14c6d7f1d9198fdfc222cb0ef">qar_render_sender_show_frame</a>(sender, &amp;show);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#ga167970742e5e5b9415f7073b71a74db4">qar_render_frame_info_handle_destroy</a>(frame_info);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(<a href="/api/qar-streaming-c/groups/qar-c-error/#ga926cc10284202d684d5cb1908969bd77">qar_result_is_error</a>(show_result))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            <a href="/api/qar-streaming-c/files/qar-streaming-c/examples/common-h/#a0078bc3763ae2e11c1cd28a58059f73c">log_result</a>(</span><span class="doxyHighlightStringLiteral">"qar_render_sender_show_frame"</span><span class="doxyHighlight">, show_result);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">break</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    <a href="/api/qar-streaming-c/groups/qar-c-render-sender/#gaa483cc940ae56a4d66545020f3957872">qar_render_stream_handle_destroy</a>(sender);</span></span></div>

</div>


<p>The clear itself:</p>


<div class="doxyProgramListing">

<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlightComment">/** \brief Clear every colour view of the frame to an animated colour. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlightKeyword">static</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">void</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight"><a href="/api/qar-streaming-c/files/qar-streaming-c/examples/d3d11-render-sender-c/#ad9c3b973b579f755c28ce21546ffae1a">clear_color_views</a>(</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11Device1* device,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    ID3D11DeviceContext1* context,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvideoframed3d11">QarVideoFrameD3D11</a>* frame,</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> frame_index</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">{</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> t = (float)(frame_index % 120) / 120.0f;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    </span><span class="doxyHighlightKeywordFlow">for</span><span class="doxyHighlight">(</span><span class="doxyHighlightKeywordType">size_t</span><span class="doxyHighlight"> i = 0; i &lt; frame-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframed3d11/#a9074fdea9590db15eefc704aa5038da2">texture_views_count</a>; ++i)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> <a href="/api/qar-streaming-c/structs/qarvideoframeview">QarVideoFrameView</a>* view = &amp;frame-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframed3d11/#a53a30fbf9bfa1cd0b6cdadb1de4e9193">texture_views</a>[i];</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#aabfc9293117118c6fb3ab13fe73c0b89">data_type</a> != <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#afd4eed6c42419cb509cfc7563f21c93ba8dbee13eb437c8059e91298f5c831eba">QAR_VIDEO_FRAME_VIEW_TYPE_COLOR</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">           || view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a40abe63ad075eed5fc6ad3f34a06275d">texture_index</a> &gt;= frame-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframed3d11/#a91df0a927c13ed1faeff3df319f8baf1">textures_count</a>)</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">continue</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        ID3D11Texture2D* texture = frame-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframed3d11/#a92eba169ce9f42807092332337ab07a9">textures</a>[view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a40abe63ad075eed5fc6ad3f34a06275d">texture_index</a>].<a href="/api/qar-streaming-c/structs/qarvideotextured3d11/#aa24c4553fcd738cbd06ec1c438cc7dc6">texture</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        ID3D11RenderTargetView* rtv = NULL;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        HRESULT hr =</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            ID3D11Device1_CreateRenderTargetView(device, (ID3D11Resource*)texture, NULL, &amp;rtv);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeywordFlow">if</span><span class="doxyHighlight">(FAILED(hr))</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            fprintf(stderr, </span><span class="doxyHighlightStringLiteral">"CreateRenderTargetView failed (0x%08lX)\n"</span><span class="doxyHighlight">, (</span><span class="doxyHighlightKeywordType">unsigned</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">long</span><span class="doxyHighlight">)hr);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            </span><span class="doxyHighlightKeywordFlow">continue</span><span class="doxyHighlight">;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightComment">/* Left eye red-ish, right eye blue-ish, both pulsing. */</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">bool</span><span class="doxyHighlight"> right = view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a67127a88873f012df0adb1d0312f3491">eye</a> == <a href="/api/qar-streaming-c/files/qar-streaming-c/include/qar-streaming-h/#a9f8b5253bfeca206f47bb55fa55ad048a37a2c4b9e2c2746f5f9a14708e33d67a">QAR_VIDEO_FRAME_VIEW_EYE_RIGHT</a>;</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> </span><span class="doxyHighlightKeywordType">float</span><span class="doxyHighlight"> color[4] = { right ? 0.1f : t, 0.2f, right ? t : 0.1f, 1.f };</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        </span><span class="doxyHighlightKeyword">const</span><span class="doxyHighlight"> D3D11_RECT rect = {</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">            (LONG)view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a6e8514662cf36528969a578a949f8792">start_x</a>, (LONG)view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a3cc6d98aef20a56db8c8cc06a3f54eba">start_y</a>, (LONG)view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#acc90d70d0085bbf4460aabf2a9cbcf69">end_x</a>, (LONG)view-&gt;<a href="/api/qar-streaming-c/structs/qarvideoframeview/#a9fec28c93ef2d33a6a4f39d6e71efc79">end_y</a></span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        };</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        ID3D11DeviceContext1_ClearView(context, (ID3D11View*)rtv, color, &amp;rect, 1);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">        ID3D11RenderTargetView_Release(rtv);</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">    }</span></span></div>
<div class="doxyCodeLine"><span class="doxyNoLineNumber">&nbsp;</span><span class="doxyLineContent"><span class="doxyHighlight">}</span></span></div>

</div>


<hr/>

<p class="doxyGeneratedBy">Generated via <a href="https://xpack.github.io/doxygen2docusaurus">doxygen2docusaurus</a> 2.2.1 by <a href="https://www.doxygen.nl">Doxygen</a> 1.9.8.</p>

</div>
