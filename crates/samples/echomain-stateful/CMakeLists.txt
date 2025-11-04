# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_echomain_stateful ALL
    COMMAND ${cargo_exe} build -p samples_echomain_stateful
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS build_fabric_rust_pal
)

set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/echomain-stateful)

if(WIN32)
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful.exe)
else()
    set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/samples_echomain_stateful)
endif(WIN32)

include(PackageSF)
add_sf_app_pkg_simple(
    TARGET build_rust_sample_echomain_stateful
    MANIFEST_DIR ${_pkg_src}/manifests
    OUTDIR ${CMAKE_BINARY_DIR}/sf_apps/echomain_stateful
    SVC_PKG_NAME EchoAppStatefulServicePackage
    CODE_ARTIFACTS ${_pkg_exe}
)