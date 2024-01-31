add_custom_target(build_rust_sample_echomain ALL
    COMMAND ${cargo_exe} build -p samples_echomain
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_root ${CMAKE_BINARY_DIR}/echoapp_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echomain)
if(WIN32)
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain.exe)
else()
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain)
endif(WIN32)
# shared files
add_custom_command(TARGET build_rust_sample_echomain POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_exe} ${_pkg_root}/EchoAppServicePackage/Code/echomain.exe # note the rename
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ServiceManifest.xml ${_pkg_root}/EchoAppServicePackage/ServiceManifest.xml
)