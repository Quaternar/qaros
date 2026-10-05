if(BUILD_CORE_EXAMPLES)
  set(QAR_DYNAMIC_EXAMPLES
      dynamic_loading
      onboarding_and_rejoin
      app_volume_management
      gui_panel_operations
      cpu_rendering_visualizer)

  foreach(sample ${QAR_DYNAMIC_EXAMPLES})
    add_executable(${sample} ${sample}.c)
    target_compile_features(${sample} PRIVATE c_std_11)
    target_link_libraries(${sample} PRIVATE qar-streaming-c-headers)
  endforeach()
endif()

if(BUILD_VULKAN_EXAMPLE)
  if(NOT WIN32)
    message(FATAL_ERROR "vulkan_cube currently requires Windows")
  endif()
  add_subdirectory(vulkan_cube)
endif()
