// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! PATCH remains available without the preview strategy configuration.

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    models::{
        AccountReference, CosmosOperation, ItemReference, PartitionKey, PatchInstructions,
        PatchOperation,
    },
    options::{DriverOptions, OperationOptions},
};

#[tokio::test]
async fn patch_uses_auto_without_preview_configuration() {
    let context = super::setup_single_region().await;
    let runtime = context.emulator.runtime_builder().build().await.unwrap();
    let account = AccountReference::with_master_key(
        Url::parse(&context.gateway_url).unwrap(),
        "ZW11bGF0b3Ita2V5",
    );
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .unwrap();
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "item1");
    let document = serde_json::json!({"id": "item1", "pk": "pk1", "visits": 1});
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item.clone())
                .with_body(serde_json::to_vec(&document).unwrap()),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    let patch = PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);
    driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item.clone())
                .with_body(serde_json::to_vec(&patch).unwrap()),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .unwrap();
    let body = response.into_body().single().unwrap();
    let actual = super::parse_json_body(&body).unwrap();
    assert_eq!(actual["visits"], 2);
}
