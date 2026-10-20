// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    models::{PartitionKeyDefinition, PartitionKeyVersion},
    PartitionKey,
};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, should_run},
};

const SHARED_ID: &str = "same-id";

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn scalar_partition_key_values_remain_distinct() -> TestResult {
    if !should_run("item.scalar-partition-keys").await? {
        return Ok(());
    }

    run_scalar_cases(PartitionKeyDefinition::from("/pk").with_version(PartitionKeyVersion::V1))
        .await?;
    run_scalar_cases(PartitionKeyDefinition::from("/pk")).await
}

async fn run_scalar_cases(partition_key: PartitionKeyDefinition) -> TestResult {
    E2eTestFixture::run_with_partition_key(partition_key, async |fixture| {
        let cases = scalar_cases();
        for case in &cases {
            let mut body = serde_json::json!({
                "id": SHARED_ID,
                "value": case.value,
            });
            if let Some(partition_key_property) = &case.body_partition_key {
                body["pk"] = partition_key_property.clone();
            }

            let create = fixture
                .container
                .create_item(case.partition_key.clone(), SHARED_ID, &body, None)
                .await?;
            assert_eq!(create.status(), StatusCode::Created, "case {}", case.id);
            assert_critical_diagnostics(
                create.diagnostics().as_ref(),
                "create_item",
                StatusCode::Created,
            );
        }

        for case in &cases {
            let stored: serde_json::Value = fixture
                .container
                .read_item(case.partition_key.clone(), SHARED_ID, None)
                .await?
                .into_model()?;
            assert_eq!(stored["id"], SHARED_ID, "case {}", case.id);
            assert_eq!(stored["value"], case.value, "case {}", case.id);
            match &case.expected_partition_key {
                Some(expected) => assert_eq!(stored.get("pk"), Some(expected), "case {}", case.id),
                None => assert!(stored.get("pk").is_none(), "case {}", case.id),
            }
        }

        for case in &cases {
            let delete = fixture
                .container
                .delete_item(case.partition_key.clone(), SHARED_ID, None)
                .await?;
            assert_eq!(delete.status(), StatusCode::NoContent, "case {}", case.id);
        }

        let object_value = serde_json::json!({
            "id": "object-value",
            "pk": {},
            "value": 6,
        });
        let create = fixture
            .container
            .create_item(
                PartitionKey::from(PartitionKey::UNDEFINED),
                "object-value",
                object_value,
                None,
            )
            .await?;
        assert_eq!(create.status(), StatusCode::Created);
        let stored: serde_json::Value = fixture
            .container
            .read_item(
                PartitionKey::from(PartitionKey::UNDEFINED),
                "object-value",
                None,
            )
            .await?
            .into_model()?;
        assert_eq!(stored["pk"], serde_json::json!({}));
        Ok(())
    })
    .await
}

struct ScalarCase {
    id: &'static str,
    partition_key: PartitionKey,
    body_partition_key: Option<serde_json::Value>,
    expected_partition_key: Option<serde_json::Value>,
    value: i64,
}

fn scalar_cases() -> Vec<ScalarCase> {
    vec![
        ScalarCase {
            id: "string",
            partition_key: PartitionKey::from("A"),
            body_partition_key: Some(serde_json::json!("A")),
            expected_partition_key: Some(serde_json::json!("A")),
            value: 1,
        },
        ScalarCase {
            id: "number",
            partition_key: PartitionKey::from(42_i64),
            body_partition_key: Some(serde_json::json!(42)),
            expected_partition_key: Some(serde_json::json!(42)),
            value: 2,
        },
        ScalarCase {
            id: "boolean",
            partition_key: PartitionKey::from(true),
            body_partition_key: Some(serde_json::json!(true)),
            expected_partition_key: Some(serde_json::json!(true)),
            value: 3,
        },
        ScalarCase {
            id: "null",
            partition_key: PartitionKey::from(PartitionKey::NULL),
            body_partition_key: Some(serde_json::Value::Null),
            expected_partition_key: Some(serde_json::Value::Null),
            value: 4,
        },
        ScalarCase {
            id: "undefined",
            partition_key: PartitionKey::from(PartitionKey::UNDEFINED),
            body_partition_key: None,
            expected_partition_key: None,
            value: 5,
        },
    ]
}
