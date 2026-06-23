// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Access Signature (SAS) builder re-exports.
//!
//! Available when the `sas_builder` feature is enabled. These are re-exports from
//! the [`azure_storage_sas`](https://docs.rs/azure_storage_sas) crate; see
//! its documentation for full details on the builder API.
//!
//! For most cases you want the
//! [`BlobClient::user_delegation_sas`](crate::BlobClient::user_delegation_sas)
//! and [`BlobContainerClient::user_delegation_sas`](crate::BlobContainerClient::user_delegation_sas)
//! convenience methods rather than the lower-level [`SasBuilder`].

pub use crate::sas::{BlobContainerSasBuilder, BlobSasBuilder};
pub use azure_storage_common::models::UserDelegationKey;
pub use azure_storage_sas::resource::blob::{
    BlobPermissions, BlobResource, ContainerPermissions, ContainerResource, DirectoryResource,
};
pub use azure_storage_sas::state::{BlobState, ContainerState, DirectoryState};
pub use azure_storage_sas::{BlobServiceState, SasBuilder, SasIpRange, SasProtocol, SAS_VERSION};
