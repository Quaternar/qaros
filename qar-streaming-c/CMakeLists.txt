add_library(qar-streaming-c-headers INTERFACE)

target_include_directories(
  qar-streaming-c-headers
  INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Consumers can opt into dynamic loading via this compile definition.
target_compile_definitions(qar-streaming-c-headers INTERFACE QAR_ENABLE_DYNAMIC_LOADING)

# Require C11 via this interface target.
target_compile_features(qar-streaming-c-headers INTERFACE c_std_11)

add_subdirectory(examples)