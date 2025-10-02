# Bootstraps vcpkg inside the CMake binary directory and forwards to the actual toolchain.

if(NOT DEFINED VCPKG_TAG)
  set(VCPKG_TAG "2025.09.17" CACHE STRING "vcpkg git tag to checkout")
endif()

set(VCPKG_REPOSITORY "https://github.com/microsoft/vcpkg.git" CACHE STRING "vcpkg git repository")
set(VCPKG_ROOT "${CMAKE_BINARY_DIR}/../vcpkg" CACHE PATH "Location where vcpkg will be cloned")


set(_VCPKG_TOOLCHAIN "${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")
set(_VCPKG_TAG_FILE "${VCPKG_ROOT}/.vcpkg-tag")
set(_vcpkg_sources_changed FALSE)
set(_needs_fetch FALSE)
set(_existing_vcpkg_tag "")

if(EXISTS "${VCPKG_ROOT}")
  if(EXISTS "${_VCPKG_TAG_FILE}")
    file(READ "${_VCPKG_TAG_FILE}" _existing_vcpkg_tag)
    string(STRIP "${_existing_vcpkg_tag}" _existing_vcpkg_tag)
  endif()
  if(NOT EXISTS "${_VCPKG_TOOLCHAIN}")
    set(_needs_fetch TRUE)
  elseif(NOT _existing_vcpkg_tag STREQUAL VCPKG_TAG)
    set(_needs_fetch TRUE)
  endif()
else()
  set(_needs_fetch TRUE)
endif()

if(_needs_fetch)
  if(EXISTS "${VCPKG_ROOT}")
    message(STATUS "Removing vcpkg checkout at ${VCPKG_ROOT} (tag changed from '${_existing_vcpkg_tag}' to '${VCPKG_TAG}')")
    file(REMOVE_RECURSE "${VCPKG_ROOT}")
  endif()
  message(STATUS "Fetching vcpkg ${VCPKG_TAG} into ${VCPKG_ROOT}")
  execute_process(
    COMMAND git clone --branch ${VCPKG_TAG} --depth 1 ${VCPKG_REPOSITORY} "${VCPKG_ROOT}"
    RESULT_VARIABLE _vcpkg_clone_result
    ERROR_VARIABLE _vcpkg_clone_stderr
  )
  if(NOT _vcpkg_clone_result EQUAL 0)
    message(FATAL_ERROR "Failed to clone vcpkg: ${_vcpkg_clone_stderr}")
  endif()
  file(WRITE "${_VCPKG_TAG_FILE}" "${VCPKG_TAG}\n")
  set(_existing_vcpkg_tag "${VCPKG_TAG}")
  set(_vcpkg_sources_changed TRUE)
else()
  message(STATUS "Using existing vcpkg checkout (${_existing_vcpkg_tag}) in ${VCPKG_ROOT}")
endif()

if(WIN32)
  set(_vcpkg_executable "${VCPKG_ROOT}/vcpkg.exe")
  set(_bootstrap_cmd cmd)
  set(_bootstrap_args /c "${VCPKG_ROOT}/bootstrap-vcpkg.bat" -disableMetrics)
else()
  set(_vcpkg_executable "${VCPKG_ROOT}/vcpkg")
  set(_bootstrap_cmd "${VCPKG_ROOT}/bootstrap-vcpkg.sh")
  set(_bootstrap_args -disableMetrics)
endif()

if(_vcpkg_sources_changed OR NOT EXISTS "${_vcpkg_executable}")
  message(STATUS "Bootstrapping vcpkg in ${VCPKG_ROOT}")
  if(WIN32)
    execute_process(
      COMMAND ${_bootstrap_cmd} ${_bootstrap_args}
      WORKING_DIRECTORY "${VCPKG_ROOT}"
      RESULT_VARIABLE _bootstrap_result
      ERROR_VARIABLE _bootstrap_stderr
    )
  else()
    execute_process(
      COMMAND ${_bootstrap_cmd} ${_bootstrap_args}
      WORKING_DIRECTORY "${VCPKG_ROOT}"
      RESULT_VARIABLE _bootstrap_result
      ERROR_VARIABLE _bootstrap_stderr
    )
  endif()
  if(NOT _bootstrap_result EQUAL 0)
    message(FATAL_ERROR "Failed to bootstrap vcpkg: ${_bootstrap_stderr}")
  endif()
endif()

if(NOT EXISTS "${_VCPKG_TOOLCHAIN}")
  message(FATAL_ERROR "vcpkg toolchain missing at ${_VCPKG_TOOLCHAIN}")
endif()

function(_qaros_copy_compile_commands_to_parent_build_dir)
  if(NOT CMAKE_EXPORT_COMPILE_COMMANDS)
    return()
  endif()

  set(_compile_commands "${CMAKE_BINARY_DIR}/compile_commands.json")
  if(NOT EXISTS "${_compile_commands}")
    return()
  endif()
  get_filename_component(_compile_commands_parent "${CMAKE_BINARY_DIR}" DIRECTORY)
  file(COPY_FILE "${_compile_commands}" "${_compile_commands_parent}/compile_commands.json")
endfunction()

if(COMMAND cmake_language)
  cmake_language(DEFER CALL _qaros_copy_compile_commands_to_parent_build_dir)
endif()

include("${_VCPKG_TOOLCHAIN}")
