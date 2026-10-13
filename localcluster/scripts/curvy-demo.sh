#!/usr/bin/env bash
#
# The Curvy side of a PIX run, as the chain sees it — a companion dashboard to pix-demo.sh.
#
# pix-demo.sh shows the Session: traffic, SSA cycles, money arriving in the Exit's Safe. It says
# nothing about *how* the money got there, and with the Curvy pool that is the whole point: the
# Entry never pays the Exit. It shields a note into the Curvy vault, allocates value to the Exit's
# one-time scan key inside a zk proof, and the Exit withdraws against the vault's Merkle root with
# another proof. This dashboard puts the three views of that side by side:
#
#   * what the Entry did, from its own log              — private to the Entry
#   * what the Exit did, from its own log               — private to the Exit
#   * what anyone can see, straight from the node's RPC: the Curvy transactions themselves
#     (sender, contract, proof bytes, events), the notes announced into the vault (an id, a view
#     tag, a ciphertext — no owner), the wxHOPR `Transfer` events, labelled — and the linkability
#     question asked of them: which addresses did the Entry pay, which paid the Exit, and do the
#     two sets meet?
#
# `--ledger` adds the bill: every transaction on the chain priced from its receipt, grouped by
# who paid the gas and what the call did (a Safe module call is named by what the Safe then
# did), with the average per action and what one PIX deposit costs the pool.
#
# The transfer view distinguishes per-session intermediaries from the shared Curvy vault.
# Direct shielding makes the vault visible on both sides. That is not evidence of which
# shielded note was spent, and this small local run is not a demonstration of anonymity.
#
# It attaches to the cluster pix-demo.sh runs; it starts nothing itself:
#
#   ./localcluster/scripts/pix-demo.sh                                  # pane 1
#   ./localcluster/scripts/curvy-demo.sh                                # pane 2, refreshes until Ctrl-C
#   watch -n 2 -c ./localcluster/scripts/curvy-demo.sh --dashboard      # or under watch(1)
#   ./localcluster/scripts/curvy-demo.sh --ledger                       # every transfer, the verdict, the gas bill, once
#   ./localcluster/scripts/curvy-demo.sh --rpc                          # every Curvy tx and note, once
#
# Sources: Blokli's GraphQL API on :8080 (the deployment's contract addresses, vault fees,
# aggregator state, the indexed notes and nullifiers); `cast` inside the chain container (the raw
# `Transfer` log, every block's transactions and receipts straight out of anvil — the one source
# that owes nothing to HOPR or Curvy code);
# the nodes' logs (found through each hoprd's stdout while it runs, `/tmp/pix-soak-logs` after);
# and the REST API for node addresses.
#
# Every reading is cached under the PIX_DEMO_STATE_DIR pix-demo.sh uses, so the closing frame —
# and `--ledger` after the run — still show the run's figures once pix-demo has torn the chain
# container down. The cache is dropped when pix-demo starts a new run.
#
# Requires Bash 4+, curl, jq, bc, docker, timeout and flock; run inside `nix develop`.

set -uo pipefail

