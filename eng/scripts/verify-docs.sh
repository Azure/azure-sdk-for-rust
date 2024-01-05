#!/usr/bin/bash
#
# simple script to check docs using `cargo-docs-rs` 
#

set -eux -o pipefail

cd $(dirname ${BASH_SOURCE[0]})/../../

SDK=$(cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')
SERVICES=$(cd services; cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')

for i in ${SDK}; do
   cargo +nightly docs-rs -p ${i}
done

for i in ${SERVICES}; do
   cargo +nightly docs-rs -p ${i} --manifest-path services/Cargo.toml
done

