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
# Requires: curl, jq. Plus whatever the test itself needs — see the test's module docs
# for HOPRD_BIN and HOPRD_CHAIN_IMAGE.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
API_PORT_BASE=13500
NODES=(Entry Relay Exit)
# Fixed rather than under $TMPDIR: the test runs inside `nix develop`, which sets its own
# TMPDIR, and `--dashboard` is meant to be usable from any shell against the same run.
STATE_DIR="/tmp/pix-demo"
TEST_LOG="$STATE_DIR/test.log"
REFRESH=2

C_RESET=$'\033[0m'; C_DIM=$'\033[2m'; C_BOLD=$'\033[1m'
C_CYAN=$'\033[36m'; C_GREEN=$'\033[32m'; C_YELLOW=$'\033[33m'; C_RED=$'\033[31m'

# ── metric helpers ──────────────────────────────────────────────────────────────

# Scrape one node's Prometheus endpoint into a cache file.
scrape() {
  curl -s --max-time 3 "http://127.0.0.1:$((API_PORT_BASE + $1))/metrics" \
    > "$STATE_DIR/metrics_$1" 2>/dev/null
}

# Sum a metric across label sets. `_total` is optional because OpenTelemetry's Prometheus
# exporter may or may not append it depending on version.
metric() {
  local file="$STATE_DIR/metrics_$1" name="$2" label="${3:-}"
  [ -r "$file" ] || { echo 0; return; }
  local lines; lines=$(grep -E "^${name}(_total)?[ {]" "$file" 2>/dev/null)
  if [ -n "$label" ]; then
    lines=$(printf '%s\n' "$lines" | grep -F -- "$label")
  fi
  printf '%s\n' "$lines" | awk '{ s += $NF } END { printf "%.0f", s + 0 }'
}

# A balance field from the REST API, unit stripped ("12.5 wxHOPR" -> "12.5").
balance() {
  curl -s --max-time 3 "http://127.0.0.1:$((API_PORT_BASE + $1))/api/v4/account/balances" 2>/dev/null \
    | jq -r ".$2 // \"0\"" 2>/dev/null \
    | awk 'NF { print $1; seen = 1; exit } END { if (!seen) print 0 }'
}

# Read a field out of the test's own startup banner, which is where the run's parameters
# (per-cycle deposit, funded cycles) are announced.
from_log() {
  [ -r "$TEST_LOG" ] || { echo ""; return; }
  sed 's/\x1b\[[0-9;]*m//g' "$TEST_LOG" 2>/dev/null \
    | grep -m1 -o "$1=[0-9.]*" | head -1 | cut -d= -f2
}

