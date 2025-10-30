// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{error::ErrorKind, http::response::ResponseBody};
use serde::de::DeserializeOwned;

/// A trait used to indicate the serialization format used for a response body.
///
/// The [`Response`](crate::http::Response) type uses this trait in parameter `F` to determine how to deserialize the body in to the model `T` when using [`Response::into_body`](crate::http::Response::into_body).
/// This allows the client library to define the format for each client method so callers can deserialize the model `T` without having to know or specify the format.
pub trait Format: std::fmt::Debug {
    /// Deserialize body into model `T`.
    ///
    /// # Arguments
    /// * `body` - The body to deserialize.
    ///
    /// # Returns
    /// * A `Result` containing the deserialized model `T`.
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T>;
}

/// A trait used to describe a type that can be deserialized using the specified [`Format`].
///
/// This trait defines the `deserialize_with` method, which takes a [`ResponseBody`] and returns the deserialized value.
/// The `F` type parameter allows for different implementations of the `deserialize_with` method based on the specific [`Format`] marker type used.
///
/// Defining our own trait allows us to implement it on foreign types and better customize deserialization for different scenarios.
///
/// # Notes
///
/// This trait does not define a default implementation using [`Format::deserialize`];
/// otherwise, [`NoFormat`] would inadvertently implement [`Response::into_body`](crate::http::Response::into_body) for non-deserializable types.
pub trait DeserializeWith<F: Format>: Sized {
    /// Deserialize the response body using the specified format.
    ///
    /// # Arguments
    /// * `body` - The response body to deserialize.
    ///
    /// # Returns
    /// A `Result` containing the deserialized value of type `Self`, or an error if deserialization fails.
    fn deserialize_with(body: ResponseBody) -> typespec::Result<Self>;
}

/// Implements [`DeserializeWith<JsonFormat>`] for an arbitrary type `D`
/// that implements [`serde::de::DeserializeOwned`] by deserializing the response body to the specified type using [`serde_json`].
impl<D: DeserializeOwned> DeserializeWith<JsonFormat> for D {
    fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.json()
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

impl Format for JsonFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T> {
        crate::json::from_json(body)
    }
}

/// A [`Format`] that deserializes response bodies using XML.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
/// * Any value implementing [`serde::de::DeserializeOwned`] - Deserializes the response body to the specified type using XML deserialization.
#[cfg(feature = "xml")]
#[derive(Debug, Clone)]
pub struct XmlFormat;

#[cfg(feature = "xml")]
impl Format for XmlFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T> {
        crate::xml::from_xml(body.as_ref())
    }
}

#[cfg(feature = "xml")]
impl<D: DeserializeOwned> DeserializeWith<XmlFormat> for D {
    fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.xml()
    }
}

/// A [`Format`] indicating that the response has no structured format.
/// This includes responses that return raw data and that don't return a response body.
///
/// This format supports deserializing response bodies to:
/// * [`ResponseBody`] - The raw response body, without any deserialization.
#[derive(Debug, Clone)]
pub struct NoFormat;

impl Format for NoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(_body: S) -> crate::Result<T> {
        Err(crate::Error::new(
            ErrorKind::DataConversion,
            "not supported",
        ))
    }
}
