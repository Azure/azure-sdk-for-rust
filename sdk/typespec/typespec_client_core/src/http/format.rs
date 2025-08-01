// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{response::ResponseBody, Body};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

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

/// A trait used to describe a type that can be serialized using the specified [`Format`].
///
/// This trait defines the `serialize_with` method, which takes a value and returns a [`Body`].
/// The `F` type parameter allows for different implementations of the `serialize_with` method based on the specific [`Format`] marker type used.
///
/// Defining our own trait allows us to implement it on foreign types and better customize serialization for different scenarios.
pub trait SerializeWith<F: Format>: Sized {
    fn serialize_with(value: Self) -> typespec::Result<Body>;
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

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        error::{Error, ErrorKind, ResultExt as _},
        time::OffsetDateTime,
    };
    use bytes::BufMut as _;
    use serde::Serializer;
    use serde_json::value::RawValue;
    use time::format_description::well_known::Rfc3339;

    macro_rules! impl_serialize_with {
        ($t:ty) => {
            impl $crate::http::SerializeWith<$crate::http::JsonFormat> for $t {
                fn serialize_with(value: Self) -> $crate::Result<$crate::http::Body> {
                    Ok($crate::json::to_json(&value)?.into())
                }
            }
        };

        ($($t:ty),*) => {
            $(impl_serialize_with!($t);)*
        };
    }

    impl_serialize_with!(bool);
    impl_serialize_with!(&str, String);
    impl_serialize_with!(i32, i64);
    impl_serialize_with!(f32, f64);
    impl_serialize_with!(serde_json::Value);

    impl SerializeWith<JsonFormat> for OffsetDateTime {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            let value = value
                .format(&Rfc3339)
                .with_context(ErrorKind::DataConversion, || "failed formatting datetime")?;
            Ok(crate::json::to_json(&value)?.into())
        }
    }

    #[cfg(feature = "decimal")]
    impl SerializeWith<JsonFormat> for rust_decimal::Decimal {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            Ok(crate::json::to_json(&value.to_string())?.into())
        }
    }

    impl<T: SerializeWith<JsonFormat>> SerializeWith<JsonFormat> for Vec<T> {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            use serde::ser::SerializeSeq;

            let mut buf = vec![].writer();
            let mut ser = serde_json::Serializer::new(&mut buf);
            let mut seq = ser
                .serialize_seq(Some(value.len()))
                .with_context(ErrorKind::Io, || "failed sequence start")?;
            for elem in value {
                let Body::Bytes(raw) = T::serialize_with(elem)? else {
                    return Err(Error::new(
                        ErrorKind::DataConversion,
                        "failed json serialization",
                    ));
                };
                let raw = RawValue::from_string(String::from_utf8(raw.to_vec())?)?;
                seq.serialize_element(&raw)
                    .with_context(ErrorKind::Io, || "failed sequence element")?;
            }
            seq.end()
                .with_context(ErrorKind::Io, || "failed sequence end")?;

            let buf = buf.into_inner();
            Ok(buf.into())
        }
    }

    impl<T: SerializeWith<JsonFormat>> SerializeWith<JsonFormat> for HashMap<String, T> {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            use serde::ser::SerializeMap;

            let mut buf = vec![].writer();
            let mut ser = serde_json::Serializer::new(&mut buf);
            let mut seq = ser
                .serialize_map(Some(value.len()))
                .with_context(ErrorKind::Io, || "failed map start")?;
            for (key, elem) in value {
                let Body::Bytes(raw) = T::serialize_with(elem)? else {
                    return Err(Error::new(
                        ErrorKind::DataConversion,
                        "failed json serialization",
                    ));
                };
                let raw = RawValue::from_string(String::from_utf8(raw.to_vec())?)?;
                seq.serialize_entry(&key, &raw)
                    .with_context(ErrorKind::Io, || "failed map entry")?;
            }
            seq.end().with_context(ErrorKind::Io, || "failed map end")?;

            let buf = buf.into_inner();
            Ok(buf.into())
        }
    }
}

#[cfg(feature = "xml")]
mod xml {
    use super::*;
    use crate::{
        error::{Error, ErrorKind, ResultExt as _},
        time::OffsetDateTime,
    };
    use serde::Serializer;
    use time::format_description::well_known::Rfc3339;

    macro_rules! impl_serialize_with {
        ($t:ty) => {
            impl $crate::http::SerializeWith<$crate::http::XmlFormat> for $t {
                fn serialize_with(value: Self) -> $crate::Result<$crate::http::Body> {
                    let value = ::quick_xml::se::to_string(&value).with_context(ErrorKind::DataConversion, || {
                        let t = core::any::type_name::<$t>();
                        format!("failed to serialize {t} into xml")
                    })?;
                    Ok(value.into())
                }
            }
        };

        ($($t:ty),*) => {
            $(impl_serialize_with!($t);)*
        };
    }

    impl_serialize_with!(bool);
    impl_serialize_with!(&str, String);
    impl_serialize_with!(i32, i64);
    impl_serialize_with!(f32, f64);
    impl_serialize_with!(serde_json::Value);

    impl SerializeWith<XmlFormat> for OffsetDateTime {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            let value = value
                .format(&Rfc3339)
                .with_context(ErrorKind::DataConversion, || "failed formatting datetime")?;
            Ok(crate::xml::to_xml(&value)?.into())
        }
    }

    #[cfg(feature = "decimal")]
    impl SerializeWith<XmlFormat> for rust_decimal::Decimal {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            Ok(crate::xml::to_xml(&value.to_string())?.into())
        }
    }

    impl<T: SerializeWith<XmlFormat>> SerializeWith<XmlFormat> for Vec<T> {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            use serde::ser::SerializeSeq;

            let mut buf = bytes::BytesMut::new();
            let ser = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
                .with_context(ErrorKind::Io, || "failed xml root")?;
            let mut seq = ser
                .serialize_seq(Some(value.len()))
                .with_context(ErrorKind::Io, || "failed sequence start")?;
            for elem in value {
                let Body::Bytes(raw) = T::serialize_with(elem)? else {
                    return Err(Error::new(
                        ErrorKind::DataConversion,
                        "failed xml serialization",
                    ));
                };
                let raw = String::from_utf8(raw.to_vec())?;
                seq.serialize_element(&raw)
                    .with_context(ErrorKind::Io, || "failed sequence element")?;
            }
            seq.end()
                .with_context(ErrorKind::Io, || "failed sequence end")?;

            let buf: crate::Bytes = buf.into();
            Ok(buf.into())
        }
    }

    impl<T: SerializeWith<XmlFormat>> SerializeWith<XmlFormat> for HashMap<String, T> {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            use serde::ser::SerializeMap;

            let mut buf = bytes::BytesMut::new();
            let ser = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
                .with_context(ErrorKind::Io, || "failed xml root")?;
            let mut seq = ser
                .serialize_map(Some(value.len()))
                .with_context(ErrorKind::Io, || "failed map start")?;
            for (key, elem) in value {
                let Body::Bytes(raw) = T::serialize_with(elem)? else {
                    return Err(Error::new(
                        ErrorKind::DataConversion,
                        "failed xml serialization",
                    ));
                };
                let raw = String::from_utf8(raw.to_vec())?;
                seq.serialize_entry(&key, &raw)
                    .with_context(ErrorKind::Io, || "failed map entry")?;
            }
            seq.end().with_context(ErrorKind::Io, || "failed map end")?;

            let buf: crate::Bytes = buf.into();
            Ok(buf.into())
        }
    }
}
