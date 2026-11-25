#!/usr/bin/env bash
#
# Verifies that the relay-only hoprd artifact links none of the Session stack, and that the
# relaying exit does.
#
# hoprd builds two artifacts:
#   * relay: `--no-default-features -F relay,...`        ticket and acknowledgement processing only
#   * exit:  `--no-default-features -F exit,...`         the relay plus Sessions, the Session
#                                                         server and PIX (`exit` implies `relay`)
#
# A passing build only proves that a feature set compiles, not that it is smaller than the other.
# This script reads the resolved dependency graph (`cargo tree`) of each artifact and requires that
# the Session crates are absent from the relay and present in the exit. Every absence check has a
# control: a crate both artifacts must link, so a broken `cargo tree` call cannot pass as "absent".
#
# Usage: scripts/check-entity-fencing.sh
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

export CARGO_TERM_COLOR=never

# The features a deployment adds on top of the role; the role is the only difference between the two.
common="runtime-tokio,telemetry,transport-quic,state-dump"
relay_features="relay,${common}"
exit_features="exit,${common}"

# Crates only a node that handles Sessions links.
session_crates=(
  hopr-transport-session
  hopr-utils-session
  hopr-session-server-forwarder
  hopr-protocol-session
  hopr-protocol-start
)
# Crates every artifact links; these are the controls.
control_crates=(
  hopr-lib
  hopr-protocol-hopr
  hopr-ticket-manager
)

failures=0
fail() {
  echo "FAIL: $*" >&2
  failures=$((failures + 1))
}
pass() {
  echo "ok:   $*"
}

# `$1`: feature set. Prints one `name vX.Y.Z` line per crate in the normal dependency graph of hoprd.
crates_of() {
  cargo tree -p hoprd --no-default-features -F "$1" -e normal --prefix none | sort -u
}

# `$1`: feature set. Prints the features cargo enabled on `hopr-lib` and `hoprd-api`.
features_of() {
  {
    cargo tree -p hoprd --no-default-features -F "$1" -e features -i hopr-lib
    cargo tree -p hoprd --no-default-features -F "$1" -e features -i hoprd-api
  } | grep -E '(hopr-lib|hoprd-api) feature' | sed -E 's/^[^a-z]*//; s/ \(\*\)$//' | sort -u
}

relay_crates=$(crates_of "$relay_features")
exit_crates=$(crates_of "$exit_features")
relay_state=$(features_of "$relay_features")
exit_state=$(features_of "$exit_features")

has_crate() { grep -Eq "^$2 v" <<<"$1"; }
has_feature() { grep -Fxq "$2" <<<"$1"; }

for crate in "${control_crates[@]}"; do
  has_crate "$relay_crates" "$crate" || fail "control: relay does not link $crate, so the checks below prove nothing"
  has_crate "$exit_crates" "$crate" || fail "control: exit does not link $crate, so the checks below prove nothing"
done

for crate in "${session_crates[@]}"; do
  if has_crate "$relay_crates" "$crate"; then
    fail "relay links $crate"
  else
    pass "relay does not link $crate"
  fi
  if has_crate "$exit_crates" "$crate"; then
    pass "exit links $crate"
  else
    fail "exit does not link $crate"
  fi
done

# The crates can be absent for the wrong reason (a renamed package), so the feature state is checked too.
for feature in 'hopr-lib feature "session-entry"' 'hopr-lib feature "session-exit"' 'hoprd-api feature "session"'; do
  if has_feature "$relay_state" "$feature"; then
    fail "relay enables $feature"
  else
    pass "relay does not enable $feature"
  fi
  if has_feature "$exit_state" "$feature"; then
    pass "exit enables $feature"
  else
    fail "exit does not enable $feature"
  fi
done
has_feature "$relay_state" 'hopr-lib feature "relay"' || fail 'relay does not enable hopr-lib feature "relay"'
has_feature "$exit_state" 'hopr-lib feature "relay"' || fail 'exit does not enable hopr-lib feature "relay"'

if [[ $failures -gt 0 ]]; then
  echo "$failures check(s) failed" >&2
  exit 1
fi
echo "entity fence holds"
