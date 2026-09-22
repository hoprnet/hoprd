#!/usr/bin/env bash
# Pull a Curvy release stack and run the PIX soak with direct Safe shielding.
# No Nix, Cargo, Docker builds, or source checkouts are used at launch time.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CONFIG_DIR="$REPO_ROOT/localcluster/curvy"
die() {
  echo "curvy-localcluster: $*" >&2
  exit 1
}
RELEASE="${CURVY_LOCALCLUSTER_RELEASE:-$CONFIG_DIR/release.json}"
DASHBOARD=true
OFFLINE=false
while (($#)); do
  case "$1" in
  --release)
    (($# >= 2)) || die "--release needs a JSON file"
    RELEASE=$2
    shift 2
    ;;
  --no-dashboard)
    DASHBOARD=false
    shift
    ;;
  --offline)
    OFFLINE=true
    shift
    ;;
  *) die "usage: $0 [--release release.json] [--offline] [--no-dashboard]" ;;
  esac
done
[[ $(uname -s) == Linux ]] || die "the soak runner requires Linux (shared host loopback)"
[[ -z ${HOPRD_CHAIN_URL:-} ]] || die "unset HOPRD_CHAIN_URL; this runner owns a fresh chain"
[[ -f $RELEASE ]] || die "set --release or CURVY_LOCALCLUSTER_RELEASE to a published release manifest; see localcluster/curvy/README.md"
for cmd in docker curl jq sha256sum setsid flock; do
  command -v "$cmd" >/dev/null || die "$cmd is required"
done
if $DASHBOARD; then
  command -v bc >/dev/null || die "bc is required for the dashboard; or use --no-dashboard"
fi
missing=$(jq -r '.images | to_entries[] | select((.value // "") | startswith("REPLACE_WITH_")) | .key' "$RELEASE")
[[ -z $missing ]] || die "release needs published image digests for: $(echo "$missing" | tr '\n' ' '); update $RELEASE"
jq -e -f "$CONFIG_DIR/validate-release.jq" "$RELEASE" >/dev/null ||
  die "invalid release manifest: use published digests or an archive manifest with saved image IDs"
if [[ $(jq -r '.source // "registry"' "$RELEASE") == archive ]]; then
  $OFFLINE || die "archive releases require --offline after loading images.tar.gz"
elif $OFFLINE; then
  die "--offline requires the archive manifest supplied with the image bundle"
fi
# Read JSON as data, never as shell code or a sourced .env file.
read_release() { jq -er "$1" "$RELEASE"; }
export CURVY_PLATFORM="$(read_release .platform)"
case "$(uname -m):$CURVY_PLATFORM" in
x86_64:linux/amd64 | aarch64:linux/arm64) ;;
*) die "release platform $CURVY_PLATFORM does not match this host; use native images for proving" ;;
esac
export CURVY_CHAIN_IMAGE="$(read_release .images.chain)"
export CURVY_LOCALDB_IMAGE="$(read_release .images.localdb)"
export CURVY_PGDATA="$(read_release '.database.pgdata // "/var/lib/postgresql/data"')"
export CURVY_INDEXER_IMAGE="$(read_release .images.indexer)"
export CURVY_RELAYER_IMAGE="$(read_release .images.relayer)"
export CURVY_BATCH_PROVER_IMAGE="$(read_release .images.batch_prover)"
export CURVY_GATEWAY_IMAGE="$(read_release .images.gateway)"
export CURVY_PENDING_GRAPH="$(read_release .pending.graph)"
export CURVY_PENDING_GRAPH_SHA256="$(read_release .pending.graph_sha256)"
export CURVY_PENDING_ZKEY="$(read_release .pending.zkey)"
export CURVY_PENDING_ZKEY_SHA256="$(read_release .pending.zkey_sha256)"
[[ -x ${HOPRD_BIN:-} && -x ${HOPRD_PIX_SOAK_BIN:-} ]] ||
  die "set HOPRD_BIN and HOPRD_PIX_SOAK_BIN to the Linux-built executables"
export HOPRD_BIN="$(realpath "$HOPRD_BIN")"
export HOPRD_PIX_SOAK_BIN="$(realpath "$HOPRD_PIX_SOAK_BIN")"
docker info >/dev/null
docker compose version >/dev/null
if $OFFLINE; then
  bash "$CONFIG_DIR/verify-local-images.sh" "$RELEASE" || die "offline image verification failed"
