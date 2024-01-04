#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

./eng/scripts/github-disk-cleanup.sh

BUILD_ARGS=""

for i in "$@"; do
   BUILD_ARGS="${BUILD_ARGS} -- --package ${i}"
done

cd services/autorust; 
cargo run --release ${BUILD_ARGS}
../../eng/scripts/services_tests.sh
