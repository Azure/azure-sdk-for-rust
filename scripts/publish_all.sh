#!/usr/bin/env bash

cd azure_sdk_core
cargo publish
cd ..

sleep 20

cd azure_sdk_storage_core
cargo publish
cd ..

sleep 20

cd azure_sdk_storage_account
cargo publish
cd ..

cd azure_sdk_storage_blob
cargo publish
cd ..

cd azure_sdk_storage_table
cargo publish
cd ..

cd azure_sdk_storage_queue
cargo publish
cd ..

cd azure_sdk_auth_aad
cargo publish
cd ..

cd azure_sdk_cosmos
cargo publish
cd ..

cd azure_sdk_service_bus
cargo publish
cd ..
