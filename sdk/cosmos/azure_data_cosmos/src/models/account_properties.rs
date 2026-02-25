// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for deserializing the Cosmos DB Account (DatabaseAccount) JSON payload.

use crate::regions::RegionName;
use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a single regional endpoint for the Cosmos DB account (readable or writable).
#[derive(SafeDebug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountRegion {
    pub name: RegionName,

    #[serde(with = "crate::serde::url")]
    pub database_account_endpoint: Url,
}

/// Top-level Cosmos DB DatabaseAccount properties returned by the control plane.
///
/// This struct captures a subset of fields surfaced in the account read payload.
/// Only the region lists are retained; other fields in the server response are
/// silently ignored during deserialization.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountProperties {
    /// Regions currently accepting writes for the account (multi-master may yield >1).
    pub writable_locations: Vec<AccountRegion>,

    /// Regions from which the account can be read (includes writable regions plus any read regions).
    pub readable_locations: Vec<AccountRegion>,

    /// Allows failover at a per-partition granularity instead of full-region only.
    pub enable_per_partition_failover_behavior: bool,
}

#[cfg(test)]
// cSpell:disable
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "_self" : "",
      "id" : "test",
      "writableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://test-westus2.documents.azure.com:443/" } ],
      "readableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://test-westus2.documents.azure.com:443/" } ],
      "enableMultipleWriteLocations" : false
    }"#;

    #[test]
    fn deserialize_account_props() {
        let props: AccountProperties = serde_json::from_str(SAMPLE).expect("deserialize");
        assert_eq!(props.writable_locations.len(), 1);
        assert_eq!(props.readable_locations.len(), 1);
    }
}
