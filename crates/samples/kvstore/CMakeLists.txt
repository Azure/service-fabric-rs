# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_kvstore ALL
    COMMAND ${cargo_exe} build -p kvstore
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_root ${CMAKE_BINARY_DIR}/kvstore_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/kvstore)

set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/kvstore.exe)
add_custom_command(TARGET build_rust_sample_kvstore POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/KvStoreServicePackage/ServiceManifest.xml ${_pkg_root}/KvStoreServicePackage/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}/ApplicationManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_exe} ${_pkg_root}/KvStoreServicePackage/Code/kvstore.exe
)