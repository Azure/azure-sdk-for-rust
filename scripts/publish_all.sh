#!/usr/bin/env bash

cd sdks/azure_sdk_core
cargo publish
cd ..

sleep 20

cd azure_storage
cargo publish
cd ..

sleep 20

cd azure_auth_aad
cargo publish
cd ..

cd azure_cosmos
cargo publish
cd ..

cd azure_service_bus
cargo publish
cd ..

cd azure_keyvault
cargo publish
cd ..