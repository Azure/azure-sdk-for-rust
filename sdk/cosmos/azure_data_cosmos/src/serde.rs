// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal serialization/deserialization helpers for Azure Cosmos DB SDK.

/// Module for use with `#[serde(with = "...")]` to serialize/deserialize `url::Url` types by parsing from and converting to strings.
pub mod url {
    pub fn serialize<S>(url: &url::Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(url.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<url::Url, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        url::Url::parse(s).map_err(serde::de::Error::custom)
    }
}
