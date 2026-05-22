#!/usr/bin/env bash
set -euo pipefail

# Locate the CA bundle provided by pkgs.cacert (present in extraContents).
for candidate in \
  /etc/ssl/certs/ca-bundle.crt \
  /etc/ssl/certs/ca-certificates.crt; do
  if [ -f "$candidate" ]; then
    export SSL_CERT_FILE="$candidate"
    export NIX_SSL_CERT_FILE="$candidate"
    break
  fi
done

mkdir -p "$TMPDIR"

# Resolve the session listen host, filling in the container IP when not set.
listen_host="${HOPRD_DEFAULT_SESSION_LISTEN_HOST:-}"
case "$listen_host" in
*:*)
  listen_host_preset_ip="${listen_host%%:*}"
  listen_host_preset_port="${listen_host#*:}"
  ;;
*)
  listen_host_preset_ip="$listen_host"
  listen_host_preset_port=""
  ;;
esac

if [ -z "${listen_host_preset_ip:-}" ]; then
  listen_host_ip="$(hostname -i | {
    read -r first _rest
    echo "$first"
  })"

  if [ -z "${listen_host_preset_port:-}" ]; then
    listen_host="${listen_host_ip}:0"
  else
    listen_host="${listen_host_ip}:${listen_host_preset_port}"
  fi
fi

export HOPRD_DEFAULT_SESSION_LISTEN_HOST="$listen_host"

# Resolve the hoprd configuration file path from CLI args or env var.
# Mirrors clap's last-wins semantics: if --configurationFilePath appears
# multiple times, the last valid value is used.
# Returns 2 (with an error message to stderr) when --configurationFilePath
# is present but has no value, so the caller never silently falls back to
# the env var in that case.
resolve_config_path() {
  local found_val="" prev="" arg
  for arg in "$@"; do
    case "$arg" in
    --configurationFilePath=*)
      local val="${arg#*=}"
      if [ -z "$val" ]; then
        echo "Error: --configurationFilePath requires a non-empty value" >&2
        return 2
      fi
      found_val="$val"
      prev=""
      continue
      ;;
    --configurationFilePath)
      prev="match"
      continue
      ;;
    esac
    if [ "$prev" = "match" ]; then
      # Treat an empty value or another flag as a missing value.
      case "$arg" in
      "" | --*)
        echo "Error: --configurationFilePath requires a non-empty value" >&2
        return 2
        ;;
      esac
      found_val="$arg"
      prev=""
      continue
    fi
  done
  if [ "$prev" = "match" ]; then
    echo "Error: --configurationFilePath requires a non-empty value" >&2
    return 2
  fi
  if [ -n "$found_val" ]; then
    echo "$found_val"
  else
    echo "${HOPRD_CONFIGURATION_FILE_PATH:-}"
  fi
  return 0
}

# Determine whether the first argument names a plain binary in /bin/.
# Reject values containing '/' (absolute paths, path traversal) so that
# /bin/${1} is always a safe single-level lookup and users cannot escape
# the /bin/ boundary.
_cmd="${1:-}"
case "$_cmd" in
"" | hoprd | */*)
  _is_escape_hatch=0
  ;;
*)
  if [ -f "/bin/$_cmd" ] && [ -x "/bin/$_cmd" ]; then
    _is_escape_hatch=1
  else
    _is_escape_hatch=0
  fi
  ;;
esac

# Validate the config file when hoprd is about to run.
# Missing-file errors are surfaced by hoprd-cfg itself for consistent output.
if [ "$_is_escape_hatch" -eq 0 ]; then
  cfg_path="$(resolve_config_path "$@")" || exit $?
  if [ -n "$cfg_path" ]; then
    /bin/hoprd-cfg --validate "$cfg_path"
  fi
fi

if [ "$_is_escape_hatch" -eq 1 ]; then
  exec "/bin/$_cmd" "${@:2}"
elif [ "$_cmd" = "hoprd" ]; then
  # Default Cmd is ["hoprd"], so $1 is the command name, not a flag — strip it.
  exec /bin/hoprd "${@:2}"
else
  exec /bin/hoprd "$@"
fi
