#!/bin/bash

set -ev

for i in azure_sdk* ; do 
	cd $i
	travis-cargo build 
	travis-cargo test 
	travis-cargo bench
	travis-cargo --only stable doc
	cd ..
done


