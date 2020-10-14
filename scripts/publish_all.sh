#!/usr/bin/env bash

cd sdks/azure_core
cargo publish
cd ..

sleep 20

cd azure_storage
cargo publish
cd ..

sleep 20

cd azure_identity
cargo publish
cd ..

cd azure_cosmos
cargo publish
cd ..

cd azure_service_bus
cargo publish
cd ..

cd azure_key_vault
cargo publish
cd ..