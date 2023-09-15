#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

npm install azurite@3.26.0
npx azurite &

rustup update --no-self-update ${BUILD}
cargo +${BUILD} test --features test_integration
