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
# The last section is the one to show. With the test pool (`PIX_POOL=test`) the sets meet on
# every deposit address, and the verdict reads LINKED; with Curvy they do not, and it reads
# UNLINKED. Nothing in the verdict knows which pool is running — it is computed from the
# `Transfer` log alone, the same way an outside observer would.
#
# It attaches to the cluster pix-demo.sh runs; it starts nothing itself:
#
#   ./localcluster/scripts/pix-demo.sh                                  # pane 1
#   ./localcluster/scripts/curvy-demo.sh                                # pane 2, refreshes until Ctrl-C
#   watch -n 2 -c ./localcluster/scripts/curvy-demo.sh --dashboard      # or under watch(1)
#   ./localcluster/scripts/curvy-demo.sh --ledger                       # every transfer + the verdict, once
#   ./localcluster/scripts/curvy-demo.sh --rpc                          # every Curvy tx and note, once
#
# Sources: Blokli's GraphQL API on :8080 (the deployment's contract addresses, vault fees,
# aggregator state, the indexed notes and nullifiers); `cast` inside the chain container (the raw
# `Transfer` log, receipts and calldata straight out of anvil — the one source that owes nothing
# to HOPR or Curvy code);
# the nodes' logs (found through each hoprd's stdout while it runs, `/tmp/pix-soak-logs` after);
# and the REST API for node addresses.
#
# Every reading is cached under the PIX_DEMO_STATE_DIR pix-demo.sh uses, so the closing frame —
# and `--ledger` after the run — still show the run's figures once pix-demo has torn the chain
# container down. The cache is dropped when pix-demo starts a new run.
#
# Requires curl, jq, bc, docker; run it inside `nix develop`.

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
LOG_COPY_DIR=/tmp/pix-soak-logs
REFRESH=2
# anvil's account 0: the localcluster's deployer and faucet, and in this demo the Curvy operator.
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
  out=$("$@" 2>/dev/null)
  if [ -n "$out" ] && printf '%s' "$out" | jq -e . >/dev/null 2>&1; then
    printf '%s\n' "$out" >"$file"
  elif [ -r "$file" ]; then
    cat "$file"
    return 0
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
  cache_json transfers timeout 15 docker exec -e FOUNDRY_DISABLE_NIGHTLY_WARNING=1 "$PIX_CHAIN_CONTAINER" \
    cast logs --rpc-url http://127.0.0.1:8545 --from-block 0 --address "$token" \
    'Transfer(address indexed from, address indexed to, uint256 value)' --json
}

# ── the Curvy transactions, as the RPC returns them ─────────────────────────────

