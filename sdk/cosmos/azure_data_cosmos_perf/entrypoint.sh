#!/bin/bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Entrypoint wrapper that optionally runs the benchmark under Valgrind.
#
# When VALGRIND_TOOL is set (dhat, massif, memcheck), the binary is
# executed under Valgrind with the appropriate tool and output options.
# When unset, the binary runs directly with zero overhead.

set -euo pipefail

if [ -n "${VALGRIND_TOOL:-}" ]; then
    echo "Valgrind enabled: running under --tool=$VALGRIND_TOOL"
    VALGRIND_OPTS="--tool=$VALGRIND_TOOL"
    case "$VALGRIND_TOOL" in
        dhat)
            VALGRIND_OPTS="$VALGRIND_OPTS --dhat-out-file=/perf-data/dhat-out.json"
            ;;
        massif)
            VALGRIND_OPTS="$VALGRIND_OPTS --massif-out-file=/perf-data/massif.out --detailed-freq=1"
            ;;
        memcheck)
            VALGRIND_OPTS="$VALGRIND_OPTS --leak-check=full --show-leak-kinds=all --track-origins=yes"
            ;;
        *)
            echo "Warning: unknown VALGRIND_TOOL '$VALGRIND_TOOL', running without valgrind"
            exec azure_data_cosmos_perf "$@"
            ;;
    esac
    exec valgrind $VALGRIND_OPTS azure_data_cosmos_perf "$@"
else
    exec azure_data_cosmos_perf "$@"
fi
