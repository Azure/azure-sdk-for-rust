// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP responses.

use crate::{
    error::ErrorKind,
    http::{headers::Headers, DeserializeWith, Format, JsonFormat, StatusCode},
    Bytes,
};
use futures::{Stream, StreamExt};
use std::{fmt, marker::PhantomData, pin::Pin, task::Poll};
use typespec::error::ResultExt as _;
pub use typespec::http::response::*;

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
    body: BufResponseBody,
}

impl BufResponse {
    /// Create a raw HTTP response from an asynchronous stream of bytes.
    pub fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body: BufResponseBody::new(stream),
        }
    }

    /// Create a raw HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: BufResponseBody::from_bytes(bytes),
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
    pub fn deconstruct(self) -> (StatusCode, Headers, BufResponseBody) {
        (self.status, self.headers, self.body)
    }

    /// Get the [`BufResponseBody`].
    pub fn into_body(self) -> BufResponseBody {
        self.body
    }

    /// Read the entire body and convert into a [`RawResponse`].
    pub async fn try_into_raw_response(self) -> crate::Result<RawResponse> {
        let body = self.body.collect().await?;
        Ok(RawResponse::from_bytes(self.status, self.headers, body))
    }

    /// Collect the stream into an internal [`Bytes`] collection.
    pub(crate) async fn buffer(self) -> crate::Result<Self> {
        Ok(Self {
            body: self.body.buffer().await?,
            ..self
        })
    }
}

/// A typed fully-buffered HTTP response.
///
/// The type parameter `T` is a marker type that indicates what the caller should expect to be able to deserialize the body into.
/// Service client methods should return a `Response<SomeModel>` where `SomeModel` is the service-specific response type.
/// For example, a service client method that returns a list of secrets should return `Response<ListSecretsResponse>`.
///
/// The type parameter `F` is a marker type that indicates the format of the data, defaulting to JSON.
/// XML is supported, and `NoFormat` indicates a binary body or no body expected e.g., for HTTP 204.
///
/// Given a `Response<T, F>`, a user can deserialize the body formatted as type `F` into the intended body type `T` by calling [`Response::into_body`].
/// However, because the type `T` is just a marker type, you can also access the raw [`ResponseBody`] using [`Response::into_raw_body`].
pub struct Response<T, F = JsonFormat> {
    raw: RawResponse,
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

    /// Get the [`ResponseBody`].
    pub fn into_raw_body(self) -> ResponseBody {
        self.raw.into_body()
    }

    /// Create a [`RawResponse`] by cloning borrowed data.
    pub fn to_raw_response(&self) -> RawResponse {
        self.raw.clone()
    }
}

impl<T: DeserializeWith<F>, F: Format> Response<T, F> {
    /// Fetches the entire body and tries to convert it into type `T`.
    ///
    /// This is the preferred method for parsing the body of a service response into it's default model type.
    ///
    /// # Examples
    ///
    /// ```
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
    /// #    typespec_client_core::http::RawResponse::from_bytes(
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
    /// let model = response.into_body().unwrap();
    /// assert_eq!(model.name, "database_password");
    /// assert_eq!(model.value, "hunter2");
    /// # }
    /// ```
    pub fn into_body(self) -> crate::Result<T> {
        let body = self.into_raw_body();
        T::deserialize_with(body)
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

impl<T, F> From<RawResponse> for Response<T, F> {
    fn from(raw: RawResponse) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T, F> From<Response<T, F>> for RawResponse {
    fn from(response: Response<T, F>) -> Self {
        response.raw
    }
}

/// A typed HTTP response that completes asynchronously outside the [`Pipeline`](crate::http::Pipeline).
///
/// The type parameter `T` is a marker type that identifies trait to deserialize defined headers;
/// otherwise, it is the unit type `()` if no headers are defined.
///
/// Given an `AsyncResponse<T>`, a user can access the raw [`BufResponseBody`] using [`AsyncResponse::into_body`].
pub struct AsyncResponse<T = ()> {
    raw: BufResponse,
    phantom: PhantomData<T>,
}

impl<T> AsyncResponse<T> {
    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.raw.status()
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        self.raw.headers()
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, BufResponseBody) {
        self.raw.deconstruct()
    }

    /// Get the [`BufResponseBody`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use typespec_client_core::http::response::AsyncResponse;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let response: AsyncResponse = unimplemented!();
    /// let body: Vec<u8> = response
    ///     .into_body()
    ///     .collect()
    ///     .await?
    ///     .to_vec();
    /// # Ok(()) }
    /// ```
    pub fn into_body(self) -> BufResponseBody {
        self.raw.into_body()
    }
}

impl<T> fmt::Debug for AsyncResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncResponse")
            .field("status", &self.raw.status())
            // TODO: Sanitize headers and emit body as "(body)".
            .finish_non_exhaustive()
    }
}

