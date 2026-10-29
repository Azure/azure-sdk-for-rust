// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for testing client libraries with fake servers.
//!
//! Enable the client library's `test` and `unstable_fake` features to use its
//! experimental `clients::fake` module.
//! Fake servers return an [`AsyncRawResponse`], allowing tests to control the status,
//! headers, and body. Import [`ResponseExt`] to create responses from models or
//! bytes, and [`AsyncResponseExt`] to create responses from streams.

use crate::{
    http::{
        headers::{Headers, CONTENT_TYPE},
        response::PinnedStream,
        AsyncRawResponse, AsyncResponse, Context, RawResponse, Response, StatusCode,
    },
    json, Bytes, Result,
};
use serde::Serialize;

/// Identifies the client operation being sent through the HTTP pipeline.
///
/// Client libraries insert the TypeSpec operation name into the request
/// [`Context`] when their `test` and `unstable_fake` features are enabled.
/// Fake servers use this value for routing independently of tracing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ApiName(&'static str);

impl From<&'static str> for ApiName {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ApiName {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// Adds an operation name to the request context for fake server routing.
///
/// Prefer [`operation_name!`](crate::http::operation_name) when writing client
/// operations so the call is a no-op unless fake support is enabled.
pub fn operation_name(name: &'static str, context: &mut Context<'_>) {
    context.insert(ApiName::from(name));
}

/// Extension methods for creating fake HTTP responses.
///
/// Implemented for [`Response`], [`RawResponse`], [`AsyncResponse`], and [`AsyncRawResponse`].
/// These methods preserve the supplied status without checking whether it indicates success.
pub trait ResponseExt: Sized {
    /// Creates a fake response from bytes without changing the status or headers.
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self;

    /// Creates a fake response by serializing a model as JSON.
    ///
    /// Adds `content-type: application/json` only if a content type isn't specified.
    /// Serialization uses the model's [`Serialize`] implementation, regardless of the
    /// response's format marker. For response-only fields marked `skip_serializing`,
    /// pass a JSON value or use [`fake_with_bytes()`](Self::fake_with_bytes) instead.
    ///
    /// # Errors
    ///
    /// Returns an error if `model` cannot be serialized as JSON.
    fn fake_with_model<T: Serialize>(
        status: StatusCode,
        mut headers: Headers,
        model: T,
    ) -> Result<Self> {
        let body = json::to_json(&model)?;
        if headers.get_optional_str(&CONTENT_TYPE).is_none() {
            headers.insert(CONTENT_TYPE, "application/json");
        }
        Ok(Self::fake_with_bytes(status, headers, body))
    }
}

impl ResponseExt for RawResponse {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self::from_bytes(status, headers, bytes)
    }
}

impl<T, F> ResponseExt for Response<T, F> {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        RawResponse::fake_with_bytes(status, headers, bytes).into()
    }
}

impl ResponseExt for AsyncRawResponse {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self::from_bytes(status, headers, bytes)
    }
}

impl<T> ResponseExt for AsyncResponse<T> {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        AsyncRawResponse::fake_with_bytes(status, headers, bytes).into()
    }
}

/// Extension methods for creating fake streaming HTTP responses.
pub trait AsyncResponseExt: ResponseExt {
    /// Creates a fake response from a stream without changing the status or headers.
    ///
    /// Stream errors are returned when the response body is read.
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self;
}

impl AsyncResponseExt for AsyncRawResponse {
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self::new(status, headers, stream)
    }
}

