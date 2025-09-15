// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP responses.

use crate::{
    http::{headers::Headers, DeserializeWith, Format, JsonFormat, RawResponse, StatusCode},
    Bytes,
};
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::{fmt, marker::PhantomData, pin::Pin};
use typespec::error::{ErrorKind, ResultExt};

/// A pinned stream of bytes that can be sent as a response body.
#[cfg(not(target_arch = "wasm32"))]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send>>;
/// A pinned stream of bytes that can be sent as a response body.
#[cfg(target_arch = "wasm32")]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// A raw HTTP response with status, headers, and body.
#[derive(Debug)]
pub struct BufResponse {
    status: StatusCode,
    headers: Headers,
    body: ResponseBody,
}

impl BufResponse {
    /// Create a raw HTTP response from an asynchronous stream of bytes.
    pub fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::new(stream),
        }
    }

    /// Create a raw HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::from_bytes(bytes),
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

    /// Deconstruct the raw HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        (self.status, self.headers, self.body)
    }

    /// Get the raw response body.
    pub fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Read the entire body and convert into a [`RawResponse`].
    pub async fn try_into_raw_response(self) -> crate::Result<RawResponse> {
        let body = self.body.collect().await?;
        Ok(RawResponse::from_bytes(self.status, self.headers, body))
    }
}

/// A typed HTTP response.
///
/// The type parameter `T` is a marker type that indicates what the caller should expect to be able to deserialize the body into.
/// Service client methods should return a `Response<SomeModel>` where `SomeModel` is the service-specific response type.
/// For example, a service client method that returns a list of secrets should return `Response<ListSecretsResponse>`.
///
/// Given a `Response<T>`, a user can deserialize the body into the intended body type `T` by calling [`Response::into_body`].
/// However, because the type `T` is just a marker type, you can also access the raw body using [`Response::into_raw_body`].
pub struct Response<T, F = JsonFormat> {
    raw: BufResponse,
    phantom: PhantomData<(T, F)>,
}

impl<T, F> Response<T, F> {
    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.raw.status()
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        self.raw.headers()
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        self.raw.deconstruct()
    }

    /// Fetches the entire body and returns it as a raw stream of bytes.
    ///
    /// This method will force the entire body to be downloaded from the server and consume the response.
    /// If you want to parse the body into a type, use [`Response::into_body`] instead.
    pub fn into_raw_body(self) -> ResponseBody {
        self.raw.into_body()
    }
}

impl<T: DeserializeWith<F>, F: Format> Response<T, F> {
    /// Fetches the entire body and tries to convert it into type `T`.
    ///
    /// This is the preferred method for parsing the body of a service response into it's default model type.
    ///
    /// # Example
    /// ```rust
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::StatusCode;
    /// # #[derive(Deserialize)]
    /// # pub struct GetSecretResponse {
    /// #   name: String,
    /// #   value: String,
    /// # }
    /// # pub struct SecretClient { }
    /// # impl SecretClient {
    /// #   pub async fn get_secret(&self) -> typespec_client_core::http::Response<GetSecretResponse> {
    /// #    typespec_client_core::http::BufResponse::from_bytes(
    /// #      StatusCode::Ok,
    /// #      typespec_client_core::http::headers::Headers::new(),
    /// #      "{\"name\":\"database_password\",\"value\":\"hunter2\"}",
    /// #    ).into()
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
    /// assert_eq!(response.status(), StatusCode::Ok);
    /// let model = response.into_body().await.unwrap();
    /// assert_eq!(model.name, "database_password");
    /// assert_eq!(model.value, "hunter2");
    /// # }
    /// ```
    pub async fn into_body(self) -> crate::Result<T> {
        let body = self.raw.into_body();
        T::deserialize_with(body).await
    }
}

impl<T, F> fmt::Debug for Response<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.raw.status())
            // TODO: Sanitize headers and emit body as "(body)".
            .finish_non_exhaustive()
    }
}

