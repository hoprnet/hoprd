#!/usr/bin/env bash
# Fixture-only dashboard regression checks; no HOPR binaries or live chain.
set -Eeuo pipefail
trap 'echo "FAIL at line $LINENO: $BASH_COMMAND" >&2' ERR
repo=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
export PIX_DEMO_STATE_DIR="$work/state"
mkdir -p "$PIX_DEMO_STATE_DIR" "$work/bin"
source "$repo/localcluster/scripts/curvy-demo.sh"
export PATH="$work/bin:$PATH"
fail() {
  echo "FAIL: $*" >&2
  exit 1
}

# Reproduce a companion opened while the next launch is still pulling images.
# Run the real launcher through its first pull, with all Docker calls stubbed.
(
  export FIXTURE_DIR="$work"
  export HOPRD_BIN="$work/bin/prebuilt" HOPRD_PIX_SOAK_BIN="$work/bin/prebuilt"
  unset HOPRD_CHAIN_URL CURVY_LOCALCLUSTER_RELEASE
  fixture_platform=linux/amd64
  [[ $(uname -m) != aarch64 ]] || fixture_platform=linux/arm64
  jq --arg platform "$fixture_platform" '.platform = $platform' \
    "$repo/localcluster/curvy/release.json" >"$work/release.json"
  printf '#!/bin/sh\nexit 0\n' >"$HOPRD_BIN"
  chmod +x "$HOPRD_BIN"
  printf '#!/bin/sh\nexit 99\n' >"$work/bin/curl"
  chmod +x "$work/bin/curl"
  printf '1\n' >"$STATE_DIR/started"
  printf '2\n' >"$STATE_DIR/finished"
  printf '1\n' >"${CACHE}_run"
  printf 'previous run\n' >"${CACHE}_excerpt_3"
  mkdir -p "$work/old-logs"
  printf 'INFO curvy previous run\n' >"$work/old-logs/hoprd_3.log"
  touch -d @2 "$work/old-logs/hoprd_3.log"
  printf '%s\n' "$work/old-logs/hoprd_3.log" >"${CACHE}_logpath_3"
  cat >"$work/bin/docker" <<'EOF'
#!/usr/bin/env bash
case "$1" in
  info | compose) exit 0 ;;
  container) exit 1 ;;
  pull)
    printf '%s\n' "$CURVY_RUN_DIR" >"$FIXTURE_DIR/launcher-run-dir"
    [[ $(cat "$PIX_DEMO_STATE_DIR/started") == "$PIX_DEMO_RUN_STARTED" &&
       $PIX_DEMO_RUN_STARTED -gt 2 && ! -e $PIX_DEMO_STATE_DIR/finished ]] || exit 99
    touch "$FIXTURE_DIR/reset-before-pull"
    exit 1 ;;
  *) exit 99 ;;
esac
EOF
  chmod +x "$work/bin/docker"
  if bash "$repo/localcluster/scripts/curvy-localcluster.sh" --release "$work/release.json" >"$work/launcher.log" 2>&1; then
    fail 'launcher should stop at the stubbed pull failure'
  fi
  if [[ -f $work/launcher-run-dir ]]; then rm -rf "$(cat "$work/launcher-run-dir")"; fi
  [[ -f $work/reset-before-pull ]] || {
    cat "$work/launcher.log"
    fail 'run not reset before image pulls'
  }
  sync_run_cache
  [[ $ELAPSED -ge 0 && $ELAPSED -lt 60 && ! -e ${CACHE}_excerpt_3 && ! -e ${CACHE}_logpath_3 ]] ||
    fail 'previous run cache or elapsed time survived'
  LOG_COPY_DIR="$work/old-logs"
  node_pid() { :; }
  [[ $(node_log 3) == /dev/null && -z $(node_excerpt 3) ]] || fail 'previous node log reused during bootstrap'
)

