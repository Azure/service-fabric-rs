cmake_minimum_required(VERSION 3.14)

project(service-fabric-rs LANGUAGES)

message(STATUS "fetching fabric_metadata")
include(FetchContent)
FetchContent_Declare(fabric_metadata
    GIT_REPOSITORY https://github.com/Azure/service-fabric-metadata.git
    GIT_TAG 87041e9790edb7f87dbf6ca8c1ca0f40db8c82dd
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

set(_runtime_file ${CMAKE_CURRENT_SOURCE_DIR}/crates/libs/com/src/Microsoft/ServiceFabric/FabricCommon/FabricRuntime/mod.rs)
set(_client_file ${CMAKE_CURRENT_SOURCE_DIR}/crates/libs/com/src/Microsoft/ServiceFabric/FabricCommon/FabricClient/mod.rs)
set(_common_file ${CMAKE_CURRENT_SOURCE_DIR}/crates/libs/com/src/Microsoft/ServiceFabric/FabricCommon/mod.rs)
add_custom_target(generate_rust
    #DEPENDS generate_winmd
    COMMAND ${cargo_exe} run -p tools_api
    COMMAND powershell -Command "\"(Get-Content ${_runtime_file}).Replace('\\\"fabricruntime\\\"','\\\"FabricRuntime\\\"') | Set-Content ${_runtime_file}\""
    COMMAND powershell -Command "\"(Get-Content ${_client_file}).Replace('\\\"fabricclient\\\"','\\\"FabricClient\\\"') | Set-Content ${_client_file}\""
    COMMAND powershell -Command "\"(Get-Content ${_common_file}).Replace('\\\"fabriccommon\\\"','\\\"FabricCommon\\\"') | Set-Content ${_common_file}\""
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
    add_subdirectory(crates/samples/echo2)
    add_subdirectory(crates/samples/kvstore)
endif(WIN32)