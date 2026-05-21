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
# Returns 2 (with an error message to stderr) when --configurationFilePath
# is present but has no value, so the caller never silently falls back to
# the env var in that case.
resolve_config_path() {
  local prev="" arg
  for arg in "$@"; do
    case "$arg" in
    --configurationFilePath=*)
      local val="${arg#*=}"
      if [ -z "$val" ]; then
        echo "Error: --configurationFilePath requires a non-empty value" >&2
        return 2
      fi
      echo "$val"
      return 0
      ;;
    --configurationFilePath)
      prev="match"
      continue
      ;;
    esac
    if [ "$prev" = "match" ]; then
      echo "$arg"
      return 0
    fi
  done
  if [ "$prev" = "match" ]; then
    echo "Error: --configurationFilePath requires a value" >&2
    return 2
  fi
  echo "${HOPRD_CONFIGURATION_FILE_PATH:-}"
  return 0
}

# Validate the config file when hoprd is about to run.
# The escape hatch (exec "$@" for another /bin/ binary) skips validation.
# Missing-file errors are surfaced by hoprd-cfg itself for consistent output.
if [ -z "${1:-}" ] || [ ! -f "/bin/${1:-}" ] || [ ! -x "/bin/${1:-}" ] || [ "${1:-}" = "hoprd" ]; then
  cfg_path="$(resolve_config_path "$@")" || exit 1
  if [ -n "$cfg_path" ]; then
    /bin/hoprd-cfg --validate "$cfg_path"
  fi
fi

if [ -n "${1:-}" ] && [ -f "/bin/${1:-}" ] && [ -x "/bin/${1:-}" ]; then
  exec "$@"
else
  exec /bin/hoprd "$@"
fi
