# docker run --name sftestcluster -d -v /var/run/docker.sock:/var/run/docker.sock -p 19080:19080 -p 19000:19000 -p 25100-25200:25100-25200 mcr.microsoft.com/service-fabric/onebox:u18
sfctl cluster select --endpoint http://localhost:19080

sfctl application upload --path build/sf_apps/echomain

sfctl application provision --application-type-build-path echomain

sfctl application create --app-name fabric:/EchoApp --app-type EchoApp --app-version 0.0.1

sfctl service resolve --service-id EchoApp/EchoAppService

sfctl application delete --application-id EchoApp

sfctl application unprovision --application-type-name EchoApp --application-type-version  0.0.1

# Need to run inside the onebox container
# to test use netcat: nc localhost 3000