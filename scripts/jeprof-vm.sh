#!/usr/bin/env bash
# Run hoprd (single node or full localcluster) with jemalloc profiling
# on any Linux host reachable via SSH. jemalloc profiling is Linux-only,
# so source is synced to the Linux host, built there with Nix, and run there.
#
# Defaults assume an OrbStack NixOS VM. Override VM_HOST and CHAIN_URL
# for other setups (Lima, UTM, remote machine, etc.).
#
# Usage:
#   scripts/jeprof-vm.sh sync                # rsync repo to VM (incremental)
#   scripts/jeprof-vm.sh build               # build hoprd profile binary
#   scripts/jeprof-vm.sh build-localcluster  # build hoprd-localcluster
#   scripts/jeprof-vm.sh run                 # single-node hoprd vs BLOKLI_URL
#   scripts/jeprof-vm.sh localcluster N      # N-node cluster vs local anvil_blokli
#   scripts/jeprof-vm.sh all                 # sync + build + run (single node)
#
# Env overrides:
#   VM_HOST          SSH target              (default: nixos-test@orb  — OrbStack)
#   BLOKLI_URL       single-node blokli URL  (default: rotsee testnet)
#   CHAIN_URL        cluster blokli URL      (default: http://host.orb.internal:8080  — OrbStack; use host bridge IP for other hypervisors)
#   HOPRD_PASSWORD   identity password       (default: test-profiling-password)
#   PROFILE_DIR      heap dump dir on VM     (default: /tmp/jeprof)
#   CLUSTER_DIR      cluster data dir on VM  (default: /tmp/hoprd-cluster)
#
# For `localcluster`, you must have anvil_blokli running on the macOS host:
#   docker rm -f anvil_blokli; docker run --rm --name anvil_blokli \
#     --platform linux/amd64 -p 8080:8080 -d \
#     europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest

set -o errexit -o nounset -o pipefail

VM_HOST="${VM_HOST:-nixos-test@orb}" # OrbStack default; override for other setups
BLOKLI_URL="${BLOKLI_URL:-https://blokli.rotsee.hoprnet.link}"
CHAIN_URL="${CHAIN_URL:-http://host.orb.internal:8080}" # OrbStack host address; Lima: 192.168.5.2, UTM: 192.168.64.1
HOPRD_PASSWORD="${HOPRD_PASSWORD:-test-profiling-password}"
PROFILE_DIR="${PROFILE_DIR:-/tmp/jeprof}"
CLUSTER_DIR="${CLUSTER_DIR:-/tmp/hoprd-cluster}"
# lg_prof_interval=N → dump every 2^N net allocated bytes.
# 25 = 32MB (sane default for multi-node cluster, ~10s of dumps per node).
# 20 = 1MB (very aggressive, generates GBs of dumps in minutes).
LG_PROF_INTERVAL="${LG_PROF_INTERVAL:-25}"
VM_REPO="hoprd"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

vm_arch() { ssh "$VM_HOST" 'uname -m' 2>/dev/null; }

ensure_git_on_vm() {
  if ! ssh "$VM_HOST" 'command -v git' >/dev/null 2>&1; then
    echo "==> git missing on VM, installing via nix-env..."
    ssh "$VM_HOST" 'nix-env -iA nixos.git'
  fi
}

cmd_sync() {
  echo "==> Syncing repo to ${VM_HOST}:~/${VM_REPO} (rsync)..."
  rsync -az --delete \
    --exclude='target/' \
    --exclude='.direnv/' \
    --exclude='result' \
    --exclude='result-*' \
    --exclude='node_modules/' \
    --exclude='.cache/' \
    --rsync-path='rsync' \
    "$REPO_ROOT/" "${VM_HOST}:~/${VM_REPO}/"
  echo "==> Sync complete: $(ssh "$VM_HOST" "du -sh ~/${VM_REPO}")"
}

cmd_build() {
  local arch target
  arch="$(vm_arch)"
  case "$arch" in
  aarch64) target="binary-hoprd-profile-aarch64-linux" ;;
  x86_64) target="binary-hoprd-profile-x86_64-linux" ;;
  *)
    echo "Unsupported VM arch: $arch" >&2
    exit 1
    ;;
  esac
  ensure_git_on_vm
  echo "==> Building .#${target} on ${VM_HOST} (first run takes ~30+ min)..."
  ssh "$VM_HOST" "cd ~/${VM_REPO} && nix --extra-experimental-features 'nix-command flakes' build .#${target} -L -o result-hoprd"
  echo "==> Built. Binary at ~/${VM_REPO}/result-hoprd/bin/hoprd"
}

cmd_build_localcluster() {
  ensure_git_on_vm
  echo "==> Building .#binary-hoprd-localcluster on ${VM_HOST}..."
  ssh "$VM_HOST" "cd ~/${VM_REPO} && nix --extra-experimental-features 'nix-command flakes' build .#binary-hoprd-localcluster -L -o result-localcluster"
  echo "==> Built. Binary at ~/${VM_REPO}/result-localcluster/bin/hoprd-localcluster"
}

