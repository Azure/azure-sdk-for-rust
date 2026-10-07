// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    options::{
        AvailabilityStrategy, BinaryEncodingOptions, ContentResponseOnWrite, ItemReadOptions,
        ItemWriteOptions, OperationOptions, ReadConsistencyStrategy, Region,
    },
    CosmosClient, RoutingStrategy,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{
        assert_critical_diagnostics, hosted_wire_counts, item, selected_scenario_profile,
        wait_for_item_replication, Item,
    },
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn binary_text_and_routing_options_preserve_behavior() -> TestResult {
    let Some(profile) = selected_scenario_profile("configuration.binary-routing").await? else {
        return Ok(());
    };
    let runtime = profile.selected_runtime()?;
    let profile_client = profile.selected_client()?;
    let preferred = build_binary_client(ClientSetup::from_profile(
        runtime,
        profile_client,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::EAST_US]),
    )?)
    .await?;
    run_binary_routing_case(preferred, "preferred-regions", Region::WEST_US, true).await?;

    let account_order = build_binary_client(ClientSetup::from_profile_with_routing_override(
        runtime,
        profile_client,
        RoutingStrategy::PreferredRegions(Vec::new()),
    )?)
    .await?;
    run_binary_routing_case(account_order, "account-order", Region::EAST_US, false).await
}

async fn build_binary_client(setup: ClientSetup) -> TestResult<CosmosClient> {
    build_client_with_customizer(setup, |builder| {
        Ok(builder.with_binary_encoding_options(
            BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(true),
        ))
    })
    .await
}

async fn run_binary_routing_case(
    client: CosmosClient,
    case: &str,
    expected_region: Region,
    wait_for_replication: bool,
) -> TestResult {
    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item(&format!("binary-routing-{case}"), "A", 7);
            let wire_before = hosted_wire_counts().await?;
            let mut operation = OperationOptions::default();
            operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
            let create = fixture
                .container
                .create_item(
                    "A",
                    &expected.id,
                    &expected,
                    Some(ItemWriteOptions::default().with_operation_options(operation)),
                )
                .await?;
            assert_eq!(create.status().status_code(), StatusCode::Created);
            assert_critical_diagnostics(&create.diagnostics(), "create_item", StatusCode::Created);
            let bytes = create.into_body().single()?;
            assert_eq!(
                bytes.first(),
                Some(&b'{'),
                "text response must be UTF-8 JSON"
            );
            assert_eq!(serde_json::from_slice::<Item>(&bytes)?, expected);
            let before = wire_before.expect("hosted emulator must expose wire counters");
            let after = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose wire counters");
            assert!(
                after.binary_negotiated_requests > before.binary_negotiated_requests,
                "binary-enabled create must negotiate CosmosBinary"
            );
            assert!(
                after.binary_payload_requests > before.binary_payload_requests,
                "binary-enabled create must send a binary item payload"
            );
            assert!(
                after.binary_response_payloads > before.binary_response_payloads,
                "binary-enabled create must receive a binary wire response before transcoding"
            );

            let mut text_write_operation = OperationOptions::default();
            text_write_operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
            text_write_operation.binary_encoding =
                Some(BinaryEncodingOptions::new().with_enabled(false));
            let text_write_before = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose wire counters");
            let text_write = fixture
                .container
                .upsert_item(
                    "A",
                    &expected.id,
                    expected.clone(),
                    Some(ItemWriteOptions::default().with_operation_options(text_write_operation)),
                )
                .await?;
            let text_write_after = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose wire counters");
            assert_eq!(
                text_write_after.binary_negotiated_requests,
                text_write_before.binary_negotiated_requests,
                "operation-level disable must suppress binary negotiation"
            );
            assert_eq!(
                text_write_after.binary_payload_requests, text_write_before.binary_payload_requests,
                "operation-level disable must send the upsert payload as text"
            );
            assert_eq!(
                text_write_after.binary_response_payloads,
                text_write_before.binary_response_payloads,
                "operation-level disable must request a text upsert response"
            );
            assert_eq!(text_write.into_model::<Item>()?, expected);

            if wait_for_replication {
                wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;
            }
            let mut operation = OperationOptions::default();
            operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Eventual);
            operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
            // Operation-level text JSON overrides the binary-enabled client.
            operation.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(false));
            let read_wire_before = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose wire counters");
            let read = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await?;
            let read_wire_after = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose wire counters");
            assert_eq!(
                read_wire_after.binary_negotiated_requests,
                read_wire_before.binary_negotiated_requests,
                "operation-level disable must suppress binary negotiation"
            );
            assert_eq!(
                read_wire_after.binary_payload_requests, read_wire_before.binary_payload_requests,
                "operation-level disable must suppress binary request payloads"
            );
            assert_eq!(
                read_wire_after.binary_response_payloads, read_wire_before.binary_response_payloads,
                "operation-level disable must suppress binary response payloads"
            );
            assert_critical_diagnostics(&read.diagnostics(), "read_item", StatusCode::Ok);
            assert_eq!(
                read.diagnostics().regions_contacted().first(),
                Some(&expected_region),
                "'{case}' routing used an unexpected initial region"
            );
            assert_eq!(read.into_model::<Item>()?, expected);
            Ok(())
        })
        .await
}