# Every Curvy transaction the two nodes sent, oldest first: `timestamp kind hash`. The hashes are
# the one thing taken from the logs; everything shown about them comes back from the chain.
curvy_txs() {
  {
    printf '%s\n' "${EXCERPT[$ENTRY_IDX]}" | grep -E "shielded the Curvy funding note|committed pending Curvy notes|aggregated Curvy PIX allocations"
    printf '%s\n' "${EXCERPT[$EXIT_IDX]}" | grep -E "withdrew Curvy PIX notes"
  } | awk '
    { ts = $1; kind = "" }
    /shielded the Curvy funding note/ { kind = "shield" }
    /committed pending Curvy notes/ { kind = "commit" }
    /aggregated Curvy PIX allocations/ { kind = "allocation" }
    /withdrew Curvy PIX notes/ { kind = "withdrawal" }
    kind != "" { if (match($0, /tx=0x[0-9a-fA-F]+/)) print ts, kind, tolower(substr($0, RSTART + 3, RLENGTH - 3)) }
  ' | sort -u
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
    timeout 15 docker exec -e FOUNDRY_DISABLE_NIGHTLY_WARNING=1 "$PIX_CHAIN_CONTAINER" sh -c \
      "cast receipt $hash --async --rpc-url http://127.0.0.1:8545 --json; cast tx $hash --rpc-url http://127.0.0.1:8545 --json" 2>/dev/null |
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
  # A new pix-demo run means a new chain and new node identities: every cached reading is from a
  # cluster that no longer exists.
  local started
  started=$(cat "$STATE_DIR/started" 2>/dev/null || echo "")
  if [ -n "$started" ] && [ "$started" != "$(cat "${CACHE}_run" 2>/dev/null)" ]; then
    rm -f "${CACHE}"_*
    printf '%s\n' "$started" >"${CACHE}_run"
  fi
  ELAPSED=""
  [ -n "$started" ] && ELAPSED=$(($(date +%s) - started))

  local i
  for i in "${ALL_IDXS[@]}"; do
    EXCERPT[i]=$(node_excerpt "$i")
    SAFE[i]=$(lower "$(node_safe "$i" "${EXCERPT[i]}")")
    ADDR[i]=$(lower "$(node_address "$i")")
  done
  POOL=$(grep -o 'pool="[a-z0-9-]*"' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d'"' -f2)

  CONTRACTS=$(contracts)
  CHAIN_ID=$(jq -r '.chainId // empty' <<<"$CONTRACTS" 2>/dev/null)
  BLOCK=$(jq -r '.blockNumber // empty' <<<"$CONTRACTS" 2>/dev/null)
  TOKEN=$(lower "$(jq -r '.contracts.token // empty' <<<"$CONTRACTS" 2>/dev/null)")
  VAULT=$(lower "$(jq -r '.contracts.curvy_vault // empty' <<<"$CONTRACTS" 2>/dev/null)")
  PORTAL_FACTORY=$(lower "$(jq -r '.contracts.curvy_portal_factory // empty' <<<"$CONTRACTS" 2>/dev/null)")
  # Not in chainInfo; the Entry logs it when it discovers the deployment.
  AGGREGATOR=$(lower "$(grep -o 'aggregator=0x[0-9a-fA-F]*' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d= -f2)")
  PORTAL=$(lower "$(grep -o 'portal=0x[0-9a-fA-F]*' <<<"${EXCERPT[$ENTRY_IDX]}" | head -1 | cut -d= -f2)")

  LABEL=()
  LABEL[$DEPLOYER]="operator (= deployer)"
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
  [ -n "${ADDR[$ENTRY_IDX]}" ] && LABEL[${ADDR[$ENTRY_IDX]}]="Entry node"
  [ -n "${ADDR[$EXIT_IDX]}" ] && LABEL[${ADDR[$EXIT_IDX]}]="Exit node"
  local n=0
  for i in "${RELAY_IDXS[@]}"; do
    n=$((n + 1))
    [ -n "${SAFE[$i]}" ] && LABEL[${SAFE[$i]}]="Relay $n Safe"
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

# ── dashboard ───────────────────────────────────────────────────────────────────

declare -a EXCERPT SAFE ADDR
declare -A INDEXED EXIT_NOTES BOOTSTRAP

render() {
  gather

  local entry="${EXCERPT[$ENTRY_IDX]}" exit_="${EXCERPT[$EXIT_IDX]}"
  local shield_tx commits aggregations allocations gross
  shield_tx=$(last_field "shielded the Curvy funding note" tx "$entry")
  gross=$(last_field "funding the Curvy shield portal" gross "$entry")
  commits=$(count "committed pending Curvy notes" "$entry")
  aggregations=$(count "aggregated Curvy PIX allocations" "$entry")
  allocations=$(grep "aggregated Curvy PIX allocations" <<<"$entry" | grep -o 'allocations=[0-9]*' | cut -d= -f2 | sum)
  local seen correlated withdrawals withdrawn_wei
  seen=$(distinct "discovered Curvy PIX pending note" "$exit_" note_id)
  correlated=$(distinct "correlated committed Curvy PIX note" "$exit_" note_id)
  withdrawals=$(count "withdrew Curvy PIX notes" "$exit_")
  withdrawn_wei=$(grep "withdrew Curvy PIX notes" <<<"$exit_" | grep -o 'amount=[0-9]*' | cut -d= -f2 | sum)

  local clock="--:--"
  [ -n "$ELAPSED" ] && clock=$(printf '%02d:%02d' $((ELAPSED / 60)) $((ELAPSED % 60)))

  printf '\033[H\033[2J'
  printf '%s╔══════════════════════════════════════════════════════════════════════════╗%s\n' "$C_MAGENTA" "$C_RESET"
  printf '%s║%s  %sCurvy privacy pool%s — the same PIX run, as the chain sees it      %s%s%s  %s║%s\n' \
    "$C_MAGENTA" "$C_RESET" "$C_BOLD" "$C_RESET" "$C_BOLD" "$clock" "$C_RESET" "$C_MAGENTA" "$C_RESET"
  printf '%s╚══════════════════════════════════════════════════════════════════════════╝%s\n\n' "$C_MAGENTA" "$C_RESET"

  printf '  %sDEPLOYMENT%s  %spool=%s%s · chain %s · block %s · vault fees %s bps in, %s out\n' \
    "$C_BOLD" "$C_RESET" "$C_DIM" "${POOL:-?}" "$C_RESET" "${CHAIN_ID:-?}" "${BLOCK:-?}" "${FEE_IN:-?}" "${FEE_OUT:-?}"
  printf '    vault %s · aggregator %s · shield portal %s\n' \
    "$(short "${VAULT:-?}")" "$(short "${AGGREGATOR:-?}")" "$(short "${PORTAL:-?}")"
  printf '    notes tree root %s · %s%d announced · %d in the tree · %d spent%s\n\n' \
    "$(short "${TREE_ROOT:-?}")" "$C_BOLD" "$PENDING" "$COMMITTED" "$NULLIFIED" "$C_RESET"

  printf '  %sENTRY%s  %swhat it did — from its own log, nobody else sees this%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -n "$shield_tx" ]; then
    printf '    %-28s %s%s wxHOPR%s  Safe → portal → vault  %stx %s%s\n' "shielded once, from the Safe" \
      "$C_GREEN$C_BOLD" "$(wei2hopr "$gross")" "$C_RESET" "$C_DIM" "$(short "$shield_tx")" "$C_RESET"
  else
    printf '    %-28s %swaiting for the first deposit%s\n' "shielded once, from the Safe" "$C_DIM" "$C_RESET"
  fi
  printf '    %-28s %s%3d%s txs   %s%3d%s allocations to the Exit'"'"'s one-time scan keys\n' \
    "PIX allocations aggregated" "$C_BOLD" "$aggregations" "$C_RESET" "$C_BOLD" "$allocations" "$C_RESET"
  printf '    %-28s %3d txs   %sproofs: pending %s · aggregation %s%s\n\n' \
    "pending notes committed" "$commits" "$C_DIM" "$(secs "$(proof_ms pending "$entry")")" \
    "$(secs "$(proof_ms pix-aggregation "$entry")")" "$C_RESET"

  printf '  %sEXIT%s  %swhat it did — from its own log, nobody else sees this%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  printf '    %-28s %s%3d%s of %s in the vault  %sscan-key trial decryption%s\n' \
    "notes recognised as its own" "$C_BOLD" "$seen" "$C_RESET" "$PENDING" "$C_DIM" "$C_RESET"
  printf '    %-28s %3d        %sSSA key recovered from the note'"'"'s allocation%s\n' \
    "committed notes correlated" "$correlated" "$C_DIM" "$C_RESET"
  printf '    %-28s %3d txs   %s%s wxHOPR%s → Exit Safe  %sproof: withdrawal %s%s\n' \
    "withdrawn with a zk proof" "$withdrawals" "$C_GREEN$C_BOLD" "$(wei2hopr "$withdrawn_wei")" "$C_RESET" \
    "$C_DIM" "$(secs "$(proof_ms pix-withdrawal "$exit_")")" "$C_RESET"
  printf '\n'

  render_txs 5
  render_notes 4

  printf '  %sON CHAIN%s  %swxHOPR Transfer events touching the Entry, the Exit, or their counterparties%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -n "$PIX_ROWS" ]; then
    print_rows "$PIX_ROWS" 4
  else
    printf '    %snothing yet%s\n' "$C_DIM" "$C_RESET"
  fi
  printf '\n'

  render_knows
  render_verdict
  if [ -n "${1:-}" ]; then
    printf '\n  %s%s%s\n' "$C_DIM" "$1" "$C_RESET"
  fi
}

# One line per Curvy transaction, from its receipt: which contract it went to, how many bytes of
# calldata (the proof), and what the vault's index says it did. The sender is not a column
# because it is always the same — the operator key — and that is the point: it names neither
# party.
render_txs() { # [max]
  local max=${1:-0}
  printf '  %sCURVY TRANSACTIONS%s  %sfrom the RPC: every one sent by the operator, to a contract%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -z "$TXS" ]; then
    printf '    %snone yet%s\n\n' "$C_DIM" "$C_RESET"
    return
  fi
  local total
  total=$(printf '%s\n' "$TXS" | grep -c .)
  local shown=$TXS
  if [ "$max" -gt 0 ] && [ "$total" -gt "$max" ]; then
    printf '    %s… %d earlier transactions%s\n' "$C_DIM" $((total - max)) "$C_RESET"
    shown=$(printf '%s\n' "$TXS" | tail -n "$max")
  fi
  local ts kind h summary blk to bytes detail pn cn nn
  while read -r ts kind h; do
    [ -z "$h" ] && continue
    summary=$(tx_summary "$h" | head -1)
    read -r blk _ to _ _ bytes <<<"$summary"
    read -r pn cn nn <<<"${INDEXED[$h]:-0 0 0}"
    detail=""
    case "$kind" in
    shield) detail="funding note announced" ;;
    commit) detail="$cn note(s) into the tree" ;;
    allocation) detail="$pn note(s) announced, encrypted" ;;
    withdrawal)
      local paid
      paid=$(printf '%s\n' "$ROWS" | awk -v h="$h" -v x="${SAFE[$EXIT_IDX]}" '$2 == h && $4 == x { print $5 }' | sum)
      detail="$nn nullifier(s) · $(wei2hopr "$paid") → Exit Safe"
      ;;
    esac
    if [ -n "$blk" ]; then
      printf '    %sblk %5d%s  %-11s → %-16s %s%6s B%s  %s%s%s\n' \
        "$C_DIM" "$blk" "$C_RESET" "$kind" "$(label "$to")" "$C_DIM" "$bytes" "$C_RESET" "$C_BOLD" "$detail" "$C_RESET"
    else
      printf '    %s%-9s%s  %-11s %s%s  (receipt pending)%s\n' "$C_DIM" "${ts:11:8}" "$C_RESET" "$kind" "$C_DIM" "$(short "$h")" "$C_RESET"
    fi
  done <<<"$shown"
  printf '    %s%d shielded-pool transactions in all; a proof is what the calldata bytes are%s\n\n' "$C_DIM" "$total" "$C_RESET"
}

