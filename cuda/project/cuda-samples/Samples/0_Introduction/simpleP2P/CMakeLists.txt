cmake_minimum_required(VERSION 3.20)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../../../cmake/Modules")

project(simpleP2P LANGUAGES C CXX CUDA)

find_package(CUDAToolkit REQUIRED)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(CMAKE_CUDA_ARCHITECTURES 50 52 60 61 70 72 75 80 86 87 89 90 100 101 120)
set(CMAKE_CUDA_FLAGS "${CMAKE_CUDA_FLAGS} -Wno-deprecated-gpu-targets")
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    # set(CMAKE_CUDA_FLAGS "${CMAKE_CUDA_FLAGS} -G")  # enable cuda-gdb (expensive)
endif()

# Include directories and libraries
include_directories(../../../Common)

# Source file
# Add target for simpleP2P
if(CMAKE_SYSTEM_PROCESSOR STREQUAL "aarch64")
    message(STATUS "Will not build sample simpleP2P - not supported on aarch64")
else()
    add_executable(simpleP2P simpleP2P.cu)

    target_compile_options(simpleP2P PRIVATE $<$<COMPILE_LANGUAGE:CUDA>:--extended-lambda>)

    target_compile_features(simpleP2P PRIVATE cxx_std_17 cuda_std_17)

    set_target_properties(simpleP2P PROPERTIES CUDA_SEPARABLE_COMPILATION ON)
endif()
