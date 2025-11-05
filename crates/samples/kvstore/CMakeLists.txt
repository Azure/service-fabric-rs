# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_kvstore ALL
    COMMAND ${cargo_exe} build -p kvstore
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

include(PackageSF)
add_sf_pkg(
    TARGET build_rust_sample_kvstore
    EXECUTABLE ${CMAKE_SOURCE_DIR}/target/debug/kvstore
    MANIFEST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/manifests
    OUT_DIR ${CMAKE_BINARY_DIR}/sf_apps/kvstore
)