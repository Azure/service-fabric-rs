# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_echo2 ALL
    COMMAND ${cargo_exe} build -p samples_echo2
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)


set(_pkg_root ${CMAKE_BINARY_DIR}/echo2_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echo2)

set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echo2.exe)
add_custom_command(TARGET build_rust_sample_echo2 POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    # COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}/Code
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/EchoAppServicePackage2/ServiceManifest.xml ${_pkg_root}/EchoAppServicePackage2/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}/ApplicationManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_exe} ${_pkg_root}/EchoAppServicePackage2/Code/echo2.exe
)