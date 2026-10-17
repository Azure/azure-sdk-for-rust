// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::ContainerClient,
    diagnostics::{DiagnosticsContext, TransportKind},
    models::{ContainerProperties, PatchInstructions, PatchOperation},
    options::{
        BatchOptions, BinaryEncodingOptions, ConnectionPoolOptions, ContentResponseOnWrite,
        MaxItemCountHint, QueryOptions, Region, ServerCertificateValidation,
    },
    AccountReference, CosmosClient, CosmosRuntime, FeedScope, RoutingStrategy, TransactionalBatch,
};
use azure_data_cosmos_driver::{
    models::{
        AccountReference as DriverAccountReference, ConnectionString, CosmosOperation, FeedRange,
    },
    options::{OperationOptions, OperationOptionsBuilder},
    CosmosDriverRuntime, DriverOptions,
};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{error::Error, num::NonZeroU32, panic::AssertUnwindSafe};
use url::Url;

type TestResult = Result<(), Box<dyn Error>>;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Item {
    id: String,
    name: String,
    pk: String,
    value: i64,
    payload: Value,
}

fn item(id: &str, pk: &str, value: i64) -> Item {
    Item {
        id: id.into(),
        name: "member-between-id-and-pk".into(),
        pk: pk.into(),
        value,
        payload: json!({
            "text": "caf\u{e9}",
            "nested": {"null": null, "empty": [], "values": [true, false, -1, 0.5]},
            "exact_integer": 9_007_199_254_740_991_i64
        }),
    }
}

fn validate_endpoint(endpoint: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(endpoint)?;
    if url.scheme() != "https"
        || !matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("vNext conformance requires an HTTPS loopback emulator endpoint".into());
    }
    Ok(url)
}

fn binary_enabled() -> bool {
    cfg!(test_category = "emulator_vnext_binary")
}

fn assert_transport(diagnostics: &DiagnosticsContext) {
    let expected = if cfg!(test_category = "emulator_vnext_gateway_v2") {
        TransportKind::GatewayV2
    } else {
        TransportKind::Gateway
    };
    let requests = diagnostics.requests();
    assert!(!requests.is_empty(), "conformance must issue real requests");
    for request in requests.iter() {
        assert_ne!(request.status().status_code(), StatusCode::from(0));
        assert_eq!(
            request.transport_kind(),
            expected,
            "vNext must exercise the selected transport; V1 fallback is not V2 coverage"
        );
    }
}

fn assert_encoding(body: &[u8]) {
    assert!(!body.is_empty());
    assert_eq!(
        body.first() == Some(&0x80),
        binary_enabled(),
        "vNext response must use the selected wire encoding"
    );
    if !binary_enabled() {
        serde_json::from_slice::<Value>(body).expect("text response must be valid JSON");
    }
}

async fn assert_stored(container: &ContainerClient, expected: &Item) -> TestResult {
    let response = container
        .read_item(&expected.pk, &expected.id, None)
        .await?;
    assert_transport(&response.diagnostics());
    assert_encoding(&response.into_body().single()?);
    let actual: Item = container
        .read_item(&expected.pk, &expected.id, None)
        .await?
        .into_model()?;
    assert_eq!(&actual, expected);
    Ok(())
}

async fn exercise_items(container: &ContainerClient) -> TestResult {
    let mut document = item("point", "A", 1);
    let create = container
        .create_item("A", &document.id, &document, None)
        .await?;
    assert_eq!(create.status(), StatusCode::Created);
    assert_transport(&create.diagnostics());
    assert_stored(container, &document).await?;

    document.value = 2;
    let replace = container
        .replace_item("A", &document.id, &document, None)
        .await?;
    assert_transport(&replace.diagnostics());
    assert_stored(container, &document).await?;

    document.value = 3;
    let upsert = container
        .upsert_item("A", &document.id, &document, None)
        .await?;
    assert_transport(&upsert.diagnostics());
    assert_stored(container, &document).await?;

    let patch = PatchInstructions::from(vec![PatchOperation::increment("/value", 4_i64)]);
    let response = container.patch_item("A", &document.id, patch, None).await?;
    assert_transport(&response.diagnostics());
    document.value = 7;
    assert_stored(container, &document).await?;

    let delete = container.delete_item("A", &document.id, None).await?;
    assert_eq!(delete.status(), StatusCode::NoContent);
    assert_transport(&delete.diagnostics());
    let missing = container
        .read_item("A", &document.id, None)
        .await
        .unwrap_err();
    assert_eq!(missing.status().status_code(), StatusCode::NotFound);
    Ok(())
}

