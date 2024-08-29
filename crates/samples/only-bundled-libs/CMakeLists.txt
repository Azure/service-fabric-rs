# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_only-bundled-libs ALL
    COMMAND ${cargo_exe} build -p only_bundled_libs
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_root ${CMAKE_BINARY_DIR}/only-bundled-libs_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/only-bundled-libs)
