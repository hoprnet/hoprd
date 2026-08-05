#!/usr/bin/env bash
#
# Live dashboard for the PIX Session soak test.
#
# Runs `session_pix_soak` against a throwaway 3-node localcluster and renders what the
# three nodes are doing while it happens: traffic crossing the Session, SSA cycles
# advancing through deposit → confirmation → key recovery → sweep, and the Exit's Safe
# filling up one quota-sized deposit at a time. The run ends by itself when the Entry can
# no longer afford a deposit and the Exit's kill switch closes the Session.
#
#   ./localcluster/scripts/pix-demo.sh                  # ~7 minutes
#   PIX_DEMO_FLOAT="150 wxHOPR" ./localcluster/scripts/pix-demo.sh    # ~55 cycles
#
# Everything on screen comes from the nodes' own Prometheus endpoints and the REST API —
# nothing is computed by the test. To watch a cluster somebody else started, or to drive
# the refresh with watch(1) instead:
#
#   watch -n 2 -c ./localcluster/scripts/pix-demo.sh --dashboard
#
# Requires curl, jq, bc, docker and cargo-nextest, so run it inside `nix develop`. Plus a
# release `hoprd` at HOPRD_BIN (default `target/release/hoprd`) and HOPRD_CHAIN_IMAGE — see
# the test's module docs.
#
# Safe to re-run: a stale chain container or leftover nodes from an interrupted attempt are
# cleared on the way in, and Ctrl-C tears the cluster down on the way out.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
API_PORT_BASE=13500
NODES=(Entry Relay Exit)
# Fixed rather than under $TMPDIR: the test runs inside `nix develop`, which sets its own
# TMPDIR, and `--dashboard` is meant to be usable from any shell against the same run.
STATE_DIR="/tmp/pix-demo"
TEST_LOG="$STATE_DIR/test.log"
REFRESH=2

C_RESET=$'\033[0m'
C_DIM=$'\033[2m'
C_BOLD=$'\033[1m'
C_CYAN=$'\033[36m'
C_GREEN=$'\033[32m'
C_YELLOW=$'\033[33m'
C_RED=$'\033[31m'

# ── metric helpers ──────────────────────────────────────────────────────────────

# Scrape one node's Prometheus endpoint into a cache file.
#
# A failed scrape leaves the previous cache in place rather than truncating it. That is what
# makes the closing frame readable: it is rendered after the test process has exited, so the
# nodes are already gone and every endpoint refuses. Redirecting curl straight at the cache
# file would blank it, and the last thing the audience saw would be a screen of zeros
# instead of the totals the run actually reached.
scrape() {
  local out
  out=$(curl -s --max-time 3 "http://127.0.0.1:$((API_PORT_BASE + $1))/metrics" 2>/dev/null)
  [ -n "$out" ] && printf '%s\n' "$out" >"$STATE_DIR/metrics_$1"
  return 0
}

# Sum a metric across label sets. `_total` is optional because OpenTelemetry's Prometheus
# exporter may or may not append it depending on version.
metric() {
  local file="$STATE_DIR/metrics_$1" name="$2" label="${3:-}"
  [ -r "$file" ] || {
    echo 0
    return
  }
  local lines
  lines=$(grep -E "^${name}(_total)?[ {]" "$file" 2>/dev/null)
  if [ -n "$label" ]; then
    lines=$(printf '%s\n' "$lines" | grep -F -- "$label")
  fi
  printf '%s\n' "$lines" | awk '{ s += $NF } END { printf "%.0f", s + 0 }'
}

# A balance field from the REST API, unit stripped ("12.5 wxHOPR" -> "12.5").
#
# Cached and falling back to the last good read for the same reason as `scrape`: once the
# nodes are down the API answers nothing, and a bare 0 in the closing frame would read as
# "the Exit received nothing".
balance() {
  local cache="$STATE_DIR/balance_$1_$2" value
  value=$(curl -s --max-time 3 "http://127.0.0.1:$((API_PORT_BASE + $1))/api/v4/account/balances" 2>/dev/null |
    jq -r ".$2 // empty" 2>/dev/null |
    awk 'NF { print $1; exit }')
  if [ -n "$value" ]; then
    printf '%s\n' "$value" >"$cache"
  elif [ -r "$cache" ]; then
    value=$(cat "$cache")
  fi
  printf '%s\n' "${value:-0}"
}

