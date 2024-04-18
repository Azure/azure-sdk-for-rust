#!/usr/bin/bash
#
# simple script to check docs using `cargo-docs-rs`
#

set -eux -o pipefail

BUILD=${1:-all}

cd $(dirname ${BASH_SOURCE[0]})/../../

SDK=$(cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')


for i in ${SDK}; do
    case ${BUILD} in
       all | sdk )
           cargo +nightly docs-rs -p ${i}
           ;;
       svc | mgmt )
           ;;
       *)
           echo "unsupported build. (${BUILD}) use all, sdk, svc, or mgmt"
           exit 1
           ;;
    esac
done
