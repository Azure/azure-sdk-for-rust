// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::options::{
    AvailabilityStrategy, HedgeThreshold, HedgingStrategy, ItemReadOptions, OperationOptionsBuilder,
};
use cosspec::{
    assert_http_count, assert_http_overlap, assert_http_regions,
    faults::{self, Op},
    profiles, spec_test,
    testdata::Item,
    ExclusiveAccount, HttpTap, RegionAlias, SharedContainer, SpecResult,
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
    let secondary = container.regions()[1];
    let slow_primary = faults::delay(Op::ReadItem, Duration::from_secs(2))
        .in_region(RegionAlias::PrimaryReadWrite);
    let client = container.client_with_faults([slow_primary]).await?;
    let expected = container.dataset().get("item-03");

    let response = client
        .read_item(
            expected.partition_key(),
            &expected.id,
            Some(hedged_after(Duration::from_millis(50))),
        )
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    assert_http_regions!(tap, "ReadItem", [RegionAlias::PrimaryReadWrite, secondary]);
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
#[known_issue("the read passes, but deleting the database at teardown is a write and hits the write_fails_over_when_write_region_offline issue", MultiRead)]
pub async fn read_fails_over_when_region_offline(
    account: ExclusiveAccount,
    tap: HttpTap,
) -> SpecResult {
    let container = account.create_container(profiles::basic_items()).await?;
    let expected = container.dataset().get("item-05");
    let secondary = account.regions()[1];

    account
        .take_region_offline(RegionAlias::PrimaryReadWrite)
        .await?;
    tap.clear();
    let response = container
        .client()
        .read_item(expected.partition_key(), &expected.id, None)
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    let reads = tap.operation("ReadItem");
    let last = reads.last().expect("at least one read");
    assert_eq!(last.region_alias.as_deref(), Some(secondary.name()));
    Ok(())
}

/// When the write region goes offline, the account promotes the next region to
/// accept writes, and a write succeeds there without surfacing an error.
#[spec_test]
#[service_profiles(MultiRead, MultiWrite)]
#[requires(AccountIsolation, RegionControl, MultiRegion)]
#[known_issue("the SDK keeps sending the write to the offline region and surfaces 503 instead of refreshing the account to find the new write region", MultiRead)]
pub async fn write_fails_over_when_write_region_offline(
    account: ExclusiveAccount,
    tap: HttpTap,
) -> SpecResult {
    let container = account.create_container(profiles::basic_items()).await?;
    let item = Item::new("after-offline", "pk-0", 1);
    let secondary = account.regions()[1];

    account
        .take_region_offline(RegionAlias::PrimaryReadWrite)
        .await?;
    tap.clear();
    container
        .client()
        .create_item(item.partition_key(), &item.id, &item, None)
        .await?;

    let writes = tap.operation("CreateItem");
    let last = writes.last().expect("at least one write");
    assert_eq!(last.region_alias.as_deref(), Some(secondary.name()));
    Ok(())
}
