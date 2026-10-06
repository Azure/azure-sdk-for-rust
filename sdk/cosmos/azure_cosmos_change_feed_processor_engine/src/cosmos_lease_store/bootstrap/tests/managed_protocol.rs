// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{fixture, policy, BootstrapStartPolicy, BootstrapStore};
use crate::ChangeFeedMode;
use std::{error::Error, time::Duration};

#[tokio::test]
async fn persisted_partial_plan_is_loaded_before_any_new_positions_are_chosen(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    assert!(BootstrapStore::load_existing_single_writer(
        f.driver.clone(),
        f.a.store.container.clone(),
        f.a.plan.source(),
        f.a.plan.group(),
        ChangeFeedMode::LatestVersion,
        &BootstrapStartPolicy::Beginning,
        policy(),
    )
    .await?
    .is_none());
    f.a.create_if_absent().await?;
    let loaded = BootstrapStore::load_existing_single_writer(
        f.driver.clone(),
        f.a.store.container.clone(),
        f.a.plan.source(),
        f.a.plan.group(),
        ChangeFeedMode::LatestVersion,
        &BootstrapStartPolicy::Beginning,
        policy(),
    )
    .await?
    .unwrap();
    assert!(loaded.plan == f.a.plan);
    let ready = loaded
        .ensure_initialized("resuming-worker", Duration::from_secs(10))
        .await?;
    assert!(ready.plan() == &f.a.plan);
    assert_eq!(ready.lease_ids().len(), 2);
    assert!(BootstrapStore::load_existing_single_writer(
        f.driver,
        f.a.store.container.clone(),
        f.a.plan.source(),
        f.a.plan.group(),
        ChangeFeedMode::AllVersionsAndDeletes,
        &BootstrapStartPolicy::Beginning,
        policy(),
    )
    .await
    .is_err());
    Ok(())
}
