add_custom_target(build_rust_sample_echomain ALL
    COMMAND ${cargo_exe} build -p samples_echomain
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)
include(PackageSF)
add_sf_pkg(
    TARGET build_rust_sample_echomain
    EXECUTABLE ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain
    MANIFEST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/manifests
    OUT_DIR ${CMAKE_BINARY_DIR}/sf_apps/samples_echomain
)