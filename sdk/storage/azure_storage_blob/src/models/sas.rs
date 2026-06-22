// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Access Signature (SAS) builder re-exports.
//!
//! Available when the `sas_builder` feature is enabled. These are re-exports from
//! the [`azure_storage_sas`](https://docs.rs/azure_storage_sas) crate; see
//! its documentation for full details on the builder API.
//!
//! For most cases you want the
//! [`BlobClient::generate_user_delegation_sas_url`](crate::BlobClient::generate_user_delegation_sas_url)
//! and [`BlobContainerClient::generate_user_delegation_sas_url`](crate::BlobContainerClient::generate_user_delegation_sas_url)
//! convenience methods rather than the lower-level [`SasBuilder`].

pub use azure_storage_common::models::UserDelegationKey;
pub use azure_storage_sas::resource::blob::{
    Blob, BlobPermissions, Container, ContainerPermissions, Directory,
};
pub use azure_storage_sas::state::{BlobState, ContainerState, DirectoryState};
pub use azure_storage_sas::{BlobServiceState, SasBuilder, SasIpRange, SasProtocol, SAS_VERSION};
