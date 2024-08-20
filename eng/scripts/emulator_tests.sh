#!/usr/bin/env bash

set -eux -o pipefail
BUILD=${1:-stable}

cd $(dirname ${BASH_SOURCE[0]})/../../
./eng/scripts/github-disk-cleanup.sh

# get the nested list of sub-processes for a given set of pids
function subprocesses {
    for P in $@; do
        echo ${P}
        for C in $(pgrep -P ${P}); do
            subprocesses ${C}
        done
    done
}

# stop all of the subprocesses for a given set of pids
function stop_subprocesses {
    # sort pids numerically, and in reverse
    PIDS=$(subprocesses $$ | sort -nr)
    for P in ${PIDS}; do
        if [ ${P} == $$ ]; then
            continue
        fi
        kill -9 ${P} || echo "stopping ${P} failed"
    done
}

# at termination, we want to cleanup the temp directory and stop all
# subprocesses (azurite)
TMP=$(mktemp -d)
function cleanup {
    stop_subprocesses || true
    rm -rf ${TMP} || true
}
trap cleanup EXIT

BASE_DIR=$(pwd)
cd ${TMP}
npm install azurite@3.28.0
npx azurite &

# wait for azurite to start
sleep 5

cd ${BASE_DIR}
rustup update --no-self-update ${BUILD}
cargo +${BUILD} test --features test_integration
