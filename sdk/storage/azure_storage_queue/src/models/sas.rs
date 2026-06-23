// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Access Signature (SAS) builder re-exports.
//!
//! Available when the `sas_builder` feature is enabled. These are re-exports from
//! the [`azure_storage_sas`](https://docs.rs/azure_storage_sas) crate; see
//! its documentation for full details on the builder API.
//!
//! For most cases you want the
//! [`QueueClient::user_delegation_sas`](crate::QueueClient::user_delegation_sas)
//! convenience method rather than the lower-level [`SasBuilder`].

pub use crate::sas::QueueSasBuilder;
pub use azure_storage_common::models::UserDelegationKey;
pub use azure_storage_sas::resource::queue::{QueuePermissions, QueueResource};
pub use azure_storage_sas::QueueState;
pub use azure_storage_sas::{SasBuilder, SasIpRange, SasProtocol};
