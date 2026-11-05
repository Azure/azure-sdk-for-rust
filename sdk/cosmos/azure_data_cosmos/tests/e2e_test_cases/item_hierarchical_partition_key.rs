// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{feed::FeedScope, PartitionKey, Query};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::should_run,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TenantItem {
    id: String,
    tenant: String,
    user: String,
    value: i64,
}

impl TenantItem {
    fn new(id: &str, tenant: &str, user: &str, value: i64) -> Self {
        Self {
            id: id.to_owned(),
            tenant: tenant.to_owned(),
            user: user.to_owned(),
            value,
        }
    }

    fn partition_key(&self) -> PartitionKey {
        PartitionKey::from((&self.tenant, &self.user))
    }
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn hierarchical_key_point_and_prefix_operations() -> TestResult {
    if !should_run("item.hierarchical-partition-key").await? {
        return Ok(());
    }

    E2eTestFixture::run_with_partition_key(("/tenant", "/user").into(), async |fixture| {
        let mut first = TenantItem::new("a-1", "tenant-a", "user-1", 1);
        let second = TenantItem::new("a-2", "tenant-a", "user-2", 2);
        let other = TenantItem::new("b-1", "tenant-b", "user-1", 3);
        for value in [&first, &second, &other] {
            fixture
                .container
                .create_item(value.partition_key(), &value.id, value, None)
                .await?;
        }

        first.value = 10;
        let replace = fixture
            .container
            .replace_item(first.partition_key(), &first.id, &first, None)
            .await?;
        assert_eq!(replace.status(), StatusCode::Ok);
        let reread: TenantItem = fixture
            .container
            .read_item(first.partition_key(), &first.id, None)
            .await?
            .into_model()?;
        assert_eq!(reread, first);

        let query = Query::from("SELECT * FROM c ORDER BY c.id ASC");
        let descendants: Vec<TenantItem> = fixture
            .container
            .query_items(query, FeedScope::partition("tenant-a"), None)
            .await?
            .try_collect()
            .await?;
        assert_eq!(descendants, vec![first.clone(), second]);

        let delete = fixture
            .container
            .delete_item(first.partition_key(), &first.id, None)
            .await?;
        assert_eq!(delete.status(), StatusCode::NoContent);
        let error = fixture
            .container
            .read_item(first.partition_key(), &first.id, None)
            .await
            .expect_err("deleted HPK item must not remain readable");
        assert_eq!(error.status().status_code(), StatusCode::NotFound);
        Ok(())
    })
    .await
}
