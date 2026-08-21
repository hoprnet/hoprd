#!/usr/bin/env bash
#
# Live dashboard for the PIX Session soak test.
#
# Runs `session_pix_soak` against a throwaway 4-node localcluster — an Entry, two relays and
# an Exit — and renders what all four are doing while it happens: traffic crossing the Session
# over both relays, SSA cycles advancing through deposit → confirmation → key recovery → sweep,
# and the Exit's Safe filling up one quota-sized deposit at a time. The run ends by itself when
# the Entry can no longer afford a deposit and the Exit's kill switch closes the Session.
#
# The single Session is one hop, but not one relay: hoprd redraws the route for every packet, so
# both relays carry it and both earn. The per-relay rows are where that shows.
#
#   ./localcluster/scripts/pix-demo.sh                  # ~7 minutes
#   PIX_DEMO_FLOAT="150 wxHOPR" ./localcluster/scripts/pix-demo.sh    # more cycles
#   PIX_DEMO_RATE=6000 ./localcluster/scripts/pix-demo.sh             # faster
#
# The flow was measured to sustain 6000 datagrams/s each way and to saturate by 6500; the
# committed default is 4000, for the margin reasons on `DEFAULT_PACKET_RATE` in the test.
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
# The hoprd binary carries exactly one deposit pool, chosen at build time by a `strategy-pix-*`
# feature; `PIX_POOL` (default `secp256k1`) says which one this run expects, and the binary is
# checked against it before the cluster is started. `PIX_POOL=curvy` selects the Baby JubJub pool,
# which is currently a stub that panics — it exists so the wiring can be exercised end to end.
#
# Safe to re-run: a stale chain container or leftover nodes from an interrupted attempt are
# cleared on the way in, and Ctrl-C tears the cluster down on the way out.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
API_PORT_BASE=13500
# Node index → role, matching `session_pix_soak`'s own ENTRY/RELAYS/EXIT. Every reading below is
# taken by index, so this is the only place the mapping is stated. Node `i` scrapes on
# `API_PORT_BASE + i`.
ENTRY_IDX=0
RELAY_IDXS=(1 2)
EXIT_IDX=3
ALL_IDXS=("$ENTRY_IDX" "${RELAY_IDXS[@]}" "$EXIT_IDX")
# Fixed rather than under $TMPDIR: the test runs inside `nix develop`, which sets its own
# TMPDIR, and `--dashboard` is meant to be usable from any shell against the same run — so the
# name has to be predictable, which rules out `mktemp -d`. A fixed name under a world-writable
# /tmp is only safe if it is checked, which the entry point below does; `PIX_DEMO_STATE_DIR` is
# the way out when the check fails, and both the run and its dashboard must be given the same one.
: "${PIX_DEMO_STATE_DIR:=/tmp/pix-demo}"
STATE_DIR="$PIX_DEMO_STATE_DIR"
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

