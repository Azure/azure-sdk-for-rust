#!/bin/bash

set -eu -o pipefail

DIR=$(dirname ${BASH_SOURCE[0]})/../../services
PACKAGES=$(cargo metadata --format-version 1 --manifest-path "$DIR/Cargo.toml" | jq -r '.packages[] | select(.name | startswith("azure")) | "\(.name):\(.version)\n"')

for p in $PACKAGES
do
  IFS=':' read -a PACKAGE <<< $p
  echo -n "Checking ${PACKAGE[0]} ... "
  PUBLISHED=$(curl --no-progress-meter "https://crates.io/api/v1/crates/${PACKAGE[0]}/${PACKAGE[1]}" | jq -r '.version.num')
  if [[ "${PUBLISHED}" == "${PACKAGE[1]}" ]]; then
    echo -e "\x1b[0;32m${PACKAGE[1]} already published\x1b[0m"
  else
    echo -en "\x1b[0;33mpublishing ${PACKAGE[1]}\x1b[0m ... "
    cargo publish --manifest-path "$DIR/Cargo.toml" --package "${PACKAGE[0]}" -q
    rm -rf -- "$DIR/target/package/${PACKAGE[0]}-${PACKAGE[1]}"
    rm -f -- "$DIR/target/package/${PACKAGE[0]}-${PACKAGE[1]}.crate"
    echo -e "\x1b[0;32mdone\x1b[0m"
  fi
done
