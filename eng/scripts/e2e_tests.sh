#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

./eng/scripts/github-disk-cleanup.sh

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}

export RUSTFLAGS="-Dwarnings"
export PROJECTS=core data_cosmos identity messaging_servicebus storage storage_blobs storage_queues storage_datalake data_tables

for PROJ in ${PROJECTS}; do
    cargo check --tests --features test_e2e --manifest-path sdk/$PROJ/Cargo.toml
done