# Packets per second for one direction, from successive readings of a cumulative counter.
#
#   pkt_rate <key> <current count>
#
# Two properties matter more than the arithmetic:
#
#   * The window is at least $RATE_WINDOW seconds, not one $REFRESH tick. At four figures a
#     second a 2 s sample jitters by enough to be distracting on a projector, and the reading
#     is meant to be looked at rather than watched.
#   * The last figure is kept when the counter stops advancing, instead of dividing zero
#     packets by a growing window and rendering "0 pkt/s". The closing frame is drawn after
#     the nodes have gone, where `scrape` is serving a cached `/metrics` and every counter is
#     frozen by definition — the same reason `scrape` and `balance` keep their last good read.
RATE_WINDOW=6
pkt_rate() { # key current_count
  local sample="$STATE_DIR/rate_$1"
  local shown="$STATE_DIR/rateval_$1"
  local now
  local count=$2
  local prev_t=""
  local prev_c=""
  local elapsed
  now=$(date +%s)
  [ -r "$sample" ] && read -r prev_t prev_c <"$sample"
  # Both fields have to be present and numeric before they are arithmetic operands: a
  # truncated sample would otherwise reach `[ "$count" -gt "$prev_c" ]` with an empty operand,
  # which is a `test` syntax error rather than a false, and would spray onto the frame.
  case "${prev_t}:${prev_c}" in
  *[!0-9:]* | :* | *: | '') prev_t="" ;;
  esac
  if [ -n "$prev_t" ]; then
    elapsed=$((now - prev_t))
    if [ "$elapsed" -ge "$RATE_WINDOW" ]; then
      # Strictly greater, so a counter that has stopped (closing frame) or restarted from zero
      # (stale sample from a previous run) leaves the last figure alone instead of rendering 0.
      if [ "$count" -gt "$prev_c" ]; then
        printf '%.0f\n' "$(echo "($count - $prev_c) / $elapsed" | bc -l)" >"$shown"
      fi
      printf '%s %s\n' "$now" "$count" >"$sample"
    fi
  else
    printf '%s %s\n' "$now" "$count" >"$sample"
  fi
  if [ -r "$shown" ]; then cat "$shown"; else echo 0; fi
}

# A node's on-chain address, cached: it never changes, and the closing frame needs it after
# the node has gone away.
node_address() {
  local cache="$STATE_DIR/addr_$1" value
  if [ -r "$cache" ]; then
    cat "$cache"
    return
  fi
  value=$(curl -s --max-time 3 "http://127.0.0.1:$((API_PORT_BASE + $1))/api/v4/account/addresses" 2>/dev/null |
    jq -r '.native // empty' 2>/dev/null)
  [ -n "$value" ] && printf '%s\n' "$value" >"$cache"
  printf '%s\n' "$value"
}

# One field of the relay's ticket statistics for the incoming channel from `$2`.
#
# Scoping by counterparty is what separates the two directions: the relay's forward leg and
# return leg are two different incoming channels, earning independently, and the unscoped
# aggregate adds them together. Auto-redeeming is off for this test, so nothing is ever moved
# out of `unredeemedValue` — it is the whole of what the leg has earned.
ticket_stat() { # node counterparty field
  local cache="$STATE_DIR/tickets_$1_$2_$3" value
  [ -z "$2" ] && {
    echo 0
    return
  }
  value=$(curl -s --max-time 3 \
    "http://127.0.0.1:$((API_PORT_BASE + $1))/api/v4/tickets/statistics?address=$2" 2>/dev/null |
    jq -r ".$3 // empty" 2>/dev/null |
    awk 'NF { print $1; exit }')
  if [ -n "$value" ]; then
    printf '%s\n' "$value" >"$cache"
  elif [ -r "$cache" ]; then
    value=$(cat "$cache")
  fi
  printf '%s\n' "${value:-0}"
}

# Read a field out of the test's own startup banner, which is where the run's parameters
# (per-cycle deposit, funded cycles) are announced.
from_log() {
  [ -r "$TEST_LOG" ] || {
    echo ""
    return
  }
  sed 's/\x1b\[[0-9;]*m//g' "$TEST_LOG" 2>/dev/null |
    grep -m1 -o "$1=[0-9.]*" | head -1 | cut -d= -f2
}

