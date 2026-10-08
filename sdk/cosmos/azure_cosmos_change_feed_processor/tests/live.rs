// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{env, error::Error, num::NonZeroU32, time::Duration};

use azure_core::{credentials::Secret, http::StatusCode};
use azure_cosmos_change_feed_processor::{ChangeFeedProcessor, ReadChangesOptions};
use serde::Deserialize;

#[derive(Deserialize)]
struct Document {
    id: String,
}

#[tokio::test]
#[ignore = "requires an existing real Cosmos account and read-only credentials"]
async fn real_account_change_feed_and_resume() -> Result<(), Box<dyn Error>> {
    tokio::time::timeout(Duration::from_secs(90), read_and_resume()).await?
}

async fn read_and_resume() -> Result<(), Box<dyn Error>> {
    let processor = ChangeFeedProcessor::connect_with_key(
        env::var("AZURE_COSMOS_ENDPOINT")?.parse()?,
        Secret::new(env::var("AZURE_COSMOS_KEY")?),
        &env::var("AZURE_COSMOS_DATABASE")?,
        &env::var("AZURE_COSMOS_CONTAINER")?,
    )
    .await?;
    let mut options =
        ReadChangesOptions::default().with_max_item_count(NonZeroU32::new(1).unwrap());
    let mut found_document = false;
    for _ in 0..16 {
        let page = processor.read_page::<Document>(&options).await?;
        assert!(matches!(
            page.status(),
            StatusCode::Ok | StatusCode::NotModified
        ));
        assert!(page.request_charge().is_some_and(|charge| charge > 0.0));
        assert!(page.activity_id().is_some_and(|id| !id.is_empty()));
        assert!(!page.continuation().is_empty());
        assert!(page
            .items()
            .iter()
            .all(|item| item.current().is_some_and(|item| !item.id.is_empty())));
        found_document |= !page.items().is_empty();
        println!(
            "live page: status={}, items={}, charge={:?}, resumed={}",
            page.status(),
            page.items().len(),
            page.request_charge(),
            options.continuation().is_some(),
        );
        options = options.with_continuation(page.continuation());
        if found_document {
            let resumed = processor.read_page::<Document>(&options).await?;
            assert!(matches!(
                resumed.status(),
                StatusCode::Ok | StatusCode::NotModified
            ));
            assert!(resumed.request_charge().is_some_and(|charge| charge > 0.0));
            assert!(!resumed.continuation().is_empty());
            println!("live resume succeeded: items={}", resumed.items().len());
            return Ok(());
        }
    }
    Err("no changes found within the bounded 16-page live read".into())
}
