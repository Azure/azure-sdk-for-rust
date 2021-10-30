#![cfg(all(test, feature = "test_e2e"))]
mod setup;

use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::resources::collection::*;
use futures::stream::StreamExt;

#[tokio::test]
async fn create_and_delete_collection() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-collection";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-collection";

    let client = setup::initialize().unwrap();

    client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let collection = database_client
        .create_collection(
            Context::new(),
            COLLECTION_NAME,
            CreateCollectionOptions::new("/id"),
        )
        .await
        .unwrap();
    let collections =
        Box::pin(database_client.list_collections(Context::new(), ListCollectionsOptions::new()))
            .next()
            .await
            .unwrap()
            .unwrap();
    assert!(collections.collections.len() == 1);

    // try to get the previously created collection
    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    let collection_after_get = collection_client
        .get_collection(Context::new(), GetCollectionOptions::new())
        .await
        .unwrap();
    assert!(collection.collection.rid == collection_after_get.collection.rid);

    // check GetPartitionKeyRanges: https://docs.microsoft.com/rest/api/cosmos-db/get-partition-key-ranges
    collection_client
        .get_partition_key_ranges()
        .execute()
        .await
        .unwrap();

    // delete the collection
    collection_client
        .delete_collection(Context::new(), DeleteCollectionOptions::new())
        .await
        .unwrap();
    let collections =
        Box::pin(database_client.list_collections(Context::new(), ListCollectionsOptions::new()))
            .next()
            .await
            .unwrap()
            .unwrap();
    assert!(collections.collections.len() == 0);

    database_client
        .delete_database(Context::new(), DeleteDatabaseOptions::new())
        .await
        .unwrap();
}

#[tokio::test]
async fn replace_collection() {
    let client = setup::initialize().unwrap();
    const DATABASE_NAME: &str = "test-cosmos-db";
    const COLLECTION_NAME: &str = "test-collection";

    client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
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
    let options = CreateCollectionOptions::new("/id")
        .offer(Offer::S2)
        .indexing_policy(indexing_policy);
    let collection = database_client
        .create_collection(Context::new(), COLLECTION_NAME, options)
        .await
        .unwrap();

    let collections =
        Box::pin(database_client.list_collections(Context::new(), ListCollectionsOptions::new()))
            .next()
            .await
            .unwrap()
            .unwrap();
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

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    let _replace_collection_response = collection_client
        .replace_collection(
            Context::new(),
            ReplaceCollectionOptions::new("/id").indexing_policy(new_ip),
        )
        .await
        .unwrap();

    let collections =
        Box::pin(database_client.list_collections(Context::new(), ListCollectionsOptions::new()))
            .next()
            .await
            .unwrap()
            .unwrap();
    assert_eq!(collections.collections.len(), 1);
    let eps: Vec<&ExcludedPath> = collections.collections[0]
        .indexing_policy
        .excluded_paths
        .iter()
        .filter(|excluded_path| excluded_path.path == "/\"excludeme\"/?")
        .collect();
    assert!(eps.len() > 0);

    database_client
        .delete_database(Context::new(), DeleteDatabaseOptions::new())
        .await
        .unwrap();
}
