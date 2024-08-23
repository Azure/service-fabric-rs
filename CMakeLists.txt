cmake_minimum_required(VERSION 3.14)

project(service-fabric-rs LANGUAGES)

message(STATUS "fetching fabric_metadata")
include(FetchContent)
FetchContent_Declare(fabric_metadata
    GIT_REPOSITORY https://github.com/Azure/service-fabric-metadata.git
    GIT_TAG 552f4e38ad2c772419e52e62207c236fc31ffcbb
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
    NAMES cargo.exe cargo
    REQUIRED
)

add_custom_target(generate_rust
    COMMAND ${cargo_exe} run -p tools_api
    COMMAND ${cargo_exe} fmt -p mssf-com # format the generated crate
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust
    COMMAND ${cargo_exe} build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_rust_sample_client ALL
    COMMAND ${cargo_exe} build -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    #DEPENDS generate_import_libs
)

add_custom_target(run_rust_sample_client
    COMMAND ${cargo_exe} run -p samples_client
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# clean generated code
add_custom_target(force_clean
    COMMAND ${CMAKE_COMMAND} -E rm -rf crates/libs/com/src/Microsoft
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(build_fabric_rust_pal ALL
    COMMAND ${cargo_exe} build -p mssf-pal
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# only format not generated code
add_custom_target(format
    COMMAND ${cargo_exe} fmt -p fabric_rs
    COMMAND ${cargo_exe} fmt -p pal
    COMMAND ${cargo_exe} fmt -p samples_client
    COMMAND ${cargo_exe} fmt -p samples_echomain
    COMMAND ${cargo_exe} fmt -p samples_echomain_stateful
    COMMAND ${cargo_exe} fmt -p samples_echomain_stateful2
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_subdirectory(crates/samples/echomain)
add_subdirectory(crates/samples/echomain-stateful2)
add_subdirectory(crates/samples/echomain-stateful)

if(WIN32) #linux is not tested
    add_subdirectory(crates/samples/kvstore)
endif(WIN32)