# Sustained packets per second for one direction, from a cumulative counter.
#
#   pkt_rate <key> <current count>
#
# The running **maximum of the cumulative average** since traffic began, which is the one
# formulation that needs no special case for the closing frame. Two simpler readings were tried
# and both are wrong there:
#
#   * *Instantaneous over a short window* decays to a true but absurd "2 pkt/s". A run does not
#     stop at its last packet — the kill switch closes the Session and the test then spends up to
#     a minute polling for in-flight sweeps, all while this loop keeps rendering, and the counters
#     advance by a trickle of acks over that span. Freezing at the end does not help: the decay
#     happens *during* the run, before any end-of-run flag could be set.
#   * *Peak over a short window* overstates by ~1.5×, because filling the Exit's SURB buffer at
#     Session start mints tens of thousands of keep-alive packets in a burst.
#
# A cumulative average has neither failure. It climbs through the warm-up, converges on the
# sustained rate (the start-up burst is a few seconds inside a window of minutes, so it barely
# moves it), then decays once traffic stops — so its maximum is what the run actually held, and
# it stays put from there on. Measured 8504 pkt/s sustained against a 12386 windowed peak.
#
# The anchor keeps sliding forward until the average clears [`TRAFFIC_FLOOR`], so it ends up at
# the start of Session traffic rather than at the counter's first advance. Bootstrap is not
# silent — probes and channel operations move the counter for ~190 s beforehand — and anchoring
# on the first non-zero reading put ~24 s of that in the denominator, understating the result by
# 15%. Nothing separates the two phases by kind, but three orders of magnitude separates them by
# rate.
TRAFFIC_FLOOR=200
pkt_rate() { # key current_count
  local anchor="$STATE_DIR/rate_$1"
  local shown="$STATE_DIR/rateval_$1"
  local now
  local count=$2
  local t0=""
  local c0=""
  now=$(date +%s)
  [ -r "$anchor" ] && read -r t0 c0 <"$anchor"
  # Both fields have to be present and numeric before they are arithmetic operands: a truncated
  # anchor would otherwise reach `test` with an empty operand, which is a syntax error rather
  # than a false and would spray onto the frame.
  case "${t0}:${c0}" in
  *[!0-9:]* | :* | *: | '') t0="" ;;
  esac
  # `-le` rather than `-lt` also re-anchors a stale anchor from a previous run, whose counters
  # restart from zero and would otherwise give a negative average.
  if [ -z "$t0" ] || [ "$count" -le "$c0" ]; then
    [ "$count" -gt 0 ] && printf '%s %s\n' "$now" "$count" >"$anchor"
  elif [ "$now" -gt "$t0" ]; then
    local avg
    avg=$(printf '%.0f' "$(echo "($count - $c0) / ($now - $t0)" | bc -l)")
    if [ "$avg" -lt "$TRAFFIC_FLOOR" ]; then
      # Still bootstrap. Slide the anchor forward so it lands at the moment traffic starts.
      printf '%s %s\n' "$now" "$count" >"$anchor"
    elif [ "$avg" -gt "$(cat "$shown" 2>/dev/null || echo 0)" ]; then
      printf '%s\n' "$avg" >"$shown"
    fi
  fi
  if [ -r "$shown" ]; then cat "$shown"; else echo 0; fi
}

# Shared by `cpu_pct`, which is still a windowed reading: CPU has no cumulative-average
# equivalent that isolates the traffic phase, since a node burns ticks throughout bootstrap too.
RATE_WINDOW=6

# CPU ticks consumed by one node's hoprd process, or empty if it is not running.
#
# Nodes are identified by `--apiPort`, which `client_helper` puts on the command line and which
# is the same port this script already scrapes — so the mapping to Entry/relay/Exit is exact
# rather than positional. The `[ ]` is the same self-match guard as `reset_cluster`'s `pkill`:
# harmless here, since the pattern only lives in this file, but it keeps the line safe to
# paste into a shell while debugging.
#
# The binary is matched as `hoprd` rather than `release/hoprd`: $HOPRD_BIN is overridable and
# need not live under target/release, and a pattern that misses just makes the CPU column read
# blank forever. `--apiPort` on this script's own fixed port block is the discriminator.
#
# `/proc/<pid>/stat` field 2 is the comm in parentheses and may contain spaces, so everything
# up to the last `)` is dropped before splitting — after which utime and stime are fields 12
# and 13 rather than 14 and 15.
node_ticks() { # node-index
  local pid
  pid=$(pgrep -f "hoprd .*--apiPort[ ]$((API_PORT_BASE + $1))" | head -1)
  [ -z "$pid" ] && return 0
  awk '{ s = $0; sub(/^.*\) /, "", s); split(s, f, " "); print f[12] + f[13] }' \
    "/proc/$pid/stat" 2>/dev/null
}

