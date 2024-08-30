# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_no_default_features ALL
    COMMAND ${cargo_exe} build -p no_default_features
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)