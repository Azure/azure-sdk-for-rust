// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{
    http::{InstrumentationOptions, StatusCode},
    Result,
};
use azure_core_test::{
    recorded,
    tracing::{ExpectedApiInformation, ExpectedInstrumentation},
    TestContext, TestMode,
};
use azure_security_keyvault_secrets::{
    models::{SecretClientGetSecretOptions, SetSecretParameters, UpdateSecretPropertiesParameters},
    ResourceExt as _, SecretClient, SecretClientOptions,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
async fn secret_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SetSecretParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    let secret = client
        .set_secret("secret-roundtrip", body.try_into()?, None)
        .await?
        .into_body()?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Get a specific version of a secret.
    let secret_version = secret.resource_id()?.version;
    let secret = client
        .get_secret(
            "secret-roundtrip",
            Some(SecretClientGetSecretOptions {
                secret_version,
                ..Default::default()
            }),
        )
        .await?
        .into_body()?;
    assert_eq!(secret.value, Some("secret-value".into()));

    Ok(())
}

#[recorded::test]
async fn update_secret_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SetSecretParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    let secret = client
        .set_secret("update-secret", body.try_into()?, None)
        .await?
        .into_body()?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Update secret properties.
    let properties = UpdateSecretPropertiesParameters {
        content_type: Some("text/plain".into()),
        secret_attributes: secret.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_secret".into(),
        )])),
    };
    let secret = client
        .update_secret_properties("update-secret", properties.try_into()?, None)
        .await?
        .into_body()?;
    assert_eq!(secret.content_type, Some("text/plain".into()));
    assert_eq!(
        secret.tags.expect("expected tags").get("test-name"),
        Some(&String::from("update_secret"))
    );

    Ok(())
}

