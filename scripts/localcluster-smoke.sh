#!/usr/bin/env bash
#
# Smoke test: bring up a real local cluster and assert it reaches `running`.
#
# This is the cheapest end-to-end signal that a localcluster is usable at all: it
# exercises frozen identity generation, Safe deployment, pre-announcement, config
# generation, hoprd startup and channel opening against a real Anvil + Blokli.
# It deliberately stays at the minimum viable size — `/readyz` requires a network
# health above `Red` (see rest-api/src/checks.rs), so a single node would never
# become ready.
#
# Required:
#   LOCALCLUSTER_BIN  path to the hoprd-localcluster binary
#   HOPRD_BIN         path to the hoprd binary
#   CHAIN_IMAGE       Anvil + Blokli container image (pin by digest)
#
# Optional:
#   CLUSTER_SIZE        number of nodes (default 2, the minimum that can become ready)
#   CHANNEL_MANAGEMENT  channel mode passed to --channel-management (default api)
#   CONTAINER_RUNTIME   container CLI (default docker)
#   DATA_DIR            cluster data directory (default a fresh mktemp -d)
#   SMOKE_TIMEOUT       seconds to wait for state=running (default 600)
#   POLL_INTERVAL       seconds between status polls (default 5)

set -euo pipefail

CLUSTER_SIZE="${CLUSTER_SIZE:-2}"
CHANNEL_MANAGEMENT="${CHANNEL_MANAGEMENT:-api}"
CONTAINER_RUNTIME="${CONTAINER_RUNTIME:-docker}"
SMOKE_TIMEOUT="${SMOKE_TIMEOUT:-600}"
POLL_INTERVAL="${POLL_INTERVAL:-5}"

# EVM addresses of the frozen node identities, in order. Must match
# NODE_IDENTITIES in localcluster/src/identity.rs — asserting them here is what
# proves the running cluster really uses the frozen identities end to end.
FROZEN_NODE_ADDRESSES=(
  "0xb6624e92ee15dd38e2922379267ceffe002d4c46"
  "0x0d9c021d5c4de58cddc8853afc3e61f9955991a1"
  "0xc8beebf9df3ece584896cf82a2f1fe9f39c3c176"
  "0xc53bea21180d83a3e8f0d836c11c7f122eb5332b"
  "0x893809ff57c310561000ff17e2cd4862a020e959"
)

die() {
  echo "smoke: $*" >&2
  exit 1
}

log() {
  echo "smoke: $*"
}

for var in LOCALCLUSTER_BIN HOPRD_BIN CHAIN_IMAGE; do
  [[ -n ${!var:-} ]] || die "$var is not set"
