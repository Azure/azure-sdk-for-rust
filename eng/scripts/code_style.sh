#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

export RUSTDOCFLAGS="-D warnings"
export RUSTFLAGS="-Dwarnings"

rustup update --no-self-update ${BUILD}
rustup component add rustfmt --toolchain ${BUILD}
cargo +${BUILD} install cargo-readme
cargo +${BUILD} fmt --all -- --check
cargo +${BUILD} clippy --all
cargo +${BUILD} doc --all --no-deps --all-features
./eng/scripts/check_json_format.sh
./eng/scripts/cargo_readme.sh
if git status sdk | grep -q '.md$'; then
    echo "Run ./eng/scripts/cargo_readme.sh to update readmes" && exit 1
fi