cmd_run() {
  echo "==> Starting single-node hoprd with jemalloc profiling on ${VM_HOST}..."
  echo "    blokli: ${BLOKLI_URL}"
  echo "    Heap dumps: ${PROFILE_DIR}/jeprof.*.heap"
  echo "    Stop with Ctrl-C; final dump emitted on shutdown."
  ssh -t "$VM_HOST" "
    set -e
    mkdir -p '${PROFILE_DIR}' /tmp/hoprd
    cd ~/${VM_REPO}
    bin=\$(ls -d result-hoprd 2>/dev/null || echo result)/bin/hoprd
    export HOPRD_PASSWORD='${HOPRD_PASSWORD}'
    export _RJEM_MALLOC_CONF='prof:true,prof_active:true,prof_final:true,prof_prefix:${PROFILE_DIR}/jeprof,lg_prof_sample:19,lg_prof_interval:${LG_PROF_INTERVAL}'
    exec \"\$bin\" \
      --data /tmp/hoprd \
      --identity /tmp/hoprd/identity \
      --apiHost 127.0.0.1 \
      --blokli-url '${BLOKLI_URL}'
  "
}

cmd_analyze() {
  local mode="${1:-summary}" # summary | diff | svg | top
  local pid_filter="${2:-}"
  echo "==> Analyzing dumps in ${PROFILE_DIR} on ${VM_HOST} (mode=${mode})..."
  ssh -t "$VM_HOST" "
    set -e
    cd '${PROFILE_DIR}'
    BIN=\$(ls -d ~/${VM_REPO}/result-hoprd 2>/dev/null || echo ~/${VM_REPO}/result)/bin/hoprd
    if [ ! -x \"\$BIN\" ]; then echo 'hoprd binary missing' >&2; exit 1; fi

    pids=\$(find . -maxdepth 1 -name 'jeprof.*.heap' -printf '%f\n' | sed 's/^jeprof\.//; s/\..*//' | sort -un)
    [ -n '${pid_filter}' ] && pids='${pid_filter}'

    for pid in \$pids; do
      echo '====================================================='
      echo \"PID \$pid\"
      echo '====================================================='
      first=\$(find . -maxdepth 1 -name \"jeprof.\$pid.*.heap\" -printf '%T@ %f\n' | sort -n | head -1 | awk '{print \$2}')
      last=\$(find . -maxdepth 1 -name \"jeprof.\$pid.*.heap\" -printf '%T@ %f\n' | sort -n | tail -1 | awk '{print \$2}')
      n=\$(find . -maxdepth 1 -name \"jeprof.\$pid.*.heap\" -printf 1 | wc -c)
      echo \"  \$n dumps; first=\$first  last=\$last\"

      case '${mode}' in
        summary)
          echo '--- top allocators in latest snapshot ---'
          jeprof --text --inuse_space \"\$BIN\" \"\$last\" 2>/dev/null | head -25
          ;;
        diff)
          echo '--- growth latest vs earliest (inuse_space) ---'
          jeprof --text --base=\"\$first\" --inuse_space \"\$BIN\" \"\$last\" 2>/dev/null | head -25
          ;;
        top)
          echo '--- top allocations cumulative (latest snapshot, alloc_space) ---'
          jeprof --text --alloc_space \"\$BIN\" \"\$last\" 2>/dev/null | head -25
          ;;
        svg)
          out=\"/tmp/jeprof_\${pid}_diff.svg\"
          echo \"--- generating SVG to \$out (latest vs earliest) ---\"
          jeprof --svg --base=\"\$first\" --inuse_space \"\$BIN\" \"\$last\" 2>/dev/null > \"\$out\"
          ls -lh \"\$out\"
          ;;
        pdf)
          out=\"/tmp/jeprof_\${pid}_diff.pdf\"
          echo \"--- generating PDF call graph to \$out (latest vs earliest) ---\"
          jeprof --pdf --base=\"\$first\" --inuse_space \"\$BIN\" \"\$last\" 2>/dev/null > \"\$out\"
          ls -lh \"\$out\"
          ;;
        *) echo \"Unknown mode '${mode}'. Use: summary | diff | top | svg\" >&2; exit 1 ;;
      esac
    done
  "
}

