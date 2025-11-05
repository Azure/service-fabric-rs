# docker run --name sftestcluster -d -v /var/run/docker.sock:/var/run/docker.sock -p 19080:19080 -p 19000:19000 -p 25100-25200:25100-25200 mcr.microsoft.com/service-fabric/onebox:u18

sfctl application upload --path build/sf_apps/samples_echomain_stateful

sfctl application provision --application-type-build-path samples_echomain_stateful

sfctl application create --app-name fabric:/StatefulEchoApp --app-type StatefulEchoApp --app-version 0.0.1

sfctl service resolve --service-id StatefulEchoApp/StatefulEchoAppService

sfctl application delete --application-id StatefulEchoApp

sfctl application unprovision --application-type-name StatefulEchoApp --application-type-version  0.0.1

# Need to run inside the onebox container
# to test use netcat: nc localhost 3000