# Progress bar. Built by slicing pre-filled strings rather than repeating a character:
# `printf 'X%.0s'` with an empty argument list still prints one X, which silently puts a
# block in every empty bar.
FULL='████████████████████████████████'
EMPTY='░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░'
bar() { # value max [width]
  local value=${1:-0} max=${2:-0} width=${3:-24} filled=0
  if [ "$max" -gt 0 ] 2>/dev/null; then
    filled=$(( value * width / max ))
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

  local sweeps; sweeps=$(metric 2 hopr_strategy_pix_sweeps)

  # The Exit's Safe only ever *gains* wxHOPR from PIX sweeps, so its growth over a baseline
  # is the recovered total. The baseline keeps moving until the first sweep lands, because
  # everything before that point is setup rather than PIX — in particular opening the
  # channels takes 800 wxHOPR back out of the Safe, which pinning the baseline at the first
  # frame would report as a large negative recovery.
  local exit_safe; exit_safe=$(balance 2 safeHopr)
  if [ "$sweeps" -eq 0 ] 2>/dev/null || [ ! -f "$STATE_DIR/baseline" ]; then
    echo "$exit_safe" > "$STATE_DIR/baseline"
  fi
  local baseline; baseline=$(cat "$STATE_DIR/baseline")
  local recovered; recovered=$(trim "$(echo "$exit_safe - $baseline" | bc -l)")

  local entry_float; entry_float=$(balance 0 hopr)
  local per_cycle; per_cycle=$(from_log "per_cycle")
  local funded; funded=$(from_log "funded_cycles")
  [ -f "$STATE_DIR/started" ] || date +%s > "$STATE_DIR/started"
  local started; started=$(cat "$STATE_DIR/started")
  local elapsed=$(( $(date +%s) - started ))

  local deposits made_failed confirmed keys last_sweep
  deposits=$(metric 0 hopr_strategy_pix_deposits)
  made_failed=$(metric 0 hopr_strategy_pix_deposits_failed)
  confirmed=$(metric 2 hopr_strategy_pix_deposit_tracking 'outcome="confirmed"')
  keys=$(metric 2 hopr_strategy_pix_keys_recovered)
  last_sweep=$(grep -E '^hopr_strategy_pix_last_sweep_hopr' "$STATE_DIR/metrics_2" 2>/dev/null \
    | awk '{ print $NF }' | head -1)

  local e_sent x_recv x_sent e_recv r_fwd
  e_sent=$(metric 0 hopr_packets_count 'type="sent"')
  e_recv=$(metric 0 hopr_packets_count 'type="received"')
  x_sent=$(metric 2 hopr_packets_count 'type="sent"')
  x_recv=$(metric 2 hopr_packets_count 'type="received"')
  r_fwd=$(metric 1 hopr_packets_count 'type="forwarded"')

  # Bars are scaled to the cycles the float pays for, which the test announces at startup.
  # Attaching to a cluster somebody else started leaves that unknown, so fall back to
  # scaling against whatever the Entry has managed so far.
  local scale="${funded:-0}"
  if [ "$scale" -le 0 ] 2>/dev/null; then
    scale=$(( deposits > 0 ? deposits : 1 ))
  fi

  printf '\033[H\033[2J'
  printf '%s╔══════════════════════════════════════════════════════════════════════════╗%s\n' "$C_CYAN" "$C_RESET"
  printf '%s║%s  %sHOPR PIX Session%s — Entry pays the Exit for data it delivers      %s%02d:%02d%s  %s║%s\n' \
    "$C_CYAN" "$C_RESET" "$C_BOLD" "$C_RESET" "$C_BOLD" $((elapsed / 60)) $((elapsed % 60)) "$C_RESET" "$C_CYAN" "$C_RESET"
  printf '%s╚══════════════════════════════════════════════════════════════════════════╝%s\n\n' "$C_CYAN" "$C_RESET"

  printf '  %sSSA CYCLE PIPELINE%s   %seach cycle: Entry deposits, Exit confirms, collects\n' "$C_BOLD" "$C_RESET" "$C_DIM"
  printf '                        shares from the SURBs it spends, then sweeps%s\n\n' "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "Entry deposits"      "$(bar "$deposits"  "$scale")" "$C_BOLD"   "$deposits"  "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "Exit confirmed"      "$(bar "$confirmed" "$scale")" ""          "$confirmed" "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "SSA keys recovered"  "$(bar "$keys"      "$scale")" ""          "$keys"      "$C_RESET"
  printf '    %-22s %s %s%4d%s\n' "swept into Safe"     "$(bar "$sweeps"    "$scale")" "$C_GREEN"  "$sweeps"    "$C_RESET"
  if [ "${made_failed:-0}" -gt 0 ]; then
    printf '    %-22s %s%4d%s  %sthe float is spent — kill switch arming%s\n' \
      "deposits refused" "$C_YELLOW" "$made_failed" "$C_RESET" "$C_DIM" "$C_RESET"
  fi
  printf '\n'

  printf '  %sMONEY%s\n' "$C_BOLD" "$C_RESET"
  printf '    %-22s %s%s wxHOPR%s' "Exit Safe received" "$C_GREEN$C_BOLD" "$recovered" "$C_RESET"
  [ -n "$per_cycle" ] && printf '  %s= %d × %s%s' "$C_DIM" "$sweeps" "$per_cycle" "$C_RESET"
  printf '\n'
  printf '    %-22s %s wxHOPR %sremaining for deposits%s\n' "Entry float" "$entry_float" "$C_DIM" "$C_RESET"
  [ -n "$last_sweep" ] && printf '    %-22s %s wxHOPR\n' "last sweep" "$last_sweep"
  printf '\n'

  printf '  %sTRAFFIC%s  %sthe Exit unlocks one share per SURB it spends replying%s\n\n' \
    "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  printf '    %-22s %12s pkts  %s->%s  %12s recv\n' "Entry -> Exit" "$(num "$e_sent")" "$C_DIM" "$C_RESET" "$(num "$x_recv")"
  printf '    %-22s %12s pkts  %s<-%s  %12s recv\n' "Exit  -> Entry" "$(num "$x_sent")" "$C_DIM" "$C_RESET" "$(num "$e_recv")"
  printf '    %-22s %12s pkts\n' "Relay forwarded" "$(num "$r_fwd")"
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

command -v jq  >/dev/null || { echo "pix-demo needs jq";  exit 1; }
command -v bc  >/dev/null || { echo "pix-demo needs bc";  exit 1; }

: "${HOPRD_BIN:=$REPO_ROOT/target/release/hoprd}"
: "${HOPRD_CHAIN_IMAGE:=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest}"
export HOPRD_BIN HOPRD_CHAIN_IMAGE
[ -n "${PIX_DEMO_FLOAT:-}" ] && export HOPRD_PIX_SOAK_FLOAT="$PIX_DEMO_FLOAT"

if [ ! -x "$HOPRD_BIN" ]; then
  echo "no hoprd binary at $HOPRD_BIN — build it first:"
  echo "    cargo build --release -p hoprd"
  exit 1
fi

rm -f "$STATE_DIR/baseline" "$TEST_LOG"
date +%s > "$STATE_DIR/started"

echo "starting the localcluster (chain, 3 nodes, channels) — this takes a couple of minutes"
echo "full test output: $TEST_LOG"
( cd "$REPO_ROOT" && cargo nextest run -p hoprd-localcluster --test session_pix_soak \
    --run-ignored ignored-only -j 1 --no-capture ) > "$TEST_LOG" 2>&1 &
TEST_PID=$!

cleanup() {
  printf '\033[?25h'
  kill "$TEST_PID" 2>/dev/null
  wait "$TEST_PID" 2>/dev/null
}
trap cleanup EXIT INT TERM

printf '\033[?25l'
until curl -s --max-time 2 "http://127.0.0.1:$((API_PORT_BASE + 2))/readyz" >/dev/null 2>&1; do
  kill -0 "$TEST_PID" 2>/dev/null || { printf '\033[?25h'; echo; tail -30 "$TEST_LOG"; exit 1; }
  printf '\r  waiting for the cluster… %ss' "$(( $(date +%s) - $(cat "$STATE_DIR/started") ))"
  sleep 2
done
# The nodes answer /readyz before the Session opens; hold off until traffic is flowing so
# the first frame is not three columns of zeroes.
sleep 5

while kill -0 "$TEST_PID" 2>/dev/null; do
  render "Ctrl-C to stop early · full log: $TEST_LOG"
  sleep "$REFRESH"
done

wait "$TEST_PID"; STATUS=$?
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
