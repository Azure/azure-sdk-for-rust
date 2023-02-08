#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

rustup update --no-self-update ${BUILD}

export RUSTFLAGS="-Dwarnings"

cargo +${BUILD} fmt --all --manifest-path services/autorust/Cargo.toml -- --check
cargo +${BUILD} clippy --all --manifest-path services/autorust/Cargo.toml
cargo +${BUILD} test --lib --manifest-path services/autorust/Cargo.toml

cd services/autorust
cargo +${BUILD} test --package autorust_openapi --test openapi_spec_examples
cargo +${BUILD} test --package autorust_openapi --test azure_rest_api_specs
cargo +${BUILD} test --package autorust_codegen --test azure_rest_api_specs
