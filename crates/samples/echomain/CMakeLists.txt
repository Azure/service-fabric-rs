add_custom_target(build_rust_sample_echomain ALL
    COMMAND ${cargo_exe} build -p samples_echomain
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echomain)
if(WIN32)
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain.exe)
else()
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain)
endif(WIN32)

include(PackageSF)
add_sf_app_pkg(
    NAME echomain
    MANIFEST_DIR ${_pkg_src}/manifests
    OUTDIR ${CMAKE_BINARY_DIR}/sf_apps/echomain
    DEPENDS build_rust_sample_echomain
)
add_sf_svc_pkg(
    APP_NAME echomain
    SVC_PKG_NAME EchoAppServicePackage
)
add_sf_code_pkg(
    APP_NAME echomain
    SVC_PKG_NAME EchoAppServicePackage
    ARTIFACTS ${_pkg_exe}
)
add_sf_config_pkg(
    APP_NAME echomain
    SVC_PKG_NAME EchoAppServicePackage
    CONFIG_PKG_NAME Config
)