// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::headers::Headers;

/// Properties of an Azure Storage container.
///
#[derive(Clone, Default, Debug)]
pub struct ContainerProperties {
    pub name: String,
    pub last_modified: String,
    pub lease_state: String,
    pub lease_status: String,
    pub lease: String,
    pub has_immutability_policy: String,
    pub has_legal_hold: String,
    pub immutable_storage_with_versioning_enabled: String,
    pub etag: String,
    pub version: String,
}

impl ContainerProperties {
    pub fn build_from_response_headers(response_headers: &Headers) -> ContainerProperties {
        let mut properties = ContainerProperties {
            ..Default::default()
        };
        for (key, value) in response_headers.iter() {
            match key.as_str() {
                "name" => properties.name = String::from(value.as_str()),
                "last-modified" => properties.last_modified = String::from(value.as_str()),
                "etag" => properties.etag = String::from(value.as_str()),
                "x-ms-has-immutability-policy" => {
                    properties.has_immutability_policy = String::from(value.as_str())
                }
                "x-ms-has-legal-hold" => properties.has_legal_hold = String::from(value.as_str()),
                "x-ms-immutable-storage-with-versioning-enabled" => {
                    properties.immutable_storage_with_versioning_enabled =
                        String::from(value.as_str())
                }
                "x-ms-lease-state" => properties.lease_state = String::from(value.as_str()),
                "x-ms-lease-status" => properties.lease_status = String::from(value.as_str()),
                "x-ms-version" => properties.version = String::from(value.as_str()),
                _ => {}
            }
        }
        properties
    }
}
