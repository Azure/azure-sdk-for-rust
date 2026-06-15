// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`DatabaseProperties`] — properties of a Cosmos DB database.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

use crate::models::SystemProperties;

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
pub struct DatabaseProperties {
    /// The ID of the database.
    ///
    /// Modeled as `Option<String>` per the Azure SDK for Rust guidelines:
    /// even fields the service contract marks "required" should be optional
    /// in the wire model, so an unexpectedly absent value cannot fail
    /// deserialization of an otherwise-valid response.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A [`SystemProperties`] object containing common system properties for the database.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}
