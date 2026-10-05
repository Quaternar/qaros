find_package(Vulkan REQUIRED COMPONENTS glslc)
find_package(glm CONFIG REQUIRED)
add_executable(qar-vulkan-source main.cpp renderer.cpp)
target_compile_features(qar-vulkan-source PRIVATE cxx_std_20)
target_compile_definitions(qar-vulkan-source PRIVATE QAR_ENABLE_DYNAMIC_LOADING QAR_ENABLE_VULKAN VK_USE_PLATFORM_WIN32_KHR NOMINMAX GLM_FORCE_DEPTH_ZERO_TO_ONE)
target_link_libraries(qar-vulkan-source PRIVATE qar-streaming-c-headers Vulkan::Vulkan glm::glm)
foreach(stage vert frag)
  add_custom_command(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/cube.${stage}.spv"
    COMMAND Vulkan::glslc "${CMAKE_CURRENT_SOURCE_DIR}/cube.${stage}"
      -o "${CMAKE_CURRENT_BINARY_DIR}/cube.${stage}.spv"
    DEPENDS "cube.${stage}" VERBATIM)
endforeach()
add_custom_command(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/CubeShaders.hpp"
  COMMAND ${CMAKE_COMMAND}
    "-DINPUT_DIRECTORY=${CMAKE_CURRENT_BINARY_DIR}"
    "-DOUTPUT=${CMAKE_CURRENT_BINARY_DIR}/CubeShaders.hpp"
    -P "${CMAKE_CURRENT_SOURCE_DIR}/embed_shaders.cmake"
  DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/cube.vert.spv"
    "${CMAKE_CURRENT_BINARY_DIR}/cube.frag.spv" embed_shaders.cmake VERBATIM)
add_custom_target(vulkan_cube_shaders DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/CubeShaders.hpp")
add_dependencies(qar-vulkan-source vulkan_cube_shaders)
target_include_directories(qar-vulkan-source PRIVATE "${CMAKE_CURRENT_BINARY_DIR}")

if(BUILD_VULKAN_EXAMPLE_TESTS)
  add_executable(vulkan_cube_gpu_test renderer_test.cpp renderer.cpp)
  target_compile_features(vulkan_cube_gpu_test PRIVATE cxx_std_20)
  target_compile_definitions(vulkan_cube_gpu_test PRIVATE QAR_ENABLE_DYNAMIC_LOADING QAR_ENABLE_VULKAN VK_USE_PLATFORM_WIN32_KHR NOMINMAX GLM_FORCE_DEPTH_ZERO_TO_ONE)
  target_link_libraries(vulkan_cube_gpu_test PRIVATE qar-streaming-c-headers Vulkan::Vulkan glm::glm)
  add_dependencies(vulkan_cube_gpu_test vulkan_cube_shaders)
  target_include_directories(vulkan_cube_gpu_test PRIVATE "${CMAKE_CURRENT_BINARY_DIR}")
  add_test(NAME vulkan_cube_two_cameras COMMAND vulkan_cube_gpu_test)
  set_tests_properties(vulkan_cube_two_cameras PROPERTIES TIMEOUT 120)
endif()
