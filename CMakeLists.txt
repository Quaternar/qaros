cmake_minimum_required(VERSION 3.29)

if(CMAKE_SOURCE_DIR STREQUAL CMAKE_BINARY_DIR)
  message(FATAL_ERROR "In-source builds are not supported. Please configure an out-of-source build.")
endif()

project(Qaros VERSION 0.1.0 LANGUAGES C CXX)

if(NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_CONFIGURATION_TYPES "Debug;Release" CACHE STRING "Supported build configurations" FORCE)
endif()

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

option(BUILD_CORE_EXAMPLES "Build core examples showing basic api usage without any need for external libraries or dependencies" ON)

add_subdirectory(qar-streaming-c)

