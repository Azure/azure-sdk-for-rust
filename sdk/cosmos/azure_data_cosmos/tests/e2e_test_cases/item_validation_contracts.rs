// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{ http::StatusCode, Uuid };
use azure_data_cosmos::{
    models::{ ContainerProperties, PatchInstructions, PatchOperation, UniqueKey, UniqueKeyPolicy },
    TransactionalBatch,
};

use crate::e2e_test_cases::{
    fixture::{ E2eTestFixture, TestResult },
    support::{ item, should_run, Item },
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn invalid_writes_preserve_state() -> TestResult {
    if !should_run("item.validation-contracts").await? {
        return Ok(());
    }

    validate_identity_partition_key_and_size().await?;
    validate_unique_key_scope().await
}

async fn validate_identity_partition_key_and_size() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let missing_id = serde_json::json!({ "pk": "A", "value": 1 });
        assert_status(
            "missing body id",
            fixture.container.create_item("A", "missing-id", missing_id, None).await,
            StatusCode::BadRequest
        );
        assert_missing(&fixture.container, "A", "missing-id").await?;

        let wrong_partition = serde_json::json!({ "id": "wrong-pk", "pk": "B", "value": 2 });
        assert_status(
            "partition key mismatch",
            fixture.container.create_item("A", "wrong-pk", wrong_partition, None).await,
            StatusCode::BadRequest
        );
        assert_missing(&fixture.container, "A", "wrong-pk").await?;
        assert_missing(&fixture.container, "B", "wrong-pk").await?;

        let original = item("stable", "A", 3);
        fixture.container.create_item("A", &original.id, &original, None).await?;
        let mismatched_id = item("different", "A", 4);
        let response = fixture.container.replace_item(
            "A",
            &original.id,
            &mismatched_id,
            None
        ).await?;
        assert_eq!(response.status(), StatusCode::Ok);
        assert_missing(&fixture.container, "A", &original.id).await?;
        let stored: Item = fixture.container
            .read_item("A", &mismatched_id.id, None).await?
            .into_model()?;
        assert_eq!(stored, mismatched_id);

        let original = stored;

        let oversized =
            serde_json::json!({
            "id": "oversized",
            "pk": "A",
            "value": "x".repeat(2 * 1024 * 1024),
        });
        assert_status(
            "oversized item",
            fixture.container.create_item("A", "oversized", oversized, None).await,
            StatusCode::PayloadTooLarge
        );
        assert_missing(&fixture.container, "A", "oversized").await?;

        let oversized_patch = PatchInstructions::from(
            vec![PatchOperation::set("/large", serde_json::json!("x".repeat(2 * 1024 * 1024)))]
        );
        let before_patch: serde_json::Value = fixture.container
            .read_item("A", &original.id, None).await?
            .into_model()?;
        assert_status(
            "oversized patch",
            fixture.container.patch_item("A", &original.id, oversized_patch, None).await,
            StatusCode::PayloadTooLarge
        );
        let stored_after_patch: serde_json::Value = fixture.container
            .read_item("A", &original.id, None).await?
            .into_model()?;
        assert_eq!(stored_after_patch, before_patch);
        assert!(stored_after_patch.get("large").is_none());
        Ok(())
    }).await
}

