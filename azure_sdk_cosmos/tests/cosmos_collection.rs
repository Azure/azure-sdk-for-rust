#![cfg(all(test, feature = "test_e2e"))]

use azure_sdk_cosmos::collection::*;

mod setup;

#[test]
fn create_and_delete_collection() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-collection";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-collection";

    let (client, mut core) = setup::initialize().unwrap();

    core.run(client.create_database(DATABASE_NAME)).unwrap();

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
    let collection = core
        .run(client.create_collection(DATABASE_NAME, 400, &collection_to_create))
        .unwrap();
    let collections = core.run(client.list_collections(DATABASE_NAME)).unwrap();
    assert!(collections.len() == 1);

    // try to get the previously created collection
    let collection_after_get = core.run(client.get_collection(DATABASE_NAME, COLLECTION_NAME)).unwrap();
    assert!(collection.rid == collection_after_get.rid);

    // delete the collection
    core.run(client.delete_collection(DATABASE_NAME, COLLECTION_NAME)).unwrap();
    let collections = core.run(client.list_collections(DATABASE_NAME)).unwrap();
    assert!(collections.len() == 0);

    core.run(client.delete_database(DATABASE_NAME)).unwrap();
}

#[test]
#[ignore]
fn replace_collection() {
    let (client, mut core) = setup::initialize().unwrap();
    const DATABASE_NAME: &str = "test-cosmos-db";
    const COLLECTION_NAME: &str = "test-collection";

    core.run(client.create_database(DATABASE_NAME)).unwrap();

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
    core.run(client.create_collection(DATABASE_NAME, 400, &collection_to_create))
        .unwrap();
    let collections = core.run(client.list_collections(DATABASE_NAME)).unwrap();
    assert!(collections.len() == 1);
    //assert!(collection.indexing_policy)

    // now try to update the indexing policy of the collection (= change_collection)
    // TODO: waiting for issue #153

    core.run(client.delete_database(DATABASE_NAME)).unwrap();
}
