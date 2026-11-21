# The member hub cube: qar-member-hub-server and qar-member-hub-client, two
# Vulkan apps that show a primary and a member hub at work. They draw the cube
# of ../vulkan_cube with its renderer and shaders. The member-hub calls are
# Windows only today: elsewhere they fail with QAR_STATUS_NOT_IMPLEMENTED.
find_package(Vulkan REQUIRED COMPONENTS glslc)
find_package(glm CONFIG REQUIRED)
find_package(Threads REQUIRED)

# Standalone asio (header only, no Boost) carries the invitation channel on
# every platform. Always the pinned release, downloaded at configure time: the
# example never takes an asio the surrounding build happens to have.
include(FetchContent)
FetchContent_Declare(qar_member_hub_asio
  URL https://github.com/chriskohlhoff/asio/archive/refs/tags/asio-1-30-2.tar.gz
  URL_HASH SHA256=755bd7f85a4b269c67ae0ea254907c078d408cce8e1a352ad2ed664d233780e8
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE)
FetchContent_MakeAvailable(qar_member_hub_asio)
if(NOT TARGET qar_member_hub_asio)
  add_library(qar_member_hub_asio INTERFACE)
  target_include_directories(qar_member_hub_asio INTERFACE
    "${qar_member_hub_asio_SOURCE_DIR}/asio/include")
endif()
set(QAR_MEMBER_HUB_ASIO qar_member_hub_asio)

set(QAR_CUBE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../vulkan_cube")
set(QAR_MEMBER_HUB_SHADERS "${CMAKE_CURRENT_BINARY_DIR}/shaders")
foreach(stage vert frag)
  add_custom_command(OUTPUT "${QAR_MEMBER_HUB_SHADERS}/cube.${stage}.spv"
    COMMAND ${CMAKE_COMMAND} -E make_directory "${QAR_MEMBER_HUB_SHADERS}"
    COMMAND Vulkan::glslc "${QAR_CUBE_DIR}/cube.${stage}"
      -o "${QAR_MEMBER_HUB_SHADERS}/cube.${stage}.spv"
    DEPENDS "${QAR_CUBE_DIR}/cube.${stage}" VERBATIM)
endforeach()
add_custom_command(OUTPUT "${QAR_MEMBER_HUB_SHADERS}/CubeShaders.hpp"
  COMMAND ${CMAKE_COMMAND}
    "-DINPUT_DIRECTORY=${QAR_MEMBER_HUB_SHADERS}"
    "-DOUTPUT=${QAR_MEMBER_HUB_SHADERS}/CubeShaders.hpp"
    -P "${QAR_CUBE_DIR}/embed_shaders.cmake"
  DEPENDS "${QAR_MEMBER_HUB_SHADERS}/cube.vert.spv"
    "${QAR_MEMBER_HUB_SHADERS}/cube.frag.spv" "${QAR_CUBE_DIR}/embed_shaders.cmake" VERBATIM)
add_custom_target(member_hub_cube_shaders DEPENDS "${QAR_MEMBER_HUB_SHADERS}/CubeShaders.hpp")

foreach(app server client)
  set(target qar-member-hub-${app})
  add_executable(${target} ${app}.cpp cube_app.cpp invite_channel.cpp "${QAR_CUBE_DIR}/renderer.cpp")
  target_compile_features(${target} PRIVATE cxx_std_20)
  target_compile_definitions(${target} PRIVATE QAR_ENABLE_DYNAMIC_LOADING QAR_ENABLE_VULKAN
    ASIO_STANDALONE GLM_FORCE_DEPTH_ZERO_TO_ONE
    $<$<PLATFORM_ID:Windows>:VK_USE_PLATFORM_WIN32_KHR;NOMINMAX;WIN32_LEAN_AND_MEAN;_WIN32_WINNT=0x0A00>)
  target_include_directories(${target} PRIVATE "${QAR_CUBE_DIR}" "${QAR_MEMBER_HUB_SHADERS}")
  target_link_libraries(${target} PRIVATE qar-streaming-c-headers Vulkan::Vulkan glm::glm
    Threads::Threads ${QAR_MEMBER_HUB_ASIO})
  add_dependencies(${target} member_hub_cube_shaders)
endforeach()
