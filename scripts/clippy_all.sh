#!/bin/bash

set -ev

for i in azure_sdk* ; do 
	cd $i
	cargo clean
	cd ..
done

cargo clippy


