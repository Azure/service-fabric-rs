cmake_minimum_required(VERSION 3.14)

project(service-fabric-rs)

# only fetch the minimum cpp part
set(service_fabric_BuildIDLOnly ON)
message(STATUS "feching service_fabric_cpp")
include(FetchContent)
FetchContent_Declare(service_fabric_cpp
    GIT_REPOSITORY https://github.com/youyuanwu/service-fabric-cpp.git
    GIT_TAG 88df1525a4dce55917078b58d6020829b4d787d1
)
FetchContent_GetProperties(service_fabric_cpp)
if(NOT service_fabric_cpp_POPULATED)
    FetchContent_Populate(service_fabric_cpp)
    # add all targets to main build. should not be too many for now.
    add_subdirectory(${service_fabric_cpp_SOURCE_DIR} ${service_fabric_cpp_BINARY_DIR})
endif()

# generate
find_program (
    dotnet_exe
    NAMES dotnet.exe
    REQUIRED
)

set(_out_winmds
    ServiceFabric.winmd
    Windows.Win32.Interop.winmd
    Windows.Win32.winmd
)
set(_out_winmd_paths "")
foreach(_out_winmd ${_out_winmds})
    list(APPEND _out_winmd_paths ${CMAKE_CURRENT_SOURCE_DIR}/.windows/winmd/${_out_winmd})
endforeach()

add_custom_command(
    OUTPUT ${_out_winmd_paths}
    COMMAND ${dotnet_exe} build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/.metadata
    VERBATIM
)

# force generate
add_custom_target(generate_winmd ALL
    DEPENDS ${_out_winmd_paths}
)

# generate rust code
find_program (
    cargo_exe
    NAMES cargo.exe
    REQUIRED
)

add_custom_target(generate_rust
    DEPENDS generate_winmd
    COMMAND ${cargo_exe} run -p tools_api
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust
    COMMAND ${cargo_exe} build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust_sample_client
    COMMAND ${cargo_exe} build -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(run_rust_sample
    COMMAND ${cargo_exe} run -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(force_clean
    COMMAND ${CMAKE_COMMAND} -E rm -rf .metadata/Obj src/Microsoft .windows
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust_sample_echomain
    COMMAND ${cargo_exe} build -p sample_echomain
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set(_pkg_root ${CMAKE_BINARY_DIR}/echoapp_root)
set(_pkg_src  ${CMAKE_CURRENT_SOURCE_DIR}/crates/samples/echomain)
set(_pkg_exe  ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/sample_echomain.exe)
add_custom_command(TARGET build_rust_sample_echomain POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ServiceManifest.xml ${_pkg_root}/EchoAppServicePackage/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_exe} ${_pkg_root}/EchoAppServicePackage/Code/echomain.exe # note the rename
)