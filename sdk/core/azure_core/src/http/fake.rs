// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for testing client libraries with fake servers.
//!
//! Enable the client library's `test` feature to use its `clients::fake` module.
//! Fake servers return an [`AsyncRawResponse`], allowing tests to control the status,
//! headers, and body. Use [`json_response()`] for models,
//! [`AsyncRawResponse::from_bytes()`] for bytes, or [`AsyncRawResponse::new()`] for streams.

use crate::{
    http::{
        headers::{Headers, CONTENT_TYPE},
        AsyncRawResponse, StatusCode,
    },
    json, Result,
};
use serde::Serialize;

/// Identifies the client operation being sent through the HTTP pipeline.
///
/// Client libraries insert the TypeSpec operation name into the request
/// [`Context`](crate::http::Context) when their `test` feature is enabled.
/// Fake servers use this value for routing independently of tracing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ApiName(pub String);

impl AsRef<str> for ApiName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Creates a fake response by serializing a model as JSON.
///
/// Preserves the supplied status and headers, adding `content-type: application/json`
/// only if the caller hasn't specified a content type.
///
/// Serialization uses the model's [`Serialize`] implementation. To include
/// response-only fields marked `skip_serializing`, use a JSON value or raw bytes instead.
///
/// # Errors
///
/// Returns an error if `model` cannot be serialized as JSON.
pub fn json_response<T: Serialize>(
    status: StatusCode,
    mut headers: Headers,
    model: &T,
) -> Result<AsyncRawResponse> {
    let body = json::to_json(model)?;
    if headers.get_optional_str(&CONTENT_TYPE).is_none() {
        headers.insert(CONTENT_TYPE, "application/json");
    }
    Ok(AsyncRawResponse::from_bytes(status, headers, body))
}

#[cfg(test)]
mod tests {
    use super::{json_response, ApiName};
    use crate::{
        http::{
            headers::{HeaderName, Headers, CONTENT_TYPE},
            AsyncRawResponse, Context, StatusCode,
        },
        json, Bytes, Result,
    };
    use futures::stream;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn api_name_in_context() {
        let name = ApiName("KeyVault.getSecret".into());
        let context = Context::new().with_value(name.clone());
        assert_eq!(context.value::<ApiName>(), Some(&name));
        assert_eq!(name.as_ref(), "KeyVault.getSecret");
        assert!(HashSet::from([name.clone()]).contains(&name));
    }

    #[tokio::test]
    async fn model_response() -> Result<()> {
        let mut headers = Headers::new();
        headers.insert("x-test", "example");
        let model = HashMap::from([("value", "fake secret")]);
        let response = json_response(StatusCode::Created, headers, &model)?
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
        let response = json_response(StatusCode::BadRequest, headers, &"example")?;
        assert_eq!(
            response.headers().get_str(&CONTENT_TYPE)?,
            "application/problem+json"
        );
        let invalid = HashMap::from([(vec![1, 2], "invalid JSON key")]);
        assert!(json_response(StatusCode::Ok, Headers::new(), &invalid).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn raw_and_streamed_responses() -> Result<()> {
        let response = AsyncRawResponse::from_bytes(
            StatusCode::NotFound,
            Headers::new(),
            Bytes::from_static(b"not JSON"),
        );
        assert_eq!(response.status(), StatusCode::NotFound);
        assert_eq!(response.into_body().collect().await?, "not JSON");

        let response = AsyncRawResponse::new(
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
}
