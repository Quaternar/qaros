if(BUILD_CORE_EXAMPLES)
  set(QAR_DYNAMIC_EXAMPLES
      dynamic_loading
      session_create_and_join
      app_volume_management
      gui_panel_operations
      cpu_rendering_visualizer)

  foreach(sample ${QAR_DYNAMIC_EXAMPLES})
    add_executable(${sample} ${sample}.c)
    target_compile_features(${sample} PRIVATE c_std_11)
    target_link_libraries(${sample} PRIVATE qar-streaming-c-headers)
  endforeach()
endif()