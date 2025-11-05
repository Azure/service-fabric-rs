function(add_sf_pkg)
    cmake_parse_arguments(
        SF_CODE_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "TARGET;EXECUTABLE;MANIFEST_DIR;OUT_DIR" # list of names of mono-valued arguments
        "" # multi-valued arguements
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )

    if(NOT SF_CODE_PKG_TARGET)
        message(FATAL_ERROR "add_sf_pkg: TARGET argument is required.")
    endif()
    if(NOT SF_CODE_PKG_EXECUTABLE)
        message(FATAL_ERROR "add_sf_pkg: EXECUTABLE argument is required.")
    endif()
    if(NOT SF_CODE_PKG_MANIFEST_DIR)
        message(FATAL_ERROR "add_sf_pkg: MANIFEST_DIR argument is required.")
    endif()
    if(NOT SF_CODE_PKG_OUT_DIR)
        message(FATAL_ERROR "add_sf_pkg: OUT_DIR argument is required.")
    endif()
    # find the service package name inside the app manifest.
    file(READ "${SF_CODE_PKG_MANIFEST_DIR}/ApplicationManifest.xml" MANIFEST_CONTENT)
    string(REGEX MATCH "ServiceManifestName=\"([^\"]+)\"" SERVICE_MANIFEST_NAME_MATCH "${MANIFEST_CONTENT}")
    if(NOT SERVICE_MANIFEST_NAME_MATCH)
        message(FATAL_ERROR "add_sf_pkg: Cannot find ServiceManifestName in ApplicationManifest.xml")
    endif()
    string(REGEX REPLACE "ServiceManifestName=\"([^\"]+)\"" "\\1" SERVICE_MANIFEST_NAME "${SERVICE_MANIFEST_NAME_MATCH}")

    # find the code directory inside the output directory.
    # it should be like: <OUT_DIR>/ServicePackageName/Code
    set(SF_CODE_PKG_CODE_DIR "${SF_CODE_PKG_OUT_DIR}/${SERVICE_MANIFEST_NAME}/Code")

    # add .exe extension for the executable if not exists.
    get_filename_component(EXE_NAME "${SF_CODE_PKG_EXECUTABLE}" NAME)
    if(NOT EXE_NAME MATCHES "\\.exe$")
        set(SF_CODE_PKG_EXECUTABLE_RENAMED "${SF_CODE_PKG_EXECUTABLE}.exe")
    else()
        set(SF_CODE_PKG_EXECUTABLE_RENAMED "${SF_CODE_PKG_EXECUTABLE}")
    endif()
    get_filename_component(SF_CODE_PKG_EXE_NO_PATH "${SF_CODE_PKG_EXECUTABLE_RENAMED}" NAME)

    add_custom_command(TARGET ${SF_CODE_PKG_TARGET} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${SF_CODE_PKG_OUT_DIR}
        COMMAND ${CMAKE_COMMAND} -E make_directory ${SF_CODE_PKG_CODE_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            ${SF_CODE_PKG_MANIFEST_DIR}
            ${SF_CODE_PKG_OUT_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy
            ${SF_CODE_PKG_EXECUTABLE}
            ${SF_CODE_PKG_CODE_DIR}/${SF_CODE_PKG_EXE_NO_PATH}
    )
endfunction(add_sf_pkg)