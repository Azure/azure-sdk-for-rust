# 0.20.0 (2023-02)

- [#1532](https://github.com/Azure/azure-sdk-for-rust/pull/1532) add azure_identity::create_credential(), SpecificAzureCredential, AppServiceManagedIdentityCredential, VirtualMachineManagedIdentityCredential
    - Most credentials now may fail earlier, when they are created, instead of only during `get_token`.
    - `DefaultAzureCredentialBuilder::build` now returns a `Result`. If fails when it is unable to create a least one source credential.


# 0.18.0 (2023-12)

- Removed AutoRefreshingTokenCredential, instead all token credentials now implement caching

# 0.3.0 (2022-05)

- [#756](https://github.com/Azure/azure-sdk-for-rust/pull/756) Export credentials from azure_identity
    - BREAKING CHANGE: the credential types have moved. For example:
    - use `azure_identity::DefaultAzureCredential` instead of `azure_identity::token_credentials::DefaultAzureCredential`
- [#751](https://github.com/Azure/azure-sdk-for-rust/pull/751) datetime from azure cli token is in the local timezone
- [#748](https://github.com/Azure/azure-sdk-for-rust/pull/748) adding option to specify client_id for MSI

# 0.2.0 (2022-05)

- update to azure_core 0.2.1

# 0.1.1 (2022-01)

- initial publish to [crates.io](https://crates.io/crates/azure_identity)
