// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{credentials::Secret, http::Url};
use azure_data_cosmos::{
    feed::{ReadManyFilter, ReadManySelection},
    options::{
        BinaryEncodingOptions, MaxItemCountHint, OperationOptionsBuilder, ReadManyOptions, Region,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    PartitionKey, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{PartitionKeyDefinition, PartitionKeyValue},
};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{num::NonZeroU32, sync::Arc};

async fn container(definition: PartitionKeyDefinition) -> ContainerClient {
    let endpoint = "https://eastus.emulator.local";
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
        VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(endpoint).unwrap(),
        )])
        .unwrap(),
    ));
    emulator.store().create_database("read-many");
    emulator.store().create_container_with_config(
        "read-many",
        "items",
        definition,
        ContainerConfig::new()
            .with_partition_count(4)
            .build()
            .unwrap(),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(
            AccountReference::with_authentication_key(
                endpoint.parse::<AccountEndpoint>().unwrap(),
                Secret::new("dGVzdGtleQ=="),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await
        .unwrap();
    client
        .database_client("read-many")
        .container_client("items", None)
        .await
        .unwrap()
}

async fn seed() -> ContainerClient {
    let container = container(PartitionKeyDefinition::new(vec!["/pk".into()])).await;
    for pk in ["one", "two", "outside"] {
        for rank in 0..5 {
            let id = format!("d{rank}");
            container
                .create_item(pk, &id, &json!({"id":id,"pk":pk,"rank":rank}), None)
                .await
                .unwrap();
        }
    }
    container
}

fn identities(items: Vec<Value>) -> Vec<(String, String)> {
    let mut result = items
        .iter()
        .map(|item| {
            (
                item["pk"].as_str().unwrap().to_owned(),
                item["id"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    result.sort();
    result
}

#[tokio::test]
async fn read_many_items_deduplicate_and_omit_missing() {
    let container = seed().await;
    let selection = ReadManySelection::Items(vec![
        ("one".into(), "d1".into()),
        ("two".into(), "d1".into()),
        ("one".into(), "d1".into()),
        ("one".into(), "missing".into()),
    ]);
    let response = container
        .read_many::<Value>(selection, None)
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert!(response.request_charge().value() > 0.0);
    assert!(response.diagnostics().is_some());
    assert_eq!(
        identities(response.into_items()),
        vec![("one".into(), "d1".into()), ("two".into(), "d1".into())]
    );
    let missing = container
        .read_many::<Value>(
            ReadManySelection::Items(vec![("one".into(), "missing".into())]),
            None,
        )
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert!(missing.items().is_empty());
    assert!(!missing.diagnostics().unwrap().is_failure());
}

#[tokio::test]
async fn read_many_partition_filter_cannot_broaden_selection() {
    let container = seed().await;
    let filter = "c.rank = @__read_many_0 OR c.pk = @outside"
        .parse::<ReadManyFilter>()
        .unwrap()
        .with_parameter("@__read_many_0", 2)
        .unwrap()
        .with_parameter("@outside", "outside")
        .unwrap();
    let options = ReadManyOptions::default()
        .with_filter(filter)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    let response = container
        .read_many::<Value>(
            ReadManySelection::Partitions(vec!["one".into(), "two".into(), "one".into()]),
            Some(options),
        )
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert_eq!(
        identities(response.into_items()),
        vec![("one".into(), "d2".into()), ("two".into(), "d2".into())]
    );
}

#[tokio::test]
async fn read_many_filtered_singleton_and_empty_selection() {
    let container = seed().await;
    let options = ReadManyOptions::default().with_filter("c.rank = 99".parse().unwrap());
    let result = container
        .read_many::<Value>(
            ReadManySelection::Items(vec![("one".into(), "d1".into())]),
            Some(options),
        )
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert!(result.items().is_empty());
    for selection in [
        ReadManySelection::Items(Vec::new()),
        ReadManySelection::Partitions(Vec::new()),
    ] {
        let result = container
            .read_many::<Value>(selection, None)
            .await
            .unwrap()
            .collect_all()
            .await
            .unwrap();
        assert!(result.items().is_empty());
        assert_eq!(result.request_charge().value(), 0.0);
        assert!(result.diagnostics().is_none());
    }
}

#[tokio::test]
async fn read_many_paging_and_collection_agree() {
    let container = seed().await;
    for binary in [false, true] {
        let options = ReadManyOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()))
            .with_operation_options(
                OperationOptionsBuilder::new()
                    .with_binary_encoding(BinaryEncodingOptions::default().with_enabled(binary))
                    .build(),
            );
        let selection = ReadManySelection::Partitions(vec!["one".into(), "two".into()]);
        let mut pages = container
            .read_many::<Value>(selection.clone(), Some(options.clone()))
            .await
            .unwrap();
        let mut items = Vec::new();
        let mut charge = 0.0;
        let mut count = 0;
        while let Some(page) = pages.next().await {
            let page = page.unwrap();
            assert!(page.headers().continuation().is_none());
            charge += page.headers().request_charge().unwrap().value();
            items.extend(page.into_items());
            count += 1;
        }
        assert!(count >= 10);
        let all = container
            .read_many::<Value>(selection, Some(options))
            .await
            .unwrap()
            .collect_all()
            .await
            .unwrap();
        assert_eq!(all.request_charge().value(), charge);
        assert_eq!(identities(all.into_items()), identities(items));
        assert!(pages.collect_all().await.is_err());
    }
}

#[tokio::test]
async fn read_many_hierarchical_keys_and_id_completion() {
    let definition =
        serde_json::from_value(json!({"paths":["/tenant","/id"],"kind":"MultiHash","version":2}))
            .unwrap();
    let container = container(definition).await;
    container
        .create_item(
            ("tenant", "item"),
            "item",
            &json!({"tenant":"tenant","id":"item"}),
            None,
        )
        .await
        .unwrap();
    let all = container
        .read_many::<Value>(
            ReadManySelection::Items(vec![
                ("tenant".into(), "item".into()),
                (("tenant", "item").into(), "item".into()),
            ]),
            None,
        )
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert_eq!(all.items().len(), 1);
    assert_eq!(all.items()[0]["id"], "item");
    assert!(container
        .read_many::<Value>(ReadManySelection::Partitions(vec!["tenant".into()]), None)
        .await
        .is_err());
    let mismatch = container
        .read_many::<Value>(
            ReadManySelection::Items(vec![
                (("tenant", "wrong").into(), "item".into()),
                (("tenant", "wrong2").into(), "item".into()),
            ]),
            None,
        )
        .await
        .unwrap()
        .collect_all()
        .await
        .unwrap();
    assert!(mismatch.items().is_empty());
}

#[tokio::test]
async fn read_many_null_and_undefined_keys_are_distinct() {
    let container = container(PartitionKeyDefinition::new(vec!["/pk".into()])).await;
    let undefined = PartitionKey::from(PartitionKeyValue::UNDEFINED);
    let null = PartitionKey::from(Option::<String>::None);
    container
        .create_item(undefined.clone(), "u", &json!({"id":"u"}), None)
        .await
        .unwrap();
    container
        .create_item(null.clone(), "n", &json!({"id":"n","pk":null}), None)
        .await
        .unwrap();
    for (key, id) in [(undefined, "u"), (null, "n")] {
        let response = container
            .read_many::<Value>(ReadManySelection::Partitions(vec![key]), None)
            .await
            .unwrap()
            .collect_all()
            .await
            .unwrap();
        assert_eq!(response.items().len(), 1);
        assert_eq!(response.items()[0]["id"], id);
    }
}

#[derive(Deserialize)]
struct RequiredNumber {
    #[serde(rename = "value")]
    _value: u64,
}

#[tokio::test]
async fn read_many_decode_error_keeps_failing_page_diagnostics() {
    let container = container(PartitionKeyDefinition::new(vec!["/pk".into()])).await;
    for (id, value) in [("a", json!(1)), ("b", json!("not a number"))] {
        container
            .create_item("pk", id, &json!({"id":id,"pk":"pk","value":value}), None)
            .await
            .unwrap();
    }
    for (selection, requests) in [
        (ReadManySelection::Items(vec![("pk".into(), "b".into())]), 1),
        (ReadManySelection::Partitions(vec!["pk".into()]), 2),
    ] {
        let options = ReadManyOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
        let error = container
            .read_many::<RequiredNumber>(selection, Some(options))
            .await
            .unwrap()
            .collect_all()
            .await
            .err()
            .expect("invalid value must fail");
        let diagnostics = error
            .diagnostics()
            .expect("all completed pages must retain diagnostics");
        assert_eq!(diagnostics.request_count(), requests);
        assert!(diagnostics.total_request_charge().value() > 0.0);
    }
}
