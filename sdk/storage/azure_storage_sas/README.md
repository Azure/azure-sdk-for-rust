# Azure Storage User Delegation SAS Builder for Rust

This crate provides a type-safe builder for constructing **user delegation** Shared Access Signature (SAS) tokens for Azure Storage resources. Account SAS and stored access policies are not supported.

User delegation SAS tokens are signed with a `UserDelegationKey` obtained from `BlobServiceClient::get_user_delegation_key` or `QueueServiceClient::get_user_delegation_key`.

## Which API should I use?

Most users should depend on `azure_storage_blob` or `azure_storage_queue` with the `sas_builder` feature enabled and call the `generate_user_delegation_sas_url` method on `BlobClient`, `BlobContainerClient`, or `QueueClient`.

Reach for `SasBuilder` directly when:

- you need to generate a SAS for a directory; there is no client method for directory SAS, so `SasBuilder::directory` must be called directly, or
- you need fields or resource shapes the client methods don't expose.
