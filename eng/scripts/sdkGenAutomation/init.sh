#!/bin/bash

# install tsp-client globally (local install may interfere with tooling)
echo Install tsp-client
npm install -g @azure-tools/typespec-client-generator-cli > /dev/null

echo "{}" >> $2
echo "[Generate] init success!!!"
