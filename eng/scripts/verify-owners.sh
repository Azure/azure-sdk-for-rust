#!/usr/bin/bash
#
# simple script to check that github team for publishing crates has been added
# for all of the crates.
#

set -e

cd $(dirname ${BASH_SOURCE[0]})/../../

OWNER=github:azure:azure-sdk-publish-rust
SDK=$(cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')
SERVICES=$(cd services; cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')

for i in ${SDK} ${SERVICES}; do
   if ! cargo owner -q --list $i 2>/dev/null | grep -qi ${OWNER}; then
       echo SDK publish team is missing on crates.io for $i
       echo "cargo owner --add ${OWNER} $i"
   fi
done
