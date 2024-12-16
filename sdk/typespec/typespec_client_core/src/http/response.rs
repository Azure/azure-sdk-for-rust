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
pub use typespec_macros::Model;

#[cfg(not(target_arch = "wasm32"))]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;
#[cfg(target_arch = "wasm32")]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// Trait that represents types that can be deserialized from an HTTP response body.
///
/// [`Response<T>`] is designed to work with values that may be serialized in various formats (JSON, XML, etc.).
/// In order to support that, the `T` provided must implement [`Model`], which provides the [`Model::from_response_body`]
/// method to deserialize the type from a response body.
pub trait Model: Sized {
    /// Deserialize the response body into type `Self`.
    ///
    /// A [`ResponseBody`] represents a stream of bytes coming from the server.
    /// The server may still be sending data, so it's up to implementors whether they want to wait for the entire body to be received or not.
    /// For example, a type representing a simple REST API response will want to wait for the entire body to be received and then parse the body.
    /// However, a type representing the download of a large file, may not want to do that and instead prepare to stream the body to a file or other destination.
    #[cfg(not(target_arch = "wasm32"))]
    fn from_response_body(
        body: ResponseBody,
    ) -> impl Future<Output = crate::Result<Self>> + Send + Sync;

    #[cfg(target_arch = "wasm32")]
    fn from_response_body(body: ResponseBody) -> impl Future<Output = crate::Result<Self>>;
}

// Allow for deserializing into "raw" JSON.
impl Model for serde_json::Value {
    #[cfg(not(target_arch = "wasm32"))]
    fn from_response_body(
        body: ResponseBody,
    ) -> impl Future<Output = crate::Result<Self>> + Send + Sync {
        body.json()
    }

    #[cfg(target_arch = "wasm32")]
    fn from_response_body(body: ResponseBody) -> impl Future<Output = crate::Result<Self>> {
        body.json()
    }
}

/// An HTTP response.
///
/// The type parameter `T` is a marker type that indicates what the caller should expect to be able to deserialize the body into.
/// Service client methods should return a `Response<SomeModel>` where `SomeModel` is the service-specific response type.
/// For example, a service client method that returns a list of secrets should return `Response<ListSecretsResponse>`.
///
/// Given a `Response<T>`, a user can deserialize the body into the intended body type `T` by calling [`Response::into_body`].
/// However, because the type `T` is just a marker type, the user can also deserialize the body into a different type by calling [`Response::into_json_body`] or [`Response::into_xml_body`],
/// or access the raw body using [`Response::into_raw_body`].
pub struct Response<T = ResponseBody> {
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

    /// Fetches the entire body and returns it as a raw stream of bytes.
    ///
    /// This method will force the entire body to be downloaded from the server and consume the response.
    /// If you want to parse the body into a type, use [`Response::into_body`] instead.
    pub fn into_raw_body(self) -> ResponseBody {
        self.body
    }

    /// Fetches the entire body and tries to deserialize it as JSON into type `U`.
    ///
    /// This method is intended for situations where:
    ///
    /// 1. The response was not given a specific type by the service SDK e.g., the API returned `Response<ResponseBody>`, indicating a raw response.
    /// 2. You want to deserialize the response into your OWN type, rather than the default type specified by the service SDK.
    ///
    /// # Example
    /// ```rust
    /// # pub struct GetSecretResponse { }
    /// use typespec_client_core::http::{Model, Response};
    /// # #[cfg(not(feature = "derive"))]
    /// # use typespec_macros::Model;
    /// use serde::Deserialize;
    /// use bytes::Bytes;
    ///
    /// #[derive(Model, Deserialize)]
    /// struct MySecretResponse {
    ///    value: String,
    /// }
    ///
    /// async fn parse_response(response: Response<GetSecretResponse>) {
    ///   // Calling `into_json_body` will parse the body into `MySecretResponse` instead of `GetSecretResponse`.
    ///   let my_struct: MySecretResponse = response.into_json_body().await.unwrap();
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
    #[cfg(feature = "json")]
    pub async fn into_json_body<U: DeserializeOwned>(self) -> crate::Result<U> {
        self.into_raw_body().json().await
    }

    /// Fetches the entire body and tries to deserialize it as XML into type `U`.
    ///
    /// This method is intended for situations where:
    ///
    /// 1. The response was not given a specific type by the service SDK (i.e. the API returned `Response<ResponseBody>`, indicating a raw response).
    /// 2. You want to deserialize the response into your OWN type, rather than the default type specified by the service SDK.
    ///
    /// # Example
    /// ```rust
    /// # pub struct GetSecretResponse { }
    /// use typespec_client_core::http::{Model, Response};
    /// # #[cfg(not(feature = "derive"))]
    /// # use typespec_macros::Model;
    /// use serde::Deserialize;
    /// use bytes::Bytes;
    ///
    /// #[derive(Model, Deserialize)]
    /// struct MySecretResponse {
    ///    value: String,
    /// }
    ///
    /// async fn parse_response(response: Response<GetSecretResponse>) {
    ///   // Calling `into_xml_body` will parse the body into `MySecretResponse` instead of `GetSecretResponse`.
    ///   let my_struct: MySecretResponse = response.into_xml_body().await.unwrap();
    ///   assert_eq!("hunter2", my_struct.value);
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #    let r: Response<GetSecretResponse> = typespec_client_core::http::Response::from_bytes(
    /// #      http_types::StatusCode::Ok,
    /// #      typespec_client_core::http::headers::Headers::new(),
    /// #      "<Response><name>database_password</name><value>hunter2</value></Response>",
    /// #    );
    /// #    parse_response(r).await;
    /// # }
    /// ```
    #[cfg(feature = "xml")]
    pub async fn into_xml_body<U: DeserializeOwned>(self) -> crate::Result<U> {
        self.into_raw_body().xml().await
    }
}

