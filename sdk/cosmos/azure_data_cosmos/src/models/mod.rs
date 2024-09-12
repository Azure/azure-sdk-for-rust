use azure_core::{
    date::{ComponentRange, OffsetDateTime},
    Model,
};
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{clients::DatabaseClientMethods, CosmosClientMethods};

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

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Model, Debug, Deserialize)]
pub struct DatabaseProperties {
    /// The ID of the database.
    pub id: String,

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