CLK_TCK=$(getconf CLK_TCK 2>/dev/null || echo 100)
# Peak per-process CPU percentage over any $RATE_WINDOW window. 100% is one core.
#
# Deliberately *not* `ps -o %cpu`, which averages over the process's whole lifetime — on a run
# that spends its first three minutes bootstrapping, that reads far below what the traffic
# phase is actually costing.
#
# The freeze rule differs from `pkt_rate`: there, a counter that stops advancing means the
# nodes are gone, so the last figure is kept. Here the two cases are distinguishable and want
# different answers — a *missing process* keeps the last figure (the closing frame), while a
# *running but idle* process should genuinely read 0, since a node falling idle mid-run is a
# fault worth seeing rather than hiding.
cpu_pct() { # node-index
  local sample="$STATE_DIR/cpu_$1"
  local shown="$STATE_DIR/cpuval_$1"
  local now ticks prev_t prev_k elapsed
  ticks=$(node_ticks "$1")
  if [ -n "$ticks" ]; then
    now=$(date +%s)
    prev_t=""
    prev_k=""
    [ -r "$sample" ] && read -r prev_t prev_k <"$sample"
    case "${prev_t}:${prev_k}" in
    *[!0-9:]* | :* | *: | '') prev_t="" ;;
    esac
    if [ -n "$prev_t" ]; then
      elapsed=$((now - prev_t))
      # A high-water mark for the same reason as `pkt_rate`: by the closing frame the nodes are
      # idle or already gone, and an instantaneous read renders 1% beside a run that was using
      # several cores a moment earlier.
      if [ "$elapsed" -ge "$RATE_WINDOW" ]; then
        local this
        this=$(printf '%.0f' "$(echo "($ticks - $prev_k) * 100 / $CLK_TCK / $elapsed" | bc -l)")
        [ "$this" -gt "$(cat "$shown" 2>/dev/null || echo 0)" ] && printf '%s\n' "$this" >"$shown"
        printf '%s %s\n' "$now" "$ticks" >"$sample"
      fi
    else
      printf '%s %s\n' "$now" "$ticks" >"$sample"
    fi
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