done
[[ -x $LOCALCLUSTER_BIN ]] || die "LOCALCLUSTER_BIN '$LOCALCLUSTER_BIN' is not executable"
[[ -x $HOPRD_BIN ]] || die "HOPRD_BIN '$HOPRD_BIN' is not executable"
command -v jq >/dev/null || die "jq is required"
command -v "$CONTAINER_RUNTIME" >/dev/null || die "container runtime '$CONTAINER_RUNTIME' not found"
((CLUSTER_SIZE >= 2)) || die "CLUSTER_SIZE must be at least 2 (a single node never reaches ready)"
((CLUSTER_SIZE <= ${#FROZEN_NODE_ADDRESSES[@]})) ||
  die "CLUSTER_SIZE exceeds the ${#FROZEN_NODE_ADDRESSES[@]} frozen identities"

DATA_DIR="${DATA_DIR:-$(mktemp -d)}"
mkdir -p "$DATA_DIR"
CLUSTER_LOG="$DATA_DIR/localcluster.log"
cluster_pid=""

dump_logs() {
  echo "===== $CLUSTER_LOG =====" >&2
  tail -n 200 "$CLUSTER_LOG" >&2 2>/dev/null || echo "(no cluster log)" >&2
  for log_file in "$DATA_DIR"/logs/*; do
    [[ -f $log_file ]] || continue
    echo "===== $log_file (tail) =====" >&2
    tail -n 100 "$log_file" >&2
  done
}

# Always give the cluster a chance to tear its own chain container down; SIGKILL
# would leak the container and the node processes.
cleanup() {
  local status=$?
  if [[ -n $cluster_pid ]] && kill -0 "$cluster_pid" 2>/dev/null; then
    log "stopping cluster (pid $cluster_pid)"
    kill -TERM "$cluster_pid" 2>/dev/null || true
    for _ in $(seq 1 60); do
      kill -0 "$cluster_pid" 2>/dev/null || break
      sleep 1
    done
    kill -KILL "$cluster_pid" 2>/dev/null || true
  fi
  ((status == 0)) || dump_logs
  exit "$status"
}
trap cleanup EXIT INT TERM

status_json() {
  "$LOCALCLUSTER_BIN" status --data-dir "$DATA_DIR" 2>/dev/null || echo '{"state":"unavailable"}'
}

# Pre-pull so the image download does not eat into Blokli's readiness timeout.
log "pulling chain image $CHAIN_IMAGE"
"$CONTAINER_RUNTIME" pull --platform linux/amd64 "$CHAIN_IMAGE" >/dev/null

log "starting cluster: size=$CLUSTER_SIZE channels=$CHANNEL_MANAGEMENT data_dir=$DATA_DIR"
"$LOCALCLUSTER_BIN" \
  --size "$CLUSTER_SIZE" \
  --channel-management "$CHANNEL_MANAGEMENT" \
  --data-dir "$DATA_DIR" \
  --hoprd-bin "$HOPRD_BIN" \
  --chain-image "$CHAIN_IMAGE" \
  --container-runtime "$CONTAINER_RUNTIME" \
  >"$CLUSTER_LOG" 2>&1 &
cluster_pid=$!
log "cluster pid $cluster_pid, log $CLUSTER_LOG"

deadline=$((SECONDS + SMOKE_TIMEOUT))
state=""
while true; do
  if ! kill -0 "$cluster_pid" 2>/dev/null; then
    die "cluster process exited before reaching state=running"
  fi

  snapshot="$(status_json)"
  state="$(jq -r '.state' <<<"$snapshot")"

  case "$state" in
  running)
    log "cluster reached state=running after $SECONDS seconds"
    break
    ;;
  failed)
    die "cluster failed: $(jq -r '.error // "unknown error"' <<<"$snapshot")"
    ;;
  esac

  ((SECONDS < deadline)) ||
    die "timed out after ${SMOKE_TIMEOUT}s in state=$state (node states: $(jq -rc '[.nodes[]?.state]' <<<"$snapshot"))"

  sleep "$POLL_INTERVAL"
done

snapshot="$(status_json)"

node_count="$(jq -r '.nodes | length' <<<"$snapshot")"
[[ $node_count == "$CLUSTER_SIZE" ]] ||
  die "expected $CLUSTER_SIZE nodes in the status snapshot, got $node_count"

expected_node_state="channels_open"
[[ $CHANNEL_MANAGEMENT != "none" ]] || expected_node_state="ready"
while read -r id node_state; do
  [[ $node_state == "$expected_node_state" ]] ||
    die "node $id is in state '$node_state', expected '$expected_node_state'"
done < <(jq -r '.nodes[] | "\(.id) \(.state)"' <<<"$snapshot")

# The cluster must be running the frozen identities, in node order.
for ((id = 0; id < CLUSTER_SIZE; id++)); do
  actual="$(jq -r --argjson id "$id" '.nodes[] | select(.id == $id) | .address // ""' <<<"$snapshot" | tr '[:upper:]' '[:lower:]')"
  expected="${FROZEN_NODE_ADDRESSES[id]}"
  [[ $actual == "$expected" ]] ||
    die "node $id reported address '$actual', expected the frozen identity '$expected'"
done
log "all $CLUSTER_SIZE nodes are $expected_node_state with their frozen identities"

# A clean SIGTERM shutdown is part of the contract: localcluster must stop its
# nodes and remove the chain container, then exit 0.
log "requesting shutdown"
kill -TERM "$cluster_pid"
shutdown_status=0
wait "$cluster_pid" || shutdown_status=$?
cluster_pid=""
((shutdown_status == 0)) || die "cluster exited with status $shutdown_status after SIGTERM"

final_state="$(jq -r '.state' <<<"$(status_json)")"
[[ $final_state == "not_running" || $final_state == "unavailable" ]] ||
  die "cluster still reports state='$final_state' after shutdown"

log "PASSED"
