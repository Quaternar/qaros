if(BUILD_CORE_EXAMPLES)
  add_executable(library_dynamic_loading library_dynamic_loading.c)
  target_compile_features(library_dynamic_loading PRIVATE c_std_11)
  target_link_libraries(library_dynamic_loading PRIVATE qar-streaming-c-headers)


endif()
