// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{Response, Url},
    Bytes, Result,
};
use azure_core_test::{TestContext, SANITIZE_BODY_NAME};
use azure_resourcemanager_keyvault::{
    models::{CreatedByType, SystemData},
    KeyVaultClient, KeyVaultClientOptions,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

pub async fn client(ctx: &TestContext) -> Result<KeyVaultClient> {
    let recording = ctx.recording();
    // The recursive name sanitizer also replaces SKU enum values.
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;
    let mut options = KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);
    let client = KeyVaultClient::new(
        recording.var("KEYVAULT_SUBSCRIPTION_ID", None),
        recording.credential(),
        Some(options),
    )?;
    let endpoint: Url = recording
        .var("KEYVAULT_RESOURCE_MANAGER_URL", None)
        .parse()?;
    assert_eq!(client.endpoint(), &endpoint);
    Ok(client)
}

pub fn model_and_json<T: DeserializeOwned>(response: Response<T>) -> Result<(T, Value)> {
    let bytes = Bytes::from(response.to_raw_response().into_body());
    let value = serde_json::from_slice(&bytes)?;
    Ok((response.into_model()?, value))
}

pub fn assert_system_data(data: Option<&SystemData>, raw: &Value) {
    if let Some(data) = data {
        for (actual, field) in [
            (&data.created_by_type, "createdByType"),
            (&data.last_modified_by_type, "lastModifiedByType"),
        ] {
            if let Some(actual) = actual {
                assert!(matches!(
                    actual,
                    CreatedByType::Application
                        | CreatedByType::Key
                        | CreatedByType::ManagedIdentity
                        | CreatedByType::User
                ));
                assert_eq!(raw["systemData"][field].as_str(), Some(actual.as_ref()));
            }
        }
    }
}

pub fn assert_enum_values<E: AsRef<str> + Debug + PartialEq>(
    actual: Option<&[E]>,
    expected: Option<&[E]>,
    raw: &Value,
) {
    if let Some(expected) = expected {
        let actual = actual.unwrap();
        let raw = raw.as_array().unwrap();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(raw.len(), expected.len());
        for value in expected {
            assert!(actual.contains(value), "missing named variant {value:?}");
            assert!(raw.iter().any(|raw| raw.as_str() == Some(value.as_ref())));
        }
    } else {
        assert!(actual.is_none_or(<[E]>::is_empty));
        assert!(raw.is_null() || raw.as_array().is_some_and(Vec::is_empty));
    }
}
