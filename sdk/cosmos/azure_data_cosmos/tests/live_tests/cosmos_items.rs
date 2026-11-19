// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::framework::{self, test_client::TEST_MODE_ENV_VAR, TestClient, TestOptions};
use azure_core::http::StatusCode;
use azure_data_cosmos::{feed::FeedScope, models::ContainerProperties};
use futures::StreamExt;
use std::{error::Error, time::Duration};

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn begin_delete_by_partition_key() -> Result<(), Box<dyn Error>> {
    framework::resolve_connection_string()
        .ok_or("live tests require a valid AZURE_COSMOS_CONNECTION_STRING")?;
    assert!(!framework::targets_emulator(), "requires a live account");
    assert!(
        !std::env::var(TEST_MODE_ENV_VAR).is_ok_and(|mode| mode.eq_ignore_ascii_case("skipped")),
        "explicitly selected live tests cannot use AZURE_COSMOS_TEST_MODE=skipped"
    );
    TestClient::run_with_unique_db(
        async |run_context, database| {
            let container = run_context
                .create_container(
                    database,
                    ContainerProperties::new("PartitionDeletion", "/pk".into()),
                    None,
                )
                .await?;
            for (pk, id) in [
                ("target", "shared"),
                ("target", "second"),
                ("control", "shared"),
            ] {
                container
                    .create_item(pk, id, &serde_json::json!({"id": id, "pk": pk}), None)
                    .await?;
                run_context.read_item(&container, pk, id, None).await?;
            }
            let response = container
                .begin_delete_by_partition_key("target", None)
                .await?;
            assert!(response.status().is_success());
            assert_eq!(
                response.diagnostics().operation_name(),
                Some("begin_delete_by_partition_key")
            );
            tokio::time::timeout(Duration::from_secs(60), async {
                loop {
                    let mut hidden = true;
                    for id in ["shared", "second"] {
                        match container.read_item("target", id, None).await {
                            Ok(_) => hidden = false,
                            Err(error) if error.status().status_code() == StatusCode::NotFound => {}
                            Err(error) => return Err(error),
                        }
                    }
                    let mut pages = container
                        .query_items::<String>(
                            "SELECT VALUE c.id FROM c",
                            FeedScope::partition("target"),
                            None,
                        )
                        .await?
                        .into_pages();
                    while let Some(page) = pages.next().await {
                        hidden &= page?.into_items().is_empty();
                    }
                    if hidden {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Ok::<_, azure_data_cosmos::CosmosError>(())
            })
            .await??;
            let control: serde_json::Value = container
                .read_item("control", "shared", None)
                .await?
                .into_model()?;
            assert_eq!(control["pk"], "control");
            let mut pages = container
                .query_items::<String>(
                    "SELECT VALUE c.id FROM c",
                    FeedScope::full_container(),
                    None,
                )
                .await?
                .into_pages();
            let mut ids = Vec::new();
            while let Some(page) = pages.next().await {
                ids.extend(page?.into_items());
            }
            assert_eq!(ids, ["shared"]);
            Ok(())
        },
        Some(TestOptions::default().with_timeout(Duration::from_secs(180))),
    )
    .await
}