cmd_pull() {
  local out="${1:-./jeprof-out}"
  mkdir -p "$out"
  echo "==> Pulling latest+earliest dump per PID + binary to $out..."
  ssh "$VM_HOST" "
    cd ${PROFILE_DIR}
    pids=\$(find . -maxdepth 1 -name 'jeprof.*.heap' -printf '%f\n' | sed 's/^jeprof\.//; s/\..*//' | sort -un)
    for pid in \$pids; do
      first=\$(find . -maxdepth 1 -name \"jeprof.\$pid.*.heap\" -printf '%T@ %f\n' | sort -n | head -1 | awk '{print \$2}')
      last=\$(find . -maxdepth 1 -name \"jeprof.\$pid.*.heap\" -printf '%T@ %f\n' | sort -n | tail -1 | awk '{print \$2}')
      echo \"\$pid: \$first \$last\"
    done
  " | while read -r pid first last; do
    [ -z "$pid" ] && continue
    scp "${VM_HOST}:${PROFILE_DIR}/${first}" "$out/" 2>/dev/null || true
    scp "${VM_HOST}:${PROFILE_DIR}/${last}" "$out/" 2>/dev/null || true
  done
  scp "${VM_HOST}:${VM_REPO}/result/bin/hoprd" "$out/hoprd" 2>/dev/null ||
    scp "${VM_HOST}:${VM_REPO}/result-hoprd/bin/hoprd" "$out/hoprd"
  ls -lh "$out"/
  echo "Run on macOS (after 'brew install jemalloc graphviz'):"
  echo "  jeprof --text --base=$out/<earliest>.heap $out/hoprd $out/<latest>.heap"
}

cmd_localcluster() {
  local size="${1:-3}"
  echo "==> Starting ${size}-node localcluster with jemalloc profiling on ${VM_HOST}..."
  echo "    chain:        ${CHAIN_URL} (must be reachable from VM)"
  echo "    cluster data: ${CLUSTER_DIR}"
  echo "    heap dumps:   ${PROFILE_DIR}/jeprof.<PID>.*.heap (one PID per child node)"

  if ! ssh "$VM_HOST" "curl -sS -o /dev/null -m 3 -w '%{http_code}' ${CHAIN_URL}/graphql 2>/dev/null | grep -q '^200$'"; then
    echo "WARN: ${CHAIN_URL} not reachable from VM. Start it on macOS first:"
    echo '  docker rm -f anvil_blokli; docker run --rm --name anvil_blokli \'
    echo '    --platform linux/amd64 -p 8080:8080 -d \'
    echo "    europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest"
    echo "Continuing anyway (localcluster will retry)..."
  fi

  ssh -t "$VM_HOST" "
    set -e
    mkdir -p '${PROFILE_DIR}' '${CLUSTER_DIR}'
    cd ~/${VM_REPO}
    hoprd_bin=\$(pwd)/\$(ls -d result-hoprd 2>/dev/null || echo result)/bin/hoprd
    lc_bin=\$(pwd)/result-localcluster/bin/hoprd-localcluster
    if [ ! -x \"\$hoprd_bin\" ]; then echo 'missing hoprd binary; run: scripts/jeprof-vm.sh build' >&2; exit 1; fi
    if [ ! -x \"\$lc_bin\"   ]; then echo 'missing hoprd-localcluster binary; run: scripts/jeprof-vm.sh build-localcluster' >&2; exit 1; fi
    # hoprd-localcluster is dynamically linked (glibc) and needs openssl at runtime.
    # The hoprd binary itself is statically linked (musl) — env is harmless.
    ssl_lib=\$(echo /nix/store/*-openssl-3*/lib | tr ' ' '\n' | head -1)
    if [ ! -d \"\$ssl_lib\" ]; then
      echo 'openssl not in nix store; install: nix-env -iA nixos.openssl' >&2; exit 1
    fi
    export LD_LIBRARY_PATH=\"\$ssl_lib\${LD_LIBRARY_PATH:+:\$LD_LIBRARY_PATH}\"
    export HOPRD_PASSWORD='${HOPRD_PASSWORD}'
    export _RJEM_MALLOC_CONF='prof:true,prof_active:true,prof_final:true,prof_prefix:${PROFILE_DIR}/jeprof,lg_prof_sample:19,lg_prof_interval:${LG_PROF_INTERVAL}'
    exec \"\$lc_bin\" \
      --chain-url '${CHAIN_URL}' \
      --hoprd-bin \"\$hoprd_bin\" \
      --size '${size}' \
      --data-dir '${CLUSTER_DIR}' \
      --identity-password '${HOPRD_PASSWORD}' \
      --skip-channels
  "
}

cmd_clean() {
  echo "==> Cleaning up transient data and dumps on ${VM_HOST}..."
  ssh "$VM_HOST" "rm -rf /tmp/hoprd /tmp/hoprd-cluster '${PROFILE_DIR}'"
  echo "==> Done."
}

case "${1:-all}" in
sync) cmd_sync ;;
build) cmd_build ;;
build-localcluster) cmd_build_localcluster ;;
run) cmd_run ;;
localcluster)
  shift
  cmd_localcluster "${1:-3}"
  ;;
analyze)
  shift
  cmd_analyze "${1:-summary}" "${2:-}"
  ;;
pull)
  shift
  cmd_pull "${1:-./jeprof-out}"
  ;;
clean) cmd_clean ;;
all)
  cmd_sync
  cmd_build
  cmd_run
  ;;
*)
  echo "Usage: $0 [sync|build|build-localcluster|run|localcluster N|analyze MODE [PID]|pull DIR|clean|all]" >&2
  exit 1
  ;;
esac