# One field of relay `$1`'s ticket statistics for the incoming channel from `$2`.
#
# Scoping by counterparty is what separates the two directions: a relay's forward leg and
# return leg are two different incoming channels, earning independently, and the unscoped
# aggregate adds them together. With two relays it also separates the two of them, since each
# has its own channel from the Entry and from the Exit. Auto-redeeming is off for this test, so
# nothing is ever moved out of `unredeemedValue` — it is the whole of what the leg has earned.
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
  for i in "${ALL_IDXS[@]}"; do scrape "$i"; done

  local sweeps deposits made_failed confirmed keys last_sweep
  sweeps=$(metric "$EXIT_IDX" hopr_strategy_pix_sweeps)
  deposits=$(metric "$ENTRY_IDX" hopr_strategy_pix_deposits)
  made_failed=$(metric "$ENTRY_IDX" hopr_strategy_pix_deposits_failed)
  confirmed=$(metric "$EXIT_IDX" hopr_strategy_pix_deposit_tracking 'outcome="confirmed"')
  keys=$(metric "$EXIT_IDX" hopr_strategy_pix_keys_recovered)
  last_sweep=$(grep -E '^hopr_strategy_pix_last_sweep_hopr' "$STATE_DIR/metrics_$EXIT_IDX" 2>/dev/null |
    awk '{ print $NF }' | head -1)

  # The Exit's Safe only ever *gains* wxHOPR from PIX sweeps, so its growth over a baseline is
  # the recovered total. Choosing when to stop moving that baseline is the whole difficulty.
  #
  # It cannot be the first frame: opening the channels takes 900 wxHOPR back out of the Safe
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
  exit_safe=$(balance "$EXIT_IDX" safeHopr)
  if [ "$deposits" -eq 0 ] 2>/dev/null || [ ! -f "$STATE_DIR/baseline" ]; then
    echo "$exit_safe" >"$STATE_DIR/baseline"
  fi
  local baseline
  baseline=$(cat "$STATE_DIR/baseline")
  local recovered
  recovered=$(trim "$(echo "$exit_safe - $baseline" | bc -l)")

  local entry_float
  entry_float=$(balance "$ENTRY_IDX" hopr)
  local per_cycle
  per_cycle=$(from_log "per_cycle")
  local funded
  funded=$(from_log "funded_cycles")
  # The run's fixed parameters, announced once in the test's startup banner. All empty when
  # attached with `--dashboard` to a cluster somebody else started, in which case the geometry
  # line is simply skipped rather than shown full of blanks.
  local polys shares surplus emitted quota price_per_byte
  polys=$(from_log "ssa_polys")
  shares=$(from_log "ssa_shares")
  surplus=$(from_log "ssa_surplus")
  quota=$(from_log "quota")
  price_per_byte=$(from_log "price_per_byte")
  # The quota counts every share the generator emits, surplus included, so it is this sum —
  # not the threshold — that the headline arithmetic has to multiply by to reach the quota.
  [ -n "$shares" ] && [ -n "$surplus" ] && emitted=$((shares + surplus))
  [ -f "$STATE_DIR/started" ] || date +%s >"$STATE_DIR/started"
  local started
  started=$(cat "$STATE_DIR/started")
  local elapsed=$(($(date +%s) - started))

  # Relay earnings, per relay and per direction. The forward leg's tickets are issued by the
  # Entry, the return leg's by the Exit, so each node's address names the channel that direction
  # pays on — and every relay has one channel from each.
  local entry_addr exit_addr
  entry_addr=$(node_address "$ENTRY_IDX")
  exit_addr=$(node_address "$EXIT_IDX")
  local -a fwd_win fwd_val ret_win ret_val
  local r
  for r in "${RELAY_IDXS[@]}"; do
    fwd_win+=("$(ticket_stat "$r" "$entry_addr" winningCount)")
    fwd_val+=("$(ticket_stat "$r" "$entry_addr" unredeemedValue)")
    ret_win+=("$(ticket_stat "$r" "$exit_addr" winningCount)")
    ret_val+=("$(ticket_stat "$r" "$exit_addr" unredeemedValue)")
  done

  local e_sent x_recv x_sent e_recv
  e_sent=$(metric "$ENTRY_IDX" hopr_packets_count 'type="sent"')
  e_recv=$(metric "$ENTRY_IDX" hopr_packets_count 'type="received"')
  x_sent=$(metric "$EXIT_IDX" hopr_packets_count 'type="sent"')
  x_recv=$(metric "$EXIT_IDX" hopr_packets_count 'type="received"')

  # Per-relay forwarding, and each relay's share of the two totals. This is where the route
  # actually shows: nothing in the test spreads the traffic, hoprd redraws the path per packet,
  # so two rows climbing together is the split happening. A relay stuck at zero means it never
  # entered the path selector's candidate set — an unopened channel or a missed announcement.
  local -a r_fwd r_share
  local r_total=0
  for r in "${RELAY_IDXS[@]}"; do
    r_fwd+=("$(metric "$r" hopr_packets_count 'type="forwarded"')")
    r_total=$((r_total + ${r_fwd[-1]}))
  done
  local n
  for n in "${r_fwd[@]}"; do
    if [ "$r_total" -gt 0 ]; then r_share+=("$((n * 100 / r_total))"); else r_share+=(0); fi
  done

  # Derived from the same counters as the totals above, so the rate and the running total can
  # never tell different stories. These are node-wide HOPR packet counts, which include the
  # SURB keep-alives the balancer sends and the acknowledgements every packet earns — so both
  # figures sit above the Session's datagram rate. That is the honest HOPR packet rate, and
  # the label says "pkt/s" rather than anything implying datagrams.
  local fwd_rate ret_rate
  fwd_rate=$(pkt_rate fwd "$e_sent")
  ret_rate=$(pkt_rate ret "$x_sent")

  # What that rate costs. Worth showing next to it: all four nodes are Sphinx-processing every
  # packet in both directions on one machine. The relays land between the two endpoints —
  # measured 527% Entry, 341% and 349% for the relays, 269% Exit — because each carries about
  # half the packets but both directions of its half.
  local cpu_e cpu_x
  cpu_e=$(cpu_pct "$ENTRY_IDX")
  cpu_x=$(cpu_pct "$EXIT_IDX")
  local -a cpu_r
  for r in "${RELAY_IDXS[@]}"; do cpu_r+=("$(cpu_pct "$r")"); done

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
  if [ -n "$polys" ] && [ -n "${emitted:-}" ]; then
    printf '  %sSSA GEOMETRY%s         %s%s polys × %s shares%s  %s→%s  %s%s B%s %squota per SSA%s\n' \
      "$C_BOLD" "$C_RESET" "$C_BOLD" "$polys" "$emitted" "$C_RESET" "$C_DIM" "$C_RESET" \
      "$C_BOLD" "$(num "${quota:-0}")" "$C_RESET" "$C_DIM" "$C_RESET"
    printf '                       %s%s to reconstruct + %s surplus, all of them billed%s\n' \
      "$C_DIM" "$shares" "$surplus" "$C_RESET"
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
  # One row per relay with its share, because the share is the point: the Session is one hop but
  # not one relay, and these two rows climbing together is hoprd redrawing the route per packet.
  local i
  for i in "${!RELAY_IDXS[@]}"; do
    printf '    %-22s %12s pkts  %s%3s%%%s %sof relayed%s\n' \
      "Relay $((i + 1)) forwarded" "$(num "${r_fwd[i]}")" \
      "$C_CYAN" "${r_share[i]}" "$C_RESET" "$C_DIM" "$C_RESET"
  done
  # The headline of the run, so it gets the same emphasis as the money. Kept on one line in
  # the same value column as the totals above rather than a fourth column on each row: those
  # rows already end at column 67 inside a 74-wide frame.
  printf '    %-22s %s%12s pkt/s fwd%s  %s·%s  %s%s pkt/s return%s\n' \
    "sustained rate" \
    "$C_CYAN$C_BOLD" "$(num "$fwd_rate")" "$C_RESET" \
    "$C_DIM" "$C_RESET" \
    "$C_CYAN$C_BOLD" "$(num "$ret_rate")" "$C_RESET"
  # 100% is one core, so these routinely exceed it — hoprd is multi-threaded and every packet
  # costs a Sphinx unwrap per hop. Four columns fit in the same 69 that three did only with the
  # labels shortened and the separators tightened; widening any of them overruns the frame.
  printf '    %-22s %sEntry %3s%%%s %s·%s %sR1 %3s%%%s %s·%s %sR2 %3s%%%s %s·%s %sExit %3s%%%s\n' \
    "peak node CPU" \
    "$C_YELLOW" "$cpu_e" "$C_RESET" "$C_DIM" "$C_RESET" \
    "$C_YELLOW" "${cpu_r[0]}" "$C_RESET" "$C_DIM" "$C_RESET" \
    "$C_YELLOW" "${cpu_r[1]}" "$C_RESET" "$C_DIM" "$C_RESET" \
    "$C_YELLOW" "$cpu_x" "$C_RESET"
  printf '\n'

  # A relay is paid separately for each leg, by whoever issued the tickets on it — a second,
  # independent incentive running alongside the Entry paying the Exit. Only a small fraction of
  # packets carry a winning ticket, so these counts are far below the packet counts above.
  #
  # Four rows rather than a relay × leg matrix: a matrix cell wide enough for a count and a
  # wxHOPR value needs 84 columns for two of them, and the frame is 74.
  printf '  %sRELAY EARNINGS%s  %seach leg is its own channel, paid for by whoever sends on it%s\n\n' \
    "$C_BOLD" "$C_RESET" "$C_DIM" "$C_RESET"
  for i in "${!RELAY_IDXS[@]}"; do
    printf '    %-22s %12s %swinning%s  %s%16s wxHOPR%s\n' \
      "Relay $((i + 1)) <- Entry (fwd)" "$(num "${fwd_win[i]}")" \
      "$C_DIM" "$C_RESET" "$C_GREEN$C_BOLD" "${fwd_val[i]}" "$C_RESET"
    printf '    %-22s %12s %swinning%s  %s%16s wxHOPR%s\n' \
      "Relay $((i + 1)) <- Exit (ret)" "$(num "${ret_win[i]}")" \
      "$C_DIM" "$C_RESET" "$C_GREEN$C_BOLD" "${ret_val[i]}" "$C_RESET"
  done
  printf '\n'

  if [ -n "${1:-}" ]; then
    printf '  %s%s%s\n' "$C_DIM" "$1" "$C_RESET"
  fi
}