# Progress bar. Built by slicing pre-filled strings rather than repeating a character:
# `printf 'X%.0s'` with an empty argument list still prints one X, which silently puts a
# block in every empty bar.
FULL='████████████████████████████████'
EMPTY='░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░'
bar() { # value max [width]
  local value=${1:-0} max=${2:-0} width=${3:-24} filled=0
  if [ "$max" -gt 0 ] 2>/dev/null; then
    filled=$((value * width / max))
  fi
  [ "$filled" -gt "$width" ] && filled=$width
  [ "$filled" -lt 0 ] && filled=0
  printf '%s%s' "${FULL:0:filled}" "${EMPTY:0:$((width - filled))}"
}

num() { printf "%'d" "${1:-0}" 2>/dev/null || echo "${1:-0}"; }

# Trim the trailing zeroes bc leaves behind ("2.65728000" -> "2.65728", "0" stays "0").
trim() { printf '%s' "${1:-0}" | sed -e 's/\(\.[0-9]*[1-9]\)0*$/\1/' -e 's/\.0*$//'; }

# ── dashboard ───────────────────────────────────────────────────────────────────

render() {
  for i in 0 1 2; do scrape "$i"; done

  local sweeps deposits made_failed confirmed keys last_sweep
  sweeps=$(metric 2 hopr_strategy_pix_sweeps)
  deposits=$(metric 0 hopr_strategy_pix_deposits)
  made_failed=$(metric 0 hopr_strategy_pix_deposits_failed)
  confirmed=$(metric 2 hopr_strategy_pix_deposit_tracking 'outcome="confirmed"')
  keys=$(metric 2 hopr_strategy_pix_keys_recovered)
  last_sweep=$(grep -E '^hopr_strategy_pix_last_sweep_hopr' "$STATE_DIR/metrics_2" 2>/dev/null |
    awk '{ print $NF }' | head -1)

  # The Exit's Safe only ever *gains* wxHOPR from PIX sweeps, so its growth over a baseline is
  # the recovered total. Choosing when to stop moving that baseline is the whole difficulty.
  #
  # It cannot be the first frame: opening the channels takes 800 wxHOPR back out of the Safe
  # after the nodes answer /readyz, which a pinned first-frame baseline reports as a large
  # negative recovery. It also cannot be "until the first sweep lands" — the sweep counter
  # increments only once `withdraw_from_signer` has returned, by which time the transaction is
  # mined and the Safe has already grown, so the last baseline written under `sweeps == 0`
  # swallows sweep #1 and every later frame is short exactly one cycle.
  #
  # The Entry's deposit counter is the signal that has slack: a sweep cannot precede the
  # deposit it sweeps, and the intervening confirmation and share collection take tens of
  # seconds — far more than this loop's refresh — so a baseline taken while `deposits == 0` is
  # guaranteed to be after channel funding and before any PIX money has moved.
  local exit_safe
  exit_safe=$(balance 2 safeHopr)
  if [ "$deposits" -eq 0 ] 2>/dev/null || [ ! -f "$STATE_DIR/baseline" ]; then
    echo "$exit_safe" >"$STATE_DIR/baseline"
  fi
  local baseline
  baseline=$(cat "$STATE_DIR/baseline")
  local recovered
  recovered=$(trim "$(echo "$exit_safe - $baseline" | bc -l)")

  local entry_float
  entry_float=$(balance 0 hopr)
  local per_cycle
  per_cycle=$(from_log "per_cycle")
  local funded
  funded=$(from_log "funded_cycles")
  # The run's fixed parameters, announced once in the test's startup banner. All empty when
  # attached with `--dashboard` to a cluster somebody else started, in which case the geometry
  # line is simply skipped rather than shown full of blanks.
  local polys shares quota price_per_byte
  polys=$(from_log "ssa_polys")
  shares=$(from_log "ssa_shares")
  quota=$(from_log "quota")
  price_per_byte=$(from_log "price_per_byte")
  [ -f "$STATE_DIR/started" ] || date +%s >"$STATE_DIR/started"
  local started
  started=$(cat "$STATE_DIR/started")
  local elapsed=$(($(date +%s) - started))

  # Relay earnings, per direction. The forward leg's tickets are issued by the Entry, the
  # return leg's by the Exit, so each node's address names the channel that direction pays on.
  local entry_addr exit_addr fwd_win fwd_val ret_win ret_val
  entry_addr=$(node_address 0)
  exit_addr=$(node_address 2)
  fwd_win=$(ticket_stat 1 "$entry_addr" winningCount)
  fwd_val=$(ticket_stat 1 "$entry_addr" unredeemedValue)
  ret_win=$(ticket_stat 1 "$exit_addr" winningCount)
  ret_val=$(ticket_stat 1 "$exit_addr" unredeemedValue)

  local e_sent x_recv x_sent e_recv r_fwd
  e_sent=$(metric 0 hopr_packets_count 'type="sent"')
  e_recv=$(metric 0 hopr_packets_count 'type="received"')
  x_sent=$(metric 2 hopr_packets_count 'type="sent"')
  x_recv=$(metric 2 hopr_packets_count 'type="received"')
  r_fwd=$(metric 1 hopr_packets_count 'type="forwarded"')

  # Derived from the same counters as the totals above, so the rate and the running total can
  # never tell different stories. These are node-wide HOPR packet counts, which include the
  # SURB keep-alives the balancer sends and the acknowledgements every packet earns — so both
  # figures sit above the Session's datagram rate. That is the honest HOPR packet rate, and
  # the label says "pkt/s" rather than anything implying datagrams.
  local fwd_rate ret_rate
  fwd_rate=$(pkt_rate fwd "$e_sent")
  ret_rate=$(pkt_rate ret "$x_sent")

  # Bars are scaled to the cycles the float pays for, which the test announces at startup.
  # Attaching to a cluster somebody else started leaves that unknown, so fall back to
  # scaling against whatever the Entry has managed so far.
  local scale="${funded:-0}"
  if [ "$scale" -le 0 ] 2>/dev/null; then
    scale=$((deposits > 0 ? deposits : 1))
  fi

  printf '\033[H\033[2J'
  printf '%s╔══════════════════════════════════════════════════════════════════════════╗%s\n' "$C_CYAN" "$C_RESET"
  printf '%s║%s  %sHOPR PIX Session%s — Entry pays the Exit for data it delivers      %s%02d:%02d%s  %s║%s\n' \
    "$C_CYAN" "$C_RESET" "$C_BOLD" "$C_RESET" "$C_BOLD" $((elapsed / 60)) $((elapsed % 60)) "$C_RESET" "$C_CYAN" "$C_RESET"
  printf '%s╚══════════════════════════════════════════════════════════════════════════╝%s\n\n' "$C_CYAN" "$C_RESET"

  # Why a deposit is the size it is, shown as the derivation rather than as a bare number: the
  # dimensions fix the quota, and the quota priced per byte fixes what the Entry has to pay.
  if [ -n "$polys" ] && [ -n "$shares" ]; then
    printf '  %sSSA GEOMETRY%s         %s%s polys × %s shares%s  %s→%s  %s%s B%s %squota per SSA%s\n' \
      "$C_BOLD" "$C_RESET" "$C_BOLD" "$polys" "$shares" "$C_RESET" "$C_DIM" "$C_RESET" \
      "$C_BOLD" "$(num "${quota:-0}")" "$C_RESET" "$C_DIM" "$C_RESET"
    printf '                       %s%s wxHOPR/byte%s  %s→%s  %s%s wxHOPR%s %sper deposit%s\n\n' \
      "$C_BOLD" "${price_per_byte:-?}" "$C_RESET" "$C_DIM" "$C_RESET" \
      "$C_GREEN$C_BOLD" "${per_cycle:-?}" "$C_RESET" "$C_DIM" "$C_RESET"
  fi

  printf '  %sSSA CYCLE PIPELINE%s   %seach cycle: Entry deposits, Exit confirms, collects\n' "$C_BOLD" "$C_RESET" "$C_DIM"
  printf '                       shares from the SURBs it spends, then sweeps%s\n\n' "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "Entry deposits" "$(bar "$deposits" "$scale")" "$C_BOLD" "$deposits" "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "Exit confirmed" "$(bar "$confirmed" "$scale")" "" "$confirmed" "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "SSA keys recovered" "$(bar "$keys" "$scale")" "" "$keys" "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "swept into Safe" "$(bar "$sweeps" "$scale")" "$C_GREEN" "$sweeps" "$C_RESET"
  if [ "${made_failed:-0}" -gt 0 ]; then
    printf '    %-22s %s%4d%s  %sthe float is spent — kill switch arming%s\n' \
      "deposits refused" "$C_YELLOW" "$made_failed" "$C_RESET" "$C_DIM" "$C_RESET"
  fi
  printf '\n'

  printf '  %sMONEY%s\n' "$C_BOLD" "$C_RESET"
  printf '    %-22s %s%s wxHOPR%s' "Exit Safe received" "$C_GREEN$C_BOLD" "$recovered" "$C_RESET"
  # Cycle count derived from the amount, not from `$sweeps`: the sweep counter increments as
  # soon as the withdrawal call returns, so it can lead the Safe balance by a block or two,
  # and pairing the two would print an equation that does not hold.
  if [ -n "$per_cycle" ]; then
    local landed
    landed=$(echo "scale=0; $recovered / $per_cycle" | bc -l 2>/dev/null)
    printf '  %s= %s × %s%s' "$C_DIM" "${landed:-0}" "$per_cycle" "$C_RESET"
  fi
  printf '\n'
  printf '    %-22s %s wxHOPR %sremaining for deposits%s\n' "Entry float" "$entry_float" "$C_DIM" "$C_RESET"
  [ -n "$last_sweep" ] && printf '    %-22s %s wxHOPR\n' "last sweep" "$last_sweep"
  printf '\n'

  printf '  %sTRAFFIC%s  %sthe Exit unlocks one share per SURB it spends replying%s\n\n' \
    "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  printf '    %-22s %12s pkts  %s->%s  %12s recv\n' "Entry -> Exit" "$(num "$e_sent")" "$C_DIM" "$C_RESET" "$(num "$x_recv")"
  printf '    %-22s %12s pkts  %s<-%s  %12s recv\n' "Exit  -> Entry" "$(num "$x_sent")" "$C_DIM" "$C_RESET" "$(num "$e_recv")"
  printf '    %-22s %12s pkts\n' "Relay forwarded" "$(num "$r_fwd")"
  # The headline of the run, so it gets the same emphasis as the money. Kept on one line in
  # the same value column as the totals above rather than a fourth column on each row: those
  # rows already end at column 67 inside a 74-wide frame.
  printf '    %-22s %s%12s pkt/s fwd%s  %s·%s  %s%s pkt/s return%s\n' \
    "current rate" \
    "$C_CYAN$C_BOLD" "$(num "$fwd_rate")" "$C_RESET" \
    "$C_DIM" "$C_RESET" \
    "$C_CYAN$C_BOLD" "$(num "$ret_rate")" "$C_RESET"
  printf '\n'

  # The relay is paid separately for each leg, by whoever issued the tickets on it — a second,
  # independent incentive running alongside the Entry paying the Exit. Only a small fraction of
  # packets carry a winning ticket, so these counts are far below the packet counts above.
  printf '  %sRELAY EARNINGS%s  %seach leg is its own channel, paid for by whoever sends on it%s\n\n' \
    "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  printf '    %-22s %12s %swinning%s  %s%16s wxHOPR%s\n' \
    "from Entry (fwd leg)" "$(num "$fwd_win")" "$C_DIM" "$C_RESET" "$C_GREEN$C_BOLD" "$fwd_val" "$C_RESET"
  printf '    %-22s %12s %swinning%s  %s%16s wxHOPR%s\n' \
    "from Exit (return leg)" "$(num "$ret_win")" "$C_DIM" "$C_RESET" "$C_GREEN$C_BOLD" "$ret_val" "$C_RESET"
  printf '\n'

  if [ -n "${1:-}" ]; then
    printf '  %s%s%s\n' "$C_DIM" "$1" "$C_RESET"
  fi
}

