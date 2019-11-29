#![cfg(all(test, feature = "test_e2e"))]
use azure_sdk_cosmos::collection::*;
use azure_sdk_cosmos::Offer;
mod setup;

#[tokio::test]
async fn create_and_delete_collection() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-collection";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-collection";

    let client = setup::initialize().unwrap();

    client.create_database(DATABASE_NAME).await.unwrap();

    // create a new collection
    let collection_to_create = Collection::new(
        COLLECTION_NAME,
        IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![],
            excluded_paths: vec![],
        },
    );
    let collection = client
        .create_collection(DATABASE_NAME, Offer::S2, &collection_to_create)
        .await
        .unwrap();
    let collections = client.list_collections(DATABASE_NAME).await.unwrap();
    assert!(collections.len() == 1);

    // try to get the previously created collection
    let collection_after_get = client
        .get_collection(DATABASE_NAME, COLLECTION_NAME)
        .await
        .unwrap();
    assert!(collection.rid == collection_after_get.rid);

    // delete the collection
    client
        .delete_collection(DATABASE_NAME, COLLECTION_NAME)
        .await
        .unwrap();
    let collections = client.list_collections(DATABASE_NAME).await.unwrap();
    assert!(collections.len() == 0);

    client.delete_database(DATABASE_NAME).await.unwrap();
}

#[ignore]
#[tokio::test]
async fn replace_collection() {
    let client = setup::initialize().unwrap();
    const DATABASE_NAME: &str = "test-cosmos-db";
    const COLLECTION_NAME: &str = "test-collection";

    client.create_database(DATABASE_NAME).await.unwrap();

    // create a new collection
    let collection_to_create = Collection::new(
        COLLECTION_NAME,
        IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![],
            excluded_paths: vec![],
        },
    );
    client
        .create_collection(DATABASE_NAME, Offer::S2, &collection_to_create)
        .await
        .unwrap();
    let collections = client.list_collections(DATABASE_NAME).await.unwrap();
    assert!(collections.len() == 1);
    //assert!(collection.indexing_policy)

    // now try to update the indexing policy of the collection (= change_collection)
    // TODO: waiting for issue #153

    client.delete_database(DATABASE_NAME).await.unwrap();
}