async fn exercise_batch(container: &ContainerClient) -> TestResult {
    let committed = item("batch", "A", 1);
    let batch = TransactionalBatch::new("A")
        .create_item(&committed)?
        .read_item(committed.id.clone(), None);
    let response = container
        .execute_transactional_batch(
            batch,
            Some(
                BatchOptions::default().with_operation_options(
                    OperationOptionsBuilder::new()
                        .with_content_response_on_write(ContentResponseOnWrite::Enabled)
                        .build(),
                ),
            ),
        )
        .await?;
    assert_transport(&response.diagnostics());
    assert_eq!(response.status(), StatusCode::Ok);
    let model = response.into_model()?;
    assert_eq!(
        model
            .results()
            .iter()
            .map(|r| r.status_code())
            .collect::<Vec<_>>(),
        [201, 200]
    );
    let actual: Item = model.results()[1].into_model()?.expect("batch read body");
    assert_eq!(actual, committed);

    let rolled_back = item("rolled-back", "A", 2);
    let batch = TransactionalBatch::new("A")
        .create_item(&rolled_back)?
        .delete_item("missing", None);
    let response = container.execute_transactional_batch(batch, None).await?;
    assert_transport(&response.diagnostics());
    assert_eq!(response.status(), StatusCode::MultiStatus);
    assert_eq!(
        response
            .into_model()?
            .results()
            .iter()
            .map(|r| r.status_code())
            .collect::<Vec<_>>(),
        [424, 404]
    );
    let missing = container
        .read_item("A", "rolled-back", None)
        .await
        .unwrap_err();
    assert_eq!(missing.status().status_code(), StatusCode::NotFound);
    assert_stored(container, &committed).await?;
    container.delete_item("A", "batch", None).await?;
    Ok(())
}

async fn exercise_queries(container: &ContainerClient, ordered: bool) -> TestResult {
    for index in 0..8 {
        let pk = if index % 2 == 0 { "A" } else { "B" };
        let document = item(&format!("query-{index}"), pk, index);
        container
            .create_item(pk, &document.id, &document, None)
            .await?;
    }
    for (scope, expected) in [
        (FeedScope::partition("A"), vec![0, 2, 4, 6]),
        (FeedScope::full_container(), (0..8).collect()),
    ] {
        let sql = if ordered {
            r#"SELECT * FROM c ORDER BY c["value"] ASC"#
        } else {
            "SELECT * FROM c"
        };
        let options = QueryOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
        let mut pages = container
            .query_items::<Item>(sql, scope.clone(), Some(options.clone()))
            .await?
            .into_pages();
        let mut values = Vec::new();
        let mut count = 0;
        let mut requests = 0;
        while let Some(page) = pages.next().await {
            count += 1;
            assert!(count <= 16, "query continuation must terminate");
            let page = page?;
            let diagnostics = page.diagnostics();
            if diagnostics.request_count() > 0 {
                assert_transport(&diagnostics);
                requests += diagnostics.request_count();
            }
            for document in page.into_items() {
                assert_eq!(document, item(&document.id, &document.pk, document.value));
                values.push(document.value);
            }
            if count == 1 {
                assert!(
                    values.len() < expected.len(),
                    "vNext must provide a continuation for this paging scenario (ordered={ordered})"
                );
                let continuation = pages.to_continuation_token()?;
                pages = container
                    .query_items::<Item>(
                        sql,
                        scope.clone(),
                        Some(options.clone().with_continuation_token(continuation)),
                    )
                    .await?
                    .into_pages();
            }
        }
        if !ordered {
            values.sort_unstable();
        }
        assert_eq!(values, expected, "query must not lose or duplicate items");
        assert!(requests > 0, "query must issue real requests");
        assert!(count > 1, "query must exercise pagination");
    }
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator_vnext",
        test_category = "emulator_vnext_gateway_v2"
    )),
    ignore = "requires a vNext matrix cell and local emulator connection string"
)]
async fn vnext_transport_and_encoding_conformance() -> TestResult {
    run_conformance(Suite::TransportAndEncoding).await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator_vnext",
        test_category = "emulator_vnext_gateway_v2"
    )),
    ignore = "requires a vNext matrix cell and local emulator connection string"
)]
async fn vnext_transactional_batch_conformance() -> TestResult {
    run_conformance(Suite::Batch).await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator_vnext",
        test_category = "emulator_vnext_gateway_v2"
    )),
    ignore = "requires a vNext matrix cell and local emulator connection string"
)]
async fn vnext_query_pagination_conformance() -> TestResult {
    run_conformance(Suite::Query).await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator_vnext",
        test_category = "emulator_vnext_gateway_v2"
    )),
    ignore = "requires a vNext matrix cell and local emulator connection string"
)]
async fn vnext_ordered_query_pagination_conformance() -> TestResult {
    run_conformance(Suite::OrderedQuery).await
}

enum Suite {
    TransportAndEncoding,
    Batch,
    Query,
    OrderedQuery,
}

