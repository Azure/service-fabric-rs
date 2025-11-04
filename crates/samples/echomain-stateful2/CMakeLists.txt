# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_echomain_stateful2 ALL
    COMMAND ${cargo_exe} build -p samples_echomain_stateful2
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echomain-stateful2)

if(WIN32)
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful2.exe)
else()
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful2)
endif(WIN32)

include(PackageSF)
add_sf_app_pkg_simple(
    NAME echomain_stateful2
    MANIFEST_DIR ${_pkg_src}/manifests
    OUTDIR ${CMAKE_BINARY_DIR}/sf_apps/echomain_stateful2
    SVC_PKG_NAME EchoAppStatefulServicePackage
    CODE_ARTIFACTS ${_pkg_exe}
    DEPENDS build_rust_sample_echomain_stateful2
)