# ── entry points ────────────────────────────────────────────────────────────────

# Every helper above caches a reading into $STATE_DIR and several read it straight back, so a
# directory somebody else controls is a directory that decides what this script displays — and
# a symlink there redirects each of those writes to wherever it points. /tmp being world-writable
# means an attacker only has to create the name first, which is cheap and needs no privileges.
# Refuse rather than adopt it: the run is worth less than the machine.
mkdir -p "$STATE_DIR" 2>/dev/null
if [ -L "$STATE_DIR" ] || [ ! -d "$STATE_DIR" ] || [ ! -O "$STATE_DIR" ]; then
  echo "$STATE_DIR must be a directory owned by $(id -un) and not a symlink."
  echo "Remove it, or point PIX_DEMO_STATE_DIR somewhere else (the dashboard needs the same value)."
  exit 1
fi

if [ "${1:-}" = "--dashboard" ]; then
  render "metrics: http://127.0.0.1:$API_PORT_BASE/metrics (+1, +2, +3)"
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
# Matched on this script's own fixed API port block rather than on the binary's path. A pattern
# of `release/hoprd` misses any $HOPRD_BIN built somewhere else — the stale nodes then survive
# the reset and take the ports the next attempt needs, which is the exact failure this function
# exists to prevent. Matching by port is also narrower: an unrelated hoprd on the same machine
# is now left alone, where the old pattern killed every node started from a config file.
#
# The bracket in the pattern stops `pkill -f` matching the shell that is running this script,
# whose own command line contains the pattern; without it the script SIGTERMs itself.
reset_cluster() {
  docker rm -f hopr-chain >/dev/null 2>&1
  local i
  for i in "${ALL_IDXS[@]}"; do
    pkill -f "hoprd .*--apiPort[ ]$((API_PORT_BASE + i))" >/dev/null 2>&1
  done
  sleep 2
}

