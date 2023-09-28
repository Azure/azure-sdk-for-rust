#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

./eng/scripts/github-disk-cleanup.sh

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}
rustup target add --toolchain ${BUILD} wasm32-unknown-unknown

export RUSTFLAGS="-Dwarnings"
cargo +${BUILD} check --target=wasm32-unknown-unknown --no-default-features
