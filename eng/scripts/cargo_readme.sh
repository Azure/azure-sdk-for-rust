#!/usr/bin/env bash

cd sdk
for crate in *
do
  if [ -d "$crate" ]; then
    cd "$crate"
    echo Updating README.md for "$crate"
    cargo readme > README.md
    cd ..
  fi
done