async fn validate_unique_key_scope() -> TestResult {
    let container_id = format!("unique-{}", Uuid::new_v4());
    let properties = ContainerProperties::new(
        container_id.clone(),
        "/pk".into()
    ).with_unique_key_policy(
        UniqueKeyPolicy::default().with_unique_key(UniqueKey::default().with_path("/email"))
    );
    E2eTestFixture::run_with_container_properties(properties, async |fixture| {
        let persisted = fixture.container.read(None).await?.into_model()?;
        assert_eq!(
            persisted.unique_key_policy
                .as_ref()
                .expect("unique key policy must round trip").unique_keys[0].paths,
            ["/email"]
        );

        assert_status(
            "immutable unique key policy",
            fixture.container.replace(
                ContainerProperties::new(container_id.clone(), "/pk".into()),
                None
            ).await,
            StatusCode::Forbidden
        );

        let first = serde_json::json!({ "id": "first", "pk": "A", "email": "same@example.test" });
        fixture.container.create_item("A", "first", first, None).await?;
        let duplicate =
            serde_json::json!({ "id": "duplicate", "pk": "A", "email": "same@example.test" });
        assert_status(
            "duplicate unique key",
            fixture.container.create_item("A", "duplicate", duplicate, None).await,
            StatusCode::Conflict
        );
        assert_missing(&fixture.container, "A", "duplicate").await?;
        let original: serde_json::Value = fixture.container
            .read_item("A", "first", None).await?
            .into_model()?;
        assert_eq!(original["email"], "same@example.test");

        let numeric = serde_json::json!({ "id": "numeric", "pk": "A", "email": 1 });
        fixture.container.create_item("A", "numeric", numeric, None).await?;
        let equivalent_numeric =
            serde_json::json!({ "id": "equivalent-numeric", "pk": "A", "email": 1.0 });
        assert_status(
            "numeric unique key equivalence",
            fixture.container.create_item(
                "A",
                "equivalent-numeric",
                equivalent_numeric,
                None
            ).await,
            StatusCode::Conflict
        );
        assert_missing(&fixture.container, "A", "equivalent-numeric").await?;

        let distinct =
            serde_json::json!({ "id": "distinct", "pk": "A", "email": "other@example.test" });
        fixture.container.create_item("A", "distinct", &distinct, None).await?;
        let duplicate_patch = PatchInstructions::from(
            vec![PatchOperation::set("/email", serde_json::json!("same@example.test"))]
        );
        assert_status(
            "patch duplicate unique key",
            fixture.container.patch_item("A", "distinct", duplicate_patch, None).await,
            StatusCode::Conflict
        );
        let distinct_after_patch: serde_json::Value = fixture.container
            .read_item("A", "distinct", None).await?
            .into_model()?;
        assert_eq!(distinct_after_patch["email"], "other@example.test");

        let duplicate_direct =
            serde_json::json!({ "id": "distinct", "pk": "A", "email": "same@example.test" });
        assert_status(
            "replace duplicate unique key",
            fixture.container.replace_item("A", "distinct", &duplicate_direct, None).await,
            StatusCode::Conflict
        );
        assert_status(
            "upsert duplicate unique key",
            fixture.container.upsert_item("A", "distinct", &duplicate_direct, None).await,
            StatusCode::Conflict
        );
        let distinct_after_direct_writes: serde_json::Value = fixture.container
            .read_item("A", "distinct", None).await?
            .into_model()?;
        assert_eq!(distinct_after_direct_writes["email"], "other@example.test");

        let duplicate_replace =
            serde_json::json!({ "id": "distinct", "pk": "A", "email": "same@example.test" });
        let batch = TransactionalBatch::new("A").replace_item("distinct", duplicate_replace, None)?;
        let response = fixture.container.execute_transactional_batch(batch, None).await?;
        assert_eq!(response.status(), StatusCode::MultiStatus);
        assert_eq!(response.into_model()?.results()[0].status_code(), 409);
        let distinct_after_batch: serde_json::Value = fixture.container
            .read_item("A", "distinct", None).await?
            .into_model()?;
        assert_eq!(distinct_after_batch["email"], "other@example.test");

        let other_partition =
            serde_json::json!({ "id": "other", "pk": "B", "email": "same@example.test" });
        let response = fixture.container.create_item("B", "other", other_partition, None).await?;
        assert_eq!(response.status(), StatusCode::Created);
        Ok(())
    }).await
}

async fn assert_missing(
    container: &azure_data_cosmos::clients::ContainerClient,
    partition_key: &str,
    id: &str
) -> TestResult {
    let error = container
        .read_item(partition_key.to_owned(), id, None).await
        .expect_err("rejected write must not persist an item");
    assert_eq!(error.status().status_code(), StatusCode::NotFound);
    Ok(())
}

fn assert_status<T>(context: &str, result: azure_data_cosmos::Result<T>, expected: StatusCode) {
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("{context}: invalid write must fail"),
    };
    assert_eq!(error.status().status_code(), expected, "{context}");
}