impl<T> From<BufResponse> for AsyncResponse<T> {
    fn from(raw: BufResponse) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> From<AsyncResponse<T>> for BufResponse {
    fn from(response: AsyncResponse<T>) -> Self {
        response.raw
    }
}

/// A response body stream.
///
/// This body can either be streamed or collected into [`Bytes`].
#[pin_project::pin_project]
pub struct BufResponseBody(#[pin] Body);

#[pin_project::pin_project(project = BodyProj)]
enum Body {
    Bytes(Option<Bytes>),
    Stream(#[pin] PinnedStream),
}

impl BufResponseBody {
    /// Create a new [`BufResponseBody`] from an async stream of bytes.
    fn new(stream: PinnedStream) -> Self {
        Self(Body::Stream(stream))
    }

    /// Create a new [`BufResponseBody`] from a byte slice.
    fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self(Body::Bytes(Some(bytes.into())))
    }

    /// Collect the stream into a [`Bytes`] collection.
    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut final_result = Vec::new();

        while let Some(res) = self.next().await {
            final_result.extend(&res?);
        }

        Ok(final_result.into())
    }

    /// Collect the stream into a [`String`].
    pub async fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.collect().await?)
            .with_context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }

    /// Collect the stream into an internal [`Bytes`] collection.
    async fn buffer(self) -> crate::Result<Self> {
        let bytes = self.collect().await?;
        Ok(Self::from_bytes(bytes))
    }
}

impl Stream for BufResponseBody {
    type Item = crate::Result<Bytes>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.0.project() {
            BodyProj::Bytes(b) => {
                if let Some(b) = b.take() {
                    return Poll::Ready(Some(Ok(b)));
                }

                Poll::Ready(None)
            }
            BodyProj::Stream(s) => s.poll_next(cx),
        }
    }
}