# ── entry points ────────────────────────────────────────────────────────────────

mkdir -p "$STATE_DIR"

if [ "${1:-}" = "--dashboard" ]; then
  render "metrics: http://127.0.0.1:$API_PORT_BASE/metrics (+1, +2)"
  exit 0
fi

command -v jq >/dev/null || {
  echo "pix-demo needs jq"
  exit 1
}
command -v bc >/dev/null || {
  echo "pix-demo needs bc"
  exit 1
}
cargo nextest --version >/dev/null 2>&1 || {
  echo 'pix-demo needs cargo-nextest on PATH — try running it inside `nix develop`'
  exit 1
}

# Tear down whatever a previous run left behind. This is not hygiene, it is the difference
# between a rehearsal and the live run working: the chain container is a fixed name and the
# nodes bind a fixed port block, so one stale process from an interrupted run makes the next
# one fail during bootstrap — in front of the audience, two minutes in.
#
# The bracket in the pattern stops `pkill -f` matching the shell that is running this script,
# whose own command line contains the pattern; without it the script SIGTERMs itself.
reset_cluster() {
  docker rm -f hopr-chain >/dev/null 2>&1
  pkill -f "release/hoprd --configuration[F]ilePath" >/dev/null 2>&1
  sleep 2
}

: "${HOPRD_BIN:=$REPO_ROOT/target/release/hoprd}"
: "${HOPRD_CHAIN_IMAGE:=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest}"
export HOPRD_BIN HOPRD_CHAIN_IMAGE
[ -n "${PIX_DEMO_FLOAT:-}" ] && export HOPRD_PIX_SOAK_FLOAT="$PIX_DEMO_FLOAT"

