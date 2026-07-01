#!/bin/bash
# Deploy / run / clean up the dummy-self-reconfig sample on a local onebox.
#
# Mirrors scripts/reflection_ctl.sh and scripts/echomain2_ctl.sh, but for the
# self-reconfiguring service sample. Values come from the app's manifests:
#   - ApplicationTypeName    : DummySelfReconfigAppType   (ApplicationManifest.xml)
#   - ApplicationTypeVersion : 1.0
#   - Default service name   : DummySelfReconfigService   (DefaultServices)
# The package folder produced by `cmake --build build` is
# build/sf_apps/dummy-self-reconfig.

sfctl cluster select --endpoint http://localhost:19080

sfctl application upload --path build/sf_apps/dummy-self-reconfig --show-progress

sfctl application provision --application-type-build-path dummy-self-reconfig

sfctl application create --app-name fabric:/DummySelfReconfigApp --app-type DummySelfReconfigAppType --app-version 1.0

sfctl service resolve --service-id DummySelfReconfigApp/DummySelfReconfigService

# --- Cleanup (run to tear the app back down) ---
# sfctl application delete --application-id DummySelfReconfigApp
# sfctl application unprovision --application-type-name DummySelfReconfigAppType --application-type-version 1.0
