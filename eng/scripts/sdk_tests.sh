#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}
rustup target add --toolchain ${BUILD} wasm32-unknown-unknown

export RUSTFLAGS="-Dwarnings"
cargo +${BUILD} check -p azure_core --no-default-features
cargo +${BUILD} check --target=wasm32-unknown-unknown --no-default-features
cargo +${BUILD} check --all --features azurite_workaround
cargo +${BUILD} test --all
