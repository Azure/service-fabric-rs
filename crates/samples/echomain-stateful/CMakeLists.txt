# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_echomain_stateful ALL
    COMMAND ${cargo_exe} build -p samples_echomain_stateful
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_root_stateful ${CMAKE_BINARY_DIR}/echoapp_root_stateful)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echomain-stateful)

if(WIN32)
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful.exe)
else()
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful)
endif(WIN32)

add_custom_command(TARGET build_rust_sample_echomain_stateful POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root_stateful}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ServiceManifest.xml ${_pkg_root_stateful}/EchoAppStatefulServicePackage/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root_stateful}
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${_pkg_exe} ${_pkg_root_stateful}/EchoAppStatefulServicePackage/Code/echomainstateful.exe # note the rename
)