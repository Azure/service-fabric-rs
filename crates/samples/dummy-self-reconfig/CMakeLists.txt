# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_dummy_self_reconfig ALL
    COMMAND ${cargo_exe} build -p dummy-self-reconfig
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)
include(PackageSF)
add_sf_pkg(
    TARGET build_rust_sample_dummy_self_reconfig
    EXECUTABLE ${CMAKE_SOURCE_DIR}/target/debug/dummy-self-reconfig
    MANIFEST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/manifests
    OUT_DIR ${CMAKE_BINARY_DIR}/sf_apps/dummy-self-reconfig
)
