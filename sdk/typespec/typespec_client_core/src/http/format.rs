// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::de::DeserializeOwned;

use crate::http::response::ResponseBody;

/// A marker trait used to indicate the serialization format used for a response body.
///
/// The [`Response<T, F>`] type uses this trait, in the `F` parameter, to determine how to deserialize the body in to the `T` model when using [`Response::into_body`].
///
/// ## How this trait works
///
/// This trait is a little funky, in order to allow the deserialization behavior of the format to be adjusted based on the type of the response body.
/// This is just a marker trait, it has no methods.
/// Instead, the method to actually perform the deserialization is defined in the [`DeserializeWith`] trait.
/// This trait is parameterized by a type that implements the [`Format`] trait.
pub trait Format {}

/// A trait used to describe a type that can be deserialized using the specified [`Format`].
///
/// This trait defines the `deserialize_with` method, which takes a [`ResponseBody`] and returns the deserialized value.
/// The `F` type parameter allows for different implementations of the `deserialize_with` method based on the specific [`Format`] marker type used.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait DeserializeWith<F: Format>: Sized {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self>;
}

/// Implements [`DeserializeWith<T>`] for [`ResponseBody`], by simply returning the body stream as is.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<F: Format> DeserializeWith<F> for ResponseBody {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        Ok(body)
    }
}

/// Implements [`DeserializeWith<DefaultFormat>`] for an arbitrary type `D`
/// that implements [`serde::de::DeserializeOwned`] by deserializing the response body to the specified type using [`serde_json`].
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<D: DeserializeOwned> DeserializeWith<DefaultFormat> for D {
    async fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.json().await
    }
}

/// The default format used for deserialization.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
/// * Any value implementing [`serde::de::DeserializeOwned`] - Deserializes the response body to the specified type using JSON deserialization.
pub struct DefaultFormat;

impl Format for DefaultFormat {}

/// A [`Format`] that deserializes response bodies using XML.
#[cfg(feature = "xml")]
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
