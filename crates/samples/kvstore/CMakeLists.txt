# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_kvstore ALL
    COMMAND ${cargo_exe} build -p kvstore
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_root ${CMAKE_BINARY_DIR}/kvstore_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/kvstore)

set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/kvstore.exe)
include(PackageSF)
add_sf_app_pkg_simple(
    NAME kvstore
    MANIFEST_DIR ${_pkg_src}/manifests
    OUTDIR ${CMAKE_BINARY_DIR}/sf_apps/kvstore
    SVC_PKG_NAME KvStoreServicePackage
    CODE_ARTIFACTS ${_pkg_exe}
    DEPENDS build_rust_sample_kvstore
)