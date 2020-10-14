#!/usr/bin/env bash

for f in sdk/*
do
	echo "Publishing $f"
    cd $f
    cargo publish
    sleep 20
    cd ../..
done