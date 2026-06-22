// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Access Signature (SAS) builder re-exports.
//!
//! Available when the `sas` feature is enabled. These are re-exports from
//! the [`azure_storage_sas`](https://docs.rs/azure_storage_sas) crate; see
//! its documentation for full details on the builder API.
//!
//! For most cases you want the
//! [`QueueClient::generate_user_delegation_sas_url`](crate::QueueClient::generate_user_delegation_sas_url)
//! convenience method rather than the lower-level [`SasBuilder`].

pub use azure_storage_common::models::UserDelegationKey;
pub use azure_storage_sas::resource::{Queue, QueuePermissions};
pub use azure_storage_sas::state::QueueState;
pub use azure_storage_sas::{SasBuilder, SasIpRange, SasProtocol, SAS_VERSION};