impl fmt::Debug for BufResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BufResponseBody")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, BufResponse, RawResponse, Response, StatusCode};
    use futures::stream;

    #[test]
    fn can_extract_raw_body() -> Result<(), Box<dyn std::error::Error>> {
        pub struct MyModel;

        let response_t: Response<MyModel> =
            RawResponse::from_bytes(StatusCode::Ok, Headers::new(), b"Hello".as_slice()).into();
        let body = response_t.into_raw_body();
        assert_eq!(b"Hello", &*body);

        Ok(())
    }

    #[test]
    fn can_convert_response_to_raw_response() {
        pub struct MyModel;

        // Test converting a typed Response to RawResponse using Into
        let typed_response: Response<MyModel> =
            RawResponse::from_bytes(StatusCode::Ok, Headers::new(), b"Hello World".as_slice())
                .into();

        // Convert using Into trait
        let raw_response: RawResponse = typed_response.into();

        // Verify the raw response has the same data
        assert_eq!(raw_response.status(), StatusCode::Ok);
        let body = raw_response.into_body();
        assert_eq!(b"Hello World", &*body);
    }

    #[tokio::test]
    async fn can_convert_buf_response_to_raw_response() {
        let buf_response =
            BufResponse::from_bytes(StatusCode::Ok, Headers::new(), b"Hello World".as_slice());
        let raw_response = buf_response
            .try_into_raw_response()
            .await
            .expect("expect RawResponse");

        // Verify the raw response has the same data
        assert_eq!(raw_response.status(), StatusCode::Ok);
        let body = raw_response.into_body();
        assert_eq!(b"Hello World", &*body);
    }

    #[tokio::test]
    async fn into_body_collects_all_bytes() {
        let response: AsyncResponse = BufResponse::new(
            StatusCode::Ok,
            Headers::new(),
            stream::iter(vec![
                Ok(Bytes::from_static(&[0xde, 0xad])),
                Ok(Bytes::from_static(&[0xbe, 0xef])),
            ])
            .boxed(),
        )
        .into();
        let buffer: Vec<u8> = response.into_body().collect().await.unwrap().to_vec();
        assert_eq!(buffer, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    mod json {
        use crate::{
            http::{headers::Headers, BufResponse, RawResponse, Response, StatusCode},
            Bytes,
        };
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
            RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"name":"my_secret","value":"my_value"}"#,
            )
            .into()
        }

        /// A sample service client function to return a list of secrets.
        fn list_secrets() -> Response<GetSecretListResponse> {
            RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"value":[{"name":"my_secret","value":"my_value"}],"nextLink":"?page=2"}"#,
            )
            .into()
        }

        #[test]
        fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.into_body().unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
        }

        #[test]
        fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
            }

            let response = get_secret();
            let secret: MySecretResponse = response.into_raw_body().json().unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
        }

        #[test]
        fn deserialize_pageable_from_body() {
            // We need to efficiently deserialize the body twice to get the "nextLink" but return it to the caller.
            let response = list_secrets();
            let (status, headers, body) = response.deconstruct();
            let model: GetSecretListResponse =
                crate::json::from_json(body.clone()).expect("deserialize GetSecretListResponse");
            assert_eq!(status, StatusCode::Ok);
            assert_eq!(model.value.len(), 1);
            assert_eq!(model.next_link, Some("?page=2".to_string()));

            let response: Response<GetSecretListResponse> =
                RawResponse::from_bytes(status, headers, body).into();
            assert_eq!(response.status(), StatusCode::Ok);
            let model = response
                .into_body()
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

            let err: ErrorResponse = err.into_body().json().expect("convert to ErrorResponse");
            assert_eq!(err.code, Some("BadParameter".into()));
            assert_eq!(err.message, Some("Bad parameter".into()));
        }
    }

    #[cfg(feature = "xml")]
    mod xml {
        use crate::http::{headers::Headers, RawResponse, Response, StatusCode, XmlFormat};
        use serde::Deserialize;

        /// An example XML-serialized response type.
        #[derive(Deserialize)]
        struct GetSecretResponse {
            name: String,
            value: String,
            whitespace: String,
        }

        /// A sample service client function.
        fn get_secret() -> Response<GetSecretResponse, XmlFormat> {
            RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                "<GetSecretResponse><name>my_secret</name><value>my_value</value><whitespace> foo </whitespace></GetSecretResponse>",
            ).into()
        }

        #[test]
        fn deserialize_default_type() {
            let response = get_secret();
            let secret = response.into_body().unwrap();
            assert_eq!(secret.name, "my_secret");
            assert_eq!(secret.value, "my_value");
            assert_eq!(secret.whitespace, " foo ");
        }

        #[test]
        fn deserialize_alternate_type() {
            #[derive(Deserialize)]
            struct MySecretResponse {
                #[serde(rename = "name")]
                yon_name: String,
                #[serde(rename = "value")]
                yon_value: String,
                #[serde(rename = "whitespace")]
                yon_whitespace: String,
            }

            let response: Response<GetSecretResponse, XmlFormat> = get_secret();
            let secret: MySecretResponse = response.into_raw_body().xml().unwrap();
            assert_eq!(secret.yon_name, "my_secret");
            assert_eq!(secret.yon_value, "my_value");
            assert_eq!(secret.yon_whitespace, " foo ");
        }
    }
}
