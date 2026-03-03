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
# Example testing against a specific SDK commit:
#   ./run_perf.sh --cosmos-commit abc123 --processes 4 \
#       --endpoint "https://myaccount.documents.azure.com" --auth key --key <key>
#
# Example polling a branch for new commits (continuous deployment):
#   ./run_perf.sh --poll-branch release/azure_data_cosmos-previews --processes 4 \
#       --endpoint "https://myaccount.documents.azure.com" --auth aad
#
# The --processes flag (default: 1) controls how many OS processes are spawned.
# The --cosmos-commit flag builds against a specific azure_data_cosmos commit/branch/tag.
# The --poll-branch flag continuously polls a remote branch for new commits and restarts.
# All other arguments are forwarded directly to each perf binary instance.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

PROCESSES=1
COSMOS_COMMIT=""
POLL_BRANCH=""
POLL_INTERVAL=43200
STAGGER_MS=200
PERF_ARGS=()

# Parse script-level args; collect everything else for the perf binary.
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
        --cosmos-commit)
            COSMOS_COMMIT="$2"
            shift 2
            ;;
        --cosmos-commit=*)
            COSMOS_COMMIT="${1#*=}"
            shift
            ;;
        --poll-branch)
            POLL_BRANCH="$2"
            shift 2
            ;;
        --poll-branch=*)
            POLL_BRANCH="${1#*=}"
            shift
            ;;
        --poll-interval)
            POLL_INTERVAL="$2"
            shift 2
            ;;
        --poll-interval=*)
            POLL_INTERVAL="${1#*=}"
            shift
            ;;
        --stagger-ms)
            STAGGER_MS="$2"
            shift 2
            ;;
        --stagger-ms=*)
            STAGGER_MS="${1#*=}"
            shift
            ;;
        --commit-sha|--commit-sha=*)
            echo "Warning: --commit-sha is managed by the launcher and will be overridden." >&2
            if [[ "$1" == "--commit-sha" ]]; then shift 2; else shift; fi
            ;;
        --help|-h)
            echo "Usage: $0 [--processes N] [--cosmos-commit REF] [--poll-branch BRANCH] [perf-tool-args...]"
            echo ""
            echo "Launches N parallel instances of azure_data_cosmos_perf."
            echo ""
            echo "Options:"
            echo "  --processes N        Number of OS processes to spawn (default: 1)"
            echo "  --cosmos-commit REF  Build against a specific azure_data_cosmos commit,"
            echo "                       branch, or tag. The SDK source is checked out at the"
            echo "                       given ref, the perf binary is built, and the original"
            echo "                       source is restored on exit."
            echo "  --poll-branch BRANCH Continuously poll a remote branch for new commits."
            echo "                       When a new commit is detected, running processes are"
            echo "                       stopped, the branch is checked out, the binary is"
            echo "                       rebuilt, and processes are restarted."
            echo "  --poll-interval SECS Seconds between branch polls (default: 43200 / 12 hours)"
            echo "  --stagger-ms MS     Milliseconds to wait between launching each process"
            echo "                       (default: 200, set to 0 for simultaneous start)"
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

if [[ -n "$COSMOS_COMMIT" && -n "$POLL_BRANCH" ]]; then
    echo "Error: --cosmos-commit and --poll-branch are mutually exclusive." >&2
    exit 1
fi

SDK_PATH="sdk/cosmos/azure_data_cosmos"
SDK_CHECKED_OUT=""
PIDS=()

# Restore the SDK source to the current branch state.
restore_sdk() {
    if [[ -n "$SDK_CHECKED_OUT" ]]; then
        echo "Restoring $SDK_PATH to working tree state..."
        git -C "$REPO_ROOT" checkout HEAD -- "$SDK_PATH" 2>/dev/null || true
        SDK_CHECKED_OUT=""
    fi
}

