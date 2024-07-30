// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{headers::Headers, StatusCode};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::{fmt, marker::PhantomData, pin::Pin};
use typespec::error::{ErrorKind, ResultExt};

#[cfg(not(target_arch = "wasm32"))]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;
#[cfg(target_arch = "wasm32")]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// Trait that represents types that can be deserialized from an HTTP response body.
pub trait FromResponseBody: Sized {
    /// Deserialize the response body into type `Self`.
    ///
    /// A [`ResponseBody`] represents a stream of bytes coming from the server.
    /// The server may still be sending data, so it's up to implementors whether they want to wait for the entire body to be received or not.
    /// For example, a type representing a simple REST API response will want to wait for the entire body to be received and then parse the body.
    /// However, a type representing the download of a large file, may not want to do that and instead prepare to stream the body to a file or other destination.
    fn from_response_body(body: ResponseBody) -> impl Future<Output = crate::Result<Self>>;
}

#[macro_export]
macro_rules! json_serializable {
    ($type:ty) => {
        impl $crate::FromResponseBody for $type {
            async fn from_response_body(body: $crate::ResponseBody) -> $crate::Result<Self> {
                body.json().await
            }
        }
    };
}

#[macro_export]
#[cfg(feature = "xml")]
macro_rules! xml_serializable {
    ($type:ty) => {
        impl $crate::FromResponseBody for $type {
            async fn from_response_body(body: $crate::ResponseBody) -> $crate::Result<Self> {
                body.xml().await
            }
        }
    };
}

/// Represents an HTTP response, which may be deserialized into a type `T`.
pub struct Response<T = ()> {
    status: StatusCode,
    headers: Headers,
    body: ResponseBody,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    /// Create an HTTP response from an asynchronous stream of bytes.
    pub fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::new(stream),
            phantom: PhantomData,
        }
    }

    /// Create an HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::from_bytes(bytes),
            phantom: PhantomData,
        }
    }

    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        (self.status, self.headers, self.body)
    }

    /// Fetches the entire body and returns it as raw bytes.
    ///
    /// This method will force the entire body to be downloaded from the server and consume the response.
    /// If you want to parse the body into a type, use [`read_body`](Response::read_body) instead.
    pub fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Produces a new [`Response`] that will parse the body into type `U`.
    ///
    /// This method is intended for use in rare cases where the body of a service response should be parsed into a user-provided type.
    ///
    /// # Example
    /// ```rust
    /// # pub struct GetSecretResponse { }
    /// use typespec_client_core::http::Response;
    /// use serde::Deserialize;
    /// use bytes::Bytes;
    ///
    /// #[derive(Deserialize)]
    /// struct MySecretResponse {
    ///    value: String,
    /// }
    ///
    /// async fn parse_response(response: Response<GetSecretResponse>) {
    ///   // Calling `map` will parse the body into `MySecretResponse` instead of `GetSecretResponse`.
    ///   let my_struct: MySecretResponse = response.map_body().read_body().await.unwrap();
    ///   println!("value: {}", my_struct.value);
    /// }
    // NOTE: You may ask yourself, why not just have 'read_body' which can deserialize into any type?
    // We don't want users to have to specify the target type unless they plan to change it from the default.
    // Service client methods are expected to return `Response<SomethingResponse>` where `SomethingResponse` is the service-specific response type.
    // Users of those clients should be able to call `read_body` without having to specify the target type.
    // However, we ALSO want to allow users to specify a different target type if they want to (for example, to limit what they deserialize or work around a service-specific issue).
    // So this method is a compromise that allows most users to avoid specifying the target type while still allowing users to specify a different target type if they want to.
    // If Rust ever stabilizes type parameter defaults for functions (https://github.com/rust-lang/rust/issues/27336), we could consider changing this.
    pub fn map_body<U>(self) -> Response<U> {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
            phantom: PhantomData,
        }
    }
}

impl<T: FromResponseBody> Response<T> {
    /// Fetches the entire body and tries to convert it into type `T`.
    pub async fn read_body(self) -> crate::Result<T> {
        T::from_response_body(self.body).await
    }
}

impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"(body)")
            .finish()
    }
}

/// A response body stream.
///
/// This body can either be streamed or collected into [`Bytes`].
#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    /// Create a new [`ResponseBody`] from an async stream of bytes.
    fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    /// Create a new [`ResponseBody`] from a byte slice.
    fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        let bytes = bytes.into();
        Self::new(Box::pin(futures::stream::once(async move { Ok(bytes) })))
    }

    /// Collect the stream into a [`Bytes`] collection.
    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut final_result = Vec::new();

        while let Some(res) = self.0.next().await {
            final_result.extend(&res?);
        }

        Ok(final_result.into())
    }

    /// Collect the stream into a [`String`].
    pub async fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.collect().await?)
            .context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }

    /// Deserialize the JSON stream into type `T`.
    #[cfg(feature = "json")]
    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::json::from_json(body)
    }

    /// Deserialize the XML stream into type `T`.
    #[cfg(feature = "xml")]
    pub async fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::xml::read_xml(&body)
    }
}

impl Stream for ResponseBody {
    type Item = crate::Result<Bytes>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.0.poll_next(cx)
    }
}

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseBody")
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ErrorKind;
    use crate::{FromResponseBody, Response};

    #[tokio::test]
    pub async fn body_type_controls_consumption_of_response_body() {
        pub struct LazyBody;
        impl FromResponseBody for LazyBody {
            async fn from_response_body(_body: crate::ResponseBody) -> crate::Result<Self> {
                // Don't actually consume the body
                Ok(LazyBody)
            }
        }

        // Create a response that fails as you read the body.
        let response = Response::<()>::new(
            http_types::StatusCode::Ok,
            crate::headers::Headers::new(),
            Box::pin(futures::stream::once(async {
                Err(ErrorKind::Other.into_error())
            })),
        );

        // Because LazyBody chose not to consume the body, this should succeed.
        let res: crate::Result<LazyBody> = response.map_body().read_body().await;
        assert!(res.is_ok());
    }

    mod json {
        use crate::http::headers::Headers;
        use crate::http::Response;
        use http_types::StatusCode;
        use serde::Deserialize;

        /// An example JSON-serialized response type.
        #[derive(Deserialize)]
        struct GetSecretResponse {
            name: String,
            value: String,
        }
        json_serializable!(GetSecretResponse);

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse> {
            Response::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                "{\"name\":\"my_secret\",\"value\":\"my_value\"}",
            )
        }

        #[tokio::test]
        pub async fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.read_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        pub async fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }
            json_serializable!(MySecretResponse);

            let response = get_secret().map_body();
            let secret: MySecretResponse = response.read_body().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }
    }

    #[cfg(feature = "xml")]
    mod xml {
        use crate::http::headers::Headers;
        use crate::http::Response;
        use http_types::StatusCode;
        use serde::Deserialize;

        /// An example XML-serialized response type.
        #[derive(Deserialize)]
        struct GetSecretResponse {
            name: String,
            value: String,
        }
        xml_serializable!(GetSecretResponse);

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse> {
            Response::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                "<GetSecretResponse><name>my_secret</name><value>my_value</value></GetSecretResponse>",
            )
        }

        #[tokio::test]
        pub async fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.read_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        pub async fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }
            xml_serializable!(MySecretResponse);

            let response = get_secret().map_body();
            let secret: MySecretResponse = response.read_body().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }
    }
}
