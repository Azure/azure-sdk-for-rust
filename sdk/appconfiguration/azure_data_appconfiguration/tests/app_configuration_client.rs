// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::StatusCode, Result};
use azure_core_test::{recorded, TestContext, SANITIZE_BODY_NAME};
use azure_data_appconfiguration::{
    models::{
        ConfigurationClientDeleteConfigurationSettingOptions,
        ConfigurationClientGetConfigurationSettingOptions,
        ConfigurationClientListConfigurationSettingsOptions, ConfigurationClientListKeysOptions,
        ConfigurationClientListLabelsOptions, ConfigurationClientListRevisionsOptions,
        ConfigurationClientSetConfigurationSettingOptions, ConfigurationSetting,
        SetConfigurationSettingRequestContentType,
    },
    ConfigurationClient, ConfigurationClientOptions,
};
use futures::TryStreamExt;
use std::collections::HashMap;

async fn client(ctx: &TestContext) -> Result<ConfigurationClient> {
    let recording = ctx.recording();
    // AZSDK3447 sanitizes $.key, which is a synthetic setting name, not a secret.
    recording
        .remove_sanitizers(&[SANITIZE_BODY_NAME, "AZSDK3447"])
        .await?;
    let mut options = ConfigurationClientOptions::default();
    recording.instrument(&mut options.client_options);
    ConfigurationClient::new(
        recording.var("AZURE_APPCONFIG_ENDPOINT", None).as_str(),
        recording.credential(),
        Some(options),
    )
}

async fn set_setting(
    client: &ConfigurationClient,
    key: &str,
    value: &str,
    options: Option<ConfigurationClientSetConfigurationSettingOptions<'_>>,
) -> Result<ConfigurationSetting> {
    let mut options = options.unwrap_or_default();
    options.entity = Some(
        ConfigurationSetting {
            value: Some(value.into()),
            content_type: Some("text/plain".into()),
            tags: Some(HashMap::from([("language".into(), "rust".into())])),
            ..Default::default()
        }
        .try_into()?,
    );
    client
        .set_configuration_setting(
            SetConfigurationSettingRequestContentType::ApplicationVndMicrosoftAppconfigKvJson,
            key,
            Some(options),
        )
        .await?
        .into_model()
}

#[recorded::test]
async fn key_value_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let key = recording.random_string::<24>(Some("rust-roundtrip-"));
    let label = recording.random_string::<24>(Some("rust-label-"));

    let result: Result<_> = async {
        let created = set_setting(
            &client,
            &key,
            "first",
            Some(ConfigurationClientSetConfigurationSettingOptions {
                label: Some(label.clone()),
                if_none_match: Some("*".into()),
                ..Default::default()
            }),
        )
        .await?;
        let fetched = client
            .get_configuration_setting(
                &key,
                Some(ConfigurationClientGetConfigurationSettingOptions {
                    label: Some(label.clone()),
                    ..Default::default()
                }),
            )
            .await?
            .into_model()?;
        let updated = set_setting(
            &client,
            &key,
            "second",
            Some(ConfigurationClientSetConfigurationSettingOptions {
                label: Some(label.clone()),
                if_match: Some(format!(
                    "\"{}\"",
                    fetched.etag.as_ref().expect("setting ETag")
                )),
                ..Default::default()
            }),
        )
        .await?;
        Ok((created, fetched, updated))
    }
    .await;

    let deleted = client
        .delete_configuration_setting(
            &key,
            Some(ConfigurationClientDeleteConfigurationSettingOptions {
                label: Some(label.clone()),
                ..Default::default()
            }),
        )
        .await;
    let (created, fetched, updated) = result?;
    let deleted = deleted?.into_model()?;
    assert_eq!(created.key.as_deref(), Some(key.as_str()));
    assert_eq!(fetched.value.as_deref(), Some("first"));
    assert_eq!(fetched.label.as_deref(), Some(label.as_str()));
    assert_eq!(fetched.content_type.as_deref(), Some("text/plain"));
    assert_eq!(
        fetched.tags.as_ref().and_then(|tags| tags.get("language")),
        Some(&"rust".to_string())
    );
    assert_eq!(updated.value.as_deref(), Some("second"));
    assert_ne!(updated.etag, fetched.etag);
    assert_eq!(deleted.value.as_deref(), Some("second"));
    let missing = client
        .get_configuration_setting(
            &key,
            Some(ConfigurationClientGetConfigurationSettingOptions {
                label: Some(label),
                ..Default::default()
            }),
        )
        .await
        .expect_err("the deleted key-value must not exist");
    assert_eq!(missing.http_status(), Some(StatusCode::NotFound));
    Ok(())
}

