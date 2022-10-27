#!/usr/bin/bash
#
# simple script to check that github team for publishing crates has been added
# for all of the crates.
# 

set -e

cd $(dirname ${BASH_SOURCE[0]})/../../

OWNER=github:azure:azure-sdk-publish-rust

for i in sdk/*/Cargo.toml ; do
  (
    CRATE_DIR=$(dirname $i)
    cd ${CRATE_DIR}
    if ! grep -qi 'publish = false' Cargo.toml; then 
      if ! cargo owner -q --list 2>/dev/null | grep -qi ${OWNER}; then 
        echo SDK publish team is missing on crates.io for $i
        echo "(cd ${CRATE_DIR}; cargo owner --add ${OWNER})"
      fi
    fi
  )
done

for i in services/svc/*/Cargo.toml services/mgmt/*/Cargo.toml; do 
  (
    CRATE_DIR=$(dirname $i)
    cd ${CRATE_DIR}
    if ! cargo owner -q --list 2>/dev/null | grep -qi ${OWNER}; then 
      echo SDK publish team is missing on crates.io for $i
      echo "(cd ${CRATE_DIR}; cargo owner --add ${OWNER})"
    fi
  )
done
