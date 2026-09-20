// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
        FaultOperationType,
    },
    options::{
        AvailabilityStrategy, HedgeThreshold, HedgingStrategy, ItemReadOptions,
        OperationOptionsBuilder, Region,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, wait_for_item_replication, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn alternate_region_wins_delayed_primary_hedge() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.hedging").await? else {
        return Ok(());
    };
    let delayed = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-hedge-primary-delay", delayed)
            .with_condition(condition)
            .build(),
    );
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::EAST_US, Region::WEST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder.with_fault_injection_rules(vec![rule])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("hedging", "A", 3);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                    HedgeThreshold::new(Duration::from_millis(50)).expect("non-zero threshold"),
                )))
                .build();
            let response = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await?;
            let diagnostics = response.diagnostics();
            assert_eq!(response.into_model::<Item>()?, expected);
            let hedge = diagnostics
                .hedge_diagnostics()
                .expect("delayed primary must produce hedge diagnostics");
            assert_eq!(hedge.terminal_state().as_str(), "alternate_won");
            assert_eq!(hedge.primary_region(), &Region::EAST_US);
            assert_eq!(hedge.alternate_region(), Some(&Region::WEST_US));
            assert_eq!(hedge.response_region(), Some(&Region::WEST_US));
            assert!(diagnostics.regions_contacted().contains(&Region::WEST_US));
            Ok(())
        })
        .await
}