#[recorded::test]
async fn conditional_writes(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let key = recording.random_string::<24>(Some("rust-etag-"));

    let result: Result<_> = async {
        let original = set_setting(&client, &key, "original", None).await?;
        let etag = format!("\"{}\"", original.etag.as_ref().expect("setting ETag"));
        let duplicate = set_setting(
            &client,
            &key,
            "duplicate",
            Some(ConfigurationClientSetConfigurationSettingOptions {
                if_none_match: Some("*".into()),
                ..Default::default()
            }),
        )
        .await;
        let updated = set_setting(
            &client,
            &key,
            "updated",
            Some(ConfigurationClientSetConfigurationSettingOptions {
                if_match: Some(etag.clone()),
                ..Default::default()
            }),
        )
        .await?;
        let stale = set_setting(
            &client,
            &key,
            "stale",
            Some(ConfigurationClientSetConfigurationSettingOptions {
                if_match: Some(etag),
                ..Default::default()
            }),
        )
        .await;
        let fetched = client
            .get_configuration_setting(&key, None)
            .await?
            .into_model()?;
        Ok((duplicate, updated, stale, fetched))
    }
    .await;

    let cleanup = client.delete_configuration_setting(&key, None).await;
    let (duplicate, updated, stale, fetched) = result?;
    cleanup?;
    assert_eq!(
        duplicate
            .expect_err("create-only writes must fail for existing settings")
            .http_status(),
        Some(StatusCode::PreconditionFailed)
    );
    assert_eq!(
        stale
            .expect_err("writes with stale ETags must fail")
            .http_status(),
        Some(StatusCode::PreconditionFailed)
    );
    assert_eq!(updated.value.as_deref(), Some("updated"));
    assert_eq!(fetched.value.as_deref(), Some("updated"));
    Ok(())
}

#[recorded::test]
async fn lock_and_unlock_key_value(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let key = recording.random_string::<24>(Some("rust-lock-"));

    let result: Result<_> = async {
        set_setting(&client, &key, "original", None).await?;
        let locked = client
            .create_read_only_lock(&key, None)
            .await?
            .into_model()?;
        let rejected = set_setting(&client, &key, "rejected", None).await;
        Ok((locked, rejected))
    }
    .await;

    let unlocked = client.delete_read_only_lock(&key, None).await;
    let updated = set_setting(&client, &key, "updated", None).await;
    let cleanup = client.delete_configuration_setting(&key, None).await;
    let (locked, rejected) = result?;
    let unlocked = unlocked?.into_model()?;
    let updated = updated?;
    cleanup?;
    assert_eq!(locked.is_read_only, Some(true));
    assert_eq!(
        rejected
            .expect_err("locked settings must reject writes")
            .http_status(),
        Some(StatusCode::Conflict)
    );
    assert_eq!(unlocked.is_read_only, Some(false));
    assert_eq!(updated.value.as_deref(), Some("updated"));
    Ok(())
}

#[recorded::test]
async fn list_key_values_keys_and_labels(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let prefix = recording.random_string::<24>(Some("rust-list-"));
    let keys = [format!("{prefix}-one"), format!("{prefix}-two")];
    let label = recording.random_string::<24>(Some("rust-label-"));

    let result: Result<_> = async {
        for key in &keys {
            set_setting(
                &client,
                key,
                key,
                Some(ConfigurationClientSetConfigurationSettingOptions {
                    label: Some(label.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        }
        let settings = client
            .list_configuration_settings(Some(
                ConfigurationClientListConfigurationSettingsOptions {
                    key: Some(format!("{prefix}*")),
                    label: Some(label.clone()),
                    ..Default::default()
                },
            ))?
            .try_collect::<Vec<_>>()
            .await?;
        let listed_keys = client
            .list_keys(Some(ConfigurationClientListKeysOptions {
                name: Some(format!("{prefix}*")),
                ..Default::default()
            }))?
            .try_collect::<Vec<_>>()
            .await?;
        let labels = client
            .list_labels(Some(ConfigurationClientListLabelsOptions {
                name: Some(label.clone()),
                ..Default::default()
            }))?
            .try_collect::<Vec<_>>()
            .await?;
        Ok((settings, listed_keys, labels))
    }
    .await;

    let mut cleanup_results = Vec::new();
    for key in &keys {
        cleanup_results.push(
            client
                .delete_configuration_setting(
                    key,
                    Some(ConfigurationClientDeleteConfigurationSettingOptions {
                        label: Some(label.clone()),
                        ..Default::default()
                    }),
                )
                .await,
        );
    }
    let (settings, listed_keys, labels) = result?;
    for cleanup in cleanup_results {
        cleanup?;
    }
    assert_eq!(settings.len(), 2);
    assert_eq!(listed_keys.len(), 2);
    for key in &keys {
        assert!(settings.iter().any(|setting| {
            setting.key.as_ref() == Some(key)
                && setting.value.as_ref() == Some(key)
                && setting.label.as_ref() == Some(&label)
        }));
        assert!(listed_keys
            .iter()
            .any(|listed| listed.name.as_ref() == Some(key)));
    }
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].name.as_ref(), Some(&label));
    Ok(())
}

#[recorded::test]
async fn list_revisions(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let key = recording.random_string::<24>(Some("rust-revision-"));

    let result: Result<_> = async {
        set_setting(&client, &key, "first", None).await?;
        set_setting(&client, &key, "second", None).await?;
        client
            .list_revisions(Some(ConfigurationClientListRevisionsOptions {
                key: Some(key.clone()),
                ..Default::default()
            }))?
            .try_collect::<Vec<_>>()
            .await
    }
    .await;

    let cleanup = client.delete_configuration_setting(&key, None).await;
    let revisions = result?;
    cleanup?;
    assert_eq!(revisions.len(), 2);
    for value in ["first", "second"] {
        assert!(revisions.iter().any(|revision| {
            revision.key.as_ref() == Some(&key) && revision.value.as_deref() == Some(value)
        }));
    }
    Ok(())
}
