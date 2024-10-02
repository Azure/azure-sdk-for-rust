// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types sent to and received from the Cosmos DB API.

use azure_core::{date::OffsetDateTime, Continuable, Model};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

#[cfg(doc)]
use crate::{
    clients::{ContainerClient, ContainerClientMethods, DatabaseClientMethods},
    CosmosClientMethods,
};

/// Returned by Cosmos DB APIs that return a single item.
///
/// In some circumstances, an API that _can_ return an item will **not** return an item.
/// For example, you can use [`ItemOptions`](crate::options::ItemOptions) to configure APIs
/// that write new or updated items to avoid returning the updated item.
/// If you do this, the [`Item<T>`] returned from the API will be empty.
/// Unwrapping an empty [`Item<T>`] with [`Item<T>::into`] will panic.
/// Unwrapping an empty [`Item<T>`] with [`Item<T>::try_into`] returns a [`MissingItem`] error.
/// You can check if an [`Item<T>`] is empty using [`Item<T>::is_empty`]
#[non_exhaustive]
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Item<T>(
    // Item<T> carries an Option<T> to prepare for supporting the "enable content response on write" option on ItemOptions.
    // When this option is set to false, write operations _will not_ return the just-written item.
    // When this option is set to true (default), write operations _will_ return the just-written item.
    // Modelling this in Rust requires that we return an Option<T>.
    // We'll have to use docs to teach users when to use `into`, and when to use `try_into`
    Option<T>,
);

impl<T> Item<T> {
    /// Returns a boolean indicating if the [`Item<T>`] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl<T> Into<T> for Item<T> {
    /// Converts the [`Item<T>`] to the inner item, panicking if the server did not return the item.
    ///
    /// If you know you do not set the "enable content response on write" option to `false` (the default is `true`), this conversion is safe.
    /// If you want to avoid panics, use [`Item<T>::try_into`] instead.
    fn into(self) -> T {
        self.0.unwrap()
    }
}

/// Error type returned when calling [`Item::try_into`] when the item is empty.
///
/// See the documentation for [`Item<T>`] for more information.
pub struct MissingItem;

impl<T> TryInto<T> for Item<T> {
    type Error = MissingItem;

    /// Converts [`Item<T>`] to the inner item, returning [`MissingItem`] if the server did not return the item.
    ///
    /// See the documentation for [`Item<T>`] for more information.
    fn try_into(self) -> Result<T, Self::Error> {
        self.0.ok_or(MissingItem)
    }
}

// The derive macro for Model doesn't currently "auto-detect" trait bounds for generic types.
// So we have to manually implement Model
// See https://github.com/Azure/azure-sdk-for-rust/issues/1803
impl<T: DeserializeOwned> Model for Item<T> {
    #[cfg(not(target_arch = "wasm32"))]
    fn from_response_body(
        body: azure_core::ResponseBody,
    ) -> impl std::future::Future<Output = typespec_client_core::Result<Self>> + Send + Sync {
        body.json()
    }

    #[cfg(target_arch = "wasm32")]
    fn from_response_body(
        body: azure_core::ResponseBody,
    ) -> impl std::future::Future<Output = typespec_client_core::Result<Self>> {
        body.json()
    }
}

fn deserialize_cosmos_timestamp<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds_since_epoch = Option::<i64>::deserialize(deserializer)?;
    match seconds_since_epoch {
        None => Ok(None),
        Some(seconds) => Ok(Some(OffsetDateTime::from_unix_timestamp(seconds).map_err(
            |_| {
                use serde::de::Error;
                D::Error::invalid_value(
                    serde::de::Unexpected::Signed(seconds),
                    &"a valid timestamp",
                )
            },
        )?)),
    }
}

/// A page of query results, where each item is a document of type `T`.
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
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
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize)]
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

    /// A [`OffsetDateTime`] representing the last modified time of the resource.
    #[serde(rename = "_ts")]
    #[serde(deserialize_with = "deserialize_cosmos_timestamp")]
    pub last_modified: Option<OffsetDateTime>,
}

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Model, Clone, Default, Debug, Deserialize)]
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
#[non_exhaustive]
#[derive(Model, Clone, Default, Debug, Deserialize)]
pub struct ContainerProperties {
    /// The ID of the container.
    pub id: String,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}
