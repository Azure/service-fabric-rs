function(add_sf_app_pkg)
    cmake_parse_arguments(
        SF_APP_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "NAME;MANIFEST_DIR;OUTDIR;DEPENDS" # list of names of mono-valued arguments
        #"SRCS;DEPS" # list of names of multi-valued arguments (output variables are lists)
        ""
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )

    if(NOT SF_APP_PKG_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_APP_PKG_NAME)

    if(NOT SF_APP_PKG_OUTDIR)
        message(FATAL_ERROR "OutDir param not found")
    endif(NOT SF_APP_PKG_OUTDIR)

    if(NOT SF_APP_PKG_MANIFEST_DIR)
        message(FATAL_ERROR "ManifestDir param not found")
    endif(NOT SF_APP_PKG_MANIFEST_DIR)

    # check that manifest dir exists
    if(NOT EXISTS ${SF_APP_PKG_MANIFEST_DIR})
        message(FATAL_ERROR "ManifestDir ${SF_APP_PKG_MANIFEST_DIR} does not exist")
    endif(NOT EXISTS ${SF_APP_PKG_MANIFEST_DIR})

    add_custom_target(sf_pkg_${SF_APP_PKG_NAME}
        # create the package dir
        COMMAND ${CMAKE_COMMAND} -E make_directory ${SF_APP_PKG_OUTDIR}
        # copy the app manifest file
        COMMAND ${CMAKE_COMMAND} -E copy ${SF_APP_PKG_MANIFEST_DIR}/ApplicationManifest.xml ${SF_APP_PKG_OUTDIR}
        DEPENDS ${SF_APP_PKG_DEPENDS}
    )

    set_target_properties(sf_pkg_${SF_APP_PKG_NAME} PROPERTIES
        SF_APP_NAME "${SF_APP_PKG_NAME}"
        SF_MANIFEST_DIR "${SF_APP_PKG_MANIFEST_DIR}"
        SF_OUTPUT_DIR "${SF_APP_PKG_OUTDIR}"
    )
endfunction(add_sf_app_pkg)

function(add_sf_svc_pkg)
    cmake_parse_arguments(
        SF_SVC_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "APP_NAME;SVC_PKG_NAME" # list of names of mono-valued arguments
        #"SRCS;DEPS" # list of names of multi-valued arguments (output variables are lists)
        ""
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )

    if(NOT SF_SVC_PKG_APP_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_SVC_PKG_APP_NAME)
    set(_app_target_name sf_pkg_${SF_SVC_PKG_APP_NAME})
    get_target_property(app_name ${_app_target_name} SF_APP_NAME)
    get_target_property(manifest_dir ${_app_target_name} SF_MANIFEST_DIR)
    get_target_property(output_dir ${_app_target_name} SF_OUTPUT_DIR)

    if(NOT SF_SVC_PKG_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_SVC_PKG_SVC_PKG_NAME)
    set(_svc_manifest_file ${manifest_dir}/${SF_SVC_PKG_SVC_PKG_NAME}/ServiceManifest.xml)
    set(_svc_manifest_out_file ${output_dir}/${SF_SVC_PKG_SVC_PKG_NAME}/ServiceManifest.xml)
    add_custom_command(
        TARGET ${_app_target_name} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy ${_svc_manifest_file} ${_svc_manifest_out_file}
        DEPENDS ${_svc_manifest_file}
    )
endfunction(add_sf_svc_pkg)

function(add_sf_code_pkg)
    cmake_parse_arguments(
        SF_CODE_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "APP_NAME;SVC_PKG_NAME;CODE_PKG_NAME" # list of names of mono-valued arguments
        "ARTIFACTS" # multi-valued arguements
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )
    if(NOT SF_CODE_PKG_APP_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_CODE_PKG_APP_NAME)
    if(NOT SF_CODE_PKG_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_CODE_PKG_SVC_PKG_NAME)
    if(NOT SF_CODE_PKG_CODE_PKG_NAME)
        set(SF_CODE_PKG_CODE_PKG_NAME "Code")
    endif(NOT SF_CODE_PKG_CODE_PKG_NAME)
    if(NOT SF_CODE_PKG_ARTIFACTS)
        message(FATAL_ERROR "Artifacts param not found")
    endif(NOT SF_CODE_PKG_ARTIFACTS)
    _add_sf_sub_folder(
        APP_NAME ${SF_CODE_PKG_APP_NAME}
        SVC_PKG_NAME ${SF_CODE_PKG_SVC_PKG_NAME}
        FOLDER_NAME ${SF_CODE_PKG_CODE_PKG_NAME}
        FOLDER_CONTENTS ${SF_CODE_PKG_ARTIFACTS}
    )
