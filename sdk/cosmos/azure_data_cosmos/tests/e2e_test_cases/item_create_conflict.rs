// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    models::{PartitionKeyDefinition, PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion},
    PartitionKey,
};
use serde_json::{json, Value};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, should_run},
};

struct DuplicateCreateCase {
    id: &'static str,
    partition_key_definition: PartitionKeyDefinition,
    partition_key: PartitionKey,
    original: Value,
    duplicate: Value,
}

fn duplicate_create_cases() -> Vec<DuplicateCreateCase> {
    let simple = |id, version| DuplicateCreateCase {
        id,
        partition_key_definition: PartitionKeyDefinition::new(vec!["/pk".into()])
            .with_kind(PartitionKeyKind::Hash)
            .with_version(version),
        partition_key: PartitionKey::from("A"),
        original: json!({ "id": "duplicate-1", "pk": "A", "value": 1 }),
        duplicate: json!({ "id": "duplicate-1", "pk": "A", "value": 2 }),
    };
    vec![
        simple("hashV1", PartitionKeyVersion::V1),
        simple("hashV2", PartitionKeyVersion::V2),
        DuplicateCreateCase {
            id: "hierarchicalV2",
            partition_key_definition: PartitionKeyDefinition::new(vec![
                "/tenant".into(),
                "/user".into(),
            ])
            .with_kind(PartitionKeyKind::MultiHash)
            .with_version(PartitionKeyVersion::V2),
            partition_key: PartitionKey::from(vec![
                PartitionKeyValue::from("tenant-a"),
                PartitionKeyValue::from("user-1"),
            ]),
            original: json!({
                "id": "duplicate-1",
                "tenant": "tenant-a",
                "user": "user-1",
                "value": 1
            }),
            duplicate: json!({
                "id": "duplicate-1",
                "tenant": "tenant-a",
                "user": "user-1",
                "value": 2
            }),
        },
    ]
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn duplicate_create_preserves_original() -> TestResult {
    if !should_run("item.create-conflict")? {
        return Ok(());
    }
    for case in duplicate_create_cases() {
        let document_id = case
            .original
            .get("id")
            .and_then(Value::as_str)
            .expect("original document must have an id");
        assert_eq!(
            case.duplicate.get("id").and_then(Value::as_str),
            Some(document_id)
        );

        E2eTestFixture::run_with_partition_key(case.partition_key_definition, async |fixture| {
            fixture
                .container
                .create_item(
                    case.partition_key.clone(),
                    document_id,
                    &case.original,
                    None,
                )
                .await?;
            let error = fixture
                .container
                .create_item(
                    case.partition_key.clone(),
                    document_id,
                    &case.duplicate,
                    None,
                )
                .await
                .expect_err("duplicate create must fail");
            assert_eq!(error.status().status_code(), StatusCode::Conflict);
            assert_eq!(
                error
                    .status()
                    .sub_status()
                    .map(|value| value.value())
                    .unwrap_or(0),
                0
            );
            assert_critical_diagnostics(
                &error
                    .diagnostics()
                    .expect("service error must carry diagnostics"),
                "create_item",
                StatusCode::Conflict,
            );
            let stored: Value = fixture
                .container
                .read_item(case.partition_key.clone(), document_id, None)
                .await?
                .into_model()?;
            for (name, expected) in case
                .original
                .as_object()
                .expect("original document must be an object")
            {
                assert_eq!(
                    stored.get(name),
                    Some(expected),
                    "fixture '{}' field '{name}' changed after duplicate create",
                    case.id
                );
            }
            Ok(())
        })
        .await?;
    }
    Ok(())
}
