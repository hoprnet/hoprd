#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 3 ]; then
  echo "Usage: $0 <binary_path> <dump_file> <output_prefix>"
  exit 1
fi

BINARY="$1"
DUMP_FILE="$2"
OUTPUT_PREFIX="$3"

if [ ! -f "$BINARY" ]; then
  echo "Error: binary not found: $BINARY"
  exit 1
fi

if [ ! -f "$DUMP_FILE" ]; then
  echo "Error: dump file not found: $DUMP_FILE"
  exit 1
fi

OUTPUT_DIR="$(dirname "$OUTPUT_PREFIX")"
mkdir -p "$OUTPUT_DIR"

# Create fontconfig cache dir to suppress warnings
mkdir -p /var/cache/fontconfig /.cache/fontconfig 2>/dev/null || true

echo "Generating comprehensive analysis for $DUMP_FILE..."

# Helper: run jeprof, tolerating "No nodes to print" for filtered views
run_jeprof() {
  local output_file="$1"
  shift
  local stderr_tmp
  stderr_tmp=$(mktemp)
  if ! jeprof "$@" >"$output_file" 2>"$stderr_tmp"; then
    if grep -q "No nodes to print" "$stderr_tmp"; then
      echo "    (skipped: no matching nodes)"
      rm -f "$output_file"
    else
      echo "Error running jeprof for $(basename "$output_file"):" >&2
      cat "$stderr_tmp" >&2
      rm -f "$output_file" "$stderr_tmp"
      return 1
    fi
  elif grep -q "No nodes to print" "$output_file"; then
    echo "    (skipped: no matching nodes)"
    rm -f "$output_file"
  fi
  rm -f "$stderr_tmp"
}

# 1. Overall memory usage
run_jeprof "${OUTPUT_PREFIX}_overview.svg" --show_bytes --svg "$BINARY" "$DUMP_FILE"

# 2. Top memory consumers
run_jeprof "${OUTPUT_PREFIX}_top20.svg" --show_bytes --nodecount=20 --svg "$BINARY" "$DUMP_FILE"

# 3. Filtered view (significant allocations only)
run_jeprof "${OUTPUT_PREFIX}_significant.svg" --show_bytes --nodefraction=0.01 --edgefraction=0.01 --svg "$BINARY" "$DUMP_FILE"

# 4. Call graph with line numbers
run_jeprof "${OUTPUT_PREFIX}_detailed.svg" --show_bytes --lines --svg "$BINARY" "$DUMP_FILE"

# 5. Object count analysis
run_jeprof "${OUTPUT_PREFIX}_objects.svg" --alloc_objects --svg "$BINARY" "$DUMP_FILE"

# Focus on Rust-specific allocations
run_jeprof "${OUTPUT_PREFIX}_rust_specific.svg" --show_bytes --focus="rust_|std::|tokio::|serde::" --svg "$BINARY" "$DUMP_FILE"

# Ignore Rust runtime allocations to see application logic
run_jeprof "${OUTPUT_PREFIX}_rust_logic.svg" --show_bytes --ignore="rust_begin_unwind|__rust_|std::panic" --svg "$BINARY" "$DUMP_FILE"

# Show only large allocations (useful for finding memory hogs)
run_jeprof "${OUTPUT_PREFIX}_large_allocs.svg" --show_bytes --nodefraction=0.05 --edgefraction=0.01 --svg "$BINARY" "$DUMP_FILE"

echo "Analysis complete:"
for svg in "${OUTPUT_PREFIX}"_*.svg; do
  [ -f "$svg" ] && echo "  - $(basename "$svg")"
done
