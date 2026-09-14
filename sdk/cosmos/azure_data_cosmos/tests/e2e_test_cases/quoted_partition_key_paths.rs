// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::FeedScope,
    models::{PartitionKeyDefinition, PartitionKeyVersion, PatchInstructions, PatchOperation},
    options::ChangeFeedStartFrom,
    PartitionKey, Query, TransactionalBatch,
};
use futures::{StreamExt, TryStreamExt};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::should_run,
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator or Azure Live"
)]
async fn quoted_paths_work_across_item_operations() -> TestResult {
    if !should_run("item.quoted-partition-key-paths").await? {
        return Ok(());
    }

    run_case(
        PartitionKeyDefinition::from(r#"/"first level' 1*()"/"le/vel2""#)
            .with_version(PartitionKeyVersion::V1),
        PartitionKey::from("v1"),
        serde_json::json!({"first level' 1*()": {"le/vel2": "v1"}}),
    )
    .await?;
    run_case(
        PartitionKeyDefinition::from(r#"/'first level" 1*()'/'le/vel2'"#),
        PartitionKey::from("v2"),
        serde_json::json!({"first level\" 1*()": {"le/vel2": "v2"}}),
    )
    .await?;
    run_case(
        PartitionKeyDefinition::from((
            r#"/"first level' 1*()"/"le/vel2""#,
            r#"/'second level" 1*()'/'le/vel2'"#,
        )),
        PartitionKey::from(("hpk-1", "hpk-2")),
        serde_json::json!({
            "first level' 1*()": {"le/vel2": "hpk-1"},
            "second level\" 1*()": {"le/vel2": "hpk-2"}
        }),
    )
    .await
}

async fn run_case(
    definition: PartitionKeyDefinition,
    partition_key: PartitionKey,
    key_properties: serde_json::Value,
) -> TestResult {
    E2eTestFixture::run_with_partition_key(definition, async |fixture| {
        let item_id = "quoted-item";
        let mut item = key_properties.clone();
        item["id"] = serde_json::json!(item_id);
        item["value"] = serde_json::json!(1);

        let created = fixture
            .container
            .create_item(partition_key.clone(), item_id, &item, None)
            .await?;
        assert_eq!(created.status(), StatusCode::Created);

        let read: serde_json::Value = fixture
            .container
            .read_item(partition_key.clone(), item_id, None)
            .await?
            .into_model()?;
        assert_eq!(read["value"], 1);

        item["value"] = serde_json::json!(2);
        let replaced = fixture
            .container
            .replace_item(partition_key.clone(), item_id, &item, None)
            .await?;
        assert_eq!(replaced.status(), StatusCode::Ok);

        item["value"] = serde_json::json!(3);
        let upserted = fixture
            .container
            .upsert_item(partition_key.clone(), item_id, &item, None)
            .await?;
        assert_eq!(upserted.status(), StatusCode::Ok);

        let patch =
            PatchInstructions::from(vec![PatchOperation::set("/value", serde_json::json!(4))]);
        let patched: serde_json::Value = fixture
            .container
            .patch_item(partition_key.clone(), item_id, patch, None)
            .await?
            .into_model()?;
        assert_eq!(patched["value"], 4);

        let query =
            Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", item_id)?;
        let queried: Vec<serde_json::Value> = fixture
            .container
            .query_items(query, FeedScope::partition(partition_key.clone()), None)
            .await?
            .try_collect()
            .await?;
        assert_eq!(queried.len(), 1);
        assert_eq!(queried[0]["id"], item_id);

        let mut changes = fixture
            .container
            .query_change_feed::<serde_json::Value>(
                FeedScope::partition(partition_key.clone()),
                ChangeFeedStartFrom::Beginning,
                None,
            )
            .await?;
        let change_page = changes
            .next()
            .await
            .expect("created item must produce a change-feed page")?;
        assert!(change_page
            .items()
            .iter()
            .any(|change| change.current().is_some_and(|value| value["id"] == item_id)));

        let mut batch_item = key_properties;
        batch_item["id"] = serde_json::json!("quoted-batch");
        batch_item["value"] = serde_json::json!(5);
        let batch = TransactionalBatch::new(partition_key.clone())
            .create_item(&batch_item)?
            .read_item("quoted-batch", None);
        let batch = fixture
            .container
            .execute_transactional_batch(batch, None)
            .await?
            .into_model()?;
        assert_eq!(
            batch
                .results()
                .iter()
                .map(|result| result.status_code())
                .collect::<Vec<_>>(),
            [201, 200]
        );

        let deleted = fixture
            .container
            .delete_item(partition_key.clone(), item_id, None)
            .await?;
        assert_eq!(deleted.status(), StatusCode::NoContent);
        let deleted = fixture
            .container
            .delete_item(partition_key, "quoted-batch", None)
            .await?;
        assert_eq!(deleted.status(), StatusCode::NoContent);
        Ok(())
    })
    .await
}