# Stop all running perf processes.
stop_processes() {
    if [[ ${#PIDS[@]} -eq 0 ]]; then
        return
    fi
    echo "Stopping all perf processes..."
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill -SIGINT "$pid" 2>/dev/null || true
        fi
    done
    sleep 2
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill -SIGKILL "$pid" 2>/dev/null || true
        fi
    done
    wait 2>/dev/null
    PIDS=()
}

cleanup() {
    echo ""
    stop_processes
    restore_sdk
    echo "All processes stopped."
}

trap cleanup SIGINT SIGTERM

BINARY="$REPO_ROOT/target/release/azure_data_cosmos_perf"

# Checkout a specific ref, build, and restore the working tree.
build_at_ref() {
    local ref="$1"
    local resolved
    resolved=$(git -C "$REPO_ROOT" rev-parse --short "$ref")
    echo "Checking out $SDK_PATH at $ref ($resolved)..."
    git -C "$REPO_ROOT" checkout "$ref" -- "$SDK_PATH"
    SDK_CHECKED_OUT="1"

    echo "Building azure_data_cosmos_perf (release)..."
    cargo build --release -p azure_data_cosmos_perf --manifest-path "$REPO_ROOT/Cargo.toml"

    restore_sdk
    echo "$resolved"
}

# Build using the current working tree.
build_current() {
    echo "Building azure_data_cosmos_perf (release)..."
    cargo build --release -p azure_data_cosmos_perf --manifest-path "$REPO_ROOT/Cargo.toml"
    git -C "$REPO_ROOT" rev-parse --short HEAD
}

# Launch perf processes with the given commit SHA.
launch_processes() {
    local sha="$1"
    echo "Launching $PROCESSES perf process(es) (commit: $sha)..."
    for i in $(seq 1 "$PROCESSES"); do
        "$BINARY" "${PERF_ARGS[@]}" --commit-sha "$sha" &
        PIDS+=($!)
        last_index=$(( ${#PIDS[@]} - 1 ))
        echo "  Process $i: PID ${PIDS[$last_index]}"
        if [[ "$STAGGER_MS" -gt 0 && "$i" -lt "$PROCESSES" ]]; then
            sleep "$(awk "BEGIN{printf \"%.3f\", $STAGGER_MS/1000}")"
        fi
    done
    echo ""
    echo "All $PROCESSES process(es) running."
    echo ""
}

# --- Main execution ---

if [[ -n "$POLL_BRANCH" ]]; then
    # Continuous polling mode: fetch, build, run, poll for changes, repeat.
    CURRENT_SHA=""

    while true; do
        echo "Fetching latest from remote..."
        git -C "$REPO_ROOT" fetch --quiet 2>/dev/null || true

        # Resolve the remote branch ref (try upstream/<branch> first, then origin/<branch>)
        REMOTE_SHA=""
        for remote in upstream origin; do
            if git -C "$REPO_ROOT" rev-parse --verify "$remote/$POLL_BRANCH" >/dev/null 2>&1; then
                REMOTE_SHA=$(git -C "$REPO_ROOT" rev-parse "$remote/$POLL_BRANCH")
                REMOTE_REF="$remote/$POLL_BRANCH"
                break
            fi
        done

        if [[ -z "$REMOTE_SHA" ]]; then
            echo "Error: branch '$POLL_BRANCH' not found on any remote." >&2
            exit 1
        fi

        if [[ "$REMOTE_SHA" != "$CURRENT_SHA" ]]; then
            SHORT_SHA=$(git -C "$REPO_ROOT" rev-parse --short "$REMOTE_SHA")
            if [[ -n "$CURRENT_SHA" ]]; then
                OLD_SHORT=$(git -C "$REPO_ROOT" rev-parse --short "$CURRENT_SHA")
                echo "New commit detected: $OLD_SHORT -> $SHORT_SHA"
                stop_processes
            fi

            RESOLVED=$(build_at_ref "$REMOTE_REF")
            CURRENT_SHA="$REMOTE_SHA"

            if [[ ! -x "$BINARY" ]]; then
                echo "Error: binary not found at $BINARY" >&2
                exit 1
            fi

            launch_processes "$RESOLVED"
        fi

        sleep "$POLL_INTERVAL"
    done

elif [[ -n "$COSMOS_COMMIT" ]]; then
    # One-shot mode with specific commit.
    if ! git -C "$REPO_ROOT" rev-parse --verify "$COSMOS_COMMIT" >/dev/null 2>&1; then
        echo "Error: '$COSMOS_COMMIT' is not a valid commit, branch, or tag." >&2
        exit 1
    fi

    RESOLVED=$(build_at_ref "$COSMOS_COMMIT")

    if [[ ! -x "$BINARY" ]]; then
        echo "Error: binary not found at $BINARY" >&2
        exit 1
    fi

    launch_processes "$RESOLVED"

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

else
    # Default: build and run with current working tree.
    CURRENT_SHA=$(build_current)

    if [[ ! -x "$BINARY" ]]; then
        echo "Error: binary not found at $BINARY" >&2
        exit 1
    fi

    launch_processes "$CURRENT_SHA"

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
fi