if [ ! -x "$HOPRD_BIN" ]; then
  echo "no hoprd binary at $HOPRD_BIN — build it first:"
  echo "    cargo build --release -p hoprd"
  exit 1
fi

# Everything cached from a previous run has to go: `scrape`/`balance` deliberately keep the
# last good read when an endpoint refuses, which would otherwise show the *previous* run's
# totals during the couple of minutes this one takes to bring the cluster up.
# `addr_*` and `tickets_*` are the sharp ones: node identities are regenerated every run, so a
# surviving `addr_*` makes this run query the *previous* run's counterparties, get a 404, and
# fall back to that run's cached earnings — confidently displaying numbers from a cluster that
# no longer exists.
#
# `rate_*`/`rateval_*` are the same hazard in a third place: a stale sample would be paired
# against this run's counter, and since the counters restart from zero it would yield a
# negative delta — suppressed as "not advancing", leaving the previous run's rate frozen on
# screen. Add new cache families here at the same time as the helper that writes them.
rm -f "$STATE_DIR/baseline" "$STATE_DIR"/metrics_* "$STATE_DIR"/balance_* \
  "$STATE_DIR"/addr_* "$STATE_DIR"/tickets_* "$STATE_DIR"/rate_* "$STATE_DIR"/rateval_* \
  "$TEST_LOG"
