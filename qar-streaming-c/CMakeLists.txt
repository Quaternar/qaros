# The C header qar_streaming.h as an INTERFACE target. It carries no language requirement: a C++
# consumer links it without enabling C. C targets set their own C standard (the examples use C11).
add_library(qar-streaming-c-headers INTERFACE)

target_include_directories(
  qar-streaming-c-headers
  INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Every consumer of this target compiles with QAR_ENABLE_DYNAMIC_LOADING: the header then declares
# the loader (qar_library_load) and the wrappers that call the runtime through it, and exactly one
# source file of the program must expand QAR_IMPLEMENT_DYNAMIC_LOADING(). A program that links the
# runtime's import library instead includes the header without this target.
target_compile_definitions(qar-streaming-c-headers INTERFACE QAR_ENABLE_DYNAMIC_LOADING)

# The C examples. ON by default only when qaros is the top-level project; a project that adds
# this folder (or qaros) with add_subdirectory gets no examples unless it asks for them.
# BUILD_CORE_EXAMPLES, the earlier name, still decides the default when it is set.
if(DEFINED BUILD_CORE_EXAMPLES)
  set(_qaros_build_examples_default ${BUILD_CORE_EXAMPLES})
elseif(DEFINED Qaros_IS_TOP_LEVEL AND Qaros_IS_TOP_LEVEL)
  set(_qaros_build_examples_default ON)
else()
  set(_qaros_build_examples_default OFF)
endif()
option(QAROS_BUILD_EXAMPLES "Build the C examples of qar-streaming-c (no external dependencies)"
       ${_qaros_build_examples_default})
unset(_qaros_build_examples_default)

if(QAROS_BUILD_EXAMPLES OR BUILD_VULKAN_EXAMPLE)
  add_subdirectory(examples)
endif()