fi
docker container inspect hopr-chain >/dev/null 2>&1 &&
  die "hopr-chain already exists; stop that cluster first"

RUN_DIR=$(mktemp -d /tmp/hopr-curvy.XXXXXX)
export CURVY_RUN_DIR="$RUN_DIR"
PROJECT="hopr-curvy-$(basename "$RUN_DIR" | tr '[:upper:].' '[:lower:]-')"
COMPOSE=(docker compose --env-file /dev/null --project-name "$PROJECT" -f "$CONFIG_DIR/compose.yml")
compose() { "${COMPOSE[@]}" "$@"; }
COMPOSE_STARTED=false
CHAIN_ID=""
ASSETS_ID=""
TEST_PID=""
cleanup() {
  local status=$?
  trap - EXIT INT TERM
  if [[ -n $TEST_PID ]]; then
    kill -TERM -- "-$TEST_PID" 2>/dev/null || true
    wait "$TEST_PID" 2>/dev/null || true
  fi
  if $COMPOSE_STARTED; then
    for service in chain db indexer relayer batch-prover gateway; do
      compose logs --no-color --no-log-prefix "$service" >"$RUN_DIR/$service.log" 2>&1 || true
    done
    compose down --volumes --timeout 15 >/dev/null 2>&1 || true
  fi
  [[ -z $ASSETS_ID ]] || docker rm -f "$ASSETS_ID" >/dev/null 2>&1 || true
  echo "curvy-localcluster: logs and release manifest saved to $RUN_DIR"
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
cp "$RELEASE" "$RUN_DIR/release.json"
mkdir -p "$RUN_DIR/keys" "$RUN_DIR/tmp"
# Deterministic, test-only service signers. Never exported to hoprd processes.
printf -v CURVY_RELAYER_KEY '0x%064x' $((0xc001))
printf -v CURVY_PROVER_KEY '0x%064x' $((0xc002))
export CURVY_RELAYER_KEY CURVY_PROVER_KEY
compose config --quiet
# Publish this run before pulling images or starting a new chain. The companion
# may already be open and must stop using the previous run's logs and caches.
export PIX_DEMO_STATE_DIR="${PIX_DEMO_STATE_DIR:-/tmp/pix-demo}"
mkdir -p "$PIX_DEMO_STATE_DIR"
[[ ! -L $PIX_DEMO_STATE_DIR && -d $PIX_DEMO_STATE_DIR && -O $PIX_DEMO_STATE_DIR ]] ||
  die "PIX_DEMO_STATE_DIR must be a directory owned by $(id -un) and not a symlink"
export PIX_DEMO_RUN_STARTED="$(date +%s)"
(
  flock -w 45 9 || die "dashboard cache is busy"
  rm -f "$PIX_DEMO_STATE_DIR/finished"
  printf '%s\n' "$PIX_DEMO_RUN_STARTED" >"$PIX_DEMO_STATE_DIR/started.tmp"
  mv "$PIX_DEMO_STATE_DIR/started.tmp" "$PIX_DEMO_STATE_DIR/started"
) 9>"$PIX_DEMO_STATE_DIR/.curvy.lock"
if ! $OFFLINE; then
  while IFS= read -r image; do
    echo "curvy-localcluster: pulling $image"
    docker pull --platform "$CURVY_PLATFORM" "$image" >>"$RUN_DIR/pull.log" 2>&1 ||
      die "cannot pull $image; see $RUN_DIR/pull.log"
  done < <(jq -r '.images | [.[] | select(. != null)] | unique[]' "$RELEASE")
fi

if jq -e '.images.artifacts != null' "$RELEASE" >/dev/null; then
  # Existing offline bundles carry the same proving files in an artifact image.
  # Dereference symlinks inside that image (including Nix store links).
  ASSETS_ID=$(docker create --pull never --platform "$CURVY_PLATFORM" --user "$(id -u):$(id -g)" \
    --volume "$RUN_DIR/keys:/export" --entrypoint /bin/sh \
    "$(read_release .images.artifacts)" -ec 'cp -RL "$1"/. /export/' sh \
    "$(read_release .artifacts_directory)")
  docker start -a "$ASSETS_ID" >"$RUN_DIR/artifacts.log" 2>&1 || die "artifact extraction failed"
  [[ $(docker inspect --format '{{.State.ExitCode}}' "$ASSETS_ID") == 0 ]] || die "artifact extraction failed"
else
  # Public files pinned to the same rs-sdk release as flake.nix. No local build.
  # Cache by checksum, rechecking each file before reuse.
  cache="${XDG_CACHE_HOME:-$HOME/.cache}/hopr/curvy-zk"
  mkdir -p "$cache"
  base_url=$(read_release .artifacts.base_url)
  while IFS=$'\t' read -r artifact checksum; do
    cached="$cache/$checksum"
    if ! printf '%s  %s\n' "$checksum" "$cached" | sha256sum -c - >/dev/null 2>&1; then
      echo "curvy-localcluster: downloading proving file $artifact"
      curl --fail --location --retry 3 --connect-timeout 20 \
        "$base_url/$artifact" --output "$RUN_DIR/keys/$artifact" >>"$RUN_DIR/artifacts.log" 2>&1 ||
        die "cannot download $artifact; see $RUN_DIR/artifacts.log"
      printf '%s  %s\n' "$checksum" "$RUN_DIR/keys/$artifact" | sha256sum -c - >/dev/null ||
        die "$artifact does not match the release manifest"
      cp "$RUN_DIR/keys/$artifact" "$cached"
    else
      cp "$cached" "$RUN_DIR/keys/$artifact"
    fi
  done < <(jq -r '.artifacts.files | to_entries[] | [.key, .value] | @tsv' "$RELEASE")
fi
# Both the host node and the container's prover user need read access.
chmod -R a+rX "$RUN_DIR/keys"
for kind in graph zkey; do
  artifact=$(read_release ".pending.$kind")
  checksum=$(read_release ".pending.${kind}_sha256")
  printf '%s  %s\n' "$checksum" "$RUN_DIR/keys/$artifact" | sha256sum -c - >/dev/null ||
    die "pending-note $kind does not match the release manifest"
done

COMPOSE_STARTED=true
compose up -d --no-build --pull never chain db
CHAIN_ID=$(compose ps -q chain)
export HOPRD_CHAIN_URL=http://127.0.0.1:8080
assert_running() {
  local service id
  for service in "$@"; do
    id=$(compose ps -a -q "$service")
    [[ -n $id && $(docker inspect --format '{{.State.Running}}' "$id") == true ]] ||
      die "$service exited; its log will be saved under $RUN_DIR"
  done
}
wait_http() {
  local url=$1 deadline=$((SECONDS + 180))
  shift
  until curl -fsS --max-time 3 "$url" >"$RUN_DIR/ready.json" 2>/dev/null; do
    assert_running "$@"
    ((SECONDS < deadline)) || die "readiness timed out: $url"
    sleep 2
  done
}
wait_http "$HOPRD_CHAIN_URL/readyz" chain
chain_cast() {
  docker exec -e FOUNDRY_DISABLE_NIGHTLY_WARNING=1 "$CHAIN_ID" \
    cast "$@" --rpc-url http://127.0.0.1:8545
}
[[ $(chain_cast chain-id) == 31337 ]] || die "expected local Anvil chain 31337"
AGGREGATOR=$(docker exec "$CHAIN_ID" cat /data/curvy_deployed_addresses.json |
  jq -er '.["CurvyAggregator#CurvyAggregatorAlphaV2"] // .["CurvyAggregator#ERC1967Proxy"]')
[[ $AGGREGATOR =~ ^0x[[:xdigit:]]{40}$ ]] || die "invalid Curvy aggregator address"
VAULT=$(chain_cast call "$AGGREGATOR" 'curvyVault()(address)')
# The Rust relayer adapter needs the collector's spend/view public keys from
# /protocol. Serve the upstream localnet identity without a metadata service;
# fail early if this chain was deployed with a different collector.
fee_key=$(jq -er '.data.feeCollector.babyJubjubPublicKey' "$CONFIG_DIR/protocol.json")
for coordinate in 0 1; do
  expected=$(cut -d . -f "$((coordinate + 1))" <<<"$fee_key")
  actual=$(chain_cast call "$AGGREGATOR" 'feeNotePublicKey(uint256)(uint256)' "$coordinate")
  [[ ${actual%% *} == "$expected" ]] || die "protocol.json fee collector does not match the chain"
done
OWNER=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
AUTHORITY=$(chain_cast call "$AGGREGATOR" 'AUTHORITY_ROLE()(bytes32)')
[[ $(chain_cast call "$AGGREGATOR" 'hasRole(bytes32,address)(bool)' "$AUTHORITY" "$OWNER") == true ]] ||
  die "local deployer has no authority on this Curvy deployment"
chain_cast send --unlocked --from "$OWNER" "$AGGREGATOR" \
  'setDirectShieldEnabled(bool)' true --json >"$RUN_DIR/enable-direct-shield.json"
[[ $(chain_cast call "$AGGREGATOR" 'directShieldEnabled()(bool)') == true ]] || die "direct shielding was not enabled"
OPERATOR_ROLE=$(chain_cast call "$AGGREGATOR" 'OPERATOR_ROLE()(bytes32)')
for service in relayer prover; do
  if [[ $service == relayer ]]; then key=$CURVY_RELAYER_KEY; else key=$CURVY_PROVER_KEY; fi
  address=$(docker exec "$CHAIN_ID" cast wallet address --private-key "$key")
  chain_cast send --unlocked --from "$OWNER" "$address" --value 10ether --json >"$RUN_DIR/fund-$service.json"
  chain_cast send --unlocked --from "$OWNER" "$AGGREGATOR" \
    'grantRole(bytes32,address)' "$OPERATOR_ROLE" "$address" --json >"$RUN_DIR/grant-$service.json"
  [[ $(chain_cast balance "$address") == 10000000000000000000 ]] || die "$service funding failed"
  [[ $(chain_cast call "$AGGREGATOR" 'hasRole(bytes32,address)(bool)' "$OPERATOR_ROLE" "$address") == true ]] ||
    die "$service role grant failed"
  echo "curvy-localcluster: $service signer $address funded and authorized"
done

# hopr-localcluster-pg already contains schemas/users. Configure the local
# fixture only on this run's fresh, private database volume when requested.
# Wait for TCP and verify the same credentials the services will use.
deadline=$((SECONDS + 180))
until compose exec -T db pg_isready -h 127.0.0.1 -U curvy -d curvy >/dev/null 2>&1; do
  assert_running db
  ((SECONDS < deadline)) || die "database initialization timed out"
  sleep 2
done
if [[ $(jq -r '.database.configure_localnet // false' "$RELEASE") == true ]]; then
  deployment=$(docker exec "$CHAIN_ID" cat /data/curvy_deployed_addresses.json)
  FACTORY=$(jq -er '.["PortalFactory#PortalFactory"]' <<<"$deployment")
  MULTICALL=$(jq -er '.["Devenv#Multicall3"]' <<<"$deployment")
  TOKEN=$(docker exec "$CHAIN_ID" cat /config.toml | sed -nE 's/^token *= *"(0x[[:xdigit:]]{40})".*/\1/p')
  for address in "$VAULT" "$FACTORY" "$MULTICALL" "$TOKEN"; do
    [[ $address =~ ^0x[[:xdigit:]]{40}$ ]] || die "invalid local fixture address: $address"
  done
  compose exec -T -e PGPASSWORD=curvy db \
    psql -X -v ON_ERROR_STOP=1 -h 127.0.0.1 -U curvy -d curvy \
    -v aggregator="$AGGREGATOR" -v vault="$VAULT" -v factory="$FACTORY" \
    -v multicall="$MULTICALL" -v token="$TOKEN" \
    -v graph="$CURVY_PENDING_GRAPH" -v graph_sha256="$CURVY_PENDING_GRAPH_SHA256" \
    -v zkey="$CURVY_PENDING_ZKEY" -v zkey_sha256="$CURVY_PENDING_ZKEY_SHA256" \
    <"$CONFIG_DIR/configure-localdb.sql" >"$RUN_DIR/configure-localdb.log"
fi
compose exec -T -e PGPASSWORD=curvy db \
  psql -X -v ON_ERROR_STOP=1 -h 127.0.0.1 -U curvy -d curvy -At \
  <"$CONFIG_DIR/check-localdb.sql" >"$RUN_DIR/localdb.json"
jq -e --arg aggregator "${AGGREGATOR,,}" --arg vault "${VAULT,,}" \
  --arg graph "$CURVY_PENDING_GRAPH_SHA256" --arg zkey "$CURVY_PENDING_ZKEY_SHA256" '
  .networks == 1 and (.aggregator | ascii_downcase) == $aggregator and
  (.vault | ascii_downcase) == $vault and .token_present and .schemas_present and
  .pending.batch_size == 5 and .pending.tree_depth == 30 and
  .pending.witness_graph_sha256 == $graph and .pending.zkey_sha256 == $zkey
' "$RUN_DIR/localdb.json" >/dev/null || die "localdb seed does not match this chain/artifact release; see $RUN_DIR/localdb.json"

compose up -d --no-build --pull never indexer relayer gateway
wait_http http://127.0.0.1:3000/protocol chain gateway
wait_http http://127.0.0.1:3000/relay-health chain relayer gateway
wait_http http://127.0.0.1:3000/sync/ready chain indexer gateway
jq -e '.chains | length == 1 and .[0].chainId == 31337 and .[0].ready == true' \
  "$RUN_DIR/ready.json" >/dev/null || die "indexer readiness did not confirm local chain 31337"
compose up -d --no-build --pull never batch-prover
deadline=$((SECONDS + 180))
# The tree is lazy: waiting for its first note here would block hoprd from
# starting and creating that note. The worker reports startup after connecting
# both databases and recovering interrupted batches.
until compose logs --no-color batch-prover | grep 'batch prover started' >/dev/null; do
  assert_running chain db indexer batch-prover
  ((SECONDS < deadline)) || die "batch prover initialization timed out"
  sleep 2
done

export PIX_POOL=curvy HOPRD_CURVY_SHIELDING=direct HOPRD_CURVY_SUBMISSION=relayer
export HOPRD_CURVY_RELAYER_URL=http://127.0.0.1:3000
export HOPRD_CURVY_NOTE_SOURCE=blokli HOPRD_CURVY_TOKEN=3
export HOPRD_CURVY_SCOPE_AGGREGATOR="$AGGREGATOR"
export CURVY_ZK_KEYS_DIR="$RUN_DIR/keys"
[[ -z ${PIX_DEMO_RATE:-} ]] || export HOPRD_PIX_SOAK_RATE="$PIX_DEMO_RATE"
[[ -z ${PIX_DEMO_FLOAT:-} ]] || export HOPRD_PIX_SOAK_FLOAT="$PIX_DEMO_FLOAT"
unset HOPRD_DEPLOYER_PRIVATE_KEY HOPRD_CURVY_OPERATOR_PRIVATE_KEY HOPRD_CURVY_OPERATOR_PRIVATE_KEYS
unset HOPRD_CURVY_INITIAL_FUNDING HOPRD_CURVY_SCOPE_RPC_URL
unset HOPRD_PIX_SOAK_POOL_PREFUNDED HOPRD_PIX_FLOAT_NODE_IDS

printf '#!/usr/bin/env bash\nunset CURVY_RELAYER_KEY CURVY_PROVER_KEY\ncd %q || exit $?\nexec sh %q\n' \
  "$RUN_DIR/tmp" "$CONFIG_DIR/run-soak.sh" >"$RUN_DIR/run-test.sh"
chmod +x "$RUN_DIR/run-test.sh"
export PIX_DEMO_TEST_RUNNER="$RUN_DIR/run-test.sh"
echo "curvy-localcluster: stack ready; direct Safe shielding, relayer submission, external batch proving"
echo "curvy-localcluster: node logs: /tmp/pix-soak-logs"
if $DASHBOARD; then
  setsid "$REPO_ROOT/localcluster/scripts/pix-demo.sh" &
else
  setsid "$PIX_DEMO_TEST_RUNNER" &
fi
TEST_PID=$!
status=0
wait "$TEST_PID" || status=$?
TEST_PID=""
# Keep the companion pane's final readings before removing the owned chain.
# Only needed if that pane was opened during this run.
if [[ -f ${PIX_DEMO_STATE_DIR:-/tmp/pix-demo}/curvy_run ]]; then
  bash "$REPO_ROOT/localcluster/scripts/curvy-demo.sh" --snapshot >"$RUN_DIR/dashboard-snapshot.log" 2>&1 ||
    echo "curvy-localcluster: final dashboard snapshot incomplete; see $RUN_DIR/dashboard-snapshot.log" >&2
fi
exit "$status"