impl<T, F> From<BufResponse> for Response<T, F> {
    fn from(raw: BufResponse) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T, F> From<Response<T, F>> for BufResponse {
    fn from(val: Response<T, F>) -> Self {
        val.raw
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
    use crate::http::{BufResponse, Response, StatusCode};

    #[tokio::test]
    async fn can_extract_raw_body() -> Result<(), Box<dyn std::error::Error>> {
        pub struct MyModel;

        {
            let response_t: Response<MyModel> =
                BufResponse::from_bytes(StatusCode::Ok, Headers::new(), b"Hello".as_slice()).into();
            let body = response_t.into_raw_body();
            assert_eq!(b"Hello", &*body.collect().await?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn can_convert_response_to_raw_response() -> Result<(), Box<dyn std::error::Error>> {
        pub struct MyModel;

        // Test converting a typed Response to BufResponse using Into
        let typed_response: Response<MyModel> =
            BufResponse::from_bytes(StatusCode::Ok, Headers::new(), b"Hello World".as_slice())
                .into();

        // Convert using Into trait
        let raw_response: BufResponse = typed_response.into();

        // Verify the raw response has the same data
        assert_eq!(raw_response.status(), StatusCode::Ok);
        let body = raw_response.into_body().collect().await?;
        assert_eq!(b"Hello World", &*body);

        Ok(())
    }

    mod json {
        use crate::http::headers::Headers;
        use crate::http::BufResponse;
        use crate::http::Response;
        use crate::http::StatusCode;
        use crate::Bytes;
        use serde::Deserialize;

        /// An example JSON-serialized response type.
        #[derive(Deserialize)]
        struct GetSecretResponse {
            name: String,
            value: String,
        }

        /// An example JSON-serialized list response type.
        #[derive(Deserialize)]
        struct GetSecretListResponse {
            value: Vec<GetSecretResponse>,
            #[serde(rename = "nextLink")]
            next_link: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse {
            code: Option<String>,
            message: Option<String>,
        }

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse> {
            BufResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"name":"my_secret","value":"my_value"}"#,
            )
            .into()
        }

        /// A sample service client function to return a list of secrets.
        fn list_secrets() -> Response<GetSecretListResponse> {
            BufResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"value":[{"name":"my_secret","value":"my_value"}],"nextLink":"?page=2"}"#,
            )
            .into()
        }

        #[tokio::test]
        async fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.into_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        async fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }

            let response = get_secret();
            let secret: MySecretResponse = response.into_raw_body().json().await.unwrap();
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
                BufResponse::from_bytes(status, headers, bytes).into();
            assert_eq!(response.status(), StatusCode::Ok);
            let model = response
                .into_body()
                .await
                .expect("deserialize GetSecretListResponse again");
            assert_eq!(model.next_link, Some("?page=2".to_string()));
        }

        #[tokio::test]
        async fn try_into_raw_response() {
            let stream = futures::stream::iter([
                Ok(Bytes::from_static(br#"{"code":"BadParameter","#)),
                Ok(Bytes::from_static(br#""message": "Bad parameter"}"#)),
            ]);
            let mut headers = Headers::new();
            headers.insert("x-ms-error", "BadParameter");
            let err = BufResponse::new(StatusCode::BadRequest, headers, Box::pin(stream));

            let err = err
                .try_into_raw_response()
                .await
                .expect("convert to RawResponse");
            assert_eq!(err.status(), StatusCode::BadRequest);
            assert!(err
                .headers()
                .iter()
                .any(|(k, v)| k.as_str() == "x-ms-error" && v.as_str() == "BadParameter"));

            let err: ErrorResponse = err.json().await.expect("convert to ErrorResponse");
            assert_eq!(err.code, Some("BadParameter".into()));
            assert_eq!(err.message, Some("Bad parameter".into()));
        }
    }

    #[cfg(feature = "xml")]
    mod xml {
        use crate::http::headers::Headers;
        use crate::http::BufResponse;
        use crate::http::Response;
        use crate::http::StatusCode;
        use crate::http::XmlFormat;
        use serde::Deserialize;

        /// An example XML-serialized response type.
        #[derive(Deserialize)]
        struct GetSecretResponse {
            name: String,
            value: String,
        }

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse, XmlFormat> {
            BufResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                "<GetSecretResponse><name>my_secret</name><value>my_value</value></GetSecretResponse>",
            ).into()
        }

        #[tokio::test]
        async fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.into_body().await.unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[tokio::test]
        async fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }

            let response: Response<GetSecretResponse, XmlFormat> = get_secret();
            let secret: MySecretResponse = response.into_raw_body().xml().await.unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }
    }
}
