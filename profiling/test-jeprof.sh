#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "==> Building hoprd-profile-docker image..."
nix build .#docker-hoprd-profile-x86_64-linux -L 2>&1 | tail -20

CONTAINER_NAME="hoprd-jeprof-test-$$"
ANALYSIS_CONTAINER="hoprd-jeprof-analyze-$$"
OUTPUT_DIR="/tmp/hoprd-jeprof-profiles-$$"
PROFILE_DIR="/app/.tmp"
BINARY_PATH="/bin/hoprd"

mkdir -p "$OUTPUT_DIR"
trap "docker rm -f $CONTAINER_NAME $ANALYSIS_CONTAINER 2>/dev/null || true" EXIT

echo "==> Loading Docker image..."
docker load <result >/dev/null

echo "==> Starting hoprd container with jemalloc profiling..."
# lg_prof_interval:20 triggers a heap dump every ~1MB of allocations
docker run -d --name "$CONTAINER_NAME" \
  -e "_RJEM_MALLOC_CONF=prof:true,prof_active:true,prof_final:true,prof_prefix:${PROFILE_DIR}/jeprof,lg_prof_sample:19,lg_prof_interval:20" \
  -e "HOPRD_PASSWORD=test-profiling-password" \
  hoprd:latest \
  bash -c "mkdir -p /tmp/hoprd ${PROFILE_DIR} && exec hoprd --data /tmp/hoprd --identity /tmp/hoprd/identity --apiHost 0.0.0.0 --blokli-url https://blokli.rotsee.hoprnet.link" >/dev/null

echo "==> Container started (PID check)..."
sleep 3

# Check if container is still running
if ! docker ps --filter "name=$CONTAINER_NAME" --format "{{.Names}}" | grep -q "$CONTAINER_NAME"; then
  echo "Error: Container failed to start. Checking logs:"
  docker logs "$CONTAINER_NAME"
  exit 1
fi

echo "==> Running hoprd for 10 seconds to generate profiling data..."
sleep 10

echo "==> Stopping container..."
docker stop "$CONTAINER_NAME" >/dev/null

echo "==> Extracting profiling data from container..."
docker cp "$CONTAINER_NAME:${PROFILE_DIR}" "$OUTPUT_DIR/profiles" 2>/dev/null || {
  echo "Warning: Could not extract profiles. Container may not have generated data."
  echo "Checking if profiles were created..."
  docker export "$CONTAINER_NAME" | tar -tf - 2>/dev/null | grep -F "jeprof" || true
  exit 1
}

echo ""
echo "==> Testing jeprof command inside a new container..."
docker run --rm hoprd:latest jeprof --help >/dev/null 2>&1 && {
  echo "✓ jeprof is executable and working"
} || {
  echo "✗ jeprof failed to execute"
  exit 1
}

echo ""
echo "==> Analyzing heap profiles inside Docker..."
mapfile -d '' heap_files < <(find "$OUTPUT_DIR/profiles" -name "*.heap" -print0 2>/dev/null)

if [ ${#heap_files[@]} -eq 0 ]; then
  echo "Warning: No .heap files found. Listing extracted contents:"
  find "$OUTPUT_DIR/profiles" -type f 2>/dev/null || echo "  (empty)"
  echo ""
  echo "==> Verification complete (no profiles to analyze)."
  echo "Summary:"
  echo "  - Docker image loaded successfully"
  echo "  - Container started with profiling enabled"
  echo "  - jeprof command is functional"
  echo "  - No heap dumps were generated (process may not have allocated enough)"
  exit 0
fi

ANALYSIS_OUTPUT="$OUTPUT_DIR/analysis"
mkdir -p "$ANALYSIS_OUTPUT"

# Run analyze_memory.sh inside a container with the heap files mounted
for HEAP_FILE in "${heap_files[@]}"; do
  BASENAME="$(basename "$HEAP_FILE" .heap)"
  echo "  Analyzing: $BASENAME"

  docker run --rm --name "$ANALYSIS_CONTAINER" \
    -v "$OUTPUT_DIR/profiles:/profiles:ro" \
    -v "$SCRIPT_DIR/analyze_memory.sh:/analyze_memory.sh:ro" \
    -v "$ANALYSIS_OUTPUT:/output" \
    hoprd:latest \
    bash -c "mkdir -p /tmp && bash /analyze_memory.sh '$BINARY_PATH' '/profiles/$(basename "$HEAP_FILE")' '/output/$BASENAME'"
done

echo ""
echo "==> Verification complete!"
echo ""
echo "Summary:"
echo "  - Docker image loaded successfully"
echo "  - Container started with profiling enabled"
echo "  - jeprof command is functional"
echo "  - Heap profiles analyzed successfully"
echo ""
echo "Analysis outputs:"
find "$ANALYSIS_OUTPUT" -name "*.svg" -printf "  - %f\n" 2>/dev/null || true
echo ""
echo "Results directory: $ANALYSIS_OUTPUT"
