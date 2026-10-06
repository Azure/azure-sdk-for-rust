// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::fixture;
use crate::ChangeFeedReadOptions;
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::models::{ChangeFeedStartFrom, FeedRange};
use std::error::Error;

#[tokio::test]
async fn recovery_confirms_committed_progress_without_adopting_a_replacement_owner(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let expected = session.lease().await;
    let mut reader = f
        .worker_a
        .open_reader(
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                .with_continuation(expected.checkpoint().clone()),
        )
        .await?;
    let candidate = reader.read_page().await?.continuation().clone();
    let observed = f.store_b.observe().await?;
    let mut committed = observed.record.clone();
    committed.checkpoint = candidate.as_str().to_owned();
    f.store_b.replace(&committed, observed.revision()).await?;
    let recovered = session
        .reconcile_checkpoint(&expected, &candidate)
        .await?
        .unwrap();
    assert!(session.control().is_lost());
    assert!(!recovered.control().is_lost());
    assert_eq!(recovered.lease().await.checkpoint(), &candidate);
    assert_eq!(recovered.lease().await.epoch(), expected.epoch());
    let fresh = recovered.lease().await;
    let replacement = f.store_b.transfer(&f.store_b.observe().await?, "B").await?;
    assert!(recovered
        .reconcile_checkpoint(&fresh, &candidate)
        .await?
        .is_none());
    assert_eq!(f.store_b.observe().await?.owner(), Some("B"));
    replacement.release().await?;
    Ok(())
}

#[tokio::test]
async fn recovery_fences_late_old_writes_before_retrying_only_the_pending_checkpoint(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let expected = session.lease().await;
    let mut reader = f
        .worker_a
        .open_reader(
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                .with_continuation(expected.checkpoint().clone()),
        )
        .await?;
    let candidate = reader.read_page().await?.continuation().clone();
    let observed = f.store_b.observe().await?;
    let mut late_write = observed.record.clone();
    late_write.checkpoint = candidate.as_str().to_owned();
    let recovered = session
        .reconcile_checkpoint(&expected, &candidate)
        .await?
        .unwrap();
    assert_eq!(recovered.lease().await.checkpoint(), expected.checkpoint());
    assert_eq!(
        f.store_b
            .replace(&late_write, observed.revision())
            .await
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::PreconditionFailed
    );
    let prior = recovered.lease().await;
    recovered.checkpoint(&prior, &candidate).await.unwrap();
    assert_eq!(f.store_b.observe().await?.checkpoint(), candidate.as_str());
    recovered.release().await?;
    Ok(())
}
