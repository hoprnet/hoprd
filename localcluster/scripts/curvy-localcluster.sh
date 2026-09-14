#!/usr/bin/env bash
# Run the Curvy PIX soak test against a fresh Anvil chain with direct shielding.
# Usage: ./localcluster/scripts/curvy-localcluster.sh [--no-dashboard]
# Missing release binaries/proving artifacts are built with Nix. The chain is
# owned by this script; its direct-shield flag and each Safe's grant are set up
# before traffic starts. An existing chain is never reconfigured or removed.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$REPO_ROOT/localcluster/scripts/curvy-localcluster.sh"
if [[ -z ${IN_NIX_SHELL:-} ]]; then
  exec nix develop "$REPO_ROOT" -c "$SCRIPT" "$@"
fi
cd "$REPO_ROOT"

die() {
  echo "curvy-localcluster: $*" >&2
  exit 1
}
[[ $# == 0 || ($# == 1 && $1 == --no-dashboard) ]] ||
  die "usage: $SCRIPT [--no-dashboard]"
[[ -z ${HOPRD_CHAIN_URL:-} ]] ||
  die "this runner creates a fresh local chain; unset HOPRD_CHAIN_URL first"

for cmd in docker curl jq setsid; do
  command -v "$cmd" >/dev/null || die "$cmd is required"
done
docker info >/dev/null
if docker container inspect hopr-chain >/dev/null 2>&1; then
  die "hopr-chain already exists; stop that cluster before starting a new one"
fi

if [[ -z ${HOPRD_BIN:-} ]]; then
  [[ -x result-curvy/bin/hoprd ]] ||
    nix build -L .#binary-hoprd-pix-curvy-x86_64-linux --out-link result-curvy
  export HOPRD_BIN="$REPO_ROOT/result-curvy/bin/hoprd"
fi
if [[ -z ${CURVY_ZK_KEYS_DIR:-} ]]; then
  [[ -d result-curvy-keys/app/hoprd/curvy-zk-keys ]] ||
    nix build -L .#curvy-zk-artifacts --out-link result-curvy-keys
  export CURVY_ZK_KEYS_DIR="$REPO_ROOT/result-curvy-keys/app/hoprd/curvy-zk-keys"
fi
[[ -x $HOPRD_BIN ]] || die "HOPRD_BIN is not executable: $HOPRD_BIN"
[[ -d $CURVY_ZK_KEYS_DIR ]] || die "CURVY_ZK_KEYS_DIR is not a directory"

# The image's Curvy deployment defaults to directShieldEnabled=false. Granting
# the Safe the aggregator target is necessary too, but does not enable shielding.
CHAIN_IMAGE=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil@sha256:32714eddb075c7eb781a4e5cc75c0e42a24ab8aa891f4749f7f49c5dcf1570f8
docker pull --platform linux/amd64 "$CHAIN_IMAGE" >/dev/null
RUN_DIR=$(mktemp -d /tmp/hopr-curvy.XXXXXX)
CONTAINER_ID=""
TEST_PID=""
# Invoked by the EXIT trap, including after the signal traps call exit.
# shellcheck disable=SC2329
cleanup() {
  local status=$?
  trap - EXIT INT TERM
  if [[ -n $TEST_PID ]]; then
    # Both runner modes get their own process group, including all hoprd children.
    kill -TERM -- "-$TEST_PID" 2>/dev/null || true
    wait "$TEST_PID" 2>/dev/null || true
  fi
  if [[ -n $CONTAINER_ID ]]; then
    docker logs "$CONTAINER_ID" >"$RUN_DIR/chain.log" 2>&1 || true
    docker rm -f "$CONTAINER_ID" >/dev/null 2>&1 || true
  fi
  echo "curvy-localcluster: chain log saved to $RUN_DIR/chain.log"
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
CONTAINER_ID=$(docker run --detach --name hopr-chain --platform linux/amd64 \
  -p 127.0.0.1:8080:8080 "$CHAIN_IMAGE")
export HOPRD_CHAIN_URL=http://127.0.0.1:8080

echo "curvy-localcluster: waiting for the local chain"
deadline=$((SECONDS + 180))
until curl -fsS --max-time 2 "$HOPRD_CHAIN_URL/readyz" >/dev/null 2>&1; do
  [[ $(docker inspect --format '{{.State.Running}}' "$CONTAINER_ID") == true ]] ||
    die "chain container exited; inspect $RUN_DIR/chain.log"
  ((SECONDS < deadline)) || die "chain readiness timed out"
  sleep 2
done

chain_cast() {
  docker exec -e FOUNDRY_DISABLE_NIGHTLY_WARNING=1 "$CONTAINER_ID" \
    cast "$@" --rpc-url http://127.0.0.1:8545
}
[[ $(chain_cast chain-id) == 31337 ]] || die "expected the local Anvil chain"
AGGREGATOR=$(docker exec "$CONTAINER_ID" cat /data/curvy_deployed_addresses.json |
  jq -er '.["CurvyAggregator#CurvyAggregatorAlphaV2"]')
[[ $AGGREGATOR =~ ^0x[[:xdigit:]]{40}$ ]] || die "invalid Curvy aggregator address"
OWNER=$(chain_cast call "$AGGREGATOR" 'owner()(address)')
[[ ${OWNER,,} == 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 ]] ||
  die "unexpected Curvy owner on the local chain"
chain_cast send --unlocked --from "$OWNER" "$AGGREGATOR" \
  'setDirectShieldEnabled(bool)' true --json >"$RUN_DIR/enable-direct-shield.json"
[[ $(chain_cast call "$AGGREGATOR" 'directShieldEnabled()(bool)') == true ]] ||
  die "direct shielding was not enabled"

export PIX_POOL=curvy HOPRD_CURVY_SHIELDING=direct HOPRD_CURVY_SUBMISSION=operator
export HOPRD_CURVY_NOTE_SOURCE=blokli HOPRD_CURVY_TOKEN=3
export HOPRD_CURVY_SCOPE_AGGREGATOR="$AGGREGATOR"
[[ -z ${PIX_DEMO_RATE:-} ]] || export HOPRD_PIX_SOAK_RATE="$PIX_DEMO_RATE"
[[ -z ${PIX_DEMO_FLOAT:-} ]] || export HOPRD_PIX_SOAK_FLOAT="$PIX_DEMO_FLOAT"
# This is a fresh local chain: let the harness supply its deployer/operator keys
# and size the shield. No state or funding assumptions from an external chain apply.
unset HOPRD_DEPLOYER_PRIVATE_KEY HOPRD_CURVY_OPERATOR_PRIVATE_KEY
unset HOPRD_CURVY_INITIAL_FUNDING HOPRD_CURVY_SCOPE_RPC_URL
unset HOPRD_PIX_SOAK_POOL_PREFUNDED HOPRD_PIX_FLOAT_NODE_IDS
echo "curvy-localcluster: direct shielding enabled; the harness will grant each Safe access"

if [[ ${1:-} == --no-dashboard ]]; then
  setsid cargo nextest run -p hoprd-localcluster --test session_pix_soak \
    --run-ignored ignored-only -j 1 --no-capture &
else
  setsid "$REPO_ROOT/localcluster/scripts/pix-demo.sh" &
fi
TEST_PID=$!
status=0
wait "$TEST_PID" || status=$?
TEST_PID=""
exit "$status"
