#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

./eng/scripts/github-disk-cleanup.sh

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}

export RUSTFLAGS="-Dwarnings"
export PROJECTS=core/azure_core identity/azure_identity

for PROJ in ${PROJECTS}; do
    cargo check --tests --features test_e2e --manifest-path sdk/$PROJ/Cargo.toml
done
