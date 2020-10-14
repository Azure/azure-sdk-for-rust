#!/usr/bin/env bash

echo "Formatting the following files:"
for f in $(find sdk -name *.json)
do
    echo "$f"
    jq . $f > temp
    mv temp $f
done