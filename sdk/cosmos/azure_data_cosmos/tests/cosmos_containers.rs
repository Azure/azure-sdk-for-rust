#![cfg(feature = "key_auth")]

mod framework;

use std::{error::Error, sync::Arc};

use azure_core::http::Method;
use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{
    models::{
        ContainerProperties, IndexingMode, IndexingPolicy, PartitionKeyKind, PropertyPath,
        ThroughputProperties,
    },
    CreateContainerOptions, Query,
};
use futures::TryStreamExt;

use framework::{test_data, LocalRecorder, TestAccount, TestAccountOptions};

#[recorded::test]
pub async fn container_crud(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;

    // Create the container
    let properties = ContainerProperties {
        id: "TheContainer".into(),
        partition_key: "/id".into(),
        indexing_policy: Some(IndexingPolicy {
            included_paths: vec!["/*".into()],
            excluded_paths: vec![r#"/"_etag"/?"#.into()],
            automatic: true,
            indexing_mode: Some(IndexingMode::Consistent),
            ..Default::default()
        }),
        ..Default::default()
    };

    let throughput = ThroughputProperties::manual(400);

    let created_properties = db_client
        .create_container(
            properties.clone(),
            Some(CreateContainerOptions {
                throughput: Some(throughput),
                ..Default::default()
            }),
        )
        .await?
        .into_body()?;

    assert_eq!(&properties.id, &created_properties.id);
    assert_eq!(
        vec![String::from("/id")],
        created_properties.partition_key.paths
    );
    assert_eq!(
        PartitionKeyKind::Hash,
        created_properties.partition_key.kind
    );
    let indexing_policy = created_properties
        .indexing_policy
        .expect("created container should have an indexing policy");
    assert_eq!(
        vec![PropertyPath::from("/*")],
        indexing_policy.included_paths
    );
    assert_eq!(
        vec![PropertyPath::from(r#"/"_etag"/?"#)],
        indexing_policy.excluded_paths
    );
    assert!(indexing_policy.automatic);
    assert_eq!(
        IndexingMode::Consistent,
        indexing_policy.indexing_mode.unwrap()
    );

    let mut query_pager = db_client.query_containers(
        Query::from("SELECT * FROM root r WHERE r.id = @id")
            .with_parameter("@id", &properties.id)?,
        None,
    )?;
    let mut ids = vec![];
    while let Some(db) = query_pager.try_next().await? {
        ids.push(db.id);
    }
    assert_eq!(vec![properties.id.clone()], ids);

    let container_client = db_client.container_client(&properties.id);
    let updated_properties = ContainerProperties {
        id: properties.id.clone(),
        partition_key: properties.partition_key.clone(),
        indexing_policy: Some(IndexingPolicy {
            included_paths: vec![],
            excluded_paths: vec![],
            automatic: false,
            indexing_mode: Some(IndexingMode::None),
            ..Default::default()
        }),
        ..Default::default()
    };
    let update_response = container_client
        .replace(updated_properties, None)
        .await?
        .into_body()?;
    let updated_indexing_policy = update_response.indexing_policy.unwrap();
    assert!(updated_indexing_policy.included_paths.is_empty());
    assert!(updated_indexing_policy.excluded_paths.is_empty());
    assert!(!updated_indexing_policy.automatic);
    assert_eq!(
        Some(IndexingMode::None),
        updated_indexing_policy.indexing_mode
    );

    let current_throughput = container_client
        .read_throughput(None)
        .await?
        .expect("throughput should be present")
        .into_body()?;

    assert_eq!(Some(400), current_throughput.throughput());

    let new_throughput = ThroughputProperties::manual(500);
    let throughput_response = container_client
        .replace_throughput(new_throughput, None)
        .await?
        .into_body()?;
    assert_eq!(Some(500), throughput_response.throughput());

    container_client.delete(None).await?;

    query_pager = db_client.query_containers(
        Query::from("SELECT * FROM root r WHERE r.id = @id")
            .with_parameter("@id", &properties.id)?,
        None,
    )?;
    let mut ids = vec![];
    while let Some(db) = query_pager.try_next().await? {
        ids.push(db.id);
    }
    assert!(ids.is_empty());

    account.cleanup().await?;

    Ok(())
}

#[recorded::test]
pub async fn container_crud_autoscale(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;

    // Create the container
    let properties = ContainerProperties {
        id: "TheContainer".into(),
        partition_key: "/id".into(),
        indexing_policy: Some(IndexingPolicy {
            included_paths: vec!["/*".into()],
            excluded_paths: vec![r#"/"_etag"/?"#.into()],
            automatic: true,
            indexing_mode: Some(IndexingMode::Consistent),
            ..Default::default()
        }),
        ..Default::default()
    };

    let throughput = ThroughputProperties::autoscale(5000, Some(42));

    db_client
        .create_container(
            properties.clone(),
            Some(CreateContainerOptions {
                throughput: Some(throughput),
                ..Default::default()
            }),
        )
        .await?
        .into_body()?;
    let container_client = db_client.container_client(&properties.id);

    let current_throughput = container_client
        .read_throughput(None)
        .await?
        .expect("throughput should be present")
        .into_body()?;

    assert_eq!(Some(500), current_throughput.throughput());
    assert_eq!(Some(5000), current_throughput.autoscale_maximum());
    assert_eq!(Some(42), current_throughput.autoscale_increment());

    account.cleanup().await?;

    Ok(())
}

#[recorded::test]
pub async fn container_crud_hierarchical_pk(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;

    // Create the container
    let properties = ContainerProperties {
        id: "TheContainer".into(),
        partition_key: ("/parent", "/child", "/grandchild").into(),
        indexing_policy: Some(IndexingPolicy {
            included_paths: vec!["/*".into()],
            excluded_paths: vec![r#"/"_etag"/?"#.into()],
            automatic: true,
            indexing_mode: Some(IndexingMode::Consistent),
            ..Default::default()
        }),
        ..Default::default()
    };

    let created_properties = db_client
        .create_container(properties.clone(), None)
        .await?
        .into_body()?;

    assert_eq!(&properties.id, &created_properties.id);
    assert_eq!(
        vec![
            String::from("/parent"),
            String::from("/child"),
            String::from("/grandchild")
        ],
        created_properties.partition_key.paths
    );
    assert_eq!(
        PartitionKeyKind::MultiHash,
        created_properties.partition_key.kind
    );

    account.cleanup().await?;

    Ok(())
}

#[recorded::test]
pub async fn container_read_throughput_twice(context: TestContext) -> Result<(), Box<dyn Error>> {
    let recorder = Arc::new(LocalRecorder::new());
    let account = TestAccount::from_env(
        context,
        Some(TestAccountOptions {
            recorder: Some(recorder.clone()),
            ..Default::default()
        }),
    )
    .await?;

    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;

    let properties = ContainerProperties {
        id: "ThroughputTestContainer".into(),
        partition_key: "/id".into(),
        ..Default::default()
    };
    let throughput = ThroughputProperties::manual(600);

    db_client
        .create_container(
            properties.clone(),
            Some(CreateContainerOptions {
                throughput: Some(throughput),
                ..Default::default()
            }),
        )
        .await?
        .into_body()?;
    let container_client = db_client.container_client(&properties.id);

    let first_throughput = container_client
        .read_throughput(None)
        .await?
        .expect("throughput should be present")
        .into_body()?;
    assert_eq!(Some(600), first_throughput.throughput());

    let second_throughput = container_client
        .read_throughput(None)
        .await?
        .expect("throughput should be present")
        .into_body()?;
    assert_eq!(Some(600), second_throughput.throughput());

    // Check the recorder to ensure only one request was made to read the container metadata
    let txs = recorder.to_transactions().await;
    assert_eq!(
        1,
        txs.iter()
            .filter(|t| t.request.method() == Method::Get
                && t.request
                    .url()
                    .path()
                    .ends_with("/colls/ThroughputTestContainer"))
            .count()
    );

    account.cleanup().await?;
    Ok(())
}
