#!/usr/bin/bash
#
# simple script to check docs using `cargo-docs-rs`
#

set -eux -o pipefail

BUILD=${1:-all}

cd $(dirname ${BASH_SOURCE[0]})/../../

SDK=$(cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')
SERVICES=$(cd services; cargo metadata --format-version=1 --no-deps | jq -r -c '.packages | .[] | select(.publish == null) | .name')


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

for i in ${SERVICES}; do
    case ${BUILD} in
       all | svc )
           if [[ ${i} =~ "azure_svc_" ]]; then
               cargo +nightly docs-rs -p ${i} --manifest-path services/Cargo.toml
           fi
           ;;
       all | mgmt )
           if [[ ${i} =~ "azure_mgmt_" ]]; then
               cargo +nightly docs-rs -p ${i} --manifest-path services/Cargo.toml
           fi
           ;;
       sdk )
           ;;
       *)
           echo "unsupported build (${BUILD}) use all, sdk, svc, or mgmt"
           exit 1
           ;;
    esac
done
