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
//!
//! # Examples
//!
//! ## Blob user delegation SAS (read a specific blob)
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey, resource::blob::{Blob, BlobPermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(1))?
//!     .blob(Blob::new("images", "photo.jpg"), BlobPermissions::new().read())
//!     .content_type("image/jpeg")
//!     .build();
//!
//! let url = format!("https://myaccount.blob.core.windows.net/images/photo.jpg?{token}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Blob snapshot SAS
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey, resource::blob::{Blob, BlobPermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(1))?
//!     .blob(
//!         Blob::new("backups", "db.bak").snapshot("2025-05-20T10:00:00.0000000Z"),
//!         BlobPermissions::new().read(),
//!     )
//!     .build();
//! // sr=bs in the output, snapshot time included in the signed token
//! # Ok(())
//! # }
//! ```
//!
//! ## Container SAS (list + read all blobs)
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, SasIpRange, UserDelegationKey, resource::blob::{Container, ContainerPermissions}};
//! use std::net::Ipv4Addr;
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(4))?
//!     .ip_range(SasIpRange::Range {
//!         start: Ipv4Addr::new(10, 0, 0, 1).into(),
//!         end: Ipv4Addr::new(10, 0, 0, 255).into(),
//!     })
//!     .container(
//!         Container::new("logs"),
//!         ContainerPermissions::new().read().list(),
//!     )
//!     .build();
//! # Ok(())
//! # }
//! ```
//!
//!
//! ## Queue SAS (read + process messages)
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, SasProtocol, UserDelegationKey, resource::{Queue, QueuePermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(8))?
//!     .protocol(SasProtocol::Https)
//!     .delegated_tenant_id("tenant-id")
//!     .queue(Queue::new("work-items"), QueuePermissions::new().read().process())
//!     .build();
//! # Ok(())
//! # }
//! ```

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