impl Response<ResponseBody> {
    /// Changes the type of the response body.
    ///
    /// Used to set the "type" of an untyped `Response<ResponseBody>`, transforming it into a `Response<T>`.
    pub(crate) fn with_default_deserialize_type<T>(self) -> Response<T> {
        // This method name is a little clunky, but it's crate-local. If we ever decide it's useful outside this crate (which might happen), we can revisit the name.
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
    /// # use typespec_macros::Model;
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
    /// let model = response.into_body().await.unwrap();
    /// assert_eq!(model.name, "database_password");
    /// assert_eq!(model.value, "hunter2");
    /// # }
    /// ```
    pub async fn into_body(self) -> crate::Result<T> {
        T::from_response_body(self.body).await
    }
}

impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            // TODO: Sanitize headers and emit body as "(body)".
            .finish_non_exhaustive()
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
    use http_types::StatusCode;

    #[tokio::test]
    pub async fn can_extract_raw_body_regardless_of_t() -> Result<(), Box<dyn std::error::Error>> {
        pub struct MyModel;
        impl Model for MyModel {
            async fn from_response_body(_body: ResponseBody) -> crate::Result<Self> {
                panic!("Should never be called in this test");
            }
        }

        {
            let response_raw: Response =
                Response::from_bytes(StatusCode::Ok, Headers::new(), b"Hello".as_slice());
            let body = response_raw.into_raw_body();
            assert_eq!(b"Hello", &*body.collect().await?);
        }

        {
            let response_t: Response<MyModel> =
                Response::from_bytes(StatusCode::Ok, Headers::new(), b"Hello".as_slice())
                    .with_default_deserialize_type();
            let body = response_t.into_raw_body();
            assert_eq!(b"Hello", &*body.collect().await?);
        }

        Ok(())
    }

    mod json {
        use crate::http::headers::Headers;
        use crate::http::Response;
        use http_types::StatusCode;
        use serde::Deserialize;
        use typespec_macros::Model;

        /// An example JSON-serialized response type.
        #[derive(Model, Deserialize)]
        #[typespec(crate = "crate")]
        struct GetSecretResponse {
            name: String,
            value: String,
        }

        /// An example JSON-serialized list response type.
        #[derive(Model, Deserialize)]
        #[typespec(crate = "crate")]
        struct GetSecretListResponse {
            value: Vec<GetSecretResponse>,
            #[serde(rename = "nextLink")]
            next_link: Option<String>,
        }

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse> {
            Response::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"name":"my_secret","value":"my_value"}"#,
            )
        }

        /// A sample service client function to return a list of secrets.
        fn list_secrets() -> Response<GetSecretListResponse> {
            Response::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"value":[{"name":"my_secret","value":"my_value"}],"nextLink":"?page=2"}"#,
            )
        }

        #[tokio::test]
        pub async fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.into_body().await.unwrap();
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

            let response = get_secret();
            let secret: MySecretResponse = response.into_json_body().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }

        #[tokio::test]
        async fn deserialize_pageable_from_body() {
            // We need to efficiently deserialize the body twice to get the "nextLink" but return it to the caller.
            let response = list_secrets();
            let (status, headers, body) = response.deconstruct();
            let bytes = body.collect().await.expect("collect response");
            let model: GetSecretListResponse =
                crate::json::from_json(bytes.clone()).expect("deserialize GetSecretListResponse");
            assert_eq!(status, StatusCode::Ok);
            assert_eq!(model.value.len(), 1);
            assert_eq!(model.next_link, Some("?page=2".to_string()));

            let response: Response<GetSecretListResponse> =
                Response::from_bytes(status, headers, bytes);
            assert_eq!(response.status(), StatusCode::Ok);
            let model = response
                .into_body()
                .await
                .expect("deserialize GetSecretListResponse again");
            assert_eq!(model.next_link, Some("?page=2".to_string()));
        }
    }

    #[cfg(feature = "xml")]
    mod xml {
        use crate::http::headers::Headers;
        use crate::http::Response;
        use http_types::StatusCode;
        use serde::Deserialize;
        use typespec_macros::Model;

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
            let secret = response.into_body().await.unwrap();
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

            let response = get_secret();
            let secret: MySecretResponse = response.into_xml_body().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }
    }
}
