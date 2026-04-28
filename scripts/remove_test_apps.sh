#!/bin/bash
# Removes all test apps deployed by prepare_test_apps.sh.
#
# Inverse of prepare_test_apps.sh. Idempotent — every step tolerates
# "doesn't exist". Safe to run after a partial / failed prepare run.
#
# Order matters: we have to release any parked GrpcController gates
# *before* asking SF to delete services, otherwise close() will hang
# forever waiting for an Approve that will never come.
#
# Steps:
#   1. (Best-effort) Detach all GrpcController replicas via reflection_ctl
#      so any parked test-control gates auto-proceed. Skipped if the
#      binary doesn't exist or the cluster doesn't support Detach.
#   2. (Best-effort) approve-all as a fallback for older deployed
#      binaries that lack the Detach RPC.
#   3. Delete every service under each app instance (catches per-test
#      services like ApprovalE2e_<uuid>).
#   4. Delete app instances (fabric:/EchoApp, fabric:/ReflectionApp).
#   5. Unprovision app types (EchoApp@0.0.1, ReflectionApp@0.0.1).
#   6. Remove uploads from the image store.

MAX_RETRY=20
RETRY_INTERVAL=3

# --- Helper: retry a command with backoff ---
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

# --- Helper: best-effort wrapper ---
# Runs the command and reports the outcome but never fails the script.
best_effort() {
    local desc=$1
    shift
    if "$@"; then
        echo "[${desc}] ok"
    else
        echo "[${desc}] skipped (non-fatal)"
    fi
}

# --- Helper: check sfctl error code from a JSON-emitting failure ---
# Returns 0 if the output indicates "doesn't exist" (any of the SF
# enum values that map to "we already deleted it" / "never existed").
not_found_err() {
    grep -qE 'FABRIC_E_(APPLICATION|SERVICE|APPLICATION_TYPE)_(NOT_FOUND|DOES_NOT_EXIST)|FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR'
}

# --- Step 0: Wait for cluster ---
retry $MAX_RETRY $RETRY_INTERVAL "cluster select" sfctl cluster select || {
    echo "Cluster not reachable; nothing to clean up."
    exit 0
}

# --- Step 1: Detach all GrpcController replicas (if our CLI exists) ---
REFLECTION_CTL="${REFLECTION_CTL:-./target/debug/reflection_ctl}"
if [[ -x "$REFLECTION_CTL" ]]; then
    echo "Detaching all GrpcController replicas via $REFLECTION_CTL"
    best_effort "detach-all" "$REFLECTION_CTL" detach --all
else
    echo "[$REFLECTION_CTL] not found; skipping detach-all"
    echo "(Build it with: cargo build -p samples_reflection --bin reflection_ctl)"
fi

# --- Step 2: Best-effort approve-all (fallback for old deployed binary) ---
if [[ -x "$REFLECTION_CTL" ]]; then
    echo "Releasing any leftover gates via approve-all"
    best_effort "approve-all" "$REFLECTION_CTL" approve-all --yes
fi

# --- Step 3: Delete every service under each app ---
delete_services_under_app() {
    local app_id=$1
    local services_json
    services_json=$(sfctl service list --application-id "$app_id" 2>/dev/null) || {
        echo "[list services for $app_id] none / app missing"
        return 0
    }
    local service_ids
    service_ids=$(echo "$services_json" | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    for s in d.get('items', []):
        # service.name is 'fabric:/AppName/SvcName'; sfctl service-id wants
        # the path *without* 'fabric:/'.
        name = s.get('name', '')
        if name.startswith('fabric:/'):
            print(name[len('fabric:/'):])
except Exception:
    pass
")
    if [[ -z "$service_ids" ]]; then
        echo "[delete-services $app_id] no services"
        return 0
    fi
    while IFS= read -r svc; do
        [[ -z "$svc" ]] && continue
        echo "[delete-service $svc] deleting"
        # Force delete to avoid hanging when a replica is mid-close.
        # sfctl 11.x requires a value for --force-remove rather than
        # treating it as a bare flag.
        best_effort "delete-service $svc" \
            sfctl service delete --service-id "$svc" --force-remove true
    done <<< "$service_ids"
}

echo "Deleting services under fabric:/EchoApp"
delete_services_under_app "EchoApp"
echo "Deleting services under fabric:/ReflectionApp"
delete_services_under_app "ReflectionApp"

# --- Step 4: Delete the app instances ---
delete_app() {
    local app_id=$1
    local out
    out=$(sfctl application delete --application-id "$app_id" 2>&1)
    local rc=$?
    if [[ $rc -eq 0 ]]; then
        echo "[delete-app $app_id] deleted"
    elif echo "$out" | not_found_err; then
        echo "[delete-app $app_id] already absent"
    else
        echo "[delete-app $app_id] failed (will retry):"
        echo "$out" | tail -3
        return 1
    fi
}

retry $MAX_RETRY $RETRY_INTERVAL "delete EchoApp"        delete_app "EchoApp"        || true
retry $MAX_RETRY $RETRY_INTERVAL "delete ReflectionApp"  delete_app "ReflectionApp"  || true

# --- Step 5: Unprovision app types ---
unprovision_app_type() {
    local type_name=$1
    local version=$2
    local out
    out=$(sfctl application unprovision \
        --application-type-name "$type_name" \
        --application-type-version "$version" 2>&1)
    local rc=$?
    if [[ $rc -eq 0 ]]; then
        echo "[unprovision $type_name@$version] ok"
    elif echo "$out" | not_found_err; then
        echo "[unprovision $type_name@$version] already absent"
    else
        echo "[unprovision $type_name@$version] failed (will retry):"
        echo "$out" | tail -3
        return 1
    fi
}

retry $MAX_RETRY $RETRY_INTERVAL "unprovision EchoApp"       \
    unprovision_app_type EchoApp 0.0.1 || true
retry $MAX_RETRY $RETRY_INTERVAL "unprovision ReflectionApp" \
    unprovision_app_type ReflectionApp 0.0.1 || true

# --- Step 6: Remove uploaded packages from the image store ---
# `sfctl store delete` removes a relative path inside the image store.
# The path is the same one we passed to `application provision
# --application-type-build-path`.
remove_store_path() {
    local path=$1
    local out
    out=$(sfctl store delete --content-path "$path" 2>&1)
    local rc=$?
    if [[ $rc -eq 0 ]]; then
        echo "[store-delete $path] ok"
    else
        # store delete returns success even if the path didn't exist on
        # most SF versions; treat any non-zero as warn-only.
        echo "[store-delete $path] non-fatal:"
        echo "$out" | tail -3
    fi
}

best_effort "store-delete samples_echomain"    remove_store_path samples_echomain
best_effort "store-delete samples_reflection"  remove_store_path samples_reflection

echo
echo "All test apps removed (best-effort). Re-run prepare_test_apps.sh to redeploy."