: "${HOPRD_BIN:=$REPO_ROOT/target/release/hoprd}"
: "${HOPRD_CHAIN_IMAGE:=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest}"
export HOPRD_BIN HOPRD_CHAIN_IMAGE
[ -n "${PIX_DEMO_FLOAT:-}" ] && export HOPRD_PIX_SOAK_FLOAT="$PIX_DEMO_FLOAT"
[ -n "${PIX_DEMO_RATE:-}" ] && export HOPRD_PIX_SOAK_RATE="$PIX_DEMO_RATE"

# The deposit pool is a *build-time* choice in the binary, and this script runs a prebuilt one.
# A binary built with the other pairing starts and bootstraps normally, then either never
# deposits (wrong curve) or panics (curvy, whose pool is a stub) — several minutes in, with the
# audience watching. So check it before spending that time.
#
# `POOL` in `hoprd::strategy` is a `&str` compiled into the binary for exactly this, and for the
# `pool=` field of the node's "enabling the PIX strategy" log line.
: "${PIX_POOL:=secp256k1}"
case "$PIX_POOL" in
secp256k1) POOL_MARKER="non-anonymous-secp256k1" ;;
curvy) POOL_MARKER="curvy" ;;
*)
  echo "PIX_POOL must be 'secp256k1' or 'curvy', got '$PIX_POOL'"
  exit 1
  ;;
esac
# Additive to the default feature set: neither pairing is default, so this is the only flag.
BUILD_CMD="cargo build --release -p hoprd --features strategy-pix-$PIX_POOL"

if [ ! -x "$HOPRD_BIN" ]; then
  echo "no hoprd binary at $HOPRD_BIN — build it first:"
  echo "    $BUILD_CMD"
  exit 1
fi

if ! grep -qa "$POOL_MARKER" "$HOPRD_BIN"; then
  echo "$HOPRD_BIN was not built with the '$PIX_POOL' deposit pool."
  echo "Rebuild it:"
  echo "    $BUILD_CMD"
  echo
  echo "(Or set PIX_POOL to match the binary. The pools are mutually exclusive and the"
  echo " binary carries exactly one.)"
  exit 1
fi

if [ "$PIX_POOL" = "curvy" ]; then
  echo "PIX_POOL=curvy selects CurvyDepositPool, whose methods are unimplemented and panic."
  echo "The cluster will bootstrap and then die on the first deposit. This is expected until"
  echo "the Baby JubJub pool is implemented; use PIX_POOL=secp256k1 for a run that completes."
  echo
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
  "$STATE_DIR"/cpu_* "$STATE_DIR"/cpuval_* \
  "$TEST_LOG"
reset_cluster
date +%s >"$STATE_DIR/started"

echo "starting the localcluster (chain, 4 nodes, 12 channels) — this takes a few minutes"
echo "full test output: $TEST_LOG"
(cd "$REPO_ROOT" && cargo nextest run -p hoprd-localcluster --test session_pix_soak \
  --run-ignored ignored-only -j 1 --no-capture) >"$TEST_LOG" 2>&1 &
TEST_PID=$!

# Killing the nextest process alone is not enough: the four `hoprd` children and the chain
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
until curl -s --max-time 2 "http://127.0.0.1:$((API_PORT_BASE + EXIT_IDX))/readyz" >/dev/null 2>&1; do
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
# the first frame is not four columns of zeroes.
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
