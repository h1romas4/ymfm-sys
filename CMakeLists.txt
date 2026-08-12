# This CMake project is used only to generate compile_commands.json.
# The following symlink makes the cxx-generated header visible to CMake.
# Run this command from the project root:
# ln -s $(pwd)/src/shim.h target/cxxbridge/ymfm-sys/src/shim.h
cmake_minimum_required(VERSION 3.16)
project(ymfm_sys_cpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(ymfm STATIC
    src/shim.cpp
    components/ymfm/src/ymfm_adpcm.cpp
    components/ymfm/src/ymfm_misc.cpp
    components/ymfm/src/ymfm_opl.cpp
    components/ymfm/src/ymfm_opm.cpp
    components/ymfm/src/ymfm_opn.cpp
    components/ymfm/src/ymfm_opq.cpp
    components/ymfm/src/ymfm_opz.cpp
    components/ymfm/src/ymfm_pcm.cpp
    components/ymfm/src/ymfm_ssg.cpp
)

target_include_directories(ymfm PUBLIC
    components/ymfm/src/
    target/cxxbridge/
)
target_compile_options(ymfm PRIVATE -Wno-unused-parameter)
