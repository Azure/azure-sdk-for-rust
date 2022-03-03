#![cfg(all(test, feature = "test_e2e"))]
mod setup;

use azure_data_cosmos::prelude::*;
use azure_data_cosmos::resources::collection::*;
use futures::StreamExt;

#[tokio::test]
async fn create_and_delete_collection() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-collection";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-collection";

    let client = setup::initialize().unwrap();

    client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.into_database(DATABASE_NAME);

    // create a new collection
    let collection = database
        .create_collection(COLLECTION_NAME, "/id")
        .into_future()
        .await
        .unwrap();
    let collections = Box::pin(database.list_collections().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(collections.collections.len() == 1);

    // try to get the previously created collection
    let collection = database.clone().into_collection(COLLECTION_NAME);

    let collection_after_get = collection.get_collection().into_future().await.unwrap();
    assert!(collection.collection.rid == collection_after_get.collection.rid);

    // check GetPartitionKeyRanges: https://docs.microsoft.com/rest/api/cosmos-db/get-partition-key-ranges
    collection
        .get_partition_key_ranges()
        .into_future()
        .await
        .unwrap();

    // delete the collection
    collection.delete_collection().into_future().await.unwrap();
    let collections = Box::pin(database.list_collections().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(collections.collections.len() == 0);

    database.delete_database().into_future().await.unwrap();
}

#[tokio::test]
async fn replace_collection() {
    let client = setup::initialize().unwrap();
    const DATABASE_NAME: &str = "test-cosmos-db";
    const COLLECTION_NAME: &str = "test-collection";

    client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.into_database(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let collection = database
        .create_collection(COLLECTION_NAME, "/id")
        .offer(Offer::S2)
        .indexing_policy(indexing_policy)
        .into_future()
        .await
        .unwrap();

    let collections = Box::pin(database.list_collections().into_stream())
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

    let collection = database.clone().into_collection(COLLECTION_NAME);

    let _replace_collection_response = collection
        .replace_collection("/id")
        .indexing_policy(new_ip)
        .into_future()
        .await
        .unwrap();

    let collections = Box::pin(database.list_collections().into_stream())
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

    database.delete_database().into_future().await.unwrap();
}
