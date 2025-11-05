#!/bin/bash
# Prepares apps for running unit tests.

set +e # do not exit on error
counter=0
COMMAND_STATUS=1
until [ $COMMAND_STATUS -eq 0 ]; do
    echo "attempt #${counter}"
    sfctl cluster select
    COMMAND_STATUS=$?
    sleep 1
    let counter=counter+1
    if [[ $counter -eq 10 ]] ;
    then
    echo "Retry max reached" && exit 1
    fi
done
sfctl cluster health

sleep 10 # wait for cluster to be up
echo "Uploading applications"
sfctl application upload --path build/sf_apps/samples_echomain
sfctl application provision --application-type-build-path samples_echomain
sfctl application create --app-name fabric:/EchoApp --app-type EchoApp --app-version 0.0.1

sfctl application upload --path build/sf_apps/samples_echomain_stateful2
sfctl application provision --application-type-build-path samples_echomain_stateful2
sfctl application create --app-name fabric:/StatefulEchoApp --app-type StatefulEchoApp --app-version 0.0.1

sleep 10 # wait for services to be up
echo "Resolving services"
sfctl service resolve --service-id EchoApp/EchoAppService
sfctl service resolve --service-id StatefulEchoApp/StatefulEchoAppService