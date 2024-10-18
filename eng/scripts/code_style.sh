#!/usr/bin/env bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

export RUSTDOCFLAGS="-D warnings"
export RUSTFLAGS="-Dwarnings"

rustup update --no-self-update ${BUILD}
rustup component add rustfmt --toolchain ${BUILD}
cargo +${BUILD} fmt --all -- --check
cargo +${BUILD} clippy --all
cargo +${BUILD} doc --all --no-deps
