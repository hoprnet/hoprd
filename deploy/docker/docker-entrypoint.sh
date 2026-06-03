#!/usr/bin/env bash
set -euo pipefail

# Determine whether the first argument names a plain binary in /bin/. This is an
# escape hatch for debugging (e.g. `docker run <image> bash`). Reject values
# containing '/' (absolute paths, path traversal) so that /bin/${1} is always a
# safe single-level lookup and users cannot escape the /bin/ boundary.
_cmd="${1:-}"
case "$_cmd" in
"" | hoprd | */*) ;;
*)
  if [ -f "/bin/$_cmd" ] && [ -x "/bin/$_cmd" ]; then
    exec "/bin/$_cmd" "${@:2}"
  fi
  ;;
esac

# The default Cmd is ["hoprd"], so when present the command word is $1, not a
# flag; strip it so "$@" is the exact argument vector hoprd will receive.
if [ "$_cmd" = "hoprd" ]; then
  shift
fi

# Validate the effective configuration (YAML file plus env-var and CLI-flag
# overrides) exactly as hoprd builds it at startup, so misconfigurations fail
# fast before launch. hoprd-cfg surfaces missing-file errors and treats a
# --help/--version request as a no-op success.
echo "hoprd container startup attempt: validating configuration" >&2
if RUST_BACKTRACE=0 /bin/hoprd-cfg --validate-args -- "$@"; then
  echo "hoprd container startup attempt: configuration valid; launching hoprd" >&2
else
  exit_code=$?
  echo "hoprd container startup attempt: configuration validation failed; exiting with status ${exit_code}. A configured container restart policy may launch another attempt." >&2
  exit "$exit_code"
fi

exec /bin/hoprd "$@"
