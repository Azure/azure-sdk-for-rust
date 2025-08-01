// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Event Hubs Checkpoint Store implementation using Azure Blob Storage.
//!
//! This crate provides a [`BlobCheckpointStore`] that implements the
//! [`CheckpointStore`] trait from `azure_messaging_eventhubs`. It uses Azure Blob Storage
//! to persist checkpoint and ownership information, enabling durable event processing
//! across application restarts and scale-out scenarios.
//!
//! # Example
//!
//! ```no_run
//! use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
//! use azure_storage_blob::BlobContainerClient;
//! use azure_identity::DefaultAzureCredential;
//! use std::sync::Arc;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let credential = DefaultAzureCredential::new()?;
//! let blob_client = BlobContainerClient::new(
//!     "https://mystorageaccount.blob.core.windows.net",
//!     "container".to_string(),
//!     credential,
//!     None
//! )?;
//!
//! let checkpoint_store = BlobCheckpointStore::new(
//!     blob_client,
//! );
//! # Ok(())
//! # }
//! ```

pub mod checkpoint_store;
pub use checkpoint_store::BlobCheckpointStore;