API_PORT_BASE=13500
# Same index → role mapping as pix-demo.sh and `session_pix_soak`.
ENTRY_IDX=0
RELAY_IDXS=(1 2)
EXIT_IDX=3
ALL_IDXS=("$ENTRY_IDX" "${RELAY_IDXS[@]}" "$EXIT_IDX")
: "${PIX_DEMO_STATE_DIR:=/tmp/pix-demo}"
STATE_DIR="$PIX_DEMO_STATE_DIR"
: "${PIX_BLOKLI_URL:=http://127.0.0.1:8080}"
: "${PIX_CHAIN_CONTAINER:=hopr-chain}"
# Where `cast` reads the chain from. Unset: inside the Anvil container, as before. Set to a JSON-RPC
# URL (a real chain): `cast` runs on this host against it, and the container is not consulted.
: "${PIX_RPC_URL:=}"
# First block the raw-log scans look at. Anvil starts at 0; a real chain wants the cluster's start.
: "${PIX_CHAIN_FROM_BLOCK:=0}"
# Run `cast` (or a shell script that calls it) where the chain is: on the host against PIX_RPC_URL,
# or inside the chain container against its Anvil. Extra `-e VAR=val` args precede the command.
chain_exec() {
  local seconds=15
  if [ "${1:-}" = --timeout ]; then
    seconds=$2
    shift 2
  fi
  local envs=()
  while [ "${1:-}" = "-e" ]; do
    envs+=("$2")
    shift 2
  done
  if [ -n "$PIX_RPC_URL" ]; then
    timeout "$seconds" env FOUNDRY_DISABLE_NIGHTLY_WARNING=1 RPC_URL="$PIX_RPC_URL" "${envs[@]}" "$@"
  else
    local dargs=(-e FOUNDRY_DISABLE_NIGHTLY_WARNING=1 -e RPC_URL=http://127.0.0.1:8545)
    local e
    for e in "${envs[@]}"; do dargs+=(-e "$e"); done
    timeout "$seconds" docker exec "${dargs[@]}" "$PIX_CHAIN_CONTAINER" "$@"
  fi
}
LOG_COPY_DIR=/tmp/pix-soak-logs
REFRESH=2
# Anvil's account 0 is the deployer/faucet. Relayer and prover may use other signers.
DEPLOYER=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
# Prefix of every file this script writes. Kept apart from pix-demo's own cache families, which
# that script clears by name.
CACHE="$STATE_DIR/curvy"
# bc wraps long numbers at 70 columns; nothing here wants a backslash inside a number.
export BC_LINE_LENGTH=0

C_RESET=$'\033[0m'
C_DIM=$'\033[2m'
C_BOLD=$'\033[1m'
C_GREEN=$'\033[32m'
C_YELLOW=$'\033[33m'
C_RED=$'\033[31m'
C_MAGENTA=$'\033[35m'

# ── small helpers ───────────────────────────────────────────────────────────────

strip_ansi() { sed 's/\x1b\[[0-9;]*m//g'; }
lower() { printf '%s' "${1:-}" | tr '[:upper:]' '[:lower:]'; }
short() {
  local a=${1:-}
  if [ ${#a} -gt 14 ]; then printf '%s…%s' "${a:0:6}" "${a: -4}"; else printf '%s' "$a"; fi
}
# Trim bc's trailing zeroes and give a bare ".5" its leading zero.
trim() { printf '%s' "${1:-0}" | sed -e 's/\(\.[0-9]*[1-9]\)0*$/\1/' -e 's/\.0*$//' -e 's/^\./0./' -e 's/^-\./-0./'; }
hex2dec() {
  local h=${1#0x}
  h=$(printf '%s' "$h" | tr '[:lower:]' '[:upper:]' | sed 's/^0*//')
  [ -z "$h" ] && h=0
  echo "ibase=16; $h" | bc
}
wei2hopr() { trim "$(echo "scale=18; ${1:-0} / 1000000000000000000" | bc)"; }
sum() { paste -sd+ - | sed 's/^$/0/' | bc; }

# ── cached readers ──────────────────────────────────────────────────────────────
#
# Each source below is read fresh every frame and kept only if the read succeeded, otherwise the
# last good reading stands. That is what keeps the closing frame legible: pix-demo removes the
# chain container the moment the test exits, and the nodes with it.

# cache <name> <command...>: run the command; a non-empty result replaces the cache, an empty one
# yields the cached value.
cache() {
  local file="${CACHE}_$1" out
  shift
  out=$("$@" 2>/dev/null)
  if [ -n "$out" ]; then
    printf '%s\n' "$out" >"$file"
  elif [ -r "$file" ]; then
    cat "$file"
    return 0
  fi
  printf '%s\n' "$out"
}
# Same, for JSON: a truncated response is not a reading.
cache_json() {
  local file="${CACHE}_$1" out
  shift
  if out=$("$@" 2>"$file.error") && [ -n "$out" ] && printf '%s' "$out" | jq -e . >/dev/null 2>&1; then
    printf '%s\n' "$out" >"$file"
    date +%s >"$file.at"
    printf 'live\n' >"$file.status"
  elif [ -r "$file" ]; then
    printf 'cached\n' >"$file.status"
    cat "$file"
    return 0
  else
    printf 'unavailable\n' >"$file.status"
    return 1
  fi
  printf '%s\n' "$out"
}

gql() {
  curl -s --max-time 4 "$PIX_BLOKLI_URL/graphql" -H 'content-type: application/json' \
    -d "{\"query\":$(jq -Rn --arg q "$1" '$q')}" 2>/dev/null
}

node_pid() { pgrep -f "hoprd .*--apiPort[ ]$((API_PORT_BASE + $1))" | head -1; }

# The node's log file: the file its stdout points at while it runs (a temp directory chosen by
# the test), the same path while that still exists, and the copy the test leaves under
# $LOG_COPY_DIR once it has finished — but only a copy written after this run started. Until
# then that directory holds the *previous* run's logs, whose transactions this chain has never
# seen.
node_log() {
  local cache="${CACHE}_logpath_$1" pid path="" copy="$LOG_COPY_DIR/hoprd_$1.log"
  pid=$(node_pid "$1")
  [ -n "$pid" ] && path=$(readlink "/proc/$pid/fd/1" 2>/dev/null)
  if [ -n "$path" ] && [ -r "$path" ]; then
    printf '%s\n' "$path" >"$cache"
    printf '%s\n' "$path"
    return
  fi
  path=$(cat "$cache" 2>/dev/null)
  if [ -n "$path" ] && [ -r "$path" ]; then
    printf '%s\n' "$path"
    return
  fi
  local started
  started=$(cat "$STATE_DIR/started" 2>/dev/null || echo 0)
  if [ -r "$copy" ] && [ "$(stat -c %Y "$copy" 2>/dev/null || echo 0)" -ge "$started" ]; then
    printf '%s\n' "$copy"
  else
    printf '/dev/null\n'
  fi
}

# The Curvy-relevant lines of one node's log, colour stripped. One grep per node per frame, and
# every field below is a cheap second pass over this excerpt.
node_excerpt() {
  local log
  log=$(node_log "$1")
  cache "excerpt_$1" sh -c "grep -a -e curvy -e 'safe_address' -e 'enabling the PIX strategy' '$log' | sed 's/\x1b\[[0-9;]*m//g'"
}

node_safe() { grep -o 'safe_address\\":\\"0x[0-9a-fA-F]*' <<<"$2" | head -1 | grep -o '0x.*'; }
node_module() { grep -o 'module_address\\":\\"0x[0-9a-fA-F]*' <<<"$2" | head -1 | grep -o '0x.*'; }

node_address() {
  cache "addr_$1" sh -c "curl -s --max-time 3 http://127.0.0.1:$((API_PORT_BASE + $1))/api/v4/account/addresses | jq -r '.native // empty'"
}

# The deployment's contract addresses, as Blokli hands them to every node.
contracts() {
  cache_json contracts sh -c "$(declare -f gql); PIX_BLOKLI_URL='$PIX_BLOKLI_URL' gql '{ chainInfo { ... on ChainInfo { chainId blockNumber contractAddresses } } }' | jq -e '.data.chainInfo | select(.contractAddresses != null) | {chainId, blockNumber, contracts: (.contractAddresses | fromjson)}'"
}

vault_state() {
  cache_json vault sh -c "$(declare -f gql); PIX_BLOKLI_URL='$PIX_BLOKLI_URL' gql '{ curvyVaultFees { ... on CurvyVaultFees { depositFee withdrawalFee } } curvyAggregatorState { ... on CurvyAggregatorState { notesTreeRoot notesBatchIndex nullifiersBatchIndex noteIndex } } }' | jq -e '.data | select(.curvyAggregatorState.notesTreeRoot != null)'"
}

notes() {
  cache_json notes sh -c "$(declare -f gql); PIX_BLOKLI_URL='$PIX_BLOKLI_URL' gql '{ curvyPendingNotes(fromBlock: 0, first: 1000) { ... on CurvyPendingNotes { notes { noteId amount viewTag isPlaintext position { block transactionHash } } } } curvyCommittedNotes(fromBlock: 0, first: 1000) { ... on CurvyCommittedNotes { notes { noteId leafIndex position { block transactionHash } } } } curvyCommittedNullifiers(fromBlock: 0, first: 1000) { ... on CurvyCommittedNullifiers { nullifiers { nullifier position { block transactionHash } } } } }' | jq -e '.data | select(.curvyPendingNotes.notes != null)'"
}

# Every wxHOPR `Transfer` since genesis, read from anvil through the `cast` that ships in the chain
# image. Raw topics and data: the token address is the only thing taken from Blokli.
#
# Re-read at most every $LEDGER_REFRESH seconds rather than every frame: the read is an
# `eth_getLogs` over the whole chain against the same single anvil the cluster's own connectors
# are talking to, and a PIX transfer lands once every several seconds at best.
LEDGER_REFRESH=6
transfers() {
  local token=$1 stamp="${CACHE}_transfers_at" now
  now=$(date +%s)
  if [ -z "$token" ] || [ $((now - $(cat "$stamp" 2>/dev/null || echo 0))) -lt "$LEDGER_REFRESH" ]; then
    [ -r "${CACHE}_transfers" ] && cat "${CACHE}_transfers"
    return 0
  fi
  printf '%s\n' "$now" >"$stamp"
  cache_json transfers chain_exec sh -c \
    'cast logs --rpc-url "$RPC_URL" --from-block "$0" --address "$1" "Transfer(address indexed from, address indexed to, uint256 value)" --json' \
    "$PIX_CHAIN_FROM_BLOCK" "$token"
}

# ── the Curvy transactions, as the RPC returns them ─────────────────────────────

# Successful Curvy transactions, discovered from RPC calldata/receipts, regardless of signer.
# Direct shielding is a nested Safe call: identify its token transfer into the vault.
# One row per transaction: `block kind hash`. No dependency on operator-mode node logs.
curvy_txs() {
  [ -f "${CACHE}_gas_next" ] || return 0
  awk -v aggregator="$AGGREGATOR" -v vault="$VAULT" -v entry="${SAFE[$ENTRY_IDX]}" -v portal="$PORTAL" '
    FILENAME == ARGV[1] { if ($4 == vault && (($3 == entry && entry != "") || ($3 == portal && portal != ""))) shield[$2] = 1; next }
    $6 == 1 {
      kind = ""
      if ($4 == aggregator && aggregator != "") {
        if ($7 == "0x8d5626c0") kind = "allocation"
        else if ($7 == "0x7fda4404") kind = "commit"
        else if ($7 == "0x87aabf09") kind = "withdrawal"
      }
      if ($2 in shield) kind = "shield"
      if (kind != "") print $1, kind, $2
    }
  ' <(printf '%s\n' "$ROWS") "${CACHE}_gas" | sort -k1,1n -k3,3 -u
}

# Receipt and calldata of one transaction, summarised: `block from to gas status calldata_bytes`
# then one `log <address> <topic0>` line per event. Immutable once mined, so cached for good; at
# most $RPC_FETCH_BUDGET transactions are fetched per frame so a frame never stalls on the RPC.
RPC_FETCH_BUDGET=3
tx_summary() {
  local hash=$1 file="${CACHE}_tx_$1"
  if [ ! -s "$file" ] && [ "$RPC_FETCH_BUDGET" -gt 0 ]; then
    RPC_FETCH_BUDGET=$((RPC_FETCH_BUDGET - 1))
    # `--async`: `cast receipt` otherwise waits for the transaction to be mined, which a hash the
    # chain has never seen (a log line from a previous run) turns into a hang.
    chain_exec sh -c \
      "cast receipt $hash --async --rpc-url \"\$RPC_URL\" --json; cast tx $hash --rpc-url \"\$RPC_URL\" --json" 2>/dev/null |
      jq -rs '
        def hex: ltrimstr("0x") | explode | map(if . >= 97 then . - 87 elif . >= 65 then . - 55 else . - 48 end) | reduce .[] as $d (0; . * 16 + $d);
        .[0] as $r | (.[1].data // .[1]) as $t
        | select($r.blockNumber != null)
        | "\($r.blockNumber | hex) \($r.from | ascii_downcase) \($r.to // "-" | ascii_downcase) \($r.gasUsed | hex) \($r.status | hex) \((($t.input // "0x") | length - 2) / 2)",
          ($r.logs[] | "log \(.address | ascii_downcase) \(.topics[0])")
      ' >"$file.tmp" 2>/dev/null
    if [ -s "$file.tmp" ]; then mv "$file.tmp" "$file"; else rm -f "$file.tmp"; fi
  fi
  [ -s "$file" ] && cat "$file"
  return 0
}

# What Blokli's index says each transaction did to the vault: `hash pending committed nullifiers`.
tx_index_counts() {
  [ -n "$NOTES" ] || return 0
  jq -r '
    [ (.curvyPendingNotes.notes[] | {h: .position.transactionHash, k: "p"}),
      (.curvyCommittedNotes.notes[] | {h: .position.transactionHash, k: "c"}),
      (.curvyCommittedNullifiers.nullifiers[] | {h: .position.transactionHash, k: "n"}) ]
    | group_by(.h)[]
    | "\(.[0].h | ascii_downcase) \(map(select(.k == "p")) | length) \(map(select(.k == "c")) | length) \(map(select(.k == "n")) | length)"
  ' <<<"$NOTES" 2>/dev/null
}

# The notes announced into the vault, newest last: `block noteId plaintext amount viewTag`.
note_rows() {
  [ -n "$NOTES" ] || return 0
  jq -r '.curvyPendingNotes.notes | sort_by(.position.block | tonumber)[]
    | "\(.position.block) \(.noteId | ascii_downcase) \(.isPlaintext) \(.amount) \(.viewTag)"' <<<"$NOTES" 2>/dev/null
}

# ── the ledger ──────────────────────────────────────────────────────────────────

declare -A LABEL
label() {
  local a
  a=$(lower "$1")
  printf '%s' "${LABEL[$a]:-$(short "$a")}"
}

# One line per transfer: `block tx from to wei`, chronological.
ledger_rows() {
  local json=$1
  [ -n "$json" ] || return 0
  printf '%s' "$json" |
    jq -r '.[] | [.blockNumber, .transactionHash, ("0x" + .topics[1][26:]), ("0x" + .topics[2][26:]), .data] | @tsv' |
    while IFS=$'\t' read -r blk tx from to data; do
      printf '%d %s %s %s %s\n' "$blk" "$tx" "$(lower "$from")" "$(lower "$to")" "$(hex2dec "$data")"
    done
}

# Reads the ledger and answers the linkability question, setting the globals below. Everything
# here is a set operation on `from`/`to`/`value` — deliberately nothing else, so that it means the
# same thing for either pool and for a reader who trusts neither.
#
#   ENTRY_PAID     addresses the Entry's Safe paid, the protocol contracts excluded
#   EXIT_PAID_BY   addresses that paid the Exit's Safe, the faucet excluded
#   SHARED         both at once — the deposit addresses of the plain pool
#   ONE_HOP        where the Entry's money went next, if it did not reach the Exit directly
#   MATCHED        Exit inflows whose amount equals some Entry outflow to the wei
#   PIX_ROWS       the rows of the ledger that touch any of the above (the bootstrap left out)
linkability() {
  local rows=$1 entry_safe=$2 exit_safe=$3
  ENTRY_PAID=()
  EXIT_PAID_BY=()
  SHARED=()
  ONE_HOP=()
  MATCHED=0
  EXIT_IN=0
  PIX_ROWS=""
  [ -n "$rows" ] && [ -n "$entry_safe" ] && [ -n "$exit_safe" ] || return 0
  local -A paid paid_by out_amounts hop
  local blk tx from to wei
  while read -r blk tx from to wei; do
    if [ "$from" = "$entry_safe" ] && [ -z "${BOOTSTRAP[$to]:-}" ] && [ "$to" != "$from" ]; then
      paid[$to]=1
      out_amounts[$wei]=1
    fi
    if [ "$to" = "$exit_safe" ] && [ "$from" != "$DEPLOYER" ] && [ -z "${BOOTSTRAP[$from]:-}" ]; then
      paid_by[$from]=1
    fi
  done <<<"$rows"
  local a
  for a in "${!paid[@]}"; do
    ENTRY_PAID+=("$a")
    [ -n "${paid_by[$a]:-}" ] && SHARED+=("$a")
  done
  for a in "${!paid_by[@]}"; do EXIT_PAID_BY+=("$a"); done
  # Second pass: what the Entry's recipients did with it, and whether the Exit's inflows match
  # any outflow exactly.
  while read -r blk tx from to wei; do
    if [ -n "${paid[$from]:-}" ] && [ "$to" != "$entry_safe" ]; then hop[$to]=1; fi
    if [ "$to" = "$exit_safe" ] && [ "$from" != "$DEPLOYER" ] && [ -z "${BOOTSTRAP[$from]:-}" ]; then
      EXIT_IN=$((EXIT_IN + 1))
      [ -n "${out_amounts[$wei]:-}" ] && MATCHED=$((MATCHED + 1))
    fi
  done <<<"$rows"
  for a in "${!hop[@]}"; do ONE_HOP+=("$a"); done
  local -A interesting=([$entry_safe]=1 [$exit_safe]=1)
  for a in "${ENTRY_PAID[@]}" "${EXIT_PAID_BY[@]}" "${ONE_HOP[@]}"; do interesting[$a]=1; done
  PIX_ROWS=$(while read -r blk tx from to wei; do
    [ "$from" = "$DEPLOYER" ] && continue
    { [ -n "${BOOTSTRAP[$from]:-}" ] || [ -n "${BOOTSTRAP[$to]:-}" ]; } && continue
    if [ -n "${interesting[$from]:-}" ] || [ -n "${interesting[$to]:-}" ]; then
      printf '%s %s %s %s %s\n' "$blk" "$tx" "$from" "$to" "$wei"
    fi
  done <<<"$rows")
  # Deposit addresses of the plain pool get a number, so the ledger reads as a sequence.
  local n=0
  for a in $(printf '%s\n' "${SHARED[@]}" | sort); do
    n=$((n + 1))
    [ -z "${LABEL[$a]:-}" ] && LABEL[$a]="deposit addr #$n"
  done
  return 0
}

print_rows() { # rows [max]
  local rows=$1 max=${2:-0} total
  total=$(printf '%s\n' "$rows" | grep -c .)
  if [ "$max" -gt 0 ] && [ "$total" -gt "$max" ]; then
    printf '    %s… %d earlier transfers%s\n' "$C_DIM" $((total - max)) "$C_RESET"
    rows=$(printf '%s\n' "$rows" | tail -n "$max")
  fi
  local blk tx from to wei
  while read -r blk tx from to wei; do
    [ -z "$blk" ] && continue
    printf '    %sblk %5d%s  %-18s %s→%s  %-18s %s%16s%s\n' \
      "$C_DIM" "$blk" "$C_RESET" "$(label "$from")" "$C_DIM" "$C_RESET" "$(label "$to")" \
      "$C_BOLD" "$(wei2hopr "$wei")" "$C_RESET"
  done <<<"$rows"
}

join_labels() {
  local out="" a
  for a in "$@"; do out+="${out:+, }$(label "$a")"; done
  printf '%s' "${out:-none}"
}

# ── gathering one frame's worth of readings ────────────────────────────────────

gather() {
  # The live pane and the launcher's final snapshot share a cache.
  exec 9>"$STATE_DIR/.curvy.lock"
  flock -w 45 9 || return 1
  gather_unlocked
  flock -u 9
}

sync_run_cache() {
  # A new pix-demo run means a new chain and new node identities: every cached reading is from a
  # cluster that no longer exists.
  local started
  started=$(cat "$STATE_DIR/started" 2>/dev/null || echo "")
  if [ -n "$started" ] && [ "$started" != "$(cat "${CACHE}_run" 2>/dev/null)" ]; then
    rm -f "${CACHE}"_*
    printf '%s\n' "$started" >"${CACHE}_run"
  fi
  ELAPSED=""
  [ -n "$started" ] && ELAPSED=$(($(cat "$STATE_DIR/finished" 2>/dev/null || date +%s) - started))
  return 0
}

gather_unlocked() {
  sync_run_cache

  local i
  for i in "${ALL_IDXS[@]}"; do
    EXCERPT[i]=$(node_excerpt "$i")
    SAFE[i]=$(lower "$(node_safe "$i" "${EXCERPT[i]}")")
    MODULE[i]=$(lower "$(node_module "$i" "${EXCERPT[i]}")")
    ADDR[i]=$(lower "$(node_address "$i")")
  done
  POOL=$(grep -o 'pool="[a-z0-9-]*"' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d'"' -f2)

  CONTRACTS=$(contracts)
  CHAIN_ID=$(jq -r '.chainId // empty' <<<"$CONTRACTS" 2>/dev/null)
  BLOCK=$(jq -r '.blockNumber // empty' <<<"$CONTRACTS" 2>/dev/null)
  TOKEN=$(lower "$(jq -r '.contracts.token // empty' <<<"$CONTRACTS" 2>/dev/null)")
  VAULT=$(lower "$(jq -r '.contracts.curvy_vault // empty' <<<"$CONTRACTS" 2>/dev/null)")
  PORTAL_FACTORY=$(lower "$(jq -r '.contracts.curvy_portal_factory // empty' <<<"$CONTRACTS" 2>/dev/null)")
  AGGREGATOR=$(lower "$(jq -r '.contracts.curvy_aggregator // empty' <<<"$CONTRACTS" 2>/dev/null)")
  [ -n "$AGGREGATOR" ] || AGGREGATOR=$(lower "$(grep -o 'aggregator=0x[0-9a-fA-F]*' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d= -f2)")
  PORTAL=$(lower "$(grep -o 'portal=0x[0-9a-fA-F]*' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d= -f2)")

  LABEL=()
  LABEL[$DEPLOYER]="deployer / faucet"
  LABEL[0x0000000000000000000000000000000000000000]="burn"
  # The HOPR protocol contracts: parties to the bootstrap (stakes, key bindings, Safe
  # deployment), never to a PIX deposit. Labelled, and kept out of the linkability sets.
  BOOTSTRAP=([0x0000000000000000000000000000000000000000]=1)
  local name addr
  while IFS=$'\t' read -r name addr; do
    addr=$(lower "$addr")
    [ -z "$addr" ] && continue
    case "$name" in
    token | curvy_*) ;;
    *)
      BOOTSTRAP[$addr]=1
      LABEL[$addr]="HOPR ${name//_/ }"
      ;;
    esac
  done < <(jq -r '.contracts // {} | to_entries[] | [.key, .value] | @tsv' <<<"$CONTRACTS" 2>/dev/null)
  [ -n "$TOKEN" ] && LABEL[$TOKEN]="wxHOPR token"
  [ -n "$VAULT" ] && LABEL[$VAULT]="Curvy vault"
  [ -n "$AGGREGATOR" ] && LABEL[$AGGREGATOR]="Curvy aggregator"
  [ -n "$PORTAL_FACTORY" ] && LABEL[$PORTAL_FACTORY]="Curvy portal factory"
  [ -n "$PORTAL" ] && LABEL[$PORTAL]="Entry's shield portal"
  [ -n "${SAFE[$ENTRY_IDX]}" ] && LABEL[${SAFE[$ENTRY_IDX]}]="Entry Safe"
  [ -n "${SAFE[$EXIT_IDX]}" ] && LABEL[${SAFE[$EXIT_IDX]}]="Exit Safe"
  [ -n "${MODULE[$ENTRY_IDX]}" ] && LABEL[${MODULE[$ENTRY_IDX]}]="Entry Safe module"
  [ -n "${MODULE[$EXIT_IDX]}" ] && LABEL[${MODULE[$EXIT_IDX]}]="Exit Safe module"
  [ -n "${ADDR[$ENTRY_IDX]}" ] && LABEL[${ADDR[$ENTRY_IDX]}]="Entry node"
  [ -n "${ADDR[$EXIT_IDX]}" ] && LABEL[${ADDR[$EXIT_IDX]}]="Exit node"
  local n=0
  for i in "${RELAY_IDXS[@]}"; do
    n=$((n + 1))
    [ -n "${SAFE[$i]}" ] && LABEL[${SAFE[$i]}]="Relay $n Safe"
    [ -n "${MODULE[$i]}" ] && LABEL[${MODULE[$i]}]="Relay $n Safe module"
    [ -n "${ADDR[$i]}" ] && LABEL[${ADDR[$i]}]="Relay $n node"
  done

  VAULT_STATE=$(vault_state)
  FEE_IN=$(jq -r '.curvyVaultFees.depositFee // empty' <<<"$VAULT_STATE" 2>/dev/null)
  FEE_OUT=$(jq -r '.curvyVaultFees.withdrawalFee // empty' <<<"$VAULT_STATE" 2>/dev/null)
  TREE_ROOT=$(jq -r '.curvyAggregatorState.notesTreeRoot // empty' <<<"$VAULT_STATE" 2>/dev/null)
  NOTES=$(notes)
  # `jq` on an empty reading prints nothing rather than 0, hence the defaults.
  PENDING=$(jq -r '.curvyPendingNotes.notes | length' <<<"$NOTES" 2>/dev/null)
  COMMITTED=$(jq -r '.curvyCommittedNotes.notes | length' <<<"$NOTES" 2>/dev/null)
  NULLIFIED=$(jq -r '.curvyCommittedNullifiers.nullifiers | length' <<<"$NOTES" 2>/dev/null)
  PLAINTEXT=$(jq -r '[.curvyPendingNotes.notes[] | select(.isPlaintext)] | length' <<<"$NOTES" 2>/dev/null)
  PENDING=${PENDING:-0} COMMITTED=${COMMITTED:-0} NULLIFIED=${NULLIFIED:-0} PLAINTEXT=${PLAINTEXT:-0}

  ROWS=$(ledger_rows "$(transfers "$TOKEN")")
  linkability "$ROWS" "${SAFE[$ENTRY_IDX]}" "${SAFE[$EXIT_IDX]}"

  gas_scan "$GAS_SCAN_BUDGET" "$GAS_SCAN_SECS"
  TXS=$(curvy_txs)
  INDEXED=()
  local h pn cn nn
  while read -r h pn cn nn; do
    [ -n "$h" ] && INDEXED[$h]="$pn $cn $nn"
  done < <(tx_index_counts)
  # The notes the Exit recognised as its own, from its log. The one fact on the notes list the
  # chain cannot supply.
  EXIT_NOTES=()
  for h in $(grep -o 'note_id=0x[0-9a-fA-F]*' <<<"${EXCERPT[$EXIT_IDX]}" | cut -d= -f2 | tr '[:upper:]' '[:lower:]' | sort -u); do
    EXIT_NOTES[$h]=1
  done
  NOTE_ROWS=$(note_rows)
  # Fetch receipts for the newest transactions first: those are the ones on screen.
  RPC_FETCH_BUDGET=3
  local kind ts
  while read -r ts kind h; do
    [ -n "$h" ] && tx_summary "$h" >/dev/null
  done < <(printf '%s\n' "$TXS" | tail -n 8 | tac)
  return 0
}

# Unknown is different from zero. Counts describe the scanned blocks shown above them.
tx_count() {
  if [ -f "${CACHE}_gas_next" ]; then
    awk -v kind="$1" '$2 == kind { n++ } END { print n+0 }' <<<"$TXS"
  else printf 'n/a'; fi
}

source_status() {
  local name=$1 state at age
  state=$(cat "${CACHE}_${name}.status" 2>/dev/null || echo unavailable)
  at=$(cat "${CACHE}_${name}.at" 2>/dev/null || echo 0)
  if [ "$at" -gt 0 ]; then
    age=$(($(date +%s) - at))
    printf '%s (%ss ago)' "$state" "$age"
  else printf '%s' "$state"; fi
}

render_sources() {
  local next head
  next=$(cat "${CACHE}_gas_next" 2>/dev/null || echo 0)
  head=$(cat "${CACHE}_gas_head" 2>/dev/null || echo '?')
  printf '  %sDATA%s  RPC %s · transfers %s · index %s\n' "$C_BOLD" "$C_RESET" \
    "$(source_status gas)" "$(source_status transfers)" "$(source_status notes)"
  if [ "$next" -gt 0 ]; then
    printf '    transaction counts cover blocks %s–%s; RPC head %s\n' "$PIX_CHAIN_FROM_BLOCK" "$((next - 1))" "$head"
    if [ "$head" != '?' ] && [ "$next" -le "$head" ]; then
      printf '    %sscan catching up — counts are partial%s\n' "$C_YELLOW" "$C_RESET"
    fi
  fi
  if [ "$PENDING" -ge 1000 ] || [ "$COMMITTED" -ge 1000 ] || [ "$NULLIFIED" -ge 1000 ]; then
    printf '    %snote index display capped at 1,000 records per category; transaction counts use RPC%s\n' "$C_YELLOW" "$C_RESET"
  fi
  printf '\n'
}

# Fields of the Entry's and the Exit's private views, from their logs.
count() { grep -c -- "$1" <<<"$2"; }
last_field() { # pattern field excerpt
  grep -- "$1" <<<"$3" | tail -1 | grep -o "$2=[^ ]*" | head -1 | cut -d= -f2- | tr -d '"'
}
distinct() { grep -- "$1" <<<"$2" | grep -o "$3=0x[0-9a-fA-F]*" | sort -u | grep -c .; }
proof_ms() { # circuit excerpt → last total_ms
  grep "proof timing" <<<"$2" | grep "circuit=\"$1\"" | tail -1 | grep -o 'total_ms=[0-9]*' | cut -d= -f2
}
secs() { [ -n "${1:-}" ] && trim "$(echo "scale=1; $1 / 1000" | bc)s" || printf '%s' "–"; }

# ── gas ─────────────────────────────────────────────────────────────────────────
#
# Every transaction on the chain, from its receipt: `block hash from to gas status selector
# inner_to inner_selector arg0`. Read block by block through the RPC — `eth_getBlockByNumber` for
# the calldata, `eth_getBlockReceipts` for the gas — and appended to `${CACHE}_gas`, with the next
# block to read in `${CACHE}_gas_next`. A block is immutable once mined and this anvil never
# reorgs, so a block is read once and the file only grows; the dashboard reads a few blocks per
# frame and keeps up with the chain, `--ledger` reads whatever is left in one go. That is what
# lets `--ledger` price the run once pix-demo has torn the chain down: the receipts are already
# here.
#
# A node's call to a protocol contract goes through its Safe module (`execTransactionFromModule`)
# and Safe-owner actions through `execTransaction`; for those the inner target and selector are
# recorded too, so the report can name what the Safe actually did and where a transfer went.
GAS_SCAN_BUDGET=40
GAS_SCAN_SECS=20
gas_scan() { # blocks-per-call [timeout-secs]
  local budget=${1:-$GAS_SCAN_BUDGET} secs=${2:-20} next out
  next=$(cat "${CACHE}_gas_next" 2>/dev/null || echo "$PIX_CHAIN_FROM_BLOCK")
  [ "$budget" -gt 0 ] || return 0
  # Runs inside the chain container: the head, then every non-empty block in the window as two
  # JSON lines (block with full transactions, receipts), then the last block read.
  # The single quotes are deliberate: it is the container's sh that expands these.
  # shellcheck disable=SC2016
  local script='
rpc=$RPC_URL
h=$(cast block-number --rpc-url $rpc) || exit 1
to=$((FROM + BUDGET - 1)); [ "$to" -gt "$h" ] && to=$h
echo "head $h"
n=$FROM
while [ "$n" -le "$to" ]; do
  x=$(printf "0x%x" "$n")
  b=$(cast rpc --rpc-url $rpc eth_getBlockByNumber "$x" true) || exit 1
  case "$b" in
  *"\"transactions\":[]"*) ;;
  *) printf "%s\n" "$b"; cast rpc --rpc-url $rpc eth_getBlockReceipts "$x" || exit 1 ;;
  esac
  n=$((n + 1))
done
echo "scanned $to"'
  # Megabytes of block JSON on a long run: kept on disk, not in a variable.
  out="${CACHE}_gas_scan.tmp"
  chain_exec --timeout "$secs" -e FROM="$next" -e BUDGET="$budget" sh -c "$script" >"$out" 2>"${CACHE}_gas.error" || {
    if [ -f "${CACHE}_gas_next" ]; then echo cached; else echo unavailable; fi >"${CACHE}_gas.status"
    rm -f "$out"
    return 0
  }
  local head scanned
  head=$(sed -n 's/^head //p' "$out")
  scanned=$(sed -n 's/^scanned //p' "$out")
  [[ $head =~ ^[0-9]+$ && $scanned =~ ^[0-9]+$ ]] || {
    if [ -f "${CACHE}_gas_next" ]; then echo cached; else echo unavailable; fi >"${CACHE}_gas.status"
    rm -f "$out"
    return 0
  }
  printf '%s\n' "$head" >"${CACHE}_gas_head"
  if ! sed '/^head /d; /^scanned /d' "$out" | jq -rs '
    def hex: ltrimstr("0x") | explode | map(if . >= 97 then . - 87 elif . >= 65 then . - 55 else . - 48 end) | reduce .[] as $d (0; . * 16 + $d);
    def word($i): .[$i*64 : $i*64+64];
    ([.[] | select(type == "array") | .[]] | map({key: (.transactionHash | ascii_downcase), value: .}) | from_entries) as $rc
    | .[] | select(type == "object") | .transactions[]
    | . as $t | ($rc[$t.hash | ascii_downcase] // error("missing transaction receipt")) as $r
    | ($t.input // "0x" | ltrimstr("0x")) as $in | $in[0:8] as $sel
    # execTransactionFromModule / execTransaction: (address to, uint256 value, bytes data, …) —
    # the bytes offset sits in word 2, the data one word past it.
    | (if ($t.to != null) and ($sel == "468721a7" or $sel == "6a761202") then
         $in[8:] as $a | ($a | word(2) | hex) as $off
         | {ito: ("0x" + ($a | word(0))[24:64]), isel: $a[$off*2+64 : $off*2+72], arg0: $a[$off*2+72 : $off*2+136]}
       else {ito: "-", isel: "", arg0: $in[8:72]} end) as $i
    | [($t.blockNumber | hex), ($t.hash | ascii_downcase), ($t.from | ascii_downcase), ($t.to // "-" | ascii_downcase),
       ($r.gasUsed // error("missing gas usage") | hex), ($r.status // error("missing receipt status") | hex),
       (if $t.to == null then "create" elif $sel == "" then "-" else "0x" + $sel end),
       $i.ito, (if $i.isel == "" then "-" else "0x" + $i.isel end), (if $i.arg0 == "" then "-" else $i.arg0 end)]
    | join(" ")' >"${CACHE}_gas.rows.tmp" 2>>"${CACHE}_gas.error"; then
    if [ -f "${CACHE}_gas_next" ]; then echo cached; else echo unavailable; fi >"${CACHE}_gas.status"
    rm -f "$out" "${CACHE}_gas.rows.tmp"
    return 0
  fi
  cat "${CACHE}_gas.rows.tmp" >>"${CACHE}_gas"
  rm -f "${CACHE}_gas.rows.tmp"
  rm -f "$out"
  printf '%s\n' $((scanned + 1)) >"${CACHE}_gas_next"
  date +%s >"${CACHE}_gas.at"
  echo live >"${CACHE}_gas.status"
}

# Function selectors → names, for the contracts this chain can see: the HOPR protocol (from
# hopr-bindings), Curvy v2 (from curvy-abi), Safe. Anything else is shown by its selector.
gas_selector_names() {
  cat <<'EOF_SEL'
0x468721a7 execTransactionFromModule
0x5229073f execTransactionFromModuleReturnData
0x6a761202 execTransaction
0xa9059cbb transfer
0x23b872dd transferFrom
0x095ea7b3 approve
0x9bd9bbc6 send
0xdcdc7dd0 mint
0xfe9d9303 burn
0x62ad1b83 operatorSend
0x959b8c3f authorizeOperator
0x2f2ff15d grantRole
0xd547741f revokeRole
0x36568abe renounceRole
0xf2fde38b transferOwnership
0x715018a6 renounceOwnership
0x4f1ef286 upgradeToAndCall
0x439fab91 initialize
0xc4d66de8 initialize
0xfc55309a fundChannel
0x0abec58f fundChannelSafe
0xeb13eb11 redeemTicket
0xab9b6ba7 redeemTicketSafe
0x1a7ffe7a closeIncomingChannel
0x54a2edf5 closeIncomingChannelSafe
0x7c8e28da initiateOutgoingChannelClosure
0xbda65f45 initiateOutgoingChannelClosureSafe
0x23cb3ac0 finalizeOutgoingChannelClosure
0x651514bf finalizeOutgoingChannelClosureSafe
0xea0a5237 announce
0xfad0e5a2 announceSafe
0x308c712e revokeSafe
0x244d496e updateKeyBindingFee
0x7f935931 registerSafeByNode
0x49d215e1 registerSafeWithNodeSig
0x91607c4c deregisterNodeBySafe
0x9a94addf clone
0x696ab635 deployModule
0x477e1487 updateHoprNetwork
0xfa2aeab4 updateSafeLibAddress
0x0233296b updateModuleSingletonAddress
0xabed205b updateErc1820Implementer
0xb5736962 includeNode
0x110dcee7 includeNodes
0x9d95f1cc addNode
0xdf9620eb addNodes
0xb2b99ec9 removeNode
0xa2450f89 addChannelsAndTokenTarget
0x739c4b08 scopeTargetChannels
0xa76c9a2f scopeTargetToken
0xdc06109d scopeTargetSend
0x3f0444c1 scopeTargetServiceRegistry
0xfa19501d scopeChannelsCapabilities
0xc68605c8 scopeTokenCapabilities
0xc68c3a83 scopeSendCapability
0x3401cde8 revokeTarget
0x8b95eccd setMultisend
0x15981650 setTicketPrice
0xfde46ff8 setWinProb
0x0ff55869 registerServiceType
0x326005af selfRegister
0x210c3298 selfUpdate
0x1e46f907 selfDeregister
0x326d1f22 setRequirement
0x48559a57 setNodeSafeRegistry
0xb3f618d1 setTypeRegistrationFee
0xf13217a9 setSelfRegistrationBurn
0x1cd0ad00 setSelfUpdateBurn
0x47ce2eef transferTypeOwnership
0x634e93da beginDefaultAdminTransfer
0x649a5ec7 changeDefaultAdminDelay
0x8d5626c0 submitAggregationRequest
0x7fda4404 commitPendingNotes
0x87aabf09 submitWithdrawalRequest
0xba48d117 autoShield
0x0f2e251b setAggregationVerifier
0x65b804ab setPendingNotesCommitmentVerifier
0x6026b96d setWithdrawalVerifier
0x1dd706c6 setCommitmentGasFeeRoot
0xc481dd26 setFeeNotePublicKey
0xf69b340b setProtocolFees
0x1b6f139f updateConfig
0x11c9a94d updateConfig
0x8340f549 deposit
0x5f0f48bd withdraw
0x09824a80 registerToken
0xbab2af1d deregisterToken
0x77e5614d setCurvyAggregatorAddress
0x7d6ba5b3 setFeeCollectorAddress
0x432c0553 setFeeAmount
0x3ad4009c setPerTokenGasFees
0xb17acdcd collectFees
0xb62c712f bootstrapAccessControl
0xb64b2a8a deployShieldPortal
0x848c9f82 deployEntryBridgePortal
0x2a33cf2e deployExitBridgePortal
0x53070b55 deployRecoveryEntryPortal
0x66e93b8c deployRecoveryExitPortal
0x199c544e shield
0xe3f5c552 bridge
0x648bf774 recover
0x76fceb76 initialize
0xb63e800d setup
0x610b5925 enableModule
0x0d582f13 addOwnerWithThreshold
0xe318b52b swapOwner
0x1688f0b9 createProxyWithNonce
0x8d80ff0a multiSend
EOF_SEL
}

# The gas report: every transaction the scan has seen, grouped by who paid for it and what it
# did, with the average that is the figure to quote; then what one PIX deposit costs the pool,
# recurring transactions only, one-offs (the shield) beside it. All of it is arithmetic over the
# receipts — the labels are the only thing the report takes from anywhere else.
#
# The chain image deploys the contracts before the cluster exists; those transactions are the
# image's, not the run's, and are listed apart. The run starts at the first block that touches a
# node address — the deployer funding it — which is also the first block the labels can name.
render_gas() {
  local rows="${CACHE}_gas" head next
  head=$(cat "${CACHE}_gas_head" 2>/dev/null || echo "")
  next=$(cat "${CACHE}_gas_next" 2>/dev/null || echo 0)
  printf '  %sGAS SPENT%s  %severy receipt on the chain, by who paid and what for — avg is the figure to quote%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ ! -s "$rows" ]; then
    printf '    %sno receipts — the chain is not reachable and nothing was read while it ran%s\n' "$C_DIM" "$C_RESET"
    return
  fi
  {
    local a
    for a in "${!LABEL[@]}"; do printf 'L %s %s\n' "$a" "${LABEL[$a]}"; done
    gas_selector_names | sed 's/^/S /'
    sed 's/^/R /' "$rows"
  } | awk -v head="${head:-?}" -v unread="$next" \
    -v BOLD="$C_BOLD" -v DIM="$C_DIM" -v GREEN="$C_GREEN" -v RESET="$C_RESET" '
  function short(a) { return length(a) > 14 ? substr(a, 1, 6) "…" substr(a, length(a) - 3) : a }
  function lbl(a) { return (a in L) ? L[a] : short(a) }
  function fn(sel) { return (sel in S) ? S[sel] : sel }
  function commas(n,   s, r) { s = sprintf("%d", n); r = ""; while (length(s) > 3) { r = "," substr(s, length(s) - 2) r; s = substr(s, 1, length(s) - 3) }; return s r }
  function dest(w) { return lbl("0x" substr(w, 25, 40)) }
  # Averages want one row per action: the two relays are one sender, the module of the node is
  # "its", the stealth deposit addresses are one destination, and so are the four nodes.
  function collapse(l) { return l ~ /^deposit addr #/ ? "a deposit addr" : l ~ /^Relay [0-9]+ node$/ ? "Relay nodes" : l }
  function collapse_dest(l) { return l ~ /^deposit addr #/ ? "a deposit addr" : l ~ /^(Entry|Exit|Relay [0-9]+) node$/ ? "a node" : l }
  function via(l) { return l ~ /^(Entry|Exit|Relay [0-9]+) Safe module$/ ? "its Safe module" : l }
  function touches_node(a) { return lbl(a) ~ /node$/ }
  # One table: the senders of `part` by what they spent, each sender by action likewise.
  function table(part,   m, i, j, t, s, r, a, b, k, p, name) {
    m = 0; for (s in ssum) { split(s, p, SUBSEP); if (p[1] == part) senders[++m] = s }
    if (m == 0) { printf "    %snothing%s\n", DIM, RESET; return }
    for (i = 1; i <= m; i++) for (j = i + 1; j <= m; j++) if (ssum[senders[j]] > ssum[senders[i]]) { t = senders[i]; senders[i] = senders[j]; senders[j] = t }
    printf "    %s%-62s %5s %9s %9s %9s %11s%s\n", DIM, "", "txs", "avg", "min", "max", "total", RESET
    for (i = 1; i <= m; i++) {
      s = senders[i]; split(s, p, SUBSEP)
      printf "    %s%-62s %5d %9s %9s %9s %11s%s\n", BOLD, p[2], sn[s], "", "", "", commas(ssum[s]), RESET
      r = 0
      for (k = 1; k <= keys; k++) { split(order[k], p, SUBSEP); if (p[1] SUBSEP p[2] == s) rk[++r] = order[k] }
      for (a = 1; a <= r; a++) for (b = a + 1; b <= r; b++) if (sum[rk[b]] > sum[rk[a]]) { t = rk[a]; rk[a] = rk[b]; rk[b] = t }
      for (a = 1; a <= r; a++) {
        k = rk[a]; split(k, p, SUBSEP); name = p[3]
        if (k in failed) name = name " (" failed[k] " failed)"
        if (length(name) > 60) name = substr(name, 1, 59) "…"
        printf "      %-60s %5d %9s %9s %9s %11s\n", name, n[k], commas(sum[k] / n[k]), commas(min[k]), commas(max[k]), commas(sum[k])
      }
    }
  }
  $1 == "L" { L[$2] = substr($0, length($2) + 4); next }
  $1 == "S" { S[$2] = $3; next }
  $1 != "R" { next }
  {
    rows[++nr] = $0
    # The run begins where the chain first touches a node: funded by the deployer, or sending.
    if (touches_node($4) || touches_node($5) || ($11 != "-" && touches_node("0x" substr($11, 25, 40))))
      if (runstart == "" || $2 + 0 < runstart + 0) runstart = $2 + 0
  }
  END {
    for (i = 1; i <= nr; i++) {
      $0 = rows[i]
      blk = $2 + 0; from = $4; to = $5; gas = $6 + 0; ok = $7; sel = $8; ito = $9; isel = $10; arg0 = $11
      part = (runstart != "" && blk < runstart) ? "setup" : "run"
      sender = collapse(lbl(from))
      kind = ""
      if (to == "-") { act = "contract creation" }
      else if (sel == "-") { act = "native transfer → " collapse_dest(lbl(to)) }
      else if (isel != "-") {
        f = fn(isel); act = lbl(ito) "." f
        if (f == "transfer" || f == "send" || f == "approve") act = act " → " collapse_dest(dest(arg0))
        act = act " via " via(lbl(to))
        if (f == "transfer" && lbl(ito) == "wxHOPR token") kind = "pix"
      } else {
        f = fn(sel); act = lbl(to) "." f
        if (f == "transfer" || f == "send" || f == "mint" || f == "approve") act = act " → " collapse_dest(dest(arg0))
        if (f == "submitAggregationRequest") { act = "allocation: " act; kind = "allocation" }
        else if (f == "commitPendingNotes") { act = "commit: " act; kind = "commit" }
        else if (f == "submitWithdrawalRequest") { act = "withdrawal: " act; kind = "withdrawal" }
        else if (f == "deployShieldPortal") { act = "shield: " act; kind = "shield" }
        else if (f == "transfer" && lbl(to) == "wxHOPR token") kind = "pix"
      }
      # The PIX side of the ledger: the vault contracts, and wxHOPR moving between the two
      # Safes, the deposit addresses and the shield portal. Everything else is bootstrap or
      # protocol.
      if (kind == "pix") {
        d = dest(arg0)
        if (!(sender ~ /^(Entry|Exit) node$/ || sender == "a deposit addr" || d == "Exit Safe" || d ~ /^deposit addr #/ || d ~ /^Entry.s shield portal$/)) kind = ""
        else if (d ~ /^Entry.s shield portal$/) kind = "shield"
        else if (sender == "Entry node") kind = "deposit"
      }
      key = part SUBSEP sender SUBSEP act
      if (!(key in n)) { n[key] = 0; sum[key] = 0; min[key] = gas; max[key] = gas; kindof[key] = kind; order[++keys] = key }
      n[key]++; sum[key] += gas; if (gas < min[key]) min[key] = gas; if (gas > max[key]) max[key] = gas
      if (ok != "1") failed[key]++
      sn[part SUBSEP sender]++; ssum[part SUBSEP sender] += gas
      total[part] += gas; txs[part]++
      if (first[part] == "" || blk < first[part]) first[part] = blk
      if (blk > last[part]) last[part] = blk
      if (kind == "allocation") allocations++
      if (kind == "deposit") deposits++
    }
    printf "    %s%d transactions in blocks %d–%d, %s gas in all", DIM, txs["run"] + txs["setup"], first["setup"] == "" ? first["run"] : first["setup"], last["run"] == "" ? last["setup"] : last["run"], commas(total["run"] + total["setup"])
    if (head != "?" && unread + 0 <= head + 0) printf " · blocks %d–%s not read yet", unread, head
    printf "%s\n", RESET
    printf "\n    %sTHE RUN%s  %sfrom block %s: %d transactions, %s gas%s\n", BOLD, RESET, DIM, first["run"] == "" ? "?" : first["run"], txs["run"], commas(total["run"]), RESET
    table("run")
    # What one deposit costs. With Curvy a deposit is one allocation and the commit that lands
    # it, plus its share of the withdrawals; with the plain pool the two transfers.
    cycles = allocations > 0 ? allocations : deposits
    printf "\n  %sPER PIX DEPOSIT%s  %swhat the pool spends per deposit, recurring transactions only%s\n", BOLD, RESET, DIM, RESET
    if (cycles == 0) printf "    %sno deposit yet%s\n", DIM, RESET
    else {
      per = 0
      split("deposit allocation commit withdrawal pix", cycle, " ")
      for (c = 1; c <= 5; c++) for (k = 1; k <= keys; k++) {
        key = order[k]
        if (kindof[key] != cycle[c]) continue
        split(key, p, SUBSEP)
        share = sum[key] / cycles; per += share
        printf "    %-58s %10s   %s%d tx / %d deposits × avg %s%s\n", substr(p[3], 1, 58), commas(share), DIM, n[key], cycles, commas(sum[key] / n[key]), RESET
      }
      printf "    %s≈ %s gas per deposit%s over %d deposit(s)", GREEN BOLD, commas(per), RESET, cycles
      one = 0
      for (k = 1; k <= keys; k++) if (kindof[order[k]] == "shield") one += sum[order[k]]
      if (one > 0) printf " · %sone-off shield %s gas, not counted%s", DIM, commas(one), RESET
      printf "\n"
    }
    if (txs["setup"] > 0) {
      printf "\n    %sCHAIN SETUP%s  %sby the image, before the cluster: blocks %d–%d, %d transactions, %s gas%s\n", BOLD, RESET, DIM, first["setup"], last["setup"], txs["setup"], commas(total["setup"]), RESET
      table("setup")
    }
  }'
}

# ── dashboard ───────────────────────────────────────────────────────────────────

declare -a EXCERPT SAFE ADDR MODULE
declare -A INDEXED EXIT_NOTES BOOTSTRAP

render() {
  gather || {
    printf 'Dashboard cache busy; retrying.\n'
    return
  }
  local entry="${EXCERPT[$ENTRY_IDX]}" exit_="${EXCERPT[$EXIT_IDX]}"
  local seen correlated clock="--:--" shield_wei exit_wei
  seen=$(distinct "discovered Curvy PIX pending note" "$exit_" note_id)
  correlated=$(distinct "correlated committed Curvy PIX note" "$exit_" note_id)
  shield_wei=$(printf '%s\n' "$ROWS" | awk -v e="${SAFE[$ENTRY_IDX]}" -v p="$PORTAL" -v v="$VAULT" \
    '$4 == v && (($3 == e && e != "") || ($3 == p && p != "")) { print $5 }' | sum)
  exit_wei=$(printf '%s\n' "$ROWS" | awk -v x="${SAFE[$EXIT_IDX]}" -v v="$VAULT" '$3 == v && $4 == x && x != "" { print $5 }' | sum)
  [ -n "$ELAPSED" ] && clock=$(printf '%02d:%02d' $((ELAPSED / 60)) $((ELAPSED % 60)))
  printf '\033[H\033[2J'
  printf '  %sCurvy privacy pool — settlement and node observations%s  %s\n\n' "$C_MAGENTA$C_BOLD" "$C_RESET" "$clock"
  printf '  %sDEPLOYMENT%s  pool=%s · chain %s · indexed block %s · vault fees %s bps in, %s out\n' \
    "$C_BOLD" "$C_RESET" "${POOL:-?}" "${CHAIN_ID:-?}" "${BLOCK:-?}" "${FEE_IN:-?}" "${FEE_OUT:-?}"
  printf '    vault %s · aggregator %s\n' "$(short "${VAULT:-?}")" "$(short "${AGGREGATOR:-?}")"
  if [ -n "$NOTES" ]; then
    printf '    notes: %s announced · %s in the tree · %s nullifiers spent\n' "$PENDING" "$COMMITTED" "$NULLIFIED"
  else printf '    note index unavailable\n'; fi
  printf '\n'
  render_sources

  printf '  %sON-CHAIN SETTLEMENT%s  %ssuccessful transactions, all submitters%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -f "${CACHE}_transfers" ] && [ -n "${SAFE[$ENTRY_IDX]}" ]; then
    printf '    %-29s %s%s wxHOPR%s  %s\n' 'Entry Safe → vault' "$C_GREEN" "$(wei2hopr "$shield_wei")" "$C_RESET" \
      "$(if [ -n "$PORTAL" ]; then echo 'via shield portal'; else echo 'direct shielding'; fi)"
  else printf '    %-29s n/a — transfer data or Safe address unavailable\n' 'Entry Safe → vault'; fi
  printf '    %-29s %s txs\n' 'PIX aggregations' "$(tx_count allocation)"
  printf '    %-29s %s txs  %sincludes shared batch-prover submissions%s\n' 'Pending-note commitments' "$(tx_count commit)" "$C_DIM" "$C_RESET"
  printf '    %-29s %s txs\n' 'Withdrawals' "$(tx_count withdrawal)"
  if [ -f "${CACHE}_transfers" ] && [ -n "${SAFE[$EXIT_IDX]}" ]; then
    printf '    %-29s %s%s wxHOPR%s\n' 'Vault → Exit Safe' "$C_GREEN$C_BOLD" "$(wei2hopr "$exit_wei")" "$C_RESET"
  else printf '    %-29s n/a — transfer data or Safe address unavailable\n' 'Vault → Exit Safe'; fi
  printf '\n'

  printf '  %sNODE OBSERVATIONS%s  %sprivate node logs; separate from the public chain view%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -n "$exit_" ]; then
    printf '    %-29s %s  %sscan-key discovery%s\n' 'Exit notes recognised' "$seen" "$C_DIM" "$C_RESET"
    printf '    %-29s %s  %scommitted note matched to a deposit; not SSA key recovery%s\n' 'Exit notes matched to SSA' "$correlated" "$C_DIM" "$C_RESET"
  else printf '    Exit log unavailable\n'; fi
  printf '    latest local proof time: aggregation %s · withdrawal %s\n\n' \
    "$(secs "$(proof_ms pix-aggregation "$entry")")" "$(secs "$(proof_ms pix-withdrawal "$exit_")")"

  render_txs 5
  render_notes 4
  printf '  %sTOKEN TRANSFERS%s  %swxHOPR movements involving the Entry, Exit and pool%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -n "$PIX_ROWS" ]; then
    print_rows "$PIX_ROWS" 4
  elif [ ! -f "${CACHE}_transfers" ]; then
    printf '    transfer data unavailable\n'
  else printf '    no matching transfers in the available data\n'; fi
  printf '\n'
  [ -z "${1:-}" ] || printf '\n  %s%s%s\n' "$C_DIM" "$1" "$C_RESET"
}

# One line per Curvy transaction, from its receipt: which contract it went to, how many bytes of
# calldata, submitter, and what the vault's index says it did. The direct shield is
# signed by the node; relayer and batch-prover transactions have their own senders.
render_txs() { # [max]
  local max=${1:-0}
  printf '  %sCURVY TRANSACTIONS%s  %smined successfully; discovered from RPC, independent of node logs%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -z "$TXS" ]; then
    printf '    %s%s%s\n\n' "$C_DIM" "$(if [ -f "${CACHE}_gas_next" ]; then echo 'none in the scanned blocks'; else echo 'transaction data unavailable'; fi)" "$C_RESET"
    return
  fi
  local total
  total=$(printf '%s\n' "$TXS" | grep -c .)
  local shown=$TXS
  if [ "$max" -gt 0 ] && [ "$total" -gt "$max" ]; then
    printf '    %s… %d earlier transactions%s\n' "$C_DIM" $((total - max)) "$C_RESET"
    shown=$(printf '%s\n' "$TXS" | tail -n "$max")
  fi
  local ts kind h summary blk from to bytes detail pn cn nn
  while read -r ts kind h; do
    [ -z "$h" ] && continue
    summary=$(tx_summary "$h" | head -1)
    read -r blk from to _ _ bytes <<<"$summary"
    read -r pn cn nn <<<"${INDEXED[$h]:-? ? ?}"
    detail=""
    case "$kind" in
    shield) detail="funding note announced" ;;
    commit) detail="$cn note(s) into the tree" ;;
    allocation) detail="$pn output note(s), including change/padding" ;;
    withdrawal)
      local paid
      paid=$(printf '%s\n' "$ROWS" | awk -v h="$h" -v x="${SAFE[$EXIT_IDX]}" '$2 == h && $4 == x { print $5 }' | sum)
      detail="$nn nullifier(s)"
      [ ! -f "${CACHE}_transfers" ] || detail="$detail · $(wei2hopr "$paid") → Exit Safe"
      ;;
    esac
    if [ -n "$blk" ]; then
      printf '    %sblk %5d%s  %-11s → %-16s %s%6s B%s  %s%s%s\n' \
        "$C_DIM" "$blk" "$C_RESET" "$kind" "$(label "$to")" "$C_DIM" "$bytes" "$C_RESET" "$C_BOLD" "$detail" "$C_RESET"
      printf '      from %s · tx %s\n' "$(label "$from")" "$(short "$h")"
    else
      printf '    %s%-9s%s  %-11s %s%s  (receipt detail unavailable)%s\n' "$C_DIM" "blk $ts" "$C_RESET" "$kind" "$C_DIM" "$(short "$h")" "$C_RESET"
    fi
  done <<<"$shown"
  printf '    %s%d successful pool transactions in scanned blocks; B = full calldata size%s\n\n' "$C_DIM" "$total" "$C_RESET"
}

# The vault's notes as the chain announced them. The right-hand marker is the one thing on this
# list that does not come from the chain: the Exit's own log, saying which note it recognised.
render_notes() { # [max]
  local max=${1:-0}
  printf '  %sNOTES IN THE VAULT%s  %spublic announcements; encrypted or plaintext amounts%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -z "$NOTE_ROWS" ]; then
    printf '    %s%s%s\n\n' "$C_DIM" "$(if [ -n "$NOTES" ]; then echo 'none in the index snapshot'; else echo 'note index unavailable'; fi)" "$C_RESET"
    return
  fi
  local total
  total=$(printf '%s\n' "$NOTE_ROWS" | grep -c .)
  local shown=$NOTE_ROWS
  if [ "$max" -gt 0 ] && [ "$total" -gt "$max" ]; then
    printf '    %s… %d earlier notes%s\n' "$C_DIM" $((total - max)) "$C_RESET"
    shown=$(printf '%s\n' "$NOTE_ROWS" | tail -n "$max")
  fi
  local blk id plain amount tag amt mark
  while read -r blk id plain amount tag; do
    [ -z "$id" ] && continue
    if [ "$plain" = "true" ]; then amt="$(wei2hopr "$amount") wxHOPR, plaintext"; else amt="amount encrypted"; fi
    [ "$tag" != null ] && [ -n "$tag" ] || tag=n/a
    mark=""
    [ -n "${EXIT_NOTES[$id]:-}" ] && mark="${C_GREEN}← the Exit's — it knows, the chain does not${C_RESET}"
    printf '    %sblk %5d%s  %s  %-27s %sview tag %-3s%s %s\n' \
      "$C_DIM" "$blk" "$C_RESET" "$(short "$id")" "$amt" "$C_DIM" "$tag" "$C_RESET" "$mark"
  done <<<"$shown"
  printf '    %soutput counts include padding/change; they are not PIX deposit counts%s\n' "$C_DIM" "$C_RESET"
  printf '    %sView tag 0 is valid. Padding uses 0, but a tag alone does not identify padding or ownership.%s\n' "$C_DIM" "$C_RESET"
  printf '\n'
}

render_verdict() {
  printf '  %sTRANSFER LINKABILITY%s  %sobserved token-transfer paths, not a privacy proof%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ ! -f "${CACHE}_transfers" ] || [ -z "${SAFE[$ENTRY_IDX]}" ] || [ -z "${SAFE[$EXIT_IDX]}" ]; then
    printf '    transfer data or node Safe addresses unavailable\n'
    return
  fi
  printf '    %-24s %s\n' 'the Entry paid' "$(join_labels "${ENTRY_PAID[@]}")"
  printf '    %-24s %s\n' 'the Exit was paid by' "$(join_labels "${EXIT_PAID_BY[@]}")"
  printf '    %-24s %d   %-24s %d of %d\n' 'shared intermediaries' "${#SHARED[@]}" 'exact amount matches' "$MATCHED" "$EXIT_IN"
  local shared_non_pool=0 a
  for a in "${SHARED[@]}"; do
    [ "$a" = "$VAULT" ] || shared_non_pool=$((shared_non_pool + 1))
  done
  if [ "$shared_non_pool" -gt 0 ]; then
    printf '    %sSHARED NON-POOL ADDRESS%s — %s intermediary address(es) connect the transfers.\n' "$C_YELLOW$C_BOLD" "$C_RESET" "$shared_non_pool"
  elif [ "$EXIT_IN" -gt 0 ] && [ -n "$VAULT" ] && [[ " ${EXIT_PAID_BY[*]} " == *" $VAULT "* ]]; then
    printf '    %sPOOL-MEDIATED%s — the vault is a shared intermediary, not a per-session address.\n' "$C_GREEN$C_BOLD" "$C_RESET"
    printf '    These transfers do not identify which shielded note funded the withdrawal.\n'
    printf '    %sThis small local run still exposes timing/amount clues; it demonstrates no anonymity set.%s\n' "$C_YELLOW" "$C_RESET"
  elif [ "$EXIT_IN" -gt 0 ]; then
    printf '    No shared intermediary identified in the available transfers.\n'
  else printf '    No Exit payout observed in the available transfers.\n'; fi
}

# ── entry points ────────────────────────────────────────────────────────────────

# Allow fixture tests to exercise readers/renderers without starting the refresh loop.
[[ ${BASH_SOURCE[0]} == "$0" ]] || return 0

mkdir -p "$STATE_DIR" 2>/dev/null
if [ -L "$STATE_DIR" ] || [ ! -d "$STATE_DIR" ] || [ ! -O "$STATE_DIR" ]; then
  echo "$STATE_DIR must be a directory owned by $(id -un) and not a symlink."
  echo "Remove it, or point PIX_DEMO_STATE_DIR somewhere else (pix-demo.sh needs the same value)."
  exit 1
fi
for tool in curl jq bc docker timeout flock; do
  command -v "$tool" >/dev/null || {
    echo "curvy-demo needs $tool — try running it inside \`nix develop\`"
    exit 1
  }
done

case "${1:-}" in
--dashboard)
  render "blokli: $PIX_BLOKLI_URL/graphql · chain: ${PIX_RPC_URL:-docker exec $PIX_CHAIN_CONTAINER} cast …"
  exit 0
  ;;
--snapshot)
  # Called before the owned chain is removed; preserve the final RPC state for the pane.
  GAS_SCAN_BUDGET=1000000
  GAS_SCAN_SECS=40
  # Force a last transfer read even if the pane polled less than six seconds ago.
  LEDGER_REFRESH=0
  gather || exit 1
  [[ $(cat "${CACHE}_gas.status" 2>/dev/null) == live &&
  $(cat "${CACHE}_transfers.status" 2>/dev/null) == live ]] || exit 1
  exit 0
  ;;
--ledger)
  # The chain, while it is still there, is read to the head before pricing the run.
  GAS_SCAN_BUDGET=1000000
  GAS_SCAN_SECS=300
  gather
  printf '\n%swxHOPR Transfer events on chain %s, %d in all, labelled:%s\n\n' "$C_BOLD" "${CHAIN_ID:-?}" "$(printf '%s\n' "$ROWS" | grep -c .)" "$C_RESET"
  print_rows "$ROWS"
  printf '\n'
  render_verdict
  printf '\n'
  render_gas
  printf '\n'
  exit 0
  ;;
--rpc)
  gather
  # Every receipt, not just a frame's worth.
  RPC_FETCH_BUDGET=1000
  while read -r _ _ h; do [ -n "$h" ] && tx_summary "$h" >/dev/null; done <<<"$TXS"
  printf '\n'
  render_sources
  render_txs
  render_notes
  exit 0
  ;;
-h | --help)
  sed -n '2,/^$/p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
  ;;
"") ;;
*)
  echo "usage: $0 [--dashboard | --ledger | --rpc | --snapshot]"
  exit 1
  ;;
esac

printf '\033[?25l'
# The cursor restore lives on EXIT alone; INT/TERM must actually exit — a trap handler that
# only prints returns into the loop, and Ctrl-C stops nothing.
trap 'printf "\033[?25h"; echo' EXIT
trap 'exit 130' INT TERM
while :; do
  render "Ctrl-C to stop · ledger: $0 --ledger"
  sleep "$REFRESH"
done
