// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::options::{
    AvailabilityStrategy, HedgeThreshold, HedgingStrategy, ItemReadOptions,
    OperationOptionsBuilder, Region,
};
use azure_data_cosmos_spec::{
    assert_http_count, assert_http_overlap, assert_http_regions,
    faults::{self, Op},
    profiles, spec_test,
    testdata::Item,
    ExclusiveAccount, HttpTap, SharedContainer, SpecResult,
};
use std::time::Duration;

fn hedged_after(threshold: Duration) -> ItemReadOptions {
    let strategy = HedgingStrategy::new(HedgeThreshold::new(threshold).unwrap());
    ItemReadOptions::default().with_operation_options(
        OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(strategy))
            .build(),
    )
}

/// When the preferred region is slow, a hedged read sends a second request to the
/// next region while the first is still in flight, returns the faster response,
/// and abandons the slow request.
#[spec_test]
#[service_profiles(MultiRead, MultiWrite)]
#[requires(MultiRegion)]
pub async fn hedge_races_alternate_region(
    #[profile(basic_items)] container: SharedContainer,
    tap: HttpTap,
) -> SpecResult {
    let slow_east = faults::delay(Op::ReadItem, Duration::from_secs(2)).in_region(Region::EAST_US);
    let client = container.client_with_faults([slow_east]).await?;
    let expected = container.dataset().get("item-03");

    let response = client
        .read_item(
            expected.partition_key(),
            &expected.id,
            Some(hedged_after(Duration::from_millis(50))),
        )
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    assert_http_regions!(tap, "ReadItem", [Region::EAST_US, Region::WEST_US]);
    let reads = tap.operation("ReadItem");
    assert_http_overlap!(reads[0], reads[1]);
    assert!(reads[0].was_cancelled(), "the slow request is abandoned");
    Ok(())
}

/// A read throttled with 429 Too Many Requests is retried until it succeeds,
/// each retry being a new HTTP request.
#[spec_test]
pub async fn throttled_read_is_retried(
    #[profile(basic_items)] container: SharedContainer,
    tap: HttpTap,
) -> SpecResult {
    let throttled_twice = faults::throttle(Op::ReadItem).times(2);
    let client = container.client_with_faults([throttled_twice]).await?;
    let expected = container.dataset().get("item-11");

    let response = client
        .read_item(expected.partition_key(), &expected.id, None)
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    assert_http_count!(tap, "ReadItem", 3);
    Ok(())
}

/// When the preferred region goes offline, reads are served by the next
/// preferred region without surfacing an error.
#[spec_test]
#[service_profiles(MultiRead, MultiWrite)]
#[requires(AccountIsolation, RegionControl, MultiRegion)]
#[known_issue("with the single write region offline, the read surfaces 503 even after the next region answers 200", MultiRead)]
pub async fn read_fails_over_when_region_offline(
    account: ExclusiveAccount,
    tap: HttpTap,
) -> SpecResult {
    let container = account.create_container(profiles::basic_items()).await?;
    let expected = container.dataset().get("item-05");

    account.take_region_offline(&Region::EAST_US).await?;
    tap.clear();
    let response = container
        .client()
        .read_item(expected.partition_key(), &expected.id, None)
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    let reads = tap.operation("ReadItem");
    let last = reads.last().expect("at least one read");
    assert_eq!(last.region.as_deref(), Some(Region::WEST_US.as_str()));
    Ok(())
}
