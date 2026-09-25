#!/usr/bin/env sh
# Entrypoint for the prebuilt libtest executable, on the Linux host.
set -eu
: "${HOPRD_BIN:?prebuilt hoprd required}"
: "${HOPRD_PIX_SOAK_BIN:?prebuilt session_pix_soak required}"
test -x "$HOPRD_BIN"
test -x "$HOPRD_PIX_SOAK_BIN"
test -d "$CURVY_ZK_KEYS_DIR"
# A caller's shell must not select another chain/funding mode.
unset HOPRD_CURVY_OPERATOR_PRIVATE_KEY HOPRD_CURVY_OPERATOR_PRIVATE_KEYS
unset HOPRD_DEPLOYER_PRIVATE_KEY HOPRD_CURVY_INITIAL_FUNDING
unset HOPRD_PIX_SOAK_POOL_PREFUNDED HOPRD_PIX_FLOAT_NODE_IDS
test_name=localcluster_pix_session_runs_until_the_entry_cannot_deposit
# libtest returns success for zero selected tests. Refuse a stale/wrong executable.
listing=$("$HOPRD_PIX_SOAK_BIN" --list --exact "$test_name")
printf '%s\n' "$listing" | grep -qx "$test_name: test"
exec "$HOPRD_PIX_SOAK_BIN" --ignored --nocapture --test-threads=1 --exact "$test_name"
