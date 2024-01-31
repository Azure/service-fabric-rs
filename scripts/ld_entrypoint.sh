#!/usr/bin/env bash

# entry point to set up ld path to be the current dir and then launch the exe.
# this was used to make *.so libs available to load in the code package.
# currently this is not used.

# use it in service manifest:
#     <EntryPoint>
#        <ExeHost>
#            <Program>ld_entrypoint.sh</Program>
#            <Arguments>-e echomain.exe</Arguments>
#            <ConsoleRedirection FileRetentionCount="5" FileMaxSizeInKb="2048"/>
#        </ExeHost>
#    </EntryPoint>

SCRIPT=$(realpath -s "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${SCRIPTPATH}"

exe=""

while getopts "h?e:" opt; do
  case "$opt" in
    h|\?)
      echo "ld_entrypoint.sh -e exe_name"
      exit 0
      ;;
    e)  exe=$OPTARG
      ;;
  esac
done

shift $((OPTIND-1))

[ "${1:-}" = "--" ] && shift

if [[ -z "$exe" ]]; then
    echo "e is empty."
    exit 1;
fi


echo "using exe ${exe} with ld path ${SCRIPTPATH}"
${SCRIPTPATH}/${exe}
exit $?