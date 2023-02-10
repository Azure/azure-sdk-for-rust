#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}
export RUSTFLAGS="-Dwarnings -Aunreachable-code -Aunused-assignments -Adead-code -Aclippy::new-without-default -Aclippy::unnecessary_to_owned"
cargo +${BUILD} check --manifest-path services/Cargo.toml --all
cargo +${BUILD} check --manifest-path services/Cargo.toml --examples
cargo +${BUILD} clippy --manifest-path services/Cargo.toml --all
cargo +${BUILD} fmt --manifest-path services/Cargo.toml --all -- --check