async fn run_conformance(suite: Suite) -> TestResult {
    if std::env::var("AZURE_COSMOS_TEST_MODE").as_deref() != Ok("required") {
        return Err("vNext conformance requires AZURE_COSMOS_TEST_MODE=required".into());
    }
    let connection: ConnectionString = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?.parse()?;
    let endpoint = validate_endpoint(connection.account_endpoint())?;
    let v2 = cfg!(test_category = "emulator_vnext_gateway_v2");
    let pool = ConnectionPoolOptions::builder()
        .with_gateway_v2_disabled(!v2)
        .with_server_certificate_validation(ServerCertificateValidation::RequiredUnlessEmulator)
        .build()?;
    let runtime = CosmosRuntime::builder()
        .with_connection_pool(pool.clone())
        .build()
        .await?;
    let client = CosmosClient::builder()
        .with_runtime(runtime)
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(binary_enabled()))
        .build(
            AccountReference::with_authentication_key(
                endpoint.as_str().parse()?,
                connection.account_key().clone(),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await?;
    let db_name = format!("vnext-conformance-{}", azure_core::Uuid::new_v4());
    client.create_database(&db_name, None).await?;
    let outcome = AssertUnwindSafe(async {
        let database = client.database_client(&db_name);
        database
            .create_container(ContainerProperties::new("items", "/pk".into()), None)
            .await?;
        let container = database.container_client("items", None).await?;
        match suite {
            Suite::Batch => return exercise_batch(&container).await,
            Suite::Query => return exercise_queries(&container, false).await,
            Suite::OrderedQuery => return exercise_queries(&container, true).await,
            Suite::TransportAndEncoding => {}
        }
        exercise_items(&container).await?;
        let seed = item("query-wire", "A", 42);
        container.create_item("A", &seed.id, &seed, None).await?;

        database
            .create_container(
                ContainerProperties::new("hpk", ("/tenantId", "/userId").into()),
                None,
            )
            .await?;
        let hpk = database.container_client("hpk", None).await?;
        let document =
            json!({"id":"patch-hpk-001","tenantId":"t1","userId":"u1","payload":"hello"});
        let response = hpk
            .create_item(("t1", "u1"), "patch-hpk-001", &document, None)
            .await?;
        assert_transport(&response.diagnostics());
        let response = hpk.read_item(("t1", "u1"), "patch-hpk-001", None).await?;
        assert_transport(&response.diagnostics());
        assert_encoding(&response.into_body().single()?);

        // The typed iterator decodes pages; inspect a raw driver query response for wire proof.
        let driver_runtime = CosmosDriverRuntime::builder()
            .with_connection_pool(pool)
            .build()
            .await?;
        let driver = driver_runtime
            .create_driver(
                DriverOptions::builder(DriverAccountReference::with_account_key(
                    endpoint,
                    connection.account_key().secret().to_string(),
                ))
                .build(),
            )
            .await?;
        let reference = driver
            .resolve_container_by_name(&db_name, "items", OperationOptions::default())
            .await?;
        let range = FeedRange::for_partition("A".into(), reference.partition_key_definition());
        let operation = CosmosOperation::query_items(reference, Some(range)).with_body(
            serde_json::to_vec(&json!({"query":"SELECT * FROM c","parameters":[]}))?,
        );
        let response = driver
            .execute_operation(
                operation,
                OperationOptionsBuilder::new()
                    .with_binary_encoding(
                        azure_data_cosmos_driver::options::BinaryEncodingOptions::new()
                            .with_enabled(binary_enabled()),
                    )
                    .build(),
            )
            .await?
            .expect("seeded query must return a response");
        assert!(response.status().is_success());
        assert_encoding(&response.into_body().single()?);
        Ok::<(), Box<dyn Error>>(())
    })
    .catch_unwind()
    .await;
    let cleanup = client.database_client(&db_name).delete(None).await;
    if let Err(error) = &cleanup {
        eprintln!("vNext cleanup failed: {error}");
    }
    match outcome {
        Ok(result) => {
            result?;
            cleanup?;
        }
        Err(panic) => {
            std::panic::resume_unwind(panic);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_endpoint;

    #[test]
    fn rejects_nonlocal_or_ambiguous_endpoints() {
        for endpoint in [
            "https://account.documents.azure.com/",
            "https://localhost.evil.invalid/",
            "http://localhost:8081/",
            "https://user@localhost/",
            "https://localhost/dbs/",
            "https://localhost/?account=remote",
        ] {
            assert!(validate_endpoint(endpoint).is_err(), "{endpoint}");
        }
        assert!(validate_endpoint("https://localhost:8081/").is_ok());
        assert!(validate_endpoint("https://127.0.0.1:18081/").is_ok());
    }
}
