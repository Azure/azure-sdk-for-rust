#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::collection::*;
use azure_cosmos::prelude::*;
use azure_cosmos::Offer;
mod setup;

#[tokio::test]
async fn create_and_delete_collection() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-collection";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-collection";

    let client = setup::initialize().unwrap();

    client
        .create_database()
        .with_database_name(&DATABASE_NAME)
        .execute()
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };
    let collection = database_client
        .create_collection()
        .with_collection_name(&COLLECTION_NAME)
        .with_offer(Offer::S2)
        .with_partition_key(&("/id".into()))
        .with_indexing_policy(&indexing_policy)
        .execute()
        .await
        .unwrap();
    let collections = database_client.list_collections().execute().await.unwrap();
    assert!(collections.collections.len() == 1);

    // try to get the previously created collection
    let collection_client = database_client.into_collection_client(COLLECTION_NAME);

    let collection_after_get = collection_client.get_collection().execute().await.unwrap();
    assert!(collection.collection.rid == collection_after_get.collection.rid);

    // check GetPartitionKeyRanges: https://docs.microsoft.com/en-us/rest/api/cosmos-db/get-partition-key-ranges
    collection_client
        .get_partition_key_ranges()
        .execute()
        .await
        .unwrap();

    // delete the collection
    collection_client
        .delete_collection()
        .execute()
        .await
        .unwrap();
    let collections = database_client.list_collections().execute().await.unwrap();
    assert!(collections.collections.len() == 0);

    database_client.delete_database().execute().await.unwrap();
}

#[tokio::test]
async fn replace_collection() {
    let client = setup::initialize().unwrap();
    const DATABASE_NAME: &str = "test-cosmos-db";
    const COLLECTION_NAME: &str = "test-collection";

    client
        .create_database()
        .with_database_name(&DATABASE_NAME)
        .execute()
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };
    let collection = database_client
        .create_collection()
        .with_collection_name(&COLLECTION_NAME)
        .with_offer(Offer::S2)
        .with_partition_key(&("/id".into()))
        .with_indexing_policy(&indexing_policy)
        .execute()
        .await
        .unwrap();

    let collection_client = database_client.into_collection_client(COLLECTION_NAME);

    let collections = database_client.list_collections().execute().await.unwrap();
    assert_eq!(collections.collections.len(), 1);
    assert_eq!(
        collection.collection.indexing_policy,
        collections.collections[0].indexing_policy
    );

    // Let's change the indexing mode!
    let indexes = IncludedPathIndex {
        kind: KeyKind::Hash,
        data_type: DataType::String,
        precision: Some(3),
    };

    let ip = IncludedPath {
        path: "/*".to_owned(),
        indexes: Some(vec![indexes]),
    };

    let mut new_ip = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![ip],
        excluded_paths: vec![],
    };

    new_ip
        .excluded_paths
        .push("/\"excludeme\"/?".to_owned().into());

    let _replace_collection_reponse = collection_client
        .replace_collection()
        .with_indexing_policy(&new_ip)
        .with_partition_key(&("/id".into()))
        .execute()
        .await
        .unwrap();

    let collections = database_client.list_collections().execute().await.unwrap();
    assert_eq!(collections.collections.len(), 1);
    let eps: Vec<&ExcludedPath> = collections.collections[0]
        .indexing_policy
        .excluded_paths
        .iter()
        .filter(|excluded_path| excluded_path.path == "/\"excludeme\"/?")
        .collect();
    assert!(eps.len() > 0);

    database_client.delete_database().execute().await.unwrap();
}
