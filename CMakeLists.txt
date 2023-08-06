cmake_minimum_required(VERSION 3.14)

project(service-fabric-rs LANGUAGES)

message(STATUS "fetching fabric_metadata")
include(FetchContent)
FetchContent_Declare(fabric_metadata
    GIT_REPOSITORY https://github.com/youyuanwu/fabric-metadata.git
    GIT_TAG 9d62ca15573a0716fb9828dce03a489ae4b8a4a7
)
FetchContent_GetProperties(fabric_metadata)
if(NOT fabric_metadata_POPULATED)
    FetchContent_Populate(fabric_metadata)
    # do not add to cmake build since we only need winmd file to generate code.
endif()

# fetch service-fabric-cpp for its scripts
message(STATUS "fetching service-fabric-cpp")
include(FetchContent)
FetchContent_Declare(service_fabric_cpp
    GIT_REPOSITORY https://github.com/youyuanwu/service-fabric-cpp.git
    GIT_TAG 997f00bd4610834a3904b9cde8bf6b2c666bcb29
)
FetchContent_GetProperties(service_fabric_cpp)
if(NOT service_fabric_cpp_POPULATED)
    FetchContent_Populate(service_fabric_cpp)
    # do not add to cmake build since we only use scripts
endif()

# generate rust code
find_program (
    cargo_exe
    NAMES cargo.exe
    REQUIRED
)

add_custom_target(generate_rust
    #DEPENDS generate_winmd
    COMMAND ${cargo_exe} run -p tools_api
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust
    COMMAND ${cargo_exe} build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust_sample_client ALL
    COMMAND ${cargo_exe} build -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS generate_import_libs
)

add_custom_target(run_rust_sample_client
    COMMAND ${cargo_exe} run -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# clean generated code
add_custom_target(force_clean
    COMMAND ${CMAKE_COMMAND} -E rm -rf src/Microsoft
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust_sample_echomain ALL
    COMMAND ${cargo_exe} build -p samples_echomain
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS generate_import_libs
)

set(_pkg_root ${CMAKE_BINARY_DIR}/echoapp_root)
set(_pkg_src  ${CMAKE_CURRENT_SOURCE_DIR}/crates/samples/echomain)
set(_pkg_exe  ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/samples_echomain.exe)
add_custom_command(TARGET build_rust_sample_echomain POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ServiceManifest.xml ${_pkg_root}/EchoAppServicePackage/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_exe} ${_pkg_root}/EchoAppServicePackage/Code/echomain.exe # note the rename
)