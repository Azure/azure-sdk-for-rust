// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types sent to and received from the Cosmos DB API.

use azure_core::{
    date::{ComponentRange, OffsetDateTime},
    Continuable,
};
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{
    clients::{ContainerClient, ContainerClientMethods, DatabaseClientMethods},
    CosmosClientMethods,
};

/// Represents a timestamp in the format expected by Cosmos DB.
///
/// Cosmos DB timestamps are represented as the number of seconds since the Unix epoch.
/// Use [`CosmosTimestamp::try_into`] implementation to convert this into an [`OffsetDateTime`].
#[derive(Serialize, Deserialize, Debug)]
pub struct CosmosTimestamp(i64);

/// Converts a [`CosmosTimestamp`] into a [`OffsetDateTime`].
impl TryInto<OffsetDateTime> for CosmosTimestamp {
    type Error = ComponentRange;

    /// Attempts to convert this [`CosmosTimestamp`] into a [`OffsetDateTime`].
    fn try_into(self) -> Result<OffsetDateTime, Self::Error> {
        OffsetDateTime::from_unix_timestamp(self.0)
    }
}

/// A page of query results, where each item is a document of type `T`.
#[derive(Debug)]
pub struct QueryResults<T> {
    pub items: Vec<T>,
    pub query_metrics: Option<String>,
    pub index_metrics: Option<String>,
    pub continuation_token: Option<String>,
}

impl<T> Continuable for QueryResults<T> {
    type Continuation = String;

    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}

/// Common system properties returned for most Cosmos DB resources.
#[derive(Debug, Deserialize)]
pub struct SystemProperties {
    /// The entity tag associated with the resource.
    #[serde(rename = "_etag")]
    pub etag: Option<azure_core::Etag>,

    /// The self-link associated with the resource.
    #[serde(rename = "_self")]
    pub self_link: Option<String>,

    /// The system-generated unique identifier associated with the resource.
    #[serde(rename = "_rid")]
    pub resource_id: Option<String>,

    /// A [`CosmosTimestamp`] representing the last modified time of the resource.
    #[serde(rename = "_ts")]
    pub last_modified: Option<CosmosTimestamp>,
}

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Debug, Deserialize)]
pub struct DatabaseProperties {
    /// The ID of the database.
    pub id: String,

    /// A [`SystemProperties`] object containing common system properties for the database.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

/// Properties of a Cosmos DB container.
///
/// Returned by [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Debug, Deserialize)]
pub struct ContainerProperties {
    /// The ID of the container.
    pub id: String,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}
