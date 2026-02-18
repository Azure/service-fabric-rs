#!/bin/bash
# Prepares apps for running unit tests.

MAX_RETRY=30
RETRY_INTERVAL=5

# --- Helper: retry a command with backoff ---
# Usage: retry <max_attempts> <delay_seconds> <description> <command...>
retry() {
    local max_attempts=$1
    local delay=$2
    local desc=$3
    shift 3
    local attempt=1
    while true; do
        echo "[${desc}] attempt #${attempt}/${max_attempts}"
        if "$@"; then
            echo "[${desc}] succeeded"
            return 0
        fi
        if [[ $attempt -ge $max_attempts ]]; then
            echo "[${desc}] failed after ${max_attempts} attempts" >&2
            return 1
        fi
        echo "[${desc}] failed, retrying in ${delay}s..."
        sleep "$delay"
        ((attempt++))
    done
}

# --- Step 1: Wait for cluster connection ---
retry $MAX_RETRY $RETRY_INTERVAL "cluster select" sfctl cluster select || exit 1

# --- Step 2: Wait for cluster to be healthy (system services including ImageStore) ---
# Helper that checks cluster health and succeeds only if Ok or Warning
check_cluster_healthy() {
    local health_output
    health_output=$(sfctl cluster health 2>&1) || return 1
    local health_state
    health_state=$(echo "$health_output" | python3 -c "import sys,json; print(json.load(sys.stdin).get('aggregatedHealthState','Unknown'))" 2>/dev/null || echo "Unknown")
    echo "Cluster health state: ${health_state}"
    [[ "$health_state" == "Ok" || "$health_state" == "Warning" ]]
}

echo "Waiting for cluster to become healthy..."
retry $MAX_RETRY $RETRY_INTERVAL "cluster health" check_cluster_healthy || exit 1
echo "Cluster is healthy"

# Give ImageStore a bit more time to stabilize after health reports Ok
sleep 5

# --- Step 3: Upload, provision, and create applications with retries ---
echo "Uploading applications"

retry $MAX_RETRY $RETRY_INTERVAL "upload echomain" \
    sfctl application upload --path build/sf_apps/samples_echomain --show-progress
retry $MAX_RETRY $RETRY_INTERVAL "provision echomain" \
    sfctl application provision --application-type-build-path samples_echomain
sfctl application create --app-name fabric:/EchoApp --app-type EchoApp --app-version 0.0.1

retry $MAX_RETRY $RETRY_INTERVAL "upload stateful2" \
    sfctl application upload --path build/sf_apps/samples_echomain_stateful2 --show-progress
retry $MAX_RETRY $RETRY_INTERVAL "provision stateful2" \
    sfctl application provision --application-type-build-path samples_echomain_stateful2
sfctl application create --app-name fabric:/StatefulEchoApp --app-type StatefulEchoApp --app-version 0.0.1

# --- Step 4: Wait for services to be resolvable ---
echo "Waiting for services to be resolvable..."
retry $MAX_RETRY $RETRY_INTERVAL "resolve EchoAppService" \
    sfctl service resolve --service-id EchoApp/EchoAppService
retry $MAX_RETRY $RETRY_INTERVAL "resolve StatefulEchoAppService" \
    sfctl service resolve --service-id StatefulEchoApp/StatefulEchoAppService

echo "All applications deployed and services resolved successfully"