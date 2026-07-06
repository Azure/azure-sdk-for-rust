// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
//!
//! # Supported resource types
//!
//! Select a resource by calling the matching method on [`SasBuilder`], then
//! chain the permission setters available in that state:
//!
//! - [`SasBuilder::blob`] — blob-level user delegation SAS (also covers snapshots and versions)
//! - [`SasBuilder::container`] — container-level user delegation SAS
//! - [`SasBuilder::directory`] — directory-level (ADLS Gen2) user delegation SAS
//! - [`SasBuilder::queue`] — queue-level user delegation SAS

mod builder;
mod common;
mod ip_range;
mod protocol;

pub mod blob;
pub mod queue;

pub use azure_storage_common::models::UserDelegationKey;
pub use builder::SasBuilder;
pub use ip_range::SasIpRange;
pub use protocol::SasProtocol;

/// The SAS service version targeted by this crate.
pub(crate) const SAS_VERSION: &str = "2026-04-06";
