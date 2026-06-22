// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
//!
//! # Supported resource types
//!
//! - [`Blob`](resource::blob::Blob) — blob-level user delegation SAS (also covers snapshots and versions)
//! - [`Container`](resource::blob::Container) — container-level user delegation SAS
//! - [`Directory`](resource::blob::Directory) — directory-level (ADLS Gen2) user delegation SAS
//! - [`Queue`](resource::Queue) — queue-level user delegation SAS

mod builder;
mod ip_range;
mod protocol;
pub mod resource;

pub use azure_storage_common::models::UserDelegationKey;
pub use builder::state;
pub use builder::BlobServiceState;
pub use builder::SasBuilder;
pub use ip_range::SasIpRange;
pub use protocol::SasProtocol;

/// The SAS service version targeted by this crate.
pub const SAS_VERSION: &str = "2026-04-06";
