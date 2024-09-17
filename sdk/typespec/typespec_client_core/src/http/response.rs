// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{headers::Headers, StatusCode};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::{fmt, marker::PhantomData, pin::Pin};
use typespec::error::{ErrorKind, ResultExt};

#[cfg(feature = "derive")]
pub use typespec_derive::Model;

#[cfg(not(target_arch = "wasm32"))]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;
#[cfg(target_arch = "wasm32")]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// Trait that represents types that can be deserialized from an HTTP response body.
pub trait Model: Sized {
    /// Deserialize the response body into type `Self`.
    ///
    /// A [`ResponseBody`] represents a stream of bytes coming from the server.
    /// The server may still be sending data, so it's up to implementors whether they want to wait for the entire body to be received or not.
    /// For example, a type representing a simple REST API response will want to wait for the entire body to be received and then parse the body.
    /// However, a type representing the download of a large file, may not want to do that and instead prepare to stream the body to a file or other destination.
    fn from_response_body(body: ResponseBody) -> impl Future<Output = crate::Result<Self>>;
}

/// An HTTP response.
///
/// The type parameter `T` is a marker type that indicates what the caller should expect to be able to deserialize the body into.
/// Service client methods should return a `Response<SomeModel>` where `SomeModel` is the service-specific response type.
/// For example, a service client method that returns a list of secrets should return `Response<ListSecretsResponse>`.
///
/// Given a `Response<T>`, a user can deserialize the body into the intended body type `T` by calling [`Response::deserialize_body`].
/// However, because the type `T` is just a marker type, the user can also deserialize the body into a different type by calling [`Response::deserialize_body_into`].
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
    /// If you want to parse the body into a type, use [`read_body`](Response::deserialize_body) instead.
    pub fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Fetches the entire body and tries to convert it into type `U`.
    ///
    /// This method is intended for use in rare cases where the body of a service response should be parsed into a user-provided type.
    ///
    /// # Example
    /// ```rust
    /// # pub struct GetSecretResponse { }
    /// use typespec_client_core::http::{Model, Response};
    /// # #[cfg(not(feature = "derive"))]
    /// # use typespec_derive::Model;
    /// use serde::Deserialize;
    /// use bytes::Bytes;
    ///
    /// #[derive(Model, Deserialize)]
    /// struct MySecretResponse {
    ///    value: String,
    /// }
    ///
    /// async fn parse_response(response: Response<GetSecretResponse>) {
    ///   // Calling `deserialize_body_into` will parse the body into `MySecretResponse` instead of `GetSecretResponse`.
    ///   let my_struct: MySecretResponse = response.deserialize_body_into().await.unwrap();
    ///   assert_eq!("hunter2", my_struct.value);
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #    let r: Response<GetSecretResponse> = typespec_client_core::http::Response::from_bytes(
    /// #      http_types::StatusCode::Ok,
    /// #      typespec_client_core::http::headers::Headers::new(),
    /// #      "{\"name\":\"database_password\",\"value\":\"hunter2\"}",
    /// #    );
    /// #    parse_response(r).await;
    /// # }
    /// ```
    pub async fn deserialize_body_into<U: Model>(self) -> crate::Result<U> {
        U::from_response_body(self.body).await
    }
}

impl Response<()> {
    /// Changes the type of the response body.
    ///
    /// Used to set the "type" of an untyped `Response<()>`, transforming it into a `Response<T>`.
    pub(crate) fn with_default_deserialize_type<T>(self) -> Response<T> {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
            phantom: PhantomData,
        }
    }
}

impl<T: Model> Response<T> {
    /// Fetches the entire body and tries to convert it into type `T`.
    ///
    /// This is the preferred method for parsing the body of a service response into it's default model type.
    ///
    /// # Example
    /// ```rust
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::Model;
    /// # #[cfg(not(feature = "derive"))]
    /// # use typespec_derive::Model;
    /// # #[derive(Model, Deserialize)]
    /// # pub struct GetSecretResponse {
    /// #   name: String,
    /// #   value: String,
    /// # }
    /// # pub struct SecretClient { }
    /// # impl SecretClient {
    /// #   pub async fn get_secret(&self) -> typespec_client_core::http::Response<GetSecretResponse> {
    /// #    typespec_client_core::http::Response::from_bytes(
    /// #      http_types::StatusCode::Ok,
    /// #      typespec_client_core::http::headers::Headers::new(),
    /// #      "{\"name\":\"database_password\",\"value\":\"hunter2\"}",
    /// #    )
    /// #  }
    /// # }
    /// # pub fn create_secret_client() -> SecretClient {
    /// #   SecretClient { }
    /// # }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let secret_client = create_secret_client();
    /// let response = secret_client.get_secret().await;
    /// assert_eq!(response.status(), http_types::StatusCode::Ok);
    /// let model = response.deserialize_body().await.unwrap();
    /// assert_eq!(model.name, "database_password");
    /// assert_eq!(model.value, "hunter2");
    /// # }
    /// ```
    pub async fn deserialize_body(self) -> crate::Result<T> {
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
    use crate::http::headers::Headers;
    use crate::http::{response::ResponseBody, Model, Response};
    use typespec::error::ErrorKind;

    #[tokio::test]
    pub async fn body_type_controls_consumption_of_response_body() {
        pub struct LazyBody;
        impl Model for LazyBody {
            async fn from_response_body(_body: ResponseBody) -> crate::Result<Self> {
                // Don't actually consume the body
                Ok(LazyBody)
            }
        }

        // Create a response that fails as you read the body.
        let response = Response::<()>::new(
            http_types::StatusCode::Ok,
            Headers::new(),
            Box::pin(futures::stream::once(async {
                Err(ErrorKind::Other.into_error())
            })),
        );

        // Because LazyBody chose not to consume the body, this should succeed.
        let res: crate::Result<LazyBody> = response.deserialize_body_into().await;
        assert!(res.is_ok());
    }

    mod json {
        use crate::http::headers::Headers;
        use crate::http::Response;
        use http_types::StatusCode;
        use serde::Deserialize;
        use typespec_derive::Model;

        /// An example JSON-serialized response type.
        #[derive(Model, Deserialize)]
        #[typespec(crate = "crate")]
        struct GetSecretResponse {
            name: String,
            value: String,
        }

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
            let secret = response.deserialize_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        pub async fn deserialize_alternate_type() {
            #[derive(Model, Deserialize)]
            #[typespec(crate = "crate")]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }

            let response = get_secret();
            let secret: MySecretResponse = response.deserialize_body_into().await.unwrap();
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
        use typespec_derive::Model;

        /// An example XML-serialized response type.
        #[derive(Model, Deserialize)]
        #[typespec(crate = "crate")]
        #[typespec(format = "xml")]
        struct GetSecretResponse {
            name: String,
            value: String,
        }

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
            let secret = response.deserialize_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        pub async fn deserialize_alternate_type() {
            #[derive(Model, Deserialize)]
            #[typespec(crate = "crate")]
            #[typespec(format = "xml")]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }

            let response = get_secret();
            let secret: MySecretResponse = response.deserialize_body_into().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }
    }
}
