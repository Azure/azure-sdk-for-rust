// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    models::{
        AppConfigurationClientGetKeyValueOptions, AppConfigurationClientListKeyValuesOptions,
        AppConfigurationClientPutKeyValueOptions, KeyValue, PutKeyValueRequestContentType,
    },
    AppConfigurationClient, AppConfigurationClientOptions,
};
use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{
        headers::{Headers, AUTHORIZATION, CONTENT_TYPE},
        AsyncRawResponse, ClientOptions, HttpClient, Method, Request, StatusCode, Transport,
    },
    time::{Duration, OffsetDateTime},
    Bytes, Result,
};
use futures::TryStreamExt;
use std::sync::Arc;

#[derive(Debug)]
struct Credential;

#[async_trait]
impl TokenCredential for Credential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _: Option<TokenRequestOptions<'_>>,
    ) -> Result<AccessToken> {
        assert_eq!(scopes, ["https://azconfig.io/.default"]);
        Ok(AccessToken::new(
            "test-token",
            OffsetDateTime::now_utc() + Duration::hours(1),
        ))
    }
}

#[derive(Debug)]
struct MockTransport(fn(&Request) -> Result<AsyncRawResponse>);

#[async_trait]
impl HttpClient for MockTransport {
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse> {
        assert_eq!(
            request.headers().get_str(&AUTHORIZATION)?,
            "Bearer test-token"
        );
        (self.0)(request)
    }
}

fn client(handler: fn(&Request) -> Result<AsyncRawResponse>) -> Result<AppConfigurationClient> {
    AppConfigurationClient::new(
        "https://example.azconfig.io",
        Arc::new(Credential),
        Some(AppConfigurationClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(Arc::new(MockTransport(handler)))),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
}

fn response(body: &'static [u8]) -> AsyncRawResponse {
    AsyncRawResponse::from_bytes(StatusCode::Ok, Headers::new(), Bytes::from_static(body))
}

#[test]
fn rejects_invalid_endpoints() {
    for endpoint in ["not a url", "ftp://example.azconfig.io"] {
        assert!(AppConfigurationClient::new(endpoint, Arc::new(Credential), None).is_err());
    }
}

#[test]
fn key_value_serialization() -> Result<()> {
    let setting = KeyValue {
        key: Some("read-only-key".into()),
        value: Some("hello".into()),
        content_type: Some("text/plain".into()),
        ..Default::default()
    };
    let json = serde_json::to_value(setting)?;
    assert_eq!(json["value"], "hello");
    assert_eq!(json["content_type"], "text/plain");
    assert!(json.get("key").is_none());
    Ok(())
}

#[tokio::test]
async fn get_key_value_sends_options_and_authentication() -> Result<()> {
    let client = client(|request| {
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.url().path(), "/kv/app:message");
        let query = request.url().query_pairs().collect::<Vec<_>>();
        assert!(query
            .iter()
            .any(|(key, value)| key == "api-version" && value == "2026-04-01"));
        assert!(query
            .iter()
            .any(|(key, value)| key == "label" && value == "development"));
        assert_eq!(request.headers().get_str(&"if-match".into())?, "\"etag\"");
        assert_eq!(
            request.headers().get_str(&"sync-token".into())?,
            "token=abc;sn=1"
        );
        Ok(response(
            br#"{"key":"app:message","label":"development","value":"hello"}"#,
        ))
    })?;
    let setting = client
        .get_key_value(
            "app:message",
            Some(AppConfigurationClientGetKeyValueOptions {
                label: Some("development".into()),
                if_match: Some("\"etag\"".into()),
                sync_token: Some("token=abc;sn=1".into()),
                ..Default::default()
            }),
        )
        .await?
        .into_model()?;
    assert_eq!(setting.key.as_deref(), Some("app:message"));
    assert_eq!(setting.value.as_deref(), Some("hello"));
    Ok(())
}

#[tokio::test]
async fn put_key_value_sends_content_type_and_conditions() -> Result<()> {
    let client = client(|request| {
        assert_eq!(request.method(), Method::Put);
        assert_eq!(request.url().path(), "/kv/app:message");
        assert_eq!(
            request.headers().get_str(&CONTENT_TYPE)?,
            "application/vnd.microsoft.appconfig.kv+json"
        );
        assert_eq!(request.headers().get_str(&"if-none-match".into())?, "*");
        Ok(response(br#"{"key":"app:message","value":"hello"}"#))
    })?;
    let setting = KeyValue {
        value: Some("hello".into()),
        ..Default::default()
    };
    let setting = client
        .put_key_value(
            PutKeyValueRequestContentType::ApplicationVndMicrosoftAppconfigKvJson,
            "app:message",
            Some(AppConfigurationClientPutKeyValueOptions {
                entity: Some(setting.try_into()?),
                if_none_match: Some("*".into()),
                ..Default::default()
            }),
        )
        .await?
        .into_model()?;
    assert_eq!(setting.value.as_deref(), Some("hello"));
    Ok(())
}

#[tokio::test]
async fn list_key_values_follows_relative_continuation_links() -> Result<()> {
    let client = client(|request| {
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.url().path(), "/kv");
        let query = request.url().query_pairs().collect::<Vec<_>>();
        assert!(query
            .iter()
            .any(|(key, value)| key == "api-version" && value == "2026-04-01"));
        assert!(query
            .iter()
            .any(|(key, value)| key == "key" && value == "app:*"));
        if query
            .iter()
            .any(|(key, value)| key == "After" && value == "next")
        {
            Ok(response(
                br#"{"items":[{"key":"app:second","value":"two"}]}"#,
            ))
        } else {
            Ok(response(br#"{"items":[{"key":"app:first","value":"one"}],"@nextLink":"/kv?key=app%3A*&After=next"}"#))
        }
    })?;
    let mut pages = client
        .list_key_values(Some(AppConfigurationClientListKeyValuesOptions {
            key: Some("app:*".into()),
            ..Default::default()
        }))?
        .into_pages();
    let first = pages.try_next().await?.expect("first page").into_model()?;
    assert_eq!(first.items[0].key.as_deref(), Some("app:first"));
    let second = pages.try_next().await?.expect("second page").into_model()?;
    assert_eq!(second.items[0].key.as_deref(), Some("app:second"));
    assert!(pages.try_next().await?.is_none());
    Ok(())
}

#[tokio::test]
async fn rejects_empty_keys_before_sending() -> Result<()> {
    let client = client(|_| panic!("empty keys must not send requests"))?;
    assert!(client.get_key_value("", None).await.is_err());
    assert!(client.delete_key_value("", None).await.is_err());
    assert!(client
        .put_key_value(
            PutKeyValueRequestContentType::ApplicationVndMicrosoftAppconfigKvJson,
            "",
            None,
        )
        .await
        .is_err());
    Ok(())
}
