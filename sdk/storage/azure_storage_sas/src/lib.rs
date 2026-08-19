// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! User delegation Shared Access Signature (SAS) builder for Azure Storage.
//!
//! # Supported resource types
//!
//! Select a resource by calling the matching method on [`SasTokenBuilder`] or
//! [`SasUrlBuilder`], then chain the permission setters available in that state:
//!
//! - [`SasTokenBuilder::blob`] — blob-level user delegation SAS (also covers snapshots and versions)
//! - [`SasTokenBuilder::container`] — container-level user delegation SAS
//! - [`SasTokenBuilder::directory`] — directory-level (ADLS Gen2) user delegation SAS
//! - [`SasTokenBuilder::queue`] — queue-level user delegation SAS
//! - [`SasTokenBuilder::table`] — table-level user delegation SAS
//! - [`SasTokenBuilder::file`] — file-level user delegation SAS
//! - [`SasTokenBuilder::share`] — share-level user delegation SAS
//!
//! # Output format
//!
//! Use [`SasTokenBuilder`] to produce a [`SasToken`] — the signed query string
//! (e.g. `sv=...&sr=b&...&sig=...`). Use [`SasUrlBuilder`] to produce a [`SasUrl`]
//! — a complete URL with the signed query string embedded.
//!
//! ```rust,ignore
//! let token: SasToken = SasTokenBuilder::new("myaccount", &udk, expiry)?.blob("c", "b").read().build();
//! let url: SasUrl = SasUrlBuilder::new("myaccount", &udk, expiry)?.blob("c", "b").read().build()?;
//! ```
//!
//! Use [`SasUrlBuilder::endpoint`] to override the default
//! `https://{account}.{service}.core.windows.net` base (e.g. for Azurite or sovereign clouds).

mod builder;
mod common;
mod ip_range;
mod protocol;
mod url;

pub mod blob;
pub mod file;
pub mod queue;
pub mod table;

pub use azure_storage_common::models::UserDelegationKey;
pub use builder::{SasToken, SasTokenBuilder, SasUrl, SasUrlBuilder};
pub use ip_range::SasIpRange;
pub use protocol::SasProtocol;

pub(crate) const SAS_VERSION: &str = "2026-04-06";
