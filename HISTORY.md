# History

## azure_sdk_for_rust crate

Francesco Cogno first published the [azure_sdk_for_rust](https://crates.io/crates/azure_sdk_for_rust) crate in January of 2016. The initial commit to https://github.com/MindFlavor/AzureSDKForRust was December of 2015. It supported [Azure Storage](https://docs.microsoft.com/azure/storage/) and [Azure Cosmos DB](https://azure.microsoft.com/services/cosmos-db/) services. azure_sdk_for_rust 0.12.0 was last published in June of 2019.

## split into azure_sdk crates

Francesco split up azure_sdk_for_rust in June of 2019 and published:

- azure_sdk_core
- azure_sdk_auth_aad
- azure_sdk_cosmos
- azure_sdk_storage_core
- azure_sdk_storage_account
- azure_sdk_storage_blob
- azure_sdk_storage_table
- azure_sdk_storage_queue

They were last published in September of 2020 before moving the code to https://github.com/Azure/azure-sdk-for-rust in October of 2020.

## Azure SDK for Rust crates

The above crates have become:
- azure_core
- azure_identity
- azure_cosmos
- azure_storage

## azure-sdk-keyvault

Guy Waldman initial commit at https://github.com/guywaldman/azure-sdk-keyvault and publish of azure-sdk-keyvault was in May of 2020. In was added to this repository in October of 2020 and has become azure_security_keyvault.

## AutoRust

Cameron Taggart initial commit at https://github.com/ctaggart/autorust was in April of 2020. The initial 100 generated control plane services committed into Azure SDK for Rust was in October of 2020.
