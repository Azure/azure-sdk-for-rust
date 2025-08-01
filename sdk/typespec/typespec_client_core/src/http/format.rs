// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::response::ResponseBody;
use serde::de::DeserializeOwned;

/// A marker trait used to indicate the serialization format used for a response body.
///
/// The [`Response`](crate::http::Response) type uses this trait, in the `F` parameter, to determine how to deserialize the body in to the `T` model when using [`Response::into_body`](crate::http::Response::into_body).
///
/// ## How this trait works
///
/// This trait is a little funky, in order to allow the deserialization behavior of the format to be adjusted based on the type of the response body.
/// This is just a marker trait, it has no methods.
/// Instead, the method to actually perform the deserialization is defined in the [`DeserializeWith`] trait.
/// This trait is parameterized by a type that implements the [`Format`] trait.
pub trait Format: std::fmt::Debug {}

/// A trait used to describe a type that can be deserialized using the specified [`Format`].
///
/// This trait defines the `deserialize_with` method, which takes a [`ResponseBody`] and returns the deserialized value.
/// The `F` type parameter allows for different implementations of the `deserialize_with` method based on the specific [`Format`] marker type used.
///
/// Defining our own trait allows us to implement it on foreign types and better customize deserialization for different scenarios.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait DeserializeWith<F: Format>: Sized {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self>;
}

/// Implements [`DeserializeWith<JsonFormat>`] for an arbitrary type `D`
/// that implements [`serde::de::DeserializeOwned`] by deserializing the response body to the specified type using [`serde_json`].
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<D: DeserializeOwned> DeserializeWith<JsonFormat> for D {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.json().await
    }
}

/// A [`Format`] that deserializes response bodies using JSON.
/// This is the default format used for deserialization.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
/// * Any value implementing [`serde::de::DeserializeOwned`] - Deserializes the response body to the specified type using JSON deserialization.
#[derive(Debug, Clone)]
pub struct JsonFormat;

impl Format for JsonFormat {}

/// A [`Format`] that deserializes response bodies using XML.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
/// * Any value implementing [`serde::de::DeserializeOwned`] - Deserializes the response body to the specified type using XML deserialization.
#[cfg(feature = "xml")]
#[derive(Debug, Clone)]
pub struct XmlFormat;

#[cfg(feature = "xml")]
impl Format for XmlFormat {}

#[cfg(feature = "xml")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<D: DeserializeOwned> DeserializeWith<XmlFormat> for D {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.xml().await
    }
}

/// A [`Format`] indicating that the response has no structured format.
/// This includes responses that return raw data and that don't return a response body.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
#[derive(Debug, Clone)]
pub struct NoFormat;

impl Format for NoFormat {}
