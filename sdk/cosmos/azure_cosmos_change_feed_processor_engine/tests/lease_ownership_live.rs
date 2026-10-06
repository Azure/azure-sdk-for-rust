// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{env, error::Error, time::Duration};

use azure_cosmos_change_feed_processor_engine::{CosmosLeaseStore, LeaseOwnershipOptions};
use azure_data_cosmos_driver::{
    models::{AccountReference, PartitionKey},
    options::DriverOptions,
    CosmosDriverRuntime,
};
use tokio::time::{sleep, timeout};

#[tokio::test]
#[ignore = "writes one explicitly supplied pre-created lease in a single-write-region account"]
async fn two_workers_contend_takeover_and_fence_obsolete_owner() -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(90), exercise()).await?
}

async fn exercise() -> Result<(), Box<dyn Error>> {
    let duration_ms: u64 = env::var("AZURE_COSMOS_LEASE_DURATION_MS")?.parse()?;
    if !(5000..=30000).contains(&duration_ms) {
        return Err("live test lease interval must be between 5 and 30 seconds".into());
    }
    let policy = LeaseOwnershipOptions::new(
        Duration::from_millis(duration_ms),
        Duration::from_millis(duration_ms / 10),
        Duration::from_millis(duration_ms / 4),
        Duration::from_secs(1),
    )?;
    let mut stores = Vec::new();
    for _ in 0..2 {
        let runtime = CosmosDriverRuntime::builder().build().await?;
        let account = AccountReference::with_master_key(
            env::var("AZURE_COSMOS_LEASE_ENDPOINT")?.parse()?,
            azure_core::credentials::Secret::new(env::var("AZURE_COSMOS_LEASE_KEY")?),
        );
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await?;
        stores.push(
            CosmosLeaseStore::new_single_writer(
                driver,
                &env::var("AZURE_COSMOS_LEASE_DATABASE")?,
                &env::var("AZURE_COSMOS_LEASE_CONTAINER")?,
                PartitionKey::from(env::var("AZURE_COSMOS_LEASE_PK")?),
                env::var("AZURE_COSMOS_LEASE_ID")?,
                policy.clone(),
            )
            .await?,
        );
    }
    let a = stores[0].observe().await?;
    let b = stores[1].observe().await?;
    if a.owner().is_some() || b.owner().is_some() {
        return Err("live test requires a released dedicated test lease; it will not take over production work".into());
    }
    let owner_a = format!("cfp-live-A-{}", std::process::id());
    let owner_b = format!("cfp-live-B-{}", std::process::id());
    let (a, b) = tokio::join!(
        stores[0].acquire(&a, owner_a),
        stores[1].acquire(&b, owner_b),
    );
    let (old, next_store) = match (a, b) {
        (Ok(Some(old)), Err(_)) => (old, &stores[1]),
        (Err(_), Ok(Some(old))) => (old, &stores[0]),
        _ => return Err("exactly one worker must establish authority".into()),
    };
    let previous = old.lease().await;
    let observation = next_store.observe().await?;
    sleep(policy.duration() + Duration::from_millis(100)).await;
    let next = next_store
        .acquire(
            &observation,
            format!("cfp-live-takeover-{}", std::process::id()),
        )
        .await?
        .ok_or("unchanged lease should permit takeover")?;
    let verification = async {
        let current = next.lease().await;
        if current.epoch().get() != previous.epoch().get() + 1 {
            return Err::<(), Box<dyn Error>>("takeover must advance the generation".into());
        }
        if old
            .checkpoint(&previous, previous.checkpoint())
            .await
            .is_ok()
            || old.release().await.is_ok()
        {
            return Err("obsolete owner must not checkpoint or release".into());
        }
        // Confirm the stored checkpoint through a real conditional write.
        next.checkpoint(&current, current.checkpoint())
            .await
            .map_err(|error| format!("{error:?}"))?;
        let stored = next_store.observe().await?;
        if stored.owner() != Some(current.owner())
            || stored.checkpoint() != previous.checkpoint().as_str()
        {
            return Err("stale operations changed replacement state".into());
        }
        Ok(())
    }
    .await;
    // Release even when verification failed, while current authority remains valid.
    let release = next.release().await;
    verification?;
    release?;
    assert!(next_store.observe().await?.owner().is_none());
    println!("two independent workers: one winner, takeover, stale fencing, and release confirmed");
    Ok(())
}
