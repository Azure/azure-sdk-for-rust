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
    container_with_emulator(definition, 4).await.0
}

async fn container_with_emulator(
    definition: PartitionKeyDefinition,
    partitions: u32,
) -> (ContainerClient, Arc<InMemoryEmulatorHttpClient>) {
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
            .with_partition_count(partitions)
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
    let container = client
        .database_client("read-many")
        .container_client("items", None)
        .await
        .unwrap();
    (container, emulator)
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
async fn read_many_populated_split_after_first_page_has_no_loss_or_duplicates() {
    let definition = PartitionKeyDefinition::new(vec!["/pk".into()]);
    for select_items in [true, false] {
        let (container, emulator) = container_with_emulator(definition.clone(), 1).await;
        let mut selections = Vec::new();
        let mut expected = Vec::new();
        for i in 0..32 {
            let pk = format!("pk-{i}");
            for selected in [true, false] {
                let id = format!("{}-{i}", if selected { "selected" } else { "excluded" });
                container
                    .create_item(
                        pk.clone(),
                        &id,
                        &json!({"id":id,"pk":pk,"selected":selected}),
                        None,
                    )
                    .await
                    .unwrap();
                if selected {
                    selections.push((pk.clone().into(), id.clone()));
                    expected.push((pk.clone(), id));
                }
            }
        }
        expected.sort();
        let selection = if select_items {
            ReadManySelection::Items(selections)
        } else {
            ReadManySelection::Partitions(selections.into_iter().map(|(key, _)| key).collect())
        };
        let options = ReadManyOptions::default()
            .with_filter("c.selected = true".parse().unwrap())
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(3).unwrap()));
        let mut pages = container
            .read_many::<Value>(selection, Some(options))
            .await
            .unwrap();
        let first = pages.next().await.unwrap().unwrap().into_items();
        assert_eq!(first.len(), 3);
        let store = emulator.store();
        store.split_partition("read-many", "items", 0, std::time::Duration::ZERO);
        store.wait_for_split("read-many", "items", 0).await;
        let mut actual = first;
        while let Some(page) = pages.next().await {
            actual.extend(page.unwrap().into_items());
        }
        assert_eq!(identities(actual), expected, "select_items={select_items}");
    }
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
async fn read_many_undefined_keys_include_empty_objects_without_including_null() {
    for hierarchical in [false, true] {
        let definition = if hierarchical {
            serde_json::from_value(
                json!({"paths":["/pk","/tenant"],"kind":"MultiHash","version":2}),
            )
            .unwrap()
        } else {
            PartitionKeyDefinition::new(vec!["/pk".into()])
        };
        let (container, _) = container_with_emulator(definition, 1).await;
        let key = |component| {
            if hierarchical {
                PartitionKey::from((component, "tenant"))
            } else {
                PartitionKey::from(component)
            }
        };
        let undefined = key(PartitionKeyValue::UNDEFINED);
        let null = key(PartitionKeyValue::NULL);
        for (pk, document) in [
            (undefined.clone(), json!({"id":"missing","tenant":"tenant"})),
            (
                undefined.clone(),
                json!({"id":"object","pk":{},"tenant":"tenant"}),
            ),
            (
                null.clone(),
                json!({"id":"null","pk":null,"tenant":"tenant"}),
            ),
        ] {
            container
                .create_item(pk, document["id"].as_str().unwrap(), &document, None)
                .await
                .unwrap();
        }
        let mut results = Vec::new();
        for (selection, filter, expected) in [
            (
                ReadManySelection::Partitions(vec![undefined.clone()]),
                None,
                vec!["missing", "object"],
            ),
            (
                ReadManySelection::Items(vec![(undefined.clone(), "object".into())]),
                None,
                vec!["object"],
            ),
            (
                ReadManySelection::Items(vec![(undefined.clone(), "object".into())]),
                Some("true"),
                vec!["object"],
            ),
            (
                ReadManySelection::Items(vec![
                    (undefined.clone(), "missing".into()),
                    (undefined.clone(), "object".into()),
                ]),
                None,
                vec!["missing", "object"],
            ),
            (
                ReadManySelection::Partitions(vec![null]),
                None,
                vec!["null"],
            ),
        ] {
            let options =
                filter.map(|text| ReadManyOptions::default().with_filter(text.parse().unwrap()));
            let response = container
                .read_many::<Value>(selection, options)
                .await
                .unwrap()
                .collect_all()
                .await
                .unwrap();
            let mut actual = response
                .items()
                .iter()
                .map(|item| item["id"].as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            actual.sort();
            assert_eq!(actual, expected);
            results.push(actual);
        }
        assert_eq!(
            results[1], results[2],
            "filtered and unfiltered identity results must agree"
        );
    }
}

#[tokio::test]
async fn read_many_object_filter_keys_keep_escape_semantics() {
    let container = seed().await;
    let expected = (0..5)
        .map(|i| ("one".to_owned(), format!("d{i}")))
        .collect::<Vec<_>>();
    for predicate in [
        r#"({"\u0061": true})["a"]"#,
        r#"({"a\"b": true})["a\"b"]"#,
        r#"({"a\\b": true})["a\\b"]"#,
    ] {
        let options = ReadManyOptions::default().with_filter(predicate.parse().unwrap());
        let response = container
            .read_many::<Value>(
                ReadManySelection::Partitions(vec!["one".into()]),
                Some(options),
            )
            .await
            .unwrap()
            .collect_all()
            .await
            .unwrap();
        assert_eq!(identities(response.into_items()), expected, "{predicate}");
    }
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