# The timeout must wrap a real executable, in both host-RPC and Docker modes.
PIX_RPC_URL=http://fixture
[[ $(chain_exec -e TEST_VALUE=hello sh -c 'printf "%s:%s" "$RPC_URL" "$TEST_VALUE"') == http://fixture:hello ]] || fail 'host RPC wrapper'
if chain_exec --timeout 0.1 sleep 2; then fail 'timeout did not fire'; else [[ $? == 124 ]] || fail 'wrong timeout exit status'; fi
cat >"$work/bin/docker" <<'EOF'
#!/usr/bin/env bash
[[ $1 == exec ]] || exit 1
shift
vars=()
while [[ ${1:-} == -e ]]; do vars+=("$2"); shift 2; done
[[ $1 == hopr-chain ]] || exit 1
shift
exec env "${vars[@]}" "$@"
EOF
chmod +x "$work/bin/docker"
PIX_RPC_URL=
[[ $(chain_exec -e TEST_VALUE=docker sh -c 'printf "%s:%s" "$RPC_URL" "$TEST_VALUE"') == http://127.0.0.1:8545:docker ]] || fail 'Docker wrapper'

# A failed reader returning syntactically valid JSON must not overwrite good data.
good() { echo '[1]'; }
bad() {
  echo '[999]'
  return 1
}
empty() { echo '[]'; }
[[ $(cache_json example good) == '[1]' ]] || fail cache
[[ $(cache_json example bad) == '[1]' ]] || fail 'failed read replaced cache'
[[ $(cat "${CACHE}_example.status") == cached ]] || fail 'cached status'
[[ $(cache_json example empty) == '[]' ]] || fail 'valid empty result'
if cache_json missing bad; then fail 'missing source accepted'; fi
[[ $(cat "${CACHE}_missing.status") == unavailable ]] || fail 'unavailable status'

# Two allocations in the same block, a failed allocation, a batch-prover commit,
# a relayed withdrawal and a direct Safe shield. No old operator-mode log lines.
AGGREGATOR=0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
VAULT=0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
PORTAL=
SAFE[0]=0x1111111111111111111111111111111111111111
SAFE[3]=0x3333333333333333333333333333333333333333
ROWS="1 0xshield ${SAFE[0]} $VAULT 218000000000000000000
3 0xwithdraw $VAULT ${SAFE[3]} 21480920064000000000"
export FIXTURE_DIR="$work"
jq -n --arg agg "$AGGREGATOR" --arg node "${SAFE[0]}" '
  {transactions: [
    {hash:"0xalloc1",blockNumber:"0x1",from:"0xrelayer",to:$agg,input:"0x8d5626c0"},
    {hash:"0xalloc2",blockNumber:"0x1",from:"0xrelayer",to:$agg,input:"0x8d5626c0"},
    {hash:"0xfail",blockNumber:"0x1",from:"0xrelayer",to:$agg,input:"0x8d5626c0"},
    {hash:"0xshield",blockNumber:"0x1",from:$node,to:"0xmodule",input:"0x8d80ff0a"},
    {hash:"0xcommit",blockNumber:"0x1",from:"0xprover",to:$agg,input:"0x7fda4404"},
    {hash:"0xwithdraw",blockNumber:"0x1",from:"0xrelayer",to:$agg,input:"0x87aabf09"}
  ]}' >"$work/block.json"
jq '[.transactions[] | {transactionHash:.hash,gasUsed:"0x1234",status:(if .hash=="0xfail" then "0x0" else "0x1" end)}]' "$work/block.json" >"$work/receipts.json"
cat >"$work/bin/cast" <<'EOF'
#!/usr/bin/env bash
case " $* " in
  *' block-number '*) echo 1 ;;
  *' eth_getBlockByNumber '*) cat "$FIXTURE_DIR/block.json" ;;
  *' eth_getBlockReceipts '*) cat "$FIXTURE_DIR/receipts.json" ;;
  *) exit 1 ;;
esac
EOF
chmod +x "$work/bin/cast"
PIX_RPC_URL=http://fixture
PIX_CHAIN_FROM_BLOCK=1
GAS_SCAN_BUDGET=10
GAS_SCAN_SECS=3
gas_scan 10 3
[[ $(cat "${CACHE}_gas_next") == 2 ]] || fail 'scan progress'
TXS=$(curvy_txs)
[[ $(tx_count allocation) == 2 ]] || fail 'same-block allocations lost or revert counted'
[[ $(tx_count commit) == 1 ]] || fail 'batch-prover commitment missing'
[[ $(tx_count withdrawal) == 1 ]] || fail 'relayer withdrawal missing'
[[ $(tx_count shield) == 1 ]] || fail 'direct shield missing'
# Poll at an unchanged head must not duplicate cached transactions.
gas_scan 10 3
[[ $(wc -l <"${CACHE}_gas") == 6 ]] || fail 'duplicated transactions'
# Malformed receipt data must not advance or corrupt the cache.
echo 1 >"${CACHE}_gas_next"
echo '[]' >"$work/receipts.json"
gas_scan 10 3
[[ $(cat "${CACHE}_gas_next") == 1 ]] || fail 'advanced past missing receipts'
[[ $(wc -l <"${CACHE}_gas") == 6 ]] || fail 'partial scan appended'

