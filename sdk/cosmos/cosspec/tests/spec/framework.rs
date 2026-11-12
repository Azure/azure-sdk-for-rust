// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Specs for the spec framework itself.

use cosspec::{
    assert_status, spec_test, testdata::Item, ExclusiveContainer, HttpTap, InvalidConfiguration,
    SharedContainer, SpecResult,
};

/// Using a capability the spec did not declare fails as an invalid configuration.
#[spec_test(id = "framework.undeclared_capability_is_rejected")]
#[service_profiles(SingleRegion)]
pub async fn undeclared_capability_is_rejected(container: ExclusiveContainer) -> SpecResult {
    let error = container
        .split()
        .await
        .expect_err("split() needs #[requires(TopologyControl)]");

    assert!(error.is::<InvalidConfiguration>(), "{error}");
    Ok(())
}

/// Writing to a shared container is rejected with 403 Forbidden before it
/// reaches the service.
#[spec_test]
#[service_profiles(SingleRegion)]
pub async fn shared_container_rejects_writes(
    #[profile(basic_items)] container: SharedContainer,
    tap: HttpTap,
) -> SpecResult {
    let item = Item::new("intruder", "pk-0", -1);

    let result = container
        .client()
        .create_item(item.partition_key(), &item.id, &item, None)
        .await;

    assert_status!(result, 403);
    // The framework also fails any spec whose tap shows a shared-container write.
    // Clear the evidence so this spec, which writes on purpose, can pass.
    tap.clear();
    Ok(())
}
