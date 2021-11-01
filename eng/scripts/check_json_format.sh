#!/usr/bin/env bash

for f in $(find test -name *.json)
do
    jq . $f > temp
    diff temp $f
    error=$?
    rm temp
    if [ $error -ne 0 ]
    then 
        exit 1
    fi
done