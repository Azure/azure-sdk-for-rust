// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::panic::AssertUnwindSafe;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::{
    models::{ContainerProperties, IndexingMode, IndexingPolicy},
    Query,
};
use futures::{FutureExt, TryStreamExt};

use crate::e2e_test_cases::{
    fixture::{build_client, TestResult},
    support::should_run,
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn database_and_container_resource_lifecycle() -> TestResult {
    if !should_run("management.resource-lifecycle").await? {
        return Ok(());
    }

    let client = build_client().await?;
    let database_id = format!("e2e-management-{}", Uuid::new_v4());
    let container_id = format!("items-{}", Uuid::new_v4());

    let create_database = client.create_database(&database_id, None).await?;
    let database = client.database_client(&database_id);
    let outcome = AssertUnwindSafe(async {
        assert_eq!(create_database.status(), StatusCode::Created);
        assert_eq!(
            create_database.into_model()?.id.as_deref(),
            Some(database_id.as_str())
        );
        run_container_lifecycle(&database, &container_id).await
    })
    .catch_unwind()
    .await;
    let cleanup = database.delete(None).await;
    match outcome {
        Ok(Ok(())) => {
            let delete_database = cleanup?;
            assert_eq!(delete_database.status(), StatusCode::NoContent);
        }
        Ok(Err(test_error)) => match cleanup {
            Ok(_) => return Err(test_error),
            Err(cleanup_error) => {
                return Err(format!(
                    "resource lifecycle failed: {test_error}; database cleanup also failed: {cleanup_error}"
                )
                .into())
            }
        },
        Err(panic) => {
            if let Err(error) = cleanup {
                eprintln!("E2E database cleanup after panic failed: {error}");
            }
            std::panic::resume_unwind(panic);
        }
    }
    let error = database
        .read(None)
        .await
        .expect_err("deleted database must not remain readable");
    assert_eq!(error.status().status_code(), StatusCode::NotFound);
    Ok(())
}

async fn run_container_lifecycle(
    database: &azure_data_cosmos::clients::DatabaseClient,
    container_id: &str,
) -> TestResult {
    let properties = ContainerProperties::new(container_id.to_owned(), "/pk".into());
    let create = database.create_container(properties, None).await?;
    assert_eq!(create.status(), StatusCode::Created);
    let created = create.into_model()?;
    assert_eq!(created.id.as_ref(), container_id);
    assert_eq!(created.partition_key.paths()[0].as_ref(), "/pk");

    let container = database.container_client(container_id, None).await?;
    let read = container.read(None).await?;
    assert_eq!(read.status(), StatusCode::Ok);
    assert_eq!(read.into_model()?.id.as_ref(), container_id);

    let mut indexing_policy = IndexingPolicy::default();
    indexing_policy.automatic = false;
    indexing_policy.indexing_mode = Some(IndexingMode::None);
    let expected_indexing_policy = indexing_policy.clone();
    let replacement = ContainerProperties::new(container_id.to_owned(), "/pk".into())
        .with_indexing_policy(indexing_policy);
    let replaced = container.replace(replacement, None).await?;
    assert_eq!(replaced.status(), StatusCode::Ok);
    let replaced = replaced.into_model()?;
    assert_eq!(
        replaced.indexing_policy.as_ref(),
        Some(&expected_indexing_policy)
    );
    assert_eq!(
        replaced
            .indexing_policy
            .as_ref()
            .and_then(|policy| policy.indexing_mode.clone()),
        Some(IndexingMode::None)
    );
    let reread = container.read(None).await?.into_model()?;
    assert_eq!(reread.indexing_policy, replaced.indexing_policy);

    let query =
        Query::from("SELECT * FROM root r WHERE r.id = @id").with_parameter("@id", container_id)?;
    let matches: Vec<ContainerProperties> = database
        .query_containers(query, None)
        .await?
        .try_collect()
        .await?;
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id.as_ref(), container_id);

    let delete = container.delete(None).await?;
    assert_eq!(delete.status(), StatusCode::NoContent);
    let error = container
        .read(None)
        .await
        .expect_err("deleted container must not remain readable");
    assert_eq!(error.status().status_code(), StatusCode::NotFound);
    Ok(())
}