#[recorded::test]
async fn list_secrets(ctx: TestContext) -> Result<()> {
    use azure_core::http::RequestContent;

    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create several secrets.
    let mut names = vec!["list-secrets-1", "list-secrets-2"];
    let secret1 = client
        .set_secret(
            names[0],
            RequestContent::from_str(r#"{"value":"secret-value-1"}"#),
            None,
        )
        .await?
        .into_body()?;
    assert_eq!(secret1.value, Some("secret-value-1".into()));

    let secret2 = client
        .set_secret(
            names[1],
            RequestContent::from_str(r#"{"value":"secret-value-2"}"#),
            None,
        )
        .await?
        .into_body()?;
    assert_eq!(secret2.value, Some("secret-value-2".into()));

    // List secrets.
    let mut pager = client.list_secret_properties(None)?.into_stream();
    while let Some(secret) = pager.try_next().await? {
        // Get the secret name from the ID.
        let name = secret.resource_id()?.name;
        if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
            names.remove(idx);
        }
    }
    assert!(names.is_empty());

    Ok(())
}

#[recorded::test]
async fn purge_secret(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a secret.
    let secret = client
        .set_secret(
            "purge-secret",
            SetSecretParameters {
                value: Some("secret-value".into()),
                ..Default::default()
            }
            .try_into()?,
            None,
        )
        .await?
        .into_body()?;

    // Delete the secret.
    let name = secret.resource_id()?.name;
    client.delete_secret(name.as_ref(), None).await?;

    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_secret(name.as_ref(), None).await {
            Ok(_) => {
                println!("{name} has been purged");
                break;
            }
            Err(err) if matches!(err.http_status(), Some(StatusCode::Conflict)) => {
                println!(
                    "Retrying in {} seconds",
                    retry.duration().unwrap_or_default().as_secs_f32()
                );
                if retry.next().await.is_none() {
                    return Err(err);
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

#[recorded::test]
async fn round_trip_secret_verify_telemetry(ctx: TestContext) -> Result<()> {
    use azure_core_test::tracing::ExpectedRestApiSpan;

    let recording = ctx.recording();

    // Verify that the distributed tracing traces generated from the API call below match the expected traces.
    azure_core_test::tracing::assert_instrumentation_information(
        |tracer_provider| {
            let mut options = SecretClientOptions::default();
            recording.instrument(&mut options.client_options);
            options.client_options.instrumentation = InstrumentationOptions {
                tracer_provider: Some(tracer_provider),
            };
            SecretClient::new(
                recording.var("AZURE_KEYVAULT_URL", None).as_str(),
                recording.credential(),
                Some(options),
            )
        },
        |client: SecretClient| {
            Box::pin(async move {
                // Set a secret.
                let body = SetSecretParameters {
                    value: Some("secret-value-instrument".into()),
                    ..Default::default()
                };
                let secret = client
                    .set_secret("secret-roundtrip-instrument", body.try_into()?, None)
                    .await?
                    .into_body()?;
                assert_eq!(secret.value, Some("secret-value-instrument".into()));

                // Get a specific version of a secret.
                let secret_version = secret.resource_id()?.version;
                let secret = client
                    .get_secret(
                        "secret-roundtrip-instrument",
                        Some(SecretClientGetSecretOptions {
                            secret_version,
                            ..Default::default()
                        }),
                    )
                    .await?
                    .into_body()?;
                assert_eq!(secret.value, Some("secret-value-instrument".into()));
                Ok(())
            })
        },
        ExpectedInstrumentation {
            package_name: recording.var("CARGO_PKG_NAME", None),
            package_version: env!("CARGO_PKG_VERSION").into(),
            package_namespace: Some("KeyVault"),
            api_calls: vec![
                ExpectedApiInformation {
                    api_name: Some("KeyVault.setSecret"),
                    api_children: vec![ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Put,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                ExpectedApiInformation {
                    api_name: Some("KeyVault.getSecret"),
                    ..Default::default()
                },
            ],
        },
    )
    .await?;

    Ok(())
}

#[recorded::test]
async fn list_secrets_verify_telemetry(ctx: TestContext) -> Result<()> {
    use azure_core_test::tracing::ExpectedRestApiSpan;

    const SECRET_COUNT: usize = 50;

    let recording = ctx.recording();

    {
        let secret_client = {
            let mut options = SecretClientOptions::default();
            recording.instrument(&mut options.client_options);
            SecretClient::new(
                recording.var("AZURE_KEYVAULT_URL", None).as_str(),
                recording.credential(),
                Some(options),
            )
        }?;
        for i in 1..=SECRET_COUNT {
            let secret = secret_client
                .set_secret(
                    &format!("secret-list-telemetry-{}", i),
                    SetSecretParameters {
                        value: Some(format!("secret-list-telemetry-value-{}", i)),
                        ..Default::default()
                    }
                    .try_into()?,
                    None,
                )
                .await?
                .into_body()?;
            assert_eq!(
                secret.value,
                Some(format!("secret-list-telemetry-value-{}", i))
            );
        }
    }
    // Verify that the distributed tracing traces generated from the API call below match the expected traces.
    let validate_result = azure_core_test::tracing::assert_instrumentation_information(
        |tracer_provider| {
            let mut options = SecretClientOptions::default();
            recording.instrument(&mut options.client_options);
            options.client_options.instrumentation = InstrumentationOptions {
                tracer_provider: Some(tracer_provider),
            };
            SecretClient::new(
                recording.var("AZURE_KEYVAULT_URL", None).as_str(),
                recording.credential(),
                Some(options),
            )
        },
        |client: SecretClient| {
            Box::pin(async move {
                let mut secrets = client.list_secret_properties(None)?;
                while let Some(secret) = secrets.try_next().await? {
                    let _ = secret.resource_id()?;
                }

                Ok(())
            })
        },
        ExpectedInstrumentation {
            package_name: recording.var("CARGO_PKG_NAME", None),
            package_version: recording.var("CARGO_PKG_VERSION", None),
            package_namespace: Some("KeyVault"),
            api_calls: vec![ExpectedApiInformation {
                api_name: Some("KeyVault.getSecrets"),
                api_children: vec![
                    ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Get,
                        ..Default::default()
                    },
                    ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Get,
                        ..Default::default()
                    },
                    ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Get,
                        ..Default::default()
                    },
                    ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Get,
                        ..Default::default()
                    },
                    ExpectedRestApiSpan {
                        api_verb: azure_core::http::Method::Get,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
        },
    )
    .await;

    validate_result
}

// Commented out for now until ItemIterator->PageIterator works for continuation tokens.

// #[recorded::test]
// async fn list_secrets_verify_telemetry_rehydrated(ctx: TestContext) -> Result<()> {
//     use azure_core_test::tracing::ExpectedRestApiSpan;

//     const SECRET_COUNT: usize = 50;

//     let recording = ctx.recording();

//     {
//         let secret_client = {
//             let mut options = SecretClientOptions::default();
//             recording.instrument(&mut options.client_options);
//             SecretClient::new(
//                 recording.var("AZURE_KEYVAULT_URL", None).as_str(),
//                 recording.credential(),
//                 Some(options),
//             )
//         }?;
//         for i in 1..=SECRET_COUNT {
//             let secret = secret_client
//                 .set_secret(
//                     &format!("secret-rehydrate-telemetry-{}", i),
//                     SetSecretParameters {
//                         value: Some(format!("secret-rehydrate-telemetry-value-{}", i)),
//                         ..Default::default()
//                     }
//                     .try_into()?,
//                     None,
//                 )
//                 .await?
//                 .into_body()?;
//             assert_eq!(
//                 secret.value,
//                 Some(format!("secret-rehydrate-telemetry-value-{}", i))
//             );
//         }
//     }
//     // Verify that the distributed tracing traces generated from the API call below match the expected traces.
//     let validate_result = azure_core_test::tracing::assert_instrumentation_information(
//         |tracer_provider| {
//             let mut options = SecretClientOptions::default();
//             recording.instrument(&mut options.client_options);
//             options.client_options.instrumentation = InstrumentationOptions {
//                 tracer_provider: Some(tracer_provider),
//             };
//             SecretClient::new(
//                 recording.var("AZURE_KEYVAULT_URL", None).as_str(),
//                 recording.credential(),
//                 Some(options),
//             )
//         },
//         |client: SecretClient| {
//             Box::pin(async move {
//                 let mut first_pager = client.list_secret_properties(None)?.into_pages();

//                 // Prime the iteration.
//                 let first_page = first_pager
//                     .try_next()
//                     .await?
//                     .expect("expected at least one page");
//                 {
//                     let secrets = first_page.into_body()?;
//                     for secret in secrets.value {
//                         let _ = secret.resource_id()?;
//                     }
//                 }

//                 let rehydration_token = first_pager
//                     .continuation_token()
//                     .expect("expected continuation token to be created after first page");

//                 let mut rehydrated_pager = client
//                     .list_secret_properties(None)?
//                     .into_pages()
//                     .with_continuation_token(rehydration_token);

//                 while let Some(secret_page) = rehydrated_pager.try_next().await? {
//                     let secrets = secret_page.into_body()?;
//                     for secret in secrets.value {
//                         let _ = secret.resource_id()?;
//                     }
//                 }

//                 Ok(())
//             })
//         },
//         ExpectedInstrumentation {
//             package_name: recording.var("CARGO_PKG_NAME", None),
//             package_version: recording.var("CARGO_PKG_VERSION", None),
//             package_namespace: Some("KeyVault"),
//             api_calls: vec![ExpectedApiInformation {
//                 api_name: Some("KeyVault.getSecrets"),
//                 api_children: vec![
//                     ExpectedRestApiSpan {
//                         api_verb: azure_core::http::Method::Get,
//                         ..Default::default()
//                     },
//                     ExpectedRestApiSpan {
//                         api_verb: azure_core::http::Method::Get,
//                         ..Default::default()
//                     },
//                     ExpectedRestApiSpan {
//                         api_verb: azure_core::http::Method::Get,
//                         ..Default::default()
//                     },
//                 ],
//                 ..Default::default()
//             }],
//         },
//     )
//     .await;

//     validate_result
// }
