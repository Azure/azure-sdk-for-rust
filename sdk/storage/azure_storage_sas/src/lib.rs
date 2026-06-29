// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
//!
//! # Supported resource types
//!
//! - [`BlobResource`](resource::blob::BlobResource) — blob-level user delegation SAS (also covers snapshots and versions)
//! - [`ContainerResource`](resource::blob::ContainerResource) — container-level user delegation SAS
//! - [`DirectoryResource`](resource::blob::DirectoryResource) — directory-level (ADLS Gen2) user delegation SAS
//! - [`QueueResource`](resource::queue::QueueResource) — queue-level user delegation SAS

mod builder;
mod ip_range;
mod protocol;
mod signing;

pub mod resource {
    pub mod blob;
    pub mod queue;
}

pub use azure_storage_common::models::UserDelegationKey;
pub use builder::SasBuilder;
pub use ip_range::SasIpRange;
pub use protocol::SasProtocol;
pub use resource::blob::{BlobServiceState, BlobState, ContainerState, DirectoryState};
pub use resource::queue::QueueState;

/// The SAS service version targeted by this crate.
pub(crate) const SAS_VERSION: &str = "2026-04-06";
