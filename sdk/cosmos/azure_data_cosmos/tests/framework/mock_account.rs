// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for building mock `GetDatabaseAccount` responses in fault injection tests.
// cSpell: disable

use azure_core::http::{headers::Headers, StatusCode};
use azure_data_cosmos::fault_injection::CustomResponse;
use azure_data_cosmos::regions::RegionName;

/// Builds a [`CustomResponse`] containing a valid `AccountProperties` JSON payload
/// with the specified writable and readable regions.
///
/// The account name defaults to `"test"` and region endpoints follow the pattern
/// `https://test-{canonical}.documents.azure.com:443/`.
///
/// # Arguments
///
/// * `writable` - Regions that accept writes.
/// * `readable` - Regions that accept reads.
/// * `multi_write` - Whether multi-write (multi-master) is enabled.
pub fn mock_database_account_response(
    writable: &[RegionName],
    readable: &[RegionName],
    multi_write: bool,
) -> CustomResponse {
    let body = mock_database_account_json("test", writable, readable, multi_write);
    CustomResponse {
        status_code: StatusCode::Ok,
        headers: Headers::new(),
        body: body.into_bytes(),
    }
}

/// Builds a valid `AccountProperties` JSON string with the specified regions.
fn mock_database_account_json(
    account_name: &str,
    writable: &[RegionName],
    readable: &[RegionName],
    multi_write: bool,
) -> String {
    let writable_json = regions_to_json(account_name, writable);
    let readable_json = regions_to_json(account_name, readable);

    format!(
        r#"{{
  "_self": "",
  "id": "{account_name}",
  "_rid": "{account_name}.documents.azure.com",
  "media": "//media/",
  "addresses": "//addresses/",
  "_dbs": "//dbs/",
  "writableLocations": [{writable_json}],
  "readableLocations": [{readable_json}],
  "enableMultipleWriteLocations": {multi_write},
  "continuousBackupEnabled": false,
  "enableNRegionSynchronousCommit": false,
  "enablePerPartitionFailoverBehavior": false,
  "userReplicationPolicy": {{ "asyncReplication": false, "minReplicaSetSize": 3, "maxReplicasetSize": 4 }},
  "userConsistencyPolicy": {{ "defaultConsistencyLevel": "Session" }},
  "systemReplicationPolicy": {{ "minReplicaSetSize": 3, "maxReplicasetSize": 4, "asyncReplication": false }},
  "readPolicy": {{ "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 }},
  "queryEngineConfiguration": "{{\"allowNewKeywords\":true}}"
}}"#
    )
}

/// Converts a slice of regions into a JSON array body for writable/readable locations.
fn regions_to_json(account_name: &str, regions: &[RegionName]) -> String {
    regions
        .iter()
        .map(|r| {
            let canonical = r.as_str();
            // Display name uses the canonical form here; the service uses display names
            // like "East US 2" but AccountRegion.name deserializes via RegionName::new()
            // which canonicalizes anyway, so the canonical form works fine for tests.
            format!(
                r#"{{ "name": "{canonical}", "databaseAccountEndpoint": "https://{account_name}-{canonical}.documents.azure.com:443/" }}"#
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos::regions;

    #[test]
    fn mock_account_deserializes() {
        let response = mock_database_account_response(
            &[regions::EAST_US_2, regions::WEST_US],
            &[regions::EAST_US_2, regions::WEST_US],
            false,
        );

        let value: serde_json::Value =
            serde_json::from_slice(&response.body).expect("should deserialize");

        let writable = value["writableLocations"].as_array().unwrap();
        let readable = value["readableLocations"].as_array().unwrap();
        assert_eq!(writable.len(), 2);
        assert_eq!(readable.len(), 2);
        assert_eq!(writable[0]["name"].as_str().unwrap(), "eastus2");
        assert_eq!(writable[1]["name"].as_str().unwrap(), "westus");
        assert_eq!(
            value["enableMultipleWriteLocations"].as_bool().unwrap(),
            false
        );
    }

    #[test]
    fn mock_account_multi_write() {
        let response = mock_database_account_response(
            &[regions::EAST_US, regions::WEST_US],
            &[regions::EAST_US, regions::WEST_US],
            true,
        );

        let value: serde_json::Value =
            serde_json::from_slice(&response.body).expect("should deserialize");

        assert!(value["enableMultipleWriteLocations"].as_bool().unwrap());
    }
}