endfunction(add_sf_code_pkg)

function(add_sf_config_pkg)
    cmake_parse_arguments(
        SF_CONFIG_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "APP_NAME;SVC_PKG_NAME;CONFIG_PKG_NAME" # list of names of mono-valued arguments
        "" # multi-valued arguements
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )

    if(NOT SF_CONFIG_PKG_APP_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_CONFIG_PKG_APP_NAME)
    if(NOT SF_CONFIG_PKG_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_CONFIG_PKG_SVC_PKG_NAME)
    set(_app_target_name sf_pkg_${SF_CONFIG_PKG_APP_NAME})
    get_target_property(manifest_dir ${_app_target_name} SF_MANIFEST_DIR)

    if(NOT SF_CONFIG_PKG_CONFIG_PKG_NAME)
        set(SF_CONFIG_PKG_CONFIG_PKG_NAME "Config")
    endif(NOT SF_CONFIG_PKG_CONFIG_PKG_NAME)
    set(_config_src_dir ${manifest_dir}/${SF_CONFIG_PKG_SVC_PKG_NAME}/${SF_CONFIG_PKG_CONFIG_PKG_NAME})
    _add_sf_sub_folder(
        APP_NAME ${SF_CONFIG_PKG_APP_NAME}
        SVC_PKG_NAME ${SF_CONFIG_PKG_SVC_PKG_NAME}
        FOLDER_NAME ${SF_CONFIG_PKG_CONFIG_PKG_NAME}
        FOLDER_CONTENTS ${_config_src_dir}/Settings.xml
    )
endfunction(add_sf_config_pkg)

function(add_sf_data_pkg)
    cmake_parse_arguments(
        SF_DATA_PKG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "APP_NAME;SVC_PKG_NAME;DATA_PKG_NAME" # list of names of mono-valued arguments
        "ARTIFACTS" # multi-valued arguements
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )
    if(NOT SF_DATA_PKG_APP_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_DATA_PKG_APP_NAME)
    if(NOT SF_DATA_PKG_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_DATA_PKG_SVC_PKG_NAME)
    if(NOT SF_DATA_PKG_DATA_PKG_NAME)
        set(SF_DATA_PKG_DATA_PKG_NAME "Data")
    endif(NOT SF_DATA_PKG_DATA_PKG_NAME)
    if(NOT SF_DATA_PKG_ARTIFACTS)
        message(FATAL_ERROR "Artifacts param not found")
    endif(NOT SF_DATA_PKG_ARTIFACTS)
    _add_sf_sub_folder(
        APP_NAME ${SF_DATA_PKG_APP_NAME}
        SVC_PKG_NAME ${SF_DATA_PKG_SVC_PKG_NAME}
        FOLDER_NAME ${SF_DATA_PKG_DATA_PKG_NAME}
        FOLDER_CONTENTS ${SF_DATA_PKG_ARTIFACTS}
    )
endfunction(add_sf_data_pkg)