impl<T> AsyncResponseExt for AsyncResponse<T> {
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        AsyncRawResponse::fake_with_stream(status, headers, stream).into()
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiName, AsyncResponseExt as _, ResponseExt as _};
    use crate::{
        error::{check_success, ErrorKind, ErrorResponse},
        http::{
            headers::{HeaderName, Headers, CONTENT_TYPE, ERROR_CODE},
            pager::PagerOptions,
            AsyncRawResponse, AsyncResponse, ClientMethodOptions, Context, RawResponse, Response,
            StatusCode,
        },
        json, Bytes, Error, Result,
    };
    use futures::stream;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn api_name_in_context() {
        let name = ApiName::from("KeyVault.getSecret");
        let context = Context::new().with_value(name);
        assert_eq!(context.value::<ApiName>(), Some(&name));
        assert_eq!(name.as_ref(), "KeyVault.getSecret");
        assert!(HashSet::from([name]).contains(&name));
    }

    #[test]
    fn operation_name_supports_method_and_pager_options() {
        let mut options = ClientMethodOptions {
            context: Context::new().with_value(42u32),
        };
        crate::http::operation_name!("KeyVault.getSecret", &mut options);
        assert_eq!(
            options.context.value::<ApiName>(),
            Some(&ApiName::from("KeyVault.getSecret"))
        );
        assert_eq!(options.context.value::<u32>(), Some(&42));
        let mut options = PagerOptions::default();
        crate::http::operation_name!("KeyVault.getSecrets", &mut options);
        assert_eq!(
            options.context.value::<ApiName>(),
            Some(&ApiName::from("KeyVault.getSecrets"))
        );
    }

    #[tokio::test]
    async fn model_response() -> Result<()> {
        let mut headers = Headers::new();
        headers.insert("x-test", "example");
        let model = HashMap::from([("value", "fake secret")]);
        let response = AsyncRawResponse::fake_with_model(StatusCode::Created, headers, &model)?
            .try_into_raw_response()
            .await?;
        assert_eq!(response.status(), StatusCode::Created);
        assert_eq!(
            response.headers().get_str(&CONTENT_TYPE)?,
            "application/json"
        );
        assert_eq!(
            response
                .headers()
                .get_str(&HeaderName::from_static("x-test"))?,
            "example"
        );
        let actual: HashMap<String, String> = json::from_json(response.body())?;
        assert_eq!(actual.get("value").map(String::as_str), Some("fake secret"));
        Ok(())
    }

    #[test]
    fn preserves_content_type_and_serialization_errors() -> Result<()> {
        let mut headers = Headers::new();
        headers.insert(CONTENT_TYPE, "application/problem+json");
        let response = RawResponse::fake_with_model(StatusCode::BadRequest, headers, "example")?;
        assert_eq!(
            response.headers().get_str(&CONTENT_TYPE)?,
            "application/problem+json"
        );
        let invalid = HashMap::from([(vec![1, 2], "invalid JSON key")]);
        assert!(RawResponse::fake_with_model(StatusCode::Ok, Headers::new(), &invalid).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn raw_and_streamed_responses() -> Result<()> {
        let response = AsyncRawResponse::fake_with_bytes(
            StatusCode::NotFound,
            Headers::new(),
            Bytes::from_static(b"not JSON"),
        );
        assert_eq!(response.status(), StatusCode::NotFound);
        assert_eq!(response.into_body().collect().await?, "not JSON");

        let response = AsyncResponse::<()>::fake_with_stream(
            StatusCode::Ok,
            Headers::new(),
            Box::pin(stream::iter([
                Ok(Bytes::from_static(b"first ")),
                Ok(Bytes::from_static(b"second")),
            ])),
        );
        assert_eq!(response.into_body().collect().await?, "first second");
        Ok(())
    }

    #[tokio::test]
    async fn typed_responses() -> Result<()> {
        let model = HashMap::from([("value".to_owned(), "fake secret".to_owned())]);
        let response: Response<HashMap<String, String>> =
            Response::fake_with_model(StatusCode::Ok, Headers::new(), &model)?;
        assert_eq!(response.into_model()?, model);

        let response: AsyncResponse =
            AsyncResponse::fake_with_model(StatusCode::Ok, Headers::new(), &model)?;
        let actual: HashMap<String, String> =
            json::from_json(response.into_body().collect().await?)?;
        assert_eq!(actual, model);
        Ok(())
    }

    #[tokio::test]
    async fn azure_error_response() -> Result<()> {
        let message = "A \"secret\" was not found.\nTry another name.";
        let mut headers = Headers::new();
        headers.insert("x-test", "example");
        headers.insert(ERROR_CODE, "SecretNotFound");
        let response = AsyncRawResponse::fake_with_model(
            StatusCode::NotFound,
            headers,
            serde_json::json!({"error": {"code": "SecretNotFound", "message": message}}),
        )?;
        assert_eq!(response.headers().get_str(&ERROR_CODE)?, "SecretNotFound");
        assert_eq!(
            response
                .headers()
                .get_str(&HeaderName::from_static("x-test"))?,
            "example"
        );
        let err = check_success(response, None).await.unwrap_err();
        assert_eq!(err.http_status(), Some(StatusCode::NotFound));
        assert!(matches!(
            err.kind(),
            ErrorKind::HttpResponse { error_code: Some(code), .. } if code == "SecretNotFound"
        ));
        let detail = ErrorResponse::try_from(err)?.error.unwrap();
        assert_eq!(detail.message.as_deref(), Some(message));
        Ok(())
    }

    #[tokio::test]
    async fn stream_errors() {
        let response = AsyncRawResponse::fake_with_stream(
            StatusCode::Ok,
            Headers::new(),
            Box::pin(stream::once(async { Err(Error::from(ErrorKind::Io)) })),
        );
        let err = response.into_body().collect().await.unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::Io));
    }
}