reset_cluster
date +%s >"$STATE_DIR/started"

echo "starting the localcluster (chain, 3 nodes, channels) — this takes a couple of minutes"
echo "full test output: $TEST_LOG"
(cd "$REPO_ROOT" && cargo nextest run -p hoprd-localcluster --test session_pix_soak \
  --run-ignored ignored-only -j 1 --no-capture) >"$TEST_LOG" 2>&1 &
TEST_PID=$!

# Killing the nextest process alone is not enough: the three `hoprd` children and the chain
# container outlive it, and on Ctrl-C the test's own teardown never runs. Left behind, they
# are exactly what `reset_cluster` has to clear before the next attempt — so clear them here
# and a re-run needs no manual intervention. Safe on the normal-exit path too, where the
# test has already torn everything down and both commands are no-ops.
cleanup() {
  printf '\033[?25h'
  kill "$TEST_PID" 2>/dev/null
  wait "$TEST_PID" 2>/dev/null
  reset_cluster
}
trap cleanup EXIT INT TERM

printf '\033[?25l'
until curl -s --max-time 2 "http://127.0.0.1:$((API_PORT_BASE + 2))/readyz" >/dev/null 2>&1; do
  kill -0 "$TEST_PID" 2>/dev/null || {
    printf '\033[?25h'
    echo
    tail -30 "$TEST_LOG"
    exit 1
  }
  printf '\r  waiting for the cluster… %ss' "$(($(date +%s) - $(cat "$STATE_DIR/started")))"
  sleep 2
done
# The nodes answer /readyz before the Session opens; hold off until traffic is flowing so
# the first frame is not three columns of zeroes.
sleep 5

while kill -0 "$TEST_PID" 2>/dev/null; do
  render "Ctrl-C to stop early · full log: $TEST_LOG"
  sleep "$REFRESH"
done

wait "$TEST_PID"
STATUS=$?
render "run finished"
printf '\033[?25h'
echo
if [ "$STATUS" -eq 0 ]; then
  printf '  %s✓ PASSED%s — the Entry spent its float, every recovered deposit reached the Safe,\n' "$C_GREEN$C_BOLD" "$C_RESET"
  printf '    and the Exit closed the Session once deposits stopped arriving.\n'
else
  printf '  %s✗ FAILED%s — see %s\n' "$C_RED$C_BOLD" "$C_RESET" "$TEST_LOG"
  sed 's/\x1b\[[0-9;]*m//g' "$TEST_LOG" | grep -A4 "panicked at" | head -12
fi
exit "$STATUS"
