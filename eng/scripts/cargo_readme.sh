#!/usr/bin/env bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

for crate in sdk/*;  do
  if [ -d "$crate" ]; then
    (cd "$crate"; cargo readme > README.md)
  fi
done