# Simple package with 1 service
function(add_sf_app_pkg_simple)
    cmake_parse_arguments(
        SF_APP_PKG_SIMPLE # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "NAME;MANIFEST_DIR;OUTDIR;SVC_PKG_NAME;CODE_PKG_NAME;CONFIG_PKG_NAME;DATA_PKG_NAME;DEPENDS" # list of names of mono-valued arguments
        "CODE_ARTIFACTS;DATA_ARTIFACTS" # multi-valued arguements
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )

    if(NOT SF_APP_PKG_SIMPLE_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_APP_PKG_SIMPLE_NAME)

    if(NOT SF_APP_PKG_SIMPLE_OUTDIR)
        message(FATAL_ERROR "OutDir param not found")
    endif(NOT SF_APP_PKG_SIMPLE_OUTDIR)
    if(NOT SF_APP_PKG_SIMPLE_MANIFEST_DIR)
        message(FATAL_ERROR "ManifestDir param not found")
    endif(NOT SF_APP_PKG_SIMPLE_MANIFEST_DIR)
    if(NOT SF_APP_PKG_SIMPLE_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_APP_PKG_SIMPLE_SVC_PKG_NAME)
    if(NOT SF_APP_PKG_SIMPLE_CODE_PKG_NAME)
        set(SF_APP_PKG_SIMPLE_CODE_PKG_NAME "Code")
    endif(NOT SF_APP_PKG_SIMPLE_CODE_PKG_NAME)
    # create the app package
    add_sf_app_pkg(
        NAME ${SF_APP_PKG_SIMPLE_NAME}
        MANIFEST_DIR ${SF_APP_PKG_SIMPLE_MANIFEST_DIR}
        OUTDIR ${SF_APP_PKG_SIMPLE_OUTDIR}
        DEPENDS ${SF_APP_PKG_SIMPLE_DEPENDS}
    )
    # create the service package
    add_sf_svc_pkg(
        APP_NAME ${SF_APP_PKG_SIMPLE_NAME}
        SVC_PKG_NAME ${SF_APP_PKG_SIMPLE_SVC_PKG_NAME}
    )
    # create the code package
    add_sf_code_pkg(
        APP_NAME ${SF_APP_PKG_SIMPLE_NAME}
        SVC_PKG_NAME ${SF_APP_PKG_SIMPLE_SVC_PKG_NAME}
        CODE_PKG_NAME ${SF_APP_PKG_SIMPLE_CODE_PKG_NAME}
        ARTIFACTS ${SF_APP_PKG_SIMPLE_CODE_ARTIFACTS}
    )
    # create the config package
    if (SF_APP_PKG_SIMPLE_CONFIG_PKG_NAME)
        add_sf_config_pkg(
            APP_NAME ${SF_APP_PKG_SIMPLE_NAME}
            SVC_PKG_NAME ${SF_APP_PKG_SIMPLE_SVC_PKG_NAME}
            CONFIG_PKG_NAME ${SF_APP_PKG_SIMPLE_CONFIG_PKG_NAME}
        )
    endif()
    # create the data package if artifacts are provided
    if(SF_APP_PKG_SIMPLE_DATA_PKG_NAME AND SF_APP_PKG_SIMPLE_DATA_ARTIFACTS)
        add_sf_data_pkg(
            APP_NAME ${SF_APP_PKG_SIMPLE_NAME}
            SVC_PKG_NAME ${SF_APP_PKG_SIMPLE_SVC_PKG_NAME}
            DATA_PKG_NAME ${SF_APP_PKG_SIMPLE_DATA_PKG_NAME}
            ARTIFACTS ${SF_APP_PKG_SIMPLE_DATA_ARTIFACTS}
        )
    endif()
endfunction(add_sf_app_pkg_simple)

# add a sub folder to the service package
function(_add_sf_sub_folder)
    cmake_parse_arguments(
        SF_FOLDER_ARG # prefix of output variables
        "" # list of names of the boolean arguments (only defined ones will be true)
        "APP_NAME;SVC_PKG_NAME;FOLDER_NAME" # list of names of mono-valued arguments
        "FOLDER_CONTENTS" # multi-valued arguements. Folder contents are optional
        ${ARGN} # arguments of the function to parse, here we take the all original ones
    )
    if(NOT SF_FOLDER_ARG_APP_NAME)
        message(FATAL_ERROR "Name param not found")
    endif(NOT SF_FOLDER_ARG_APP_NAME)
    set(_app_target_name sf_pkg_${SF_FOLDER_ARG_APP_NAME})
    get_target_property(app_name ${_app_target_name} SF_APP_NAME)
    get_target_property(manifest_dir ${_app_target_name} SF_MANIFEST_DIR)
    get_target_property(output_dir ${_app_target_name} SF_OUTPUT_DIR)
    if(NOT SF_FOLDER_ARG_SVC_PKG_NAME)
        message(FATAL_ERROR "SvcPkgName param not found")
    endif(NOT SF_FOLDER_ARG_SVC_PKG_NAME)
    if(NOT SF_FOLDER_ARG_FOLDER_NAME)
        message(FATAL_ERROR "FolderName param not found")
    endif(NOT SF_FOLDER_ARG_FOLDER_NAME)
    set(_folder_out_dir ${output_dir}/${SF_FOLDER_ARG_SVC_PKG_NAME}/${SF_FOLDER_ARG_FOLDER_NAME})
    add_custom_command(
        TARGET ${_app_target_name} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${_folder_out_dir}
    )
    if(SF_FOLDER_ARG_FOLDER_CONTENTS)
        foreach(_item IN LISTS SF_FOLDER_ARG_FOLDER_CONTENTS)
            # get the file name with extension and without path
            cmake_path(GET _item FILENAME _file_name)
            add_custom_command(
                TARGET ${_app_target_name} POST_BUILD
                COMMAND ${CMAKE_COMMAND} -E copy 
                    ${_item} ${_folder_out_dir}/${_file_name}
            )
        endforeach()
    endif()
endfunction(_add_sf_sub_folder)