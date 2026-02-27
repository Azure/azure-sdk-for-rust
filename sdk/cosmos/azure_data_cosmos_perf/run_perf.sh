#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Launches multiple instances of the azure_data_cosmos_perf binary in parallel.
#
# Usage:
#   ./run_perf.sh --processes 4 --endpoint https://... --auth key --key <key> [other perf args...]
#
# Example with excluded regions for writes only:
#   ./run_perf.sh --processes 40 \
#       --endpoint "https://myaccount.documents.azure.com" \
#       --auth aad \
#       --excluded-regions "Central US EUAP" \
#       --exclude-regions-for writes
#
# The --processes flag (default: 1) controls how many OS processes are spawned.
# All other arguments are forwarded directly to each perf binary instance.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

PROCESSES=1
PERF_ARGS=()

# Parse --processes from args; collect everything else for the perf binary.
while [[ $# -gt 0 ]]; do
    case "$1" in
        --processes)
            PROCESSES="$2"
            shift 2
            ;;
        --processes=*)
            PROCESSES="${1#*=}"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 --processes N [perf-tool-args...]"
            echo ""
            echo "Launches N parallel instances of azure_data_cosmos_perf."
            echo ""
            echo "Options:"
            echo "  --processes N   Number of OS processes to spawn (default: 1)"
            echo ""
            echo "All other arguments are forwarded to each perf binary instance."
            echo ""
            echo "Common perf tool arguments:"
            echo "  --endpoint URL                  Cosmos DB account endpoint"
            echo "  --auth key|aad                  Authentication method"
            echo "  --key KEY                       Account key (when --auth=key)"
            echo "  --preferred-regions R1,R2       Comma-separated preferred regions"
            echo "  --excluded-regions R1,R2        Comma-separated regions to exclude"
            echo "  --exclude-regions-for TYPE      reads, writes, or both (default: both)"
            echo "  --results-endpoint URL          Separate endpoint for results storage"
            echo "  --results-auth key|aad          Auth method for results account"
            echo ""
            echo "Run 'cargo run -p azure_data_cosmos_perf -- --help' for full options."
            exit 0
            ;;
        *)
            PERF_ARGS+=("$1")
            shift
            ;;
    esac
done

if ! [[ "$PROCESSES" =~ ^[1-9][0-9]*$ ]]; then
    echo "Error: --processes must be a positive integer, got '$PROCESSES'" >&2
    exit 1
fi

BINARY="$REPO_ROOT/target/release/azure_data_cosmos_perf"

echo "Building azure_data_cosmos_perf (release)..."
cargo build --release -p azure_data_cosmos_perf --manifest-path "$REPO_ROOT/Cargo.toml"

if [[ ! -x "$BINARY" ]]; then
    echo "Error: binary not found at $BINARY" >&2
    exit 1
fi

PIDS=()

cleanup() {
    echo ""
    echo "Stopping all perf processes..."
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill -SIGINT "$pid" 2>/dev/null || true
        fi
    done
    # Give processes a moment to shut down gracefully, then force-kill stragglers.
    sleep 2
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill -SIGKILL "$pid" 2>/dev/null || true
        fi
    done
    wait 2>/dev/null
    echo "All processes stopped."
}

trap cleanup SIGINT SIGTERM

echo "Launching $PROCESSES perf process(es)..."
for i in $(seq 1 "$PROCESSES"); do
    "$BINARY" "${PERF_ARGS[@]}" &
    PIDS+=($!)
    last_index=$(( ${#PIDS[@]} - 1 ))
    echo "  Process $i: PID ${PIDS[$last_index]}"
done

echo ""
echo "All $PROCESSES process(es) running. Press Ctrl+C to stop."
echo ""

# Wait for all children; track failures.
FAILURES=0
for pid in "${PIDS[@]}"; do
    if ! wait "$pid"; then
        FAILURES=$((FAILURES + 1))
    fi
done

if [[ "$FAILURES" -gt 0 ]]; then
    echo "$FAILURES of $PROCESSES process(es) exited with errors."
    exit 1
else
    echo "All $PROCESSES process(es) completed successfully."
fi