# The vault's notes as the chain announced them. The right-hand marker is the one thing on this
# list that does not come from the chain: the Exit's own log, saying which note it recognised.
render_notes() { # [max]
  local max=${1:-0}
  printf '  %sNOTES IN THE VAULT%s  %sas announced on chain: an id, a view tag, a ciphertext — no owner%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -z "$NOTE_ROWS" ]; then
    printf '    %snone yet%s\n\n' "$C_DIM" "$C_RESET"
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
    mark=""
    [ -n "${EXIT_NOTES[$id]:-}" ] && mark="${C_GREEN}← the Exit's — it knows, the chain does not${C_RESET}"
    printf '    %sblk %5d%s  %s  %-27s %sview tag %-3s%s %s\n' \
      "$C_DIM" "$blk" "$C_RESET" "$(short "$id")" "$amt" "$C_DIM" "$tag" "$C_RESET" "$mark"
  done <<<"$shown"
  # An allocation announces more notes than it creates — the aggregation circuit pads its
  # output — so the Exit's note is one of ten announced per deposit, and nothing on chain says
  # which. The count of its own is the Exit's log's; the rest of the line is the chain's.
  local allocations mine
  allocations=$(printf '%s\n' "$TXS" | grep -c " allocation ")
  mine=${#EXIT_NOTES[@]}
  if [ "$allocations" -gt 0 ]; then
    printf '    %s%d notes announced by %d allocations; the Exit'"'"'s %d are among them and nothing on\n' \
      "$C_DIM" $((total - 1)) "$allocations" "$mine"
    printf '      chain says which%s\n' "$C_RESET"
  fi
  printf '\n'
}

# Each fact the chain publishes, beside the fact it withholds. Every number on the left is one
# an outside observer can read off the RPC; nothing on the right is on the chain at all.
render_knows() {
  local allocations notes_n encrypted nullifiers withdrawals shield_wei shield_txt
  allocations=$(printf '%s\n' "$TXS" | grep -c " allocation ")
  withdrawals=$(printf '%s\n' "$TXS" | grep -c " withdrawal ")
  notes_n=${PENDING:-0}
  encrypted=$((notes_n - ${PLAINTEXT:-0}))
  nullifiers=${NULLIFIED:-0}
  shield_wei=$(printf '%s\n' "$ROWS" | awk -v p="$PORTAL" -v v="$VAULT" '$3 == p && $4 == v { print $5 }' | sum)
  if [ "${shield_wei:-0}" != "0" ]; then shield_txt="$(wei2hopr "$shield_wei") wxHOPR, once"; else shield_txt="nothing yet"; fi
  local exit_in_txt="nothing yet"
  if [ "${EXIT_IN:-0}" -gt 0 ] && [ -n "$VAULT" ]; then
    local exit_wei per_out
    exit_wei=$(printf '%s\n' "$ROWS" | awk -v v="$VAULT" -v x="${SAFE[$EXIT_IDX]}" '$3 == v && $4 == x { print $5 }' | sum)
    per_out=$(echo "$exit_wei / $withdrawals" | bc 2>/dev/null || echo 0)
    exit_in_txt="$withdrawals × $(wei2hopr "$per_out") wxHOPR"
  fi
  printf '  %sWHAT THE CHAIN KNOWS%s                         %sWHAT IT DOES NOT%s\n' "$C_BOLD" "$C_RESET" "$C_BOLD" "$C_RESET"
  printf '    %-18s %-22s %s%s%s\n' "Entry → vault" "$shield_txt" "$C_DIM" "what any of it was for" "$C_RESET"
  printf '    %-18s %-22s %s%s%s\n' "allocations" "$allocations txs, by the operator" "$C_DIM" "whom — a scan key, no address" "$C_RESET"
  printf '    %-18s %-22s %s%s%s\n' "notes announced" "$notes_n ($encrypted encrypted)" "$C_DIM" "amounts, or which is the Exit's" "$C_RESET"
  printf '    %-18s %-22s %s%s%s\n' "nullifiers spent" "$nullifiers" "$C_DIM" "which note each one retires" "$C_RESET"
  printf '    %-18s %-22s %s%s%s\n' "vault → Exit Safe" "$exit_in_txt" "$C_DIM" "that the Entry paid the Exit" "$C_RESET"
  printf '\n'
}

render_verdict() {
  printf '  %sLINKABILITY%s  %sfrom the Transfer log alone — what an outside observer can conclude%s\n' "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  if [ -z "$ROWS" ] || [ -z "${SAFE[$ENTRY_IDX]}" ]; then
    printf '    %sno ledger yet%s\n' "$C_DIM" "$C_RESET"
    return
  fi
  printf '    %-24s %s\n' "the Entry paid" "$(join_labels "${ENTRY_PAID[@]}")"
  printf '    %-24s %s\n' "the Exit was paid by" "$(join_labels "${EXIT_PAID_BY[@]}")"
  printf '    %-24s %s\n' "the money went on to" "$(join_labels "${ONE_HOP[@]}")"
  printf '    %-24s %d   %-24s %d of %d\n' "addresses in both sets" "${#SHARED[@]}" "exact amount matches" "$MATCHED" "$EXIT_IN"
  if [ "${#SHARED[@]}" -gt 0 ]; then
    printf '    %s✗ LINKED%s — %d address(es) took the Entry'"'"'s money and handed it to the Exit;\n' "$C_RED$C_BOLD" "$C_RESET" "${#SHARED[@]}"
    printf '      every deposit is a public transfer chain Entry → address → Exit\n'
  elif [ "$EXIT_IN" -gt 0 ]; then
    printf '    %s✓ UNLINKED%s — no address both received from the Entry and paid the Exit, and no\n' "$C_GREEN$C_BOLD" "$C_RESET"
    printf '      amount matches. The Exit was paid by the vault against a Merkle root; which note\n'
    printf '      it spent is inside the proof, not on the chain.\n'
    printf '    %scaveat: on this 4-node chain the vault has one depositor, so timing alone still\n' "$C_YELLOW"
    printf '      tells the story. On a shared deployment the set is every depositor'"'"'s.%s\n' "$C_RESET"
  else
    printf '    %sthe Exit has not been paid yet%s\n' "$C_DIM" "$C_RESET"
  fi
}

# ── entry points ────────────────────────────────────────────────────────────────

mkdir -p "$STATE_DIR" 2>/dev/null
if [ -L "$STATE_DIR" ] || [ ! -d "$STATE_DIR" ] || [ ! -O "$STATE_DIR" ]; then
  echo "$STATE_DIR must be a directory owned by $(id -un) and not a symlink."
  echo "Remove it, or point PIX_DEMO_STATE_DIR somewhere else (pix-demo.sh needs the same value)."
  exit 1
fi
for tool in curl jq bc docker; do
  command -v "$tool" >/dev/null || {
    echo "curvy-demo needs $tool — try running it inside \`nix develop\`"
    exit 1
  }
done

case "${1:-}" in
--dashboard)
  render "blokli: $PIX_BLOKLI_URL/graphql · chain: docker exec $PIX_CHAIN_CONTAINER cast …"
  exit 0
  ;;
--ledger)
  gather
  printf '\n%swxHOPR Transfer events on chain %s, %d in all, labelled:%s\n\n' "$C_BOLD" "${CHAIN_ID:-?}" "$(printf '%s\n' "$ROWS" | grep -c .)" "$C_RESET"
  print_rows "$ROWS"
  printf '\n'
  render_verdict
  printf '\n'
  exit 0
  ;;
--rpc)
  gather
  # Every receipt, not just a frame's worth.
  RPC_FETCH_BUDGET=1000
  while read -r _ _ h; do [ -n "$h" ] && tx_summary "$h" >/dev/null; done <<<"$TXS"
  printf '\n'
  render_txs
  render_notes
  render_knows
  exit 0
  ;;
-h | --help)
  sed -n '2,/^$/p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
  ;;
"") ;;
*)
  echo "usage: $0 [--dashboard | --ledger | --rpc]"
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
