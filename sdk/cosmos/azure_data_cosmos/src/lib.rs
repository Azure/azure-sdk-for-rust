// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
//!
//! ## Unstable OpenTelemetry
//!
//! Enable the off-by-default `unstable_opentelemetry` feature to emit metrics and
//! distributed traces through the OpenTelemetry diagnostics handlers. This
//! integration is unstable because the `opentelemetry` crate is still in
//! preview. Diagnostics contexts and non-OpenTelemetry handlers remain available
//! without the feature.

// =========================================================================
// Public API
// =========================================================================

pub use account_endpoint::AccountEndpoint;
pub use account_reference::AccountReference;
#[doc(inline)]
pub use clients::{ContainerClient, CosmosClient, CosmosClientBuilder, DatabaseClient};
#[cfg(feature = "unstable_dtx")]
pub use clients::{DistributedReadTransaction, DistributedWriteTransaction};
pub use credential::CosmosCredential;
pub use error::{CosmosError, CosmosStatus, Result, SubStatusCode};
pub use feed::{FeedScope, Query};
pub use models::{PartitionKey, TransactionalBatch};
pub use options::RoutingStrategy;
pub use resource_identity::{ResourceId, ResourceIdentity};
pub use runtime::{CosmosRuntime, CosmosRuntimeBuilder};

// =========================================================================
// Public modules
// =========================================================================

pub mod clients;
pub mod diagnostics;
pub mod error;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
pub mod feed;
pub mod models;
pub mod options;

// =========================================================================
// Internal modules
// =========================================================================

mod account_endpoint;
mod account_reference;
mod constants;
mod credential;
mod driver_bridge;
mod region_proximity;
mod resource_identity;
mod runtime;
mod session_helpers;

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    #[test]
    fn docs_rs_includes_public_features() {
        // These features are already enabled by the default feature set.
        const IGNORE: &[&str] = &["default", "hmac_rust", "reqwest", "rustls"];

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let manifest: toml::Table = fs::read_to_string(path)
            .expect("read Cargo.toml")
            .parse()
            .expect("parse Cargo.toml");
        let features = manifest
            .get("features")
            .map(|value| value.as_table().expect("features must be a table"));
        let docs_metadata =
            ["package", "metadata", "docs", "rs"]
                .iter()
                .try_fold(&manifest, |table, key| {
                    table
                        .get(*key)
                        .map(|value| value.as_table().expect("docs.rs metadata must be tables"))
                });
        let docs_features: Vec<&str> = docs_metadata
            .and_then(|table| table.get("features"))
            .map(|value| {
                value
                    .as_array()
                    .expect("docs.rs features must be an array")
                    .iter()
                    .map(|value| value.as_str().expect("feature names must be strings"))
                    .collect()
            })
            .unwrap_or_default();
        let missing: Vec<&str> = features
            .into_iter()
            .flat_map(|features| features.keys())
            .map(String::as_str)
            .filter(|feature| {
                !feature.starts_with("__")
                    && !IGNORE.contains(feature)
                    && !docs_features.contains(feature)
            })
            .collect();
        assert!(
            missing.is_empty(),
            "Add these features to package.metadata.docs.rs.features: {missing:?}"
        );
    }
}