# A shared vault must not be presented as a per-session link or anonymity proof.
LABEL=()
BOOTSTRAP=()
LABEL[$VAULT]='Curvy vault'
echo '[]' >"${CACHE}_transfers"
linkability "$ROWS" "${SAFE[0]}" "${SAFE[3]}"
verdict=$(render_verdict)
[[ $verdict == *POOL-MEDIATED* && $verdict != *UNLINKED* ]] || fail 'misleading privacy verdict'

# Render a complete frame with unavailable sources and no old log messages.
gather() { :; }
EXCERPT[0]='INFO hopr_strategy::pix::pools::curvy: shielding the Curvy funding note directly from the Safe'
EXCERPT[3]='INFO hopr_strategy::pix::pools::curvy: relayed a Curvy PIX withdrawal amount=21480920064000000000'
ELAPSED=30
POOL=curvy
CHAIN_ID=31337
BLOCK=1
FEE_IN=10
FEE_OUT=20
NOTES=
PENDING=0
COMMITTED=0
NULLIFIED=0
NOTE_ROWS=
INDEXED=()
EXIT_NOTES=()
# Avoid extra receipt requests: cached summaries still report the actual signer.
for h in alloc1 alloc2 commit withdraw shield; do
  printf '1 0xrelayer %s 100 1 4\n' "$AGGREGATOR" >"${CACHE}_tx_0x$h"
done
if ! frame=$(render); then fail "frame rendering"; fi
[[ $frame == *'2 txs'* && $frame == *'21.480920064'* ]] || fail 'settlement frame'
[[ $frame == *'note index unavailable'* && $frame != *'receipt pending'* ]] || fail 'unknown treated as zero/pending'
rm -f "${CACHE}_gas_next"
[[ $(tx_count allocation) == n/a ]] || fail 'missing scan reported as zero'

# Zero is an actual view tag, distinct from missing data, and does not establish
# whether the note is padding or belongs to the Exit.
NOTES='{"curvyPendingNotes":{"notes":[
  {"noteId":"0xzero","position":{"block":1},"isPlaintext":false,"amount":"123","viewTag":0},
  {"noteId":"0xmissing","position":{"block":1},"isPlaintext":false,"amount":"456"}
]}}'
NOTE_ROWS=$(note_rows)
note_display=$(render_notes)
[[ $note_display == *'view tag 0'* && $note_display == *'view tag n/a'* ]] || fail 'zero and missing tags conflated'

# Billing dimensions come from the current run; application bytes come from the
# latest echo report, never from multiplying node-wide packet counters.
(
  source "$repo/localcluster/scripts/pix-demo.sh"
  export LC_ALL=C
  geometry=$(render_geometry 128 108 54 21523968 0.000001 21.523968)
  [[ $geometry == *'20736 emissions × 1038 B'* && $geometry == *'21523968 B per SSA'* ]] || fail 'SSA byte factor missing'
  geometry=$(render_geometry 8 4 2 48000 0.01 480)
  [[ $geometry == *'48 emissions × 1000 B'* ]] || fail 'hardcoded SSA dimensions'
  printf 'INFO live sent_mb=12 echoed_mb=10\nINFO live sent_mb=490 echoed_mb=480\n' >"$TEST_LOG"
  accounting=$(render_traffic_accounting 10 21523968)
  [[ $accounting == *'215239680 B'* && $accounting == *'490 MB sent · 480 MB echoed'* ]] || fail 'billing/application volumes conflated'
  printf 'INFO PIX soak test PASSED sent_mb=500 echoed_mb=495\n' >>"$TEST_LOG"
  accounting=$(render_traffic_accounting 10 21523968)
  [[ $accounting == *'500 MB sent · 495 MB echoed'* ]] || fail 'final application reading ignored'
  : >"$TEST_LOG"
  accounting=$(render_traffic_accounting 10 21523968)
  [[ $accounting == *'unavailable'* ]] || fail 'missing application reading shown as zero'
)

# Visible pipeline names change while the existing metric label remains compatible.
grep -q '"Exit observed"' "$repo/localcluster/scripts/pix-demo.sh"
grep -q 'outcome="confirmed"' "$repo/localcluster/scripts/pix-demo.sh"
! grep -q '"Exit confirmed"' "$repo/localcluster/scripts/pix-demo.sh"
echo 'PASS: startup reset, RPC discovery, caching, shielding, billing/traffic units, view tags and unavailable sources'
